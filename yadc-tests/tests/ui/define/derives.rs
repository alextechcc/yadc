//! The derives' own errors: what each attribute must say.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(const_cmp, const_default, derive_const, const_ops, const_convert, const_try)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use yadc::Dimension;

/// No `#[default]`: which label is plain?
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum NoPlain {
    Linear,
    Affine,
}

/// A label declares sums, not products.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum MulRule {
    #[default]
    Plain,
    #[kind(mul(Plain) = Plain)]
    Other,
}

/// A variant says its sums, or its behaviour: `affine` or `log`.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum NotARule {
    #[default]
    Plain,
    #[kind(Affine)]
    Other,
}

/// One variant is the point.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum TwoPoints {
    #[default]
    Plain,
    #[kind(affine)]
    Altitude,
    #[kind(affine)]
    Instant,
}

/// The plain label is an amount, never a point.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum DefaultPoint {
    #[default]
    #[kind(affine)]
    Plain,
}

/// A logarithm's sums are fixed: no rule names one.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum RuleOnLog {
    #[default]
    Plain,
    #[kind(log)]
    Level,
    #[kind(add(Level) = Other)]
    Other,
}

/// `absolute` is a label's; the plain label has its own sums.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum DefaultAbsolute {
    #[default]
    #[kind(absolute)]
    Plain,
}

/// The kind field is marked alone: its variants say which is a point.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub struct BadField {
    pub m: i8,
    #[kind(affine = MulRule::Other)]
    pub kind: MulRule,
}

/// `Cast` changes the number type: a family needs one as a parameter.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
#[quantity(impl(Cast))]
pub struct Fixed<const D: NoPlain>(f64);

/// An unknown extra.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
#[quantity(impl(Neg, Zero))]
pub struct Qty<const D: NoPlain>(f64);

/// A unit metre, to name in the enums below.
#[derive(Copy, Clone)]
pub struct Metre;

impl yadc::Unit<f64, f64> for Metre {
    fn quantity(&self, value: f64) -> f64 {
        value
    }
    fn value(&self, quantity: f64) -> f64 {
        quantity
    }
}

/// The derive is for an enum: a unit type has a hand-written `impl Unit`.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64, number = f64)]
pub struct NotAnEnum;

/// Units that do not say what they measure.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(number = f64)]
pub enum NoTarget {
    #[unit(Metre)]
    Metre,
}

/// Units that do not say which numbers they count in.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64)]
pub enum NoNumber {
    #[unit(Metre)]
    Metre,
}

/// A variant names a unit type, and has no fields.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64, number = f64)]
pub enum WithFields {
    #[unit(Metre)]
    Metre(f64),
}

/// A variant that names no unit type.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64, number = f64)]
pub enum Unnamed {
    Metre,
}

/// An unknown item.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64, number = f64, by = 1.0)]
pub enum Unknown {
    #[unit(Metre)]
    Metre,
}

/// An unknown extra.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64, number = f64, impl(Debug))]
pub enum UnknownExtra {
    #[unit(Metre)]
    Metre,
}

/// The enum's own generics: the impl's go in `for<..>`.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64, number = f64)]
pub enum Generic<T> {
    #[unit(Metre)]
    Metre,
    #[unit(Metre)]
    Other(T),
}

/// A variant's type must be a unit of what the enum measures.
#[derive(Copy, Clone)]
pub struct NotAUnit;

#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64, number = f64, impl())]
pub enum NamesNoUnit {
    #[unit(NotAUnit)]
    Other,
}

fn main() {}
