//! **yadc** — *Yet Another Dimensional Crate.*
//!
//! Core traits and derive macros for building a compile-time-checked unit
//! system. The user defines the dimension layout (a struct of `i8` axes plus
//! an optional `#[kind]` field), the kind enum, and the quantity/unit types;
//! the derives wire up arithmetic, formatting, the trait method bodies, and
//! (optionally) serde.
//!
//! Two quantity flavors share the same surface via the [`StaticQuantity`] and
//! [`DynamicQuantity`] traits:
//! - `Qty<const D: Dim>` — dimension carried in the type, checked at compile
//!   time via the [`CanUseWithUnit`] marker bound.
//! - `DynQty` — dimension carried at runtime; mixed-dim ops return `Option`.
//!
//! `from_unit` / `as_unit` / `fmt_as_unit` are trait methods with default
//! bodies that drive the math through the kind algebra exposed by [`Unit`]
//! (see [`Unit::Multiplier`] vs [`Unit::Offset`] for how affine units like
//! Celsius are handled).
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
//! | Just uses already-defined aliases (`Mass`, `Force`, …) | `#![feature(generic_const_exprs)]` |
//! | Defines its own `Qty<const D: Dim>`, custom `Dim` / `Kind`, or new `pub type Foo = Qty<{ … }>;` aliases | all three (`generic_const_exprs`, `adt_const_params`, `unsized_const_params`) |
//!
//! Plus `#![allow(incomplete_features)]` to silence the warnings.
//!
//! Why each flag is only needed where it is:
//!
//! - **`generic_const_exprs`** — needed wherever a value's *type* involves a
//!   const-fn call, e.g. `Qty<{ dims_mul(A, B) }>` (the `Output` of
//!   `Mass * Acceleration`). Every consumer trips this when they invoke an
//!   op like `mass * accel`, so every consumer needs the flag.
//! - **`adt_const_params`** — only needed where you *write* a const generic
//!   whose type is a user-defined ADT (`Qty<const D: Dim>(f64)` itself, or a
//!   new alias `pub type Foo = Qty<{ Dim { … } }>;`). Consumers referencing
//!   already-named aliases never write such const generics, so they don't
//!   need it.
//! - **`unsized_const_params`** — only needed where `ConstParamTy_` is
//!   *named* (the supertrait of [`Dimension`], emitted by the [`Dimension`]
//!   and [`Kind`](macro@Kind) derives). Consumers never see this trait,
//!   only the resolved [`Dimension`] bound.

#![no_std]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![allow(incomplete_features)]

pub mod traits;

pub use traits::{
    Assert, CanUseWithUnit, Dimension, DynamicQuantity, Format, IsTrue, StaticQuantity, Unit,
    UnitDisplay,
};
// `Kind` (the trait) lives at `yadc::traits::Kind` to avoid colliding with
// downstream `pub enum Kind` types when they also need `use yadc::Kind` for
// the derive macro (which lives in the macro namespace and does not conflict).
pub use yadc_derive::{
    Dimension, DimensionDeserialize, DimensionSerialize, DynamicQuantity, Kind, StaticQuantity,
    Unit,
};
