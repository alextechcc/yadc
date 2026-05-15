#![cfg_attr(not(feature = "std"), no_std)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(structural_match)]
#![allow(incomplete_features)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::approx_constant)]

use yadc::{Dimension, DynamicQuantity, Kind, StaticQuantity};
#[cfg(feature = "serde")]
use yadc::{DimensionDeserialize, DimensionSerialize};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// The Kind enum tags non-numeric "kind" of a quantity. It's mutually exclusive
// with the seven SI numeric axes, and lets the type system distinguish quantities
// that share the same base dimension (Angle vs Dimensionless, T vs ΔT, etc.).
//
// `#[derive(Kind)]` generates ConstParamTy_, ZERO/INVALID consts, impl const
// PartialEq/Eq, and impl const Mul/Div/Add/Sub.
// Variant names match the standard roles; rename with `#[kind(<Role>)]` to use
// custom variant identifiers.
#[derive(Copy, Clone, Debug, Kind)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Kind {
    Linear,
    Affine,
    Angle,
    SolidAngle,
    Invalid,
}

#[derive(Copy, Clone, Debug, Dimension)]
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(DimensionSerialize, DimensionDeserialize))]
pub struct ISQ {
    pub m: i8,
    pub kg: i8,
    pub s: i8,
    pub A: i8,
    pub K: i8,
    pub mol: i8,
    pub cd: i8,
    #[kind(annotate)]
    pub kind: Kind,
}

// StaticSIQuantity serializes transparently as just `f64` (its only field). The dimension
// `D` is determined by the deserialization target — schema-trusted, no dim
// metadata on the wire. If you need the dim recorded too, hand-roll a custom
// impl on `StaticSIQuantity` (the inner f64 is reachable from this module).
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, StaticQuantity)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StaticSIQuantity<const D: ISQ>(f64);

#[derive(Copy, Clone, PartialEq, Debug, DynamicQuantity)]
#[static_quantity(StaticSIQuantity)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DynamicSIQuantity {
    value: f64,
    dim: ISQ,
}

pub mod dimensionless;

#[cfg(feature = "uom")]
pub mod uom_bridge;

#[cfg(feature = "angular")]
pub mod angular;
#[cfg(feature = "chemistry")]
pub mod chemistry;
#[cfg(feature = "density")]
pub mod density;
#[cfg(feature = "electromagnetic")]
pub mod electromagnetic;
#[cfg(feature = "fluid")]
pub mod fluid;
#[cfg(feature = "information")]
pub mod information;
#[cfg(feature = "mechanical")]
pub mod mechanical;
#[cfg(feature = "photometric")]
pub mod photometric;
#[cfg(feature = "thermal")]
pub mod thermal;
