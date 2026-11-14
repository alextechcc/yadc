//! Sets: your own enums of quantities and of units, chosen at run time. The
//! enum *is* a quantity set - a closed one, holding exactly the quantities it
//! lists. yadc-si gives the statics (quantities and unit constants) and the
//! unit enums; `const_derive_more`'s derives give the plain enum plumbing -
//! conversions (`Members`), arithmetic borrowed from the open type (`Via`),
//! case analysis on the enums of units (`Visit`), a walk over the variants
//! (`Variants`, for menus) and a unit enum's `SIUnit`, `Display` and
//! `FromStr` (`Values`). Which quantity a unit family measures is an
//! associated type of a trait written here, read by visitors written once
//! for every family; an enum of quantities is taken apart by a `match`
//! calling functions written once for every quantity.
#![feature(
    generic_const_exprs,
    adt_const_params,
    unsized_const_params,
    const_trait_impl
)]
#![feature(const_convert, const_default, const_destruct, const_ops, derive_const)]
#![allow(incomplete_features)]

use const_derive_more::NoValue;
use const_derive_more::{Members, VariantVisitor, Variants, Via, Visit, Visitor};
use core::fmt;
use core::str::FromStr;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::mechanical::{AreaUnit, ForceUnit, MassUnit};
use yadc_si::dynamic::thermal::ThermodynamicTemperatureUnit;
use yadc_si::mechanical::{BAR, KILOMETRE, PASCAL};
use yadc_si::{BASE, Exact, KILO, KILOGRAM, METRE, NOT_BASE, SIQuantity, SIUnit};

/// The app's own unit: the kilopascal, a thousand pascals.
const KILOPASCAL: SIQuantity<{ KILO.unit().times(PASCAL.unit(), 1) }, Exact, NOT_BASE> =
    KILO * PASCAL;

/// The units of pressure the app offers: a family of your own, a unit of
/// its own among them. The first is the default.
#[derive_const(Default)]
#[derive(Clone, Copy, PartialEq, Debug, const_derive_more::Values, Variants)]
#[values(into(SIUnit))]
enum PressureUnit {
    #[default]
    #[value(PASCAL.unit())]
    Pascal,
    #[value(KILOPASCAL.unit())]
    Kilopascal,
    #[value(BAR.unit())]
    Bar,
}

/// The units of length the app offers.
#[derive_const(Default)]
#[derive(Clone, Copy, PartialEq, Debug, const_derive_more::Values, Variants)]
#[values(into(SIUnit))]
enum LengthUnit {
    #[default]
    #[value(METRE.unit())]
    Metre,
    #[value(KILOMETRE.unit())]
    Kilometre,
}

/// The quantities the sets hold, in base units.
type Pressure = yadc_si::mechanical::Pressure<f64>;
type Temperature = yadc_si::thermal::ThermodynamicTemperature<f64>;
type Length = yadc_si::mechanical::Length<f64>;
type Area = yadc_si::mechanical::Area<f64>;
type Force = yadc_si::mechanical::Force<f64>;

/// What the app measures. Its arithmetic runs through the open type.
#[derive(Clone, Copy, PartialEq, Debug, Members, Via)]
#[members(convert(DynamicSIQuantity))]
#[via(DynamicSIQuantity, Add, Sub, Mul, Div, Mul<f64>, Div<f64>)]
enum Quantities {
    Pressure(Pressure),
    Temperature(Temperature),
    Length(Length),
    Area(Area),
    Force(Force),
}

/// The setting: which quantity, shown in which unit. A menu of menus: the
/// quantities, then the units of the chosen one.
#[derive(Clone, Copy, PartialEq, Debug, Visit, Variants)]
enum Units {
    Pressure(PressureUnit),
    Temperature(ThermodynamicTemperatureUnit),
    Length(LengthUnit),
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

/// A number of `unit` as its quantity.
fn quantity<Q: InBase>(unit: impl Into<SIUnit>, n: f64) -> Q {
    Q::of(unit.into().to_base(n))
}

/// The number of `unit` a quantity is.
fn value<Q: InBase>(unit: impl Into<SIUnit>, q: Q) -> f64 {
    unit.into().from_base(q.base_units())
}

/// The relation between the two sets: the quantity a unit family measures.
trait Measures: Copy + Into<SIUnit> {
    type Quantity: InBase;
}
impl Measures for PressureUnit {
    type Quantity = Pressure;
}
impl Measures for ThermodynamicTemperatureUnit {
    type Quantity = Temperature;
}
impl Measures for LengthUnit {
    type Quantity = Length;
}
impl Measures for AreaUnit {
    type Quantity = Area;
}
impl Measures for ForceUnit {
    type Quantity = Force;
}

/// Build: a number in a unit is that unit's quantity, put in the set.
struct Build(f64);
impl<U: Measures> Visitor<U> for Build
where
    Quantities: From<U::Quantity>,
{
    type Output = Quantities;
    fn visit(self, u: U) -> Quantities {
        Quantities::from(quantity::<U::Quantity>(u, self.0))
    }
}

/// Read: the number a quantity is in a unit, when it is that unit's
/// quantity; the set handed back otherwise.
struct Read(Quantities);
impl<U: Measures> Visitor<U> for Read
where
    U::Quantity: TryFrom<Quantities, Error = Quantities>,
{
    type Output = Result<f64, Quantities>;
    fn visit(self, u: U) -> Result<f64, Quantities> {
        Ok(value(u, U::Quantity::try_from(self.0)?))
    }
}

impl Units {
    fn quantity(self, v: f64) -> Quantities {
        self.visit_with(Build(v))
    }

    /// The set hands itself back when the unit measures another quantity.
    fn try_value(self, q: Quantities) -> Result<f64, Quantities> {
        self.visit_with(Read(q))
    }

    /// A reading: the unit's quantity, the unit kept. Two enums of the same
    /// shape: a plain `match`.
    fn reading(self, v: f64) -> Measurement {
        match self {
            Units::Pressure(u) => Measurement::Pressure(quantity(u, v), u),
            Units::Temperature(u) => Measurement::Temperature(quantity(u, v), u),
            Units::Length(u) => Measurement::Length(quantity(u, v), u),
            Units::Area(u) => Measurement::Area(quantity(u, v), u),
            Units::Force(u) => Measurement::Force(quantity(u, v), u),
        }
    }
}

/// A flat enum's choices: every variant without contents, built.
struct Choices<E>(Vec<E>);
impl<E> VariantVisitor<E, ()> for Choices<E> {
    fn visit(&mut self, _: &'static str, make: fn(()) -> E) {
        self.0.push(make(()));
    }
}
fn choices<E: Variants<Choices<E>>>() -> Vec<E> {
    let mut all = Choices(Vec::new());
    E::each_variant(&mut all);
    all.0
}

/// An enum of enums' groups: each variant's name, and the choices of the
/// enum it wraps, wrapped.
struct Groups<E>(Vec<(&'static str, Vec<E>)>);
impl<E, T: Variants<Choices<T>>> VariantVisitor<E, T> for Groups<E> {
    fn visit(&mut self, name: &'static str, make: fn(T) -> E) {
        self.0
            .push((name, choices::<T>().into_iter().map(make).collect()));
    }
}
fn menu() -> Vec<(&'static str, Vec<Units>)> {
    let mut groups = Groups(Vec::new());
    Units::each_variant(&mut groups);
    groups.0
}

/// Print whichever unit it is, through its own `Display`.
struct ShowUnit<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl<U: fmt::Display> Visitor<U> for ShowUnit<'_, '_> {
    type Output = fmt::Result;
    fn visit(self, u: U) -> fmt::Result {
        fmt::Display::fmt(&u, self.0)
    }
}

impl Units {
    /// The name of this unit's group in the menu.
    fn group(&self) -> &'static str {
        menu()
            .into_iter()
            .find(|(_, units)| units.contains(self))
            .map_or("", |(name, _)| name)
    }
}

/// Saved as `Group:symbol` (`Pressure:kPa`).
impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.group())?;
        (*self).visit_with(ShowUnit(f))
    }
}

/// Parsed from `Group:symbol`: the entry of the menus that prints as it.
impl FromStr for Units {
    type Err = NoValue;
    fn from_str(s: &str) -> Result<Units, NoValue> {
        menu()
            .into_iter()
            .flat_map(|(_, units)| units)
            .find(|u| u.to_string() == s)
            .ok_or(NoValue)
    }
}

/// A value that keeps its unit: the quantity is the member, the unit rides
/// along. Built from a bare
/// quantity, the unit is its family's default (`#[members(default)]`);
/// arithmetic whose result is the left operand's quantity keeps the left
/// operand's unit (`#[via(keep)]`).
#[derive(Clone, Copy, Debug, Members, Via)]
#[members(convert(Quantities, DynamicSIQuantity))]
#[via(DynamicSIQuantity, Add, Sub, Mul, Div, Mul<f64>, Div<f64>)]
enum Measurement {
    Pressure(
        #[members(member)] Pressure,
        #[members(default)]
        #[via(keep)]
        PressureUnit,
    ),
    Temperature(
        #[members(member)] Temperature,
        #[members(default)]
        #[via(keep)]
        ThermodynamicTemperatureUnit,
    ),
    Length(
        #[members(member)] Length,
        #[members(default)]
        #[via(keep)]
        LengthUnit,
    ),
    Area(
        #[members(member)] Area,
        #[members(default)]
        #[via(keep)]
        AreaUnit,
    ),
    Force(
        #[members(member)] Force,
        #[members(default)]
        #[via(keep)]
        ForceUnit,
    ),
}

/// Struct variants with fields of your own; the marker says which field is
/// the member. Not a `Copy` type.
#[derive(Clone, Debug, PartialEq, Members, Via)]
#[members(convert(DynamicSIQuantity))]
#[via(DynamicSIQuantity, Add, Sub, Mul, Div, Mul<f64>, Div<f64>)]
enum Channel {
    Pressure {
        #[members(default)]
        label: &'static str,
        #[members(member)]
        reading: Pressure,
        #[members(default)]
        shown_in: PressureUnit,
    },
    Temperature {
        #[members(default)]
        label: &'static str,
        #[members(member)]
        reading: Temperature,
        #[members(default)]
        shown_in: ThermodynamicTemperatureUnit,
    },
}

// --- Behaviour on the carried unit: functions, written once ---------------------

/// Print a quantity in a unit: the number with the caller's flags, then the
/// unit.
fn show(
    f: &mut fmt::Formatter<'_>,
    q: impl InBase,
    u: impl Into<SIUnit> + fmt::Display + Copy,
) -> fmt::Result {
    fmt::Display::fmt(&value(u, q), f)?;
    write!(f, " {u}")
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Measurement::Pressure(q, u) => show(f, q, u),
            Measurement::Temperature(q, u) => show(f, q, u),
            Measurement::Length(q, u) => show(f, q, u),
            Measurement::Area(q, u) => show(f, q, u),
            Measurement::Force(q, u) => show(f, q, u),
        }
    }
}

impl Measurement {
    /// The number in the carried unit: the unit reads it.
    fn in_its_unit(self) -> f64 {
        match self {
            Measurement::Pressure(q, u) => value(u, q),
            Measurement::Temperature(q, u) => value(u, q),
            Measurement::Length(q, u) => value(u, q),
            Measurement::Area(q, u) => value(u, q),
            Measurement::Force(q, u) => value(u, q),
        }
    }
}

impl Channel {
    /// ... and a channel's, only lent.
    fn in_its_unit(&self) -> f64 {
        match self {
            Channel::Pressure {
                reading, shown_in, ..
            } => value(*shown_in, *reading),
            Channel::Temperature {
                reading, shown_in, ..
            } => value(*shown_in, *reading),
        }
    }
}

/// A quantity's unit, the const parameter of its static type.
fn unit_of<const U: SIUnit>(_: SIQuantity<U, f64, BASE>) -> SIUnit {
    U
}

impl Quantities {
    /// The member's unit, at its own static type.
    fn unit(self) -> SIUnit {
        match self {
            Quantities::Pressure(q) => unit_of(q),
            Quantities::Temperature(q) => unit_of(q),
            Quantities::Length(q) => unit_of(q),
            Quantities::Area(q) => unit_of(q),
            Quantities::Force(q) => unit_of(q),
        }
    }
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

#[test]
fn a_setting_saves_and_loads_as_text() {
    let unit: Units = "Pressure:kPa".parse().unwrap();
    assert_eq!(unit, Units::Pressure(PressureUnit::Kilopascal));
    assert_eq!(unit.to_string(), "Pressure:kPa");
    assert_eq!(unit.group(), "Pressure");
    assert_eq!(
        "Force:N".parse::<Units>().unwrap(),
        Units::Force(ForceUnit::Newton)
    );
    assert!("Pressure:kg".parse::<Units>().is_err());
    assert!("Mass:kg".parse::<Units>().is_err());
    assert!("kPa".parse::<Units>().is_err());
}

#[test]
fn menus_list_quantities_then_their_units() {
    // Menu 1: one group per quantity, named by its variant.
    let groups = menu();
    let names: Vec<_> = groups.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        ["Pressure", "Temperature", "Length", "Area", "Force"]
    );
    // Menu 2: the units of the chosen quantity - the family's own choices,
    // wrapped.
    let (_, pressure) = &groups[0];
    assert_eq!(pressure.len(), choices::<PressureUnit>().len());
    assert!(pressure.contains(&Units::Pressure(PressureUnit::Kilopascal)));
    assert!(pressure.iter().all(|u| u.group() == "Pressure"));
    // A unit family's choices are flat, in declaration order.
    assert!(choices::<MassUnit>().contains(&MassUnit::Kilogram));
}

#[test]
fn units_convert_through_the_set() {
    let kpa: Units = "Pressure:kPa".parse().unwrap();
    let bar: Units = "Pressure:bar".parse().unwrap();
    let q: Quantities = kpa.quantity(101.325);
    assert!(matches!(q, Quantities::Pressure(_)));
    close(bar.try_value(q).unwrap(), 1.01325);
    // A mismatched pair hands the quantity back.
    let metres: Units = "Length:m".parse().unwrap();
    assert_eq!(metres.try_value(q), Err(q));
    // Units with a zero too.
    let celsius: Units = "Temperature:°C".parse().unwrap();
    let kelvin: Units = "Temperature:K".parse().unwrap();
    let t: Quantities = celsius.quantity(20.0);
    close(kelvin.try_value(t).unwrap(), 293.15);
}

#[test]
fn the_set_bridges_to_static_quantities() {
    let p: Pressure = quantity(PressureUnit::Bar, 2.0);
    let q = Quantities::from(p);
    assert_eq!(Pressure::try_from(q), Ok(p));
    assert_eq!(Length::try_from(q), Err(q));
    // A visitor sees the member at its own static type.
    assert_eq!(q.unit(), unit_of(p));
    assert_eq!(q.unit(), PASCAL.unit());
    let units: Vec<SIUnit> = [q, Quantities::from(1.0 * METRE)]
        .into_iter()
        .map(Quantities::unit)
        .collect();
    assert_eq!(units, [PASCAL.unit(), METRE.unit()]);
}

#[test]
fn arithmetic_stays_inside_the_set() {
    let p = Quantities::from(quantity::<Pressure>(PressureUnit::Pascal, 10.0));
    let a = Quantities::from(quantity::<Area>(AreaUnit::SquareMetre, 2.0));
    let l = Quantities::from(quantity::<Length>(LengthUnit::Metre, 3.0));
    // Pressure × area is a force, which the set lists.
    let f = (p * a).unwrap();
    assert!(matches!(f, Quantities::Force(_)));
    close(Units::Force(ForceUnit::Newton).try_value(f).unwrap(), 20.0);
    // Length × length is an area; area / length a length.
    assert!(matches!(l * l, Some(Quantities::Area(_))));
    assert!(matches!(a / l, Some(Quantities::Length(_))));
    // Pressure × length has no variant: None.
    assert_eq!(p * l, None);
    // Adding mismatched quantities: None.
    assert_eq!(p + l, None);
    // Two temperatures add and subtract, in kelvin; their product with a
    // length, a kelvin-metre, is not in the set: None.
    let t: Quantities = Units::Temperature(ThermodynamicTemperatureUnit::Kelvin).quantity(300.0);
    assert!(matches!(t + t, Some(Quantities::Temperature(_))));
    assert!(matches!(t - t, Some(Quantities::Temperature(_))));
    assert_eq!(t * l, None);
    // Scaling by a number is std `*` and `/`; it cannot change the variant,
    // so it is never `None`.
    let pascals = Units::Pressure(PressureUnit::Pascal);
    close(pascals.try_value(p * 2.0).unwrap(), 20.0);
    close(pascals.try_value(p / 4.0).unwrap(), 2.5);
    let kelvins = Units::Temperature(ThermodynamicTemperatureUnit::Kelvin);
    close(kelvins.try_value(t * 2.0).unwrap(), 600.0);
}

#[test]
fn a_measurement_keeps_its_unit() {
    let kpa: Units = "Pressure:kPa".parse().unwrap();
    // The unit builds a reading shown in itself.
    let m: Measurement = kpa.reading(101.325);
    assert!(matches!(
        m,
        Measurement::Pressure(_, PressureUnit::Kilopascal)
    ));
    assert_eq!(m.to_string(), "101.325 kPa");
    assert_eq!(format!("{m:.1}"), "101.3 kPa");
    close(m.in_its_unit(), 101.325);
    // Another unit of the quantity reads it - as a number, or as a reading
    // shown in that unit.
    let bar: Units = "Pressure:bar".parse().unwrap();
    close(bar.try_value(m.into()).unwrap(), 1.01325);
    let in_bar: Measurement = bar.reading(bar.try_value(m.into()).unwrap());
    assert_eq!(format!("{in_bar:.5}"), "1.01325 bar");
    // A unit of another quantity hands the reading back.
    let metres: Units = "Length:m".parse().unwrap();
    assert!(matches!(
        metres.try_value(m.into()),
        Err(Quantities::Pressure(_))
    ));
    // Into the plain set, for storage or arithmetic.
    assert!(matches!(Quantities::from(m), Quantities::Pressure(_)));
    // Arithmetic that keeps the quantity keeps the left operand's unit.
    assert_eq!((m + m).unwrap().to_string(), "202.65 kPa");
    assert_eq!((m * 2.0).to_string(), "202.65 kPa");
    assert_eq!((m / 2.0).to_string(), "50.6625 kPa");
    let sum = (m + in_bar).unwrap();
    assert!(matches!(
        sum,
        Measurement::Pressure(_, PressureUnit::Kilopascal)
    ));
    close(sum.in_its_unit(), 202.65);
    let sum_in_bar = (in_bar + m).unwrap();
    assert!(matches!(
        sum_in_bar,
        Measurement::Pressure(_, PressureUnit::Bar)
    ));
    // A new quantity takes its family's default unit: the base one.
    let a: Measurement = Units::Area(AreaUnit::SquareMetre).reading(2.0);
    let pa: Measurement = Units::Pressure(PressureUnit::Pascal).reading(10.0);
    let f = (pa * a).unwrap();
    assert!(matches!(f, Measurement::Force(_, ForceUnit::Newton)));
    assert_eq!(f.to_string(), "20 N");
    close(
        Units::Force(ForceUnit::Newton).try_value(f.into()).unwrap(),
        20.0,
    );
    // Pressure × pressure is not in the set; a temperature scales and keeps
    // its unit.
    assert!((m * m).is_none());
    let t: Measurement = Units::Temperature(ThermodynamicTemperatureUnit::Kelvin).reading(300.0);
    assert_eq!((t * 2.0).to_string(), "600 K");
}

#[test]
fn a_measurement_converts_through_its_members() {
    // From a bare quantity: the family's default unit.
    let p: Pressure = quantity(PressureUnit::Bar, 1.0);
    let m = Measurement::from(p);
    assert!(matches!(m, Measurement::Pressure(_, PressureUnit::Pascal)));
    assert_eq!(m.to_string(), "100000 Pa");
    assert_eq!(Pressure::try_from(m).ok(), Some(p));
    assert!(Length::try_from(m).is_err());
    // To and from the plain set and the open type.
    let kpa: Measurement = Units::Pressure(PressureUnit::Kilopascal).reading(100.0);
    let q = Quantities::from(kpa);
    assert_eq!(q, Quantities::Pressure(p));
    assert!(matches!(
        Measurement::try_from(q),
        Ok(Measurement::Pressure(_, PressureUnit::Pascal))
    ));
    let open = DynamicSIQuantity::from(kpa);
    assert_eq!(Pressure::try_from(open), Ok(p));
    assert_eq!(
        Measurement::try_from(open).unwrap().to_string(),
        "100000 Pa"
    );
    let mass = DynamicSIQuantity::from(1.0 * KILOGRAM);
    assert_eq!(Measurement::try_from(mass).err(), Some(mass));
    // The reading's unit is its quantity's base unit.
    assert_eq!(Quantities::from(kpa).unit(), unit_of(p));
    assert_eq!(Quantities::from(kpa).unit(), PASCAL.unit());
}

#[test]
fn struct_variants_carry_your_own_fields() {
    let c = Channel::Pressure {
        label: "inlet",
        reading: quantity(PressureUnit::Bar, 2.0),
        shown_in: PressureUnit::Kilopascal,
    };
    // Read by reference: the channel is only lent.
    close(c.in_its_unit(), 200.0);
    assert_eq!(
        Pressure::try_from(c.clone()),
        Ok(quantity(PressureUnit::Bar, 2.0))
    );
    // Scaling keeps the unit and the label.
    let half = c.clone() * 0.5;
    let Channel::Pressure {
        label, shown_in, ..
    } = &half
    else {
        panic!()
    };
    assert_eq!((*label, *shown_in), ("inlet", PressureUnit::Kilopascal));
    close(half.in_its_unit(), 100.0);
    // From a bare quantity - the `#[members(member)]` field: the
    // `#[members(default)]` fields take their `Default`s.
    let t = Channel::from(quantity::<Temperature>(
        ThermodynamicTemperatureUnit::Kelvin,
        300.0,
    ));
    let Channel::Temperature {
        label, shown_in, ..
    } = &t
    else {
        panic!()
    };
    assert_eq!(*label, "");
    assert_eq!(*shown_in, ThermodynamicTemperatureUnit::default());
    assert_eq!(Pressure::try_from(t.clone()), Err(t));
    assert_eq!(Pressure::try_from(c), Ok(quantity(PressureUnit::Bar, 2.0)));
}

#[test]
fn static_closed_and_open_sets_read_the_same_way() {
    // The same generic code reads a static quantity (a set of one: `TryFrom`
    // to itself cannot fail), a closed set or a reading: give up the unit's
    // quantity with `TryFrom`, then the unit reads it.
    fn read<S, Q: TryFrom<S> + InBase>(u: impl Into<SIUnit>, s: S) -> Option<f64> {
        Q::try_from(s).ok().map(|q| value(u, q))
    }
    let p: Pressure = quantity(PressureUnit::Bar, 1.0);
    close(
        read::<_, Pressure>(PressureUnit::Kilopascal, p).unwrap(),
        100.0,
    );
    let closed: Quantities = Units::Pressure(PressureUnit::Bar).quantity(1.0);
    close(
        read::<_, Pressure>(PressureUnit::Kilopascal, closed).unwrap(),
        100.0,
    );
    let reading: Measurement = Units::Pressure(PressureUnit::Bar).reading(1.0);
    close(
        read::<_, Pressure>(PressureUnit::Kilopascal, reading).unwrap(),
        100.0,
    );
    // A unit of another quantity: no reading.
    assert_eq!(read::<_, Length>(LengthUnit::Metre, closed), None);
    // Std `From` / `TryFrom` convert between them: closed to open, open back
    // to closed or to static, an error where the target has no such member.
    let open = DynamicSIQuantity::from(closed);
    assert_eq!(Quantities::try_from(open).ok(), Some(closed));
    assert_eq!(Pressure::try_from(open).ok(), Some(p));
    assert!(Length::try_from(open).is_err());
    let mass = DynamicSIQuantity::from(1.0 * KILOGRAM);
    assert!(Quantities::try_from(mass).is_err());
}

/// Generic code over any closed set that borrows the open type's
/// arithmetic names the std operator.
fn total<S: core::ops::Add<Output = Option<S>>>(a: S, b: S) -> Option<S> {
    a + b
}

#[test]
fn generic_code_over_sets() {
    let a = Quantities::from(quantity::<Length>(LengthUnit::Metre, 1.0));
    let b = Quantities::from(quantity::<Length>(LengthUnit::Kilometre, 1.0));
    let sum = total(a, b).unwrap();
    close(
        Units::Length(LengthUnit::Metre).try_value(sum).unwrap(),
        1001.0,
    );
    let reading = Units::Length(LengthUnit::Kilometre).reading(1.0);
    assert_eq!(total(reading, reading).unwrap().to_string(), "2 km");
}
