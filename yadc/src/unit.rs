//! Units: named ways to turn a number into a quantity and back.

/// A unit *of* `Q`, counted in numbers of type `N`: a named way to turn a
/// number into a `Q` and back.
///
/// On the whiteboard, a coordinate chart: two inverse functions between `Q`
/// and its numbers. That is the only way a number becomes a quantity or a
/// quantity a number - there is no raw access - so every number in a program
/// says which unit it is in.
///
/// A unit is a type of yours with an `impl Unit`:
///
/// - **a quantity value** is a unit of its own kind (`#[derive(Quantity)]`
///   writes it): `q.quantity(n)` is `q * n`, `q.value(x)` is `x / q`;
/// - **a unit type of a system**, whose definition its system stores as it
///   likes - a size and a zero in const parameters, `SIUnit<D, C>` - with one
///   hand-written impl for all of them;
/// - **anything else**: the Beaufort scale is two functions on a zero-sized
///   `struct Beaufort;`.
///
/// A choice of them - for a menu, a config file, a parser - is an enum of
/// yours with `#[derive(Unit)]`, each variant naming the unit type it is.
///
/// ```ignore
/// let t = Celsius {}.quantity(25.0);                     // build: 298.15 K
/// let f = Fahrenheit {}.value(t);                        // read: 77.0
/// let u = TemperatureUnit::Celsius;                      // a choice made at run time
/// println!("{:.1} {u}", u.value(t));                     // "25.0 °C": the unit's `Display`
/// ```
///
/// These two methods are the only way in and out; generic code asks for
/// `U: Unit<Q, N>`.
///
/// `impl Unit<Mass, f64> for Pound` is the whole declaration that the two
/// belong together: there is no structural check to pass instead, because two
/// systems may encode the same idea differently, so the impl *is* the proof. A
/// unit may implement it for any number of types, and `Q` may be anything: a
/// quantity, a point, a type of yours. `N` is any type of numbers: a float, an
/// integer, a grid's `[f64; 2]`.
///
/// Printing a quantity in a unit is the number and the unit's own `Display`:
/// `format!("{:.1} {}", unit.value(q), unit)`.
///
/// Dyn compatible: `&dyn Unit<Length, f64>` is "some unit of length", picked
/// at run time and still checked at compile time; `&U` implements `Unit<Q, N>`
/// whenever `U` does.
///
/// A `const trait`: a unit whose conversion is `const` builds quantities at
/// compile time (`const P: Pressure<f64> = Psi {}.quantity(14.7);`).
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a unit of `{Q}` counted in `{N}`",
    label = "not a unit of this quantity in these numbers",
    note = "a unit declares what it measures with a hand-written `impl Unit<Q, N>`, or - an enum \
            of units - with `#[unit(of = Q, number = N)]` on its `#[derive(Unit)]`"
)]
pub const trait Unit<Q, N> {
    /// The `Q` that is `value` in this unit: `Pound.quantity(180.0)`.
    fn quantity(&self, value: N) -> Q;
    /// The number `quantity` is in this unit: `Kilogram.value(m)`.
    fn value(&self, quantity: Q) -> N;
}

const impl<Q, N, U: [const] Unit<Q, N> + ?Sized> Unit<Q, N> for &U {
    fn quantity(&self, value: N) -> Q {
        (**self).quantity(value)
    }
    fn value(&self, quantity: Q) -> N {
        (**self).value(quantity)
    }
}

/// The error of parsing a unit from its symbol (`"mi".parse::<LengthUnit>()`):
/// no unit of that type has this symbol.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UnknownUnit;

impl ::core::fmt::Display for UnknownUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str("unknown unit symbol")
    }
}

impl ::core::error::Error for UnknownUnit {}
