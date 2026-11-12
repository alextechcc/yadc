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

/// A unit that does not say what it measures.
#[derive(Copy, Clone, yadc::Unit)]
#[unit("m", "metres", by = 1.0)]
pub struct NoTarget;

/// A unit without a description.
#[derive(Copy, Clone, yadc::Unit)]
#[unit("m", "metres", of = f64)]
pub struct NoForm;

/// `Display` needs labels.
#[derive(Copy, Clone, yadc::Unit)]
#[unit(of = f64, by = 1.0)]
pub struct NoLabels;

/// A unit is named by a unit value: there is no other description.
#[derive(Copy, Clone, yadc::Unit)]
#[unit("x", "x", of = f64, step = 1.0)]
pub struct Described;

fn main() {}
