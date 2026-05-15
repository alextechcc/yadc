//! Define a unit system from scratch: kind, dim, quantity, and unit enums —
//! covering both pure-linear (Length, Area) and affine (Temperature)
//! families. No external `yadc-si` — every type below is built locally.

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(structural_match)]
#![allow(incomplete_features)]

use yadc::{Dimension, DynamicQuantity, Format, StaticQuantity, Unit};

// ─── Kind ────────────────────────────────────────────────────────────────────
// The five mandatory roles. `#[derive(yadc::Kind)]` generates `ConstParamTy_`,
// `impl yadc::Kind` (ZERO/INVALID/name), `impl const PartialEq/Eq`, and
// `impl const Mul/Div/Add/Sub`.

#[derive(Copy, Clone, Debug, yadc::Kind)]
pub enum Kind {
    Linear,
    Affine,
    Angle,
    SolidAngle,
    Invalid,
}

// ─── Dim ─────────────────────────────────────────────────────────────────────
// One numeric axis per base dimension (we only need `m` here) plus the
// mandatory `#[kind]` field.

#[derive(Copy, Clone, Debug, Dimension)]
pub struct Dim {
    pub m: i8,
    pub k: i8,
    #[kind]
    pub kind: Kind,
}

// ─── StaticQuantity ──────────────────────────────────────────────────────────
// Single generic type parameterized by a `Dim` const-generic.

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, StaticQuantity)]
pub struct Qty<const D: Dim>(f64);

#[derive(Copy, Clone, PartialEq, yadc::DynamicQuantity)]
#[static_quantity(Qty)]
pub struct DynQty {
    value: f64,
    dim: Dim,
}

// ─── Concrete quantities + units ─────────────────────────────────────────────

pub type Length = Qty<{ Dim { m: 1, ..Dim::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Length)]
pub enum LengthUnit {
    #[unit(1.0, "m", "meters")]
    Meter,
    #[unit(1000.0, "km", "kilometers")]
    Kilometer,
}

pub type Area = Qty<{ Dim { m: 2, ..Dim::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Area)]
pub enum AreaUnit {
    #[unit(1.0, "m²", "square meters")]
    SquareMeter,
    #[unit(1_000_000.0, "km²", "square kilometers")]
    SquareKilometer,
}

// ─── Affine quantity (Temperature) and its Linear delta type ─────────────────
// Same numeric axes (k = 1), different `kind`. Subtracting two
// `Temperature` values produces a `TemperatureDelta`; adding a delta back to
// an absolute keeps it absolute. The kind algebra in the StaticQuantity derive's
// `Add`/`Sub` impls makes this work without any per-type code.

pub type Temperature = Qty<
    {
        Dim {
            k: 1,
            kind: Kind::Affine,
            ..Dim::ZERO
        }
    },
>;
pub type TemperatureDelta = Qty<{ Dim { k: 1, ..Dim::ZERO } }>;

// Affine unit: `#[dim_delta(...)]` points at the Linear-kind delta type used
// as `Unit::Multiplier` (a slope, K per °C). Without `#[dim_delta(...)]` the
// derive defaults `Multiplier = Offset`, which is correct only for
// pure-linear units like `LengthUnit` above.
#[derive(Copy, Clone, Unit)]
#[dim(Temperature)]
#[dim_delta(TemperatureDelta)]
pub enum TempUnit {
    #[unit(1.0, "K", "kelvin")]
    Kelvin,
    #[unit(1.0, "°C", "celsius", 273.15)]
    Celsius,
}

// ─── Quick smoke test ────────────────────────────────────────────────────────

fn main() {
    // Pure-linear: from_unit / as_unit / Mul composes Dim.
    let side = Length::from_unit(2.5, LengthUnit::Kilometer);
    let area: Area = side * side;
    println!(
        "side = {}",
        side.fmt_as_unit(LengthUnit::Kilometer, Format::Long)
    );
    println!(
        "area = {}  ({:.0})",
        area.fmt_as_unit(AreaUnit::SquareKilometer, Format::Long),
        area.fmt_as_unit(AreaUnit::SquareMeter, Format::Short),
    );
    println!("side as f64 m: {}", side.as_unit(LengthUnit::Meter));

    // Affine: from_unit math is `Linear * V + Affine = Affine` → `Temperature`,
    // as_unit math is `Affine - Affine = Linear`, `Linear / Linear =
    // Dimensionless` → derefs to V. Both directions are dim-checked at compile
    // time via `CanUseWithUnit`.
    let body = Temperature::from_unit(36.6, TempUnit::Celsius);
    println!(
        "body temp = {} (= {})",
        body.fmt_as_unit(TempUnit::Celsius, Format::Short),
        body.fmt_as_unit(TempUnit::Kelvin, Format::Short),
    );

    let same = Length::DIM == Length::DIM;
    let diff = Length::DIM == Area::DIM;
    println!("dim eq(L==L) = {same}, dim eq(L==A) = {diff}");

    // DynQty: dim carried at runtime, dim-mismatched `as_unit` returns None.
    let dyn_len = DynQty::from_unit(1500.0, LengthUnit::Meter);
    println!(
        "DynQty len in km = {:?}",
        dyn_len.as_unit(LengthUnit::Kilometer),
    );
    println!(
        "DynQty len in m² (mismatched) = {:?}",
        dyn_len.as_unit(AreaUnit::SquareMeter),
    );
}
