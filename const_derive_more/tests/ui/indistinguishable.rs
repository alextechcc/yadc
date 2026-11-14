//! A family and a plain type with the same kind of facts and no tag: the way
//! back could not tell a `Qty<TIME>` from an `Instant`.
#![feature(adt_const_params, const_trait_impl, derive_const, const_cmp, const_destruct)]

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, core::marker::ConstParamTy)]
pub struct Dim(i8);
pub const TIME: Dim = Dim(1);

#[derive(Copy, Clone)]
pub struct Qty<const D: Dim>(f64);
#[derive(Copy, Clone)]
pub struct Instant {
    since: Qty<TIME>,
}

#[derive(const_derive_more::Dynamic)]
#[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]
#[dynamic(Instant { since: Qty<TIME> } => { dim: TIME, value: .since.0 })]
pub struct Reading {
    dim: Dim,
    value: f64,
}

fn main() {}
