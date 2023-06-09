use crate::filter_state::FilterState;
use crate::frame_decoder::FrameDecoder;
use crate::header::SBCHeader;

const DATA: [u8; 576] = [
    0x9C, 0x00, 0x18, 0xC1, 0xA9, 0x76, 0x7F, 0x7D, 0xEE, 0x83, 0x82, 0x0D, 0x82, 0x5D, 0x81, 0xE3,
    0xD0, 0xC9, 0x9C, 0x00, 0x18, 0xA9, 0xDD, 0xDD, 0x80, 0xD3, 0xCE, 0xF9, 0x43, 0xCF, 0xB0, 0x23,
    0x96, 0xA9, 0xB1, 0xBA, 0x9C, 0x00, 0x18, 0xCE, 0xDE, 0xBE, 0x25, 0x1C, 0x22, 0xB9, 0x00, 0x8E,
    0x3A, 0xF4, 0x1F, 0x8E, 0x39, 0x9E, 0x9C, 0x00, 0x18, 0x5A, 0xDE, 0xDE, 0x2C, 0xB9, 0xAF, 0xB7,
    0x2C, 0x23, 0xC2, 0xD4, 0xE7, 0x62, 0x55, 0x93, 0x9C, 0x00, 0x18, 0x5A, 0xDE, 0xDE, 0xAA, 0x9B,
    0x9B, 0x53, 0x35, 0x99, 0xA2, 0xDB, 0xF1, 0x1F, 0x8E, 0x67, 0x9C, 0x00, 0x18, 0xB2, 0xED, 0xBB,
    0x4F, 0x0F, 0x48, 0x7A, 0x95, 0xE3, 0x93, 0x55, 0xF3, 0xC4, 0xA1, 0x2E, 0x9C, 0x00, 0x18, 0x64,
    0xDD, 0xCD, 0x92, 0x49, 0x26, 0x98, 0x4B, 0x64, 0x37, 0x86, 0xD2, 0xA4, 0xB3, 0x7C, 0x9C, 0x00,
    0x18, 0x37, 0xDE, 0xED, 0x4F, 0x95, 0x50, 0x76, 0xB3, 0xCB, 0x3C, 0x96, 0x02, 0xD7, 0x15, 0x2A,
    0x9C, 0x00, 0x18, 0xE2, 0xBE, 0xDD, 0xE6, 0xD7, 0xAD, 0x7E, 0x2B, 0x99, 0x98, 0xCE, 0xD5, 0x7A,
    0x7B, 0x4E, 0x9C, 0x00, 0x18, 0x97, 0xDE, 0xCE, 0x6A, 0x9F, 0x4E, 0x39, 0x91, 0xDA, 0x09, 0x09,
    0xA2, 0xA4, 0xFB, 0x61, 0x9C, 0x00, 0x18, 0xBC, 0xEE, 0xDD, 0x95, 0x01, 0xEE, 0xBF, 0x11, 0xF1,
    0x89, 0x42, 0x9A, 0x76, 0x67, 0x39, 0x9C, 0x00, 0x18, 0xC9, 0xCE, 0xDD, 0x52, 0x7B, 0x65, 0x71,
    0x56, 0x27, 0x63, 0x42, 0xCC, 0x23, 0x06, 0xD9, 0x9C, 0x00, 0x18, 0x7D, 0xDE, 0xDD, 0x63, 0x01,
    0xF7, 0xE4, 0xE2, 0x37, 0x71, 0x5C, 0x6A, 0xA5, 0xC6, 0x69, 0x9C, 0x00, 0x18, 0xA9, 0xDD, 0xDD,
    0x58, 0x81, 0xE6, 0xAB, 0x5C, 0x37, 0x34, 0x2A, 0x1D, 0x44, 0xE2, 0x32, 0x9C, 0x00, 0x18, 0x20,
    0xDD, 0xBC, 0x0F, 0x93, 0x4E, 0xBC, 0xC3, 0x9C, 0xC5, 0x4C, 0x64, 0xDC, 0xB7, 0x4C, 0x9C, 0x00,
    0x18, 0x7D, 0xDE, 0xDD, 0xA5, 0xB2, 0x84, 0x3A, 0xB9, 0x48, 0x9D, 0x26, 0x1E, 0x50, 0x98, 0xF2,
    0x9C, 0x00, 0x18, 0xA9, 0xDD, 0xDD, 0x88, 0xA3, 0x93, 0x33, 0xC9, 0x60, 0x93, 0x78, 0xCA, 0xD8,
    0x40, 0x05, 0x9C, 0x00, 0x18, 0x64, 0xDD, 0xCD, 0x5C, 0x3A, 0x59, 0xBB, 0xD0, 0x73, 0x3F, 0x32,
    0x59, 0xBD, 0x1A, 0x81, 0x9C, 0x00, 0x18, 0xB0, 0xDE, 0xCD, 0x1C, 0xF9, 0x44, 0x36, 0x67, 0x6E,
    0x39, 0x36, 0x93, 0xB8, 0xC9, 0xEF, 0x9C, 0x00, 0x18, 0xA9, 0xDD, 0xDD, 0xE9, 0x53, 0xEE, 0xB0,
    0x1B, 0x96, 0xB1, 0xB1, 0xBA, 0x1F, 0x3A, 0xF2, 0x9C, 0x00, 0x18, 0x5A, 0xDE, 0xDE, 0xB7, 0x03,
    0x4E, 0x42, 0xF2, 0xDF, 0x86, 0x33, 0x9E, 0x2C, 0xB9, 0xAF, 0x9C, 0x00, 0x18, 0xB0, 0xDE, 0xCD,
    0xBF, 0x2E, 0x33, 0xBA, 0xD6, 0xF7, 0x62, 0x57, 0xA3, 0xB2, 0x9D, 0x8B, 0x9C, 0x00, 0x18, 0x5A,
    0xDE, 0xDE, 0x49, 0x35, 0x99, 0xA4, 0xDB, 0xF1, 0x27, 0x8E, 0x67, 0x16, 0x8C, 0x9D, 0x9C, 0x00,
    0x18, 0xA5, 0xED, 0xCD, 0x7B, 0x2B, 0xEC, 0x96, 0xAB, 0xF0, 0xBF, 0x43, 0x2F, 0x8A, 0x49, 0x26,
    0x9C, 0x00, 0x18, 0x7D, 0xDE, 0xDD, 0x9E, 0xA5, 0xC4, 0x2D, 0x46, 0xB2, 0xA8, 0xD9, 0xDC, 0x55,
    0x97, 0x50, 0x9C, 0x00, 0x18, 0xE3, 0xDD, 0xED, 0x6C, 0x6B, 0xCB, 0x42, 0x2E, 0x02, 0xDD, 0x2D,
    0x2A, 0x8F, 0xCD, 0xCD, 0x9C, 0x00, 0x18, 0x71, 0xAE, 0xDE, 0xA4, 0x67, 0x29, 0xD1, 0x9D, 0xA5,
    0x24, 0xF6, 0x9E, 0x0A, 0x8B, 0x4E, 0x9C, 0x00, 0x18, 0x6C, 0xEE, 0xCC, 0x5D, 0x98, 0xE4, 0x3F,
    0x0C, 0xD6, 0x95, 0x01, 0xB3, 0x97, 0x01, 0xEC, 0x9C, 0x00, 0x18, 0x7D, 0xDE, 0xDD, 0xF7, 0x11,
    0xF1, 0x9B, 0x42, 0x9A, 0x6E, 0x67, 0x39, 0x5F, 0x3D, 0xA5, 0x9C, 0x00, 0x18, 0xC9, 0xCE, 0xDD,
    0x81, 0x56, 0x27, 0x67, 0x42, 0xCC, 0x0F, 0x06, 0xD9, 0x56, 0x03, 0xD7, 0x9C, 0x00, 0x18, 0x7D,
    0xDE, 0xDD, 0xE4, 0xE2, 0x37, 0x69, 0x5C, 0x6A, 0xAD, 0xC6, 0x69, 0x56, 0xC0, 0xE6, 0x9C, 0x00,
    0x18, 0x64, 0xDD, 0xCD, 0xA3, 0x5C, 0x77, 0x3E, 0x28, 0x3D, 0x42, 0xE0, 0x92, 0x07, 0x93, 0xAB,
];

#[test]
fn test1() {
    let data = &mut &DATA[..];
    let mut decoder = FilterState::<1, 4>::new();
    for _i in 0..32 {
        let h = SBCHeader::decode(data).unwrap();
        let frame = FrameDecoder::new(&h, &mut decoder, data).unwrap();
        for x in frame {
            for i in x {
                for j in i {
                    println!("{}", j);
                }
            }
        }
    }
}

const DATA2: [u8; 451] = [
    0x9C, 0x00, 0x18, 0xA9, 0x76, 0x7F, 0x7D, 0xEE, 0x83, 0x82, 0x0D, 0x82, 0x5D, 0x81, 0xE3, 0xD0,
    0xC9, 0xDD, 0xDD, 0x80, 0xD3, 0xCE, 0xF9, 0x43, 0xCF, 0xB0, 0x23, 0x96, 0xA9, 0xB1, 0xBA, 0xDE,
    0xBE, 0x25, 0x1C, 0x22, 0xB9, 0x00, 0x8E, 0x3A, 0xF4, 0x1F, 0x8E, 0x39, 0x9E, 0xDE, 0xDE, 0x2C,
    0xB9, 0xAF, 0xB7, 0x2C, 0x23, 0xC2, 0xD4, 0xE7, 0x62, 0x55, 0x93, 0xDE, 0xDE, 0xAA, 0x9B, 0x9B,
    0x53, 0x35, 0x99, 0xA2, 0xDB, 0xF1, 0x1F, 0x8E, 0x67, 0xED, 0xBB, 0x4F, 0x0F, 0x48, 0x7A, 0x95,
    0xE3, 0x93, 0x55, 0xF3, 0xC4, 0xA1, 0x2E, 0xDD, 0xCD, 0x92, 0x49, 0x26, 0x98, 0x4B, 0x64, 0x37,
    0x86, 0xD2, 0xA4, 0xB3, 0x7C, 0xDE, 0xED, 0x4F, 0x95, 0x50, 0x76, 0xB3, 0xCB, 0x3C, 0x96, 0x02,
    0xD7, 0x15, 0x2A, 0xBE, 0xDD, 0xE6, 0xD7, 0xAD, 0x7E, 0x2B, 0x99, 0x98, 0xCE, 0xD5, 0x7A, 0x7B,
    0x4E, 0xDE, 0xCE, 0x6A, 0x9F, 0x4E, 0x39, 0x91, 0xDA, 0x09, 0x09, 0xA2, 0xA4, 0xFB, 0x61, 0xEE,
    0xDD, 0x95, 0x01, 0xEE, 0xBF, 0x11, 0xF1, 0x89, 0x42, 0x9A, 0x76, 0x67, 0x39, 0xCE, 0xDD, 0x52,
    0x7B, 0x65, 0x71, 0x56, 0x27, 0x63, 0x42, 0xCC, 0x23, 0x06, 0xD9, 0xDE, 0xDD, 0x63, 0x01, 0xF7,
    0xE4, 0xE2, 0x37, 0x71, 0x5C, 0x6A, 0xA5, 0xC6, 0x69, 0xDD, 0xDD, 0x58, 0x81, 0xE6, 0xAB, 0x5C,
    0x37, 0x34, 0x2A, 0x1D, 0x44, 0xE2, 0x32, 0xDD, 0xBC, 0x0F, 0x93, 0x4E, 0xBC, 0xC3, 0x9C, 0xC5,
    0x4C, 0x64, 0xDC, 0xB7, 0x4C, 0xDE, 0xDD, 0xA5, 0xB2, 0x84, 0x3A, 0xB9, 0x48, 0x9D, 0x26, 0x1E,
    0x50, 0x98, 0xF2, 0xDD, 0xDD, 0x88, 0xA3, 0x93, 0x33, 0xC9, 0x60, 0x93, 0x78, 0xCA, 0xD8, 0x40,
    0x05, 0xDD, 0xCD, 0x5C, 0x3A, 0x59, 0xBB, 0xD0, 0x73, 0x3F, 0x32, 0x59, 0xBD, 0x1A, 0x81, 0xDE,
    0xCD, 0x1C, 0xF9, 0x44, 0x36, 0x67, 0x6E, 0x39, 0x36, 0x93, 0xB8, 0xC9, 0xEF, 0xDD, 0xDD, 0xE9,
    0x53, 0xEE, 0xB0, 0x1B, 0x96, 0xB1, 0xB1, 0xBA, 0x1F, 0x3A, 0xF2, 0xDE, 0xDE, 0xB7, 0x03, 0x4E,
    0x42, 0xF2, 0xDF, 0x86, 0x33, 0x9E, 0x2C, 0xB9, 0xAF, 0xDE, 0xCD, 0xBF, 0x2E, 0x33, 0xBA, 0xD6,
    0xF7, 0x62, 0x57, 0xA3, 0xB2, 0x9D, 0x8B, 0xDE, 0xDE, 0x49, 0x35, 0x99, 0xA4, 0xDB, 0xF1, 0x27,
    0x8E, 0x67, 0x16, 0x8C, 0x9D, 0xED, 0xCD, 0x7B, 0x2B, 0xEC, 0x96, 0xAB, 0xF0, 0xBF, 0x43, 0x2F,
    0x8A, 0x49, 0x26, 0xDE, 0xDD, 0x9E, 0xA5, 0xC4, 0x2D, 0x46, 0xB2, 0xA8, 0xD9, 0xDC, 0x55, 0x97,
    0x50, 0xDD, 0xED, 0x6C, 0x6B, 0xCB, 0x42, 0x2E, 0x02, 0xDD, 0x2D, 0x2A, 0x8F, 0xCD, 0xCD, 0xAE,
    0xDE, 0xA4, 0x67, 0x29, 0xD1, 0x9D, 0xA5, 0x24, 0xF6, 0x9E, 0x0A, 0x8B, 0x4E, 0xEE, 0xCC, 0x5D,
    0x98, 0xE4, 0x3F, 0x0C, 0xD6, 0x95, 0x01, 0xB3, 0x97, 0x01, 0xEC, 0xDE, 0xDD, 0xF7, 0x11, 0xF1,
    0x9B, 0x42, 0x9A, 0x6E, 0x67, 0x39, 0x5F, 0x3D, 0xA5, 0xCE, 0xDD, 0x81, 0x56, 0x27, 0x67, 0x42,
    0xCC, 0x0F, 0x06, 0xD9, 0x56, 0x03, 0xD7, 0xDE, 0xDD, 0xE4, 0xE2, 0x37, 0x69, 0x5C, 0x6A, 0xAD,
    0xC6, 0x69, 0x56, 0xC0, 0xE6, 0xDD, 0xCD, 0xA3, 0x5C, 0x77, 0x3E, 0x28, 0x3D, 0x42, 0xE0, 0x92,
    0x07, 0x93, 0xAB,
];

#[test]
fn test2() {
    let data = &mut &DATA2[..];
    let mut decoder = FilterState::<1, 4>::new();
    let h = SBCHeader::decode(data).unwrap();
    for _i in 0..32 {
        let frame = FrameDecoder::new_no_crc(&h, &mut decoder, data).unwrap();
        for x in frame {
            for i in x {
                for j in i {
                    println!("{}", j);
                }
            }
        }
    }
}
