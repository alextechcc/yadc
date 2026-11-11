//! A `shared` mapping is the same value as exactly one other mapping: one
//! that can hold the facts of two is refused, naming both, and so is one
//! that can hold none's.
#![feature(adt_const_params, const_trait_impl, derive_const, const_cmp, const_destruct, const_convert)]

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, core::marker::ConstParamTy)]
pub struct Dim(i8);
pub const NUMBER: Dim = Dim(0);

pub struct Qty<const D: Dim>(f64);
pub struct Ratio<const D: Dim>(f64);
pub struct Num<const T: u8>(f64);

/// `Num<0>` is a `Qty<NUMBER>`, and `Num<1>` a `Ratio<NUMBER>`.
#[derive(dynamickit::Dynamic)]
#[dynamic(Qty<const D: Dim> => { tag: 0u8, dim: D, value: .0 })]
#[dynamic(Ratio<const D: Dim> => { tag: 1u8, dim: D, value: .0 })]
#[dynamic(shared Num<const T: u8> => { tag: T, dim: NUMBER, value: .0 })]
pub struct Reading {
    tag: u8,
    dim: Dim,
    value: f64,
}

/// `shared` on a static type no other mapping can coincide with.
#[derive(dynamickit::Dynamic)]
#[dynamic(Qty<const D: Dim> => { tag: 0u8, dim: D, value: .0 })]
#[dynamic(shared f64 => { tag: 1u8, dim: NUMBER, value: self })]
pub struct Tagged {
    tag: u8,
    dim: Dim,
    value: f64,
}

fn main() {}
