//! Sources whose type is not the field's: the error is at the source - a
//! path both ways (moved in by `From`, moved back by `TryFrom`), a constant
//! where it is written.
#![feature(const_trait_impl, derive_const, const_cmp, const_destruct, const_convert)]

pub struct Count(pub u32);

#[derive(const_derive_more::Dynamic)]
#[dynamic(Count => { value: .0 })]
pub struct Reading {
    value: f64,
}

#[derive(const_derive_more::Dynamic)]
#[dynamic(Count => { tag: 1.5, value: .0 })]
pub struct Tagged {
    tag: u8,
    value: u32,
}

fn main() {}
