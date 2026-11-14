//! Derives for families of static types: `derive_more`-style std impls,
//! `const` where they can be, for types that carry a value in their type.
//!
//! A family of types like `Bag<F: Fruit>` or `Qty<const P: Param, N>` checks
//! at compile time that you never add apples to oranges. const_derive_more
//! gives you, from derives on *your* types:
//!
//! 1. **Lifted operators** ([`#[derive(ConstOps)]`](macro@ConstOps)): `+ - *
//!    /` of a struct holding a number and const parameters, the number by
//!    its own operators and each lifted parameter by its type's -
//!    `Qty<A, N> * Qty<B, N>` is a `Qty<{ A * B }, N>`, and exists only where
//!    `A * B` does. Plus scaling, negation, the assign operators, `Sum` and
//!    number conversions.
//! 2. **The dynamic type**: one struct of yours that holds any of several
//!    static types - a whole family, generic over its type or const
//!    parameters, and plain types too - by moving their fields in and
//!    recording what was in the type as ordinary values
//!    ([`#[derive(Dynamic)]`](macro@Dynamic)), for when the choice is made at
//!    run time (read from a file, picked by a user). Not `dyn` - no trait
//!    objects, no dispatch. Its operators and methods are the fields' own,
//!    lifted - you state each one's output, the compiler checks it.
//! 3. **Closed sets**: your enums of a few members - visiting a value's
//!    contents with generic code ([`Visit`](trait@Visit)), one value or
//!    several at once; walking the variants themselves
//!    ([`Variants`](trait@Variants)); converting through the members
//!    ([`Members`](macro@Members)); and borrowing the dynamic type's
//!    arithmetic ([`Via`](macro@Via)).
//!
//! const_derive_more is general-purpose (nothing in it knows about units,
//! fruit or shapes) and depends on nothing. It has two pairs of traits - a
//! visit and its visitor, for a value ([`Visit`](trait@Visit), [`Visitor`])
//! and for a type's variants ([`Variants`](trait@Variants),
//! [`VariantVisitor`]) - and everything else its derives write is a std
//! trait (`From`, `TryFrom`, the operators, `PartialOrd`, `Sum`) or a trait
//! you name in `impl(..)`. `ConstOps` also writes, beside your struct, the
//! hidden checks behind its error messages; the other derives write only
//! impls. Your static types are untouched; the dynamic type points at them.
//!
//! The examples (`cargo run -p const_derive_more --example dynamic`, ...)
//! each show one feature, with the impls the derive writes spelled out by
//! hand beside it: `const_ops`, `dynamic`, `dynamic_const`, `members`,
//! `lifting`, `visit`, `members_via`, `pairing`, `variants`, `values`; and
//! `dynamic_uom` makes another crate's family dynamic.
//!
//! # The dynamic type: `#[derive(Dynamic)]`
//!
//! ```
//! # #![feature(const_trait_impl, const_cmp, const_ops, const_convert, const_destruct)]
//! # #![feature(derive_const, const_default)]
//! # use core::marker::PhantomData;
//! /// The fruits, as values.
//! #[derive_const(PartialEq)]
//! #[derive(Copy, Clone, Debug)]
//! pub enum FruitKind { Apple, Orange }
//!
//! /// A fruit, as a type: the fact a bag of it carries in its type.
//! pub trait Fruit { const KIND: FruitKind; }
//! #[derive(Copy, Clone, PartialEq, Debug)]
//! pub struct Apple;
//! impl Fruit for Apple { const KIND: FruitKind = FruitKind::Apple; }
//! #[derive(Copy, Clone, PartialEq, Debug)]
//! pub struct Orange;
//! impl Fruit for Orange { const KIND: FruitKind = FruitKind::Orange; }
//!
//! // The family: static types, untouched. A `Bag<Apple>` and a
//! // `Bag<Orange>` are different types, so adding them does not compile.
//! #[derive(Copy, Clone, PartialEq, Debug)]
//! pub struct Bag<F: Fruit> { count: u32, fruit: PhantomData<F> }
//!
//! // Its dynamic version: one type for every fruit. `F::KIND` is a fact
//! // (recorded, checked on the way back), `.count` payload (moved); the
//! // other field is declared, for the way back to fill with `Default`.
//! #[derive(Copy, Clone, PartialEq, Debug, const_derive_more::Dynamic)]
//! #[dynamic(Bag<F: Fruit> { fruit: PhantomData<F> } => { fruit: F::KIND, count: .count })]
//! #[dynamic(impl(Add -> Option<Self>, Mul<u32> -> Self))]
//! pub struct AnyBag {
//!     #[dynamic(same)]
//!     fruit: FruitKind,
//!     count: u32,
//! }
//! # fn main() {
//! # let apples = |count| Bag::<Apple> { count, fruit: PhantomData };
//! # let oranges = |count| Bag::<Orange> { count, fruit: PhantomData };
//! let a = AnyBag::from(apples(3));
//! let b = AnyBag::from(apples(2));
//! assert_eq!(Bag::<Apple>::try_from((a + b).unwrap()), Ok(apples(5)));
//! assert_eq!(a + AnyBag::from(oranges(2)), None);  // apples and oranges
//! assert_eq!(Bag::<Apple>::try_from(a * 4), Ok(apples(12)));
//! assert!(Bag::<Orange>::try_from(a).is_err());    // not oranges: `a` comes back
//! # }
//! ```
//!
//! A mapping `Static => { field: source, .. }` fills **every** field of the
//! struct, each from one of:
//!
//! | Source | Role | |
//! |---|---|---|
//! | a declared const parameter, `D` (from `Qty<const D: Dim>`) | fact | recorded by `From`, compared by `TryFrom` |
//! | a constant expression, `TIME`, `0u8`, or one naming a declared parameter, `F::KIND` (from `Bag<F: Fruit>`) | fact | written by `From`, compared by `TryFrom` |
//! | a field path, `.0`, `.since.0`, or `self` (the whole value: `f64 => { .., value: self }`) | payload | moved in by `From`, moved back by `TryFrom` |
//!
//! There may be several mappings - families and plain types - in one
//! struct. A field plays the same role in every mapping, and the facts must
//! identify the static type. The derive refuses a parameter that is in no
//! fact (two members differing only in it would come back as each other),
//! and two mappings whose facts could be equal, naming both. Where values
//! tell two mappings apart, they are checked at compile time: a `const`
//! assertion that the constants differ, or - when one side's fact comes from
//! its parameter (`F::KIND`) - an assertion in that mapping's impls, checked
//! for each member you use. The constants compared there are compared at
//! compile time, so their type's `PartialEq` is `const`
//! (`#[derive_const(PartialEq)]`).
//!
//! A constant cannot be told apart from a const parameter's value: `dim:
//! TIME` beside `dim: D` is refused, since `D` may be `TIME`. A tag - a
//! field filled with a different constant in each mapping - tells them
//! apart:
//!
//! ```
//! # #![feature(adt_const_params, const_trait_impl, derive_const)]
//! # #![feature(const_cmp, const_convert, const_destruct)]
//! # #[derive_const(PartialEq)]
//! # #[derive(Copy, Clone, Debug, Eq, core::marker::ConstParamTy)]
//! # pub struct Dim(i8);
//! # pub const TIME: Dim = Dim(1);
//! # #[derive(Copy, Clone, PartialEq, Debug)]
//! # pub struct Qty<const D: Dim>(f64);
//! # pub type Time = Qty<TIME>;
//! /// A point in time: not a quantity, though its payload is a `Time`.
//! #[derive(Copy, Clone, PartialEq, Debug)]
//! pub struct Instant { since: Time }
//!
//! #[derive(Copy, Clone, PartialEq, Debug, const_derive_more::Dynamic)]
//! #[dynamic(Qty<const D: Dim>       => { tag: 0u8, dim: D,    value: .0 })]
//! #[dynamic(Instant { since: Time } => { tag: 1u8, dim: TIME, value: .since.0 })]
//! pub struct Reading { tag: u8, dim: Dim, value: f64 }
//! # fn main() {
//! let t = Reading::from(Instant { since: Qty(2.0) });
//! assert!(Time::try_from(t).is_err());  // an instant is not a time
//! assert_eq!(Instant::try_from(t), Ok(Instant { since: Qty(2.0) }));
//! # }
//! ```
//!
//! Several unrelated types with nothing in common - an RGB and an HSV
//! colour - are not this: they are a plain enum, one variant each, with
//! [`Members`](macro@Members) for the std conversions (the `members`
//! example).
//!
//! Two members of *one* family are told apart by their parameter's facts
//! alone, so those must differ per member (`Apple::KIND != Orange::KIND`):
//! that is your family's business, which no derive can see.
//!
//! Sometimes two static types *are* the same value: a bare `f64` and a
//! dimensionless quantity. Say so with `shared` before the static type -
//! `#[dynamic(shared f64 => { dim: Dim::DIMENSIONLESS, value: self })]` - and
//! the two may hold the same facts: the way back gives the value as either.
//! A `shared` mapping is the same value as exactly one other mapping, and
//! is told apart from the rest like any mapping: one that could hold the
//! facts of two others, or of none, is refused.
//!
//! A static type the way back rebuilds as a struct expression, so the payload
//! must be visible where the derive is (same module, or `pub` fields), and
//! every other field of it - a `PhantomData`, or the types a deep path passes
//! through - is declared after the type: `Bag<F: Fruit> { fruit:
//! PhantomData<F> }`, `Instant { since: Time } => { .., value: .since.0 }`.
//! (A declared field that is not a path is filled with `Default`.)
//!
//! Per mapping it writes std `From<Static> for Self` and the way back, std
//! `TryFrom<Self> for Static` (`Error = Self`). The way back is an impl on
//! the static type, which the orphan rule allows wherever the dynamic type
//! is local - its first local type argument - so a family from another
//! crate works too, as long as its payload fields are visible.
//!
//! # Lifting: `#[dynamic(impl(..))]`
//!
//! Every lifted item does one thing: **something to every field**, building
//! `Self` from the results. What a field does is one of two things, and you
//! say which on the field:
//!
//! - **its own operator** (the default): `a.f ∘ b.f`, `a.f ∘ k`, `-a.f`,
//!   `Trait::method(a.f, ..)`. The field's type has that operator: a count
//!   adds, a dimension multiplies (its exponents add) and has a sum that may
//!   not exist.
//! - **equality**, `#[dynamic(same)]`: for a field that says *what* the
//!   value is and has no arithmetic - a fruit, a colour space. Two operands
//!   must be equal in it, otherwise there is no result; an operator with one
//!   `Self` operand (scaling, negation, a method) keeps it.
//!
//! You state the output, and the compiler checks it against the fields' own
//! impls (no inference, no helper):
//!
//! - `-> Self`: each field's result is the field's new value (its operator
//!   must return the field's type);
//! - `-> Option<Self>`: each field's result goes through `Into<Option<_>>`
//!   (a plain value or an `Option`), and any field's `None` makes the
//!   result `None`. An operator on two values with a `same` field is always
//!   this.
//!
//! | Item | Writes |
//! |---|---|
//! | `Mul -> Self`, `Add -> Option<Self>`, ... (`Mul`, `Div`, `Add`, `Sub`, `Rem`) | `Self ∘ Self`: `a.f ∘ b.f` for every field `f` |
//! | `Mul<K> -> ..`, `Div<K> -> ..` | by a `K`: `a.f ∘ k` for every field |
//! | `Neg -> ..` | `-a.f` for every field |
//! | `path::Trait::method(self, n: i8, ..) -> Option<Self>` | any trait's method, lifted the same way: `Trait::method(field, n, ..)` per field; `&self` and `&mut self` receivers pass `&field` / `&mut field`; associated types in the path (`Trait<D = X>`) |
//! | `path::Trait::getter(&self) -> T = self.field` | a getter: the trait's method returns that field; the trait's impl is a `const impl` (so the trait must be a `const trait`) when every item listed for that trait is a getter, an ordinary impl otherwise |
//! | `PartialOrd` | the payload compared in field order when the facts are equal; `None` otherwise |
//!
//! A field with an algebra of its own that takes part in `Mul<K>` says what
//! scaling does to it, in an impl of its own: a dimension scaled by a number
//! is the same dimension - `impl Mul<f64> for Dim { type Output = Dim; .. }`.
//! Under `Mul<K> -> Option<Self>` its result goes through `Into<Option<_>>`,
//! so `type Output = Option<Dim>` works as well, for a field that cannot
//! always be scaled; under `Mul<K> -> Self` it is `Dim`.
//!
//! Operators are `const impl`s with `[const]` bounds on the field types (so
//! a field whose operator is not `const` still works at run time); a lifted
//! method is an ordinary impl.
//!
//! # How it maps to plain Rust
//!
//! - The dynamic type is type erasure by moving fields: like `Box<dyn Any>`
//!   and `downcast`, but the "type id" is the facts, and the value is moved
//!   into your struct, not boxed. Erasing is std `From`, the way back std
//!   `TryFrom`.
//! - A lifted operator is the static's operator with the facts as values:
//!   where a static family computes `Qty<{ A * B }>` in the type, the
//!   dynamic type computes `dim: a.dim * b.dim` in a field.
//! - A **closed set** is an ordinary enum; [`Visit`](trait@Visit) is a
//!   `match` with one generic arm.
//!
//! # Closed sets
//!
//! An enum of a few members - the run-time choice an app offers. Its derives
//! know nothing about the dynamic type:
//!
//! | Derive | Attributes | Writes |
//! |---|---|---|
//! | [`Visit`](macro@Visit) | | [`Visit<X>`](trait@Visit) for the enum and for `&Enum`: hand the variant's contents (or references to them) to a generic [`Visitor`] |
//! | [`Variants`](macro@Variants) | | [`Variants<X>`](trait@Variants): hand every variant's name and constructor to a generic [`VariantVisitor`] |
//! | [`Members`](macro@Members) | `#[members(member)]`, `#[members(default)]` on fields; `#[members(convert(X, ..))]` | std only: `TryFrom<Enum> for M` per variant's member `M`, and `From<M> for Enum` where the variant's other fields are `#[members(default)]`; to and from `X` through the members |
//! | [`Via`](macro@Via) | `#[via(X, Add, Sub, Mul<K>, ..)]`; `#[via(keep)]` on fields | std only, the operators listed: `Self ∘ Self` through `X` (`Option<Self>`), `* K` and `/ K` on the member (`Self`) |
//! | [`Values`](macro@Values) | `#[value(V)]` on variants; `#[values(delegate(Trait { fn .. }), into(X), impl(..), for<..>)]` | for an enum whose variants each name a value: the traits listed, as the variant's value; `From<Enum> for X` through it, `Display` as it, `FromStr` by it ([`NoValue`]) |
//!
//! Two values at once come in two forms:
//!
//! - **checked when compiling**: `(a, b).visit_with(x)` hands both contents
//!   to `x`, which is a [`Visitor`] of every pair of members - one generic
//!   `impl<A: Shape, B: Shape> Visitor<(A, B)>`. Tuples nest, so any number
//!   of values work: `((a, b), c)`.
//! - **decided at run time, through the dynamic type**: an operator that
//!   exists for some pairs only (`Bag<Apple> + Bag<Apple>`, not `+
//!   Bag<Orange>`) cannot be a visitor of every pair, so `#[derive(Via)]`
//!   converts both operands into the dynamic type, computes there, and
//!   converts back: `Option<Self>`.
//!
//! Relating two closed sets - each unit menu to its quantity, each colour
//! family to its shape - is an associated type of a trait of yours (`impl
//! Paints for Red { type Shape = Circle; }`), read by a visitor: see the
//! `pairing` example. A list of the variants - for a menu - is a
//! [`VariantVisitor`] that collects them: see the `variants` example.
//!
//! # Const, and where not
//!
//! The derives emit `const impl`s, so a crate using them needs
//! `#![feature(const_trait_impl)]` (and `const_ops`, `const_convert`,
//! `const_cmp`, `const_destruct` as the impls it uses require; `const_default`
//! where a field is filled with `Default` - a declared field such as a
//! `PhantomData`, or `#[members(default)]`). Each impl is
//! `const` where its bounds are: `impl(PartialOrd)` is, when the dynamic
//! type's `PartialEq` is (`#[derive_const(PartialEq)]`). Not `const`: a
//! lifted method; `#[derive(Via)]` (it converts through `X` with ordinary
//! bounds, and clones the left operand for `#[via(keep)]`);
//! `#[derive(Variants)]`; and `#[derive(Visit)]` (several `[const]` bounds
//! of one trait over types with const-expression arguments, `Qty<{ .. }>`,
//! are ambiguous to today's trait solver, so its bounds are ordinary ones).
//!
//! `no_std`.
#![no_std]
#![feature(const_trait_impl)]

pub use const_derive_more_derive::{ConstOps, Dynamic, Members, Values, Variants, Via, Visit};

/// The error of parsing an enum of values ([`Values`](macro@Values)): no
/// variant's value prints as the string.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NoValue;

impl core::fmt::Display for NoValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("no value prints as this")
    }
}

impl core::error::Error for NoValue {}

/// Whether `value` prints as `s`, compared as it is written - no allocation.
/// What `#[derive(Values)]`'s `FromStr` asks of each variant's value.
#[doc(hidden)]
pub fn __prints_as(value: &dyn core::fmt::Display, s: &str) -> bool {
    struct Rest<'a>(&'a str, bool);
    impl core::fmt::Write for Rest<'_> {
        fn write_str(&mut self, part: &str) -> core::fmt::Result {
            match self.0.strip_prefix(part) {
                Some(r) => self.0 = r,
                None => self.1 = false,
            }
            Ok(())
        }
    }
    let mut rest = Rest(s, true);
    let _ = core::fmt::write(&mut rest, format_args!("{value}"));
    rest.1 && rest.0.is_empty()
}

/// Every const_derive_more trait, for `use const_derive_more::prelude::*`: the methods of
/// [`Visit`](trait@Visit) (`visit_with`) and [`Variants`](trait@Variants)
/// (`each_variant`) come from their traits.
pub mod prelude {
    pub use crate::{VariantVisitor, Variants, Visit, Visitor};
}

// ===== A value's case =========================================================

/// Something to do with one variant's contents, `T`: the variant's single
/// field, a tuple of its fields in order, or `()` for none - or references
/// to them, when the enum is visited by reference. Implement it generically,
/// with whatever bounds the job needs.
pub const trait Visitor<T> {
    /// What the visit produces.
    type Output;
    /// Do it.
    fn visit(self, contents: T) -> Self::Output;
}

/// Case analysis on an enum: hand this value's contents to `X`. On the
/// whiteboard, the one operation a tagged union has - give one function per
/// case (copairing). `#[derive(Visit)]` implements it for the enum (moving
/// the contents out) and for a reference to it (handing references).
///
/// A pair of values is visited at once: `(a, b).visit_with(x)` hands `x` the
/// pair of their contents, so `x` is a [`Visitor`] of every pair of members.
/// Pairs nest for more: `((a, b), c)`.
pub const trait Visit<X> {
    /// What every case produces.
    type Output;
    /// Hand this value's contents to `x`.
    fn visit_with(self, x: X) -> Self::Output;
}

/// The visitor of a pair's first value, holding the second and the pair's
/// visitor: it visits the second with [`WithFirst`]. Currying, for
/// [`Visit`] on a pair.
pub struct WithSecond<B, X>(B, X);

/// The visitor of a pair's second value, holding the first's contents and
/// the pair's visitor: it hands that visitor both. Currying, for [`Visit`]
/// on a pair.
pub struct WithFirst<A, X>(A, X);

impl<A, B, X: Visitor<(A, B)>> Visitor<B> for WithFirst<A, X> {
    type Output = X::Output;
    fn visit(self, b: B) -> X::Output {
        self.1.visit((self.0, b))
    }
}

impl<A, B, X> Visitor<A> for WithSecond<B, X>
where
    B: Visit<WithFirst<A, X>>,
{
    type Output = B::Output;
    fn visit(self, a: A) -> B::Output {
        self.0.visit_with(WithFirst(a, self.1))
    }
}

/// Two values at once: the first is visited by a visitor that visits the
/// second, and `x` gets both contents.
impl<A, B, X> Visit<X> for (A, B)
where
    A: Visit<WithSecond<B, X>>,
{
    type Output = A::Output;
    fn visit_with(self, x: X) -> A::Output {
        self.0.visit_with(WithSecond(self.1, x))
    }
}

// ===== A type's cases =========================================================

/// Something to do with one variant of the closed set `E`, without a value
/// of it: the variant's name, and the function that builds it from its
/// contents `T` (`()`, the single field, or a tuple of the fields).
///
/// What the function is good for is the visitor's business: build the
/// variant from a `T::default()`, from a parsed `T`, from a `T` converted
/// out of something else - or only note the name.
pub trait VariantVisitor<E, T> {
    /// Do it, for this variant.
    fn visit(&mut self, name: &'static str, make: fn(T) -> E);
}

/// Every variant of a closed set, in order: hand each one's name and
/// constructor to `X`. On the whiteboard, the injections of a sum type, one
/// per case. `#[derive(Variants)]` implements it.
pub trait Variants<X>: Sized {
    /// Visit every variant, in declaration order.
    fn each_variant(x: &mut X);
}
