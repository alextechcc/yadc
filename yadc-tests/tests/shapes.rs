//! Shapes the derives accept: a quantity with only some of its extras, and a
//! unit type defined by a const parameter, named by an enum of units.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(
    const_cmp,
    const_default,
    derive_const,
    const_ops,
    const_convert,
    const_try
)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use yadc::{Dimension, Unit};
use yadc_tests::{Length, METRE};

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub struct Dim {
    pub m: i8,
}

/// `Sum` alone: it adds the numbers, needing no other extra.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
#[quantity(impl(Sum))]
pub struct Qty<const D: Dim>(f64);

pub type Distance = Qty<{ Dim { m: 1 } }>;
pub const ONE_METRE: Distance = Qty(1.0);

#[test]
fn sum_alone() {
    let total: Distance = [ONE_METRE, ONE_METRE * 2.0].into_iter().sum();
    let by_ref: Distance = [ONE_METRE, ONE_METRE].iter().sum();
    assert_eq!(ONE_METRE.value(total), 3.0);
    assert_eq!(ONE_METRE.value(by_ref), 2.0);
}

/// A unit type defined by a const parameter: `Metres<1000>` is the
/// kilometre. One impl for every size, and a unit of two quantities.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Metres<const N: u32>;

impl<const N: u32> const Unit<Length, f64> for Metres<N> {
    fn quantity(&self, value: f64) -> Length {
        METRE.quantity(value * N as f64)
    }
    fn value(&self, quantity: Length) -> f64 {
        METRE.value(quantity) / N as f64
    }
}

impl<const N: u32> const Unit<Distance, f64> for Metres<N> {
    fn quantity(&self, value: f64) -> Distance {
        ONE_METRE.quantity(value * N as f64)
    }
    fn value(&self, quantity: Distance) -> f64 {
        ONE_METRE.value(quantity) / N as f64
    }
}

impl<const N: u32> core::fmt::Display for Metres<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            1 => f.pad("m"),
            1000 => f.pad("km"),
            _ => write!(f, "{N} m"),
        }
    }
}

/// The units, as constants of their types.
pub const METRES: Metres<1> = Metres;
pub const KILOMETRE: Metres<1000> = Metres;

/// A unit itself, in `const`: its impl is `const`.
const TEN_KM: Length = KILOMETRE.quantity(10.0);

/// Variants naming values of a generic unit type; a unit of each
/// `delegate`.
#[derive(Copy, Clone, Debug, PartialEq, dynamickit::Values)]
#[values(
    delegate(Unit<Length, f64> {
        fn quantity(&self, value: f64) -> Length;
        fn value(&self, quantity: Length) -> f64;
    }),
    delegate(Unit<Distance, f64> {
        fn quantity(&self, value: f64) -> Distance;
        fn value(&self, quantity: Distance) -> f64;
    })
)]
pub enum Lengths {
    #[value(METRES)]
    Metre,
    #[value(KILOMETRE)]
    Kilometre,
}

#[test]
fn variants_name_unit_values() {
    assert_eq!(Lengths::Metre.value(TEN_KM), 10_000.0);
    let d: Distance = Lengths::Kilometre.quantity(2.0);
    assert_eq!(ONE_METRE.value(d), 2000.0);
    assert_eq!(
        format!("{}|{:>3}", Lengths::Metre, Lengths::Kilometre),
        "m| km"
    );
    assert_eq!("km".parse::<Lengths>(), Ok(Lengths::Kilometre));
    assert_eq!("mi".parse::<Lengths>(), Err(dynamickit::NoValue));
}
