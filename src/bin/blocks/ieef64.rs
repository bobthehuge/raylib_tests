use raylib::math::Vector2;
use std::mem;

pub fn ieef64_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

#[derive(Hash, Eq, PartialEq)]
pub struct IeeF64((u64, i16, i8));

impl IeeF64 {
    pub fn new(val: f64) -> Self {
        IeeF64(ieef64_decode(val))
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct Vec2ieeF64 {
    x: IeeF64,
    y: IeeF64,
}

impl Vec2ieeF64 {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: IeeF64::new(x.into()), 
            y: IeeF64::new(y.into()), 
        }
    }

    pub fn from_vec2(v: Vector2) -> Self {
        Self::new(v.x, v.y)
    }
}

