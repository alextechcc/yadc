//! Refusals: why an operator a static family does not have does not compile.
//!
//! An operator the dimension algebra rejects has no impl - `METRE + SECOND`,
//! `altitude * 2.0` and `altitude * length` do not compile. Each refusal is
//! one of the traits below, implemented by `#[derive(Quantity)]` for every
//! operand pair where the refusal does not apply; the operator's impl asks
//! for them. The checks of one operator are exclusive, so a refused operator
//! fails one refusal. The same holds in the crate that defines the system and
//! in every crate that uses it (yadc's `yadc-tests` crate pins both).
//!
//! What the compiler shows depends on the operator:
//!
//! - `+ - * /` (and scaling, `n * a`), `.powi::<N>()`, `.root::<N>()`,
//!   `.inv()`, a unit read: the refusal's own message - "cannot add: the
//!   dimensions differ", "a point cannot be scaled", "cannot multiply:
//!   these have no product".
//! - a `Q: Quantity` bound: the trait's message ("not a `Quantity`: this
//!   value cannot be scaled"), with the refusal named in a `help:` line.
//! - `+= -= *= /=` and `Sum`: std's message ("cannot add-assign ..."), with
//!   the refusal named in a `help:` line (`the trait AddSameDimension<..> is
//!   not implemented for ..`) - the compiler reports these operators by
//!   their std trait.
//! - unary `-` of a point: rustc's "cannot apply unary operator `-`", with no
//!   reason - it reports no unmet bound for a unary operator.
//!
//! You never name these traits, except in `const` generic code over a
//! family's dimension parameter, which repeats an operator's bounds. The
//! `const fn`s are the checks themselves, evaluated at compile time from the
//! [`Dimension`]'s own operators: nothing here knows what a dimension looks
//! like.
//!
//! | Operator | Refusals |
//! |---|---|
//! | `a + b`, `a += b`, `Sum` | [`AddSameDimension`], [`AddNotTwoPoints`], [`AddInOrder`], [`AddLabels`] |
//! | `a - b` | [`SubSameDimension`], [`SubNotFromAmount`], [`SubLabels`] |
//! | `a -= b` | the three of `a - b`, and [`SubInPlace`] |
//! | `a * b`, `a / b` | [`MulExists`], [`DivExists`] |
//! | `a *= b`, `a /= b` | [`MulExists`] and [`MulInPlace`], [`DivExists`] and [`DivInPlace`] |
//! | scaling: `a * n`, `n * a`, `a / n`, `a *= n`, `a /= n`, `-a`; [`Quantity`](crate::Quantity), [`Unit<Self>`](crate::Unit) | [`ScaleNotPoint`], [`ScaleLabel`] |
//! | `a.powi::<N>()`, `a.inv()` | [`PowiExists`], [`InvExists`] |
//! | `a.root::<N>()` | [`RootHasSquare`], [`RootOfLabel`], [`RootExact`] |
//!
//! A point is a dimension of the affine kind: its own `*` has no result
//! ([`is_point`]). Everything here is phrased through the [`Dimension`]
//! trait, so a hand-written dimension gets the same messages.

use crate::Dimension;
use core::ops::{Div, Mul};

// --- How a check becomes a trait bound --------------------------------------
//
// A `const` bool cannot be matched against `true` in an impl header: the
// compiler would unify the unevaluated constant with `true` and report a
// refusal as "expected `false`, found `true`". So a check is read through an
// associated type - `Bool<{ check(A, B) }>: Value<Is = True>` - and that
// bound sits in the where clause of a refusal trait's impl marked
// `#[diagnostic::do_not_recommend]`: when it fails, the compiler reports the
// refusal trait itself, with its message.

/// A `bool` known at compile time, as a type: see [`Value`].
pub struct Bool<const B: bool>;

/// What a [`Bool`] is: [`True`] or [`False`].
pub trait Value {
    /// [`True`] or [`False`].
    type Is;
}

/// [`Bool<true>`]'s [`Value`].
pub struct True;

/// [`Bool<false>`]'s [`Value`].
pub struct False;

impl Value for Bool<true> {
    type Is = True;
}

impl Value for Bool<false> {
    type Is = False;
}

// --- The dimension's own operators, as total functions ----------------------
//
// A result type is written `Qty<{ product_or_one::<Dim>(A, B) }>`: it must
// not panic for a refused pair (the compiler may compute it while trying
// impls out), so a missing result is `DIMENSIONLESS` - and the refusal
// traits remove the impl.

/// `a * b`, read through `Into<Option<D>>`: `None` where there is no product.
pub const fn product<D: [const] Dimension>(a: D, b: D) -> Option<D>
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    (a * b).into()
}

/// `a / b`, read through `Into<Option<D>>`: `None` where there is no
/// quotient.
pub const fn quotient<D: [const] Dimension>(a: D, b: D) -> Option<D>
where
    <D as Div>::Output: [const] Into<Option<D>>,
{
    (a / b).into()
}

/// The dimension inside `Some`, or [`Dimension::DIMENSIONLESS`].
pub const fn or_dimensionless<D: Dimension>(dim: Option<D>) -> D {
    match dim {
        Some(dim) => dim,
        None => D::DIMENSIONLESS,
    }
}

/// [`product`], or `DIMENSIONLESS` where there is none.
pub const fn product_or_one<D: [const] Dimension>(a: D, b: D) -> D
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    or_dimensionless(product(a, b))
}

/// [`quotient`], or `DIMENSIONLESS` where there is none.
pub const fn quotient_or_one<D: [const] Dimension>(a: D, b: D) -> D
where
    <D as Div>::Output: [const] Into<Option<D>>,
{
    or_dimensionless(quotient(a, b))
}

/// `1 / a`, or `DIMENSIONLESS` where there is none.
pub const fn reciprocal_or_one<D: [const] Dimension>(a: D) -> D
where
    <D as Div>::Output: [const] Into<Option<D>>,
{
    quotient_or_one(D::DIMENSIONLESS, a)
}

/// `a - b`, or `DIMENSIONLESS` where there is none.
pub const fn difference_or_one<D: [const] Dimension>(a: D, b: D) -> D {
    or_dimensionless(a - b)
}

/// [`PartialPowi::partial_powi`](crate::PartialPowi::partial_powi), or
/// `DIMENSIONLESS` where there is none.
pub const fn power_or_one<D: [const] Dimension>(a: D, n: i8) -> D {
    or_dimensionless(a.partial_powi(n))
}

/// [`PartialRoot::partial_root`](crate::PartialRoot::partial_root), or
/// `DIMENSIONLESS` where there is none.
pub const fn root_or_one<D: [const] Dimension>(a: D, n: i8) -> D {
    or_dimensionless(a.partial_root(n))
}

// --- What the messages tell apart -------------------------------------------

/// A point: a dimension of the affine kind. It has no products and no sum
/// with itself, and two of it differ by something else - an amount.
#[allow(clippy::eq_op)] // `a + a`, `a - a`: what two of one dimension give
pub const fn is_point<D: [const] Dimension>(a: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    product(a, D::DIMENSIONLESS).is_none()
        && (a + a).is_none()
        && matches!(a - a, Some(d) if d != a)
}

/// A dimension with no square - a point or a logarithm: `a * a` has no
/// result, so neither has any power or root of it.
const fn has_no_square<D: [const] Dimension>(a: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    product(a, a).is_none()
}

/// The amount a dimension measures, label forgotten: its plain form, and for
/// a point the plain form of the difference of two such points (an altitude
/// measures a length).
const fn amount_of<D: [const] Dimension>(a: D) -> Option<D>
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    if is_point(a) {
        // The dimension's own `-` of a point and itself: an amount.
        #[allow(clippy::eq_op)]
        let difference = a - a;
        match difference {
            Some(d) => Some(d.plain()),
            None => None,
        }
    } else {
        Some(a.plain())
    }
}

/// `a` and `b` measure the same amount: they differ at most in label.
const fn same_dimension<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    match (amount_of(a), amount_of(b)) {
        (Some(x), Some(y)) => x == y,
        (None, None) => true,
        _ => false,
    }
}

/// `a + b` is `a`: the sum exists (and keeps its left operand, as a sum does).
const fn sum_is_left<D: [const] Dimension>(a: D, b: D) -> bool {
    match a + b {
        Some(d) => d == a,
        None => false,
    }
}

/// Scaling is repeated addition: `a + a` is `a` again.
const fn scales<D: [const] Dimension>(a: D) -> bool {
    sum_is_left(a, a)
}

// --- The checks: `true` where the refusal does not apply --------------------

/// `a + b`: not refused for differing dimensions.
pub const fn add_same_dimension<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    sum_is_left(a, b) || same_dimension(a, b) || sum_is_left(b, a)
}

/// `a + b`: not refused as two points.
pub const fn add_not_two_points<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    sum_is_left(a, b) || !same_dimension(a, b) || !(is_point(a) && is_point(b))
}

/// `a + b`: not refused as written in the wrong order - `b + a` exists (an
/// amount plus a point, a gain plus a level).
pub const fn add_in_order<D: [const] Dimension>(a: D, b: D) -> bool {
    sum_is_left(a, b) || !sum_is_left(b, a)
}

/// `a + b`: not refused for the labels.
pub const fn add_labels<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    sum_is_left(a, b) || !same_dimension(a, b) || is_point(b) || sum_is_left(b, a)
}

/// `a - b`: not refused for differing dimensions.
pub const fn sub_same_dimension<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    (a - b).is_some() || same_dimension(a, b)
}

/// `a - b`: not refused as an amount minus a point.
pub const fn sub_not_from_amount<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    (a - b).is_some() || !same_dimension(a, b) || is_point(a) || !is_point(b)
}

/// `a - b`: not refused for the labels.
pub const fn sub_labels<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    (a - b).is_some() || !same_dimension(a, b) || is_point(b)
}

/// `a -= b`: the difference, where there is one, is `a` again.
pub const fn sub_in_place<D: [const] Dimension>(a: D, b: D) -> bool {
    match a - b {
        Some(d) => d == a,
        None => true,
    }
}

/// `a * b` exists.
pub const fn mul_exists<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    product(a, b).is_some()
}

/// `a / b` exists.
pub const fn div_exists<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Div>::Output: [const] Into<Option<D>>,
{
    quotient(a, b).is_some()
}

/// `a *= b`: the product, where there is one, is `a` again.
pub const fn mul_in_place<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    match product(a, b) {
        Some(d) => d == a,
        None => true,
    }
}

/// `a /= b`: the quotient, where there is one, is `a` again.
pub const fn div_in_place<D: [const] Dimension>(a: D, b: D) -> bool
where
    <D as Div>::Output: [const] Into<Option<D>>,
{
    match quotient(a, b) {
        Some(d) => d == a,
        None => true,
    }
}

/// Scaling: not refused as a point.
pub const fn scale_not_point<D: [const] Dimension>(a: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    scales(a) || !is_point(a)
}

/// Scaling: not refused for the label.
pub const fn scale_label<D: [const] Dimension>(a: D) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    scales(a) || is_point(a)
}

/// `a^n` exists.
pub const fn powi_exists<D: [const] Dimension>(a: D, n: i8) -> bool {
    a.partial_powi(n).is_some()
}

/// `1 / a` exists.
pub const fn inv_exists<D: [const] Dimension>(a: D) -> bool
where
    <D as Div>::Output: [const] Into<Option<D>>,
{
    quotient(D::DIMENSIONLESS, a).is_some()
}

/// The `n`-th root: not refused as a point's or a logarithm's.
pub const fn root_has_square<D: [const] Dimension>(a: D, n: i8) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    a.partial_root(n).is_some() || !has_no_square(a)
}

/// The `n`-th root: not refused as a label's (a label has no root but its
/// first, though its exponents may).
pub const fn root_of_label<D: [const] Dimension>(a: D, n: i8) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    a.partial_root(n).is_some() || has_no_square(a) || a.plain().partial_root(n).is_none()
}

/// The `n`-th root: not refused as inexact.
pub const fn root_exact<D: [const] Dimension>(a: D, n: i8) -> bool
where
    <D as Mul>::Output: [const] Into<Option<D>>,
{
    a.partial_root(n).is_some() || has_no_square(a) || a.plain().partial_root(n).is_some()
}

// --- The refusals ------------------------------------------------------------

macro_rules! refusals {
    ($($(#[$doc:meta])* $name:ident [$($gen:tt)*] = $msg:literal, $label:literal, $note:literal;)*) => {$(
        $(#[$doc])*
        #[diagnostic::on_unimplemented(message = $msg, label = $label, note = $note)]
        pub trait $name<$($gen)*> {}
    )*};
}

refusals! {
    /// `a + b` (and `a += b`, `Sum`) of two different dimensions: a length
    /// and a time.
    AddSameDimension[R] = "cannot add: the dimensions differ",
        "the dimensions differ",
        "only values of one dimension add: a length to a length, a time to a time";
    /// `a + b` of two points: an altitude plus an altitude.
    AddNotTwoPoints[R] = "cannot add a point to a point",
        "a point cannot be added to a point",
        "a point's zero is a convention, so a sum of two would depend on it: \
         subtract them instead (`point - point` is an amount), or move one by an amount \
         (`point + amount`)";
    /// `a + b` of an amount and a point: a length plus an altitude.
    AddInOrder[R] = "cannot add these in this order",
        "write the other operand first",
        "a sum keeps its left operand's type: an amount moves a point (`point + amount`), a \
         gain moves a level (`level + gain`), so the point or the level goes on the left";
    /// `a + b` whose labels have no sum: a temperature step plus an absolute
    /// temperature.
    AddLabels[R] = "cannot add: the labels have no sum",
        "these labels do not add",
        "a sum keeps its left operand's type, and the labels' rules decide what may be \
         added to it - here there is none (a label's sums are declared on the kind enum: \
         `#[kind(add(X) = Y)]`); two levels sum to the level of a product, another \
         dimension: add the quantities they are the levels of";
    /// `a - b` of two different dimensions.
    SubSameDimension[R] = "cannot subtract: the dimensions differ",
        "the dimensions differ",
        "only values of one dimension subtract: a length from a length, a time from a time";
    /// `a - b` of an amount and a point: a length minus an altitude.
    SubNotFromAmount[R] = "cannot subtract a point from an amount",
        "a point cannot be subtracted from an amount",
        "an amount minus a point would depend on where the point's zero was \
         put: subtract two points (`point - point` is an amount), or move a point by an \
         amount (`point - amount`)";
    /// `a - b` whose labels have no difference: a temperature step minus an
    /// absolute temperature.
    SubLabels[R] = "cannot subtract: the labels have no difference",
        "these labels do not subtract",
        "the labels' rules decide what may be subtracted - here there is none (a label's \
         differences are declared on the kind enum: `#[kind(sub(X) = Y)]`)";
    /// `a -= b` whose difference is another type: an altitude `-=` an
    /// altitude.
    SubInPlace[R] = "`-=` cannot change the type: the difference is another quantity",
        "the difference has another type",
        "`a -= b` stores `a - b` in `a`, so the difference must have `a`'s type: two points \
         differ by an amount, two absolute temperatures by a step - write `let d = a - b;`";
    /// `a * b` with no product: a point or a level times anything.
    MulExists[R] = "cannot multiply: these have no product",
        "no product",
        "the dimension's own `*` has no result here. Beside a plain number a product is a \
         scaling, which only what adds to itself has. A point's zero is a convention, so its \
         products would depend on it: multiply the amount `point - zero` instead. A level \
         is a logarithm, and adding levels is what multiplies: multiply the quantity it is \
         the level of";
    /// `a / b` with no quotient: a point or a level divided by anything, or
    /// anything divided by a point or a level.
    DivExists[R] = "cannot divide: these have no quotient",
        "no quotient",
        "the dimension's own `/` has no result here. A point's zero is a convention, so its \
         quotients would depend on it: divide the amount `point - zero` instead. A level is \
         a logarithm, and subtracting levels is what divides: divide the quantity it is the \
         level of";
    /// `a *= b` whose product is another type: a length `*=` a length.
    MulInPlace[R] = "`*=` cannot change the type: the product is another quantity",
        "the product has another type",
        "`a *= b` stores `a * b` in `a`, so `b` must be a plain number (a dimensionless \
         quantity) - write `let p = a * b;`";
    /// `a /= b` whose quotient is another type: a length `/=` a time.
    DivInPlace[R] = "`/=` cannot change the type: the quotient is another quantity",
        "the quotient has another type",
        "`a /= b` stores `a / b` in `a`, so `b` must be a plain number (a dimensionless \
         quantity) - write `let q = a / b;`";
    /// Scaling a point: `altitude * 2.0`, `-altitude`; a point is not a
    /// [`Quantity`](crate::Quantity), nor a unit of itself.
    ScaleNotPoint[] = "a point cannot be scaled",
        "a point cannot be scaled",
        "scaling is repeated addition, and a point cannot be added to a point: a point is \
         not a `Quantity` nor a unit of itself - move it by an amount (`point + amount`) and \
         read it on a scale: a zero and a step, as a unit";
    /// Scaling a label that cannot be added to itself.
    ScaleLabel[] = "cannot scale: this label cannot be added to itself",
        "this label cannot be scaled",
        "scaling is repeated addition, and this dimension has no sum with itself that is \
         itself - its label declares none, or it is a level, whose double is the level of a \
         square: it is not a `Quantity` nor a unit of itself";
    /// `a.powi::<N>()` with no power: a point, a logarithm.
    PowiExists[const N: i8] = "this power has no dimension",
        "no dimension is this power",
        "a power is repeated `*`, and a point or a logarithm has no square";
    /// `a.inv()` with no reciprocal: a point, a level.
    InvExists[] = "this reciprocal has no dimension: a point or a level has no reciprocal",
        "a point or a level has no reciprocal",
        "a reciprocal is `1 / a`, and a point or a level has no quotients";
    /// `a.root::<N>()` of a point or a logarithm.
    RootHasSquare[const N: i8] = "a point or a logarithm has no roots",
        "a point or a logarithm has no roots",
        "a root is the inverse of a power, and a point or a logarithm has no powers";
    /// `a.root::<N>()` of a labelled dimension whose exponents have the root.
    RootOfLabel[const N: i8] = "a label has no root",
        "a label has no root",
        "a label survives a product only beside a plain number, so its powers are all \
         plain and only its first root is itself";
    /// `a.root::<N>()` with no exact dimension: √m with integer exponents.
    RootExact[const N: i8] = "this root has no dimension",
        "no dimension is this root",
        "with integer exponents only exact roots exist (√m² is m, √m is none); a dimension \
         with rational exponents (`yadc::Exponent`) has every root";
}
