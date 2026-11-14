//! A family from another crate, made dynamic: `uom`'s quantities.
//!
//! [`uom`](https://crates.io/crates/uom) checks dimensions at compile time:
//! a `Length` and a `Time` are different types, `Quantity<D, U, V>` with the
//! dimension `D` a type. It has no type for "a quantity whose dimension is
//! known at run time" - read from a file, typed by a user.
//!
//! `AnyQuantity` is that type, written here without touching `uom`: the
//! dimension `D` becomes a field of exponents (a *fact*), the number moves
//! in (the *payload*). Its `*`, `/` and `+` are the fields' own: the
//! exponents add under `*`, subtract under `/` and must be equal under `+`;
//! the numbers do what numbers do. The way back to a `uom` type is checked.
#![feature(const_trait_impl, const_cmp, const_ops, const_convert, const_destruct)]
#![feature(derive_const, const_default)]

use core::marker::PhantomData;
use core::ops::{Add, Div, Mul};
use typenum::Integer;
use uom::si::f64::{Length, Time, Velocity};
use uom::si::{Dimension, Quantity, SI, length, time, velocity};

/// A `uom` dimension, as a value: the exponent of each SI base quantity -
/// length, mass, time, current, temperature, amount, luminous intensity.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug)]
pub struct Exponents(pub [i8; 7]);

impl Exponents {
    /// The exponents `uom` keeps in the type `D`.
    pub const fn of<D: Dimension + ?Sized>() -> Exponents {
        Exponents([
            D::L::I8,
            D::M::I8,
            D::T::I8,
            D::I::I8,
            D::Th::I8,
            D::N::I8,
            D::J::I8,
        ])
    }
}

/// A product: the exponents add (m × m = m²).
impl const Mul for Exponents {
    type Output = Exponents;
    fn mul(mut self, o: Exponents) -> Exponents {
        let mut i = 0;
        while i < 7 {
            self.0[i] += o.0[i];
            i += 1;
        }
        self
    }
}

/// A quotient: the exponents subtract (m / s = m·s⁻¹).
impl const Div for Exponents {
    type Output = Exponents;
    fn div(mut self, o: Exponents) -> Exponents {
        let mut i = 0;
        while i < 7 {
            self.0[i] -= o.0[i];
            i += 1;
        }
        self
    }
}

/// A sum: only of the same dimension.
impl const Add for Exponents {
    type Output = Option<Exponents>;
    fn add(self, o: Exponents) -> Option<Exponents> {
        if self == o { Some(self) } else { None }
    }
}

/// Any `uom` quantity stored in SI base units as an `f64`. The mapping names
/// `uom`'s own struct: its `value` is the payload, and its two `PhantomData`
/// fields are declared so the way back can fill them.
#[derive(Copy, Clone, Debug, PartialEq, const_derive_more::Dynamic)]
#[dynamic(
    Quantity<D: Dimension<Kind = dyn uom::Kind> + ?Sized, SI<f64>, f64> {
        dimension: PhantomData<D>,
        units: PhantomData<SI<f64>>,
    } => { exponents: Exponents::of::<D>(), value: .value }
)]
#[dynamic(impl(
    Mul -> Self,
    Div -> Self,
    Add -> Option<Self>,
))]
pub struct AnyQuantity {
    pub exponents: Exponents,
    pub value: f64,
}

// What the derive writes, by hand (with `Q<D>` for `Quantity<D, SI<f64>,
// f64>`, and `Div` the same as `Mul`):
//
// impl<D: Dimension<Kind = dyn uom::Kind> + ?Sized> From<Q<D>> for AnyQuantity {
//     fn from(q: Q<D>) -> Self { AnyQuantity { exponents: Exponents::of::<D>(), value: q.value } }
// }
// impl<D: Dimension<Kind = dyn uom::Kind> + ?Sized> TryFrom<AnyQuantity> for Q<D> {
//     type Error = AnyQuantity;
//     fn try_from(a: AnyQuantity) -> Result<Self, AnyQuantity> {
//         if a.exponents == Exponents::of::<D>() {
//             Ok(Quantity { value: a.value, dimension: PhantomData, units: PhantomData })
//         } else {
//             Err(a)
//         }
//     }
// }
// impl Mul for AnyQuantity {
//     type Output = AnyQuantity;
//     fn mul(self, o: AnyQuantity) -> AnyQuantity {
//         AnyQuantity { exponents: self.exponents * o.exponents, value: self.value * o.value }
//     }
// }
// impl Add for AnyQuantity {
//     type Output = Option<AnyQuantity>;
//     fn add(self, o: AnyQuantity) -> Option<AnyQuantity> {
//         Some(AnyQuantity { exponents: (self.exponents + o.exponents)?, value: self.value + o.value })
//     }
// }

fn main() {
    let distance = AnyQuantity::from(Length::new::<length::kilometer>(3.0));
    let duration = AnyQuantity::from(Time::new::<time::minute>(10.0));
    println!("distance = {distance:?}");
    println!("duration = {duration:?}");

    // Arithmetic at run time: the exponents are checked as values.
    let speed = distance / duration;
    println!("distance / duration = {speed:?}");
    println!("distance + distance = {:?}", distance + distance);
    println!("distance + duration = {:?}", distance + duration);

    // Back to a `uom` type: checked, and read with `uom`'s own units.
    let v = Velocity::try_from(speed).expect("a length over a time is a velocity");
    println!(
        "as Velocity: {} km/h",
        v.get::<velocity::kilometer_per_hour>()
    );
    println!("as Length:   {}", Length::try_from(speed).is_ok());
}
