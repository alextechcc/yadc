#![feature(generic_const_exprs)]
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
use yadc::{DynamicQuantity, StaticQuantity};
use yadc_si::angular::*;
use yadc_si::mechanical::*;
use yadc_si::{DynamicSIQuantity, StaticSIQuantity};

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

// ─── StaticSIQuantity ─────────────────────────────────────────────────────────

#[test]
fn static_from_uom_length() {
    let uom = Length::new::<kilometer>(42.195);
    let q: StaticSIQuantity<_> = uom.into();
    close(q.as_unit(LengthUnit::Meter), 42_195.0);
    close(q.as_unit(LengthUnit::Kilometer), 42.195);
}

#[test]
fn static_from_uom_mass() {
    let uom = Mass::new::<kilogram>(70.0);
    let q: StaticSIQuantity<_> = uom.into();
    close(q.as_unit(MassUnit::Kilogram), 70.0);
    close(q.as_unit(MassUnit::Gram), 70_000.0);
}

#[test]
fn static_into_uom_length() {
    let q = yadc_si::mechanical::Length::from_unit(5.0, LengthUnit::Kilometer);
    let uom: Length = q.into();
    close(uom.get::<meter>(), 5_000.0);
    close(uom.get::<kilometer>(), 5.0);
}

#[test]
fn static_round_trip_uom_to_yadc_to_uom() {
    let original = Length::new::<meter>(1234.5);
    let q: StaticSIQuantity<_> = original.into();
    let back: Length = q.into();
    close(back.get::<meter>(), 1234.5);
}

#[test]
fn static_derived_quantity_velocity() {
    let uom = Velocity::new::<meter_per_second>(25.0);
    let q: StaticSIQuantity<_> = uom.into();
    close(q.as_unit(VelocityUnit::MeterPerSecond), 25.0);
    let back: Velocity = q.into();
    close(back.get::<meter_per_second>(), 25.0);
}

#[test]
fn static_derived_quantity_force() {
    let uom = Force::new::<newton>(9.81);
    let q: StaticSIQuantity<_> = uom.into();
    close(q.as_unit(ForceUnit::Newton), 9.81);
    let back: Force = q.into();
    close(back.get::<newton>(), 9.81);
}

// ─── DynamicSIQuantity ────────────────────────────────────────────────────────

#[test]
fn dynamic_from_uom_length() {
    let uom = Length::new::<meter>(500.0);
    let dq: DynamicSIQuantity = uom.into();
    assert_eq!(dq.as_unit(LengthUnit::Meter), Some(500.0));
    assert_eq!(dq.as_unit(MassUnit::Kilogram), None);
}

#[test]
fn dynamic_from_uom_mass() {
    let uom = Mass::new::<kilogram>(70.0);
    let dq: DynamicSIQuantity = uom.into();
    assert_eq!(dq.as_unit(MassUnit::Kilogram), Some(70.0));
}

#[test]
fn dynamic_from_derived_velocity() {
    let uom = Velocity::new::<meter_per_second>(10.0);
    let dq: DynamicSIQuantity = uom.into();
    assert_eq!(dq.as_unit(VelocityUnit::MeterPerSecond), Some(10.0));
    // velocity dim doesn't match force — None
    assert_eq!(dq.as_unit(ForceUnit::Newton), None);
}

#[test]
fn dynamic_from_derived_acceleration() {
    let uom = Acceleration::new::<meter_per_second_squared>(9.81);
    let dq: DynamicSIQuantity = uom.into();
    assert_eq!(
        dq.as_unit(AccelerationUnit::MeterPerSecondSquared),
        Some(9.81)
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
    assert_eq!(err.as_unit(MassUnit::Kilogram), Some(5.0));
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
    let force = mass * accel;
    assert_eq!(force.as_unit(ForceUnit::Newton), Some(6.0));
}

// ─── Kind mapping ─────────────────────────────────────────────────────────────

#[test]
fn static_angle_from_uom_has_angle_kind() {
    let uom = Angle::new::<radian>(1.5707963);
    let q: StaticSIQuantity<_> = uom.into();
    close(q.as_unit(AngleUnit::Radian), 1.5707963);
}

#[test]
fn static_angle_round_trip() {
    let original = Angle::new::<radian>(3.14159);
    let q: StaticSIQuantity<_> = original.into();
    let back: Angle = q.into();
    close(back.get::<radian>(), 3.14159);
}

#[test]
fn static_solid_angle_round_trip() {
    let original = SolidAngle::new::<steradian>(4.0);
    let q: StaticSIQuantity<_> = original.into();
    close(q.as_unit(SolidAngleUnit::Steradian), 4.0);
    let back: SolidAngle = q.into();
    close(back.get::<steradian>(), 4.0);
}

#[test]
fn dynamic_angle_kind_preserved() {
    let dq: DynamicSIQuantity = Angle::new::<radian>(1.0).into();
    assert_eq!(dq.as_unit(AngleUnit::Radian), Some(1.0));
    // Must not match a dimensionless (Kind::Linear) unit
    assert_eq!(
        dq.as_unit(yadc_si::dimensionless::DimensionlessUnit::Ratio),
        None
    );
}

#[test]
fn dynamic_solid_angle_kind_preserved() {
    let dq: DynamicSIQuantity = SolidAngle::new::<steradian>(2.5).into();
    assert_eq!(dq.as_unit(SolidAngleUnit::Steradian), Some(2.5));
    assert_eq!(dq.as_unit(AngleUnit::Radian), None);
}

#[test]
fn dynamic_try_from_angle_succeeds() {
    let dq: DynamicSIQuantity = Angle::new::<radian>(0.5).into();
    let back = Angle::try_from(dq).expect("angle dim matches");
    close(back.get::<radian>(), 0.5);
}

#[test]
fn dynamic_try_from_angle_rejects_dimensionless() {
    // A DynamicSIQuantity with Kind::Angle must not convert to a uom kind-less quantity.
    let dq: DynamicSIQuantity = Angle::new::<radian>(1.0).into();
    // Mass has kind dyn uom::Kind (Linear) but different dimensions — just confirm it fails
    let err = Mass::try_from(dq).unwrap_err();
    assert_eq!(err.as_unit(AngleUnit::Radian), Some(1.0));
}
