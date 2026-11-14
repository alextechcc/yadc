//! A deep path through a tuple field: the way back rebuilds each declared
//! field as a struct expression, so a tuple type cannot be declared.
#![feature(const_trait_impl, derive_const, const_cmp, const_destruct, const_convert)]

pub struct Span {
    pub ends: (f64, f64),
}

#[derive(const_derive_more::Dynamic)]
#[dynamic(Span { ends: (f64, f64) } => { start: .ends.0, end: .ends.1 })]
pub struct Reading {
    start: f64,
    end: f64,
}

fn main() {}
