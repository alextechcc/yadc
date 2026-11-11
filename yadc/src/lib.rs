//! **yadc** - *Yet Another Dimensional Crate.*
//!
//! Core traits and derive macros for building a compile-time-checked unit
//! system. You define the dimension (a struct of exponent axes, plus an
//! optional `#[kind]`), the quantity family
//! (`Qty<const D: Dim, N = f64>`), the constants, and the units; the derives
//! write the arithmetic, the conversions and the labels. yadc is patterns, a
//! few small traits and the macros that implement them: every type is yours.
//!
//! # Quantities are static families
//!
//! A quantity's dimension is a const parameter, so every operator is typed
//! and checked when compiling: `Mass * Acceleration` is a `Force`, and
//! `Length + Time` does not exist - the compiler says "cannot add: the
//! dimensions differ" (every refusal and its message is listed in
//! [`gates`]). A quantity's dimension is its type's const parameter, so
//! everything optimizes to the bare numbers.
//!
//! ```ignore
//! #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
//! pub struct Qty<const D: Dim, N = f64>(N);
//!
//! pub type Length = Qty<{ Dim { m: 1, ..Dim::DIMENSIONLESS } }>;
//! pub type Length32 = Qty<{ Dim { m: 1, ..Dim::DIMENSIONLESS } }, f32>;
//! ```
//!
//! Values whose dimension is only known at run time - an enum of the
//! quantities an app measures, a type that carries its dimension in a field -
//! are yours too: they convert to and from the statics with std `From` /
//! `TryFrom`. The `dynamickit` crate helps build them; yadc does not depend on it.
//!
//! # Units are values
//!
//! A quantity's number is never read or written directly. A number becomes a
//! quantity, and a quantity a number, only through a unit - a value
//! implementing [`Unit`]. A unit is a constant, and it is called like the
//! function it is - on a number it builds, on a quantity it reads:
//!
//! - **every quantity constant** is a unit of its own kind: `KILOPASCAL(2.0)`
//!   is two kilopascals, `PSI(p)` is how many psi `p` is, and `&KILOPASCAL`
//!   is a `&dyn Unit<Pressure>`;
//! - **a scale** is two of them, a zero and a step, in a tuple struct of
//!   yours with an `impl Unit` of yours (`n` is `zero + n · step`): °C is
//!   `Scale(ICE_POINT, KELVIN)` - start at the ice point, count kelvins -
//!   and `CELSIUS.quantity(25.0)` a temperature. A level is a scale on a
//!   logarithmic line, where a step multiplies: dBm is `Scale(MILLIWATT_LEVEL,
//!   DECIBEL)`, a unit of power levels, and the watts are one std `From` of
//!   yours away;
//! - **anything else** is a type of yours with a hand-written `impl Unit`:
//!   the Beaufort scale is two functions on a zero-sized `struct Beaufort;`.
//!
//! `#[derive(Unit)]` on an enum of yours names a choice of them, for a menu.
//! Every unit has [`Unit`]'s two methods, `quantity` (build) and `value`
//! (read) - which is what generic code and `&dyn Unit<Q>` use; a quantity
//! constant and a derived unit enum's value are also called, std's `Fn` over
//! those two (`KILOMETRE(3.0)`, `MILE(d)`).
//!
//! The base constants are the only raw numbers in a system - one per base
//! unit, written where the field is visible. They are where numbers enter and
//! leave: everything else is defined from them with the quantities' own
//! `const` operators, the way a standards document writes it. A step (a
//! difference of temperatures: `KELVIN`, `DEGREE_FAHRENHEIT`), a zero (a
//! temperature: `ABSOLUTE_ZERO`, `ICE_POINT`, `FAHRENHEIT_ZERO`) and a scale
//! (a unit reading absolute temperatures: `CELSIUS`, `FAHRENHEIT`, each a
//! zero and a step) are different things with different names.
//!
//! ```ignore
//! pub const METRE: Length = Qty(1.0);                    // base constants: the only raw numbers
//! pub const KILOGRAM: Mass = Qty(1.0);
//! pub const SECOND: Time = Qty(1.0);
//! pub const KELVIN: TemperatureStep = Qty(1.0);          // a step: 1 K, the same as 1 °C of difference
//! pub const ABSOLUTE_ZERO: Temperature = Qty(0.0);
//!
//! pub const INCH: Length = 0.0254 * METRE;               // everything else, from them
//! pub const POUND: Mass = 0.45359237 * KILOGRAM;
//! pub const STANDARD_GRAVITY: Acceleration = 9.80665 * METRE / (SECOND * SECOND);
//! pub const PSI: Pressure = POUND * STANDARD_GRAVITY / (INCH * INCH);
//! pub const DEGREE_FAHRENHEIT: TemperatureStep = KELVIN * (5.0 / 9.0);
//! pub const ICE_POINT: Temperature = ABSOLUTE_ZERO + 273.15 * KELVIN;
//! pub const FAHRENHEIT_ZERO: Temperature = ICE_POINT - 32.0 * DEGREE_FAHRENHEIT;
//!
//! pub struct Scale<P, S>(pub P, pub S);                  // a zero and a step, with an `impl Unit` of yours
//! pub const CELSIUS: Scale<Temperature, TemperatureStep> = Scale(ICE_POINT, KELVIN);
//!
//! let p = PSI(14.7);                                     // build
//! let kpa = KILOPASCAL(p);                               // read
//! let t = CELSIUS.quantity(25.0);                       // 298.15 K: the zero point is the unit's business
//! let c = CELSIUS.value(t);                              // 25.0
//! ```
//!
//! A family of named units picked at run time - for a menu, a config file, a
//! parser - is an enum, each variant naming the unit value it is.
//! `#[derive(Unit)]` writes [`Unit`], and `Display` / `FromStr` from each
//! unit's symbol and name:
//!
//! ```ignore
//! #[derive(Copy, Clone, yadc::Unit)]
//! #[unit(of = Temperature)]
//! pub enum TemperatureUnit {
//!     #[unit("K", "kelvin", by = Scale(ABSOLUTE_ZERO, KELVIN))] Kelvin,
//!     #[unit("°C", "degrees Celsius", by = CELSIUS)] Celsius,
//!     #[unit("°F", "degrees Fahrenheit", by = Scale(FAHRENHEIT_ZERO, DEGREE_FAHRENHEIT))] Fahrenheit,
//! }
//! let u: TemperatureUnit = "°F".parse()?;
//! println!("{:.1} {u:#}", u.value(t));                   // "77.0 degrees Fahrenheit"
//! ```
//!
//! Quantities print themselves too: `{}` is the number and the dimension
//! (`101325 kg·m⁻¹·s⁻²`), written by the derives (see [`display`]).
//!
//! # Kinds: amounts, points and logarithms
//!
//! A dimension's kind says how a quantity behaves. The ordinary kind is an
//! amount: its zero is a fact (no mass, no length), so it adds, scales and
//! multiplies. A dimension may name two more:
//!
//! ```ignore
//! #[derive_const(PartialEq, Default)]
//! #[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
//! pub enum Kind { #[default] Linear, #[kind(affine)] Affine, #[kind(log)] Log }
//!
//! pub struct Dim { pub m: i8, .., #[kind] pub kind: Kind }
//!
//! pub type Altitude = Qty<{ Dim { m: 1, kind: Kind::Affine, ..Dim::DIMENSIONLESS } }>;
//! pub const SEA_LEVEL: Altitude = Qty(0.0);
//! pub const FEET_AMSL: Scale<Altitude, Length> = Scale(SEA_LEVEL, FOOT);
//! ```
//!
//! **Affine: a point.** Its zero is a convention - an altitude above sea
//! level, an instant since an epoch, a temperature on a scale. Two points of
//! one dimension differ by an amount (`altitude - altitude` is a `Length`),
//! an amount moves a point (`altitude ± length` is an `Altitude`), and
//! nothing else compiles: `altitude + altitude` ("cannot add a point to a
//! point"), `altitude * 2.0` ("a point cannot be scaled"), `altitude *
//! length` ("these have no product") - each answer would depend on where the
//! zero was put. Which zero is not in the type: a point is read on a scale,
//! a zero and a step, and every scale of one dimension reads the same line -
//! `Scale(SEA_LEVEL, FOOT)`, `Scale(runway, FOOT)` made at run time, °C and °F.
//!
//! **Log: a logarithm.** A level or a gain, in decibels. Adding logarithms
//! multiplies what they are the logarithms of, so `+` and `-` add and
//! subtract the exponents: a power level less a power level is a gain (the
//! logarithm of watts over watts), a gain moves a level, and only a logarithm
//! adds to a logarithm. The logarithm itself is one pair of std `From`s of
//! yours between the level and the linear quantity.
//!
//! Any other variant of the enum is a label of your own, with `add` / `sub`
//! rules, telling apart quantities of the same exponents. A point, a level
//! and a label that cannot be added to itself are not a [`Quantity`].
//!
//! # The whiteboard principle
//!
//! Every trait is a structure you could write on a whiteboard, and its bounds
//! are exactly the operations that structure needs. A trait or bound that
//! can't be named as a piece of mathematics doesn't go in; one the mathematics
//! needs is asked for, and nothing more.
//!
//! | Trait | On the whiteboard | It asks for |
//! |---|---|---|
//! | [`Dimension`] | exponents that add under `*`, plus optionally a kind - an amount, a point, a logarithm, or a label of yours: a product and a quotient (total for exponents alone, partial with a kind - a point, a level, or a label with no sum beside a plain number has none; not associative with labels), a sum that may not exist and keeps its left operand (an action, not a commutative sum) | std `*` `/` (`Output: Into<Option<Self>>`), `+` `-` returning `Option`, a power and a root ([`PartialPowi`], [`PartialRoot`]), `DIMENSIONLESS` (dimension one); `plain` (the label forgotten) is provided |
//! | [`Quantity`] | a value you can add, scale and compare, with itself as a unit | `Copy`, `PartialOrd`, `+`, scaling by its [`Number`](Quantity::Number)s, [`Unit<Self>`](Unit) |
//! | [`Unit<Q, N>`] | two inverse functions between numbers and `Q` (a coordinate chart) | `quantity`, `value` |
//!
//! [`Cast<T>`] is a change of number type (`KILOPASCAL.cast::<f32>()`). A
//! kind is given by a constant that has it: a plain number of bels times the
//! bel is a gain, `n * BEL`.
//!
//! # The traits
//!
//! | module | what | |
//! |---|---|---|
//! | (root) | `Dimension`, `Label`, `Axis`, `PartialPowi`, `PartialRoot`, `Exponent` / `ParseExponentError`, `Powi` / `PowiAs`, `Root` / `RootAs`, `Cast` / `CastAs` | the generic maths, from the `yadc-math` crate, re-exported |
//! | [`quantity`] | `Quantity` | a value you can add, scale and measure with |
//! | [`unit`](mod@unit) | `Unit`, `UnknownUnit` | turning numbers into quantities and points and back; parsing |
//! | [`display`] | `write_dimension`, `write_quantity` | how the derived `Display` impls print |
//! | [`gates`] | the refusals | why an operator a family does not have does not compile; the derives' helpers |
//! | `ucum` | `UcumKind` | the serde add-on, behind the `ucum` Cargo feature |
//!
//! The traits of `quantity` and `unit` are re-exported at the crate root;
//! `UcumKind` is not (so a downstream `enum Kind` never collides with it).
//!
//! Every method-syntax helper follows one scheme: the maths is a trait `X<P>`
//! with a method `x_into` ([`Powi::powi_into`], [`Root::root_into`],
//! [`Cast::cast_into`]), and a blanket trait `XAs` gives every type the
//! method `x::<P>()` that names `P`: `x.powi::<2>()`, `x.root::<2>()`,
//! `x.cast::<f32>()`.
//!
//! | Trait | Role |
//! |---|---|
//! | std `Mul` / `Div` with `Output: Into<Option<Self>>` | a dimension's product and quotient: `Self` (total) for exponents alone, `Option<Self>` (`None` where a point or a level has no product) for a dimension with a kind |
//! | std `Add` / `Sub` with `Output = Option<Self>` | a partial operation: `None` where there is no result; const, so it runs inside type checks, and chains with `?` |
//! | [`PartialPowi`] / [`PartialRoot`] | an integer power / n-th root that may not exist: repeated `*` (none for a point or a level); exact division for an integer exponent, a label having only its first root; for `f32` / `f64` `powi` / `powf` |
//! | [`Dimension`] | the above operators, `PartialPowi` and `PartialRoot`, plus `DIMENSIONLESS` (dimension one: the empty product, what `x / x` gives); `plain` provided; implemented by `#[derive(Dimension)]` on a struct (a dimension) or on an enum (a label). A reciprocal is `DIMENSIONLESS / d` |
//! | [`Powi<N>`] / [`Root<N>`] | a power or root with the exponent `N` known at compile time, typed: `Length` squared is an `Area`; called as `x.powi::<N>()` / `x.root::<N>()` with [`PowiAs`] / [`RootAs`] in scope |
//! | [`Quantity`] | an amount: adds, scales by its `Number`s, compares, and is a unit of itself; by `#[derive(Quantity)]` for every member that can be scaled |
//! | [`Unit<Q, N>`] | a unit of any `Q` counted in `N` (`f64` by default): `quantity(n)` builds, `value(q)` reads; dyn compatible. Quantity constants, and types of yours - `#[derive(Unit)]` or hand-written |
//! | [`Cast<T>`] / [`CastAs`] | a change of number type, `x.cast::<f32>()`: primitives by `as`, a family generic over its number type by the derive (which also writes std `From` where the change is lossless: `Pressure::from(p32)`) |
//!
//! Printing is std `Display`: a unit's (`{}` its symbol, `{:#}` its name) -
//! `format!("{:.1} {}", unit.value(q), unit)` - and a quantity's own (its
//! number and dimension). Everything else is a standard trait too: `From`,
//! `Mul`, `Div`, `Add`, `Sub`, `num_traits::Inv`. A static quantity's operators are typed and cannot
//! fail: an operation the dimension algebra rejects has no impl, and the
//! compiler says why.
//!
//! # Nothing is hidden
//!
//! The traits are the whole contract. Nothing generated relies on an
//! inherent method or a field of some *other* type existing: the quantity
//! derive reaches dimensions only through [`Dimension`] (the std operators,
//! [`PartialPowi`], [`PartialRoot`], `DIMENSIONLESS` and `plain`), and a derived unit enum
//! delegates to the unit values you name in its attributes. Only the crate
//! that owns a quantity type can write its field, so its base constants are
//! the only raw construction; everyone else builds and reads through units.
//! Anything a derive can produce, you can write by hand, and the two
//! interoperate. Every generated `::yadc` / `::serde` path can be redirected
//! with `#[yadc(crate = "..", serde = "..")]`.
//!
//! Each derive writes its own trait (and, for `Quantity`, the operators that
//! are its essence; the rest only as listed - see the derive). They emit
//! trait impls and nothing else: no inherent method or constant is added to
//! your types.
//!
//! | Instead of | Write | Trait |
//! |---|---|---|
//! | a typed power or root | `q.powi::<2>()`, `q.root::<2>()` | [`PowiAs`] / [`RootAs`] (in scope), over [`Powi<N>`] / [`Root<N>`] |
//! | a dimension's power or root with a run-time exponent | `d.partial_powi(n)`, `d.partial_root(n)` | [`PartialPowi`], [`PartialRoot`] |
//! | a dimension's reciprocal | `Dim::DIMENSIONLESS / d` | std `Div` |
//! | a quantity's number in a unit | `KILOPASCAL.value(q)` | [`Unit`] |
//! | another number type | `q.cast::<f32>()` | [`CastAs`] over [`Cast<T>`] |
//!
//! # Nightly features required in downstream crates
//!
//! This crate is deliberately uncompromising about using what nightly offers,
//! so defining a unit system needs a handful of feature flags. Using one needs
//! a single flag.
//!
//! | What your crate does | Required at crate root |
//! |---|---|
//! | Just uses already-defined aliases (`Mass`, `Force`, ...) | `#![feature(generic_const_exprs)]` |
//! | ... and does quantity arithmetic or unit conversions in its own `const` items (`const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`) | additionally `const_trait_impl`, `const_ops` |
//! | Defines its own `Qty<const D: Dim>`, custom `Dim` / `Kind`, or new `pub type Foo = Qty<{ ... }>;` aliases | `generic_const_exprs`, `min_adt_const_params`, `const_trait_impl`, `const_ops`, `const_cmp`, `const_default`, `const_convert`, `const_try`, `derive_const`, and for the default `Fn` extra (a constant is called: `KILOMETRE(3.0)`) `fn_traits`, `unboxed_closures`, `const_destruct` |
//! | Defines unit enums (`#[derive(Unit)]`) | `const_trait_impl`, and for the default `Fn` extra `fn_traits`, `unboxed_closures` |
//!
//! Plus `#![allow(incomplete_features)]` to silence the `generic_const_exprs`
//! warning.
//!
//! - **`generic_const_exprs`** - needed wherever a value's *type* involves a
//!   const expression, e.g. `Qty<{ A * B }>` (the `Output` of
//!   `Mass * Acceleration`). Every consumer trips this when they invoke an
//!   op like `mass * accel`, so every consumer needs the flag.
//! - **`min_adt_const_params`** - to write a const generic whose type is a
//!   user-defined ADT (`Qty<const D: Dim>(f64)` itself, or a new alias
//!   `pub type Foo = Qty<{ Dim { ... } }>;`), and for the
//!   `#[derive(ConstParamTy)]` your `Dim` and `Kind` need.
//! - **`const_trait_impl`** - the derives emit `const impl`s of the std
//!   operators, of `PartialRoot`, of `Quantity` and `Unit`,
//!   so the algebra can run inside const-generic bounds and units in `const`
//!   items.
//! - **`const_ops`**, **`const_cmp`**, **`const_default`**,
//!   **`const_convert`** - the derives emit the quantity operators and the
//!   dimensionless quantity's `From` as `const impl`s, compare / default
//!   exponents and kinds in const context, and read a dimension's product
//!   through a const
//!   `Into<Option<_>>`. That is what makes `const P: Pressure =
//!   PSI.quantity(14.7);` work.
//! - **`const_try`** - the `Dimension` derive's `+`, `-` and root use `?` on
//!   the label's `Option` in const code, so a crate defining a `Dim` needs it.
//! - **`const_destruct`** - for the quantity derive's `Fn` extra (on by
//!   default), and for a quantity whose number type is a generic parameter
//!   (`Qty<const D: Dim, N = f64>`): its const operators drop an `N`.
//! - **`derive_const`** - `#[derive_const(PartialEq)]` on your `Dim` and
//!   `Kind` (and `Default` on a `Kind`), so `==` on them is const-callable.

#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(min_adt_const_params)]
#![feature(const_ops)]

pub mod display;
pub mod gates;
pub mod quantity;
#[cfg(feature = "ucum")]
pub mod ucum;
pub mod unit;

/// Re-export used by the derives: `real::Real` supplies `powf` for roots,
/// `ConstOne` / `One` / `Zero` powers,
/// reciprocals (`Inv`) and sums, so `no_std` downstream crates need no extra
/// dependency. Enable `yadc/libm` for a genuinely std-free build.
pub use num_traits;

pub use quantity::Quantity;
pub use unit::{Unit, UnknownUnit};
pub use yadc_math::{
    Axis, Cast, CastAs, Dimension, Exponent, Label, ParseExponentError, PartialPowi, PartialRoot, Powi,
    PowiAs, Root, RootAs,
};
// The `UcumKind` *trait* lives at `yadc::ucum::UcumKind`, so a downstream
// `pub enum Kind` never collides with anything re-exported here.
pub use yadc_derive::{Dimension, Quantity, Unit};
#[cfg(feature = "ucum")]
pub use yadc_derive::{UcumDeserialize, UcumKind, UcumSerialize};
