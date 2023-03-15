use core::fmt::{Debug, Formatter};

pub type ByteError = ();

pub trait ByteInput {
    fn read(&mut self, data: &mut [u8]) -> Result<(), ByteError>;

    fn read_u8(&mut self) -> Result<u8, ByteError> {
        let mut b = [0];
        self.read(&mut b)?;
        Ok(b[0])
    }
}

impl ByteInput for &[u8] {
    fn read(&mut self, data: &mut [u8]) -> Result<(), ByteError> {
        if data.len() > self.len() {
            return Err(());
        }
        let amt = data.len();
        let (a, b) = core::mem::replace(self, &[]).split_at(amt);
        data[..amt].copy_from_slice(a);
        *self = b;
        Ok(())
    }
}

pub trait ByteOutput {
    fn write(&mut self, data: &[u8]) -> Result<(), ByteError>;

    fn write_u8(&mut self, data: u8) -> Result<(), ByteError> {
        let b = [data];
        self.write(&b)?;
        Ok(())
    }
}

impl ByteOutput for &mut [u8] {
    fn write(&mut self, data: &[u8]) -> Result<(), ByteError> {
        let amt = data.len().min(self.len());
        let (a, b) = core::mem::replace(self, &mut []).split_at_mut(amt);
        a.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(())
    }
}

pub struct BitInput<'b, B: ByteInput> {
    i: &'b mut B,
    b: u8,
    o: usize,
}

impl<'b, B: ByteInput> Debug for BitInput<'b, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut s = f.debug_struct("BitInput");
        s.field("b", &self.b);
        s.field("o", &self.o);
        s.finish()
    }
}

impl<'b, B: ByteInput> BitInput<'b, B> {
    pub fn new(i: &'b mut B) -> Self {
        Self { i, b: 0, o: 0 }
    }

    pub fn read_u8(&mut self, bits: usize) -> Result<u8, ByteError> {
        if bits > 8 {
            return Err(());
        }
        if bits == 0 {
            return Ok(0);
        }
        let s = self.o + 8 - bits;
        if s >= 8 {
            self.o = s - 8;
            Ok((self.b >> self.o) & !(0xFF << bits))
        } else {
            self.o = s;
            let b = self.b;
            self.b = self.i.read_u8()?;
            Ok(((b << (8 - s)) | (self.b >> s)) & !(0xFF << bits))
        }
    }

    pub fn read_u16(&mut self, bits: usize) -> Result<u16, ByteError> {
        if bits > 16 {
            return Err(());
        }
        if bits == 0 {
            return Ok(0);
        }
        let s = self.o + 16 - bits;
        if s >= 16 {
            self.o = s - 16;
            return Ok(((self.b as u16) >> self.o) & !(0xFF << bits));
        } else if s >= 8 {
            self.o = s - 8;
            let b = self.b;
            self.b = self.i.read_u8()?;
            return Ok((((b as u16) << (16 - s)) | (self.b as u16 >> (s - 8))) & !(0xFF << bits));
        } else {
            self.o = s;
            let b = self.b;
            let mut d = [0; 2];
            self.i.read(&mut d)?;
            self.b = d[1];
            return Ok(
                (((b as u16) << (16 - s)) | ((d[0] as u16) << (8 - s)) | (d[1] as u16 >> s))
                    & !(0xFFFF << bits),
            );
        }
    }
}
