//! Units print and parse through std traits; a quantity's number type is a
//! generic parameter.
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use const_derive_more::NoValue;
use core::ops::{Add, Mul};
use yadc_si::dynamic::mechanical::{LengthUnit, MassUnit, TimeUnit};
use yadc_si::mechanical::HOUR;
use yadc_si::mechanical::{Area, KILOMETRE, Length, MILE, Time, Velocity};
use yadc_si::{Exact, METRE, NOT_BASE, SECOND, SIQuantity, SIUnit};

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

// --- Display and FromStr ------------------------------------------------------

#[test]
fn units_print_their_symbol() {
    // A unit enum prints as its unit; a unit as its symbol.
    assert_eq!(format!("{}", LengthUnit::Mile), "mi");
    assert_eq!(LengthUnit::Mile.to_string(), "mi");
    assert_eq!(MILE.unit().to_string(), "mi");
    assert_eq!(LengthUnit::Kilometre.to_string(), "km");
    assert_eq!(
        format!("{} / {}", MassUnit::Kilogram, TimeUnit::Second),
        "kg / s"
    );
    // A coherent unit with a name prints it; any other product its named
    // units with their powers.
    assert_eq!((METRE / SECOND).unit().to_string(), "m/s");
    assert_eq!((KILOMETRE / HOUR).unit().to_string(), "km/h");
    // A quantity shown in a unit: the number the unit reads, with its own
    // flags, then the unit.
    let d: Length<f64> = (1.5_f64 * KILOMETRE).into();
    let km = SIUnit::from(LengthUnit::Kilometre);
    let m = SIUnit::from(LengthUnit::Metre);
    let metres = f64::from(d / METRE);
    assert_eq!(format!("{:.2} {}", km.from_base(metres), km), "1.50 km");
    assert_eq!(format!("{} {}", m.from_base(metres), m), "1500 m");
    assert_eq!(format!("{:e} {}", m.from_base(metres), m), "1.5e3 m");
    // A quantity prints itself as its number and its unit.
    assert_eq!(format!("{d}"), "1500 m");
    assert_eq!(format!("{}", 1.5_f64 * KILOMETRE), "1.5 km");
}

#[test]
fn units_parse_from_their_symbol() {
    let km: LengthUnit = "km".parse().unwrap();
    assert_eq!(km, LengthUnit::Kilometre);
    assert_eq!(km.to_string(), "km");
    close(SIUnit::from(km).from_base(2500.0), 2.5);
    // Every unit round-trips through its symbol.
    for unit in [LengthUnit::Metre, LengthUnit::Mile, LengthUnit::Angstrom] {
        let back: LengthUnit = unit.to_string().parse().unwrap();
        assert_eq!(back, unit);
    }
    // An unknown symbol - or a long name - is an error.
    assert_eq!(
        "furlongs per fortnight".parse::<LengthUnit>().err(),
        Some(NoValue)
    );
    assert!("kilometres".parse::<LengthUnit>().is_err());
    assert!("kg".parse::<LengthUnit>().is_err());
    let _: Result<TimeUnit, NoValue> = "h".parse();
}

/// A unit of your own: a named unit, a size times a unit of the library's.
const LEAGUE: SIQuantity<
    { SIUnit::scaled("lea", Exact::int(4828), METRE.unit()) },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

// An enum of units may leave out the derived `FromStr` - `impl(Display)`
// writes only `Display` - and write its own: here one that also accepts the
// long names.
#[derive(Copy, Clone, Debug, PartialEq, const_derive_more::Values)]
#[values(into(SIUnit), impl(Display))]
enum Distance {
    #[value(METRE.unit())]
    Metre,
    #[value(LEAGUE.unit())]
    League,
}

impl core::str::FromStr for Distance {
    type Err = NoValue;
    fn from_str(s: &str) -> Result<Self, NoValue> {
        match s {
            "m" | "metre" | "metres" => Ok(Distance::Metre),
            "lea" | "league" | "leagues" => Ok(Distance::League),
            _ => Err(NoValue),
        }
    }
}

#[test]
fn a_hand_written_from_str() {
    assert_eq!("leagues".parse(), Ok(Distance::League));
    assert_eq!("m".parse(), Ok(Distance::Metre));
    assert_eq!("mile".parse::<Distance>(), Err(NoValue));
    assert_eq!(Distance::League.to_string(), "lea");
    let league = SIUnit::from(Distance::League);
    close(
        league.from_base(SIUnit::from(Distance::Metre).to_base(2414.0)),
        0.5,
    );
    // A unit of your own converts like the library's.
    let d: Length<f64> = (2.0_f64 * LEAGUE).into();
    close(f64::from(d / METRE), 9656.0);
    close(f64::from(d / LEAGUE), 2.0);
}

// --- A generic number type ------------------------------------------------------

/// One function for every number type: the area of a rectangle. Generic
/// code names the quantities' operators it uses.
fn rectangle<N>(a: Length<N>, b: Length<N>) -> Area<N>
where
    Length<N>: Mul<Output = Area<N>>,
{
    a * b
}

/// A sum of lengths, in any number type whose lengths add.
fn perimeter<N: Copy>(a: Length<N>, b: Length<N>) -> Length<N>
where
    Length<N>: Add<Output = Length<N>>,
{
    a + b + a + b
}

#[test]
fn a_generic_number_type() {
    let a32: Length<f32> = 3.0_f32 * METRE;
    let b32: Length<f32> = 4.0_f32 * METRE;
    let area32 = rectangle(a32, b32);
    assert_eq!(f32::from(area32 / (METRE * METRE)), 12.0);
    assert_eq!(perimeter(a32, b32), 14.0_f32 * METRE);

    let a64: Length<f64> = 1.5 * METRE;
    let t64: Time<f64> = 0.5 * SECOND;
    let speed: Velocity<f64> = a64 / t64;
    close(f64::from(speed / (METRE / SECOND)), 3.0);
    close(f64::from(a64 * 2.0 / METRE), 3.0);

    // Integers count too: a number times a unit, in that number type.
    let ai: Length<i32> = 3 * METRE;
    let bi: Length<i32> = 4 * METRE;
    assert_eq!(rectangle(ai, bi), 12 * METRE * METRE);
    assert_eq!(perimeter(ai, bi).to_string(), "14 m");
    // Conversion needs a number type the exact sizes convert to.
    let km: Length<i64> = (2_i64 * KILOMETRE).into();
    assert_eq!(km, 2000_i64 * METRE);
}
