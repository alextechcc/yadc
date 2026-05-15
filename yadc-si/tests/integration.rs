#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::{Format, Unit};
use yadc_si::DynamicSIQuantity;
use yadc_si::angular::*;
use yadc_si::dimensionless::*;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

#[test]
fn quantity_and_from_quantity_round_trip() {
    let mass = MassUnit::Gram.quantity(1500.0);
    close(MassUnit::Kilogram.from_quantity(mass), 1.5);
    close(MassUnit::Gram.from_quantity(mass), 1500.0);
}

#[test]
fn dynamic_try_from_quantity_returns_option() {
    let m = DynamicSIQuantity::from(MassUnit::Kilogram.quantity(2.5));
    assert_eq!(MassUnit::Gram.try_from_quantity(m), Some(2500.0));
    assert_eq!(ForceUnit::Newton.try_from_quantity(m), None);
}

#[test]
fn mul_and_div_compose_dims() {
    let m = MassUnit::Kilogram.quantity(2.0);
    let a = AccelerationUnit::MeterPerSecondSquared.quantity(3.0);
    let f: Force = m * a;
    close(ForceUnit::Newton.from_quantity(f), 6.0);

    // Same-dim division yields Dimensionless.
    let r1 = ForceUnit::Newton.quantity(10.0);
    let r2 = ForceUnit::Newton.quantity(2.0);
    let ratio: Dimensionless = r1 / r2;
    close(DimensionlessUnit::Ratio.from_quantity(ratio), 5.0);
}

#[test]
fn affine_temperature_subtraction_yields_interval() {
    let hot = ThermodynamicTemperatureUnit::Celsius.quantity(100.0);
    let cold = ThermodynamicTemperatureUnit::Celsius.quantity(20.0);
    let delta: TemperatureInterval = hot - cold;
    close(TemperatureIntervalUnit::Kelvin.from_quantity(delta), 80.0);
    close(TemperatureIntervalUnit::Celsius.from_quantity(delta), 80.0);
}

#[test]
fn affine_temperature_addition_with_interval_stays_absolute() {
    let warm = ThermodynamicTemperatureUnit::Celsius.quantity(20.0);
    let rise = TemperatureIntervalUnit::Celsius.quantity(5.0);
    let hotter: ThermodynamicTemperature = warm + rise;
    close(ThermodynamicTemperatureUnit::Celsius.from_quantity(hotter), 25.0);
}

#[test]
fn temperature_intervals_compose_normally() {
    let a = TemperatureIntervalUnit::Celsius.quantity(5.0);
    let b = TemperatureIntervalUnit::Celsius.quantity(3.0);
    let sum: TemperatureInterval = a + b;
    close(TemperatureIntervalUnit::Kelvin.from_quantity(sum), 8.0);
}

#[test]
fn angle_distinct_from_dimensionless() {
    // Angle is a zero-numeric-axis dim *with* Kind::Angle, distinct from a plain
    // dimensionless ratio (Kind::Linear) even though the numerics match.
    let half_circle = AngleUnit::Degree.quantity(180.0);
    close(AngleUnit::Radian.from_quantity(half_circle), std::f64::consts::PI);

    let ratio = DimensionlessUnit::Ratio.quantity(0.5);
    close(DimensionlessUnit::Ratio.from_quantity(ratio), 0.5);

    // Angle / Angle -> Dimensionless (Angle / Angle = Linear in the kind table).
    let twice = AngleUnit::Degree.quantity(360.0);
    let dimensionless: Dimensionless = twice / half_circle;
    close(DimensionlessUnit::Ratio.from_quantity(dimensionless), 2.0);
}

#[test]
fn add_assign_and_sub_assign() {
    let mut m = MassUnit::Kilogram.quantity(1.0);
    m += MassUnit::Gram.quantity(500.0);
    close(MassUnit::Kilogram.from_quantity(m), 1.5);
    m -= MassUnit::Gram.quantity(250.0);
    close(MassUnit::Kilogram.from_quantity(m), 1.25);
}

#[test]
fn quantity_convenience_ops() {
    let zero = MassUnit::Kilogram.quantity(0.0);
    close(MassUnit::Kilogram.from_quantity(zero), 0.0);

    let debt = -MassUnit::Gram.quantity(500.0);
    close(MassUnit::Kilogram.from_quantity(debt), -0.5);

    assert!(MassUnit::Kilogram.quantity(2.0) > MassUnit::Kilogram.quantity(1.0));
    assert!(
        ThermodynamicTemperatureUnit::Celsius.quantity(25.0)
            > ThermodynamicTemperatureUnit::Celsius.quantity(20.0)
    );

    let addend = MassUnit::Gram.quantity(250.0);
    let mut borrowed = MassUnit::Kilogram.quantity(1.0);
    borrowed += &addend;
    close(MassUnit::Kilogram.from_quantity(borrowed), 1.25);
    borrowed -= &addend;
    close(MassUnit::Kilogram.from_quantity(borrowed), 1.0);

    let delta = TemperatureIntervalUnit::Celsius.quantity(5.0);
    let mut temp = ThermodynamicTemperatureUnit::Celsius.quantity(20.0);
    temp += &delta;
    close(ThermodynamicTemperatureUnit::Celsius.from_quantity(temp), 25.0);
    temp -= &delta;
    close(ThermodynamicTemperatureUnit::Celsius.from_quantity(temp), 20.0);

    let mut scaled = MassUnit::Kilogram.quantity(2.0);
    scaled *= DimensionlessUnit::Percent.quantity(50.0);
    close(MassUnit::Kilogram.from_quantity(scaled), 1.0);
    scaled /= DimensionlessUnit::Percent.quantity(25.0);
    close(MassUnit::Kilogram.from_quantity(scaled), 4.0);
}

#[test]
fn nonlinear_db_units() {
    // dB on Power: dBW is a real unit of Power (stored value is watts).
    let p = PowerUnit::DecibelWatt.quantity(20.0); // 20 dBW = 100 W
    close(PowerUnit::Watt.from_quantity(p), 100.0);
    close(PowerUnit::DecibelWatt.from_quantity(p), 20.0);
    // 0 dBW = 1 W = 30 dBm.
    let one_watt = PowerUnit::DecibelWatt.quantity(0.0);
    close(PowerUnit::DecibelMilliwatt.from_quantity(one_watt), 30.0);

    // dB on a dimensionless Ratio: stored value is the linear ratio.
    let r = DimensionlessUnit::Decibel.quantity(3.0); // ~2x power
    close(DimensionlessUnit::Ratio.from_quantity(r), 10f64.powf(0.3));
    close(DimensionlessUnit::Decibel.from_quantity(r), 3.0);
    let bel = DimensionlessUnit::Bel.quantity(1.0);
    close(DimensionlessUnit::Ratio.from_quantity(bel), 10.0);

    // Power flows through ordinary arithmetic; dB is just a readout.
    let doubled = p * 2.0;
    close(
        PowerUnit::DecibelWatt.from_quantity(doubled),
        20.0 + 10.0 * 2f64.log10(),
    );
}

#[test]
fn format_quantity_via_unit() {
    let p = PowerUnit::Watt.quantity(100.0);
    let s = format!("{}", PowerUnit::DecibelWatt.format_quantity(p, Format::Short));
    assert_eq!(s, "20 dBW");
}
