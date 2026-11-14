//! `#[derive(ConstOps)]` with several number types: `from(f32 => f64)` and
//! `from(i32 => f64)` on one struct, every operator both ways round, and
//! the shapes of struct it takes.
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
use core::marker::{ConstParamTy, PhantomData};
use core::ops::{Add, Div, Mul, Sub};

/// The exponents of metres and seconds.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Dim {
    m: i8,
    s: i8,
}

const impl Mul for Dim {
    type Output = Option<Dim>;
    fn mul(self, o: Dim) -> Option<Dim> {
        Some(Dim {
            m: self.m + o.m,
            s: self.s + o.s,
        })
    }
}
const impl Div for Dim {
    type Output = Option<Dim>;
    fn div(self, o: Dim) -> Option<Dim> {
        Some(Dim {
            m: self.m - o.m,
            s: self.s - o.s,
        })
    }
}
const impl Add for Dim {
    type Output = Option<Dim>;
    fn add(self, o: Dim) -> Option<Dim> {
        if self == o { Some(self) } else { None }
    }
}
const impl Sub for Dim {
    type Output = Option<Dim>;
    fn sub(self, o: Dim) -> Option<Dim> {
        if self == o { Some(self) } else { None }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, ConstOps)]
#[const_ops(from(f32 => f64), from(i32 => f64))]
pub struct Qty<#[ops(one = Dim { m: 0, s: 0 })] const D: Dim, N>(N);

type Length<N> = Qty<{ Dim { m: 1, s: 0 } }, N>;
type Time<N> = Qty<{ Dim { m: 0, s: 1 } }, N>;
type Velocity<N> = Qty<{ Dim { m: 1, s: -1 } }, N>;

const METRE: Length<f64> = Qty(1.0);
const SECOND: Time<f64> = Qty(1.0);
/// Counted in `f32`.
const KILOMETRE_32: Length<f32> = Qty(1000.0);
/// Counted in `i32`.
const HOUR_I: Time<i32> = Qty(3600);

/// Mixed in a `const` item.
const LIMIT: Velocity<f64> = 50.0 * KILOMETRE_32 / HOUR_I;

#[test]
fn the_result_counts_in_the_target() {
    let trip: Length<f64> = METRE * 250.0;
    let total: Length<f64> = trip + KILOMETRE_32;
    assert_eq!(total, KILOMETRE_32 + trip);
    assert_eq!(total, METRE * 1250.0);
    assert_eq!(KILOMETRE_32 - trip, METRE * 750.0);
    assert_eq!(trip - KILOMETRE_32, METRE * -750.0);
    let speed: Velocity<f64> = total / HOUR_I;
    assert_eq!(speed, METRE / SECOND * (1250.0 / 3600.0));
    let back: Length<f64> = HOUR_I * speed;
    assert!(((back - total) / METRE).0.abs() < 1e-9);
    assert!(((LIMIT * HOUR_I / KILOMETRE_32).0 - 50.0).abs() < 1e-9);
}

#[test]
fn scaled_by_the_target_from_either_side() {
    let a: Length<f64> = KILOMETRE_32 * 2.5;
    let b: Length<f64> = 2.5 * KILOMETRE_32;
    assert_eq!(a, b);
    assert_eq!(a, METRE * 2500.0);
}

#[test]
fn compared_either_way() {
    let d: Length<f64> = METRE * 999.0;
    assert!(d < KILOMETRE_32 && KILOMETRE_32 > d);
    assert!(METRE * 1000.0 == KILOMETRE_32 && KILOMETRE_32 == METRE * 1000.0);
}

#[test]
fn in_place() {
    let mut d: Length<f64> = METRE * 1.0;
    d += KILOMETRE_32;
    d -= KILOMETRE_32;
    assert_eq!(d, METRE);
}

#[test]
fn converted() {
    assert_eq!(Length::<f64>::from(KILOMETRE_32), METRE * 1000.0);
    assert_eq!((HOUR_I / HOUR_I).0, 1);
}

/// `Sum`, owned and borrowed.
#[test]
fn sums() {
    let owned: Length<f64> = [METRE, METRE * 2.0].into_iter().sum();
    let borrowed: Length<f64> = [METRE, METRE].iter().sum();
    assert_eq!(owned, METRE * 3.0);
    assert_eq!(borrowed, METRE * 2.0);
}

/// A named value field beside a `PhantomData`, and a type parameter passed
/// through.
#[derive(Copy, Clone, PartialEq, Debug, ConstOps)]
pub struct Tagged<#[ops(one = Dim { m: 0, s: 0 })] const D: Dim, N, F> {
    #[value]
    value: N,
    frame: PhantomData<F>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct World;

#[test]
fn named_value_and_phantom() {
    let a: Tagged<{ Dim { m: 1, s: 0 } }, f64, World> = Tagged {
        value: 3.0,
        frame: PhantomData,
    };
    let area: Tagged<{ Dim { m: 2, s: 0 } }, f64, World> = a * a;
    assert_eq!(area.value, 9.0);
    assert_eq!((a + a).value, 6.0);
    assert_eq!((2.0 * a).value, 6.0);
}
