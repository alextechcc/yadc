//! A unit is a type of its own, `SIUnit<D, C>`: its dimension and its
//! definition in const parameters. `quantity` builds, `value` reads, in any
//! number type its size converts to; an enum of them is a run-time choice,
//! and `DynamicSIUnit` a unit of any dimension.
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_si::dynamic::DynamicSIUnit;
use yadc_si::dynamic::mechanical::{LengthUnit, VelocityUnit};
use yadc_si::dynamic::thermal::ThermodynamicTemperatureUnit;
use yadc_si::mechanical::units::{
    DecibelMilliwatt, Foot, Inch, JulianDate, KilometrePerHour, Metre, MetrePerSecond, Mile,
    UnixSecond, Watt,
};
use yadc_si::mechanical::{Length, Power, PowerLevel};
use yadc_si::thermal::units::{Celsius, DegreeCelsius, Fahrenheit, KelvinScale};
use yadc_si::{Prefix, Prefixed};

#[test]
fn a_unit_builds_and_reads() {
    let d = Mile {}.quantity(3.0_f64);
    assert!((Metre {}.value(d) - 4828.032).abs() < 1e-9);
    assert!((Mile {}.value(d) - 3.0).abs() < 1e-15);
    let v = KilometrePerHour {}.quantity(36.0_f64);
    assert!((MetrePerSecond {}.value(v) - 10.0).abs() < 1e-12);
}

#[test]
fn any_number_type() {
    let d: Length<f32> = Inch {}.quantity(10.0_f32);
    assert!((Metre {}.value(d) - 0.254).abs() < 1e-6);
    // An integer counts whole metres.
    let k: Length<i32> = Metre {}.quantity(1609);
    assert_eq!(Metre {}.value(k), 1609);
}

/// Folded at compile time.
const MARATHON: Length<f64> = Metre {}.quantity(42195.0);

#[test]
fn const_units() {
    assert!((Mile {}.value(MARATHON) - 26.218757).abs() < 1e-6);
}

#[test]
fn zeros() {
    let t = Celsius {}.quantity(25.0_f64);
    assert!((KelvinScale {}.value(t) - 298.15).abs() < 1e-12);
    assert!((Fahrenheit {}.value(t) - 77.0).abs() < 1e-12);
    let i = JulianDate {}.quantity(2440587.5_f64);
    assert!(UnixSecond {}.value(i).abs() < 1e-6);
    let p: Power<f64> = PowerLevel::from(DecibelMilliwatt {}.quantity(30.0)).into();
    assert!((Watt {}.value(p) - 1.0).abs() < 1e-12);
    let _ = DegreeCelsius {};
}

#[test]
fn prints_and_parses() {
    assert_eq!(format!("{} / {:#}", Mile {}, Mile {}), "mi / miles");
    assert_eq!("ft".parse::<LengthUnit>(), Ok(LengthUnit::Foot));
    assert_eq!(LengthUnit::Foot.to_string(), "ft");
    assert_eq!(
        "km/h".parse::<VelocityUnit>(),
        Ok(VelocityUnit::KilometrePerHour)
    );
    let _ = Foot {};
}

#[test]
fn an_enum_is_a_run_time_choice() {
    let setting = LengthUnit::Inch;
    let d = Foot {}.quantity(1.0_f64);
    assert!((setting.value(d) - 12.0).abs() < 1e-12);
    let c = ThermodynamicTemperatureUnit::Celsius;
    assert!((c.value(KelvinScale {}.quantity(273.15_f64)) - 0.0).abs() < 1e-12);
}

#[test]
fn a_prefixed_unit() {
    let km = Prefixed {
        prefix: Some(Prefix::Kilo),
        unit: LengthUnit::Metre,
    };
    let d: Length<f64> = km.quantity(3.0);
    assert!((Metre {}.value(d) - 3000.0).abs() < 1e-9);
    assert_eq!(km.to_string(), "km");
}

/// The open level: a unit of any dimension.
#[test]
fn a_dynamic_unit() {
    let mile = DynamicSIUnit::from(Mile {});
    let metre = DynamicSIUnit::from(LengthUnit::Metre);
    let d = mile.quantity(Some(2.0));
    assert!((metre.value(d).unwrap() - 3218.688).abs() < 1e-9);
    assert_eq!(DynamicSIUnit::from(Celsius {}).value(d), None);
    assert!(Mile::try_from(mile).is_ok());
    assert_eq!(mile.to_string(), "mi");
}
