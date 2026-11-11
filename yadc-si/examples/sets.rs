//! A settings menu where the user picks a quantity and the unit to show it
//! in: a closed set of quantities, and the set of their units beside it.
//!
//! `Quantities` is an enum of yours - the quantities this app measures.
//! `Units` is another enum of yours: for each quantity, its family of units.
//! Which quantity a unit family measures is a fact about the types, written
//! as one: `impl Measures for PressureUnit { type Quantity = Pressure; }`.
//! dynamickit gives the plumbing:
//!
//! - `Variants` walks the choices: one group per quantity, then the units
//!   of each - the two menus a UI would render;
//! - `Visit` hands the chosen unit family to a generic visitor: one builds
//!   the family's quantity from a number, one reads a quantity if it is the
//!   family's;
//! - `Members` and `Via` convert the set through the open type,
//!   `DynamicSIQuantity`, and borrow its arithmetic.
//!
//! Run with: cargo run -p yadc-si --example sets
#![feature(generic_const_exprs, const_trait_impl)]
#![feature(const_convert, const_destruct, const_ops)]
#![feature(fn_traits, unboxed_closures)]
#![allow(incomplete_features)]

use core::fmt;
use core::str::FromStr;
use dynamickit::{Members, VariantVisitor, Variants, Via, Visit, Visitor};
use yadc::{Unit, UnknownUnit};
use yadc_si::mechanical::consts::{BAR, PASCAL, PSI, STANDARD_ATMOSPHERE};
use yadc_si::mechanical::{Area, AreaUnit, Force, ForceUnit, Pressure};
use yadc_si::{DynamicSIQuantity, KILO};
use yadc_si::thermal::{ThermodynamicTemperature, ThermodynamicTemperatureUnit};

/// What the app measures: a closed set. It converts to and from the open
/// type through its members, and does its arithmetic there.
#[derive(Clone, Copy, Debug, Members, Via)]
#[members(convert(DynamicSIQuantity))]
#[via(DynamicSIQuantity, Add, Sub, Mul, Div, Mul<f64>, Div<f64>)]
enum Quantities {
    Pressure(Pressure<f64>),
    Temperature(ThermodynamicTemperature<f64>),
    Area(Area<f64>),
    Force(Force<f64>),
}

/// The units of pressure the app offers: a family of your own, a prefixed
/// unit among them.
#[derive(Clone, Copy, PartialEq, Debug, yadc::Unit, Variants)]
#[unit(of = Pressure<f64>)]
enum PressureUnit {
    #[unit("Pa", "pascals", by = PASCAL)]
    Pascal,
    #[unit("kPa", "kilopascals", by = KILO * PASCAL)]
    Kilopascal,
    #[unit("bar", "bar", by = BAR)]
    Bar,
    #[unit("atm", "standard atmospheres", by = STANDARD_ATMOSPHERE)]
    StandardAtmosphere,
    #[unit("psi", "pounds-force per square inch", by = PSI)]
    Psi,
}

/// The setting: which quantity, shown in which unit.
#[derive(Clone, Copy, PartialEq, Debug, Visit, Variants)]
enum Units {
    Pressure(PressureUnit),
    Temperature(ThermodynamicTemperatureUnit),
    Area(AreaUnit),
    Force(ForceUnit),
}

/// The relation between the two sets: the quantity a unit family measures.
trait Measures: Unit<Self::Quantity> {
    type Quantity;
}
impl Measures for PressureUnit {
    type Quantity = Pressure<f64>;
}
impl Measures for ThermodynamicTemperatureUnit {
    type Quantity = ThermodynamicTemperature<f64>;
}
impl Measures for AreaUnit {
    type Quantity = Area<f64>;
}
impl Measures for ForceUnit {
    type Quantity = Force<f64>;
}

/// Build: a number in a unit is that unit's quantity, put in the set by
/// std `From`.
struct Build(f64);

impl<U: Measures> Visitor<U> for Build
where
    Quantities: From<U::Quantity>,
{
    type Output = Quantities;
    fn visit(self, unit: U) -> Quantities {
        Quantities::from(unit.quantity(self.0))
    }
}

/// Read: the number a quantity is in a unit - when it is that unit's
/// quantity, taken out of the set by std `TryFrom`.
struct Read(Quantities);

impl<U: Measures> Visitor<U> for Read
where
    U::Quantity: TryFrom<Quantities>,
{
    type Output = Option<f64>;
    fn visit(self, unit: U) -> Option<f64> {
        let quantity = U::Quantity::try_from(self.0).ok()?;
        Some(unit.value(quantity))
    }
}

/// The unit-set API, in your words: each a one-liner over a visit.
impl Units {
    fn quantity(self, n: f64) -> Quantities {
        self.visit_with(Build(n))
    }

    /// The number, when the quantity is the one this unit measures.
    fn try_value(self, q: Quantities) -> Option<f64> {
        self.visit_with(Read(q))
    }
}

/// The menu, from `Variants`. A flat enum's choices: every variant without
/// contents, built.
struct Choices<E>(Vec<E>);

impl<E> VariantVisitor<E, ()> for Choices<E> {
    fn visit(&mut self, _name: &'static str, make: fn(()) -> E) {
        self.0.push(make(()));
    }
}

/// An enum of enums' groups: each variant's name, and the choices of the
/// enum it wraps, wrapped.
struct Groups<E>(Vec<(&'static str, Vec<E>)>);

impl<E, T: Variants<Choices<T>>> VariantVisitor<E, T> for Groups<E> {
    fn visit(&mut self, name: &'static str, make: fn(T) -> E) {
        let mut inner = Choices(Vec::new());
        T::each_variant(&mut inner);
        self.0.push((name, inner.0.into_iter().map(make).collect()));
    }
}

/// Menu 1 is the groups' names; menu 2 a group's entries.
fn menu() -> Vec<(&'static str, Vec<Units>)> {
    let mut groups = Groups(Vec::new());
    Units::each_variant(&mut groups);
    groups.0
}

/// Print whichever unit it is through its own `Display`, the caller's flags
/// included (`{:#}` is the unit's name).
struct ShowUnit<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl<U: fmt::Display> Visitor<U> for ShowUnit<'_, '_> {
    type Output = fmt::Result;
    fn visit(self, unit: U) -> fmt::Result {
        fmt::Display::fmt(&unit, self.0)
    }
}

/// The saved form: `Pressure:kPa`; `{:#}` spells the unit out,
/// `Pressure:kilopascals`.
impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let groups = menu();
        let group = groups.iter().find(|(_, units)| units.contains(self));
        write!(f, "{}:", group.map_or("", |(name, _)| name))?;
        (*self).visit_with(ShowUnit(f))
    }
}

/// And back: the menu entry that prints as the saved form.
impl FromStr for Units {
    type Err = UnknownUnit;
    fn from_str(s: &str) -> Result<Self, UnknownUnit> {
        menu()
            .into_iter()
            .flat_map(|(_, units)| units)
            .find(|unit| unit.to_string() == s)
            .ok_or(UnknownUnit)
    }
}

fn main() {
    // Menu 1: which quantity.
    let groups = menu();
    let quantities: Vec<_> = groups.iter().map(|(name, _)| *name).collect();
    println!("quantities: {quantities:?}");

    // The user picked pressure; menu 2 lists its units, by name.
    let (picked, units) = &groups[0];
    println!("picked:     {picked}");
    let some_units: Vec<_> = units.iter().take(4).map(|u| format!("{u:#}")).collect();
    println!("units:      {some_units:?} ...");

    // The setting round-trips through a string.
    let setting: Units = "Pressure:kPa".parse().unwrap();
    println!("saved:      {setting}");

    // A sensor reports 101.325 in the configured unit: the unit builds the
    // quantity, and any unit of the same quantity reads it.
    let p = setting.quantity(101.325);
    let bar: Units = "Pressure:bar".parse().unwrap();
    println!("in bar:     {:?}", bar.try_value(p));
    // A unit of another quantity can't read it.
    let square_metres: Units = "Area:m²".parse().unwrap();
    println!("in m²:      {:?}", square_metres.try_value(p));

    // Arithmetic stays inside the set: pressure × area is a force; pressure
    // × pressure is not in the set.
    let force = (p * square_metres.quantity(0.5)).unwrap();
    println!(
        "in N:       {:?}",
        Units::Force(ForceUnit::Newton).try_value(force)
    );
    println!("p × p:      {:?}", p * p);
    // Scaling cannot change the quantity, so it is never `None`.
    println!("in kPa:     {:?}", setting.try_value(p * 2.0));

    // A scale (a zero and a step) builds and reads like any other unit.
    let celsius: Units = "Temperature:°C".parse().unwrap();
    let kelvin = Units::Temperature(ThermodynamicTemperatureUnit::Kelvin);
    println!(
        "21.5 °C:    {:?} K",
        kelvin.try_value(celsius.quantity(21.5))
    );
}
