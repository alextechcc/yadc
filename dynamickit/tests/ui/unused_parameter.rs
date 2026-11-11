//! A parameter in no fact: two members differing only in it would come back
//! as each other.
#![feature(adt_const_params, const_trait_impl, derive_const, const_cmp, const_destruct)]

pub struct Tagged<const T: u8, const U: u8>(f64);

#[derive(dynamickit::Dynamic)]
#[dynamic(Tagged<const T: u8, const U: u8> => { tag: T, value: .0 })]
pub struct AnyTagged {
    tag: u8,
    value: f64,
}

fn main() {}
