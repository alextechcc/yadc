//! Shapes the derives accept: a quantity with only some of its extras, a unit
//! with fields of its own, and an enum whose variants carry their unit value.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(const_cmp, const_default, derive_const, const_ops, const_convert, const_try)]
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

/// A unit with a field of its own: it has no value to parse into, so no
/// `FromStr`; it still prints and measures.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = Length)]
#[unit("m", "metres", by = METRE)]
pub struct Rounded {
    pub digits: u8,
}

#[test]
fn unit_with_fields() {
    let u = Rounded { digits: 2 };
    assert_eq!(u.value(u.quantity(1.5)), 1.5);
    assert_eq!(format!("{} {:#} {}", u, u, u.digits), "m metres 2");
}

/// Each variant carries its unit value in a field.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = Length, impl())]
pub enum Measured {
    Ruler(#[unit(by)] Length),
    Tape(#[unit(by)] Length),
}

#[test]
fn variants_carry_their_unit() {
    let tape = Measured::Tape(100.0 * METRE);
    assert_eq!(tape.value(tape.quantity(2.0)), 2.0);
    assert_eq!(METRE.value(Measured::Ruler(0.3 * METRE).quantity(1.0)), 0.3);
}

/// Fieldless units of every shape parse: `struct U;`, `U()`, `U {}`.
#[derive(Copy, Clone, Debug, PartialEq, yadc::Unit)]
#[unit(of = Length)]
pub enum Shapes {
    #[unit("a", "as", by = METRE)]
    Unit,
    #[unit("b", "bs", by = METRE)]
    Tuple(),
    #[unit("c", "cs", by = METRE)]
    Braced {},
}

#[test]
fn fieldless_shapes_parse() {
    assert_eq!("b".parse::<Shapes>(), Ok(Shapes::Tuple()));
    assert_eq!("c".parse::<Shapes>(), Ok(Shapes::Braced {}));
}
