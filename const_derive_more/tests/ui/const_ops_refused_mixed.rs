//! A mixed operator keeps the refusals of the same-type one; so do the
//! in-place operators.
#![feature(generic_const_exprs, adt_const_params, const_trait_impl, const_ops, const_cmp)]
#![feature(derive_const, const_destruct, const_convert)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Tag(u8);

const impl Mul for Tag {
    type Output = Option<Tag>;
    fn mul(self, o: Tag) -> Option<Tag> {
        if self.0 == 9 || o.0 == 9 { None } else { Some(Tag(self.0 + o.0)) }
    }
}
const impl Div for Tag {
    type Output = Option<Tag>;
    fn div(self, o: Tag) -> Option<Tag> {
        if self.0 == 9 || o.0 == 9 { None } else { Some(Tag(self.0 - o.0)) }
    }
}
const impl Add for Tag {
    type Output = Option<Tag>;
    fn add(self, o: Tag) -> Option<Tag> {
        if self == o { Some(self) } else { None }
    }
}
const impl Sub for Tag {
    type Output = Option<Tag>;
    fn sub(self, o: Tag) -> Option<Tag> {
        if self == o { Some(self) } else { None }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, const_derive_more::ConstOps)]
#[const_ops(from(f32 => f64))]
pub struct Tagged<#[ops(one = Tag(0))] const P: Tag, N>(N);

fn main() {
    let a32: Tagged<{ Tag(1) }, f32> = Tagged(1.0);
    let a: Tagged<{ Tag(1) }, f64> = Tagged(1.0);
    let b: Tagged<{ Tag(2) }, f64> = Tagged(1.0);
    let frozen: Tagged<{ Tag(9) }, f64> = Tagged(1.0);
    let _ = a32 + b;
    let _ = b - a32;
    let _ = a32 * frozen;
    let mut c = a;
    c += b;
    c -= b;
    c /= frozen;
}
