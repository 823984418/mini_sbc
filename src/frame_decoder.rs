use crate::crc::crc8;
use crate::filter_state::FilterState;
use crate::header::{Blocks, ChannelMode, MSBC_BLOCKS, SBCHeader};
use crate::io::{BitInput, ByteError, ByteInput};
use crate::sbc;
use crate::sbc::{Channels, Subbands, ValidChannels, ValidSubbands};

const SBCDEC_FIXED_EXTRA_BITS: u8 = 2;

#[derive(Debug)]
pub enum FrameDecodeError {
    ByteError(ByteError),
    NoBlock,
    SizeBed,
    CRCBed,
}

impl From<ByteError> for FrameDecodeError {
    fn from(value: ByteError) -> Self {
        Self::ByteError(value)
    }
}

#[derive(Debug)]
pub struct FrameDecoder<'d, 'b, const CHANNELS: usize, const SUBBANDS: usize, B: ByteInput>
where
    Channels<CHANNELS>: ValidChannels,
    Subbands<SUBBANDS>: ValidSubbands,
{
    decoder: &'d mut FilterState<CHANNELS, SUBBANDS>,
    joint: u8,
    scale_factor: [[u8; SUBBANDS]; CHANNELS],
    bits: [[u8; SUBBANDS]; CHANNELS],
    buffer: BitInput<'b, B>,
    blocks: usize,
}

impl<'d, 'b, const CHANNELS: usize, const SUBBANDS: usize, B: ByteInput>
    FrameDecoder<'d, 'b, CHANNELS, SUBBANDS, B>
where
    Channels<CHANNELS>: ValidChannels,
    Subbands<SUBBANDS>: ValidSubbands,
{
    pub fn new_no_crc(
        header: &SBCHeader,
        decoder: &'d mut FilterState<CHANNELS, SUBBANDS>,
        buffer: &'b mut B,
    ) -> Result<Self, FrameDecodeError> {
        if header.channel_mode().channels() != CHANNELS || header.subbands().subbands() != SUBBANDS
        {
            return Err(FrameDecodeError::SizeBed);
        }
        let mut i = BitInput::new(buffer);
        let joint = if let ChannelMode::SBC_MODE_JOINT_STEREO = header.channel_mode() {
            i.read_u8(SUBBANDS)?
        } else {
            0
        };
        let scale_factor = {
            let mut s = [[0; SUBBANDS]; CHANNELS];
            for ch in 0..CHANNELS {
                for sb in 0..SUBBANDS {
                    s[ch][sb] = i.read_u8(4)?;
                }
            }
            s
        };

        let bits = sbc::calculate_bits(header, &scale_factor);

        Ok(Self {
            decoder,
            joint,
            scale_factor,
            bits,
            buffer: i,
            blocks: header.blocks().map(Blocks::blocks).unwrap_or(MSBC_BLOCKS),
        })
    }

    fn crc(&self, header: &SBCHeader) -> u8 {
        let mut crc = crc8(0x0F, &header.encode_array()[1..], 16);
        if let ChannelMode::SBC_MODE_JOINT_STEREO = header.channel_mode() {
            crc = crc8(
                crc,
                &[self.joint >> (8 - header.subbands().subbands())],
                header.subbands().subbands(),
            );
        }
        for ch in 0..CHANNELS {
            let mut b = [0_u8; SUBBANDS];
            for sb in 0..SUBBANDS {
                b[sb >> 1] |= (self.scale_factor[ch][sb] & 0x0F) << ((!sb & 1) * 4);
            }
            crc = crc8(crc, &b, SUBBANDS * 4);
        }
        crc
    }

    pub fn new(
        header: &SBCHeader,
        decoder: &'d mut FilterState<CHANNELS, SUBBANDS>,
        buffer: &'b mut B,
    ) -> Result<Self, FrameDecodeError> {
        let crc = buffer.read_u8()?;
        let s = Self::new_no_crc(header, decoder, buffer)?;
        if s.crc(header) == crc {
            Ok(s)
        } else {
            Err(FrameDecodeError::CRCBed)
        }
    }

    pub fn new_skip_crc(
        header: &SBCHeader,
        decoder: &'d mut FilterState<CHANNELS, SUBBANDS>,
        buffer: &'b mut B,
    ) -> Result<Self, FrameDecodeError> {
        let _crc = buffer.read_u8()?;
        Self::new_no_crc(header, decoder, buffer)
    }

    pub fn next(&mut self) -> Result<[[i16; SUBBANDS]; CHANNELS], FrameDecodeError> {
        if self.blocks == 0 {
            return Err(FrameDecodeError::NoBlock);
        }
        self.blocks -= 1;
        let mut sample = [[0_i32; SUBBANDS]; CHANNELS];
        for ch in 0..CHANNELS {
            for sb in 0..SUBBANDS {
                let bits = self.bits[ch][sb];
                if bits == 0 {
                    sample[ch][sb] = 0;
                    continue;
                }
                let shift = self.scale_factor[ch][sb] + 1 + SBCDEC_FIXED_EXTRA_BITS;
                let s = self.buffer.read_u16(bits as usize)? as i32;

                sample[ch][sb] =
                    ((((s as i64) << 1 | 1) << shift) / ((1 << bits) - 1) - (1 << shift)) as i32;
            }
        }
        if CHANNELS == 2 && self.joint != 0 {
            for sb in 0..SUBBANDS {
                if (self.joint & (1 << sb)) != 0 {
                    let l = sample[0][sb] + sample[1][sb];
                    let r = sample[0][sb] - sample[1][sb];
                    sample[0][sb] = l;
                    sample[1][sb] = r;
                }
            }
        }

        Ok(self.decoder.filter(&sample))
    }
}

impl<'d, 'b, const CHANNELS: usize, const SUBBANDS: usize, B: ByteInput> Iterator
    for FrameDecoder<'d, 'b, CHANNELS, SUBBANDS, B>
where
    Channels<CHANNELS>: ValidChannels,
    Subbands<SUBBANDS>: ValidSubbands,
{
    type Item = [[i16; SUBBANDS]; CHANNELS];

    fn next(&mut self) -> Option<Self::Item> {
        match self.next() {
            Ok(v) => Some(v),
            Err(FrameDecodeError::NoBlock) => None,
            Err(e) => Err(e).unwrap(),
        }
    }
}
