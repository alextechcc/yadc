//! Units: named ways to turn a number into a quantity and back.

/// A unit *of* `Q`, counted in numbers of type `N`: two inverse functions,
/// `quantity` (a number to a `Q`) and `value` (a `Q` to its number).
///
/// They are the only way a number becomes a quantity or a quantity a
/// number - there is no raw access - so every number in a program says
/// which unit it is in. A system implements it for its unit type, once:
/// yadc-si's `SIUnit<D, C>`, a constant of which is each unit (`MILE`).
///
/// ```ignore
/// let d = MILE.quantity(3.0);            // build
/// let n = METRE.value(d);                // read: 4828.032
/// let t = CELSIUS.quantity(25.0);        // a unit with a zero: 298.15 K
/// println!("{:.1} {MILE}", MILE.value(d)); // its `Display`: "3.0 mi"
/// ```
///
/// Generic code asks for `U: Unit<Q, N>`; an enum of units implements it by
/// delegating to the unit each variant names (`dynamickit`'s
/// `#[derive(Values)]` writes that), and so does a unit chosen at run time. `Q` may be anything - a quantity, a point, a type of yours - and
/// `N` any type of numbers.
///
/// A `const trait`: a unit whose functions are `const` builds quantities at
/// compile time (`const P: Pressure<f64> = PSI.quantity(14.7);`).
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a unit of `{Q}` counted in `{N}`",
    label = "not a unit of this quantity in these numbers"
)]
pub const trait Unit<Q, N> {
    /// The `Q` that is `value` in this unit: `MILE.quantity(3.0)`.
    fn quantity(&self, value: N) -> Q;
    /// The number `quantity` is in this unit: `MILE.value(d)`.
    fn value(&self, quantity: Q) -> N;
}

impl<Q, N, U: [const] Unit<Q, N> + ?Sized> const Unit<Q, N> for &U {
    fn quantity(&self, value: N) -> Q {
        (**self).quantity(value)
    }
    fn value(&self, quantity: Q) -> N {
        (**self).value(quantity)
    }
}
