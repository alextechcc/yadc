//! **yadc** - *Yet Another Dimensional Crate.*
//!
//! A pattern for compile-time-checked unit systems, and a code generator
//! that writes them. yadc has no traits and no types: a system crate owns
//! everything it uses.
//!
//! A system's quantity is a struct holding a number and its unit as a const
//! parameter, `SIQuantity<const U: SIUnit, N, const B: Base>(N)`, `B`
//! saying whether `U` is a base unit or a product of base units. The unit
//! type and `Base` have std operators whose outputs are `Option`s (`None`
//! where two units do not combine), and `#[derive(const_derive_more::ConstOps)]`
//! lifts them onto the quantity: `mass * acceleration` is a `Force`, and
//! `length + time` does not compile. Each unit is a constant, one of it; a number
//! times it is a quantity (`2.0_f64 * KILOMETRE`), and std `From` converts
//! to and from the base units of its dimension.
//!
//! `yadc-codegen` writes such a crate from a `.toml` definition (`yadc-si`)
//! or from the UCUM tables (`yadc-ucum`).
#![no_std]
