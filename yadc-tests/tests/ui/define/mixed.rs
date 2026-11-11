//! `from(M => N)`: one direction only, on a family with a number parameter;
//! a mixed operator keeps the same-type operator's refusals.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(fn_traits, unboxed_closures, const_destruct)]
#![feature(const_cmp, const_default, derive_const, const_ops, const_convert, const_try)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use yadc::Dimension;

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, Dimension)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
}

/// Listed both ways: `f32 + f64` would have two results.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
#[quantity(from(f32 => f64), from(f64 => f32))]
pub struct BothWays<const D: Dim, N>(N);

/// No number parameter to mix.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
#[quantity(from(f32 => f64))]
pub struct Fixed<const D: Dim>(f64);

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
#[quantity(from(f32 => f64))]
pub struct Qty<const D: Dim, N>(N);

pub const METRE_32: Qty<{ Dim { m: 1, s: 0 } }, f32> = Qty(1.0);
pub const SECOND: Qty<{ Dim { m: 0, s: 1 } }, f64> = Qty(1.0);

fn main() {
    let _ = METRE_32 + SECOND; // cannot add: the dimensions differ
}
