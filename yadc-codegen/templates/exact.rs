//! The exact number a unit's size and zero are written in.
//!
//! A unit's size is written as its definition - an inch is exactly 0.0254
//! m, a degree π/180 rad, a pound-force `0.45359237 * 9.80665` N - and kept
//! exact through every product and quotient. A number type is chosen where a
//! unit is used, and its size and zero are converted with `From`, rounding
//! once, correctly: the nearest `f64`, the nearest integer. A quantity of a
//! plain number read as a number (`f64::from(d / KILOMETRE)`) converts its
//! unit's size in a `const` block, so the compiler does that rounding.

use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Sub};

/// An exact real number `num / den · 10^exp10 · π^pi`, normalized: `den`
/// positive and coprime to `num` and to 10, `num` not a multiple of 10 (zero
/// is `0 / 1`). The normal form is unique, so `==` compares values.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, core::marker::ConstParamTy)]
pub struct Exact {
    num: i128,
    den: i128,
    exp10: i16,
    pi: i8,
}

const fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.unsigned_abs(), b.unsigned_abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a as i128
}

const fn checked(x: Option<i128>) -> i128 {
    match x {
        Some(v) => v,
        None => panic!("an exact constant does not fit: its digits overflow i128"),
    }
}

const fn ten_to(n: i16) -> i128 {
    let mut r: i128 = 1;
    let mut i = 0;
    while i < n {
        r = checked(r.checked_mul(10));
        i += 1;
    }
    r
}

impl Exact {
    /// Zero.
    pub const ZERO: Exact = Exact { num: 0, den: 1, exp10: 0, pi: 0 };
    /// One.
    pub const ONE: Exact = Exact { num: 1, den: 1, exp10: 0, pi: 0 };
    /// π.
    pub const PI: Exact = Exact { num: 1, den: 1, exp10: 0, pi: 1 };

    /// `num / den · 10^exp10 · π^pi`, in normal form.
    pub const fn new(num: i128, den: i128, exp10: i16, pi: i8) -> Exact {
        assert!(den != 0, "an exact number with a zero denominator");
        if num == 0 {
            return Exact::ZERO;
        }
        let (mut num, mut den) = if den < 0 { (-num, -den) } else { (num, den) };
        let g = gcd(num, den);
        num /= g;
        den /= g;
        let mut exp10 = exp10;
        // 1/2 = 5/10 and 1/5 = 2/10: the denominator gives its factors of
        // two and five to the power of ten.
        while den % 2 == 0 {
            den /= 2;
            num = checked(num.checked_mul(5));
            exp10 -= 1;
        }
        while den % 5 == 0 {
            den /= 5;
            num = checked(num.checked_mul(2));
            exp10 -= 1;
        }
        while num % 10 == 0 {
            num /= 10;
            exp10 += 1;
        }
        let g = gcd(num, den);
        Exact { num: num / g, den: den / g, exp10, pi }
    }

    /// An integer.
    pub const fn int(n: i128) -> Exact {
        Exact::new(n, 1, 0, 0)
    }

    /// A decimal, `mantissa · 10^exp10`: 0.45359237 is `dec(45359237, -8)`.
    pub const fn dec(mantissa: i128, exp10: i16) -> Exact {
        Exact::new(mantissa, 1, exp10, 0)
    }

    /// Whether it is a whole number.
    pub const fn is_integer(self) -> bool {
        self.den == 1 && self.pi == 0 && self.exp10 >= 0
    }

    /// The nearest `f64`, or `f32`, to the rational part `num / den ·
    /// 10^exp10`, rounded once (to nearest, ties to even).
    const fn rational_bits(self, mantissa_bits: u32) -> (bool, i32, u64) {
        let negative = self.num < 0;
        let mut n = Big::from(self.num.unsigned_abs());
        let mut d = Big::from(self.den as u128);
        let mut i = 0;
        while i < self.exp10 {
            n = n.mul_small(10);
            i += 1;
        }
        while i > self.exp10 {
            d = d.mul_small(10);
            i -= 1;
        }
        // Scale so the quotient has `mantissa_bits + 2` or `+ 3` bits.
        let shift = (mantissa_bits as i32 + 2) + d.bits() as i32 - n.bits() as i32;
        let (n, d) = if shift >= 0 {
            (n.shl(shift as u32), d)
        } else {
            (n, d.shl((-shift) as u32))
        };
        let (q, rest) = n.div_rem(&d);
        let q = q.low();
        let extra = 64 - q.leading_zeros() - mantissa_bits;
        let mut m = q >> extra;
        let dropped = q & ((1u64 << extra) - 1);
        let half = 1u64 << (extra - 1);
        if dropped > half || (dropped == half && (!rest.is_zero() || m & 1 == 1)) {
            m += 1;
        }
        let mut exp = extra as i32 - shift;
        if m == 1u64 << mantissa_bits {
            m >>= 1;
            exp += 1;
        }
        // value = m · 2^exp, with m of `mantissa_bits` bits.
        (negative, exp + mantissa_bits as i32 - 1, m)
    }

    /// The nearest `f64`. Rounded once where there is no π; a power of π
    /// multiplies the rounded rational by `π^k`, rounding again.
    pub const fn to_f64(self) -> f64 {
        if self.num == 0 {
            return 0.0;
        }
        let (negative, e, m) = self.rational_bits(53);
        assert!(e > -1023 && e < 1024, "an exact constant outside the range of f64");
        let bits = ((e + 1023) as u64) << 52 | (m & ((1u64 << 52) - 1));
        let mut r = f64::from_bits(bits);
        let mut k = 0;
        while k < self.pi {
            r *= core::f64::consts::PI;
            k += 1;
        }
        while k > self.pi {
            r /= core::f64::consts::PI;
            k -= 1;
        }
        if negative { -r } else { r }
    }

    /// The nearest integer, ties to even. A power of π rounds its nearest
    /// `f64`. Panics where it does not fit an `i128` - in a `const`, a
    /// compile error.
    pub const fn to_i128(self) -> i128 {
        if self.num == 0 {
            return 0;
        }
        if self.pi != 0 {
            let r = self.to_f64().round_ties_even();
            assert!(r > -1.7e38 && r < 1.7e38, "an exact constant outside the range of i128");
            return r as i128;
        }
        let mut n = Big::from(self.num.unsigned_abs());
        let mut d = Big::from(self.den as u128);
        let mut i = 0;
        while i < self.exp10 {
            n = n.mul_small(10);
            i += 1;
        }
        while i > self.exp10 {
            d = d.mul_small(10);
            i -= 1;
        }
        let (q, rest) = n.div_rem(&d);
        assert!(q.bits() < 127, "an exact constant outside the range of i128");
        let mut q = (q.0[1] as u128) << 64 | q.0[0] as u128;
        // Half way or more: compare twice the remainder with the divisor.
        let twice = rest.shl(1);
        if twice.ge(&d) && (!d.ge(&twice) || q & 1 == 1) {
            q += 1;
        }
        if self.num < 0 { -(q as i128) } else { q as i128 }
    }

    /// The nearest integer, which must lie in `min..=max`.
    const fn to_integer(self, min: i128, max: u128) -> i128 {
        let v = self.to_i128();
        let fits = if v < 0 { v >= min } else { v as u128 <= max };
        assert!(fits, "an exact constant outside the range of its integer type");
        v
    }

    /// The nearest `f32`, rounded once where there is no π.
    pub const fn to_f32(self) -> f32 {
        if self.num == 0 {
            return 0.0;
        }
        if self.pi != 0 {
            return self.to_f64() as f32;
        }
        let (negative, e, m) = self.rational_bits(24);
        assert!(e > -127 && e < 128, "an exact constant outside the range of f32");
        let bits = ((e + 127) as u32) << 23 | (m as u32 & ((1u32 << 23) - 1));
        let r = f32::from_bits(bits);
        if negative { -r } else { r }
    }
}

const impl Mul for Exact {
    type Output = Exact;
    fn mul(self, o: Exact) -> Exact {
        let g1 = gcd(self.num, o.den);
        let g2 = gcd(o.num, self.den);
        let (g1, g2) = (if g1 == 0 { 1 } else { g1 }, if g2 == 0 { 1 } else { g2 });
        Exact::new(
            checked((self.num / g1).checked_mul(o.num / g2)),
            checked((self.den / g2).checked_mul(o.den / g1)),
            self.exp10 + o.exp10,
            self.pi + o.pi,
        )
    }
}

const impl Div for Exact {
    type Output = Exact;
    fn div(self, o: Exact) -> Exact {
        assert!(o.num != 0, "an exact division by zero");
        self * Exact { num: o.den, den: 1, exp10: -o.exp10, pi: -o.pi } * Exact::new(1, o.num, 0, 0)
    }
}

const impl Add for Exact {
    type Output = Exact;
    fn add(self, o: Exact) -> Exact {
        if self.num == 0 {
            return o;
        }
        if o.num == 0 {
            return self;
        }
        assert!(self.pi == o.pi, "an exact sum needs the same power of π on both sides");
        let e = if self.exp10 < o.exp10 { self.exp10 } else { o.exp10 };
        let a = checked(self.num.checked_mul(ten_to(self.exp10 - e)));
        let b = checked(o.num.checked_mul(ten_to(o.exp10 - e)));
        let num = checked(checked(a.checked_mul(o.den)).checked_add(checked(b.checked_mul(self.den))));
        Exact::new(num, checked(self.den.checked_mul(o.den)), e, self.pi)
    }
}

const impl Neg for Exact {
    type Output = Exact;
    fn neg(self) -> Exact {
        Exact { num: -self.num, ..self }
    }
}

const impl Sub for Exact {
    type Output = Exact;
    fn sub(self, o: Exact) -> Exact {
        self + -o
    }
}

impl PartialOrd for Exact {
    fn partial_cmp(&self, o: &Exact) -> Option<Ordering> {
        if self.pi == o.pi {
            let d = *self - *o;
            Some(d.num.cmp(&0))
        } else {
            self.to_f64().partial_cmp(&o.to_f64())
        }
    }
}

impl core::fmt::Display for Exact {
    /// The exact form, its decimal part as a decimal: `1609.344`, `1000`,
    /// `1200/3937`, `0.3/7`, `1/180·π`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.num < 0 {
            f.write_str("-")?;
        }
        let digits = self.num.unsigned_abs();
        if self.exp10 >= 0 {
            write!(f, "{digits}")?;
            for _ in 0..self.exp10 {
                f.write_str("0")?;
            }
        } else {
            // `digits · 10^exp10`: the point `-exp10` digits from the right.
            let point = (-self.exp10) as u32;
            let scale = 10u128.checked_pow(point);
            match scale {
                Some(scale) => {
                    let (whole, frac) = (digits / scale, digits % scale);
                    write!(f, "{whole}.{frac:0width$}", width = point as usize)?;
                }
                None => {
                    f.write_str("0.")?;
                    let shown = digits.ilog10() + 1;
                    for _ in shown..point {
                        f.write_str("0")?;
                    }
                    write!(f, "{digits}")?;
                }
            }
        }
        if self.den != 1 {
            write!(f, "/{}", self.den)?;
        }
        match self.pi {
            0 => Ok(()),
            1 => write!(f, "·π"),
            k => write!(f, "·π^{k}"),
        }
    }
}

const impl From<Exact> for f64 {
    fn from(x: Exact) -> f64 {
        x.to_f64()
    }
}

const impl From<Exact> for f32 {
    fn from(x: Exact) -> f32 {
        x.to_f32()
    }
}

/// The nearest integer, ties to even; a panic where it does not fit the
/// type. The size of `KILO * METRE` in `i32` is 1000; an inch's, in metres,
/// is 0 - the rounding is yours to choose by the unit you count in.
const impl From<Exact> for i8 {
    fn from(x: Exact) -> i8 {
        x.to_integer(i8::MIN as i128, i8::MAX as u128) as i8
    }
}
const impl From<Exact> for i16 {
    fn from(x: Exact) -> i16 {
        x.to_integer(i16::MIN as i128, i16::MAX as u128) as i16
    }
}
const impl From<Exact> for i32 {
    fn from(x: Exact) -> i32 {
        x.to_integer(i32::MIN as i128, i32::MAX as u128) as i32
    }
}
const impl From<Exact> for i64 {
    fn from(x: Exact) -> i64 {
        x.to_integer(i64::MIN as i128, i64::MAX as u128) as i64
    }
}
const impl From<Exact> for i128 {
    fn from(x: Exact) -> i128 {
        x.to_integer(i128::MIN, i128::MAX as u128)
    }
}
const impl From<Exact> for isize {
    fn from(x: Exact) -> isize {
        x.to_integer(isize::MIN as i128, isize::MAX as u128) as isize
    }
}
const impl From<Exact> for u8 {
    fn from(x: Exact) -> u8 {
        x.to_integer(u8::MIN as i128, u8::MAX as u128) as u8
    }
}
const impl From<Exact> for u16 {
    fn from(x: Exact) -> u16 {
        x.to_integer(u16::MIN as i128, u16::MAX as u128) as u16
    }
}
const impl From<Exact> for u32 {
    fn from(x: Exact) -> u32 {
        x.to_integer(u32::MIN as i128, u32::MAX as u128) as u32
    }
}
const impl From<Exact> for u64 {
    fn from(x: Exact) -> u64 {
        x.to_integer(u64::MIN as i128, u64::MAX as u128) as u64
    }
}
const impl From<Exact> for u128 {
    fn from(x: Exact) -> u128 {
        x.to_integer(u128::MIN as i128, u128::MAX) as u128
    }
}
const impl From<Exact> for usize {
    fn from(x: Exact) -> usize {
        x.to_integer(usize::MIN as i128, usize::MAX as u128) as usize
    }
}

/// An unsigned integer of up to 512 bits, for rounding exactly.
#[derive(Copy, Clone)]
struct Big([u64; 8]);

impl Big {
    const fn from(x: u128) -> Big {
        let mut l = [0u64; 8];
        l[0] = x as u64;
        l[1] = (x >> 64) as u64;
        Big(l)
    }
    const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < 8 {
            if self.0[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }
    const fn low(&self) -> u64 {
        self.0[0]
    }
    const fn bits(&self) -> u32 {
        let mut i = 8;
        while i > 0 {
            i -= 1;
            if self.0[i] != 0 {
                return i as u32 * 64 + 64 - self.0[i].leading_zeros();
            }
        }
        0
    }
    const fn bit(&self, i: u32) -> bool {
        (self.0[(i / 64) as usize] >> (i % 64)) & 1 == 1
    }
    const fn mul_small(self, k: u64) -> Big {
        let mut l = [0u64; 8];
        let mut carry: u128 = 0;
        let mut i = 0;
        while i < 8 {
            let v = self.0[i] as u128 * k as u128 + carry;
            l[i] = v as u64;
            carry = v >> 64;
            i += 1;
        }
        assert!(carry == 0, "an exact constant too large to round");
        Big(l)
    }
    const fn shl(self, s: u32) -> Big {
        assert!(self.bits() + s <= 512, "an exact constant too large to round");
        let (w, b) = ((s / 64) as usize, s % 64);
        let mut l = [0u64; 8];
        let mut i = 8;
        while i > w {
            i -= 1;
            let lo = self.0[i - w] << b;
            let carry = if b > 0 && i - w > 0 { self.0[i - w - 1] >> (64 - b) } else { 0 };
            l[i] = lo | carry;
        }
        Big(l)
    }
    const fn ge(&self, o: &Big) -> bool {
        let mut i = 8;
        while i > 0 {
            i -= 1;
            if self.0[i] != o.0[i] {
                return self.0[i] > o.0[i];
            }
        }
        true
    }
    const fn sub(self, o: &Big) -> Big {
        let mut l = [0u64; 8];
        let mut borrow = 0u64;
        let mut i = 0;
        while i < 8 {
            let (v, b1) = self.0[i].overflowing_sub(o.0[i]);
            let (v, b2) = v.overflowing_sub(borrow);
            l[i] = v;
            borrow = (b1 || b2) as u64;
            i += 1;
        }
        Big(l)
    }
    /// The quotient and the remainder, by long division one bit at a time.
    const fn div_rem(&self, d: &Big) -> (Big, Big) {
        let mut q = Big([0; 8]);
        let mut r = Big([0; 8]);
        let mut i = self.bits();
        while i > 0 {
            i -= 1;
            r = r.shl(1);
            if self.bit(i) {
                r.0[0] |= 1;
            }
            if r.ge(d) {
                r = r.sub(d);
                q.0[(i / 64) as usize] |= 1u64 << (i % 64);
            }
        }
        (q, r)
    }
}
