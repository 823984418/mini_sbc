
#[allow(dead_code)]
pub(crate) const fn round64(x: i64, r: usize) -> i64 {
    if r == 0 {
        return x;
    }
    let s = (x << (64 - r)) as u64;
    if s == (1_u64 << 63) {
        ((x >> (r - 1)) + ((x >> 63) & 1)) >> 1
    } else {
        if s >= (1_u64 << 63) {
            ((x >> (r - 1)) + 1) >> 1
        } else {
            x >> r
        }
    }
}

#[allow(dead_code)]
pub(crate) const fn round32(x: i32, r: usize) -> i32 {
    if r == 0 {
        return x;
    }
    let s = (x << (32 - r)) as u32;
    if s == (1_u32 << 31) {
        ((x >> (r - 1)) + ((x >> 31) & 1)) >> 1
    } else {
        if s >= (1_u32 << 31) {
            ((x >> (r - 1)) + 1) >> 1
        } else {
            x >> r
        }
    }
}

#[macro_export(crate)]
macro_rules! const_for {
    ($i:ident in ($f:expr, $t:expr) $b:block) => {{
        let mut $i = $f;
        while $i < $t {
            {
                let $i = $i;
                $b;
            }
            $i += 1;
        }
    }};
}

pub(crate) const fn saturating_i16(v: i32) -> i16 {
    let x = v as i16;
    if v == (x as i32) {
        x
    } else if v > 0 {
        i16::MAX
    } else {
        i16::MIN
    }
}
