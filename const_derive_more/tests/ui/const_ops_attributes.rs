//! What `#[const_ops(..)]`, `#[ops(..)]` and the struct's shape must say.
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

/// Listed both ways: `f32 + f64` would have two results.
#[derive(const_derive_more::ConstOps)]
#[const_ops(from(f32 => f64), from(f64 => f32))]
pub struct BothWays<#[ops(one = Tag(0))] const P: Tag, N>(N);

/// A number type meets itself already.
#[derive(const_derive_more::ConstOps)]
#[const_ops(from(f64 => f64))]
pub struct Itself<#[ops(one = Tag(0))] const P: Tag, N>(N);

/// No number parameter to mix.
#[derive(const_derive_more::ConstOps)]
#[const_ops(from(f32 => f64))]
pub struct Fixed<#[ops(one = Tag(0))] const P: Tag>(f64);

/// A lifted parameter without its `one`.
#[derive(const_derive_more::ConstOps)]
pub struct NoOne<#[ops] const P: Tag, N>(N);

/// An unknown option.
#[derive(const_derive_more::ConstOps)]
#[const_ops(zero = Tag(0))]
pub struct Unknown<#[ops(one = Tag(0))] const P: Tag, N>(N);

/// No lifted parameter.
#[derive(const_derive_more::ConstOps)]
pub struct NoneLifted<const P: Tag, N>(N);

/// Two value fields.
#[derive(const_derive_more::ConstOps)]
pub struct TwoValues<#[ops(one = Tag(0))] const P: Tag>(f64, f64);

fn main() {}
