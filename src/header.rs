use crate::io::{ByteError, ByteInput, ByteOutput};

pub const SBC_SYNCWORD: u8 = 0x9C;
pub const MSBC_SYNCWORD: u8 = 0xAD;

pub const MSBC_BLOCKS: usize = 15;

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Copy, Clone)]
pub enum Frequency {
    #[default]
    SBC_FREQ_16000 = 0,
    SBC_FREQ_32000 = 1,
    SBC_FREQ_44100 = 2,
    SBC_FREQ_48000 = 3,
}

impl Frequency {
    pub const fn decode(i: u8) -> Self {
        match (i >> 6) & 3 {
            0 => Self::SBC_FREQ_16000,
            1 => Self::SBC_FREQ_32000,
            2 => Self::SBC_FREQ_44100,
            3 => Self::SBC_FREQ_48000,
            _ => unreachable!(),
        }
    }

    pub const fn encode(self) -> u8 {
        (self as u8) << 6
    }

    pub const fn frequency(self) -> u16 {
        match self {
            Self::SBC_FREQ_16000 => 16000,
            Self::SBC_FREQ_32000 => 32000,
            Self::SBC_FREQ_44100 => 44100,
            Self::SBC_FREQ_48000 => 48000,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Copy, Clone)]
pub enum Blocks {
    #[default]
    SBC_BLK_4 = 0,
    SBC_BLK_8 = 1,
    SBC_BLK_12 = 2,
    SBC_BLK_16 = 3,
}

impl Blocks {
    pub const fn decode(i: u8) -> Self {
        match (i >> 4) & 3 {
            0 => Self::SBC_BLK_4,
            1 => Self::SBC_BLK_8,
            2 => Self::SBC_BLK_12,
            3 => Self::SBC_BLK_16,
            _ => unreachable!(),
        }
    }

    pub const fn encode(self) -> u8 {
        (self as u8) << 4
    }

    pub const fn blocks(self) -> usize {
        match self {
            Blocks::SBC_BLK_4 => 4,
            Blocks::SBC_BLK_8 => 8,
            Blocks::SBC_BLK_12 => 12,
            Blocks::SBC_BLK_16 => 16,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Copy, Clone)]
pub enum ChannelMode {
    #[default]
    SBC_MODE_MONO = 0,
    SBC_MODE_DUAL_CHANNEL = 1,
    SBC_MODE_STEREO = 2,
    SBC_MODE_JOINT_STEREO = 3,
}

impl ChannelMode {
    pub const fn decode(i: u8) -> Self {
        match (i >> 2) & 3 {
            0 => Self::SBC_MODE_MONO,
            1 => Self::SBC_MODE_DUAL_CHANNEL,
            2 => Self::SBC_MODE_STEREO,
            3 => Self::SBC_MODE_JOINT_STEREO,
            _ => unreachable!(),
        }
    }

    pub const fn encode(self) -> u8 {
        (self as u8) << 2
    }

    pub fn channels(self) -> usize {
        match self {
            Self::SBC_MODE_MONO => 1,
            Self::SBC_MODE_DUAL_CHANNEL => 2,
            Self::SBC_MODE_STEREO => 2,
            Self::SBC_MODE_JOINT_STEREO => 2,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Copy, Clone)]
pub enum AllocationMethod {
    #[default]
    SBC_AM_LOUDNESS = 0,
    SBC_AM_SNR = 1,
}

impl AllocationMethod {
    pub const fn decode(i: u8) -> Self {
        match (i >> 1) & 1 {
            0 => Self::SBC_AM_LOUDNESS,
            1 => Self::SBC_AM_SNR,
            _ => unreachable!(),
        }
    }

    pub const fn encode(self) -> u8 {
        (self as u8) << 1
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Copy, Clone)]
pub enum Subbands {
    #[default]
    SBC_SB_4 = 0,
    SBC_SB_8 = 1,
}

impl Subbands {
    pub const fn decode(i: u8) -> Self {
        match i & 1 {
            0 => Self::SBC_SB_4,
            1 => Self::SBC_SB_8,
            _ => unreachable!(),
        }
    }

    pub const fn encode(self) -> u8 {
        self as u8
    }

    pub const fn subbands(self) -> usize {
        match self {
            Subbands::SBC_SB_4 => 4,
            Subbands::SBC_SB_8 => 8,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum SBCHeader {
    #[default]
    MSBC,
    SBC {
        frequency: Frequency,
        blocks: Blocks,
        channel_mode: ChannelMode,
        allocation_method: AllocationMethod,
        subbands: Subbands,
        bitpool: u8,
    },
}

impl SBCHeader {
    pub const fn decode_array(array: &[u8; 3]) -> Option<Self> {
        match array[0] {
            SBC_SYNCWORD => Some(Self::SBC {
                frequency: Frequency::decode(array[1]),
                blocks: Blocks::decode(array[1]),
                channel_mode: ChannelMode::decode(array[1]),
                allocation_method: AllocationMethod::decode(array[1]),
                subbands: Subbands::decode(array[1]),
                bitpool: array[2],
            }),
            MSBC_SYNCWORD => Some(Self::MSBC),
            _ => None,
        }
    }

    pub const fn encode_array(&self) -> [u8; 3] {
        match *self {
            SBCHeader::MSBC => [MSBC_SYNCWORD, 0, 0],
            SBCHeader::SBC {
                frequency,
                blocks,
                channel_mode,
                allocation_method,
                subbands,
                bitpool,
            } => [
                SBC_SYNCWORD,
                frequency.encode()
                    | blocks.encode()
                    | channel_mode.encode()
                    | allocation_method.encode()
                    | subbands.encode(),
                bitpool,
            ],
        }
    }

    pub fn decode<B: ByteInput>(input: &mut B) -> Result<Self, ByteError> {
        let mut data = [0; 3];
        input.read(&mut data)?;
        Self::decode_array(&data).ok_or(())
    }

    pub fn encode<B: ByteOutput>(&self, output: &mut B) -> Result<(), ByteError> {
        output.write(&self.encode_array())
    }

    pub const fn frequency(&self) -> Frequency {
        match *self {
            SBCHeader::MSBC => Frequency::SBC_FREQ_16000,
            SBCHeader::SBC { frequency, .. } => frequency,
        }
    }
    pub const fn channel_mode(&self) -> ChannelMode {
        match *self {
            SBCHeader::MSBC => ChannelMode::SBC_MODE_MONO,
            SBCHeader::SBC { channel_mode, .. } => channel_mode,
        }
    }
    pub const fn subbands(&self) -> Subbands {
        match *self {
            SBCHeader::MSBC => Subbands::SBC_SB_8,
            SBCHeader::SBC { subbands, .. } => subbands,
        }
    }
    pub const fn blocks(&self) -> Option<Blocks> {
        match *self {
            SBCHeader::MSBC => None,
            SBCHeader::SBC { blocks, .. } => Some(blocks),
        }
    }
    pub const fn allocation_method(&self) -> AllocationMethod {
        match *self {
            SBCHeader::MSBC => AllocationMethod::SBC_AM_LOUDNESS,
            SBCHeader::SBC {
                allocation_method, ..
            } => allocation_method,
        }
    }
    pub const fn bitpool(&self) -> u8 {
        match *self {
            SBCHeader::MSBC => 24,
            SBCHeader::SBC { bitpool, .. } => bitpool,
        }
    }
}
