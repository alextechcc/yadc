//! **yadc** - *Yet Another Dimensional Crate.*
//!
//! Core traits and derive macros for building a compile-time-checked unit
//! system. The user defines the dimension layout (a struct of `i8` axes plus
//! an optional `#[kind]` field), the kind enum, and the quantity/unit types;
//! the derives wire up arithmetic, formatting, the trait method bodies, and
//! (optionally) serde.
//!
//! Two quantity flavors share one [`Quantity`] marker trait (`type D` + `type V`):
//! - `Qty<const D: Dim>` - dimension carried in the type, checked at compile
//!   time.
//! - `DynQty` - dimension carried at runtime; mixed-dim ops return `Option`.
//!
//! Conversion is owned by the [`Unit`] trait: `unit.quantity(value)` builds the
//! quantity (stored in base units) and `unit.from_quantity(q)` reads it back. The `Unit`
//! derive emits those bodies per unit (a multiplier for linear units, a
//! multiplier + offset for affine units, a custom expression for nonlinear units
//! like dB), so a `MassUnit` can only ever produce a `Mass`.
//!
//! See `yadc-si` for a ready-made SI implementation, and `yadc/examples/` for
//! `define_unit_system.rs` (from scratch) and `basic_use.rs` (using
//! `yadc-si`).
//!
//! # Nightly features required in downstream crates
//!
//! Three feature flags surface at different points; which ones your crate
//! needs depends on what it does.
//!
//! | What your crate does | Required at crate root |
//! |---|---|
//! | Just uses already-defined aliases (`Mass`, `Force`, ...) | `#![feature(generic_const_exprs)]` |
//! | Defines its own `Qty<const D: Dim>`, custom `Dim` / `Kind`, or new `pub type Foo = Qty<{ ... }>;` aliases | all three (`generic_const_exprs`, `adt_const_params`, `unsized_const_params`) |
//!
//! Plus `#![allow(incomplete_features)]` to silence the warnings.
//!
//! Why each flag is only needed where it is:
//!
//! - **`generic_const_exprs`** - needed wherever a value's *type* involves a
//!   const-fn call, e.g. `Qty<{ dims_mul(A, B) }>` (the `Output` of
//!   `Mass * Acceleration`). Every consumer trips this when they invoke an
//!   op like `mass * accel`, so every consumer needs the flag.
//! - **`adt_const_params`** - only needed where you *write* a const generic
//!   whose type is a user-defined ADT (`Qty<const D: Dim>(f64)` itself, or a
//!   new alias `pub type Foo = Qty<{ Dim { ... } }>;`). Consumers referencing
//!   already-named aliases never write such const generics, so they don't
//!   need it.
//! - **`unsized_const_params`** - only needed where `ConstParamTy_` is
//!   *named* (the supertrait of [`Dimension`], emitted by the [`Dimension`]
//!   and [`Kind`](macro@Kind) derives). Consumers never see this trait,
//!   only the resolved [`Dimension`] bound.

#![no_std]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![allow(incomplete_features)]

pub mod traits;

/// Re-export so the derive macros can reference `::yadc::num_traits::real::Real`
/// (used in generated nonlinear-unit bodies) without consumers adding the dep.
pub use num_traits;

pub use traits::{Assert, Dimension, Format, IsTrue, Quantity, Unit, UnitDisplay};
// The `Kind` / `UcumKind` *traits* live at `yadc::traits::{Kind, UcumKind}` to
// avoid colliding with downstream `pub enum Kind` types when they also need
// `use yadc::Kind` for the derive macro (which lives in the macro namespace and
// does not conflict).
pub use yadc_derive::{
    Dimension, DynamicQuantity, Kind, StaticQuantity, UcumDeserialize, UcumKind, UcumSerialize,
    Unit,
};
