//! The bridge to uom: std `From` and `TryFrom` between uom's quantities and
//! yadc-si's static and dynamic ones, the number moving as it is, in coherent
//! SI units.
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use std::convert::TryFrom;

use uom::si::acceleration::meter_per_second_squared;
use uom::si::angle::radian;
use uom::si::f64::{Acceleration, Angle, Force, Length, Mass, SolidAngle, Velocity};
use uom::si::force::newton;
use uom::si::length::{kilometer, meter};
use uom::si::mass::kilogram;
use uom::si::solid_angle::steradian;
use uom::si::velocity::meter_per_second;
use yadc::Unit;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::angular::*;
use yadc_si::dynamic::mechanical::*;
use yadc_si::mechanical::*;
use yadc_si::{Prefix, Prefixed, SIQuantity};

const KILOMETRE: Prefixed<LengthUnit> = Prefixed {
    prefix: Some(Prefix::Kilo),
    unit: LengthUnit::Metre,
};

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

// --- SIQuantity ---------------------------------------------------------

#[test]
fn static_from_uom_length() {
    let uom = Length::new::<kilometer>(42.195);
    let q: SIQuantity<_, f64> = uom.into();
    close(LengthUnit::Metre.value(q), 42_195.0);
    close(KILOMETRE.value(q), 42.195);
}

#[test]
fn static_from_uom_mass() {
    let uom = Mass::new::<kilogram>(70.0);
    let q: SIQuantity<_, f64> = uom.into();
    close(MassUnit::Kilogram.value(q), 70.0);
    close(MassUnit::Gram.value(q), 70_000.0);
}

#[test]
fn static_into_uom_length() {
    let q = KILOMETRE.quantity(5.0);
    let uom: Length = q.into();
    close(uom.get::<meter>(), 5_000.0);
    close(uom.get::<kilometer>(), 5.0);
}

#[test]
fn static_round_trip_uom_to_yadc_to_uom() {
    let original = Length::new::<meter>(1234.5);
    let q: SIQuantity<_, f64> = original.into();
    let back: Length = q.into();
    close(back.get::<meter>(), 1234.5);
}

#[test]
fn static_derived_quantity_velocity() {
    let uom = Velocity::new::<meter_per_second>(25.0);
    let q: SIQuantity<_, f64> = uom.into();
    close(VelocityUnit::MetrePerSecond.value(q), 25.0);
    let back: Velocity = q.into();
    close(back.get::<meter_per_second>(), 25.0);
}

#[test]
fn static_derived_quantity_force() {
    let uom = Force::new::<newton>(9.81);
    let q: SIQuantity<_, f64> = uom.into();
    close(ForceUnit::Newton.value(q), 9.81);
    let back: Force = q.into();
    close(back.get::<newton>(), 9.81);
}

// --- DynamicSIQuantity --------------------------------------------------------

#[test]
fn dynamic_from_uom_length() {
    let uom = Length::new::<meter>(500.0);
    let dq: DynamicSIQuantity = uom.into();
    close(
        LengthUnit::Metre.value(yadc_si::mechanical::Length::try_from(dq).unwrap()),
        500.0,
    );
    assert!(yadc_si::mechanical::Mass::try_from(dq).is_err());
}

#[test]
fn dynamic_from_uom_mass() {
    let uom = Mass::new::<kilogram>(70.0);
    let dq: DynamicSIQuantity = uom.into();
    close(
        MassUnit::Kilogram.value(yadc_si::mechanical::Mass::try_from(dq).unwrap()),
        70.0,
    );
}

#[test]
fn dynamic_from_derived_velocity() {
    let uom = Velocity::new::<meter_per_second>(10.0);
    let dq: DynamicSIQuantity = uom.into();
    close(
        VelocityUnit::MetrePerSecond.value(yadc_si::mechanical::Velocity::try_from(dq).unwrap()),
        10.0,
    );
    // velocity dim doesn't match force
    assert!(yadc_si::mechanical::Force::try_from(dq).is_err());
}

#[test]
fn dynamic_from_derived_acceleration() {
    let uom = Acceleration::new::<meter_per_second_squared>(9.81);
    let dq: DynamicSIQuantity = uom.into();
    close(
        AccelerationUnit::MetrePerSecondSquared
            .value(yadc_si::mechanical::Acceleration::try_from(dq).unwrap()),
        9.81,
    );
}

#[test]
fn dynamic_try_from_matching_dim_succeeds() {
    let dq: DynamicSIQuantity = Length::new::<meter>(300.0).into();
    let uom = Length::try_from(dq).expect("dim matches");
    close(uom.get::<meter>(), 300.0);
}

#[test]
fn dynamic_try_from_wrong_dim_returns_original() {
    let dq: DynamicSIQuantity = Mass::new::<kilogram>(5.0).into();
    let err = Length::try_from(dq).unwrap_err();
    // The original DynamicSIQuantity is returned so the caller can recover it.
    close(
        MassUnit::Kilogram.value(yadc_si::mechanical::Mass::try_from(err).unwrap()),
        5.0,
    );
}

#[test]
fn dynamic_round_trip_uom_to_dynamic_to_uom() {
    let original = Force::new::<newton>(42.0);
    let dq: DynamicSIQuantity = original.into();
    let back = Force::try_from(dq).expect("round-trip");
    close(back.get::<newton>(), 42.0);
}

#[test]
fn dynamic_arithmetic_after_uom_conversion() {
    // Demonstrate that a DynamicSIQuantity converted from uom still participates
    // in runtime dim algebra.
    let mass: DynamicSIQuantity = Mass::new::<kilogram>(2.0).into();
    let accel: DynamicSIQuantity = Acceleration::new::<meter_per_second_squared>(3.0).into();
    // A product of two amounts exists: mass x acceleration composes to a
    // Force.
    let force = (mass * accel).unwrap();
    close(
        ForceUnit::Newton.value(yadc_si::mechanical::Force::try_from(force).unwrap()),
        6.0,
    );
}

// --- Kind mapping -------------------------------------------------------------
// uom tells angles apart from plain ratios with a kind; yadc-si counts them on
// its `rad` axis. uom's temperature kind is yadc-si's `Absolute` label.

#[test]
fn static_angle_from_uom_is_one_radian_axis() {
    let uom = Angle::new::<radian>(std::f64::consts::FRAC_PI_2);
    let q: SIQuantity<_, f64> = uom.into();
    close(AngleUnit::Radian.value(q), std::f64::consts::FRAC_PI_2);
}

#[test]
fn static_angle_round_trip() {
    let original = Angle::new::<radian>(std::f64::consts::PI);
    let q: SIQuantity<_, f64> = original.into();
    let back: Angle = q.into();
    close(back.get::<radian>(), std::f64::consts::PI);
}

#[test]
fn static_solid_angle_round_trip() {
    let original = SolidAngle::new::<steradian>(4.0);
    let q: SIQuantity<_, f64> = original.into();
    close(SolidAngleUnit::Steradian.value(q), 4.0);
    let back: SolidAngle = q.into();
    close(back.get::<steradian>(), 4.0);
}

#[test]
fn dynamic_angle_kind_preserved() {
    let dq: DynamicSIQuantity = Angle::new::<radian>(1.0).into();
    close(
        AngleUnit::Radian.value(yadc_si::angular::Angle::try_from(dq).unwrap()),
        1.0,
    );
    // Must not match a plain dimensionless quantity (no radians).
    assert!(yadc_si::dimensionless::Dimensionless::try_from(dq).is_err());
}

#[test]
fn dynamic_solid_angle_kind_preserved() {
    let dq: DynamicSIQuantity = SolidAngle::new::<steradian>(2.5).into();
    close(
        SolidAngleUnit::Steradian.value(yadc_si::angular::SolidAngle::try_from(dq).unwrap()),
        2.5,
    );
    assert!(yadc_si::angular::Angle::try_from(dq).is_err());
}

#[test]
fn dynamic_try_from_angle_succeeds() {
    let dq: DynamicSIQuantity = Angle::new::<radian>(0.5).into();
    let back = Angle::try_from(dq).expect("angle dim matches");
    close(back.get::<radian>(), 0.5);
}

#[test]
fn dynamic_try_from_angle_rejects_dimensionless() {
    // An angle (rad¹) must not convert to a uom quantity of another dimension;
    // the original comes back.
    let dq: DynamicSIQuantity = Angle::new::<radian>(1.0).into();
    let err = Mass::try_from(dq).unwrap_err();
    close(
        AngleUnit::Radian.value(yadc_si::angular::Angle::try_from(err).unwrap()),
        1.0,
    );
}

#[test]
fn uom_torque_is_joules_per_radian() {
    use uom::si::f64::Torque;
    use uom::si::torque::newton_meter;
    // uom marks torque with its angle kind; in yadc-si that is rad⁻¹, so the
    // conversion lands on `Torque` (J/rad) and torque × angle is energy.
    let q: yadc_si::mechanical::Torque<f64> = Torque::new::<newton_meter>(5.0).into();
    close(TorqueUnit::NewtonMetre.value(q), 5.0);
    let work: Energy<f64> = q * AngleUnit::Radian.quantity(2.0);
    close(EnergyUnit::Joule.value(work), 10.0);
}

#[test]
fn uom_temperature_kind_is_absolute() {
    use uom::si::f64::ThermodynamicTemperature;
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use yadc_si::dynamic::thermal::ThermodynamicTemperatureUnit;
    let t: yadc_si::thermal::ThermodynamicTemperature<f64> =
        ThermodynamicTemperature::new::<degree_celsius>(25.0).into();
    close(ThermodynamicTemperatureUnit::Kelvin.value(t), 298.15);
    let dq: DynamicSIQuantity = ThermodynamicTemperature::new::<kelvin>(300.0).into();
    // A temperature, not an interval.
    assert!(yadc_si::thermal::ThermodynamicTemperature::try_from(dq).is_ok());
    assert!(yadc_si::thermal::TemperatureInterval::try_from(dq).is_err());
    let back = ThermodynamicTemperature::try_from(dq).unwrap();
    close(back.get::<kelvin>(), 300.0);
}

#[test]
fn dynamic_plain_ratio_is_not_a_uom_angle() {
    // A plain ratio has no radians, so it does not convert to uom's Angle;
    // the original comes back.
    use yadc_si::dimensionless::Dimensionless;
    use yadc_si::dynamic::dimensionless::DimensionlessUnit;
    let ratio = DynamicSIQuantity::from(DimensionlessUnit::Ratio.quantity(1.0));
    let err = Angle::try_from(ratio).unwrap_err();
    close(
        DimensionlessUnit::Ratio.value(Dimensionless::try_from(err).unwrap()),
        1.0,
    );
}

#[test]
fn uom_curvature_is_radians_per_metre() {
    use uom::si::curvature::radian_per_meter;
    use uom::si::f64::Curvature;
    // uom marks curvature with its angle kind: rad/m, not a plain 1/m.
    let q: yadc_si::mechanical::Curvature<f64> = Curvature::new::<radian_per_meter>(0.25).into();
    close(CurvatureUnit::RadianPerMetre.value(q), 0.25);
    let back: Curvature = q.into();
    close(back.get::<radian_per_meter>(), 0.25);
    let dq: DynamicSIQuantity = Curvature::new::<radian_per_meter>(0.25).into();
    assert!(yadc_si::mechanical::Curvature::try_from(dq).is_ok());
    assert!(yadc_si::mechanical::ReciprocalLength::try_from(dq).is_err());
    // An arc length times its curvature is the angle it turns through.
    let turn = q * LengthUnit::Metre.quantity(4.0);
    close(AngleUnit::Radian.value(turn), 1.0);
}

#[test]
fn uom_angular_kinds_land_on_their_radians() {
    use uom::si::angular_absement::radian_second;
    use uom::si::angular_acceleration::radian_per_second_squared;
    use uom::si::angular_jerk::radian_per_second_cubed;
    use uom::si::angular_momentum::kilogram_square_meter_per_second;
    use uom::si::angular_velocity::radian_per_second;
    use uom::si::f64::{
        AngularAbsement, AngularAcceleration, AngularJerk, AngularMomentum, AngularVelocity,
    };
    let w: yadc_si::angular::AngularVelocity<f64> =
        AngularVelocity::new::<radian_per_second>(2.0).into();
    close(AngularVelocityUnit::RadianPerSecond.value(w), 2.0);
    let a: yadc_si::angular::AngularAcceleration<f64> =
        AngularAcceleration::new::<radian_per_second_squared>(3.0).into();
    close(
        AngularAccelerationUnit::RadianPerSecondSquared.value(a),
        3.0,
    );
    let j: yadc_si::angular::AngularJerk<f64> =
        AngularJerk::new::<radian_per_second_cubed>(4.0).into();
    close(AngularJerkUnit::RadianPerSecondCubed.value(j), 4.0);
    let s: yadc_si::angular::AngularAbsement<f64> =
        AngularAbsement::new::<radian_second>(5.0).into();
    close(AngularAbsementUnit::RadianSecond.value(s), 5.0);
    let l: yadc_si::angular::AngularMomentum<f64> =
        AngularMomentum::new::<kilogram_square_meter_per_second>(6.0).into();
    close(
        AngularMomentumUnit::KilogramSquareMetrePerSecond.value(l),
        6.0,
    );
    let back: AngularMomentum = l.into();
    close(back.get::<kilogram_square_meter_per_second>(), 6.0);
}
