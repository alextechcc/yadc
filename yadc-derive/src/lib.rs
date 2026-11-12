//! The derive macros of yadc. Each derive writes its own trait: `Dimension`
//! writes `yadc::Dimension` (on a struct, a dimension; on an enum, a label),
//! `Quantity` writes `yadc::Quantity` (with its supertraits, the cross-type
//! operators and the extras it lists), `Unit` writes `yadc::Unit` (with
//! `Display` and `FromStr` from the unit's labels unless told otherwise), and
//! the UCUM derives the serde add-on.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod dimension;
mod kind;
mod paths;
mod quantity;
mod ucum;
mod unit;
mod util;

use dimension::expand_dimension;
use kind::expand_ucum_kind;
use quantity::expand_quantity;
use ucum::{expand_ucum_deserialize, expand_ucum_serialize};
use unit::expand_unit;

/// Derive `yadc::Dimension`: on a **struct**, a dimension - exponent axes,
/// plus an optional kind; on an **enum**, a kind (a label). The derive picks
/// from the item's shape.
///
/// # On a struct: a dimension
///
/// ```ignore
/// use core::marker::ConstParamTy;
///
/// #[derive_const(PartialEq)]
/// #[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
/// pub struct Dim {
///     pub m: i8, pub kg: i8, pub s: i8, pub k: i8,
///     #[kind]                           // optional: the kind
///     pub kind: Kind,
/// }
/// ```
///
/// Every field not marked is an exponent axis. Nothing about an axis type is
/// assumed beyond what the operators use: const `PartialEq` and `Default`
/// (the zero exponent - no literal `0` is written), `yadc::Axis` (a sum and
/// difference that are `None` where they do not fit: a product with an
/// exponent out of range has no result, or panics without a `#[kind]`
/// field) and `yadc::PartialRoot` - nothing is matched by name. Every signed
/// integer qualifies (an exponent may be negative); a rational type of yours
/// with those gives fractional powers (`m^½`, `V/√Hz`), and `yadc::Exponent`
/// is one such type, ready-made. There may be any number of axes, of
/// different types. Each axis may also carry `#[ucum("kg")]` for the serde
/// derives (see [`UcumSerialize`](macro@UcumSerialize)); it is ignored here.
///
/// ## The kind: how a quantity behaves
///
/// The `#[kind]` field holds an enum of yours (with this derive: see below).
/// Its `#[default]` variant is the ordinary kind - an amount, which adds to
/// its like, scales and multiplies. `#[kind(affine)]` and `#[kind(log)]` on
/// the enum's variants mark up to two more, each a different piece of
/// mathematics (the derive on the enum writes `yadc::Label`, which says
/// which; this derive asks it):
///
/// - **affine: a point.** Measured from a zero somebody chose - an altitude,
///   an instant, a temperature. Two points of one dimension differ by an
///   amount (`altitude - altitude` is a length), an amount moves a point
///   (`altitude ± length` is an altitude), and nothing else: no sum of two
///   points, no scaling, no products, powers or roots - each would depend on
///   where the zero was put. Which zero is not in the type: every scale of
///   one dimension - °C and °F, feet above sea level and above this runway -
///   reads the same line.
/// - **log: a logarithm.** A level or a gain, counted in decibels. Adding
///   logarithms multiplies what they are the logarithms of, so `+` and `-`
///   add and subtract the exponents: a power level less a power level is the
///   logarithm of watts over watts, a gain; a gain moves a level; two power
///   levels sum to the logarithm of watts squared, which is not a power
///   level. Only a logarithm adds to a logarithm. A gain - no exponents -
///   scales (`3.0 * DECIBEL`) and leaves any other product as a label does
///   (decibels per kilometre is a plain per-length); a level has no
///   products at all, not even with a plain number: twice a level would be
///   the level of a square.
///
/// Beside a plain number every product is a scaling: it keeps the other
/// operand only where that adds to itself as itself, so a point, a level
/// and a label with no sum have none.
///
/// Any other variant is a label of your own (see below): it tells apart
/// quantities of the same exponents, and decides `+` and `-` only.
///
/// ## What is written
///
/// `yadc::Dimension` and the operators its supertraits ask for, as `const
/// impl`s:
///
/// - `*` and `/`: exponents add / subtract. Beside a plain number (every
///   exponent zero and the ordinary kind) a product is a scaling: `2.0 * x`
///   keeps `x`'s label where `x + x` is `x`, and has no result otherwise (a
///   point, a level, a label with no sum); any other product is ordinary. Without a
///   `#[kind]` field they are total, `Output = Self`; with one they are
///   partial, `Output = Option<Self>`, `None` where the rules above give no
///   product (the derive cannot see which variants are points or levels, so
///   the type is the same for every kind). An exponent that does not fit its type (`m^100 *
///   m^100` in `i8`) panics - at compile time, where the product is a type
///   (at run time with an integer axis, std's overflow rules apply).
/// - `+` and `-`, `Output = Option<Self>`: equal exponents, then the kind
///   decides (a logarithm's sum adds exponents instead); a rejected sum is
///   `None`.
/// - `yadc::PartialRoot`: every exponent rooted exactly, the label rooted;
///   none for a point or a logarithm.
/// - `DIMENSIONLESS`: every exponent zero, the ordinary kind - so aliases
///   read `{ Dim { m: 1, ..Dim::DIMENSIONLESS } }`.
/// - `plain`, only with a `#[kind]` field: the exponents kept, the kind
///   forgotten (the trait's provided `plain` - the identity - is right
///   without one).
/// - `yadc::PartialPowi`: repeated `*`, so `None` where a step has no
///   product.
/// - Scaling, `Mul<n>` / `Div<n> -> Option<Self>` for every primitive number
///   type `n`: the same dimension where it adds to itself as itself, `None`
///   otherwise (a point, a level, a label with no sum). It is what a dynamic
///   quantity's `* f64` asks of its dimension.
/// - `Display`, unless `#[dimension(impl())]` leaves it out: the product of
///   the axes with superscript exponents, positive first (`kg·m⁻¹·s⁻²`),
///   each axis named by its field or by `#[dimension(symbol = "kg")]` on the
///   field; then a kind that is not the ordinary one, by its `Display`, in
///   braces: `K{Affine}`. Dimension one prints as nothing. The axis types
///   need `Display` (an exponent is negative when it prints with a leading
///   `-`).
///
/// A reciprocal is `Dim::DIMENSIONLESS / d`. The `+`, `-` and root use `?`
/// in const code, so the defining crate needs `#![feature(const_try)]`.
///
/// # On an enum: a kind
///
/// ```ignore
/// #[derive_const(PartialEq, Default)]
/// #[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
/// pub enum Kind {
///     #[default] Linear,                // an ordinary amount
///     #[kind(affine)] Affine,           // a point
///     #[kind(log)] Log,                 // a logarithm: a level or a gain
///     #[kind(add(Rank) = invalid)]      // a label of your own, with a rule
///     Rank,
/// }
/// ```
///
/// - **The ordinary kind** is the variant marked with std `#[default]`: the
///   enum's `DIMENSIONLESS`, and every label's `plain`.
/// - **The affine and the logarithmic kind** are the variants marked
///   `#[kind(affine)]` and `#[kind(log)]` (at most one each, neither the
///   `#[default]` one, and no `add`/`sub` rules on or naming them: their sums
///   are fixed). The derive writes `yadc::Label`, which says which they are;
///   the dimension's derive asks it.
/// - **A label of your own** tells apart quantities with the same exponents
///   that must not be added together (a torque and an energy). It matters to
///   `+` and `-` only: by default it works within itself (`x + x = x`,
///   `x - x = x`), and survives `*` and `/` only beside a plain number.
/// - **`#[kind(absolute)]`** marks a label of an amount with a true zero (a
///   temperature from absolute zero), short for its three rules: two differ
///   by a plain amount (`sub(X) = Plain`), and a plain amount moves one
///   (`add(Plain) = X`, `sub(Plain) = X`), where `Plain` is the `#[default]`
///   variant. More rules may follow it.
///
/// Nothing depends on a variant's name.
///
/// ## Rules
///
/// `#[kind(add(Other) = Result, sub(Other) = Result)]` on a variant,
/// repeatable, for a label of your own. `Other` and `Result` are variant
/// names; a `Result` may also be `invalid` (no result), which **revokes** a
/// default entry. A sum keeps its left operand's label, so an `add` rule's
/// result must be the variant it is written on (or `invalid`).
///
/// ```ignore
/// pub enum MoneyKind {
///     #[default] Scalar,
///     // `Money + Money`, `Money * Scalar` and `Money / Money` (a plain
///     // ratio) come free.
///     Money,
///     // A balance moves by an amount of money: balance ± money is a balance,
///     // and the difference of two balances is money.
///     #[kind(add(Money) = Balance, sub(Money) = Balance, sub(Balance) = Money)]
///     Balance,
/// }
/// ```
///
/// ## UCUM prefix
///
/// `#[ucum(prefix = "..")]` on a variant gives it a UCUM unit prefix, and
/// `#[ucum(prefix = none)]` says it has none (the default). Read only by
/// [`UcumKind`](macro@UcumKind).
///
/// The enum's own `PartialRoot` is the label itself for the first root, the
/// plain label for any root of the plain label, and `None` otherwise (a
/// label's powers are all plain); its `PartialPowi` is repeated `*`; its `Display` (unless
/// `#[dimension(impl())]`) is the variant's name. Every variant must be a
/// unit variant.
///
/// # What you derive yourself
///
/// `Copy`, `Clone`, `Debug`, `PartialEq` (as a const impl:
/// `#[derive_const(PartialEq)]`), `Eq` and `ConstParamTy` - a dimension is
/// the const generic parameter of the quantity type, which the compiler
/// checks structurally - and, for a kind, `Default` (const).
///
/// # Crate-root features
///
/// A crate that *defines* a unit system - a kind, a dimension and a
/// quantity type - needs these ten (`const_destruct` for a quantity type
/// generic over its number type):
///
/// ```ignore
/// #![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
/// #![feature(const_cmp, const_default, derive_const, const_ops, const_try)]
/// #![feature(const_convert, const_destruct)]
/// #![allow(incomplete_features)]
/// ```
///
/// A crate that only *uses* existing aliases needs `generic_const_exprs`
/// alone. `#[yadc(crate = "path")]` redirects the emitted `::yadc::` paths.
#[proc_macro_derive(Dimension, attributes(yadc, dimension, kind, ucum, default))]
pub fn derive_dimension(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_dimension(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derive `yadc::ucum::UcumKind` for a label enum - the UCUM serialization
/// mapping (`name`, `from_name`, `ucum_prefix`, `from_ucum_prefix`).
///
/// Kept separate from the label's `Dimension` so the algebra carries no
/// wire-format knowledge. Only [`UcumSerialize`](macro@UcumSerialize) /
/// [`UcumDeserialize`](macro@UcumDeserialize) call it, so derive it under
/// the same serde gate, e.g.
/// `#[cfg_attr(feature = "serde", derive(yadc::UcumKind))]`.
///
/// `name` is the variant identifier. The prefix comes from
/// `#[ucum(prefix = "..")]` on the variant; `#[ucum(prefix = none)]` (the
/// default) says it has none. Both `ucum_prefix` and `from_ucum_prefix` are
/// emitted from that one list, so the two directions cannot drift, and two
/// variants claiming the same prefix is an error.
/// `#[yadc(crate = "path")]` redirects the emitted `::yadc::` paths.
#[proc_macro_derive(UcumKind, attributes(yadc, kind, ucum, default))]
pub fn derive_ucum_kind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_ucum_kind(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derive `yadc::Unit<Q, N>` for a type of yours that names unit values -
/// an enum of units, or a struct that is one - plus std `Display` and
/// `FromStr` from each unit's symbol and name. Trait impls only - no inherent
/// items.
///
/// A unit itself is a constant: a quantity constant (`MILE`), or a
/// scale, a zero and a step (`pub const CELSIUS: Scale<Temperature,
/// TemperatureStep> = Scale(ICE_POINT, KELVIN);`). The derive gives a choice
/// of them a type, labels and a parser:
///
/// ```ignore
/// #[derive(Copy, Clone, yadc::Unit)]                    // a choice of units, for a menu
/// #[unit(of = Temperature)]
/// pub enum TemperatureUnit {
///     #[unit("K", "kelvin", by = Scale(ABSOLUTE_ZERO, KELVIN))] Kelvin,
///     #[unit("°C", "degrees Celsius", by = CELSIUS)] Celsius,
///     #[unit("°F", "degrees Fahrenheit", by = FAHRENHEIT)] Fahrenheit,
/// }
///
/// #[derive(Copy, Clone, yadc::Unit)]                    // one unit that prints
/// #[unit("ft", "feet", of = Length, by = FOOT)]
/// pub struct Foot;
///
/// #[derive(Copy, Clone, yadc::Unit)]                    // a unit value held in a field
/// #[unit("ft AGL", "feet above ground level", of = Altitude)]
/// pub struct FeetAboveGround { #[unit(by)] pub scale: Scale<Altitude, Length> }
///
/// #[derive(Copy, Clone, yadc::Unit)]                    // a level reading the watts
/// #[unit(of = Power, non_const)]
/// pub enum PowerUnit {
///     #[unit("W", "watts", by = WATT)] Watt,
///     #[unit("dBm", "decibel-milliwatts", by = DBM, via = PowerLevel)] DecibelMilliwatt,
/// }
///
/// let (celsius, kelvin) = (TemperatureUnit::Celsius, TemperatureUnit::Kelvin);
/// let t = celsius.quantity(25.0);                     // build
/// let k = kelvin.value(t);                            // read: 298.15
/// println!("{k:.2} {kelvin:#}");                      // "298.15 kelvin"
/// ```
///
/// # Naming a unit
///
/// On a struct, in its own `#[unit(..)]`; on an enum, on each variant:
/// `by = U`, where `U` is any unit value - a quantity constant, a scale, a
/// unit type of yours with a hand-written `impl Unit` - and the unit does
/// whatever `U` does. `U` is a constant expression, evaluated once, at
/// compile time (`by = KILO * METRE`, `by = Length::<f64>::from(INCH)` are
/// one number in the program). Instead, `U` may be a field of the unit marked
/// `#[unit(by)]`: a unit known only at run time.
///
/// `via = P` beside it says `U` measures another type `P`, which the unit's
/// own type converts to and from with std `From`: a level's scale (a unit of
/// `PowerLevel`) as a unit of `Power`.
///
/// # Labels and extras
///
/// The symbol and the name come first: `#[unit("°C", "degrees Celsius",
/// ..)]`. From them the derive writes std `Display` (`{}` the symbol, `{:#}`
/// the name, through `Formatter::pad`, so width and alignment apply) and std
/// `FromStr` (the symbol back to the unit, `Err(yadc::UnknownUnit)`
/// otherwise; two units sharing a symbol: the first wins; only when no unit
/// has fields). `#[unit(impl(..))]` on the type writes exactly the listed
/// ones - `impl(Display)`, `impl()` for none (then no labels are needed).
///
/// # On the type
///
/// - `of = Q` (repeatable): one `impl Unit<Q, N>` each, matching on the
///   variant. That impl *is* the declaration that the unit measures `Q`;
///   with concrete types it has no `where` clause, so each `by` is checked
///   here, for each `Q`.
/// - `number = f32`: the numbers' type `N`, `f64` by default.
/// - `for<N> where ..`: the impls are generic, with these bounds - the last
///   item of its `#[unit(..)]`, as a where clause takes the rest. `of =
///   Length<N>, number = N` with `for<N> where Length<N>: [const]
///   Unit<Length<N>, N> + [const] Destruct, ..` counts in any number type
///   the bounds allow, each `by` still evaluated at compile time, once per
///   number type.
/// - `non_const`: an ordinary impl instead of a `const impl`, when a `by = U`
///   names a value whose own `Unit` impl is not `const` (the derive cannot
///   see that) - a level read as a linear quantity, a hand-written formula.
///   An impl is const or not as a whole.
/// - `#[yadc(crate = "path")]` redirects the emitted `::yadc::` paths.
#[proc_macro_derive(Unit, attributes(yadc, unit))]
pub fn derive_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_unit(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derive a static quantity family: `Qty<const D: Dim>(f64)`, or
/// `Qty<const D: Dim, N>(N)` generic over its number type.
///
/// It writes `yadc::Quantity`, its supertraits, and the operators that are
/// its essence; the extras below are written too unless
/// `#[quantity(impl(..))]` lists which ones to write. Add `Copy`, `Clone`,
/// `PartialEq`, `PartialOrd`, `Debug`, ... to `#[derive(...)]` yourself.
/// Everything is a trait impl, `const` except `Display`, `Sum`, `Inv` and
/// `Root`.
///
/// # Always
///
/// - `yadc::Quantity` (`type Number = N`) and its supertrait
///   `Unit<Self, N>` (`BAR.quantity(2.0)` is `BAR * 2.0`,
///   `BAR.value(q)` is `q / BAR`, so every amount constant is a
///   unit), for every member that can be scaled.
/// - The cross-type operators, between any two members, wherever the
///   dimension's own operator has a result: `+` (the result keeps the left
///   operand's type: `absolute + step` is absolute, `altitude + length` an
///   altitude), `-` (`Qty<{ A - B }>`: `altitude - altitude` is a length),
///   `*` and `/` (`Qty<{ A * B }>`; also with a borrowed right operand). The
///   product rule is the dimension's own `*`: a point has none.
/// - Scaling: `Mul<N>` / `Div<N>`, and `N * Qty` (for a generic `N`, once
///   per primitive number type, by the orphan rule).
/// - The refusals of `yadc::gates`: one marker impl each, so that a refused
///   operator fails naming why ("cannot add: the dimensions differ", "a
///   point cannot be scaled", ...), in the defining crate and in every crate
///   using it - see `yadc::gates` for what the compiler shows per operator.
///
/// # Extras: `#[quantity(impl(..))]`
///
/// Without the attribute, all of these; with it, exactly those listed:
///
/// - `Display`: the number with the caller's flags (`{:.2}`, width, sign),
///   then a space and the dimension's own `Display` - `101325.00
///   kg·m⁻¹·s⁻²`, `273.15 K{Affine}`, and nothing after a plain number.
///   Needs `N: Display` and the dimension's `Display` (which
///   `#[derive(Dimension)]` writes).
/// - `Neg`: `-x` (scaling by -1).
/// - `AddAssign`, `SubAssign`: in place, owned and borrowed right operand;
///   the type cannot change (`altitude -= length` works, `altitude -=
///   altitude` does not).
/// - `MulAssign`, `DivAssign`: by a number, and by a member whose product /
///   quotient is the left operand's type (a plain number); owned and
///   borrowed.
/// - `Powi`: `yadc::Powi<N>` (const), `x.powi::<2>()` with `yadc::PowiAs` in
///   scope - needs `N: num_traits::ConstOne` plus `*` / `/`.
/// - `Root`: `yadc::Root<N>`, `x.root::<2>()` with `yadc::RootAs` - where
///   the root's dimension exists; needs `N: num_traits::real::Real`. (A
///   number type without these bounds simply lacks the impl.)
/// - `Inv`: `num_traits::Inv`, `x.inv()` is `1 / x`.
/// - `Sum`: `Sum<Qty<D>>` / `Sum<&Qty<D>>`, seeded with `num_traits::Zero`.
/// - `Cast` (a generic number type only): `yadc::Cast<W>` -
///   `BAR.cast::<f32>()` is the same pressure counted in `f32`,
///   `const` - for every `N: Cast<W>`.
/// - `From`: `From<Qty<{ DIMENSIONLESS }>> for N` - a dimensionless quantity
///   is a number, `f64::from(a / b)` - and, for a generic number type, std
///   `From<Qty<D, N>> for Qty<D, W>` wherever std converts the primitives
///   losslessly (`f32` to `f64`, `u8` to `i32`, ..), `const`:
///   `Pressure::<f64>::from(p32)`.
///
/// ```ignore
/// #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
/// #[quantity(impl(Neg, AddAssign, SubAssign, Powi))]
/// pub struct Qty<const D: Dim>(f64);
/// ```
///
/// # Number types that meet: `#[quantity(from(M => N, ..))]`
///
/// For a family with a number type parameter, lists number types `M` whose
/// members meet members counted in `N`; the result is counted in `N`, the
/// `M` side converted with std `From<M> for N` (`const` where that is). For
/// each pair it writes:
///
/// - `* / + -` between an `M` and an `N` member, either way round, with the
///   same refusals as the same-type operators (a sum keeps its left
///   operand's dimension); `+=` and `-=` into the `N` side (with the
///   `AddAssign` / `SubAssign` extras);
/// - scaling an `M` member by an `N` number, from either side;
/// - every `M` member as a `Unit` of `N` members (`MILE_32.value(d)`);
/// - `==` and `<` either way round.
///
/// It runs one way: listing a pair both ways is an error, since `f32 + f64`
/// would then have two results. So a system's constants can be counted in
/// an exact number type of its own and meet values in whatever type stores
/// them: `#[quantity(from(Exact => f32, f64))]`.
///
/// ```ignore
/// #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
/// #[quantity(from(f32 => f64), from(i32 => f64))]
/// pub struct Qty<const D: Dim, N>(N);
///
/// const LIMIT: Velocity<f64> = 50.0 * MILE_32 / HOUR_I32;   // f32 and i32 constants, f64 result
/// ```
///
/// # Shape
///
/// Nothing positional is assumed. The **dimension** is the const generic
/// marked `#[dim]`, or the struct's only const parameter; the **number** is
/// the field marked `#[value]`, or the struct's only field that is not
/// `PhantomData` - named or positional, so `struct Qty<const D: Dim>(f64)`
/// and `struct Qty<const D: Dim> { #[value] v: f64 }` both work. Other
/// `PhantomData` fields are filled in wherever the derive builds the struct.
///
/// Every other generic parameter and the whole `where` clause are passed
/// through to each emitted impl, and the bounds an operator needs on the
/// number type are added to that impl's `where` clause. The number type
/// parameter itself takes no bounds: the impls for `n * Qty` and the
/// lossless `From`s are written once per primitive number type, which a
/// bound such as `N: Float` would rule out - each impl asks for what it
/// uses instead. `#[quantity(impl(Cast))]` needs that parameter. The number
/// type parameter is best put last (`Qty<const D: Dim, N>`), and every use
/// names it: `Qty<LENGTH, f64>` is an `f64` length, `Qty<LENGTH, f32>` an
/// `f32` one, and `Qty<LENGTH, N>` a length in any.
///
/// `#[yadc(crate = "path")]` redirects the emitted `::yadc::` paths. The
/// crate defining `Qty` (or any new `pub type Foo = Qty<{ ... }>;` alias)
/// needs the ten feature flags listed on [`Dimension`](macro@Dimension); a
/// crate that only *uses* existing aliases needs `generic_const_exprs`
/// alone.
#[proc_macro_derive(Quantity, attributes(yadc, quantity, dim, value))]
pub fn derive_quantity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_quantity(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derive a UCUM-compliant `serde::Serialize` impl for a dimension struct.
///
/// Axes are dot-joined with exponents written directly after the symbol
/// (e.g. `s-2`, not `s^-2`). An axis's UCUM atom defaults to its field name and
/// is overridden with `#[ucum("kg")]`, so a field may be called `kilogram` and
/// still serialize as `kg`.
///
/// The exponent type is reached only through `Default` + `PartialEq` (the zero
/// test) and `::core::fmt::Display` (rendering), so an axis need not be `i8` -
/// or an integer at all. UCUM itself only writes integral exponents and
/// spells exponent one as a bare symbol, which here means "the exponent renders
/// as `1`"; a rational axis therefore needs a `Display`/`FromStr` pair that
/// round-trips its integral values.
///
/// The `#[kind]` field produces a UCUM unit prefix through
/// `UcumKind::ucum_prefix` - never by matching a variant name here - so which
/// labels carry a prefix is decided entirely by `#[ucum(prefix = ..)]` on the
/// label enum. Labels with no prefix (other than the plain label) are written
/// as a `{LabelName}` annotation. The dimensionless dimension
/// (`Dimension::DIMENSIONLESS`) serializes as `"1"`. `#[yadc(crate = "path", serde = "path")]` redirects the
/// emitted paths.
///
/// ```text
/// "m.kg.s-2"    // Force
/// "K{Affine}" // Temperature (a label with no prefix: annotation)
/// "K"           // TemperatureStep (the plain label)
/// "rad"         // Angle (with a `rad` axis)
/// "s-1.rad"     // AngularVelocity
/// "rad2"        // SolidAngle
/// "1{Log}"    // Gain (a kind on dimension one)
/// "1"           // Dimensionless
/// ```
///
/// Pair with [`UcumDeserialize`](macro@UcumDeserialize) to
/// round-trip. Both impls are emitted unconditionally - gate them behind a
/// feature in your crate with
/// `#[cfg_attr(feature = "serde", derive(...))]` if serde should be optional.
#[proc_macro_derive(UcumSerialize, attributes(yadc, kind, ucum))]
pub fn derive_ucum_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_ucum_serialize(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derive a `serde::Deserialize` impl matching
/// [`UcumSerialize`](macro@UcumSerialize).
///
/// Parses a UCUM unit string: `"1"` produces the zero dimension; a leading
/// token that `UcumKind::from_ucum_prefix` recognizes sets the label
/// (whatever `#[ucum(prefix = ..)]` declared) - the exact inverse of what
/// [`UcumSerialize`](macro@UcumSerialize) wrote, since both come from the same
/// list; a trailing `{Name}` annotation is read back with
/// `UcumKind::from_name`; otherwise axes are parsed from dot-separated tokens
/// with direct exponents (`m2`, `s-2`), matching each axis's `#[ucum("..")]`
/// symbol or its field name, at the field's own type. Missing axes default to
/// zero, the label to the plain label. Unknown unit symbols produce a
/// `serde::de::Error::custom`. `#[yadc(crate = "path", serde = "path")]`
/// redirects the emitted paths.
#[proc_macro_derive(UcumDeserialize, attributes(yadc, kind, ucum))]
pub fn derive_ucum_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_ucum_deserialize(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
