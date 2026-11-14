//! An operator `T`'s own operator gives no result for: refused, saying which.
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
pub struct Tagged<#[ops(one = Tag(0))] const P: Tag, N>(N);

/// Two lifted parameters: an operator exists only where both have a result.
#[derive(Copy, Clone, PartialEq, Debug, const_derive_more::ConstOps)]
pub struct Two<#[ops(one = Tag(0))] const P: Tag, #[ops(one = Tag(0))] const Q: Tag, N>(N);

fn main() {
    let a: Tagged<{ Tag(1) }, f64> = Tagged(1.0);
    let b: Tagged<{ Tag(2) }, f64> = Tagged(1.0);
    let frozen: Tagged<{ Tag(9) }, f64> = Tagged(1.0);
    let _ = a + b;
    let _ = a - b;
    let _ = a * frozen;
    let _ = frozen / a;
    let mut c = a;
    c *= b;
    // The first parameters multiply; the second do not.
    let x: Two<{ Tag(1) }, { Tag(1) }, f64> = Two(1.0);
    let y: Two<{ Tag(1) }, { Tag(9) }, f64> = Two(1.0);
    let _ = x * y;
}
