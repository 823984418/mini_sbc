use crate::const_for;
use crate::helper::round64;
use crate::sbc::FILTER_ORDER;

#[test]
fn gen_con_n_pi_d16() {
    println!("const COS_N_PI_D16: [i64; 9] = {{");
    println!("    [");
    for i in 0..9 {
        let f = f64::cos(i as f64 * (core::f64::consts::PI / 16.0));
        let v = (f * ((1_i64 << 62) as f64)) as i64;
        println!(
            "        {:#018X}, // = cos({} * pi / 16) = {:10.8}",
            v, i, f
        );
    }
    println!("    ]");
    println!("}};");
}

const COS_N_PI_D16: [i64; 9] = {
    [
        0x4000000000000000, // = cos(0 * pi / 16) = 1.00000000
        0x3EC52F9FEEB96000, // = cos(1 * pi / 16) = 0.98078528
        0x3B20D79E651A8C00, // = cos(2 * pi / 16) = 0.92387953
        0x3536CC521D434600, // = cos(3 * pi / 16) = 0.83146961
        0x2D413CCCFE779A00, // = cos(4 * pi / 16) = 0.70710678
        0x238E76735CD19200, // = cos(5 * pi / 16) = 0.55557023
        0x187DE2A6AEA96400, // = cos(6 * pi / 16) = 0.38268343
        0x0C7C5C1E34D30680, // = cos(7 * pi / 16) = 0.19509032
        0x000000000000011A, // = cos(8 * pi / 16) = 0.00000000
    ]
};

#[test]
fn show_cos_n_pi_d16() {
    for i in 0..9 {
        let v = round64(COS_N_PI_D16[i], 32 + 3) as i32;
        print!("{:#010X}, ", v);
        println!("{:#010X}, ", -v);
    }
}

const M_COS_N_PI_D16: [i32; 9] = {
    let mut v = [0; 9];
    const_for!(i in (0, 9) {
        v[i] = round64(COS_N_PI_D16[i], 32 + 3 + 11 + 1 + 2) as i32;
    });
    v
};

#[test]
fn show_m_cos_n_pi_d16() {
    for i in 0..9 {
        let v = M_COS_N_PI_D16[i];
        print!("{:#010X}, ", v);
        println!("{:#010X}, ", -v);
    }
}

pub(crate) const M_1_000: i32 = M_COS_N_PI_D16[0];
pub(crate) const M_0_980: i32 = M_COS_N_PI_D16[1];
pub(crate) const M_0_923: i32 = M_COS_N_PI_D16[2];
pub(crate) const M_0_831: i32 = M_COS_N_PI_D16[3];
pub(crate) const M_0_707: i32 = M_COS_N_PI_D16[4];
pub(crate) const M_0_555: i32 = M_COS_N_PI_D16[5];
pub(crate) const M_0_382: i32 = M_COS_N_PI_D16[6];
pub(crate) const M_0_195: i32 = M_COS_N_PI_D16[7];

#[test]
fn show_syn_matrix4() {
    for n in 0..8 {
        print!("{} ", n);
        for m in 0..4 {
            let x = (2 * m + 1) * (n + 2);
            print!(
                "{:11.8}, ",
                f64::cos(x as f64 * (core::f64::consts::PI / 8.0))
            );
        }
        println!();
    }
    println!();
    for n in 0..8 {
        for m in 0..4 {
            let x = (2 * m + 1) * (n + 2);
            print!("cos({:-3} * pi / 8), ", x);
        }
        println!();
    }
}

#[test]
fn show_syn_matrix8() {
    for n in 0..16 {
        print!("{:-2} ", n);
        for m in 0..8 {
            let x = (2 * m + 1) * (n + 4);
            print!(
                "{:11.8}, ",
                f64::cos(x as f64 * (core::f64::consts::PI / 16.0))
            );
        }
        println!();
    }
    println!();
    for n in 0..16 {
        for m in 0..8 {
            let x = (2 * m + 1) * (n + 4);
            print!("cos({:-3} * pi / 16), ", x);
        }
        println!();
    }
}

const F_PROTO_4: [[&'static str; 4]; FILTER_ORDER] = {
    [
        [
            "+0.00000000E+00",
            "+5.36548976E-04",
            "+1.49188357E-03",
            "+2.73370904E-03",
        ],
        [
            "+3.83720193E-03",
            "+3.89205149E-03",
            "+1.86581691E-03",
            "-3.06012286E-03",
        ],
        [
            "+1.09137620E-02",
            "+2.04385087E-02",
            "+2.88757392E-02",
            "+3.21939290E-02",
        ],
        [
            "+2.58767811E-02",
            "+6.13245186E-03",
            "-2.88217274E-02",
            "-7.76463494E-02",
        ],
        [
            "+1.35593274E-01",
            "+1.94987841E-01",
            "+2.46636662E-01",
            "+2.81828203E-01",
        ],
        [
            "+2.94315332E-01",
            "+2.81828203E-01",
            "+2.46636662E-01",
            "+1.94987841E-01",
        ],
        [
            "-1.35593274E-01",
            "-7.76463494E-02",
            "-2.88217274E-02",
            "+6.13245186E-03",
        ],
        [
            "+2.58767811E-02",
            "+3.21939290E-02",
            "+2.88757392E-02",
            "+2.04385087E-02",
        ],
        [
            "-1.09137620E-02",
            "-3.06012286E-03",
            "+1.86581691E-03",
            "+3.89205149E-03",
        ],
        [
            "+3.83720193E-03",
            "+2.73370904E-03",
            "+1.49188357E-03",
            "+5.36548976E-04",
        ],
    ]
};

#[test]
fn gen_proto_4() {
    let mut array = [[0.0; FILTER_ORDER]; 4];
    for sb in 0..4 {
        for i in 0..FILTER_ORDER {
            array[sb][i] = F_PROTO_4[i][sb].parse::<f64>().unwrap();
        }
    }

    println!("#[allow(overflowing_literals)]");
    println!("const PROTO_4: [[i64; 4]; FILTER_ORDER] = {{");
    println!("    [");
    for i in 0..FILTER_ORDER {
        print!("        [");
        for sb in 0..4 {
            let v = array[sb][i] * -4.0;
            print!("{:#018X}", (v * ((1_i64 << 62) as f64)) as i64);
            if sb != 3 {
                print!(", ");
            }
        }
        println!("],");
    }
    println!("    ]");
    println!("}};");
}

#[allow(overflowing_literals)]
const PROTO_4: [[i64; 4]; FILTER_ORDER] = {
    [
        [
            0x0000000000000000,
            0xFFDCD633B2044610,
            0xFF9E3A58DB7F3D5C,
            0xFF4CD7F913904A50,
        ],
        [
            0xFF04866F33D72A70,
            0xFF00EE3622D79378,
            0xFF85B8C9642766E0,
            0x00C88C579AFE3EF8,
        ],
        [
            0xFD34C17522385B80,
            0xFAC48AB98DF2F5C0,
            0xF89B997C7CF90600,
            0xF7C2237FD0B47C80,
        ],
        [
            0xF96023A77322C1C0,
            0xFE6E1A87D12747F0,
            0x0760DC5898E3E040,
            0x13E0A19353A87500,
        ],
        [
            0xDD49C25A9CC86600,
            0xCE1546DFC98FE800,
            0xC0DC6B72B7241000,
            0xB7DA1B5D0648F800,
        ],
        [
            0xB4A7C01A593F0400,
            0xB7DA1B5D0648F800,
            0xC0DC6B72B7241000,
            0xCE1546DFC98FE800,
        ],
        [
            0x22B63DA563379A00,
            0x13E0A19353A87500,
            0x0760DC5898E3E040,
            0xFE6E1A87D12747F0,
        ],
        [
            0xF96023A77322C1C0,
            0xF7C2237FD0B47C80,
            0xF89B997C7CF90600,
            0xFAC48AB98DF2F5C0,
        ],
        [
            0x02CB3E8ADDC7A480,
            0x00C88C579AFE3EF8,
            0xFF85B8C9642766E0,
            0xFF00EE3622D79378,
        ],
        [
            0xFF04866F33D72A70,
            0xFF4CD7F913904A50,
            0xFF9E3A58DB7F3D5C,
            0xFFDCD633B2044610,
        ],
    ]
};

#[test]
fn show_proto_4() {
    for i in 0..FILTER_ORDER {
        for sb in 0..4 {
            let v = PROTO_4[i][sb];
            print!("{:#010X}, ", round64(v, 32 + 3) as i32);
        }
        println!();
    }
}

pub(crate) const M_PRORO_4: [[i32; 4]; FILTER_ORDER] = {
    let mut v = [[0; 4]; FILTER_ORDER];
    const_for!(i in (0, FILTER_ORDER) {
        const_for!(sb in (0, 4) {
            v[i][sb] = round64(PROTO_4[i][sb], 32 + 3 + 12) as i32;
        });
    });
    v
};

const F_PROTO_8: [[&'static str; 8]; FILTER_ORDER] = {
    [
        [
            "+0.00000000E+00",
            "+1.56575398E-04",
            "+3.43256425E-04",
            "+5.54620202E-04",
            "+8.23919506E-04",
            "+1.13992507E-03",
            "+1.47640169E-03",
            "+1.78371725E-03",
        ],
        [
            "+2.01182542E-03",
            "+2.10371989E-03",
            "+1.99454554E-03",
            "+1.61656283E-03",
            "+9.02154502E-04",
            "-1.78805361E-04",
            "-1.64973098E-03",
            "-3.49717454E-03",
        ],
        [
            "+5.65949473E-03",
            "+8.02941163E-03",
            "+1.04584443E-02",
            "+1.27472335E-02",
            "+1.46525263E-02",
            "+1.59045603E-02",
            "+1.62208471E-02",
            "+1.53184106E-02",
        ],
        [
            "+1.29371806E-02",
            "+8.85757540E-03",
            "+2.92408442E-03",
            "-4.91578024E-03",
            "-1.46404076E-02",
            "-2.61098752E-02",
            "-3.90751381E-02",
            "-5.31873032E-02",
        ],
        [
            "+6.79989431E-02",
            "+8.29847578E-02",
            "+9.75753918E-02",
            "+1.11196689E-01",
            "+1.23264548E-01",
            "+1.33264415E-01",
            "+1.40753505E-01",
            "+1.45389847E-01",
        ],
        [
            "+1.46955068E-01",
            "+1.45389847E-01",
            "+1.40753505E-01",
            "+1.33264415E-01",
            "+1.23264548E-01",
            "+1.11196689E-01",
            "+9.75753918E-02",
            "+8.29847578E-02",
        ],
        [
            "-6.79989431E-02",
            "-5.31873032E-02",
            "-3.90751381E-02",
            "-2.61098752E-02",
            "-1.46404076E-02",
            "-4.91578024E-03",
            "+2.92408442E-03",
            "+8.85757540E-03",
        ],
        [
            "+1.29371806E-02",
            "+1.53184106E-02",
            "+1.62208471E-02",
            "+1.59045603E-02",
            "+1.46525263E-02",
            "+1.27472335E-02",
            "+1.04584443E-02",
            "+8.02941163E-03",
        ],
        [
            "-5.65949473E-03",
            "-3.49717454E-03",
            "-1.64973098E-03",
            "-1.78805361E-04",
            "+9.02154502E-04",
            "+1.61656283E-03",
            "+1.99454554E-03",
            "+2.10371989E-03",
        ],
        [
            "+2.01182542E-03",
            "+1.78371725E-03",
            "+1.47640169E-03",
            "+1.13992507E-03",
            "+8.23919506E-04",
            "+5.54620202E-04",
            "+3.43256425E-04",
            "+1.56575398E-04",
        ],
    ]
};

#[test]
fn gen_proto_8_80() {
    let mut array = [[0.0; FILTER_ORDER]; 8];
    for sb in 0..8 {
        for i in 0..FILTER_ORDER {
            array[sb][i] = F_PROTO_8[i][sb].parse::<f64>().unwrap();
        }
    }

    println!("#[allow(overflowing_literals)]");
    println!("const PROTO_8: [[i64; 8]; FILTER_ORDER] = {{");
    println!("    [");
    for i in 0..FILTER_ORDER {
        print!("        [");
        for sb in 0..8 {
            let v = array[sb][i] * -8.0;
            print!("{:#018X}", (v * ((1_i64 << 62) as f64)) as i64);
            if sb != 7 {
                print!(", ");
            }
        }
        println!("],");
    }
    println!("    ]");
    println!("}};");
}

#[allow(overflowing_literals)]
const PROTO_8: [[i64; 8]; FILTER_ORDER] = {
    [
        [
            0x0000000000000000,
            0xFFEB7A33928CFA03,
            0xFFD30239C2CEAF16,
            0xFFB74E08BDCD94D4,
            0xFF9401D9559A63BC,
            0xFF6A9676357BB6D8,
            0xFF3E7C2E0D275A88,
            0xFF1634617DEBB590,
        ],
        [
            0xFEF84E573B4F3040,
            0xFEEC42DFBEB90330,
            0xFEFA922845485080,
            0xFF2C1D2706C4E030,
            0xFF89C0B7D60045E8,
            0x00176FB65B0E6E4E,
            0x00D83BC9366CB208,
            0x01CA61B48E2F32C0,
        ],
        [
            0xFD1A32DE720E56E0,
            0xFBE391A7493AF580,
            0xFAA530D787679D40,
            0xF97931D2018C6980,
            0xF87F76CD79B67380,
            0xF7DB5B834EF88D80,
            0xF7B1E6B06258C680,
            0xF8282F6EE48177C0,
        ],
        [
            0xF9604C40D752E600,
            0xFB770516AB983DC0,
            0xFE80BC0217390410,
            0x02845236BAF167E0,
            0x077EF28FAF2905C0,
            0x0D5E46082C5C4500,
            0x1401A81073CE5500,
            0x1B3B5DBF9CE53300,
        ],
        [
            0xDD2F3E1672248E00,
            0xD58305965D732400,
            0xCE0A9926A633BC00,
            0xC7113A42A49DE200,
            0xC0E3781B2AE4EC00,
            0xBBC4C43FB5779C00,
            0xB7EF2816748B9800,
            0xB58F7643EDDA5400,
        ],
        [
            0xB4C24E29EAC64400,
            0xB58F7643EDDA5400,
            0xB7EF2816748B9800,
            0xBBC4C43FB5779C00,
            0xC0E3781B2AE4EC00,
            0xC7113A42A49DE200,
            0xCE0A9926A633BC00,
            0xD58305965D732400,
        ],
        [
            0x22D0C1E98DDB7200,
            0x1B3B5DBF9CE53300,
            0x1401A81073CE5500,
            0x0D5E46082C5C4500,
            0x077EF28FAF2905C0,
            0x02845236BAF167E0,
            0xFE80BC0217390410,
            0xFB770516AB983DC0,
        ],
        [
            0xF9604C40D752E600,
            0xF8282F6EE48177C0,
            0xF7B1E6B06258C680,
            0xF7DB5B834EF88D80,
            0xF87F76CD79B67380,
            0xF97931D2018C6980,
            0xFAA530D787679D40,
            0xFBE391A7493AF580,
        ],
        [
            0x02E5CD218DF1A920,
            0x01CA61B48E2F32C0,
            0x00D83BC9366CB208,
            0x00176FB65B0E6E4E,
            0xFF89C0B7D60045E8,
            0xFF2C1D2706C4E030,
            0xFEFA922845485080,
            0xFEEC42DFBEB90330,
        ],
        [
            0xFEF84E573B4F3040,
            0xFF1634617DEBB590,
            0xFF3E7C2E0D275A88,
            0xFF6A9676357BB6D8,
            0xFF9401D9559A63BC,
            0xFFB74E08BDCD94D4,
            0xFFD30239C2CEAF16,
            0xFFEB7A33928CFA03,
        ],
    ]
};

#[test]
fn show_proto_8() {
    for i in 0..FILTER_ORDER {
        for sb in 0..8 {
            let v = PROTO_8[i][sb];
            print!("{:#010X}, ", round64(v, 32 + 1) as i32);
        }
        println!();
    }
}

pub(crate) const M_PRORO_8: [[i32; 8]; FILTER_ORDER] = {
    let mut v = [[0; 8]; FILTER_ORDER];
    const_for!(i in (0, FILTER_ORDER) {
        const_for!(sb in (0, 8) {
            v[i][sb] = round64(PROTO_8[i][sb], 32 + 1 + 14) as i32;
        });
    });
    v
};
