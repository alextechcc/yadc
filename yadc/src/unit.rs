//! Units: named ways to turn a number into a quantity and back.

/// A unit *of* `Q`, counted in numbers of type `N`: a named way to turn a
/// number into a `Q` and back.
///
/// On the whiteboard, a coordinate chart: two inverse functions between `Q`
/// and its numbers. That is the only way a number becomes a quantity or a
/// quantity a number - there is no raw access - so every number in a program
/// says which unit it is in.
///
/// A unit is a value - a constant - of types that are yours:
///
/// - **a quantity constant**: every [`Quantity`](crate::Quantity) is a unit
///   of itself (`#[derive(Quantity)]` writes it) - `BAR.quantity(n)` is
///   `BAR * n`, `BAR.value(q)` is `q / BAR`;
/// - **a type of yours with a hand-written `impl Unit`**: two functions. A
///   scale is one such type, a zero and a step in a tuple struct - `n` is
///   `zero + n · step`, a value reads `(value - zero) / step` - so that °C is
///   `Scale(ICE_POINT, KELVIN)`, feet above *this* runway `Scale(runway,
///   FOOT)`, dBm `Scale(MILLIWATT_LEVEL, DECIBEL)` on a logarithmic line; the
///   Beaufort scale is another, on a zero-sized `struct Beaufort;`.
///
/// A choice of them - for a menu, a config file, a parser - is an enum of
/// yours with `#[derive(Unit)]`, each variant naming the unit value it is.
///
/// ```ignore
/// pub const CELSIUS: Scale<Temperature, TemperatureStep> = Scale(ICE_POINT, KELVIN);
///
/// let t = CELSIUS.quantity(25.0);                         // build: 298.15 K
/// let f = FAHRENHEIT.value(t);                             // read: 77.0
/// let v = MILE(60.0) / HOUR(1.0);                          // a constant is also called
/// let any: &dyn Unit<Pressure> = &BAR;                     // picked at run time
/// let u = TemperatureUnit::Celsius;
/// println!("{:.1} {u}", u.value(t));                       // "25.0 °C": the enum's `Display`
/// ```
///
/// The derives also write std's `Fn` for a unit, over these two methods, so
/// a unit is called like the function it is: on a number it builds, on a
/// quantity it reads. Generic code may ask for either - `U: Unit<Q>`, or
/// `impl Fn(f64) -> Q` and `impl Fn(Q) -> f64`.
///
/// `impl Unit<Mass> for MassUnit` is the whole declaration that the two
/// belong together: there is no structural check to pass instead, because two
/// systems may encode the same idea differently, so the impl *is* the proof. A
/// unit may implement it for any number of types, in any crate that owns one
/// of them - there is no "home" quantity - and `Q` may be anything: a
/// quantity, a point, a type of yours. `N` is usually a float (`f64` by
/// default), but can be any type of numbers: a grid's position is
/// `[f64; 2]`.
///
/// When a unit measures several types, building names the target:
/// `let m: Mass = Pound.quantity(10.0)`. Reading never needs it. A read that
/// can fail - a value whose dimension is only known at run time - is
/// `TryFrom` first, then the unit.
///
/// Printing a quantity in a unit is the number and the unit's own `Display`
/// (which `#[derive(Unit)]` writes: `{}` the symbol, `{:#}` the name):
/// `format!("{:.1} {}", unit.value(q), unit)`.
///
/// Dyn compatible: `&dyn Unit<Length>` is "some unit of length", picked at
/// run time and still checked at compile time; `&U` implements `Unit<Q, N>`
/// whenever `U` does.
///
/// A `const trait`: quantity constants and scales convert at compile time
/// (`const P: Pressure = PSI.quantity(14.7);`); a level read as a linear
/// quantity (its conversion takes a logarithm) and most hand-written function
/// units do not.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a unit of `{Q}` counted in `{N}`",
    label = "not a unit of this quantity in these numbers",
    note = "a unit declares what it measures with `#[unit(of = ..)]` on its `#[derive(Unit)]`, or a \
            hand-written `impl Unit<Q, N>`; its numbers are `f64` unless `#[unit(number = ..)]` says \
            otherwise"
)]
pub const trait Unit<Q, N = f64> {
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
