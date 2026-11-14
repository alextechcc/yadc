//! The SI system at work: a number times a unit constant is a quantity
//! (`2.0 * KILOMETRE`), `From` takes it to the base units of its dimension
//! and back, the static quantities compose with typed operators, and the
//! open type (`DynamicSIQuantity`) decides at run time. A unit enum is a
//! choice among a quantity's units (`LengthUnit`), each an `SIUnit`.
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![feature(const_trait_impl, const_ops)]
#![allow(incomplete_features)]

use yadc_si::angular as ang;
use yadc_si::chemistry as chem;
use yadc_si::dimensionless::*;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::mechanical::{InstantUnit, LengthUnit, MassUnit, PowerUnit, VelocityUnit};
use yadc_si::dynamic::thermal::ThermodynamicTemperatureUnit;
use yadc_si::electromagnetic as em;
use yadc_si::fluid;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;
use yadc_si::*;

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

/// A quantity in a unit that is not a base unit is `n` of it: compared with
/// quantities of the same unit, within a relative 1e-9 - the reading for
/// units with a zero or a log scale, which have no quotient.
fn near<const U: SIUnit>(q: SIQuantity<U, f64, false>, n: f64) {
    let one = SIQuantity::<U, Exact, false>::ONE;
    let tol = 1e-9 * n.abs().max(1.0);
    assert!(
        q > (n - tol) * one && q < (n + tol) * one,
        "expected {n} {U}, got {q}"
    );
}

/// The number of kelvins a temperature is.
fn kelvins(t: ThermodynamicTemperature<f64>) -> f64 {
    f64::from(t / (1.0 * KELVIN))
}

/// The number of kilograms a mass is.
fn kilograms(m: Mass<f64>) -> f64 {
    f64::from(m / (1.0 * KILOGRAM))
}

#[test]
fn quantity_and_value_round_trip() {
    let mass: Mass<f64> = (1500.0_f64 * GRAM).into();
    close(kilograms(mass), 1.5);
    let grams: Mass<f64, { GRAM.unit() }, false> = mass.into();
    close(f64::from(grams / (1.0 * GRAM)), 1500.0);
    // The international pound, exactly.
    let m: Mass<f64> = (10.0_f64 * POUND).into();
    close(kilograms(m), 4.5359237);
}

#[test]
fn units_build_quantities_and_quantities_compose() {
    // Quantities in base units compose; a unit that is not one converts first.
    let d: Length<f64> = (90.0_f64 * KILOMETRE).into();
    let hour: Time<f64> = (1.0_f64 * HOUR).into();
    let v: Velocity<f64> = d / hour;
    close(f64::from(v / (1.0 * METRE / SECOND)), 25.0);
    // A quotient of units is a unit, and a conversion's target.
    let kmh: Velocity<f64, { KILOMETRE.unit().times(HOUR.unit(), -1) }, false> = v.into();
    close(f64::from(kmh / (1.0 * KILOMETRE / HOUR)), 90.0);
    close(
        f64::from((1.0 * METRE) / (1.0 * SECOND) / (1.0 * METRE / SECOND)),
        1.0,
    );
    let back: Length<f64, { KILOMETRE.unit() }, false> = (v * hour).into();
    near(back, 90.0);
    let miles: Length<f64, { MILE.unit() }, false> = d.into();
    near(miles, 90.0 / 1.609344);
}

#[test]
fn dynamic_read_is_checked() {
    let m = DynamicSIQuantity::from(2.5 * KILOGRAM);
    // Read through the static quantity: `TryFrom` is the check.
    let mass: Mass<f64> = Mass::<f64>::try_from(m).unwrap();
    let grams: Mass<f64, { GRAM.unit() }, false> = mass.into();
    close(f64::from(grams / (1.0 * GRAM)), 2500.0);
    // The wrong quantity reads as an error, handing the quantity back.
    assert_eq!(Force::<f64>::try_from(m), Err(m));
}

#[test]
fn mul_and_div_compose_dims() {
    let m: Mass<f64> = 2.0 * KILOGRAM;
    let a: Acceleration<f64> = 3.0 * METRE / (SECOND * SECOND);
    let f: Force<f64> = m * a;
    close(f64::from(f / (1.0 * NEWTON)), 6.0);

    // A quotient of one dimension is a plain number.
    let ratio: Dimensionless<f64> = (10.0 * NEWTON) / (2.0 * NEWTON);
    close(f64::from(ratio), 5.0);
    let percent: Dimensionless<f64, { PERCENT.unit() }, false> = ratio.into();
    near(percent, 500.0);
}

/// Temperatures and their differences are counted in kelvins alike; a unit
/// with a zero (°C, °F) converts to kelvins first.
#[test]
fn temperatures_subtract_and_add_in_kelvins() {
    let hot: ThermodynamicTemperature<f64> = (100.0_f64 * CELSIUS).into();
    let cold: ThermodynamicTemperature<f64> = (20.0_f64 * CELSIUS).into();
    let delta: TemperatureInterval<f64> = hot - cold;
    close(f64::from(delta / (1.0 * KELVIN)), 80.0);
    close(f64::from(delta / (1.0 * DEGREE_CELSIUS)), 80.0);
    let in_f: TemperatureInterval<f64, { DEGREE_FAHRENHEIT.unit() }, false> = delta.into();
    near(in_f, 144.0);
    // An interval moves a temperature.
    let rise: TemperatureInterval<f64> = 5.0 * DEGREE_CELSIUS;
    let hotter: SIQuantity<{ CELSIUS.unit() }, f64, false> = (cold + rise).into();
    near(hotter, 25.0);
    // Intervals add.
    close(f64::from((rise + rise) / (1.0 * KELVIN)), 10.0);
}

#[test]
fn round_trip_celsius_fahrenheit_kelvin() {
    // The zero point is the unit's business: 25 °C is 298.15 K, 77 °F.
    let t: ThermodynamicTemperature<f64> = (25.0_f64 * CELSIUS).into();
    close(kelvins(t), 298.15);
    let f: SIQuantity<{ FAHRENHEIT.unit() }, f64, false> = t.into();
    near(f, 77.0);
    // ... and back: 77 °F is 25 °C.
    let k: ThermodynamicTemperature<f64> = (77.0_f64 * FAHRENHEIT).into();
    let c: SIQuantity<{ CELSIUS.unit() }, f64, false> = k.into();
    near(c, 25.0);
    // Nine degrees Fahrenheit of difference are 5 K.
    let step: TemperatureInterval<f64> = (9.0_f64 * DEGREE_FAHRENHEIT).into();
    close(f64::from(step / (1.0 * KELVIN)), 5.0);
}

#[test]
fn angle_is_not_dimensionless() {
    // An angle is counted in radians, a base unit; a ratio has no unit.
    let half_circle: ang::Angle<f64> = (180.0_f64 * ang::DEGREE).into();
    close(
        f64::from(half_circle / (1.0 * RADIAN)),
        std::f64::consts::PI,
    );
    assert!(!RADIAN.unit().same_dimension(RATIO.unit()));
    // Angle / angle is a plain number: the radians cancel.
    let twice: ang::Angle<f64> = (360.0_f64 * ang::DEGREE).into();
    let n: Dimensionless<f64> = twice / half_circle;
    close(f64::from(n), 2.0);
}

#[test]
fn radians_are_a_base_unit() {
    let radian: ang::Angle<f64> = 1.0 * RADIAN;
    // Torque is J/rad, so torque × angle is energy (the work of a turn).
    let torque: Torque<f64> = 10.0 * NEWTON_METRE;
    let turn: ang::Angle<f64> = (1.0_f64 * ang::REVOLUTION).into();
    let work: Energy<f64> = torque * turn;
    close(f64::from(work / (1.0 * JOULE)), 20.0 * std::f64::consts::PI);
    // Angular speed × time is an angle.
    let rpm: ang::AngularVelocity<f64> = (60.0_f64 * (ang::REVOLUTION / MINUTE)).into();
    let turned: ang::Angle<f64> = rpm * (1.0 * SECOND);
    let in_rev: ang::Angle<f64, { ang::REVOLUTION.unit() }, false> = turned.into();
    near(in_rev, 1.0);
    // Angle² is a solid angle.
    let patch: ang::SolidAngle<f64> = (0.1 * RADIAN) * (0.2 * RADIAN);
    close(f64::from(patch / (1.0 * ang::STERADIAN)), 0.02);
    // Geometry says where a radian is dropped: arc length s = r·θ / rad, and
    // rim speed v = ω·r / rad.
    let r: Length<f64> = 2.0 * METRE;
    let arc: Length<f64> = r * (1.5 * RADIAN) / radian;
    close(f64::from(arc / (1.0 * METRE)), 3.0);
    let omega: ang::AngularVelocity<f64> = 3.0 * RADIAN / SECOND;
    let rim: Velocity<f64> = omega * r / radian;
    close(f64::from(rim / (1.0 * METRE / SECOND)), 6.0);
}

#[test]
fn add_assign_and_sub_assign() {
    let mut m: Mass<f64> = 1.0 * KILOGRAM;
    m += Mass::<f64>::from(500.0_f64 * GRAM);
    close(kilograms(m), 1.5);
    m -= Mass::<f64>::from(250.0_f64 * GRAM);
    close(kilograms(m), 1.25);
}

#[test]
fn quantity_convenience_ops() {
    let zero: Mass<f64> = 0.0 * KILOGRAM;
    close(kilograms(zero), 0.0);

    let debt: Mass<f64> = -Mass::<f64>::from(500.0_f64 * GRAM);
    close(kilograms(debt), -0.5);

    assert!(2.0 * KILOGRAM > 1.0 * KILOGRAM);
    // Quantities of one unit compare, a unit with a zero too.
    assert!(25.0 * CELSIUS > 20.0 * CELSIUS);

    let addend: Mass<f64> = (250.0_f64 * GRAM).into();
    let mut borrowed: Mass<f64> = 1.0 * KILOGRAM;
    borrowed += &addend;
    close(kilograms(borrowed), 1.25);
    borrowed -= &addend;
    close(kilograms(borrowed), 1.0);

    let mut scaled: Mass<f64> = 2.0 * KILOGRAM;
    scaled *= 0.5;
    close(kilograms(scaled), 1.0);
    scaled /= 0.25;
    close(kilograms(scaled), 4.0);
    // A plain number's quantity scales too.
    let half: Dimensionless<f64> = (50.0_f64 * PERCENT).into();
    close(kilograms(scaled * half), 2.0);
}

/// Bels, decibels and nepers are log units of the plain number: `From`
/// takes them to the ratio and back.
#[test]
fn decibels_are_a_log_unit_of_the_plain_number() {
    // A ratio of about 2: 3 dB.
    let r: Dimensionless<f64> = (3.0_f64 * DECIBEL).into();
    close(f64::from(r), 10f64.powf(0.3));
    let db: SIQuantity<{ DECIBEL.unit() }, f64, false> = r.into();
    near(db, 3.0);
    let ten: Dimensionless<f64> = (1.0_f64 * BEL).into();
    close(f64::from(ten), 10.0);
    // A neper is a field ratio: 1 Np is a power ratio of e².
    let np: Dimensionless<f64> = (1.0_f64 * NEPER).into();
    assert!((f64::from(np) - std::f64::consts::E.powi(2)).abs() < 1e-12);
    let in_db: SIQuantity<{ DECIBEL.unit() }, f64, false> = np.into();
    near(in_db, 20.0 / std::f64::consts::LN_10);
    let thousand: Dimensionless<f64> = 1000.0 * RATIO;
    near(
        SIQuantity::<{ DECIBEL.unit() }, f64, false>::from(thousand),
        30.0,
    );
    // Numbers of one log unit add: adding decibels multiplies the ratios.
    let g = 3.0_f64 * DECIBEL + 7.0_f64 * DECIBEL;
    close(f64::from(Dimensionless::<f64>::from(g)), 10.0);
    assert_eq!(g.to_string(), "10 dB");
}

/// dBm and dBW are log units of power.
#[test]
fn power_levels_are_log_units_of_power() {
    let level = 30.0_f64 * DECIBEL_MILLIWATT;
    let p: Power<f64> = level.into();
    close(f64::from(p / (1.0 * WATT)), 1.0);
    near(
        SIQuantity::<{ DECIBEL_WATT.unit() }, f64, false>::from(p),
        0.0,
    );
    let tenth: Power<f64> = 0.1 * WATT;
    near(
        SIQuantity::<{ DECIBEL_MILLIWATT.unit() }, f64, false>::from(tenth),
        20.0,
    );
    // A link budget: numbers of dBm add, multiplying the power.
    let received = level - 3.0 * DECIBEL_MILLIWATT + 13.0 * DECIBEL_MILLIWATT;
    near(received, 40.0);
    close(f64::from(Power::<f64>::from(received) / (1.0 * WATT)), 10.0);
    // Doubling the watts is 3.01 dB.
    let doubled: SIQuantity<{ DECIBEL_MILLIWATT.unit() }, f64, false> = (2.0 * p).into();
    near(doubled, 30.0 + 10.0 * 2f64.log10());
    // The unit enum's values are the constants' units.
    assert_eq!(
        SIUnit::from(PowerUnit::DecibelMilliwatt),
        DECIBEL_MILLIWATT.unit()
    );
    assert_eq!(
        format!("{} {}", PowerUnit::DecibelMilliwatt, PowerUnit::DecibelWatt),
        "dBm dBW"
    );
    close(
        SIUnit::from(PowerUnit::DecibelWatt).from_base(DECIBEL_MILLIWATT.unit().to_base(30.0)),
        0.0,
    );

    // The dynamic type: numbers of one log unit add and scale; a log unit
    // has no products.
    let d = DynamicSIQuantity::from(level);
    assert_eq!(
        SIQuantity::<{ DECIBEL_MILLIWATT.unit() }, f64, false>::try_from(d),
        Ok(level)
    );
    assert!(Power::<f64>::try_from(d).is_err());
    assert_eq!((d + d).map(|s| s.value), Some(60.0));
    assert_eq!((d * 2.0).map(|s| s.value), Some(60.0));
    assert_eq!(d * d, None);
}

#[test]
fn display_quantity() {
    let p: Power<f64> = 100.0 * WATT;
    // The number with the caller's flags, then the unit's symbol.
    assert_eq!(format!("{p}"), "100 W");
    assert_eq!(format!("{p:.2}"), "100.00 W");
    assert_eq!(format!("{p:>8.1}|"), "   100.0 W|");
    let level: SIQuantity<{ DECIBEL_WATT.unit() }, f64, false> = p.into();
    assert_eq!(format!("{level:.0}"), "20 dBW");
    // A coherent unit without a name prints as its base units.
    let v: Velocity<f64> = 2.0 * METRE / SECOND;
    assert_eq!(format!("{v}"), "2 m·s^-1");
}

#[test]
fn display_an_open_set_quantity_after_try_from() {
    let dq = DynamicSIQuantity::from(100.0 * WATT);
    let p = Power::<f64>::try_from(dq).expect("dim matches");
    assert_eq!(p.to_string(), "100 W");
    assert!(Mass::<f64>::try_from(dq).is_err());
}

#[test]
fn static_dynamic_bridge_round_trips_and_keeps_the_original_on_failure() {
    let dq = DynamicSIQuantity::from(2.5 * KILOGRAM);
    let mass: Mass<f64> = dq.try_into().expect("dim matches");
    close(kilograms(mass), 2.5);
    // The failure carries the original quantity back, not `()`.
    let err: DynamicSIQuantity = Force::<f64>::try_from(dq).unwrap_err();
    close(kilograms(Mass::<f64>::try_from(err).unwrap()), 2.5);
}

#[test]
fn dynamic_arithmetic_is_partial() {
    let distance: Length<f64> = (90.0_f64 * KILOMETRE).into();
    let one_km: Length<f64> = (1.0_f64 * KILOMETRE).into();
    let hour: Time<f64> = (1.0_f64 * HOUR).into();
    let d = DynamicSIQuantity::from(distance);
    let h = DynamicSIQuantity::from(hour);
    let km = DynamicSIQuantity::from(one_km);
    // The open type composes units at run time: every step may have no
    // result.
    let v = (d / (h * 2.0).unwrap()).unwrap();
    let v = Velocity::<f64>::try_from(v).unwrap();
    close(f64::from(v / (1.0 * METRE / SECOND)), 12.5);

    // `m + s` is ill-formed: the operator has no result ...
    assert_eq!(d + h, None);
    // ... so a chain stops at the first rejected step, with `?` ...
    let rejected = || -> Option<DynamicSIQuantity> { ((d + h)? * d)? / km };
    assert_eq!(rejected(), None);
    // ... while a well-formed chain goes through.
    let accepted = || -> Option<DynamicSIQuantity> { (((d * d)? / km)? + d)? - d };
    let total = accepted()
        .and_then(|x| Length::<f64>::try_from(x).ok())
        .unwrap();
    close(f64::from(total / (1.0 * METRE)), 8_100_000.0);
    // A read of the wrong quantity rejects it.
    assert!(f64::try_from(d).is_err());
    // A base unit and another unit do not mix: convert first.
    assert_eq!(d / DynamicSIQuantity::from(1.0 * HOUR), None);

    // Temperatures in kelvins are ordinary amounts: they scale, add, and
    // multiply.
    let t = DynamicSIQuantity::from(300.0 * KELVIN);
    assert!((t * 2.0).is_some_and(|q| ThermodynamicTemperature::<f64>::try_from(q).is_ok()));
    assert!((t / 2.0).is_some_and(|q| ThermodynamicTemperature::<f64>::try_from(q).is_ok()));
    assert!((t + t).is_some_and(|q| ThermodynamicTemperature::<f64>::try_from(q).is_ok()));
    assert!((t * km).is_some_and(|q| ThermodynamicTemperature::<f64>::try_from(q).is_err()));
    // A unit with a zero has no products.
    let c = DynamicSIQuantity::from(25.0 * CELSIUS);
    assert_eq!(c * km, None);
    assert!(ThermodynamicTemperature::<f64>::try_from(c).is_err());
    near(
        SIQuantity::<{ CELSIUS.unit() }, f64, false>::try_from(c).unwrap(),
        25.0,
    );
}

/// The number `q` is in `unit`, where `unit` measures `q`'s dimension.
fn read(unit: SIUnit, q: DynamicSIQuantity) -> Option<f64> {
    unit.same_dimension(q.unit)
        .then(|| unit.from_base(q.unit.to_base(q.value)))
}

/// A unit of any dimension, chosen at run time, reads the open type, and
/// reads `None` for another dimension.
#[test]
fn a_unit_chosen_at_run_time_reads_the_open_type() {
    let mile = SIUnit::from(LengthUnit::Mile);
    let mph = (MILE / HOUR).unit();
    let d = DynamicSIQuantity::from(Length::<f64>::from(60.0_f64 * MILE));
    let t = DynamicSIQuantity::from(Time::<f64>::from(1.5_f64 * HOUR));
    let v = (d / t).unwrap();
    close(read(mph, v).unwrap(), 40.0);
    assert_eq!(read(mile, v), None);
    close(read(mile, d).unwrap(), 60.0);
    assert_eq!(mph.to_string(), "h^-1·mi");
    assert_eq!(mile, MILE.unit());
}

#[test]
fn a_temperature_is_an_amount() {
    let cold: ThermodynamicTemperature<f64> = (20.0_f64 * CELSIUS).into();
    let hot: ThermodynamicTemperature<f64> = (100.0_f64 * CELSIUS).into();
    let rise = hot - cold;
    let warmer = cold + rise / 2.0;
    near(
        SIQuantity::<{ CELSIUS.unit() }, f64, false>::from(warmer),
        60.0,
    );
    near(
        SIQuantity::<{ FAHRENHEIT.unit() }, f64, false>::from(warmer),
        140.0,
    );
    close(kelvins(warmer), 333.15);
    // In place.
    let mut t = cold;
    t += rise;
    t -= rise / 4.0;
    near(SIQuantity::<{ CELSIUS.unit() }, f64, false>::from(t), 80.0);
    assert!(cold < hot);
    // It averages and scales, in kelvins.
    close(kelvins((cold + hot) / 2.0), 333.15);
    close(kelvins(2.0 * warmer), 666.3);
    // It multiplies: the ideal gas law, p V = n R T, and a fourth power.
    let r: chem::MolarHeatCapacity<f64> = (1.0_f64 * chem::MOLAR_GAS_CONSTANT).into();
    let n_r: HeatCapacity<f64> = (1.0 * MOLE) * r;
    let energy: Energy<f64> = n_r * warmer;
    close(f64::from(energy / (1.0 * JOULE)), 8.314462618 * 333.15);
    let fourth = warmer * warmer * warmer * warmer;
    assert_eq!(fourth.unit(), SIUnit::ONE.times(KELVIN.unit(), 4));
    close(f64::from(hot / cold), 373.15 / 293.15);
}

/// m·s⁻², spelled as a product of base units.
const ACCELERATION_UNIT: SIUnit = METRE.unit().times(SECOND.unit(), -2);

/// A unit's own operators decide what has a sum and a product: one unit
/// adds to itself; units multiply unless one has a zero or a log scale.
#[test]
fn units_decide_sums_and_products() {
    let (k, m, c) = (KELVIN.unit(), METRE.unit(), CELSIUS.unit());
    assert_eq!(k + k, Some(k));
    assert_eq!(k - k, Some(k));
    assert_eq!(k + m, None);
    assert_eq!(c + c, Some(c));
    assert_eq!(c + k, None);
    assert_eq!(k * m, Some(m.times(k, 1)));
    assert_eq!(m / k, Some(m.times(k, -1)));
    assert_eq!(c * m, None);
    assert_eq!(c / SIUnit::ONE, None);
    assert_eq!(KILOGRAM.unit() * ACCELERATION_UNIT, Some(NEWTON.unit()));
    let (dbw, w) = (DECIBEL_WATT.unit(), WATT.unit());
    assert_eq!(dbw * SIUnit::ONE, None);
    assert_eq!(dbw + w, None);
    assert_eq!(dbw + dbw, Some(dbw));
    // Every linear unit has a reciprocal: one over an angle is rad⁻¹.
    assert_eq!(
        SIUnit::ONE / RADIAN.unit(),
        Some(SIUnit::ONE.times(RADIAN.unit(), -1))
    );
    // Dimensions: a unit and the base units it is defined by.
    assert!(MILE.unit().same_dimension(m));
    assert!(c.same_dimension(k));
    assert!(dbw.same_dimension(w));
    assert!(!DECIBEL.unit().same_dimension(w));
}

/// Instants are counted in seconds of Unix time; GPS seconds and Julian
/// dates count from their own zeros on the same line.
#[test]
fn instants_count_from_their_zeros() {
    let launch: Instant<f64> = 1_700_000_000.0 * UNIX_SECOND;
    let landing = launch + Time::<f64>::from(2.0_f64 * HOUR);
    let flight: Time<f64> = landing - launch;
    near(Time::<f64, { HOUR.unit() }, false>::from(flight), 2.0);
    close(
        f64::from((landing - 30.0 * SECOND) / (1.0 * UNIX_SECOND)),
        1_700_007_170.0,
    );
    assert!(launch < landing);
    let mut t = launch;
    t += Time::<f64>::from(1.0_f64 * DAY);
    t -= Time::<f64>::from(1.0_f64 * HOUR);
    near(Time::<f64, { HOUR.unit() }, false>::from(t - launch), 23.0);

    // One line, many units: each counts from its own zero on it.
    let gps_epoch: Instant<f64> = (0.0_f64 * GPS_SECOND).into();
    let unix_epoch: Instant<f64> = 0.0 * UNIX_SECOND;
    close(f64::from(gps_epoch / (1.0 * UNIX_SECOND)), 315_964_800.0);
    near(
        Instant::<f64, { GPS_SECOND.unit() }, false>::from(unix_epoch),
        -315_964_800.0,
    );
    near(
        Instant::<f64, { JULIAN_DATE.unit() }, false>::from(unix_epoch),
        2_440_587.5,
    );
    near(
        Instant::<f64, { MODIFIED_JULIAN_DATE.unit() }, false>::from(unix_epoch),
        40_587.0,
    );
    near(
        Instant::<f64, { MODIFIED_JULIAN_DATE.unit() }, false>::from(gps_epoch),
        44_244.0,
    );
    // J2000.0, noon on 1 January 2000, is JD 2451545.0 and Unix 946728000.
    let j2000: Instant<f64> = (2_451_545.0_f64 * JULIAN_DATE).into();
    close(f64::from(j2000 / (1.0 * UNIX_SECOND)), 946_728_000.0);
    near(
        Instant::<f64, { GPS_SECOND.unit() }, false>::from(j2000),
        946_728_000.0 - 315_964_800.0,
    );
    near(
        Time::<f64, { DAY.unit() }, false>::from(j2000 - gps_epoch),
        7_300.5,
    );
    // Seconds as `f64` resolve today's instants to a fraction of a
    // microsecond, so a millisecond count survives.
    let precise: Instant<f64> = (1_700_000_000_123.0_f64 * UNIX_MILLISECOND).into();
    assert!((f64::from(precise / (1.0 * UNIX_SECOND)) - 1_700_000_000.123).abs() < 1e-6);
    // A linear unit's conversion is `const`, its zero and all.
    close(EPOCH_JD, 2_440_587.5);

    // The unit enum: a menu of them, printed and parsed as the unit.
    let mjd = SIUnit::from(InstantUnit::ModifiedJulianDate);
    close(
        mjd.from_base(SIUnit::from(InstantUnit::JulianDate).to_base(2_400_000.5)),
        0.0,
    );
    assert_eq!(
        format!(
            "{} / {}",
            InstantUnit::UnixMillisecond,
            InstantUnit::JulianDate
        ),
        "ms Unix / JD"
    );
    assert_eq!("MJD".parse(), Ok(InstantUnit::ModifiedJulianDate));
    assert_eq!(InstantUnit::default(), InstantUnit::UnixSecond);
}

/// The Julian date of the Unix epoch, in a `const` item.
const EPOCH_JD: f64 = JULIAN_DATE.unit().from_coherent(0.0);

/// A static quantity in base units, built and read by generic code: a unit
/// chosen at run time, an `SIUnit`, converts its numbers to and from base
/// units.
trait InBase: Sized {
    /// `n` base units.
    fn of(n: f64) -> Self;
    /// The number of base units it is.
    fn number(self) -> f64;
}

impl<const U: SIUnit> InBase for SIQuantity<U, f64, true> {
    fn of(n: f64) -> Self {
        let q = DynamicSIQuantity {
            unit: U,
            is_base: true,
            value: n,
        };
        Self::try_from(q).unwrap()
    }
    fn number(self) -> f64 {
        DynamicSIQuantity::from(self).value
    }
}

fn number_of<Q: InBase>(unit: impl Into<SIUnit>, q: Q) -> f64 {
    unit.into().from_base(q.number())
}
fn from_number<Q: InBase>(unit: impl Into<SIUnit>, n: f64) -> Q {
    Q::of(unit.into().to_base(n))
}

#[test]
fn generic_code_reads_and_builds_through_a_unit() {
    use ThermodynamicTemperatureUnit::{Celsius, Kelvin};
    let t: ThermodynamicTemperature<f64> = from_number(Kelvin, 300.0);
    close(number_of(Celsius, t), 26.85);
    close(number_of(Kelvin, t), 300.0);
    close(number_of(FAHRENHEIT.unit(), t), 80.33);
    let m: Mass<f64> = from_number(MassUnit::Kilogram, 2.5);
    close(number_of(MassUnit::Gram, m), 2500.0);
    // Units picked at run time.
    let units: [SIUnit; 3] = [
        MassUnit::Kilogram.into(),
        MassUnit::Pound.into(),
        OUNCE.unit(),
    ];
    close(number_of(units[0], m), 2.5);
    close(number_of(units[2], m) / number_of(units[1], m), 16.0);
}

#[test]
fn units_picked_at_run_time() {
    // Some unit of length: each prints its symbol.
    let picks: [SIUnit; 4] = [
        LengthUnit::Metre.into(),
        LengthUnit::Kilometre.into(),
        LengthUnit::Mile.into(),
        FOOT.unit(),
    ];
    let d: Length<f64> = from_number(picks[1], 3.0);
    let shown: Vec<String> = picks
        .iter()
        .map(|u| format!("{:.1} {}", number_of(*u, d), u))
        .collect();
    assert_eq!(shown, ["3000.0 m", "3.0 km", "1.9 mi", "9842.5 ft"]);

    // Some quantity, decided at run time: the open type, read back through a
    // static quantity.
    let masses: [SIUnit; 2] = [MassUnit::Kilogram.into(), MassUnit::Gram.into()];
    let dq = DynamicSIQuantity::from(Mass::<f64>::from(10.0_f64 * POUND));
    let m = Mass::<f64>::try_from(dq).unwrap();
    let labels: Vec<String> = masses.iter().map(|u| u.to_string()).collect();
    assert_eq!(labels, ["kg", "g"]);
    assert_eq!(
        format!("{:.1} {}", number_of(masses[1], m), masses[1]),
        "4535.9 g"
    );
    // The wrong quantity hands the open-set quantity back.
    assert_eq!(Length::<f64>::try_from(dq), Err(dq));
}

/// The Beaufort wind scale: a unit of one's own that is a formula, not a
/// factor - `v = 0.836 B^1.5` m/s.
struct Beaufort;

impl Beaufort {
    fn velocity(b: f64) -> Velocity<f64> {
        0.836 * b.powf(1.5) * METRE / SECOND
    }
    fn number(v: Velocity<f64>) -> f64 {
        (f64::from(v / (1.0 * METRE / SECOND)) / 0.836).powf(2.0 / 3.0)
    }
}

#[test]
fn function_units_mix_with_linear_ones() {
    let gale = Beaufort::velocity(5.0);
    close(
        f64::from(gale / (1.0 * METRE / SECOND)),
        0.836 * 5f64.powf(1.5),
    );
    close(Beaufort::number(gale), 5.0);
    // Any unit of velocity reads it, and it reads any.
    let in_knots: Velocity<f64, { KNOT.unit() }, false> = gale.into();
    close(Beaufort::number(in_knots.into()), 5.0);
    let twenty_knots: Velocity<f64> = (20.0_f64 * KNOT).into();
    close(
        Beaufort::number(twenty_knots),
        (20.0 * 1852.0 / 3600.0 / 0.836f64).powf(2.0 / 3.0),
    );
    // ... and from the open type, through the static quantity.
    let d = DynamicSIQuantity::from(Beaufort::velocity(3.0));
    close(Beaufort::number(Velocity::<f64>::try_from(d).unwrap()), 3.0);
    let mass = DynamicSIQuantity::from(1.0 * KILOGRAM);
    assert!(Velocity::<f64>::try_from(mass).is_err());
}

#[test]
fn every_generated_unit_reaches_the_open_set() {
    let pounds = 10.0 * POUND;
    let dq = DynamicSIQuantity::from(pounds);
    // Back to its own unit, then through the static path to kilograms.
    let back = SIQuantity::<{ POUND.unit() }, f64, false>::try_from(dq);
    assert_eq!(back, Ok(pounds));
    close(kilograms(back.unwrap().into()), kilograms(pounds.into()));
    // A mismatch hands the original back: pounds are not kilograms.
    assert_eq!(Mass::<f64>::try_from(dq), Err(dq));
    assert_eq!(Force::<f64>::try_from(dq), Err(dq));

    // Zeros: the zero point is the unit's business on the dynamic side too.
    let t = DynamicSIQuantity::from(25.0 * CELSIUS);
    close(read(KELVIN.unit(), t).unwrap(), 298.15);
}

#[test]
fn reciprocals_are_one_over() {
    // One over a time is a frequency.
    let f: Frequency<f64> = (1.0 * RATIO) / (2.0 * SECOND);
    close(f64::from(f / (1.0 * HERTZ)), 0.5);
    // "Per hour": one over an hour, times an hour, is one.
    let hour: Time<f64> = (1.0_f64 * HOUR).into();
    let per_hour: Frequency<f64> = (1.0 * RATIO) / hour;
    close(f64::from(per_hour * hour), 1.0);
    close(f64::from(per_hour / (1.0 * HERTZ)), 1.0 / 3600.0);
    // One over an angle times an angle is a plain number.
    let per_rad = (1.0 * RATIO) / (2.0 * RADIAN);
    close(f64::from(per_rad * (1.0 * RADIAN)), 0.5);

    // The open type: a bare `f64` is a plain number's quantity.
    let d = (DynamicSIQuantity::from(1.0) / DynamicSIQuantity::from(4.0 * METRE)).unwrap();
    let one_metre = DynamicSIQuantity::from(1.0 * METRE);
    assert_eq!(f64::try_from((d * one_metre).unwrap()), Ok(0.25));
    let per_rad = (DynamicSIQuantity::from(1.0) / DynamicSIQuantity::from(2.0 * RADIAN)).unwrap();
    assert!(f64::try_from(per_rad).is_err());
}

// --- Compile-time units ---------------------------------------------------------

// A unit constant is a quantity counted in `Exact`: products of them, and
// numbers times them, are `const`.
const LIMIT: SIQuantity<{ KILOMETRE.unit().times(HOUR.unit(), -1) }, f64, false> =
    50.0 * (KILOMETRE / HOUR);
const ELECTRON_REST_ENERGY: SIQuantity<
    { ELECTRON_MASS.unit().times(SPEED_OF_LIGHT.unit(), 2) },
    Exact,
    false,
> = ELECTRON_MASS * SPEED_OF_LIGHT * SPEED_OF_LIGHT;
const BOILING: SIQuantity<{ CELSIUS.unit() }, f64, false> = 100.0 * CELSIUS;

#[test]
fn const_items_build() {
    let v: Velocity<f64> = LIMIT.into();
    close(f64::from(v / (1.0 * METRE / SECOND)), 50.0 / 3.6);
    // E = mc² for the electron: 8.187e-14 J (0.511 MeV), exact until read.
    let joules = f64::from(Energy::<f64>::from(1.0 * ELECTRON_REST_ENERGY) / (1.0 * JOULE));
    assert!((joules - 8.1871057769e-14).abs() < 1e-23, "got {joules}");
    let boiling: ThermodynamicTemperature<f64> = BOILING.into();
    close(kelvins(boiling), 373.15);
    near(
        SIQuantity::<{ FAHRENHEIT.unit() }, f64, false>::from(boiling),
        212.0,
    );
}

/// A unit's zero, in base units: what 0 of it is.
const fn zero(unit: SIUnit) -> Exact {
    unit.to_coherent(Exact::ZERO)
}

#[test]
fn units_carry_their_exact_definitions() {
    // Exact, by the 1959 agreement.
    assert_eq!(POUND.unit().size(), Exact::dec(45359237, -8));
    assert_eq!(zero(POUND.unit()), Exact::ZERO);
    close(kilograms((1.0_f64 * POUND).into()), 0.45359237);
    assert_eq!(WATT_HOUR.unit().size(), Exact::int(3600));
    close(
        f64::from(Dimensionless::<f64>::from(1.0_f64 * PERCENT)),
        0.01,
    );
    // A temperature unit is a size and a zero: °C counts kelvin-sized steps
    // from the ice point, °F steps of 5/9 K from 459.67 °F below it.
    assert_eq!(CELSIUS.unit().size(), Exact::ONE);
    assert_eq!(zero(CELSIUS.unit()), Exact::dec(27315, -2));
    assert_eq!(
        DEGREE_FAHRENHEIT.unit().size(),
        Exact::int(5) / Exact::int(9)
    );
    assert_eq!(
        zero(FAHRENHEIT.unit()),
        Exact::dec(45967, -2) * Exact::int(5) / Exact::int(9)
    );
    near(
        SIQuantity::<{ FAHRENHEIT.unit() }, f64, false>::from(0.0 * KELVIN),
        -459.67,
    );
    // Modules are namespaces: a minute of time and a minute of arc coexist.
    assert_eq!(MINUTE.unit().size(), Exact::int(60));
    assert_eq!(
        ang::MINUTE.unit().size() / ang::DEGREE.unit().size(),
        Exact::int(1) / Exact::int(60)
    );
    // Within one module, a shared name is prefixed with the quantity.
    let d: fluid::DiffusionCoefficient<f64> =
        (1.0_f64 * fluid::DIFFUSION_COEFFICIENT_STOKES).into();
    close(f64::from(d / (1.0 * METRE * METRE / SECOND)), 1e-4);
    let k: fluid::KinematicViscosity<f64> = (1.0_f64 * fluid::KINEMATIC_VISCOSITY_STOKES).into();
    close(f64::from(k / (1.0 * METRE * METRE / SECOND)), 1e-4);
}

#[test]
fn physical_constants_are_units_of_their_quantities() {
    // Standard gravity times a mass is a force (a weight).
    let g0: Acceleration<f64> = (1.0_f64 * STANDARD_GRAVITY).into();
    let weight: Force<f64> = g0 * (70.0 * KILOGRAM);
    close(f64::from(weight / (1.0 * NEWTON)), 686.4655);
    // k·T is an energy: kT at 300 K ≈ 4.14e-21 J.
    let k: HeatCapacity<f64> = (1.0_f64 * BOLTZMANN_CONSTANT).into();
    let kt: Energy<f64> = k * (300.0 * KELVIN);
    close(f64::from(kt / (1.0 * JOULE)) * 1e21, 4.141947);
    // R = N_A · k, within the CODATA rounding of R.
    let n_a: chem::InverseAmountOfSubstance<f64> = (1.0_f64 * chem::AVOGADRO_CONSTANT).into();
    let r: chem::MolarHeatCapacity<f64> = n_a * k;
    let r_unit: chem::MolarHeatCapacity<f64> = (1.0_f64 * chem::MOLAR_GAS_CONSTANT).into();
    close(f64::from(r / r_unit), 1.0);
    // ħ = h / 2π rad: an angular momentum (J·s per radian), not an action.
    let h: Action<f64> = (1.0_f64 * PLANCK_CONSTANT).into();
    let hbar: ang::AngularMomentum<f64> = h / (2.0 * std::f64::consts::PI * RADIAN);
    let hbar_unit: ang::AngularMomentum<f64> = (1.0_f64 * ang::REDUCED_PLANCK_CONSTANT).into();
    close(f64::from(hbar / hbar_unit), 1.0);
    // 1 atm in pascals, exactly, and the elementary charge in coulombs.
    assert_eq!(STANDARD_ATMOSPHERE.unit().size(), Exact::int(101_325));
    let e: em::ElectricCharge<f64> = (1.0_f64 * em::ELEMENTARY_CHARGE).into();
    close(f64::from(e / (1.0 * em::COULOMB)) * 1e19, 1.602176634);
    // ε₀ μ₀ c² = 1, to the precision of the CODATA values.
    let c: Velocity<f64> = (1.0_f64 * SPEED_OF_LIGHT).into();
    let eps0: em::ElectricPermittivity<f64> = (1.0_f64 * em::VACUUM_PERMITTIVITY).into();
    let mu0: em::MagneticPermeability<f64> = (1.0_f64 * em::VACUUM_PERMEABILITY).into();
    let one: Dimensionless<f64> = eps0 * mu0 * c * c;
    assert!((f64::from(one) - 1.0).abs() < 1e-9);
    // G M / r² at the Earth's surface is about standard gravity.
    let big_g: SpecificGravitationalParameter<f64> = (1.0_f64 * GRAVITATIONAL_CONSTANT).into();
    let earth_radius: Length<f64> = (6371.0_f64 * KILOMETRE).into();
    let g: Acceleration<f64> = big_g * (5.9722e24 * KILOGRAM) / (earth_radius * earth_radius);
    assert!((f64::from(g / g0) - 1.0).abs() < 0.01);
    // σ T⁴: a black body at 300 K radiates about 459 W/m².
    let t: ThermodynamicTemperature<f64> = 300.0 * KELVIN;
    let sigma: StefanBoltzmannCoefficient<f64> = (1.0_f64 * STEFAN_BOLTZMANN_CONSTANT).into();
    let exitance: HeatFluxDensity<f64> = sigma * t * t * t * t;
    let w_per_m2 = f64::from(exitance * (1.0 * METRE) * (1.0 * METRE) / (1.0 * WATT));
    assert!((w_per_m2 - 459.3).abs() < 0.1, "got {w_per_m2}");
}

#[test]
fn velocities_read_in_units_of_c() {
    // A physical constant is a unit: read a speed as a fraction of c.
    let v: Velocity<f64> = 29_979_245.8 * METRE / SECOND;
    let in_c: Velocity<f64, { SPEED_OF_LIGHT.unit() }, false> = v.into();
    close(f64::from(in_c / (1.0 * SPEED_OF_LIGHT)), 0.1);
    // ... and build one from it.
    let half_c = 0.5_f64 * SPEED_OF_LIGHT;
    let v: Velocity<f64> = half_c.into();
    close(f64::from(v / (1.0 * METRE / SECOND)), 149_896_229.0);
    assert_eq!(format!("{half_c:.2}"), "0.50 c");
    assert_eq!(
        SIUnit::from(VelocityUnit::SpeedOfLight),
        SPEED_OF_LIGHT.unit()
    );
    // The Avogadro constant: one mole of things, counted.
    let n_a: chem::InverseAmountOfSubstance<f64> = (1.0_f64 * chem::AVOGADRO_CONSTANT).into();
    close(f64::from(n_a * (1.0 * MOLE)) / 1e23, 6.02214076);
}

// --- Units of your own -------------------------------------------------------------

/// The degree Réaumur: a unit of yours, a size and a zero - 1.25 K steps
/// from the ice point.
const REAUMUR: SIQuantity<
    {
        SIUnit::offset(
            "°Ré",
            Exact::dec(125, -2),
            Exact::dec(27315, -2),
            KELVIN.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

const BODY: SIQuantity<{ CELSIUS.unit() }, f64, false> = 37.0 * CELSIUS;

#[test]
fn units_of_your_own_convert_like_the_librarys() {
    let body: ThermodynamicTemperature<f64> = BODY.into();
    near(
        SIQuantity::<{ FAHRENHEIT.unit() }, f64, false>::from(body),
        98.6,
    );
    near(
        SIQuantity::<{ REAUMUR.unit() }, f64, false>::from(body),
        29.6,
    );
    close(kelvins(body), 310.15);
    near(
        SIQuantity::<{ RANKINE.unit() }, f64, false>::from(body),
        310.15 * 1.8,
    );
    let atm: Pressure<f64> = (1.0_f64 * STANDARD_ATMOSPHERE).into();
    let psi: Pressure<f64, { PSI.unit() }, false> = atm.into();
    assert!(
        (f64::from(psi / (1.0 * PSI)) - 14.6959).abs() < 1e-4,
        "{psi}"
    );
    close(f64::from(atm / (1.0 * PASCAL)), 101_325.0);
    assert_eq!(format!("{}", 2.0_f64 * REAUMUR), "2 °Ré");
    // Picked at run time: any unit of a temperature.
    let scales: [SIUnit; 3] = [
        CELSIUS.unit(),
        REAUMUR.unit(),
        ThermodynamicTemperatureUnit::Fahrenheit.into(),
    ];
    close(number_of(scales[0], body), 37.0);
    close(number_of(scales[1], body), 29.6);
    close(number_of(scales[2], body), 98.6);
    // Run-time conversion between any two units of a dimension.
    fn convert(n: f64, from: SIUnit, to: SIUnit) -> f64 {
        to.from_base(from.to_base(n))
    }
    close(convert(1.0, FOOT.unit(), METRE.unit()), 0.3048);
    close(convert(100.0, CELSIUS.unit(), FAHRENHEIT.unit()), 212.0);
    close(convert(80.0, REAUMUR.unit(), CELSIUS.unit()), 100.0);
}

/// The number type is a parameter: a unit's exact size is rounded to the
/// one a quantity counts in, and std `From` widens it losslessly.
#[test]
fn other_number_types() {
    let p32: Pressure<f32> = 2500.0_f32 * PASCAL;
    let bar32: Pressure<f32, { BAR.unit() }, false> = p32.into();
    assert_eq!(f32::from(bar32 / (1.0_f32 * BAR)), 0.025_f32);
    let p: Pressure<f64> = Pressure::<f64>::from(p32);
    close(f64::from(p / (1.0 * PASCAL)), 2500.0);
    // Code generic over the number type.
    fn area<N>(side: Length<N>) -> Area<N>
    where
        Length<N>: Copy + core::ops::Mul<Output = Area<N>>,
    {
        side * side
    }
    assert_eq!(area(3.0_f32 * METRE), 9.0_f32 * METRE * METRE);
    assert_eq!(area(3_i32 * METRE), 9_i32 * METRE * METRE);
}
