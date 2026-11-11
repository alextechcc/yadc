//! A crate that defines its system and misuses it in place.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(fn_traits, unboxed_closures, const_destruct)]
#![feature(const_cmp, const_default, derive_const, const_ops, const_convert, const_try)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use yadc::Dimension;
use yadc::{PowiAs, RootAs, Unit};

#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum Kind {
    #[default]
    Linear,
    Affine,
    Log,
}

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
    pub k: i8,
    #[kind(affine = Kind::Affine, log = Kind::Log)]
    pub kind: Kind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
pub struct Qty<const D: Dim>(f64);

pub type Length = Qty<{ Dim { m: 1, ..Dim::DIMENSIONLESS } }>;
pub type Time = Qty<{ Dim { s: 1, ..Dim::DIMENSIONLESS } }>;
pub type Temperature = Qty<{ Dim { k: 1, kind: Kind::Affine, ..Dim::DIMENSIONLESS } }>;
pub type TemperatureStep = Qty<{ Dim { k: 1, ..Dim::DIMENSIONLESS } }>;
pub type Altitude = Qty<{ Dim { m: 1, kind: Kind::Affine, ..Dim::DIMENSIONLESS } }>;
pub type Gain = Qty<{ Dim { kind: Kind::Log, ..Dim::DIMENSIONLESS } }>;
pub type LengthLevel = Qty<{ Dim { m: 1, kind: Kind::Log, ..Dim::DIMENSIONLESS } }>;

pub const METRE: Length = Qty(1.0);
pub const SECOND: Time = Qty(1.0);
pub const KELVIN: TemperatureStep = Qty(1.0);
pub const ABSOLUTE_ZERO: Temperature = Qty(0.0);
pub const SEA_LEVEL: Altitude = Qty(0.0);
pub const BEL: Gain = Qty(1.0);
pub const METRE_LEVEL: LengthLevel = Qty(0.0);

// Refused in `const` items too.
const _: () = {
    let _ = SEA_LEVEL * METRE;
};

fn main() {
    let _ = METRE + SECOND;
    let _ = SEA_LEVEL + SEA_LEVEL;
    let _ = METRE + SEA_LEVEL;
    let _ = KELVIN + ABSOLUTE_ZERO;
    let _ = METRE - SECOND;
    let _ = METRE - SEA_LEVEL;
    let _ = SEA_LEVEL - ABSOLUTE_ZERO;
    let _ = METRE_LEVEL + METRE_LEVEL;
    let _ = BEL + METRE_LEVEL;
    let _ = METRE_LEVEL * 2.0;
    let _ = METRE_LEVEL * METRE;
    let _ = KELVIN - ABSOLUTE_ZERO;
    let _ = SEA_LEVEL * 2.0;
    let _ = SEA_LEVEL / SECOND;
    let _ = SEA_LEVEL.powi::<2>();
    let _ = METRE.root::<2>();
    let _ = SEA_LEVEL.value(SEA_LEVEL);
}
