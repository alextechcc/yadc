//! `#[derive(ConstOps)]`: a monomial `c·xᵈ`, its coefficient `c` a number
//! and its degree `d` in its type. Terms of one degree add; a product adds
//! the degrees; a quotient subtracts them and exists only where the result
//! is still a monomial (no negative degree).
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

/// A degree, with its own operators: each `Option`, `None` where there is
/// no result.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Degree(u8);

const impl Mul for Degree {
    type Output = Option<Degree>;
    fn mul(self, o: Degree) -> Option<Degree> {
        match self.0.checked_add(o.0) {
            Some(d) => Some(Degree(d)),
            None => None,
        }
    }
}
const impl Div for Degree {
    type Output = Option<Degree>;
    fn div(self, o: Degree) -> Option<Degree> {
        match self.0.checked_sub(o.0) {
            Some(d) => Some(Degree(d)),
            None => None,
        }
    }
}
/// Terms add only to terms of their own degree.
const impl Add for Degree {
    type Output = Option<Degree>;
    fn add(self, o: Degree) -> Option<Degree> {
        if self == o { Some(self) } else { None }
    }
}
const impl Sub for Degree {
    type Output = Option<Degree>;
    fn sub(self, o: Degree) -> Option<Degree> {
        if self == o { Some(self) } else { None }
    }
}

/// `c·xᵈ`. Degree 0, a constant, is the plain number: `one = Degree(0)`.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, ConstOps)]
#[const_ops(from(i32 => f64))]
pub struct Term<#[ops(one = Degree(0))] const D: Degree, N>(N);

/// `x`, with an integer coefficient.
const X: Term<{ Degree(1) }, i32> = Term(1);

fn main() {
    // `3x · 2x² = 6x³`: `Mul` for `Term<A, N>` and `Term<B, N>`, the result
    // `Term<{ A * B }, N>`; it exists because `Degree(1) * Degree(2)` is
    // `Some`.
    let a: Term<{ Degree(1) }, f64> = Term(3.0);
    let b: Term<{ Degree(2) }, f64> = Term(2.0);
    let c: Term<{ Degree(3) }, f64> = a * b;
    assert_eq!(c, Term(6.0));

    // `6x³ / 3x = 2x²`; `3x / 2x²` has no degree, so it does not compile:
    // "cannot divide: `/` of these parameters has no result".
    let d: Term<{ Degree(2) }, f64> = c / a;
    assert_eq!(d, Term(2.0));

    // Terms of one degree add; `a + b` does not compile.
    assert_eq!(a + a, Term(6.0));

    // Scaling, from either side, and a mixed number type: `from(i32 =>
    // f64)` lets an `i32` term meet an `f64` one, the result in `f64`.
    let e: Term<{ Degree(1) }, f64> = 2.5 * X;
    assert_eq!(e, Term(2.5));
    assert_eq!(X + a, Term(4.0));

    // A quotient of two terms of one degree is of degree zero.
    let same = a;
    assert_eq!(a / same, Term::<{ Degree(0) }, f64>(1.0));

    // In a `const`: every operator here is a `const impl`.
    const TWO_X_SQUARED: Term<{ Degree(2) }, i32> = X * X * 2;
    assert_eq!(TWO_X_SQUARED, Term(2));

    // By hand, the product above is
    //
    //     const impl<N, const A: Degree, const B: Degree> Mul<Term<B, N>> for Term<A, N>
    //     where
    //         Term<{ product(A, B) }, N>: Sized,   // `A * B`, or `Degree(0)` if `None`
    //         Term<A, N>: MulResult<Term<B, N>>,   // `A * B` is `Some`
    //         N: [const] Mul<Output = N> + [const] Destruct,
    //     {
    //         type Output = Term<{ product(A, B) }, N>;
    //         fn mul(self, rhs: Term<B, N>) -> Self::Output { Term(self.0 * rhs.0) }
    //     }
    println!("{c:?} / {a:?} = {d:?}");
}
