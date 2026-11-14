//! A settings menu where the user picks a quantity and the unit to show it
//! in: a closed set of quantities, and the set of their units beside it.
//!
//! `Quantities` is an enum of yours - the quantities this app measures.
//! `Units` is another enum of yours: for each quantity, its family of units.
//! Which quantity a unit family measures is a fact about the types, written
//! as one: `impl Measures for PressureUnit { type Quantity = Pressure<f64>; }`.
//! const_derive_more gives the plumbing:
//!
//! - `Variants` walks the choices: one group per quantity, then the units
//!   of each - the two menus a UI would render;
//! - `Visit` hands the chosen unit family to a generic visitor: one builds
//!   the family's quantity from a number, one reads a quantity if it is the
//!   family's;
//! - `Values` makes each unit enum its variant's `SIUnit`: printed and
//!   parsed as its symbol, and converting numbers to and from base units;
//! - `Members` and `Via` convert the set through the open type,
//!   `DynamicSIQuantity`, and borrow its arithmetic.
//!
//! Run with: cargo run -p yadc-si --example sets
#![feature(
    generic_const_exprs,
    adt_const_params,
    unsized_const_params,
    const_trait_impl
)]
#![feature(const_convert, const_destruct, const_ops)]
#![allow(incomplete_features)]

use const_derive_more::NoValue;
use const_derive_more::{Members, VariantVisitor, Variants, Via, Visit, Visitor};
use core::fmt;
use core::str::FromStr;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::mechanical::{AreaUnit, ForceUnit};
use yadc_si::dynamic::thermal::ThermodynamicTemperatureUnit;
use yadc_si::mechanical::{Area, BAR, Force, PASCAL, PSI, Pressure, STANDARD_ATMOSPHERE};
use yadc_si::thermal::ThermodynamicTemperature;
use yadc_si::{BASE, Exact, KILO, NOT_BASE, SIQuantity, SIUnit};

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

/// The app's own unit: a kilopascal, a thousand pascals.
const KILOPASCAL: SIQuantity<{ KILO.unit().times(PASCAL.unit(), 1) }, Exact, NOT_BASE> =
    KILO * PASCAL;

/// The units of pressure the app offers: a family of your own, a unit of
/// its own among them.
#[derive(Clone, Copy, PartialEq, Debug, const_derive_more::Values, Variants)]
#[values(into(SIUnit))]
enum PressureUnit {
    #[value(PASCAL.unit())]
    Pascal,
    #[value(KILOPASCAL.unit())]
    Kilopascal,
    #[value(BAR.unit())]
    Bar,
    #[value(STANDARD_ATMOSPHERE.unit())]
    StandardAtmosphere,
    #[value(PSI.unit())]
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

/// A quantity in base units, for code generic over its unit: built from and
/// read as a number of base units, through the open type.
trait InBase: Sized {
    fn of(base_units: f64) -> Self;
    fn base_units(self) -> f64;
}

impl<const U: SIUnit> InBase for SIQuantity<U, f64, BASE> {
    fn of(base_units: f64) -> Self {
        let open = DynamicSIQuantity {
            unit: U,
            is_base: BASE,
            value: base_units,
        };
        Self::try_from(open).unwrap()
    }
    fn base_units(self) -> f64 {
        DynamicSIQuantity::from(self).value
    }
}

/// The relation between the two sets: the quantity a unit family measures.
trait Measures: Into<SIUnit> {
    type Quantity: InBase;
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
        Quantities::from(U::Quantity::of(unit.into().to_base(self.0)))
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
        Some(unit.into().from_base(quantity.base_units()))
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

/// Print whichever unit it is through its own `Display`.
struct ShowUnit<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl<U: fmt::Display> Visitor<U> for ShowUnit<'_, '_> {
    type Output = fmt::Result;
    fn visit(self, unit: U) -> fmt::Result {
        fmt::Display::fmt(&unit, self.0)
    }
}

/// The saved form: `Pressure:kPa`.
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
    type Err = NoValue;
    fn from_str(s: &str) -> Result<Self, NoValue> {
        menu()
            .into_iter()
            .flat_map(|(_, units)| units)
            .find(|unit| unit.to_string() == s)
            .ok_or(NoValue)
    }
}

fn main() {
    // Menu 1: which quantity.
    let groups = menu();
    let quantities: Vec<_> = groups.iter().map(|(name, _)| *name).collect();
    println!("quantities: {quantities:?}");

    // The user picked pressure; menu 2 lists its units.
    let (picked, units) = &groups[0];
    println!("picked:     {picked}");
    let some_units: Vec<_> = units.iter().take(4).map(|u| u.to_string()).collect();
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
    let square_metres: Units = "Area:m^2".parse().unwrap();
    println!("in m^2:     {:?}", square_metres.try_value(p));

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

    // A unit with a zero builds and reads like any other.
    let celsius: Units = "Temperature:°C".parse().unwrap();
    let kelvin = Units::Temperature(ThermodynamicTemperatureUnit::Kelvin);
    println!(
        "21.5 °C:    {:?} K",
        kelvin.try_value(celsius.quantity(21.5))
    );
}
