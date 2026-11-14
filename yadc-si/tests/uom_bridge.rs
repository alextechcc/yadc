//! The bridge to uom: std `From` and `TryFrom` between uom's quantities and
//! yadc-si's static and dynamic ones, the number moving as it is, in SI base
//! units.
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use std::convert::TryFrom;

use uom::si::acceleration::meter_per_second_squared;
use uom::si::angle::radian;
use uom::si::f64 as u;
use uom::si::force::newton;
use uom::si::length::{kilometer, meter};
use uom::si::mass::kilogram;
use uom::si::solid_angle::steradian;
use uom::si::velocity::meter_per_second;
use yadc_si::angular::*;
use yadc_si::dimensionless::Dimensionless;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::mechanical::*;
use yadc_si::thermal::ThermodynamicTemperature;
use yadc_si::{KELVIN, KILOGRAM, METRE, RADIAN, SECOND, SIQuantity};

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

/// The number of kilometres a length is.
fn kilometres(q: Length<f64>) -> f64 {
    let km: SIQuantity<{ KILOMETRE.unit() }, f64, false> = q.into();
    f64::from(km / (1.0 * KILOMETRE))
}

// --- SIQuantity ---------------------------------------------------------

#[test]
fn static_from_uom_length() {
    let q: Length<f64> = u::Length::new::<kilometer>(42.195).into();
    close(f64::from(q / (1.0 * METRE)), 42_195.0);
    close(kilometres(q), 42.195);
}

#[test]
fn static_from_uom_mass() {
    let q: Mass<f64> = u::Mass::new::<kilogram>(70.0).into();
    close(f64::from(q / (1.0 * KILOGRAM)), 70.0);
    let grams: SIQuantity<{ GRAM.unit() }, f64, false> = q.into();
    close(f64::from(grams / (1.0 * GRAM)), 70_000.0);
}

#[test]
fn static_into_uom_length() {
    let q: Length<f64> = (5.0_f64 * KILOMETRE).into();
    let uom: u::Length = q.into();
    close(uom.get::<meter>(), 5_000.0);
    close(uom.get::<kilometer>(), 5.0);
}

#[test]
fn static_round_trip_uom_to_yadc_to_uom() {
    let original = u::Length::new::<meter>(1234.5);
    let q: Length<f64> = original.into();
    let back: u::Length = q.into();
    close(back.get::<meter>(), 1234.5);
}

#[test]
fn static_derived_quantity_velocity() {
    let q: Velocity<f64> = u::Velocity::new::<meter_per_second>(25.0).into();
    close(f64::from(q / (1.0 * METRE / SECOND)), 25.0);
    let back: u::Velocity = q.into();
    close(back.get::<meter_per_second>(), 25.0);
}

#[test]
fn static_derived_quantity_force() {
    let q: Force<f64> = u::Force::new::<newton>(9.81).into();
    close(f64::from(q / (1.0 * NEWTON)), 9.81);
    let back: u::Force = q.into();
    close(back.get::<newton>(), 9.81);
}

// --- DynamicSIQuantity --------------------------------------------------------

#[test]
fn dynamic_from_uom_length() {
    let dq: DynamicSIQuantity = u::Length::new::<meter>(500.0).into();
    let q = Length::<f64>::try_from(dq).unwrap();
    close(f64::from(q / (1.0 * METRE)), 500.0);
    assert!(Mass::<f64>::try_from(dq).is_err());
}

#[test]
fn dynamic_from_uom_mass() {
    let dq: DynamicSIQuantity = u::Mass::new::<kilogram>(70.0).into();
    let q = Mass::<f64>::try_from(dq).unwrap();
    close(f64::from(q / (1.0 * KILOGRAM)), 70.0);
}

#[test]
fn dynamic_from_derived_velocity() {
    let dq: DynamicSIQuantity = u::Velocity::new::<meter_per_second>(10.0).into();
    let q = Velocity::<f64>::try_from(dq).unwrap();
    close(f64::from(q / (1.0 * METRE / SECOND)), 10.0);
    // A velocity is not a force.
    assert!(Force::<f64>::try_from(dq).is_err());
}

#[test]
fn dynamic_from_derived_acceleration() {
    let dq: DynamicSIQuantity = u::Acceleration::new::<meter_per_second_squared>(9.81).into();
    let q = Acceleration::<f64>::try_from(dq).unwrap();
    close(f64::from(q / (1.0 * METRE / (SECOND * SECOND))), 9.81);
}

#[test]
fn dynamic_try_from_matching_unit_succeeds() {
    let dq: DynamicSIQuantity = u::Length::new::<meter>(300.0).into();
    let uom = u::Length::try_from(dq).expect("unit matches");
    close(uom.get::<meter>(), 300.0);
}

#[test]
fn dynamic_try_from_wrong_unit_returns_original() {
    let dq: DynamicSIQuantity = u::Mass::new::<kilogram>(5.0).into();
    let err = u::Length::try_from(dq).unwrap_err();
    // The original DynamicSIQuantity is returned so the caller can recover it.
    let q = Mass::<f64>::try_from(err).unwrap();
    close(f64::from(q / (1.0 * KILOGRAM)), 5.0);
}

/// A quantity in another unit than a base unit is not uom's: uom counts in
/// base units.
#[test]
fn dynamic_try_from_a_unit_that_is_not_base_fails() {
    let dq = DynamicSIQuantity::from(2.0_f64 * KILOMETRE);
    assert!(u::Length::try_from(dq).is_err());
    let metres: Length<f64> = (2.0_f64 * KILOMETRE).into();
    let uom = u::Length::try_from(DynamicSIQuantity::from(metres)).unwrap();
    close(uom.get::<meter>(), 2000.0);
}

#[test]
fn dynamic_round_trip_uom_to_dynamic_to_uom() {
    let dq: DynamicSIQuantity = u::Force::new::<newton>(42.0).into();
    let back = u::Force::try_from(dq).expect("round-trip");
    close(back.get::<newton>(), 42.0);
}

#[test]
fn dynamic_arithmetic_after_uom_conversion() {
    // A DynamicSIQuantity converted from uom takes part in run-time unit
    // algebra: mass × acceleration is a force.
    let mass: DynamicSIQuantity = u::Mass::new::<kilogram>(2.0).into();
    let accel: DynamicSIQuantity = u::Acceleration::new::<meter_per_second_squared>(3.0).into();
    let force = (mass * accel).unwrap();
    let q = Force::<f64>::try_from(force).unwrap();
    close(f64::from(q / (1.0 * NEWTON)), 6.0);
}

// --- Kind mapping -------------------------------------------------------------
// uom tells angles apart from plain ratios with a kind; yadc-si counts them in
// radians, a base unit.

#[test]
fn static_angle_from_uom_is_in_radians() {
    let q: Angle<f64> = u::Angle::new::<radian>(std::f64::consts::FRAC_PI_2).into();
    close(f64::from(q / (1.0 * RADIAN)), std::f64::consts::FRAC_PI_2);
}

#[test]
fn static_angle_round_trip() {
    let original = u::Angle::new::<radian>(std::f64::consts::PI);
    let q: Angle<f64> = original.into();
    let back: u::Angle = q.into();
    close(back.get::<radian>(), std::f64::consts::PI);
}

#[test]
fn static_solid_angle_round_trip() {
    let q: SolidAngle<f64> = u::SolidAngle::new::<steradian>(4.0).into();
    close(f64::from(q / (1.0 * STERADIAN)), 4.0);
    let back: u::SolidAngle = q.into();
    close(back.get::<steradian>(), 4.0);
}

#[test]
fn dynamic_angle_kind_preserved() {
    let dq: DynamicSIQuantity = u::Angle::new::<radian>(1.0).into();
    let q = Angle::<f64>::try_from(dq).unwrap();
    close(f64::from(q / (1.0 * RADIAN)), 1.0);
    // Not a plain number (no radians).
    assert!(Dimensionless::<f64>::try_from(dq).is_err());
}

#[test]
fn dynamic_solid_angle_kind_preserved() {
    let dq: DynamicSIQuantity = u::SolidAngle::new::<steradian>(2.5).into();
    let q = SolidAngle::<f64>::try_from(dq).unwrap();
    close(f64::from(q / (1.0 * STERADIAN)), 2.5);
    assert!(Angle::<f64>::try_from(dq).is_err());
}

#[test]
fn dynamic_try_from_angle_succeeds() {
    let dq: DynamicSIQuantity = u::Angle::new::<radian>(0.5).into();
    let back = u::Angle::try_from(dq).expect("angle unit matches");
    close(back.get::<radian>(), 0.5);
}

#[test]
fn dynamic_try_from_angle_rejects_another_quantity() {
    // An angle (rad) does not convert to a uom quantity of another dimension;
    // the original comes back.
    let dq: DynamicSIQuantity = u::Angle::new::<radian>(1.0).into();
    let err = u::Mass::try_from(dq).unwrap_err();
    let q = Angle::<f64>::try_from(err).unwrap();
    close(f64::from(q / (1.0 * RADIAN)), 1.0);
}

#[test]
fn uom_torque_is_joules_per_radian() {
    use uom::si::torque::newton_meter;
    // uom marks torque with its angle kind; in yadc-si that is rad⁻¹, so the
    // conversion lands on `Torque` (J/rad) and torque × angle is energy.
    let q: Torque<f64> = u::Torque::new::<newton_meter>(5.0).into();
    close(f64::from(q / (1.0 * NEWTON_METRE)), 5.0);
    let work: Energy<f64> = q * (2.0 * RADIAN);
    close(f64::from(work / (1.0 * JOULE)), 10.0);
}

#[test]
fn uom_temperature_is_kelvin() {
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    let t: ThermodynamicTemperature<f64> =
        u::ThermodynamicTemperature::new::<degree_celsius>(25.0).into();
    close(f64::from(t / (1.0 * KELVIN)), 298.15);
    let dq: DynamicSIQuantity = u::ThermodynamicTemperature::new::<kelvin>(300.0).into();
    assert!(ThermodynamicTemperature::<f64>::try_from(dq).is_ok());
    let back = u::ThermodynamicTemperature::try_from(dq).unwrap();
    close(back.get::<kelvin>(), 300.0);
}

#[test]
fn dynamic_plain_ratio_is_not_a_uom_angle() {
    // A plain ratio has no radians, so it does not convert to uom's Angle;
    // the original comes back.
    let ratio = DynamicSIQuantity::from(1.0);
    let err = u::Angle::try_from(ratio).unwrap_err();
    let q = Dimensionless::<f64>::try_from(err).unwrap();
    close(f64::from(q), 1.0);
}

#[test]
fn uom_curvature_is_radians_per_metre() {
    use uom::si::curvature::radian_per_meter;
    // uom marks curvature with its angle kind: rad/m, not a plain 1/m.
    let q: Curvature<f64> = u::Curvature::new::<radian_per_meter>(0.25).into();
    close(f64::from(q / (1.0 * RADIAN / METRE)), 0.25);
    let back: u::Curvature = q.into();
    close(back.get::<radian_per_meter>(), 0.25);
    let dq: DynamicSIQuantity = u::Curvature::new::<radian_per_meter>(0.25).into();
    assert!(Curvature::<f64>::try_from(dq).is_ok());
    assert!(ReciprocalLength::<f64>::try_from(dq).is_err());
    // An arc length times its curvature is the angle it turns through.
    let turn: Angle<f64> = q * (4.0 * METRE);
    close(f64::from(turn / (1.0 * RADIAN)), 1.0);
}

#[test]
fn uom_angular_kinds_land_on_their_radians() {
    use uom::si::angular_absement::radian_second;
    use uom::si::angular_acceleration::radian_per_second_squared;
    use uom::si::angular_jerk::radian_per_second_cubed;
    use uom::si::angular_momentum::kilogram_square_meter_per_second;
    use uom::si::angular_velocity::radian_per_second;
    let w: AngularVelocity<f64> = u::AngularVelocity::new::<radian_per_second>(2.0).into();
    close(f64::from(w / (1.0 * RADIAN / SECOND)), 2.0);
    let a: AngularAcceleration<f64> =
        u::AngularAcceleration::new::<radian_per_second_squared>(3.0).into();
    close(f64::from(a / (1.0 * RADIAN / (SECOND * SECOND))), 3.0);
    let j: AngularJerk<f64> = u::AngularJerk::new::<radian_per_second_cubed>(4.0).into();
    close(
        f64::from(j / (1.0 * RADIAN / (SECOND * SECOND * SECOND))),
        4.0,
    );
    let s: AngularAbsement<f64> = u::AngularAbsement::new::<radian_second>(5.0).into();
    close(f64::from(s / (1.0 * RADIAN_SECOND)), 5.0);
    let l: AngularMomentum<f64> =
        u::AngularMomentum::new::<kilogram_square_meter_per_second>(6.0).into();
    close(
        f64::from(l / (1.0 * KILOGRAM * METRE * METRE / (SECOND * RADIAN))),
        6.0,
    );
    let back: u::AngularMomentum = l.into();
    close(back.get::<kilogram_square_meter_per_second>(), 6.0);
}
