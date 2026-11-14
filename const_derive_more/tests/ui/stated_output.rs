//! The stated output is checked against the fields' own operators: a
//! dimension's `+` returns an `Option`, so `Add -> Self` does not compile;
//! and an operator must state its output.
#![feature(adt_const_params, const_trait_impl, derive_const, const_cmp, const_destruct, const_ops, const_convert)]

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, core::marker::ConstParamTy)]
pub struct Dim(i8);
const impl core::ops::Add for Dim {
    type Output = Option<Dim>;
    fn add(self, r: Dim) -> Option<Dim> {
        if self == r { Some(self) } else { None }
    }
}

pub struct Qty<const D: Dim>(f64);

#[derive(const_derive_more::Dynamic)]
#[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]
#[dynamic(impl(Add -> Self))]
pub struct DynQty {
    dim: Dim,
    value: f64,
}

#[derive(const_derive_more::Dynamic)]
#[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]
#[dynamic(impl(Add))]
pub struct Unstated {
    dim: Dim,
    value: f64,
}

fn main() {}
