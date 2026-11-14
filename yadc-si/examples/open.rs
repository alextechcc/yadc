//! The open type: `DynamicSIQuantity` holds a quantity of *any* dimension,
//! decided at run time - what a calculator, a parser or a config file needs.
//!
//! A static quantity (`SIQuantity<PRESSURE, f64>`, ...) has its dimension in its type,
//! so the compiler checks every operator. The open type keeps the dimension
//! in a field instead, and checks at run time: any operator may have no
//! result (`None`) - a sum of two dimensions, a product with a level, a root
//! that does not divide. Into it is std `From`; back out is std `TryFrom`,
//! which checks the dimension and hands the value back on a mismatch.
//!
//! Numbers go in and out through units: `DynamicSIUnit`, a unit of any
//! dimension, builds an open quantity from a number and reads one back -
//! `None` for a quantity of another dimension, or for no quantity, so it
//! reads an operator's result as it comes.
//!
//! Run with: cargo run -p yadc-si --example open
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use yadc::{PartialPowi, PartialRoot, Unit};
use yadc_si::SIQuantity;
use yadc_si::dynamic::mechanical::{
    AreaUnit, ForceUnit, GainUnit, LengthUnit, PowerLevelUnit, PressureUnit, TimeUnit, VolumeUnit,
};
use yadc_si::dynamic::thermal::{TemperatureIntervalUnit, ThermodynamicTemperatureUnit};
use yadc_si::dynamic::{DynamicSIQuantity, DynamicSIUnit};
use yadc_si::mechanical::units::{BAR, PSI, UNIX_SECOND};
use yadc_si::mechanical::{LENGTH, PRESSURE};

/// Generic over any dynamic version of `T`: in, and back.
fn round_trip<T, X: From<T>>(t: T) -> Result<T, X>
where
    T: TryFrom<X, Error = X>,
{
    T::try_from(X::from(t))
}

fn main() {
    let bar = DynamicSIUnit::from(BAR);
    let square_metre = DynamicSIUnit::from(AreaUnit::SquareMetre);
    let newton = DynamicSIUnit::from(ForceUnit::Newton);
    let p = bar.quantity(Some(1.01325)).unwrap();
    let a = square_metre.quantity(Some(0.5)).unwrap();

    // Arithmetic decided at run time: every operator may have no result. An
    // amount times an amount has one ...
    println!("p × a       = {:.1?} {newton}", newton.value(p * a));
    // ... `+` only for one dimension; scaling keeps the dimension.
    println!("p + a       = {:?}", p + a);
    println!("p × 2       = {:?} {bar}", bar.value(p * 2.0));
    // A unit of another dimension reads nothing.
    println!("p in m²     = {:?}", square_metre.value(Some(p)));
    // `?` stops a chain at the first step that has no result.
    let chain = || ((p * a)? + (p * a)?)? * 2.0;
    println!("(p×a + p×a)×2 = {:.1?} {newton}", newton.value(chain()));

    // Powers and roots with an exponent known only at run time: a root
    // exists only where every exponent divides.
    let metre = DynamicSIUnit::from(LengthUnit::Metre);
    let side = a.partial_root(2).unwrap();
    println!("√a          = {:.4?} {metre}", metre.value(Some(side)));
    println!("√√a         = {:?}", side.partial_root(2));
    println!("a³ exists   = {:?}", a.partial_powi(3).is_some());
    let cubic_metre = DynamicSIUnit::from(VolumeUnit::CubicMetre);
    println!(
        "a × √a      = {:.4?} {cubic_metre}",
        cubic_metre.value(a * side)
    );

    // A bare number is the same value as a dimensionless quantity, both
    // ways.
    let ratio = (p / bar.quantity(Some(1.0)).unwrap()).unwrap();
    println!("p / 1 bar   = {:?}", f64::try_from(ratio));
    println!(
        "a × 0.5     = {:?} {square_metre}",
        square_metre.value(a * DynamicSIQuantity::from(0.5))
    );

    // The way back to a static quantity: std `TryFrom` checks the dimension,
    // and a mismatch hands the value back.
    let back: SIQuantity<PRESSURE, f64> = p.try_into().unwrap();
    println!(
        "back        = {:.3} {}",
        PressureUnit::Pascal.value(back),
        PressureUnit::Pascal
    );
    println!(
        "as a length = {:?}",
        SIQuantity::<LENGTH, _>::try_from(p).is_ok()
    );
    let kept = round_trip::<SIQuantity<PRESSURE, f64>, DynamicSIQuantity>(back);
    println!("round trip  = {:.3?} psi", kept.map(|q| PSI.value(q)));

    // A temperature is an amount kept apart from its differences: it scales
    // and averages as a temperature, and two differ by an interval.
    let celsius = DynamicSIUnit::from(ThermodynamicTemperatureUnit::Celsius);
    let kelvin = DynamicSIUnit::from(TemperatureIntervalUnit::Kelvin);
    let t = celsius.quantity(Some(25.0)).unwrap();
    let ice = celsius.quantity(Some(0.0)).unwrap();
    println!(
        "(t + ice)/2 = {:.2?} {celsius}",
        celsius.value((t + ice).and_then(|s| s / 2.0))
    );
    println!("t - ice     = {:?} {kelvin}", kelvin.value(t - ice));
    println!("t × 2       = {:.2?} {celsius}", celsius.value(t * 2.0));

    // A power level is a logarithm: a gain moves it, and it has no products
    // and cannot be scaled.
    let dbm = DynamicSIUnit::from(PowerLevelUnit::DecibelMilliwatt);
    let decibel = DynamicSIUnit::from(GainUnit::Decibel);
    let level = dbm.quantity(Some(30.0)).unwrap();
    let gain = decibel.quantity(Some(13.0)).unwrap();
    println!("30 dBm+13dB = {:.1?} {dbm}", dbm.value(level + gain));
    println!("level × a   = {:?}", level * a);
    println!("level × 2   = {:?}", level * 2.0);

    // An instant is a point: two differ by a time, a time moves one, and it
    // has no sum with another, no products and no scaling.
    let unix = DynamicSIUnit::from(UNIX_SECOND);
    let hour = DynamicSIUnit::from(TimeUnit::Hour);
    let launch = unix.quantity(Some(1_700_000_000.0)).unwrap();
    let later = (launch + hour.quantity(Some(2.0)).unwrap()).unwrap();
    println!("later       = {:?} {unix:#}", unix.value(Some(later)));
    println!("later - t0  = {:?} {hour}", hour.value(later - launch));
    println!("t0 + later  = {:?}", launch + later);
    println!("t0 × 2      = {:?}", launch * 2.0);

    // Values of one dimension are ordered; of two, not.
    println!("p < 2p      = {:?}", p < (p * 2.0).unwrap());
    println!("p vs a      = {:?}", p.partial_cmp(&a));
}
