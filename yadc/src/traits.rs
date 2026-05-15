use core::marker::ConstParamTy_;

/// A dimensional kind tag - the categorical part of a dimension, mutually
/// exclusive with the numeric exponent axes.
///
/// Implemented automatically by `#[derive(Kind)]`. The const ops form the kind
/// algebra: `Linear` is the identity, `Affine + Linear = Affine`, `Affine -
/// Affine = Linear`, etc.
///
/// Only [`ZERO`](Self::ZERO) (the `Linear` identity) and [`INVALID`](Self::INVALID)
/// are required of every kind system. All other roles (Affine, Angle, SolidAngle,
/// Logarithmic, ...) are optional: a `#[derive(Kind)]` enum declares only the roles
/// it needs, and the derive emits algebra arms only for the roles present.
///
/// This trait is pure dimensional algebra - no serialization. The UCUM string
/// mapping lives in [`UcumKind`], a separate trait derived only when serde is on.
pub trait Kind:
    Copy
    + PartialEq
    + Eq
    + ::core::ops::Mul<Output = Self>
    + ::core::ops::Div<Output = Self>
    + ::core::ops::Add<Output = Self>
    + ::core::ops::Sub<Output = Self>
{
    /// The `Linear` identity kind (all-default dimension).
    const ZERO: Self;
    /// The sentinel produced by any ill-formed kind combination.
    const INVALID: Self;
}

/// UCUM serialization mapping for a [`Kind`].
///
/// Derived by `#[derive(UcumKind)]`, kept separate from [`Kind`] so the core
/// algebra trait carries no wire-format knowledge. Gate the derive behind your
/// serde feature alongside [`UcumSerialize`](macro@crate::UcumSerialize)
/// / [`UcumDeserialize`](macro@crate::UcumDeserialize), which are its
/// only callers.
///
/// [`ucum_prefix`](Self::ucum_prefix) and [`from_ucum_prefix`](Self::from_ucum_prefix)
/// are inverses (`Angle` <-> `"rad"`, `SolidAngle` <-> `"sr"`), so the
/// round-trip mapping lives in exactly one place.
pub trait UcumKind: Kind {
    /// The short name of this kind (its variant identifier), e.g. `"Affine"`.
    fn name(self) -> &'static str;
    /// Returns the UCUM unit prefix for prefix-bearing kinds (`"rad"`, `"sr"`),
    /// or `None` for kinds that require no prefix in a UCUM string.
    fn ucum_prefix(self) -> Option<&'static str>;
    /// Inverse of [`ucum_prefix`](Self::ucum_prefix): maps a UCUM prefix token
    /// back to its kind, or `None` if the token names no prefix-bearing kind.
    fn from_ucum_prefix(prefix: &str) -> Option<Self>;
}

/// A dimension descriptor - a small struct of `i8` exponents (one per base
/// axis) plus a required `#[kind]` field carrying a user-defined [`Kind`]-tagged
/// enum (its `Invalid` variant is the sentinel that makes operations rejectable).
///
/// Implemented automatically by `#[derive(Dimension)]`. Acts as the const
/// generic parameter on the user's quantity type (`Qty<const D: Dim>`).
///
/// The derive emits `impl const Mul/Div/Add/Sub/PartialEq/Eq` on the dim plus
/// inherent `const fn` predicates (`valid`, `numeric_eq`, `dims_eq`, `can_mul`,
/// `can_add`, …) that the `Qty` operators use in their const-generic `where`
/// bounds. Those are inherent methods on the concrete dim, not trait items — they
/// are only ever called on a known dim value, so the trait itself only needs to
/// supply [`ZERO`](Self::ZERO) generically.
///
/// **`Add`/`Sub` contract:** the derive's `Add`/`Sub` yield the `Invalid` kind
/// when the operands' numeric axes differ, so `(a + b).valid()` is the complete
/// add/sub validity check (no separate axis-equality test).
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
    /// The zero dimension (all axes 0, kind `Linear`).
    const ZERO: Self;
}

/// A unit of measure that converts to and from its [`Quantity`](Self::Quantity).
///
/// Each variant of a unit enum (e.g. `MassUnit::Kilogram`) maps a numeric value
/// in that unit to a quantity stored in base units, and back. The conversion
/// lives here: [`quantity`](Self::quantity) builds the quantity from a value, and
/// [`from_quantity`](Self::from_quantity) reads it back. The `Unit` derive emits
/// those bodies per variant from its `#[linear]` / `#[affine]` / `#[log]` /
/// `#[tan]` attribute; because conversion is just these two methods, the trait
/// can also be implemented by hand (e.g. a unit whose factor is only known at
/// runtime - see `examples/custom_unit_impl.rs`).
///
/// Because a unit names exactly one [`Quantity`](Self::Quantity), `MassUnit` can
/// only ever produce a `Mass` - the wrong-dimension case is structurally
/// impossible, so there is no separate compile-time guard.
pub trait Unit: ::core::marker::Copy {
    /// The quantity this unit measures (e.g. `Mass`). Carries the dimension
    /// and the numeric value type.
    type Quantity: Quantity;

    /// Build the quantity from `value` expressed in this unit. The stored value
    /// is in base units.
    fn quantity(self, value: <Self::Quantity as Quantity>::V) -> Self::Quantity;

    /// Read `q` back as a value in this unit.
    fn from_quantity(self, q: Self::Quantity) -> <Self::Quantity as Quantity>::V;

    /// Compact symbol (`"kg"`, `"m"`, `"Ohm"`).
    fn symbol(self) -> &'static str;

    /// Long-form name (`"kilograms"`, `"meters"`).
    fn long(self) -> &'static str;

    /// Render `q` in this unit, returning a [`UnitDisplay`] that honors all
    /// standard format flags (`{:.2}`, `{:>10}`, `{:+}`).
    fn format_quantity(
        self,
        q: Self::Quantity,
        fmt: Format,
    ) -> UnitDisplay<<Self::Quantity as Quantity>::V> {
        let label = match fmt {
            Format::Short => self.symbol(),
            Format::Long => self.long(),
        };
        UnitDisplay::__new(self.from_quantity(q), label)
    }

    /// Read any quantity that converts into [`Quantity`](Self::Quantity) - in
    /// particular a dynamic quantity (`DynQty`), whose `TryInto` fails (`None`)
    /// on a dimension mismatch. For the static quantity itself use
    /// [`from_quantity`](Self::from_quantity).
    fn try_from_quantity<Q>(self, q: Q) -> Option<<Self::Quantity as Quantity>::V>
    where
        Q: ::core::convert::TryInto<Self::Quantity>,
    {
        q.try_into().ok().map(|q| self.from_quantity(q))
    }

    /// Format any quantity that converts into [`Quantity`](Self::Quantity)
    /// (e.g. a dynamic quantity); `None` on a dimension mismatch.
    fn try_format_quantity<Q>(
        self,
        q: Q,
        fmt: Format,
    ) -> Option<UnitDisplay<<Self::Quantity as Quantity>::V>>
    where
        Q: ::core::convert::TryInto<Self::Quantity>,
    {
        q.try_into().ok().map(|q| self.format_quantity(q, fmt))
    }
}

/// Which form a unit label should take when formatting via
/// [`Unit::format_quantity`] / [`Unit::try_format_quantity`].
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Format {
    /// `unit.symbol()` - e.g. `"kg"`.
    Short,
    /// `unit.long()` - e.g. `"kilograms"`.
    Long,
}

/// A quantity - a value tagged with a [`Dimension`].
///
/// Two concrete shapes implement this:
/// - `Qty<const D: Dim>` - the dimension is a const-generic type parameter,
///   checked at compile time; cross-dim `*` / `/` produce a new `Qty` type and
///   same-dim `+` / `-` are gated by compile-time predicates.
/// - `DynQty` - the dimension is carried at runtime; `*` / `/` always succeed
///   and `+` / `-` return `Option` (`None` on a dim mismatch).
///
/// The trait is just a marker naming the two type parameters that define a
/// quantity - the dimension type [`D`](Self::D) and the value type [`V`](Self::V).
/// Every actual capability lives on the concrete type, emitted by the derives:
/// the arithmetic operators, value extraction (`Into<V>` / `Into<Option<V>>`),
/// the inherent `from_canonical()` constructor and `DIM` const. Generic code bounds
/// exactly the operations it uses (e.g. `Q: Quantity + Add<Q, Output = Q>`).
///
/// Conversion to and from units lives on the [`Unit`] trait, not here.
pub trait Quantity {
    /// The dimension type this quantity is tagged with.
    type D: Dimension;
    /// The numeric value type (stored in base units).
    type V;
}

/// `Display`/`Debug`-friendly wrapper around a value + unit label.
///
/// Returned by [`Unit::format_quantity`] / [`Unit::try_format_quantity`].
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
