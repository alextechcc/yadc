//! Two static types told apart by constants of a type whose `PartialEq` is
//! not `const`: the constants are compared at compile time, so the error is
//! at the field's type.
#![feature(const_trait_impl, derive_const, const_cmp, const_destruct, const_convert)]

#[derive(Copy, Clone, PartialEq)]
pub struct Tag(u8);

pub struct Celsius(f64);
pub struct Kelvin(f64);

#[derive(dynamickit::Dynamic)]
#[dynamic(Celsius => { tag: Tag(0), value: .0 })]
#[dynamic(Kelvin => { tag: Tag(1), value: .0 })]
pub struct Reading {
    tag: Tag,
    value: f64,
}

fn main() {}
