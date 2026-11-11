//! Number types that meet: `#[quantity(from(M => N))]`. A member counted in
//! `M` meets one counted in `N` and the result is counted in `N`, converted
//! with std `From<M> for N`.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(fn_traits, unboxed_closures, const_destruct)]
#![feature(const_cmp, const_default, derive_const, const_ops, const_convert, const_try)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use yadc::{Dimension, Unit};

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
#[quantity(from(f32 => f64), from(i32 => f64))]
pub struct Qty<const D: Dim, N>(N);

pub type Length<N> = Qty<{ Dim { m: 1, ..Dim::DIMENSIONLESS } }, N>;
pub type Time<N> = Qty<{ Dim { s: 1, ..Dim::DIMENSIONLESS } }, N>;
pub type Velocity<N> = Qty<{ Dim { m: 1, s: -1 } }, N>;

pub const METRE: Length<f64> = Qty(1.0);
pub const SECOND: Time<f64> = Qty(1.0);
/// Counted in `f32`, used with `f64` values.
pub const KILOMETRE_32: Length<f32> = Qty(1000.0);
/// Counted in `i32`.
pub const HOUR_I: Time<i32> = Qty(3600);

/// Mixed in `const` items too.
const LIMIT: Velocity<f64> = 50.0 * KILOMETRE_32 / HOUR_I;

#[test]
fn operators_count_in_the_target() {
    let trip: Length<f64> = METRE * 250.0;
    let total: Length<f64> = trip + KILOMETRE_32; // f64 + f32
    let total_2: Length<f64> = KILOMETRE_32 + trip; // f32 + f64: the sum keeps the left's dimension
    assert_eq!(METRE.value(total), 1250.0);
    assert_eq!(total, total_2);
    let speed: Velocity<f64> = total / HOUR_I; // f64 / i32
    assert_eq!((METRE / SECOND).value(speed), 1250.0 / 3600.0);
    let back: Length<f64> = speed * HOUR_I; // f64 * i32
    assert!((METRE.value(back) - 1250.0).abs() < 1e-9);
    assert_eq!(METRE.value(KILOMETRE_32 - trip), 750.0);
    assert!((KILOMETRE_32.value(LIMIT * HOUR_I) - 50.0).abs() < 1e-9);
}

#[test]
fn an_m_member_is_a_unit_of_n_members() {
    let d: Length<f64> = KILOMETRE_32(3.0); // build an f64 length
    assert_eq!(METRE.value(d), 3000.0);
    assert_eq!(KILOMETRE_32(d), 3.0); // read it, as f64
    assert_eq!(KILOMETRE_32.value(METRE * 500.0), 0.5);
    let scaled: Length<f64> = KILOMETRE_32 * 2.5; // scaled by an f64
    let scaled_l: Length<f64> = 2.5 * KILOMETRE_32;
    assert_eq!(scaled, scaled_l);
    assert_eq!(METRE.value(scaled), 2500.0);
}

#[test]
fn compared_either_way() {
    let d: Length<f64> = METRE * 999.0;
    assert!(d < KILOMETRE_32 && KILOMETRE_32 > d);
    assert!(METRE * 1000.0 == KILOMETRE_32 && KILOMETRE_32 == METRE * 1000.0);
}

#[test]
fn in_place() {
    let mut d: Length<f64> = METRE * 1.0;
    d += KILOMETRE_32;
    d -= KILOMETRE_32;
    assert_eq!(METRE.value(d), 1.0);
}
