//! The open type: `DynamicSIQuantity` holds a quantity of *any* dimension,
//! decided at run time - what a calculator, a parser or a config file needs.
//!
//! A static quantity (`Pressure`, `Area`, ...) has its dimension in its type,
//! so the compiler checks every operator. The open type keeps the dimension
//! in a field instead, and checks at run time: any operator may have no
//! result (`None`) - a sum of two dimensions, a product with a level, a root
//! that does not divide. Into
//! it is std `From`; back out is std `TryFrom`, which checks the dimension
//! and hands the value back on a mismatch. Numbers still go in and out
//! through units, by way of a static quantity: a constant is called on a number
//! to build (`KILOPASCAL(101.325)`, with `KILOPASCAL` being `KILO * PASCAL`
//! rounded to `f64`), and on a quantity to read.
//!
//! Run with: cargo run -p yadc-si --example open
#![feature(generic_const_exprs, const_trait_impl, const_convert, const_ops)]
#![allow(incomplete_features)]

use yadc::{PartialPowi, PartialRoot, Unit};
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::mechanical::{
    Area, Force, Gain, Instant, Length, PowerLevel, Pressure, Time, Volume, consts,
};
use yadc_si::thermal::{TemperatureInterval, ThermodynamicTemperature};
use yadc_si::{KILO, Scale};

const KILOPASCAL: Pressure<f64> = (KILO * consts::PASCAL).into();
const BAR: Pressure<f64> = consts::BAR.into();
const PSI: Pressure<f64> = consts::PSI.into();
const NEWTON: Force<f64> = consts::NEWTON.into();
const METRE: Length<f64> = consts::METRE.into();
const SQUARE_METRE: Area<f64> = consts::SQUARE_METRE.into();
const CUBIC_METRE: Volume<f64> = consts::CUBIC_METRE.into();
const HOUR: Time<f64> = consts::HOUR.into();
const DECIBEL: Gain<f64> = consts::DECIBEL.into();
const KELVIN: TemperatureInterval<f64> = yadc_si::thermal::consts::KELVIN.into();
const CELSIUS: Scale<ThermodynamicTemperature<f64>, TemperatureInterval<f64>> =
    yadc_si::thermal::consts::CELSIUS.into();
const DBM: Scale<PowerLevel<f64>, Gain<f64>> = consts::DBM.into();
const UNIX_SECONDS: Scale<Instant<f64>, Time<f64>> = consts::UNIX_SECONDS.into();

/// Generic over any dynamic version of `T`: in, and back.
fn round_trip<T, X: From<T>>(t: T) -> Result<T, X>
where
    T: TryFrom<X, Error = X>,
{
    T::try_from(X::from(t))
}

fn main() {
    let p = DynamicSIQuantity::from(KILOPASCAL(101.325));
    let a = DynamicSIQuantity::from(SQUARE_METRE(0.5));

    // Numbers come out through a static quantity and a unit, checked.
    let kpa = |x| Some(KILOPASCAL(Pressure::try_from(x).ok()?));
    let m2 = |x| Some(SQUARE_METRE(Area::try_from(x).ok()?));

    // Arithmetic decided at run time: every operator may have no result. An
    // amount times an amount has one ...
    let f = (p * a).unwrap();
    let newtons = Force::try_from(f).map(NEWTON);
    println!("p × a       = {newtons:.1?} N");
    // ... `+` only for one dimension; scaling keeps the dimension.
    println!("p + a       = {:?}", p + a);
    println!("p × 2       = {:?} kPa", (p * 2.0).and_then(kpa));
    // `?` stops a chain at the first step that has no result.
    let chain = || ((p * a)? + f)? * 2.0;
    let newtons = |x| Some(NEWTON(Force::try_from(x).ok()?));
    println!("(p×a + f)×2 = {:.1?} N", chain().and_then(newtons));

    // Powers and roots with an exponent known only at run time: a root
    // exists only where every exponent divides.
    let side = a.partial_root(2).unwrap();
    let metres = Length::try_from(side).map(METRE);
    println!("√a          = {metres:.4?} m");
    println!("√√a         = {:?}", side.partial_root(2));
    println!("a³ exists   = {:?}", a.partial_powi(3).is_some());
    let m3 = Volume::try_from((a * side).unwrap()).map(CUBIC_METRE);
    println!("a × √a      = {m3:.4?} m³");

    // A bare number is the same value as a dimensionless quantity, both
    // ways.
    let ratio = (p / DynamicSIQuantity::from(BAR(1.0))).unwrap();
    println!("p / 1 bar   = {:?}", f64::try_from(ratio));
    println!(
        "a × 0.5     = {:?} m²",
        (a * DynamicSIQuantity::from(0.5)).and_then(m2)
    );

    // The way back: std `TryFrom` checks the dimension, and a mismatch hands
    // the value back.
    let back: Pressure<f64> = p.try_into().unwrap();
    println!("back        = {:.3} kPa", KILOPASCAL(back));
    println!("as a length = {:?}", Length::try_from(p).is_ok());
    let kept = round_trip::<Pressure<f64>, DynamicSIQuantity>(back);
    println!("round trip  = {:.3?} psi", kept.map(PSI));
    // A temperature is an amount kept apart from its differences: it scales
    // and averages as a temperature, and two differ by an interval.
    let t = DynamicSIQuantity::from(CELSIUS.quantity(25.0));
    let ice = DynamicSIQuantity::from(CELSIUS.quantity(0.0));
    let celsius = |x| Some(CELSIUS.value(ThermodynamicTemperature::try_from(x).ok()?));
    let kelvins = |x| Some(KELVIN(TemperatureInterval::try_from(x).ok()?));
    println!(
        "(t + ice)/2 = {:.2?} °C",
        ((t + ice).and_then(|s| s / 2.0)).and_then(celsius)
    );
    println!("t - ice     = {:?} K", (t - ice).and_then(kelvins));
    println!("t × 2       = {:.2?} °C", (t * 2.0).and_then(celsius));
    // A power level is a logarithm: a gain moves it, and it has no products
    // and cannot be scaled.
    let level = DynamicSIQuantity::from(DBM.quantity(30.0));
    let gain = DynamicSIQuantity::from(DECIBEL(13.0));
    let dbm = |x| Some(DBM.value(PowerLevel::try_from(x).ok()?));
    println!("30 dBm+13dB = {:.1?} dBm", (level + gain).and_then(dbm));
    println!("level × a   = {:?}", level * a);
    println!("level × 2   = {:?}", level * 2.0);
    // An instant is a point: two differ by a time, a time moves one, and it
    // has no sum with another, no products and no scaling.
    let launch = DynamicSIQuantity::from(UNIX_SECONDS.quantity(1_700_000_000.0));
    let later = (launch + DynamicSIQuantity::from(HOUR(2.0))).unwrap();
    let hours = |x| Some(HOUR(Time::try_from(x).ok()?));
    let unix = |x| Some(UNIX_SECONDS.value(Instant::try_from(x).ok()?));
    println!("later       = {:?} s Unix", unix(later));
    println!("later - t0  = {:?} h", (later - launch).and_then(hours));
    println!("t0 + later  = {:?}", launch + later);
    println!("t0 × 2      = {:?}", launch * 2.0);
    // Values of one dimension are ordered; of two, not.
    println!("p < 2p      = {:?}", p < (p * 2.0).unwrap());
    println!("p vs a      = {:?}", p.partial_cmp(&a));
}
