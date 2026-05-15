use core::marker::ConstParamTy_;

/// A dimensional kind tag — the categorical part of a dimension, mutually
/// exclusive with the numeric exponent axes.
///
/// Implemented automatically by `#[derive(Kind)]`. The const ops form the kind
/// algebra: `Linear` is the identity, `Affine + Linear = Affine`, `Affine -
/// Affine = Linear`, etc. `name` enables string round-tripping used by
/// `DimensionSerialize`.
pub trait Kind:
    Copy
    + PartialEq
    + Eq
    + ::core::ops::Mul<Output = Self>
    + ::core::ops::Div<Output = Self>
    + ::core::ops::Add<Output = Self>
    + ::core::ops::Sub<Output = Self>
{
    const ZERO: Self;
    const INVALID: Self;
    const ANGLE: Self;
    const SOLID_ANGLE: Self;
    fn name(self) -> &'static str;
    /// Returns the UCUM unit prefix for non-linear kinds (`"rad"`, `"sr"`),
    /// or `None` for kinds that require no prefix in a UCUM string.
    fn ucum_prefix(self) -> Option<&'static str>;
}

/// A dimension descriptor — usually a small struct of `i8` exponents (one per
/// SI base axis) plus an optional `#[kind]` field carrying a user-defined
/// [`Kind`]-tagged enum.
///
/// Implemented automatically by `#[derive(Dimension)]`. Acts as the const
/// generic parameter on the user's quantity type (`Qty<const D: Dim>`).
///
/// The derive also emits `impl const Mul/Div/Add/Sub/PartialEq/Eq` on the Dim
/// type, and inherent const fn predicates (`valid`, `numeric_eq`, `can_*`)
/// for use in const-generic where clauses.
pub trait Dimension:
    ConstParamTy_
    + Copy
    + Eq
    + ::core::fmt::Debug
    + ::core::ops::Mul<Output = Self>
    + ::core::ops::Div<Output = Self>
    + ::core::ops::Add<Output = Self>
    + ::core::ops::Sub<Output = Self>
    + 'static
{
    /// The zero dimension (all exponents 0, kind defaulted to Linear).
    const ZERO: Self;

    /// Returns `true` unless the kind field is `Invalid`.
    fn valid(self) -> bool;

    /// Returns `true` when the numeric exponent axes equal those of `other`,
    /// ignoring the kind field.
    fn numeric_eq(self, other: Self) -> bool;

    fn can_mul(self, b: Self) -> bool {
        (self * b).valid()
    }
    fn can_div(self, b: Self) -> bool {
        (self / b).valid()
    }
    fn can_add(self, b: Self) -> bool {
        self.numeric_eq(b) && (self + b).valid()
    }
    fn can_sub(self, b: Self) -> bool {
        self.numeric_eq(b) && (self - b).valid()
    }
    fn can_add_assign(self, b: Self) -> bool {
        self.numeric_eq(b) && (self + b) == self
    }
    fn can_sub_assign(self, b: Self) -> bool {
        self.numeric_eq(b) && (self - b) == self
    }
}

/// A unit of measure tied to a single [`Dimension`] value.
///
/// Each variant of a unit enum (e.g. `MassUnit::Kilogram`) carries the
/// conversion slope ([`multiplier`](Self::multiplier)), the affine reference
/// point ([`offset`](Self::offset)), and two labels
/// ([`symbol`](Self::symbol), [`long`](Self::long)).
///
/// `multiplier` and `offset` return *typed quantities* — not raw `V` — so the
/// default trait bodies on [`StaticQuantity`] and [`DynamicQuantity`] can drive
/// `from_unit` / `as_unit` purely through the dim/kind algebra without ever
/// touching the inner numeric field. For pure-linear units (meters,
/// kilograms) [`Multiplier`](Self::Multiplier) and [`Offset`](Self::Offset)
/// are the same type; for affine units (Celsius, Fahrenheit)
/// [`Multiplier`](Self::Multiplier) is the Linear-kind "delta" type (e.g.
/// `TemperatureDelta`) while [`Offset`](Self::Offset) carries the Affine
/// kind.
///
/// [`DIM`](Self::DIM) captures the value-level dimension this unit applies
/// to — used by [`CanUseWithUnit`] to reject e.g. passing a `MassUnit` to a
/// `Force` constructor at compile time.
pub trait Unit: ::core::marker::Copy {
    /// The dimension type this unit belongs to.
    type D: Dimension;
    /// The numeric value type underlying [`Multiplier`](Self::Multiplier) and
    /// [`Offset`](Self::Offset).
    type V: Default + ::core::marker::Copy;
    /// Quantity type returned by [`multiplier`](Self::multiplier) — always
    /// Linear-kind. For a pure-linear unit (meters, kilograms) this equals
    /// [`Offset`](Self::Offset). For an affine unit (Celsius, Fahrenheit)
    /// this is the Linear "delta" version of the offset's dim — e.g.
    /// `TemperatureDelta` for a `Celsius` unit whose
    /// `Offset = Temperature`. Specify via `#[dim_delta(...)]` on the Unit
    /// derive; omitted = same as `Offset`.
    type Multiplier: StaticQuantity<V = Self::V>
        + ::core::ops::Mul<Self::V, Output = Self::Multiplier>
        + ::core::ops::Div<Self::Multiplier, Output: ::core::convert::Into<Self::V>>;
    /// Quantity type returned by [`offset`](Self::offset), carrying the
    /// unit's full kind (Linear for plain-scale units, Affine for units with
    /// a shifted origin). Specified via `#[dim(...)]` on the Unit derive.
    type Offset: StaticQuantity<D = Self::D, V = Self::V>;
    /// The specific dimension value this unit measures (e.g. `MASS_DIM`).
    const DIM: Self::D;
    /// Conversion slope from this unit to the canonical (SI base) value.
    /// `canonical = user_value * multiplier() + offset()`.
    fn multiplier(&self) -> Self::Multiplier;
    /// Additive reference point. Zero for plain-scale units; a point on the
    /// absolute scale (e.g. `Temperature(273.15)` for `Celsius`) for affine
    /// units.
    fn offset(&self) -> Self::Offset;
    /// Compact symbol (`"kg"`, `"m"`, `"Ω"`). Always present.
    fn symbol(&self) -> &'static str;
    /// Long-form name (`"kilograms"`, `"meters"`).
    fn long(&self) -> &'static str;
}

/// Which form a unit label should take when formatting via
/// [`StaticQuantity::fmt_as_unit`] or [`DynamicQuantity::fmt_as_unit`].
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Format {
    /// `unit.symbol()` — e.g. `"kg"`.
    Short,
    /// `unit.long()` — e.g. `"kilograms"`.
    Long,
}

/// Implemented by a quantity for each unit `U` whose dimension matches.
/// Bundles [`StaticQuantity`] with the arithmetic relationships needed to
/// construct, convert, and format quantities via `U`.
///
/// Auto-implemented by `#[derive(StaticQuantity)]`. Write `Q: CanUseWithUnit<U>`
/// in generic functions instead of spelling out the full bound set.
pub trait CanUseWithUnit<U: Unit>:
    StaticQuantity + ::core::ops::Sub<<U as Unit>::Offset, Output = <U as Unit>::Multiplier>
where
    <U as Unit>::Multiplier: ::core::ops::Add<<U as Unit>::Offset, Output = Self>,
{
}

/// Blanket impl so any [`StaticQuantity`] can satisfy a [`DynamicQuantity`] bound.
impl<T> DynamicQuantity for T
where
    T: StaticQuantity + ::core::convert::Into<::core::option::Option<T::V>>,
{
    type D = T::D;
    type V = T::V;
}

/// A statically-dimensioned quantity (e.g. `Qty<MASS_DIM>`).
///
/// Constructed via [`from_unit`](Self::from_unit) / read back via
/// [`as_unit`](Self::as_unit) / displayed via
/// [`fmt_as_unit`](Self::fmt_as_unit). All three are trait methods with
/// default bodies that compose through the dim/kind algebra — the derives
/// don't need to emit per-`Qty<D>` method bodies.
///
/// The [`CanUseWithUnit<U>`](CanUseWithUnit) bound on each method is supplied
/// by the StaticQuantity derive only when `U::DIM` matches `Self::DIM`, so
/// passing a wrong-dim unit fails at compile time.
///
/// `DIM` is deliberately not part of this trait — it is emitted as an
/// inherent `pub const` on the concrete quantity type by `#[derive(StaticQuantity)]`,
/// so it remains accessible (`Mass::DIM`) without being a trait requirement.
pub trait StaticQuantity:
    Sized
    + ::core::marker::Copy
    + ::core::ops::Mul<Self::V, Output = Self>
    + ::core::ops::Div<Self::V, Output = Self>
{
    /// The dimension type this quantity is parameterized over.
    type D: Dimension;
    /// The numeric value type.
    type V: Default + ::core::marker::Copy;

    /// The multiplicative identity of this quantity (`V`'s "one"). Used by
    /// the `Unit` derive to construct multiplier/offset values through type
    /// aliases. Override with `#[identities(one = MyType(1.0))]` on the struct.
    fn unit() -> Self;

    /// Construct from a value in `unit`. The `Self: CanUseWithUnit<U>` bound
    /// is implemented by the StaticQuantity derive only when `U::DIM` matches
    /// `Self::DIM`, so passing a wrong-dim unit fails at compile time.
    ///
    /// Default body: `unit.multiplier() * value + unit.offset()`.
    /// Works for both pure-linear and affine units — the kind algebra in
    /// `Add` produces `Self` whether `U::Multiplier + U::Offset` is
    /// `Linear + Linear = Linear` (pure-linear) or `Linear + Affine = Affine`
    /// (Celsius-style).
    fn from_unit<U>(value: Self::V, unit: U) -> Self
    where
        U: Unit<D = Self::D, V = Self::V>,
        Self: CanUseWithUnit<U>,
        U::Multiplier: ::core::ops::Add<U::Offset, Output = Self>,
    {
        unit.multiplier() * value + unit.offset()
    }

    /// Convert to a raw `Self::V` measured in `unit`. Same compile-time dim
    /// check as `from_unit`.
    ///
    /// Default body: `((self - unit.offset()) / unit.multiplier()).into()` —
    /// the arithmetic produces a dimensionless quantity that converts
    /// `Into<V>` (via the impl emitted by `#[derive(StaticQuantity)]` for the
    /// zero-dim specialisation). For affine units `Self - U::Offset =
    /// U::Multiplier` (Affine − Affine = Linear); for pure-linear units the
    /// two coincide.
    fn as_unit<U>(self, unit: U) -> Self::V
    where
        U: Unit<D = Self::D, V = Self::V>,
        Self: CanUseWithUnit<U>,
        U::Multiplier: ::core::ops::Add<U::Offset, Output = Self>,
    {
        ((self - unit.offset()) / unit.multiplier()).into()
    }

    /// Format this quantity in a given unit, returning a [`UnitDisplay`]
    /// that respects all standard format flags (`{:.2}`, `{:>10}`, `{:+}`).
    ///
    /// Default body: same arithmetic as [`as_unit`](Self::as_unit), wrapped
    /// in a [`UnitDisplay`] with the requested label form.
    fn fmt_as_unit<U>(self, unit: U, fmt: Format) -> UnitDisplay<Self::V>
    where
        U: Unit<D = Self::D, V = Self::V>,
        Self: CanUseWithUnit<U>,
        U::Multiplier: ::core::ops::Add<U::Offset, Output = Self>,
    {
        let label = match fmt {
            Format::Short => unit.symbol(),
            Format::Long => unit.long(),
        };
        UnitDisplay::__new(self.as_unit(unit), label)
    }
}

/// A dynamically-dimensioned quantity — the dimension is carried at runtime
/// instead of at the type level.
///
/// Useful when the dimension isn't known statically (user input,
/// heterogeneous collections). Arithmetic that combines dims (`*`, `/`)
/// always succeeds; arithmetic that requires matching dims (`+`, `-`,
/// `as_unit`, `fmt_as_unit`) returns `Option`, with `None` on dim mismatch.
///
/// Default trait bodies do the math in the unit's static-dim quantity types
/// (where the kind algebra is rich) and bridge to the dyn-dim carrier via
/// `Into` at the end. [`from_unit`](Self::from_unit) always succeeds;
/// [`as_unit`](Self::as_unit) relies on `Sub<Self, Output=Option<Self>>`
/// returning `None` on dim mismatch rather than an explicit dim check.
///
/// The `Into<Option<Self::V>>` supertrait is the single value-extraction path:
/// `Some(v)` when `self` is dimensionless, `None` otherwise. Implemented by
/// `#[derive(DynamicQuantity)]`.
pub trait DynamicQuantity:
    Sized + ::core::marker::Copy + ::core::convert::Into<::core::option::Option<Self::V>>
{
    /// The dimension type carried at runtime.
    type D: Dimension;
    /// The numeric value type.
    type V: Default + ::core::marker::Copy;

    /// Construct from a value in `unit` — the resulting dynamic quantity carries
    /// `unit.DIM` at runtime. Always succeeds.
    ///
    /// Default body: `(unit.multiplier() * value + unit.offset()).into()` —
    /// arithmetic happens in the unit's static-dim quantity types, then the
    /// result converts into the dyn-dim carrier via `From<Qty<D>>`.
    fn from_unit<U>(value: Self::V, unit: U) -> Self
    where
        U: Unit<D = Self::D, V = Self::V>,
        U::Multiplier: ::core::ops::Mul<Self::V, Output = U::Multiplier>
            + ::core::ops::Add<U::Offset, Output = U::Offset>,
        U::Offset: ::core::convert::Into<Self>,
    {
        (unit.multiplier() * value + unit.offset()).into()
    }

    /// Convert to a raw `Self::V` measured in `unit`. Returns `None` when
    /// the carried dims don't match the unit's `DIM`.
    ///
    /// Default body: attempts the subtraction and lets `Sub<Self,
    /// Output=Option<Self>>` return `None` on dim mismatch, then divides and
    /// converts via `Into<Option<V>>`. No explicit dim comparison required.
    fn as_unit<U>(&self, unit: U) -> Option<Self::V>
    where
        U: Unit<D = Self::D, V = Self::V>,
        U::Multiplier: ::core::convert::Into<Self>,
        U::Offset: ::core::convert::Into<Self>,
        Self: ::core::ops::Sub<Self, Output = Option<Self>> + ::core::ops::Div<Self, Output = Self>,
    {
        let off: Self = unit.offset().into();
        let mul: Self = unit.multiplier().into();
        let delta = (*self - off)?;
        (delta / mul).into()
    }

    /// Format in a given unit. Returns `None` if `unit`'s `DIM` doesn't match
    /// `self`'s carried dimension.
    fn fmt_as_unit<U>(&self, unit: U, fmt: Format) -> Option<UnitDisplay<Self::V>>
    where
        U: Unit<D = Self::D, V = Self::V>,
        U::Multiplier: ::core::convert::Into<Self>,
        U::Offset: ::core::convert::Into<Self>,
        Self: ::core::ops::Sub<Self, Output = Option<Self>> + ::core::ops::Div<Self, Output = Self>,
    {
        let label = match fmt {
            Format::Short => unit.symbol(),
            Format::Long => unit.long(),
        };
        Some(UnitDisplay::__new(self.as_unit(unit)?, label))
    }
}

/// `Display`/`Debug`-friendly wrapper around a value + unit label.
///
/// Returned by [`StaticQuantity::fmt_as_unit`] and [`DynamicQuantity::fmt_as_unit`].
/// Format flags (precision, width, fill, sign) pass through to the inner
/// value; the unit label is appended verbatim after a space.
pub struct UnitDisplay<V> {
    value: V,
    label: &'static str,
}

impl<V> UnitDisplay<V> {
    #[doc(hidden)]
    pub fn __new(value: V, label: &'static str) -> Self {
        Self { value, label }
    }
}

impl<V: ::core::fmt::Display> ::core::fmt::Display for UnitDisplay<V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.value, f)?;
        if !self.label.is_empty() {
            ::core::write!(f, " {}", self.label)?;
        }
        ::core::result::Result::Ok(())
    }
}

impl<V: ::core::fmt::Debug> ::core::fmt::Debug for UnitDisplay<V> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.value, f)?;
        if !self.label.is_empty() {
            ::core::write!(f, " {}", self.label)?;
        }
        ::core::result::Result::Ok(())
    }
}

#[doc(hidden)]
pub struct Assert<const COND: bool>;
#[doc(hidden)]
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
