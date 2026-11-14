//! The open type: `DynamicSIQuantity` holds a quantity of *any* unit,
//! decided at run time - what a calculator, a parser or a config file needs.
//!
//! A static quantity (`Pressure<f64>`, ...) has its unit in its type, so the
//! compiler checks every operator. The open type keeps the unit in a field
//! instead, and checks at run time: any operator may have no result
//! (`None`): a sum of two units, a product with a unit that has a zero or a
//! log scale. Into it is std `From`; back out is std `TryFrom`, which checks the
//! unit and hands the value back on a mismatch.
//!
//! Numbers go in and out through units chosen at run time - the unit enums,
//! each variant an `SIUnit`: `to_base` counts a number of a unit in base
//! units, `from_base` back.
//!
//! Run with: cargo run -p yadc-si --example open
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::dimensionless::DimensionlessUnit;
use yadc_si::dynamic::mechanical::{AreaUnit, ForceUnit, PowerUnit, PressureUnit};
use yadc_si::dynamic::thermal::{TemperatureIntervalUnit, ThermodynamicTemperatureUnit};
use yadc_si::mechanical::{BAR, Length, PSI, Pressure};
use yadc_si::{BASE, SIUnit};

/// The product of base units a unit is counted in: kilometres' metres,
/// bars' kg·m⁻¹·s⁻².
fn base_of(unit: SIUnit) -> SIUnit {
    let (symbols, powers) = unit.in_base();
    symbols
        .iter()
        .zip(powers)
        .filter(|(_, p)| *p != 0)
        .fold(SIUnit::ONE, |u, (s, p)| u.times(SIUnit::base(s), p))
}

/// `n` of a unit chosen at run time, as an open quantity in base units.
fn open(unit: impl Into<SIUnit>, n: f64) -> DynamicSIQuantity {
    let unit = unit.into();
    DynamicSIQuantity {
        unit: base_of(unit),
        is_base: BASE,
        value: unit.to_base(n),
    }
}

/// The number of a unit an operator's result is: `None` for no result, or
/// for a quantity of another dimension.
fn read(unit: impl Into<SIUnit>, q: Option<DynamicSIQuantity>) -> Option<f64> {
    let unit = unit.into();
    q.filter(|q| q.unit.same_dimension(unit))
        .map(|q| unit.from_base(q.unit.to_base(q.value)))
}

/// Generic over any dynamic version of `T`: in, and back.
fn round_trip<T, X: From<T>>(t: T) -> Result<T, X>
where
    T: TryFrom<X, Error = X>,
{
    T::try_from(X::from(t))
}

fn main() {
    let bar = BAR.unit();
    let square_metre = AreaUnit::SquareMetre;
    let newton = ForceUnit::Newton;
    let p = open(bar, 1.01325);
    let a = open(square_metre, 0.5);

    // Arithmetic decided at run time: every operator may have no result. A
    // product of two linear units has one ...
    println!("p × a       = {:.1?} {newton}", read(newton, p * a));
    // ... `+` only for one unit; scaling keeps the unit.
    println!("p + a       = {:?}", p + a);
    println!("p × 2       = {:?} {bar}", read(bar, p * 2.0));
    // A unit of another dimension reads nothing.
    println!("p in m²     = {:?}", read(square_metre, Some(p)));
    // `?` stops a chain at the first step that has no result.
    let chain = || ((p * a)? + (p * a)?)? * 2.0;
    println!("(p×a + p×a)×2 = {:.1?} {newton}", read(newton, chain()));
    // A quantity kept in its own unit is not in base units, and neither is
    // its product with one that is.
    let in_bar = DynamicSIQuantity::from(1.01325_f64 * BAR);
    println!(
        "1.01325 bar = {} {}, in base units: {:?}",
        in_bar.value, in_bar.unit, in_bar.is_base
    );
    let force = (in_bar * a).unwrap();
    println!(
        "(p in bar) × a = {} {}, in base units: {:?}",
        force.value, force.unit, force.is_base
    );

    // A bare number is the same value as a quantity of the plain number's
    // unit, both ways.
    let ratio = (p / open(bar, 1.0)).unwrap();
    println!("p / 1 bar   = {:?}", f64::try_from(ratio));
    println!(
        "a × 0.5     = {:?} {square_metre}",
        read(square_metre, a * DynamicSIQuantity::from(0.5))
    );

    // The way back to a static quantity: std `TryFrom` checks the unit, and
    // a mismatch hands the value back.
    let back = Pressure::<f64>::try_from(p).unwrap();
    println!("back        = {back:.3}");
    println!("as a length = {:?}", Length::<f64>::try_from(p).is_ok());
    let kept = round_trip::<Pressure<f64>, DynamicSIQuantity>(back);
    println!(
        "round trip  = {:.3?} {}",
        read(PSI.unit(), kept.ok().map(DynamicSIQuantity::from)),
        PressureUnit::Psi
    );

    // A temperature in degrees Celsius counts from a zero: in the open type
    // it is in kelvin, where it averages, differs and scales as numbers do.
    let celsius = ThermodynamicTemperatureUnit::Celsius;
    let kelvin = TemperatureIntervalUnit::Kelvin;
    let t = open(celsius, 25.0);
    let ice = open(celsius, 0.0);
    println!(
        "(t + ice)/2 = {:.2?} {celsius}",
        read(celsius, (t + ice).and_then(|s| s / 2.0))
    );
    println!("t - ice     = {:?} {kelvin}", read(kelvin, t - ice));
    println!("t × 2       = {:.2?} {celsius}", read(celsius, t * 2.0));
    // Kept in degrees Celsius, it has no products.
    let in_celsius = DynamicSIQuantity::from(25.0_f64 * yadc_si::thermal::CELSIUS);
    println!("25 °C × 25 °C = {:?}", in_celsius * in_celsius);

    // A power level is a log unit of power, a gain a log unit of the plain
    // number: in base units, a gain multiplies a power.
    let dbm = PowerUnit::DecibelMilliwatt;
    let decibel = DimensionlessUnit::Decibel;
    let level = open(dbm, 30.0);
    let gain = open(decibel, 13.0);
    println!("30 dBm × 13 dB = {:.1?} {dbm}", read(dbm, level * gain));
    // Kept in dBm, it has no products.
    let in_dbm = DynamicSIQuantity::from(30.0_f64 * yadc_si::mechanical::DECIBEL_MILLIWATT);
    println!("30 dBm × 30 dBm = {:?}", in_dbm * in_dbm);

    // Values of one unit are ordered; of two, not.
    println!("p < 2p      = {:?}", p < (p * 2.0).unwrap());
    println!("p vs a      = {:?}", p.partial_cmp(&a));
}
