//! Two plain types told apart only by constants that turn out equal: the
//! `const` assertion fails.
#![feature(const_trait_impl, derive_const, const_cmp, const_destruct, const_convert)]

pub struct Celsius(f64);
pub struct Kelvin(f64);
pub const C: u8 = 1;
pub const K: u8 = 1;

#[derive(const_derive_more::Dynamic)]
#[dynamic(Celsius => { tag: C, value: .0 })]
#[dynamic(Kelvin => { tag: K, value: .0 })]
pub struct Reading {
    tag: u8,
    value: f64,
}

fn main() {}
