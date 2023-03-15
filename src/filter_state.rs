use crate::sbc::FILTER_ORDER;
use crate::sbc::{Channels, Subbands, ValidChannels, ValidSubbands};
use core::fmt::{Debug, Formatter};

pub struct FilterState<const CHANNELS: usize, const SUBBANDS: usize>
where
    Channels<CHANNELS>: ValidChannels,
    Subbands<SUBBANDS>: ValidSubbands,
{
    filter_state: [[[i32; FILTER_ORDER]; SUBBANDS]; CHANNELS],
    step: u8,
}

impl<const CHANNELS: usize, const SUBBANDS: usize> FilterState<CHANNELS, SUBBANDS>
where
    Channels<CHANNELS>: ValidChannels,
    Subbands<SUBBANDS>: ValidSubbands,
{
    pub const fn new() -> Self {
        Self {
            filter_state: [[[0; FILTER_ORDER]; SUBBANDS]; CHANNELS],
            step: 0,
        }
    }

    pub fn filter(&mut self, s: &[[i32; SUBBANDS]; CHANNELS]) -> [[i16; SUBBANDS]; CHANNELS] {
        let step = self.step as usize;

        let mut o = [[0; SUBBANDS]; CHANNELS];
        for ch in 0..CHANNELS {
            <Subbands<SUBBANDS> as ValidSubbands>::decode16(
                step,
                &mut self.filter_state[ch],
                &s[ch],
                &mut o[ch],
            );
        }

        if step + 1 == FILTER_ORDER {
            self.step = 0;
        } else {
            self.step = (step + 1) as u8;
        }
        o
    }
}

impl<const CHANNELS: usize, const SUBBANDS: usize> Debug for FilterState<CHANNELS, SUBBANDS>
where
    Channels<CHANNELS>: ValidChannels,
    Subbands<SUBBANDS>: ValidSubbands,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut s = f.debug_struct("Decoder");
        s.field("CHANNELS", &CHANNELS);
        s.field("SUBBANDS", &SUBBANDS);
        s.field("filter_state", &self.filter_state);
        s.field("step", &self.step);
        s.finish()
    }
}
