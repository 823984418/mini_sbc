use crate::header::{AllocationMethod, ChannelMode, Frequency, SBCHeader};
use crate::helper;
use crate::table::{
    M_0_195, M_0_382, M_0_555, M_0_707, M_0_831, M_0_923, M_0_980, M_1_000, M_PRORO_4, M_PRORO_8,
};

pub const FILTER_ORDER: usize = 10;

pub struct Channels<const V: usize>;

pub trait ValidChannels {}

impl ValidChannels for Channels<1> {}

impl ValidChannels for Channels<2> {}

pub struct Subbands<const V: usize>;

pub trait ValidSubbands {
    fn offset(frquency: Frequency, sb: usize) -> i8;
    fn decode16(step: usize, v: &mut [[i32; FILTER_ORDER]], s: &[i32], o: &mut [i16]);
}

impl ValidSubbands for Subbands<4> {
    fn offset(frquency: Frequency, sb: usize) -> i8 {
        const SBC_OFFSET4: [[i8; 4]; 4] =
            [[-1, 0, 0, 0], [-2, 0, 0, 1], [-2, 0, 0, 1], [-2, 0, 0, 1]];
        SBC_OFFSET4[frquency as usize][sb]
    }

    fn decode16(step: usize, v: &mut [[i32; FILTER_ORDER]], s: &[i32], o: &mut [i16]) {
        assert!(step < FILTER_ORDER);
        assert_eq!(v.len(), 4);
        assert_eq!(s.len(), 4);
        assert_eq!(o.len(), 4);

        let a03 = s[0] + s[3];
        let s03 = s[0] - s[3];
        let a12 = s[1] + s[2];
        let s12 = s[1] - s[2];

        let v0 = (a03 - a12) * M_0_707; // v[0]
        let v1 = s03 * M_0_382 - s12 * M_0_923; // v[1]
        let _v2 = 0; // 0
        let _v3 = -v1; // -v[1]
        let _v4 = -v0; // -v[0]
        let v5 = -(s03 * M_0_923 + s12 * M_0_382); // v[2]
        let v6 = -((a03 + a12) * M_1_000); // v[3]
        let _v7 = v5; // v[2]

        v[0][step] = v0 >> 15;
        v[1][step] = v1 >> 15;
        v[2][step] = v5 >> 15;
        v[3][step] = v6 >> 15;

        let mut sum = [0_i32; 4];

        let mut i = step;
        for f in 0..FILTER_ORDER {
            if (f & 1) == 0 {
                sum[0] += v[0][i] * M_PRORO_4[f][0];
                sum[1] += v[1][i] * M_PRORO_4[f][1];
                sum[2] += 0 * M_PRORO_4[f][2];
                sum[3] -= v[1][i] * M_PRORO_4[f][3];
            } else {
                sum[0] -= v[0][i] * M_PRORO_4[f][0];
                sum[1] += v[2][i] * M_PRORO_4[f][1];
                sum[2] += v[3][i] * M_PRORO_4[f][2];
                sum[3] += v[2][i] * M_PRORO_4[f][3];
            };
            if i == 0 {
                i = FILTER_ORDER - 1;
            } else {
                i -= 1;
            }
        }
        for sb in 0..4 {
            o[sb] = helper::saturating_i16(sum[sb] >> 15);
        }
    }
}

impl ValidSubbands for Subbands<8> {
    fn offset(frquency: Frequency, sb: usize) -> i8 {
        const SBC_OFFSET8: [[i8; 8]; 4] = [
            [-2, 0, 0, 0, 0, 0, 0, 1],
            [-3, 0, 0, 0, 0, 0, 1, 2],
            [-4, 0, 0, 0, 0, 0, 1, 2],
            [-4, 0, 0, 0, 0, 0, 1, 2],
        ];
        SBC_OFFSET8[frquency as usize][sb]
    }

    fn decode16(step: usize, v: &mut [[i32; FILTER_ORDER]], s: &[i32], o: &mut [i16]) {
        assert!(step < FILTER_ORDER);
        assert_eq!(v.len(), 8);
        assert_eq!(s.len(), 8);
        assert_eq!(o.len(), 8);

        let a07 = s[0] + s[7];
        let a16 = s[1] + s[6];
        let a25 = s[2] + s[5];
        let a34 = s[3] + s[4];
        let s07 = s[0] - s[7];
        let s16 = s[1] - s[6];
        let s25 = s[2] - s[5];
        let s34 = s[3] - s[4];

        let v0 = (a07 - a16 - a25 + a34) * M_0_707; // v[0]
        let v1 = s07 * M_0_555 - s16 * M_0_980 + s25 * M_0_195 + s34 * M_0_831; // v[1]
        let v2 = (a07 - a34) * M_0_382 + (a25 - a16) * M_0_923; // v[2]
        let v3 = s07 * M_0_195 - s16 * M_0_555 + s25 * M_0_831 - s34 * M_0_980; // v[3]
        let _v4 = 0; // 0
        let _v5 = -v3; // -v[3]
        let _v6 = -v2; // -v[2]
        let _v7 = -v1; // -v[1]
        let _v8 = -v0; // -v[0]
        let v9 = -s07 * M_0_831 + s16 * M_0_195 + s25 * M_0_980 + s34 * M_0_555; // v[4]
        let v10 = (a34 - a07) * M_0_923 + (a25 - a16) * M_0_382; // v[5]
        let v11 = -s07 * M_0_980 - s16 * M_0_831 - s25 * M_0_555 - s34 * M_0_195; // v[6]
        let v12 = -(a07 + a16 + a25 + a34) * M_1_000; // v[7]
        let _v13 = v11; // v[6]
        let _v14 = v10; // v[5]
        let _v15 = v9; // v[4]

        v[0][step] = v0 >> 15;
        v[1][step] = v1 >> 15;
        v[2][step] = v2 >> 15;
        v[3][step] = v3 >> 15;
        v[4][step] = v9 >> 15;
        v[5][step] = v10 >> 15;
        v[6][step] = v11 >> 15;
        v[7][step] = v12 >> 15;

        let mut sum = [0_i32; 8];
        let mut f = 0;
        let mut i = step;
        for f in 0..FILTER_ORDER {
            if (f & 1) == 0 {
                sum[0] += v[0][i] * M_PRORO_8[f][0];
                sum[1] += v[1][i] * M_PRORO_8[f][1];
                sum[2] += v[2][i] * M_PRORO_8[f][2];
                sum[3] += v[3][i] * M_PRORO_8[f][3];
                sum[4] += 0 * M_PRORO_8[f][4];
                sum[5] -= v[3][i] * M_PRORO_8[f][5];
                sum[6] -= v[2][i] * M_PRORO_8[f][6];
                sum[7] -= v[1][i] * M_PRORO_8[f][7];
            } else {
                sum[0] -= v[0][i] * M_PRORO_8[f][0];
                sum[1] += v[4][i] * M_PRORO_8[f][1];
                sum[2] += v[5][i] * M_PRORO_8[f][2];
                sum[3] += v[6][i] * M_PRORO_8[f][3];
                sum[4] += v[7][i] * M_PRORO_8[f][4];
                sum[5] += v[6][i] * M_PRORO_8[f][5];
                sum[6] += v[5][i] * M_PRORO_8[f][6];
                sum[7] += v[4][i] * M_PRORO_8[f][7];
            };
            if i == 0 {
                i = FILTER_ORDER - 1;
            }else{
                i -= 1;
            }
        }
        for sb in 0..8 {
            o[sb] = helper::saturating_i16(sum[sb] >> 15);
        }
    }
}

pub(crate) fn calculate_bits<const CHANNELS: usize, const SUBBANDS: usize>(
    header: &SBCHeader,
    scale_factor: &[[u8; SUBBANDS]; CHANNELS],
) -> [[u8; SUBBANDS]; CHANNELS]
where
    Channels<CHANNELS>: ValidChannels,
    Subbands<SUBBANDS>: ValidSubbands,
{
    let mut bits = [[0; SUBBANDS]; CHANNELS];
    match header.channel_mode() {
        ChannelMode::SBC_MODE_MONO | ChannelMode::SBC_MODE_DUAL_CHANNEL => {
            let mut bitneed = [[0; SUBBANDS]; CHANNELS];
            for ch in 0..CHANNELS {
                match header.allocation_method() {
                    AllocationMethod::SBC_AM_SNR => {
                        for sb in 0..SUBBANDS {
                            bitneed[ch][sb] = scale_factor[ch][sb] as i8;
                        }
                    }
                    AllocationMethod::SBC_AM_LOUDNESS => {
                        for sb in 0..SUBBANDS {
                            let loudness = scale_factor[ch][sb] as i8
                                - <Subbands<SUBBANDS> as ValidSubbands>::offset(
                                    header.frequency(),
                                    sb,
                                );
                            bitneed[ch][sb] = if loudness > 0 { loudness / 2 } else { loudness };
                        }
                    }
                }
                let max_bitneed = bitneed[ch].iter().copied().max().unwrap();

                let mut bitcount = 0;
                let mut slicecount = 0;
                let mut bitslice = max_bitneed + 1;
                loop {
                    bitslice -= 1;
                    bitcount += slicecount;
                    slicecount = bitneed[ch]
                        .iter()
                        .copied()
                        .map(|n| match n {
                            n if n > bitslice + 1 && n < bitslice + 16 => 1,
                            n if n == bitslice + 1 => 2,
                            _ => 0,
                        })
                        .sum();
                    if bitcount + slicecount >= header.bitpool() {
                        break;
                    }
                }

                if bitcount + slicecount < header.bitpool() {
                    bitslice -= 1;
                    bitcount += slicecount;
                }

                for sb in 0..SUBBANDS {
                    if bitneed[ch][sb] < bitslice + 2 {
                        bits[ch][sb] = 0;
                    } else {
                        bits[ch][sb] = (bitneed[ch][sb] - bitslice).min(16);
                    }
                }

                for sb in 0..SUBBANDS {
                    if bitcount >= header.bitpool() {
                        break;
                    }
                    if bits[ch][sb] >= 2 && bits[ch][sb] < 16 {
                        bits[ch][sb] += 1;
                        bitcount += 1;
                    } else if bitneed[ch][sb] == bitslice + 1 && header.bitpool() > bitcount + 1 {
                        bits[ch][sb] = 2;
                        bitcount += 2;
                    }
                }
                for sb in 0..SUBBANDS {
                    if bitcount >= header.bitpool() {
                        break;
                    }
                    if bits[ch][sb] < 16 {
                        bits[ch][sb] += 1;
                        bitcount += 1;
                    }
                }
            }
        }
        ChannelMode::SBC_MODE_STEREO | ChannelMode::SBC_MODE_JOINT_STEREO => {
            let mut bitneed = [[0; SUBBANDS]; CHANNELS];
            match header.allocation_method() {
                AllocationMethod::SBC_AM_SNR => {
                    for ch in 0..CHANNELS {
                        for sb in 0..SUBBANDS {
                            bitneed[ch][sb] = scale_factor[ch][sb] as i8;
                        }
                    }
                }
                AllocationMethod::SBC_AM_LOUDNESS => {
                    for ch in 0..CHANNELS {
                        for sb in 0..SUBBANDS {
                            let loudness = scale_factor[ch][sb] as i8
                                - <Subbands<SUBBANDS> as ValidSubbands>::offset(
                                    header.frequency(),
                                    sb,
                                );
                            bitneed[ch][sb] = if loudness > 0 { loudness / 2 } else { loudness };
                        }
                    }
                }
            }
            let max_bitneed = bitneed.iter().flatten().copied().max().unwrap();

            let mut bitcount = 0;
            let mut slicecount = 0;
            let mut bitslice = max_bitneed + 1;
            loop {
                bitslice -= 1;
                bitcount += slicecount;
                for ch in 0..CHANNELS {
                    slicecount = bitneed[ch]
                        .iter()
                        .copied()
                        .map(|n| match n {
                            n if n > bitslice + 1 && n < bitslice + 16 => 1,
                            n if n == bitslice + 1 => 2,
                            _ => 0,
                        })
                        .sum();
                }
                if bitcount + slicecount >= header.bitpool() {
                    break;
                }
            }

            if bitcount + slicecount < header.bitpool() {
                bitslice -= 1;
                bitcount += slicecount;
            }

            for ch in 0..CHANNELS {
                for sb in 0..SUBBANDS {
                    if bitneed[ch][sb] < bitslice + 2 {
                        bits[ch][sb] = 0;
                    } else {
                        bits[ch][sb] = (bitneed[ch][sb] - bitslice).min(16);
                    }
                }
            }
            for sb in 0..SUBBANDS {
                for ch in 0..CHANNELS {
                    if bitcount >= header.bitpool() {
                        break;
                    }
                    if bits[ch][sb] >= 2 && bits[ch][sb] < 16 {
                        bits[ch][sb] += 1;
                        bitcount += 1;
                    } else if bitneed[ch][sb] == bitslice + 1 && header.bitpool() > bitcount + 1 {
                        bits[ch][sb] = 2;
                        bitcount += 2;
                    }
                }
            }

            for sb in 0..SUBBANDS {
                for ch in 0..CHANNELS {
                    if bitcount >= header.bitpool() {
                        break;
                    }
                    if bits[ch][sb] < 16 {
                        bits[ch][sb] += 1;
                        bitcount += 1;
                    }
                }
            }
        }
    }

    let mut r = [[0; SUBBANDS]; CHANNELS];
    for ch in 0..CHANNELS {
        for sb in 0..SUBBANDS {
            r[ch][sb] = bits[ch][sb] as u8;
        }
    }
    r
}
