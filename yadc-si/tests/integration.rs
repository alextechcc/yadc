#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::{DynamicQuantity, Format, StaticQuantity};
use yadc_si::DynamicSIQuantity;
use yadc_si::angular::*;
use yadc_si::dimensionless::*;
use yadc_si::electromagnetic::*;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

#[test]
fn from_unit_and_as_round_trip() {
    let mass = Mass::from_unit(1500.0, MassUnit::Gram);
    close(mass.as_unit(MassUnit::Kilogram), 1.5);
    close(mass.as_unit(MassUnit::Gram), 1500.0);
}

#[test]
fn dynamic_as_returns_option() {
    let m = DynamicSIQuantity::from_unit(2.5, MassUnit::Kilogram);
    assert_eq!(m.as_unit(MassUnit::Gram), Some(2500.0));
    assert_eq!(m.as_unit(ForceUnit::Newton), None);
}

#[test]
fn mul_and_div_compose_dims() {
    let m = Mass::from_unit(2.0, MassUnit::Kilogram);
    let a = Acceleration::from_unit(3.0, AccelerationUnit::MeterPerSecondSquared);
    let f: Force = m * a;
    close(f.as_unit(ForceUnit::Newton), 6.0);

    // Same-dim division yields Dimensionless.
    let r1 = Force::from_unit(10.0, ForceUnit::Newton);
    let r2 = Force::from_unit(2.0, ForceUnit::Newton);
    let ratio: Dimensionless = r1 / r2;
    close(ratio.as_unit(DimensionlessUnit::Ratio), 5.0);
}

#[test]
fn affine_temperature_subtraction_yields_interval() {
    // T - T : ThermodynamicTemperature - ThermodynamicTemperature = TemperatureInterval
    let hot = ThermodynamicTemperature::from_unit(100.0, ThermodynamicTemperatureUnit::Celsius);
    let cold = ThermodynamicTemperature::from_unit(20.0, ThermodynamicTemperatureUnit::Celsius);
    let delta: TemperatureInterval = hot - cold;
    close(delta.as_unit(TemperatureIntervalUnit::Kelvin), 80.0);
    close(delta.as_unit(TemperatureIntervalUnit::Celsius), 80.0);
}

#[test]
fn affine_temperature_addition_with_interval_stays_absolute() {
    // T + ΔT : ThermodynamicTemperature + TemperatureInterval = ThermodynamicTemperature
    let warm = ThermodynamicTemperature::from_unit(20.0, ThermodynamicTemperatureUnit::Celsius);
    let rise = TemperatureInterval::from_unit(5.0, TemperatureIntervalUnit::Celsius);
    let hotter: ThermodynamicTemperature = warm + rise;
    close(hotter.as_unit(ThermodynamicTemperatureUnit::Celsius), 25.0);
}

#[test]
fn temperature_intervals_compose_normally() {
    // ΔT + ΔT = ΔT
    let a = TemperatureInterval::from_unit(5.0, TemperatureIntervalUnit::Celsius);
    let b = TemperatureInterval::from_unit(3.0, TemperatureIntervalUnit::Celsius);
    let sum: TemperatureInterval = a + b;
    close(sum.as_unit(TemperatureIntervalUnit::Kelvin), 8.0);
}

#[test]
fn angle_distinct_from_dimensionless() {
    // Angle is a Linear-axis-zero dim *with* Kind::Angle, while Dimensionless has
    // Kind::Linear. Even though numerics match, the types are distinct: assigning
    // one to the other does not type-check. We can still construct both and use
    // them within their own families.
    let half_circle = Angle::from_unit(180.0, AngleUnit::Degree);
    close(half_circle.as_unit(AngleUnit::Radian), std::f64::consts::PI);

    let ratio = Dimensionless::from_unit(0.5, DimensionlessUnit::Ratio);
    close(ratio.as_unit(DimensionlessUnit::Ratio), 0.5);

    // Angle / Angle → Dimensionless (per Kind::div table: Angle/Angle = Linear).
    let twice = Angle::from_unit(360.0, AngleUnit::Degree);
    let dimensionless: Dimensionless = twice / half_circle;
    close(dimensionless.as_unit(DimensionlessUnit::Ratio), 2.0);
}

#[test]
fn add_assign_and_sub_assign() {
    let mut m = Mass::from_unit(1.0, MassUnit::Kilogram);
    m += Mass::from_unit(500.0, MassUnit::Gram);
    close(m.as_unit(MassUnit::Kilogram), 1.5);
    m -= Mass::from_unit(250.0, MassUnit::Gram);
    close(m.as_unit(MassUnit::Kilogram), 1.25);
}

#[test]
fn quantity_convenience_ops() {
    let zero = Mass::from_unit(0.0, MassUnit::Kilogram);
    close(zero.as_unit(MassUnit::Kilogram), 0.0);

    let debt = -Mass::from_unit(500.0, MassUnit::Gram);
    close(debt.as_unit(MassUnit::Kilogram), -0.5);

    assert!(Mass::from_unit(2.0, MassUnit::Kilogram) > Mass::from_unit(1.0, MassUnit::Kilogram));
    assert!(
        ThermodynamicTemperature::from_unit(25.0, ThermodynamicTemperatureUnit::Celsius)
            > ThermodynamicTemperature::from_unit(20.0, ThermodynamicTemperatureUnit::Celsius)
    );

    let addend = Mass::from_unit(250.0, MassUnit::Gram);
    let mut borrowed = Mass::from_unit(1.0, MassUnit::Kilogram);
    borrowed += &addend;
    close(borrowed.as_unit(MassUnit::Kilogram), 1.25);
    borrowed -= &addend;
    close(borrowed.as_unit(MassUnit::Kilogram), 1.0);

    let delta = TemperatureInterval::from_unit(5.0, TemperatureIntervalUnit::Celsius);
    let mut temp = ThermodynamicTemperature::from_unit(20.0, ThermodynamicTemperatureUnit::Celsius);
    temp += &delta;
    close(temp.as_unit(ThermodynamicTemperatureUnit::Celsius), 25.0);
    temp -= &delta;
    close(temp.as_unit(ThermodynamicTemperatureUnit::Celsius), 20.0);

    let mut scaled = Mass::from_unit(2.0, MassUnit::Kilogram);
    scaled *= Dimensionless::from_unit(50.0, DimensionlessUnit::Percent);
    close(scaled.as_unit(MassUnit::Kilogram), 1.0);
    scaled /= Dimensionless::from_unit(25.0, DimensionlessUnit::Percent);
    close(scaled.as_unit(MassUnit::Kilogram), 4.0);
}

#[test]
fn quantities_can_sum_owned_and_borrowed_iterators() {
    let masses = [
        Mass::from_unit(1.0, MassUnit::Kilogram),
        Mass::from_unit(500.0, MassUnit::Gram),
        Mass::from_unit(250.0, MassUnit::Gram),
    ];

    let borrowed: Mass = masses.iter().sum();
    close(borrowed.as_unit(MassUnit::Kilogram), 1.75);

    let owned: Mass = masses.into_iter().sum();
    close(owned.as_unit(MassUnit::Kilogram), 1.75);
}

#[test]
fn temperature_in_place_with_interval() {
    // T += ΔT must work (output kind == self kind).
    let mut t = ThermodynamicTemperature::from_unit(20.0, ThermodynamicTemperatureUnit::Celsius);
    t += TemperatureInterval::from_unit(5.0, TemperatureIntervalUnit::Celsius);
    close(t.as_unit(ThermodynamicTemperatureUnit::Celsius), 25.0);
    t -= TemperatureInterval::from_unit(10.0, TemperatureIntervalUnit::Celsius);
    close(t.as_unit(ThermodynamicTemperatureUnit::Celsius), 15.0);
}

#[test]
fn format_short_and_long() {
    let f = Force::from_unit(9.81, ForceUnit::Newton);
    assert_eq!(
        format!("{}", f.fmt_as_unit(ForceUnit::Newton, Format::Short)),
        "9.81 N"
    );
    assert_eq!(
        format!("{}", f.fmt_as_unit(ForceUnit::Newton, Format::Long)),
        "9.81 newtons"
    );
    assert_eq!(
        format!("{:.3}", f.fmt_as_unit(ForceUnit::Kilonewton, Format::Short)),
        "0.010 kN"
    );
}

#[test]
fn dimensionless_as_unit() {
    let half = Dimensionless::from_unit(50.0, DimensionlessUnit::Percent);
    close(half.as_unit(DimensionlessUnit::Ratio), 0.5);
}

#[test]
fn dyn_qty_arithmetic_and_dims_check() {
    let v = DynamicSIQuantity::from_unit(12.0, ElectricPotentialUnit::Volt);
    let i = DynamicSIQuantity::from_unit(2.0, ElectricCurrentUnit::Ampere);
    let p = v * i;
    assert_eq!(p.as_unit(PowerUnit::Watt), Some(24.0));

    let mass = DynamicSIQuantity::from_unit(1.0, MassUnit::Kilogram);
    let force = DynamicSIQuantity::from_unit(1.0, ForceUnit::Newton);
    assert!((mass + force).is_none()); // different dims, runtime add fails
}

#[test]
fn dyn_into_typed_round_trip() {
    let typed = Mass::from_unit(2.0, MassUnit::Kilogram);
    let d: DynamicSIQuantity = typed.into();
    let back: Mass = d.try_into().expect("round-trip");
    close(back.as_unit(MassUnit::Kilogram), 2.0);

    // Wrong dim → TryFrom Err.
    let force = DynamicSIQuantity::from_unit(1.0, ForceUnit::Newton);
    assert!(Mass::try_from(force).is_err());
}
