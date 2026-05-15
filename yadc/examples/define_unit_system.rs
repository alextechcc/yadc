//! Define a unit system from scratch: kind, dim, quantity, and unit enums -
//! covering both pure-linear (Length, Area) and affine (Temperature)
//! families. No external `yadc-si` - every type below is built locally.

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(structural_match)]
#![allow(incomplete_features)]

use yadc::{Dimension, Format, StaticQuantity, Unit};

// --- Kind --------------------------------------------------------------------
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

// --- Dim ---------------------------------------------------------------------
// One numeric axis per base dimension (we only need `m` here) plus the
// mandatory `#[kind]` field.

#[derive(Copy, Clone, Debug, Dimension)]
pub struct Dim {
    pub m: i8,
    pub k: i8,
    #[kind]
    pub kind: Kind,
}

// --- StaticQuantity ----------------------------------------------------------
// Single generic type parameterized by a `Dim` const-generic.

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, StaticQuantity)]
pub struct Qty<const D: Dim>(f64);

#[derive(Copy, Clone, PartialEq, yadc::DynamicQuantity)]
#[static_quantity(Qty)]
pub struct DynQty {
    value: f64,
    dim: Dim,
}

// --- Concrete quantities + units ---------------------------------------------

pub type Length = Qty<{ Dim { m: 1, ..Dim::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Length)]
pub enum LengthUnit {
    #[linear("m", "meters", 1.0)]
    Meter,
    #[linear("km", "kilometers", 1000.0)]
    Kilometer,
}

pub type Area = Qty<{ Dim { m: 2, ..Dim::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(Area)]
pub enum AreaUnit {
    #[linear("m²", "square meters", 1.0)]
    SquareMeter,
    #[linear("km²", "square kilometers", 1_000_000.0)]
    SquareKilometer,
}

// --- Affine quantity (Temperature) and its Linear delta type -----------------
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

// Affine unit: the `offset` in `#[affine("s", "l", scale, offset)]` shifts the
// origin (canonical = (value + offset) * scale, so Celsius = Kelvin - 273.15).
// Conversion happens in scalar value-space, so no separate "delta" type is needed.
#[derive(Copy, Clone, Unit)]
#[dim(Temperature)]
pub enum TempUnit {
    #[linear("K", "kelvin", 1.0)]
    Kelvin,
    #[affine("°C", "celsius", 1.0, 273.15)]
    Celsius,
}

// --- Quick smoke test --------------------------------------------------------

fn main() {
    // Pure-linear: quantity / from_quantity / Mul composes Dim.
    let side = LengthUnit::Kilometer.quantity(2.5);
    let area: Area = side * side;
    println!(
        "side = {}",
        LengthUnit::Kilometer.format_quantity(side, Format::Long)
    );
    println!(
        "area = {}  ({:.0})",
        AreaUnit::SquareKilometer.format_quantity(area, Format::Long),
        AreaUnit::SquareMeter.format_quantity(area, Format::Short),
    );
    println!("side as f64 m: {}", LengthUnit::Meter.from_quantity(side));

    // Affine: `quantity` math is `value*M + O` then lift to `Temperature`;
    // `from_quantity` reads back as `(SI - O)/M`. A unit names exactly one
    // quantity, so the dimension is checked structurally (`Unit<Quantity = _>`).
    let body = TempUnit::Celsius.quantity(36.6);
    println!(
        "body temp = {} (= {})",
        TempUnit::Celsius.format_quantity(body, Format::Short),
        TempUnit::Kelvin.format_quantity(body, Format::Short),
    );

    let same = Length::DIM == Length::DIM;
    let diff = Length::DIM == Area::DIM;
    println!("dim eq(L==L) = {same}, dim eq(L==A) = {diff}");

    // DynQty: dim carried at runtime; dim-mismatched read returns None.
    let dyn_len = DynQty::from(LengthUnit::Meter.quantity(1500.0));
    println!(
        "DynQty len in km = {:?}",
        LengthUnit::Kilometer.try_from_quantity(dyn_len),
    );
    println!(
        "DynQty len in m² (mismatched) = {:?}",
        AreaUnit::SquareMeter.try_from_quantity(dyn_len),
    );
}
