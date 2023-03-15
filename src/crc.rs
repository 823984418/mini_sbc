use crate::const_for;

const CRC_POLY: u8 = 0x1D;

const CRC_TABLE: [u8; 256] = crc_table(CRC_POLY);

const fn crc_table(poly: u8) -> [u8; 256] {
    let mut table = [0; 256];
    const_for!(i in (0, 256) {
        table[i] = {
            let i = i as u8;
            let mut crc = i;
            const_for!(_j in (0, 8) {
                if (crc & 0x80) != 0 {
                    crc = (crc << 1) ^ poly;
                } else {
                    crc = crc << 1;
                }
            });
            crc
        };
    });
    table
}

pub const fn crc8(init: u8, data: &[u8], bits: usize) -> u8 {
    assert!(data.len() * 8 >= bits);

    let mut crc = init;
    const_for!(i in (0, bits / 8) {
        crc = CRC_TABLE[(crc ^ data[i]) as usize];
    });

    let bits = bits % 8;
    if bits != 0 {
        crc ^= data[bits / 8] & (0xFF << (8 - bits));
        const_for!(_i in (0, bits) {
            if (crc & 0x80) != 0 {
                crc = (crc << 1) ^ CRC_POLY;
            } else {
                crc = crc << 1;
            }
        });
    }
    return crc;
}
