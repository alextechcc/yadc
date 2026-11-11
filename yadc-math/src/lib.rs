//! The generic maths under yadc: what a dimension is, partial roots,
//! typed powers and roots, rational exponents, and changes of number type.
//! Nothing here knows about units.
//!
//! - [`Dimension`]: exponents (and optionally a kind) with a
//!   product and a quotient (std `*` / `/`, total or partial), a partial sum
//!   (std `+` / `-` returning `Option`), [`PartialPowi`] and [`PartialRoot`];
//! - [`PartialPowi`] / [`PartialRoot`]: an integer power and an n-th root
//!   that may not exist - for dimensions, and for `f32` / `f64`;
//! - [`Exponent`]: a rational exponent, for systems with fractional powers;
//! - [`Powi`] / [`Root`] with method syntax [`PowiAs`] / [`RootAs`]: a power
//!   or root whose exponent is known at compile time, typed;
//! - [`Cast`] with method syntax [`CastAs`]: a change of number type.
//!
//! "May fail" is always a std operator returning `Option` - `impl Add for
//! Dim { type Output = Option<Self>; }` - which says so in the type, chains
//! with `?`, and runs in `const` code on nightly. No trait of its own is
//! needed for that.
#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(derive_const)]
#![feature(min_adt_const_params)]
#![feature(const_default)]
#![feature(const_ops)]

#[cfg(feature = "std")]
extern crate std;

use core::ops::{Add, Div, Mul, Sub};

/// A partial n-th root: the value whose n-th power is `self`, or `None` where
/// there is none in this type.
///
/// For an integer exponent the root is exact division - the exponent of
/// `√(m²)` is `2 / 2 = 1`, and `√m` has no integer exponent, so `None` (std's
/// `/` truncates, which is why this is a trait). For a dimension it is the
/// root of every exponent and of the kind (√ of a steradian, rad², is a radian);
/// with [`Exponent`] every root exists. Its partner is [`PartialPowi`].
///
/// For `f32` / `f64` (with the `std` or `libm` feature) every root but the
/// zeroth is `Some(x.powf(1 / n))` - NaN where the real root does not exist,
/// as `powf` has it.
pub const trait PartialRoot: Sized {
    /// The n-th root, or `None`.
    fn partial_root(self, n: i8) -> Option<Self>;
}

/// A partial integer power: `self` multiplied by itself `n` times (for a
/// negative `n` the reciprocal of that), or `None` where a step has no
/// result - a point has no products, so no powers.
///
/// For a dimension it is repeated `*` (yadc's `#[derive(Dimension)]` writes
/// it); `self^0` is `self / self`. For `f32` / `f64` it is always `Some`.
pub const trait PartialPowi: Sized {
    /// The n-th power, or `None`.
    fn partial_powi(self, n: i8) -> Option<Self>;
}

/// A float's power by repeated multiplication: always `Some`, `const`.
macro_rules! float_powi {
    ($($t:ty),*) => {$(
        const impl PartialPowi for $t {
            fn partial_powi(self, n: i8) -> Option<$t> {
                let mut acc: $t = 1.0;
                let mut i = 0;
                while i < n.unsigned_abs() {
                    acc *= self;
                    i += 1;
                }
                Some(if n < 0 { 1.0 / acc } else { acc })
            }
        }
    )*};
}
float_powi!(f32, f64);

/// A float's n-th root, `powf(1 / n)`: from std, or from libm in `no_std`.
macro_rules! float_root {
    ($($t:ty: $libm:ident),*) => {$(
        #[cfg(any(feature = "std", feature = "libm"))]
        impl PartialRoot for $t {
            fn partial_root(self, n: i8) -> Option<$t> {
                if n == 0 {
                    return None;
                }
                let e = 1.0 / n as $t;
                #[cfg(feature = "std")]
                let r = self.powf(e);
                #[cfg(all(not(feature = "std"), feature = "libm"))]
                let r = libm::$libm(self, e);
                Some(r)
            }
        }
    )*};
}
float_root!(f32: powf, f64: pow);

/// The root of an integer exponent: exact division - `4` rooted by `2` is
/// `Some(2)`, `3` rooted by `2` has no integer result.
macro_rules! exponent_root {
    ($($t:ty),*) => {$(
        const impl PartialRoot for $t {
            fn partial_root(self, n: i8) -> Option<$t> {
                let n = n as $t;
                match self.checked_rem(n) {
                    Some(0) => self.checked_div(n),
                    _ => None,
                }
            }
        }
    )*};
}
exponent_root!(i8, i16, i32, i64, isize);

/// A rational exponent, for systems with fractional powers: `V/√Hz` noise
/// density, fracture toughness in `MPa·√m`, or CGS-Gaussian units, whose
/// base dimensions carry half powers.
///
/// One ready-made exponent type, and an optional one: a dimension's
/// exponent fields may be of any type with the operators yadc's
/// `#[derive(Dimension)]` uses (const `PartialEq`, `Default`, `Add` / `Sub`
/// with `Output: Into<Option<Self>>`, [`PartialRoot`]); with this one every
/// root exists. Kept in lowest terms with a positive denominator -
/// build values with [`new`](Self::new) / [`int`](Self::int) so equal
/// exponents compare equal. The fields are public only because a const
/// generic parameter's fields must be.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, Hash, core::marker::ConstParamTy)]
pub struct Exponent {
    /// Numerator.
    pub num: i8,
    /// Denominator, always positive.
    pub den: i8,
}

impl Exponent {
    /// `num / den`, reduced to lowest terms. Panics on a zero denominator -
    /// meant for writing constants; arithmetic on exponents is checked.
    pub const fn new(num: i8, den: i8) -> Self {
        assert!(den != 0, "an exponent's denominator cannot be zero");
        match Exponent::reduced(num as i32, den as i32) {
            Some(e) => e,
            None => panic!("this exponent does not fit in i8 / i8"),
        }
    }

    /// The whole-number exponent `n`.
    pub const fn int(n: i8) -> Self {
        Exponent { num: n, den: 1 }
    }

    /// `num / den` in lowest terms with a positive denominator, or `None` if
    /// it doesn't fit (or `den` is zero).
    const fn reduced(num: i32, den: i32) -> Option<Self> {
        if den == 0 {
            return None;
        }
        let g = gcd(num, den);
        let sign = if den < 0 { -1 } else { 1 };
        let (n, d) = (sign * num / g, sign * den / g);
        if n < i8::MIN as i32 || n > i8::MAX as i32 || d > i8::MAX as i32 {
            None
        } else {
            Some(Exponent {
                num: n as i8,
                den: d as i8,
            })
        }
    }
}

const fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (a.unsigned_abs(), b.unsigned_abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 { 1 } else { a as i32 }
}

const impl Default for Exponent {
    fn default() -> Self {
        Exponent::int(0)
    }
}

/// Exponents add under a dimension's `*`; `None` if the sum doesn't fit.
const impl Add for Exponent {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Option<Self> {
        let (a, b) = (self, rhs);
        Exponent::reduced(
            a.num as i32 * b.den as i32 + b.num as i32 * a.den as i32,
            a.den as i32 * b.den as i32,
        )
    }
}

/// Exponents subtract under a dimension's `/`; `None` if the difference
/// doesn't fit.
const impl Sub for Exponent {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Option<Self> {
        let (a, b) = (self, rhs);
        Exponent::reduced(
            a.num as i32 * b.den as i32 - b.num as i32 * a.den as i32,
            a.den as i32 * b.den as i32,
        )
    }
}

/// Every rational exponent has every root that fits: divide by `n`.
const impl PartialRoot for Exponent {
    fn partial_root(self, n: i8) -> Option<Exponent> {
        Exponent::reduced(self.num as i32, self.den as i32 * n as i32)
    }
}

impl core::fmt::Display for Exponent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

/// Parses what [`Display`](core::fmt::Display) writes: `"3"`, `"-5/2"`;
/// reduced to lowest terms. An error for anything else, a zero denominator, or
/// a value that doesn't fit.
impl core::str::FromStr for Exponent {
    type Err = ParseExponentError;
    fn from_str(s: &str) -> Result<Self, ParseExponentError> {
        let (num, den) = match s.split_once('/') {
            Some((n, d)) => (n.trim(), d.trim()),
            None => (s.trim(), "1"),
        };
        let num: i32 = num.parse().map_err(|_| ParseExponentError)?;
        let den: i32 = den.parse().map_err(|_| ParseExponentError)?;
        Exponent::reduced(num, den).ok_or(ParseExponentError)
    }
}

/// The error from parsing an [`Exponent`]: not `n` or `n/d` in range.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ParseExponentError;

impl core::fmt::Display for ParseExponentError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("not an exponent: expected `n` or `n/d` with a nonzero `d`, in range")
    }
}

impl core::error::Error for ParseExponentError {}

/// `x^N` for a compile-time integer `N`: the result's type is the `N`-th
/// power's, so with yadc a `Length` squared is an `Area`.
///
/// Written by yadc's `#[derive(Quantity)]` as a `const impl` wherever the
/// dimension's power ([`PartialPowi`]) exists - a point has none. Call it
/// as `x.powi::<N>()`, through [`PowiAs`].
pub const trait Powi<const N: i8> {
    /// The type of the `N`-th power.
    type Output;
    /// `self` raised to the `N`-th power.
    fn powi_into(self) -> Self::Output;
}

/// `x.powi::<N>()` for every type: method syntax for [`Powi<N>`], whose `N`
/// sits on the trait where a method call cannot name it.
pub const trait PowiAs: Sized {
    /// `self` to the `N`-th power: [`Powi::powi_into`].
    fn powi<const N: i8>(self) -> <Self as Powi<N>>::Output
    where
        Self: [const] Powi<N>,
    {
        <Self as Powi<N>>::powi_into(self)
    }
}
const impl<X> PowiAs for X {}

/// The `N`-th root for a compile-time integer `N`: `Area` -> `Length`.
///
/// Exists only where the root's type does - every root with rational
/// exponents ([`Exponent`]), only exact ones with integer exponents (√`m²` but
/// not √`m`), none of a point. Call it as `x.root::<N>()`, through
/// [`RootAs`]. A `const trait` whose impls are ordinary ones: the number needs
/// `powf`, which is not const.
pub const trait Root<const N: i8> {
    /// The type of the `N`-th root.
    type Output;
    /// The `N`-th root of `self`.
    fn root_into(self) -> Self::Output;
}

/// `x.root::<N>()` for every type: method syntax for [`Root<N>`], whose `N`
/// sits on the trait where a method call cannot name it.
pub const trait RootAs: Sized {
    /// The `N`-th root of `self`: [`Root::root_into`].
    fn root<const N: i8>(self) -> <Self as Root<N>>::Output
    where
        Self: [const] Root<N>,
    {
        <Self as Root<N>>::root_into(self)
    }
}
const impl<X> RootAs for X {}

/// A dimension: what a value of a family *is*, and how products and sums of
/// them combine.
///
/// On the whiteboard:
///
/// - **a product and a quotient**, std `*` and `/`, whose `Output` is either
///   `Self` (total: every two dimensions have a product) or `Option<Self>`
///   (partial: `None` where there is none) - the trait asks only for
///   `Output: Into<Option<Self>>`, so generic code reads both the same way.
///   Exponents add under `*` and subtract under `/`.
///   [`DIMENSIONLESS`](Self::DIMENSIONLESS) (dimension one) is the identity.
/// - **a sum**, std `+` and `-` returning `Option<Self>`: `a + b` is the
///   dimension of a sum of values, `None` where they cannot be added.
/// - **roots**, [`PartialRoot`] (with integer exponents the square root of a
///   length has none), and **powers**, [`PartialPowi`]: repeated `*`.
///
/// Exponents alone make the product total, and the dimensions under `*` an
/// abelian group. A kind (yadc's `#[kind]` field) changes that, and the laws
/// are stated here as they are:
///
/// - **A label** (a torque and an energy share their exponents) matters to
///   `+` and `-` only. It survives `*` and `/` only beside a plain number,
///   which makes `*` **not associative**: `(torque × m) / m` is a plain
///   newton-metre (the label meets a metre and is dropped), while `torque ×
///   (m / m)` is a torque (`m / m` is a plain number).
/// - **A point** (the affine kind: an altitude, an instant) has no products:
///   `*` and `/` return `Option<Self>`, `None` whenever a point is involved,
///   so `x * DIMENSIONLESS == x` holds only for a dimension that is not a
///   point. Two points differ by an amount, and an amount moves one.
/// - **A logarithm** (the log kind: a level, a gain) adds where the things
///   it is the logarithm of multiply: its `+` and `-` add and subtract the
///   exponents, and its only products are with a plain number, and - for a
///   gain, which has no exponents - those a label has.
///
/// `a + b` keeps its left operand: where a sum of values exists it is `a`
/// again (a point plus a length is the same point, a level plus a gain the
/// same level). It is an **action** of `b` on `a`, not a
/// commutative sum: `step + t` has no result; write `t + step`.
///
/// A dimension is usually also the const generic parameter of a family of
/// static types (`Qty<const D: Dim>`), so it derives `ConstParamTy` and, for
/// const `==`, `#[derive_const(PartialEq)]`; no trait here can express that.
///
/// `const` generic code that multiplies dimensions asks for the conversion to
/// be const as well: `where <D as Mul>::Output: [const] Into<Option<D>>` (a
/// bound on the trait itself would not carry `[const]`). Powers need no such
/// bound: [`partial_powi`](PartialPowi::partial_powi).
pub const trait Dimension:
    Copy
    + [const] PartialEq
    + [const] Mul<Output: Into<Option<Self>>>
    + [const] Div<Output: Into<Option<Self>>>
    + [const] Add<Output = Option<Self>>
    + [const] Sub<Output = Option<Self>>
    + [const] PartialPowi
    + [const] PartialRoot
{
    /// Dimension one: the empty product, the dimension of a pure number
    /// (ISO 80000 calls dimensionless quantities "quantities of dimension
    /// one"). `x * DIMENSIONLESS == x` and `x / x == DIMENSIONLESS` for every
    /// `x` that is not a point, labelled or not.
    const DIMENSIONLESS: Self;

    /// The same exponents with the plain label: the map that
    /// forgets labels. `plain(K, a point) = K`, `plain(a gain) = 1`. Two
    /// dimensions measure the same amount when their plain forms are equal.
    ///
    /// Provided as the identity, which is right for a dimension without
    /// labels (it is its own plain form); a dimension with labels overrides
    /// it (yadc's `#[derive(Dimension)]` does).
    fn plain(self) -> Self {
        self
    }
}

/// A change of number type, `as` in Rust: `2.5f64` to `2.5f32`, `7u8` to
/// `7.0f64`. On the whiteboard, the inclusion (or rounding) of one number set
/// into another.
///
/// `Output` is what the change produces: `T` itself for a number; for a
/// quantity family generic over its number type (`Qty<D, N>`), the same
/// quantity counted in `T`, which yadc's `#[derive(Quantity)]` writes.
/// Implemented for every pair of primitive integer and float types, by `as`
/// (rounding toward zero and saturating, as `as` does). Call it as
/// `x.cast::<f32>()`, through [`CastAs`].
pub const trait Cast<T> {
    /// The result: `T`, or a quantity counted in `T`.
    type Output;
    /// `self` with its numbers changed to `T`.
    fn cast_into(self) -> Self::Output;
}

/// `x.cast::<T>()` for every type: method syntax for [`Cast<T>`], whose `T`
/// sits on the trait where a method call cannot name it.
/// `const F: Pressure32 = KILOPASCAL.cast::<f32>();` works.
pub const trait CastAs: Sized {
    /// `self` with its numbers changed to `T`: [`Cast::cast_into`].
    fn cast<T>(self) -> <Self as Cast<T>>::Output
    where
        Self: [const] Cast<T>,
    {
        <Self as Cast<T>>::cast_into(self)
    }
}
const impl<X> CastAs for X {}

macro_rules! cast_all {
    ($($a:ty),*) => { cast_all!(@each [$($a),*] [$($a),*]); };
    (@each [$($a:ty),*] $bs:tt) => { $( cast_all!(@one $a $bs); )* };
    (@one $a:ty [$($b:ty),*]) => {$(
        const impl Cast<$b> for $a {
            type Output = $b;
            fn cast_into(self) -> $b {
                self as $b
            }
        }
    )*};
}
cast_all!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);
