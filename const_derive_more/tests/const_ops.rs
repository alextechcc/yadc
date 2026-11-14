//! `#[derive(ConstOps)]`: a value and const parameters, lifted together.
#![feature(
    generic_const_exprs,
    adt_const_params,
    const_trait_impl,
    const_ops,
    const_cmp
)]
#![feature(derive_const, const_destruct, const_convert)]
#![allow(incomplete_features)]

use const_derive_more::ConstOps;
use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

/// A power of some base; `frozen` powers have no products.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Pow {
    exp: i8,
    frozen: bool,
}

impl Pow {
    pub const ONE: Pow = Pow {
        exp: 0,
        frozen: false,
    };
    pub const fn of(exp: i8) -> Pow {
        Pow { exp, frozen: false }
    }
}

const impl Mul for Pow {
    type Output = Option<Pow>;
    fn mul(self, o: Pow) -> Option<Pow> {
        if self.frozen || o.frozen {
            None
        } else {
            Some(Pow::of(self.exp + o.exp))
        }
    }
}
const impl Div for Pow {
    type Output = Option<Pow>;
    fn div(self, o: Pow) -> Option<Pow> {
        if self.frozen || o.frozen {
            None
        } else {
            Some(Pow::of(self.exp - o.exp))
        }
    }
}
const impl Add for Pow {
    type Output = Option<Pow>;
    fn add(self, o: Pow) -> Option<Pow> {
        if self == o { Some(self) } else { None }
    }
}
const impl Sub for Pow {
    type Output = Option<Pow>;
    fn sub(self, o: Pow) -> Option<Pow> {
        if self == o { Some(self) } else { None }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, ConstOps)]
#[const_ops(from(i32 => f64))]
pub struct Tagged<#[ops(one = Pow::ONE)] const P: Pow, N>(N);

/// A second const parameter, passed through.
#[derive(Copy, Clone, PartialEq, Debug, ConstOps)]
pub struct Marked<#[ops(one = Pow::ONE)] const P: Pow, N, const K: bool>(N);

/// Two lifted parameters, each combined by its type's own operators.
#[derive(Copy, Clone, PartialEq, Debug, ConstOps)]
pub struct Both<#[ops(one = Pow::ONE)] const P: Pow, N, #[ops(one = Pow::ONE)] const Q: Pow>(N);

const X: Tagged<{ Pow::of(1) }, f64> = Tagged(3.0);
const Y: Tagged<{ Pow::of(2) }, f64> = Tagged(2.0);

#[test]
fn operators() {
    let p: Tagged<{ Pow::of(3) }, f64> = X * Y;
    assert_eq!(p, Tagged(6.0));
    let q: Tagged<{ Pow::of(-1) }, f64> = X / Y;
    assert_eq!(q, Tagged(1.5));
    assert_eq!(X + X, Tagged(6.0));
    assert_eq!(X - X, Tagged(0.0));
    #[allow(clippy::op_ref)]
    let borrowed = X * &Y;
    assert_eq!(borrowed, p);
    assert_eq!(-X, Tagged(-3.0));
    assert_eq!(X * 2.0, Tagged(6.0));
    assert_eq!(2.0 * X, Tagged(6.0));
    assert_eq!(X / 2.0, Tagged(1.5));
    let mut a = X;
    a += X;
    a *= 2.0;
    a -= &X;
    assert_eq!(a, Tagged(9.0));
    let s: Tagged<{ Pow::of(1) }, f64> = [X, X, X].into_iter().sum();
    assert_eq!(s, Tagged(9.0));
    let r: Tagged<{ Pow::of(0) }, f64> = X / X;
    assert_eq!(r, Tagged(1.0));
    let inverse: Tagged<{ Pow::of(-1) }, f64> = 6.0 / X;
    assert_eq!(inverse, Tagged(2.0));
}

#[test]
fn in_const() {
    const P: Tagged<{ Pow::of(3) }, f64> = X * Y;
    assert_eq!(P, Tagged(6.0));
}

#[test]
fn mixed_numbers() {
    let i: Tagged<{ Pow::of(1) }, i32> = Tagged(2);
    let f: Tagged<{ Pow::of(3) }, f64> = i * Y;
    assert_eq!(f, Tagged(4.0));
    assert_eq!(i + X, Tagged(5.0));
    assert_eq!(1.5 * i, Tagged(3.0));
    assert!(i < X);
    assert_eq!(Tagged::<{ Pow::of(1) }, f64>::from(i), Tagged(2.0));
    let wide: Tagged<{ Pow::of(1) }, i64> = i.into();
    assert_eq!(wide, Tagged(2));
}

#[test]
fn passed_through() {
    let a: Marked<{ Pow::of(1) }, f64, true> = Marked(2.0);
    let b: Marked<{ Pow::of(2) }, f64, true> = a * a;
    assert_eq!(b, Marked(4.0));
}

#[test]
fn two_lifted_parameters() {
    let a: Both<{ Pow::of(1) }, f64, { Pow::of(2) }> = Both(3.0);
    let b: Both<{ Pow::of(2) }, f64, { Pow::of(4) }> = a * a;
    assert_eq!(b, Both(9.0));
    let c: Both<{ Pow::of(0) }, f64, { Pow::of(-2) }> =
        a / Both::<{ Pow::of(1) }, f64, { Pow::of(4) }>(3.0);
    assert_eq!(c, Both(1.0));
    let d: Both<{ Pow::of(-1) }, f64, { Pow::of(-2) }> = 6.0 / a;
    assert_eq!(d, Both(2.0));
    assert_eq!(a + a, Both(6.0));
}
