//! The SI system at work: units build and read quantities, the static
//! quantities compose with typed operators, the open type
//! (`DynamicSIQuantity`) decides at run time, and the exact constants compose
//! in `const` items and round to `f64` there.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl, const_ops)]
#![feature(const_convert)]
#![allow(incomplete_features)]

use yadc::{Dimension, PartialPowi, PartialRoot, PowiAs, RootAs, Unit};
use yadc_si::angular::*;
use yadc_si::chemistry::InverseAmountOfSubstance;
use yadc_si::dimensionless::*;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::electromagnetic::{ElectricCharge, ElectricPermittivity, MagneticPermeability};
use yadc_si::mechanical::*;
use yadc_si::thermal::*;
use yadc_si::{CENTI, Exact, ISQ, KILO, Kind, MILLI, Prefix, Prefixed, SIQuantity, Scale};

/// A static quantity's dimension: its const parameter.
fn dim_of<const D: ISQ, N>(_: SIQuantity<D, N>) -> ISQ {
    D
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

#[test]
fn quantity_and_value_round_trip() {
    use MassUnit::{Gram, Kilogram, Pound};
    let mass = Gram.quantity(1500.0);
    close(Kilogram.value(mass), 1.5);
    close(Gram.value(mass), 1500.0);

    // Naming the target quantity, where a unit measures several.
    let m: Mass<f64> = Pound.quantity(10.0);
    close(Kilogram.value(m), 4.5359237); // the international pound, exactly
}

#[test]
fn units_build_quantities_and_quantities_compose() {
    use LengthUnit::Metre;
    use TimeUnit::{Hour, Second};
    use VelocityUnit::{KilometrePerHour, MetrePerSecond};

    // Units never take part in arithmetic: build quantities, then compute.
    let v: Velocity<f64> = KILOMETRE.quantity(90.0) / Hour.quantity(1.0);
    close(MetrePerSecond.value(v), 25.0);
    close(KilometrePerHour.value(v), 90.0);
    close(
        MetrePerSecond.value(Metre.quantity(1.0) / Second.quantity(1.0)),
        1.0,
    );
    close(KILOMETRE.value(v * Hour.quantity(1.0)), 90.0);
}

#[test]
fn dynamic_read_is_checked() {
    use MassUnit::{Gram, Kilogram};
    let m = DynamicSIQuantity::from(Kilogram.quantity(2.5));
    // Read through the static quantity: `TryFrom` is the check.
    assert_eq!(Mass::try_from(m).map(|m| Gram.value(m)), Ok(2500.0));
    close(Gram.value(Mass::try_from(m).unwrap()), 2500.0);
    // The wrong quantity reads as an error, handing the quantity back.
    assert_eq!(
        Force::try_from(m).map(|f| ForceUnit::Newton.value(f)),
        Err(m)
    );
}

#[test]
fn mul_and_div_compose_dims() {
    let m = MassUnit::Kilogram.quantity(2.0);
    let a = AccelerationUnit::MetrePerSecondSquared.quantity(3.0);
    let f: Force<f64> = m * a;
    close(ForceUnit::Newton.value(f), 6.0);

    // Same-dim division yields Dimensionless, which is a plain number.
    let r1 = ForceUnit::Newton.quantity(10.0);
    let r2 = ForceUnit::Newton.quantity(2.0);
    let ratio: Dimensionless<f64> = r1 / r2;
    close(f64::from(ratio), 5.0);
    close(DimensionlessUnit::Ratio.value(ratio), 5.0);
    close(DimensionlessUnit::Percent.value(ratio), 500.0);
}

#[test]
fn temperature_subtraction_yields_interval() {
    use ThermodynamicTemperatureUnit::Celsius;
    let hot = Celsius.quantity(100.0);
    let cold = Celsius.quantity(20.0);
    let delta: TemperatureInterval<f64> = hot - cold;
    close(TemperatureIntervalUnit::Kelvin.value(delta), 80.0);
    close(TemperatureIntervalUnit::Celsius.value(delta), 80.0);
    close(TemperatureIntervalUnit::Fahrenheit.value(delta), 144.0);
}

#[test]
fn temperature_addition_with_interval_stays_a_temperature() {
    use ThermodynamicTemperatureUnit::Celsius;
    let warm = Celsius.quantity(20.0);
    let rise = TemperatureIntervalUnit::Celsius.quantity(5.0);
    let hotter: ThermodynamicTemperature<f64> = warm + rise;
    close(Celsius.value(hotter), 25.0);
}

#[test]
fn round_trip_celsius_fahrenheit_kelvin() {
    use ThermodynamicTemperatureUnit::{Celsius, Fahrenheit, Kelvin};
    // The zero point is the unit's business: 25 °C is 298.15 K.
    let t = Celsius.quantity(25.0);
    close(Kelvin.value(t), 298.15);
    close(Fahrenheit.value(t), 77.0);

    // ... and back: 77 °F is 25 °C is 298.15 K.
    let f = Fahrenheit.quantity(77.0);
    close(Celsius.value(f), 25.0);
    let k = Kelvin.quantity(298.15);
    close(Celsius.value(k), 25.0);
    close(Fahrenheit.value(k), 77.0);

    // A step of one degree Celsius is an interval, measured by its own unit.
    let step: TemperatureInterval<f64> = TemperatureIntervalUnit::Celsius.quantity(1.0);
    close(TemperatureIntervalUnit::Kelvin.value(step), 1.0);
}

#[test]
fn temperature_intervals_compose_normally() {
    use TemperatureIntervalUnit::{Celsius, Kelvin};
    let a = Celsius.quantity(5.0);
    let b = Celsius.quantity(3.0);
    let sum: TemperatureInterval<f64> = a + b;
    close(Kelvin.value(sum), 8.0);
}

#[test]
fn angle_is_not_dimensionless() {
    // An angle is one radian on the `rad` axis (`rad: 1`), distinct from a plain
    // dimensionless ratio (every exponent zero) even though both are "m/m".
    let half_circle = AngleUnit::Degree.quantity(180.0);
    close(AngleUnit::Radian.value(half_circle), std::f64::consts::PI);

    let ratio = DimensionlessUnit::Ratio.quantity(0.5);
    close(f64::from(ratio), 0.5);

    // Angle / Angle -> Dimensionless: the radians cancel.
    let twice = AngleUnit::Degree.quantity(360.0);
    let dimensionless: Dimensionless<f64> = twice / half_circle;
    close(f64::from(dimensionless), 2.0);
}

#[test]
fn radians_are_an_exponent_axis() {
    const RADIAN: Angle<f64> = yadc_si::angular::consts::RADIAN.into();
    // Torque is J/rad, so torque × angle is energy (the work of a turn).
    let torque = TorqueUnit::NewtonMetre.quantity(10.0);
    let work: Energy<f64> = torque * AngleUnit::Revolution.quantity(1.0);
    close(EnergyUnit::Joule.value(work), 20.0 * std::f64::consts::PI);
    // Angular speed × time is an angle.
    let turned: Angle<f64> =
        AngularVelocityUnit::RevolutionPerMinute.quantity(60.0) * TimeUnit::Second.quantity(1.0);
    close(AngleUnit::Revolution.value(turned), 1.0);
    // Angle² is a solid angle.
    let patch: SolidAngle<f64> = AngleUnit::Radian.quantity(0.1) * AngleUnit::Radian.quantity(0.2);
    close(SolidAngleUnit::Steradian.value(patch), 0.02);
    // Geometry says where a radian is dropped: arc length s = r·θ / rad, and
    // rim speed v = ω·r / rad.
    let r = LengthUnit::Metre.quantity(2.0);
    let arc: Length<f64> = r * AngleUnit::Radian.quantity(1.5) / RADIAN;
    close(LengthUnit::Metre.value(arc), 3.0);
    let rim: Velocity<f64> = AngularVelocityUnit::RadianPerSecond.quantity(3.0) * r / RADIAN;
    close(VelocityUnit::MetrePerSecond.value(rim), 6.0);
}

#[test]
fn add_assign_and_sub_assign() {
    use MassUnit::{Gram, Kilogram};
    let mut m = Kilogram.quantity(1.0);
    m += Gram.quantity(500.0);
    close(Kilogram.value(m), 1.5);
    m -= Gram.quantity(250.0);
    close(Kilogram.value(m), 1.25);
}

#[test]
fn quantity_convenience_ops() {
    use MassUnit::{Gram, Kilogram};
    use ThermodynamicTemperatureUnit::Celsius;

    let zero = Kilogram.quantity(0.0);
    close(Kilogram.value(zero), 0.0);

    let debt = -Gram.quantity(500.0);
    close(Kilogram.value(debt), -0.5);

    assert!(Kilogram.quantity(2.0) > Kilogram.quantity(1.0));
    assert!(Celsius.quantity(25.0) > Celsius.quantity(20.0));

    let addend = Gram.quantity(250.0);
    let mut borrowed = Kilogram.quantity(1.0);
    borrowed += &addend;
    close(Kilogram.value(borrowed), 1.25);
    borrowed -= &addend;
    close(Kilogram.value(borrowed), 1.0);

    let delta = TemperatureIntervalUnit::Celsius.quantity(5.0);
    let mut temp = Celsius.quantity(20.0);
    temp += &delta;
    close(Celsius.value(temp), 25.0);
    temp -= &delta;
    close(Celsius.value(temp), 20.0);

    let mut scaled = Kilogram.quantity(2.0);
    scaled *= DimensionlessUnit::Percent.quantity(50.0);
    close(Kilogram.value(scaled), 1.0);
    scaled /= DimensionlessUnit::Percent.quantity(25.0);
    close(Kilogram.value(scaled), 4.0);
}

#[test]
fn power_levels_share_the_enum_with_watts() {
    use PowerUnit::{DecibelMilliwatt, DecibelWatt, Watt};

    // 30 dBm is 30 dB above 1 mW: 1 W = 0 dBW.
    let p = DecibelMilliwatt.quantity(30.0);
    close(Watt.value(p), 1.0);
    close(DecibelWatt.value(p), 0.0);

    // 20 dBW = 100 W = 50 dBm.
    let p = DecibelWatt.quantity(20.0);
    close(Watt.value(p), 100.0);
    close(DecibelMilliwatt.value(p), 50.0);

    // Power flows through ordinary arithmetic; the level is just a readout.
    let doubled = p * 2.0;
    close(DecibelWatt.value(doubled), 20.0 + 10.0 * 2f64.log10());

    // A multiple and the levels are values of one type, picked alike.
    let kilowatt = Prefixed { prefix: Some(Prefix::Kilo), unit: Watt };
    let picks = [kilowatt, Watt.into(), DecibelWatt.into(), DecibelMilliwatt.into()];
    let readouts: Vec<String> = picks
        .iter()
        .map(|u| format!("{:.1} {}", u.value(p), u))
        .collect();
    assert_eq!(readouts, ["0.1 kW", "100.0 W", "20.0 dBW", "50.0 dBm"]);
}

#[test]
fn gains_are_logarithms_of_ratios() {
    use GainUnit::{Bel, Decibel, Neper};

    // A gain is a quantity of its own, with plain units: decibels add.
    let g: Gain<f64> = 3.0 * DECIBEL + 7.0 * DECIBEL;
    close(Decibel.value(g), 10.0);
    close(Bel.value(g), 1.0);
    close(BEL.value(Decibel.quantity(20.0)), 2.0);
    // 1 Np = 20 / ln 10 dB.
    close(Decibel.value(NEPER), 20.0 / std::f64::consts::LN_10);
    close(Neper.value(Neper.quantity(2.0)), 2.0);
    assert_eq!(format!("{Decibel} {Decibel:#}"), "dB decibels");

    // The plain ratio a gain is, and back: a power ratio, as for the bel.
    let r = Dimensionless::from(3.0 * DECIBEL); // ~2x power
    close(f64::from(r), 10f64.powf(0.3));
    close(DECIBEL.value(Gain::from(r)), 3.0);
    close(f64::from(Dimensionless::from(BEL)), 10.0);
    close(
        f64::from(Dimensionless::from(NEPER)),
        std::f64::consts::E.powi(2),
    );
    close(
        DECIBEL.value(Gain::from(DimensionlessUnit::Ratio.quantity(1000.0))),
        30.0,
    );
}

#[test]
fn power_levels_are_scales_on_the_logarithmic_line() {
    const DBW: Scale<PowerLevel<f64>, Gain<f64>> = mech::DBW.into();

    // A scale of levels is a unit of `PowerLevel`; the watts are one std
    // `From` away, either way.
    let level = DBM.quantity(30.0);
    let p = Power::from(level);
    close(WATT.value(p), 1.0);
    close(DBW.value(level), 0.0);
    close(DBM.value(PowerLevel::from(100.0 * MILLIWATT)), 20.0);
    close(DBM.value(MILLIWATT.into()), 0.0);
    close(DBM.value(PowerLevel::from(mech::MILLIWATT_LEVEL)), 0.0);

    // A link budget, in decibels: on the line, multiplying is adding.
    let received = level - 3.0 * DECIBEL + 13.0 * DECIBEL;
    close(DBM.value(received), 40.0);
    close(WATT.value(Power::from(received)), 10.0);
    // Two levels differ by a gain; a gain moves a level.
    let gain: Gain<f64> = received - level;
    close(DECIBEL.value(gain), 10.0);
    close(DBM.value(level + gain), 40.0);
    // The linear side and the line agree: doubling the watts is 3.01 dB.
    close(
        DECIBEL.value(PowerLevel::from(2.0 * p) - level),
        10.0 * 2f64.log10(),
    );
    // In the unit enum of `Power`, a level's scale reads the watts `via` its
    // line.
    let dbm = PowerUnit::DecibelMilliwatt;
    close(WATT.value(dbm.quantity(30.0)), 1.0);
    close(dbm.value(2.0 * p), 30.0 + 10.0 * 2f64.log10());

    // The level's own unit enum, and the dynamic type.
    use PowerLevelUnit::{DecibelMilliwatt, DecibelWatt};
    close(DecibelWatt.value(DecibelMilliwatt.quantity(30.0)), 0.0);
    assert_eq!(format!("{DecibelMilliwatt} {DecibelWatt:#}"), "dBm decibels-watt");
    let d = DynamicSIQuantity::from(level);
    assert_eq!(PowerLevel::try_from(d), Ok(level));
    assert!(Power::try_from(d).is_err());
    // A level and a gain have a sum; two levels sum to the logarithm of watts
    // squared, which is no power level; and a level cannot be scaled.
    assert!((d + d).is_some_and(|sum| PowerLevel::try_from(sum).is_err()));
    assert_eq!(d * 2.0, None);
    let moved = (d + DynamicSIQuantity::from(gain)).unwrap();
    close(DBM.value(PowerLevel::try_from(moved).unwrap()), 40.0);
    assert_eq!(
        (moved - d).map(|x| Gain::try_from(x).is_ok()),
        Some(true)
    );
}

#[test]
fn display_quantity_via_unit() {
    use PowerUnit::{DecibelWatt, Watt};
    let p = Watt.quantity(100.0);
    // The number read in the unit, then the unit's own `Display`: `{}` is
    // its symbol ...
    assert_eq!(
        format!("{:.0} {}", DecibelWatt.value(p), DecibelWatt),
        "20 dBW"
    );
    // ... and `{:#}` its long name.
    assert_eq!(
        format!("{:.0} {:#}", DecibelWatt.value(p), DecibelWatt),
        "20 decibels-watt"
    );
    // Format flags go to whichever part they are written on.
    assert_eq!(format!("{:.2} {}", Watt.value(p), Watt), "100.00 W");
    assert_eq!(format!("{:>8.1} {}|", Watt.value(p), Watt), "   100.0 W|");
    // A quantity prints itself too: its number and its dimension.
    assert_eq!(format!("{p}"), "100 m²·kg·s⁻³");
}

#[test]
fn display_an_open_set_quantity_after_try_from() {
    use PowerUnit::Watt;
    let dq = DynamicSIQuantity::from(Watt.quantity(100.0));
    let p = Power::try_from(dq).expect("dim matches");
    assert_eq!(format!("{} {}", Watt.value(p), Watt), "100 W");
    assert!(Mass::try_from(dq).is_err());
}

#[test]
fn static_dynamic_bridge_round_trips_and_keeps_the_original_on_failure() {
    use MassUnit::Kilogram;
    let dq = DynamicSIQuantity::from(Kilogram.quantity(2.5));
    let mass: Mass<f64> = dq.try_into().expect("dim matches");
    close(Kilogram.value(mass), 2.5);

    // The failure carries the original quantity back, not `()`.
    let err: DynamicSIQuantity = Force::try_from(dq).unwrap_err();
    close(Kilogram.value(Mass::try_from(err).unwrap()), 2.5);
}

#[test]
fn dynamic_arithmetic_is_partial() {
    use TimeUnit::Hour;

    let d = DynamicSIQuantity::from(KILOMETRE.quantity(90.0));
    let hour = DynamicSIQuantity::from(Hour.quantity(1.0));
    let km = DynamicSIQuantity::from(KILOMETRE);
    // The open type composes dimensions at run time: every step may have no
    // result.
    let v = (d / (hour * 2.0).unwrap()).unwrap();
    assert_eq!(
        Velocity::try_from(v).map(|v| VelocityUnit::KilometrePerHour.value(v)),
        Ok(45.0)
    );

    // `km + h` is ill-formed: the operator has no result ...
    assert_eq!(d + hour, None);
    // ... so a chain stops at the first rejected step, with `?` ...
    let rejected = || -> Option<DynamicSIQuantity> { ((d + hour)? * d)? / km };
    assert_eq!(rejected(), None);
    // ... while a well-formed chain goes through.
    let accepted = || -> Option<DynamicSIQuantity> { (((d * d)? / km)? + d)? - d };
    let total = accepted().and_then(|x| Length::try_from(x).ok()).unwrap();
    close(KILOMETRE.value(total), 8100.0);
    // A read of the wrong quantity rejects it.
    assert!(f64::try_from(d).is_err());
    assert!(Length::try_from(v).is_err());

    // Temperatures work dynamically too.
    use ThermodynamicTemperatureUnit::Celsius;
    let t = DynamicSIQuantity::from(Celsius.quantity(25.0));
    close(
        Celsius.value(ThermodynamicTemperature::try_from(t).unwrap()),
        25.0,
    );
    let rise = (t - DynamicSIQuantity::from(Celsius.quantity(0.0))).unwrap();
    close(
        TemperatureIntervalUnit::Kelvin.value(TemperatureInterval::try_from(rise).unwrap()),
        25.0,
    );
    // A temperature is an amount: it scales and adds to another as itself,
    // and its product with a length is an ordinary kelvin-metre; the step
    // between two of them is an ordinary amount too.
    assert!((t * 2.0).is_some_and(|q| ThermodynamicTemperature::try_from(q).is_ok()));
    assert!((t / 2.0).is_some_and(|q| ThermodynamicTemperature::try_from(q).is_ok()));
    assert!((t + t).is_some_and(|q| ThermodynamicTemperature::try_from(q).is_ok()));
    assert!((t * km).is_some_and(|q| ThermodynamicTemperature::try_from(q).is_err()));
    assert!((rise * 2.0).is_some());
    // `+` is onto its left operand: a step plus a temperature has no result
    // (write `t + step`).
    assert_eq!(rise + t, None);
    assert!((t + rise).is_some_and(|q| ThermodynamicTemperature::try_from(q).is_ok()));
}

// --- Kinds: linear, absolute, logarithmic ---------------------------------------

#[test]
fn a_temperature_is_an_amount_apart_from_its_differences() {
    use ThermodynamicTemperatureUnit::{Celsius, Fahrenheit, Kelvin};
    let (cold, hot) = (Celsius.quantity(20.0), Celsius.quantity(100.0));
    // Two temperatures differ by an interval; an interval moves a
    // temperature, either way. (`rise + cold` does not compile: "cannot add:
    // the labels have no sum" - write `cold + rise`.)
    let rise: TemperatureInterval<f64> = hot - cold;
    close(TemperatureIntervalUnit::Kelvin.value(rise), 80.0);
    let warmer: ThermodynamicTemperature<f64> = cold + rise / 2.0;
    close(Celsius.value(warmer), 60.0);
    let cooler: ThermodynamicTemperature<f64> = hot - rise;
    close(Celsius.value(cooler), 20.0);
    // Every scale reads the one type.
    close(Fahrenheit.value(warmer), 140.0);
    close(Kelvin.value(warmer), 333.15);
    // In place.
    let mut t = cold;
    t += rise;
    t -= rise / 4.0;
    close(Celsius.value(t), 80.0);
    assert!(cold < hot);
    // Absolute zero is a fact, so a temperature is an amount: it averages
    // and scales - in kelvins, whatever scale built it - and is a unit of
    // its own kind.
    let middle: ThermodynamicTemperature<f64> = (cold + hot) / 2.0;
    close(Celsius.value(middle), 60.0);
    let doubled: ThermodynamicTemperature<f64> = 2.0 * warmer;
    close(Kelvin.value(doubled), 666.3);
    close(
        ThermodynamicTemperature::<f64>::from(therm::ICE_POINT).value(doubled),
        666.3 / 273.15,
    );
    // It multiplies, and the label drops out of the product: the ideal gas
    // law, p V = n R T, and a fourth power for radiation.
    let energy = HeatCapacity::<f64>::from(chem::MOLE * chem::MOLAR_GAS_CONSTANT) * warmer;
    close(EnergyUnit::Joule.value(energy), 8.314462618 * 333.15);
    let fourth = warmer.powi::<4>();
    assert_eq!(dim_of(fourth).kind, Kind::Linear);
    assert_eq!(dim_of(fourth).K, 4);
    let ratio = hot / cold;
    close(f64::from(ratio), 373.15 / 293.15);
}

#[test]
fn kinds_decide_sums_and_products() {
    // A quantity's dimension is its type's const parameter - here, a unit
    // constant's.
    let absolute = dim_of(therm::ABSOLUTE_ZERO);
    let step = dim_of(therm::KELVIN);
    let metre = dim_of(mech::METRE);
    assert_eq!(absolute.kind, Kind::Absolute);
    assert_eq!(<Kind as Dimension>::DIMENSIONLESS, Kind::Linear);
    // Absolute, a label marked `absolute`: two differ by a step, a step
    // moves one, and they add to each other (an average); nothing with a
    // step on the left. Beside a plain number the label survives a product;
    // any other product is linear.
    assert_eq!(absolute - absolute, Some(step));
    assert_eq!(absolute + step, Some(absolute));
    assert_eq!(absolute - step, Some(absolute));
    assert_eq!(absolute + absolute, Some(absolute));
    assert_eq!(step + absolute, None);
    assert_eq!(step - absolute, None);
    assert_eq!(absolute + metre, None);
    assert_eq!(absolute * ISQ::DIMENSIONLESS, Some(absolute));
    assert_eq!(
        absolute * metre,
        Some(ISQ {
            m: 1,
            K: 1,
            ..ISQ::DIMENSIONLESS
        })
    );
    assert_eq!(
        metre / absolute,
        Some(ISQ {
            m: 1,
            K: -1,
            ..ISQ::DIMENSIONLESS
        })
    );
    // Linear: products always exist, sums within one dimension.
    assert_eq!(
        dim_of(mech::KILOGRAM) * dim_of(mech::METRE_PER_SECOND_SQUARED),
        Some(dim_of(mech::NEWTON))
    );
    assert_eq!(dim_of(mech::KILOGRAM) + metre, None);
    // One over an angle exists: `rad: -1`.
    assert_eq!(
        ISQ::DIMENSIONLESS / dim_of(yadc_si::angular::consts::RADIAN),
        Some(ISQ {
            rad: -1,
            ..ISQ::DIMENSIONLESS
        })
    );
    assert_eq!(
        ISQ::DIMENSIONLESS,
        dim_of(yadc_si::dimensionless::consts::RATIO)
    );
    // Logarithmic: `+` and `-` add and subtract the exponents - adding
    // logarithms multiplies what they are the logarithms of.
    let level = dim_of(mech::WATT_LEVEL);
    let gain = dim_of(mech::BEL);
    let watt = dim_of(mech::WATT);
    assert_eq!(level, ISQ { kind: Kind::Log, ..watt });
    assert_eq!(level - level, Some(gain));
    assert_eq!(level + gain, Some(level));
    assert_eq!(level - gain, Some(level));
    assert_eq!(gain + gain, Some(gain));
    assert_eq!(
        level + level,
        Some(ISQ { kind: Kind::Log, ..(watt * watt).unwrap() })
    );
    // Only a logarithm adds to a logarithm.
    assert_eq!(level + watt, None);
    assert_eq!(gain + ISQ::DIMENSIONLESS, None);
    // Beside a plain number a product is a scaling: a gain scales (gains
    // add), a level does not (two levels sum to another dimension). A gain
    // leaves any other product, as a label does; a level has no other product.
    assert_eq!(gain * ISQ::DIMENSIONLESS, Some(gain));
    assert_eq!(level * ISQ::DIMENSIONLESS, None);
    assert_eq!(gain / metre, ISQ::DIMENSIONLESS / metre);
    assert_eq!(level * metre, None);
    assert_eq!(gain * gain, None);

    // The open type follows the same algebra.
    let t = ThermodynamicTemperatureUnit::Kelvin.quantity(300.0);
    let step = TemperatureIntervalUnit::Kelvin.quantity(5.0);
    let (dt, dstep) = (DynamicSIQuantity::from(t), DynamicSIQuantity::from(step));
    assert!((dt + dstep).is_some_and(|s| ThermodynamicTemperature::try_from(s).is_ok()));
    assert!((dt - dt).is_some_and(|s| TemperatureInterval::try_from(s).is_ok()));
    assert_eq!(dstep + dt, None);
}

#[test]
fn a_gain_per_length_is_a_plain_rate() {
    // An attenuation of 0.2 dB per kilometre: a gain over a length leaves the
    // logarithmic kind, as a label leaves a product - a plain number per
    // metre, counting bels.
    let attenuation: yadc_si::density::LinearNumberDensity<f64> =
        (0.2 * DECIBEL) / KILOMETRE;
    // Over 15 km it is a plain number again - of bels; that many bels is the
    // gain.
    let bels: Dimensionless<f64> = attenuation * KILOMETRE.quantity(15.0);
    let loss: Gain<f64> = bels * BEL;
    close(DECIBEL.value(loss), 3.0);
    // Launched at 0 dBm, the level after the fibre.
    let launched: PowerLevel<f64> = DBM.quantity(0.0);
    let received: PowerLevel<f64> = launched - loss;
    close(DBM.value(received), -3.0);
    let power: Power<f64> = received.into();
    close(MILLIWATT.value(power), 10f64.powf(-0.3));
}

/// Generic code reads and builds a value through any unit it is handed: the
/// unit is the only way between numbers and quantities, so it is all the
/// code needs - offset units included.
/// An instant is a point on the one line of times (the affine kind): two
/// differ by a `Time`, a `Time` moves one, and every scale - Unix seconds,
/// GPS seconds, Julian dates - reads the same line. Not compiled, because
/// refused: `a + b` ("cannot add a point to a point"), `2.0 * a` ("a point
/// cannot be scaled"), `a * SECOND` ("these have no product"), `HOUR + a`
/// ("cannot add these in this order").
#[test]
fn an_instant_is_a_point_in_time() {
    type InstantScale = Scale<Instant<f64>, Time<f64>>;
    const DAY: Time<f64> = mech::DAY.into();
    const HOUR: Time<f64> = mech::HOUR.into();
    const SECOND: Time<f64> = mech::SECOND.into();
    const GPS_EPOCH: Instant<f64> = mech::GPS_EPOCH.into();
    const UNIX_EPOCH: Instant<f64> = mech::UNIX_EPOCH.into();
    const GPS_SECONDS: InstantScale = mech::GPS_SECONDS.into();
    const JULIAN_DATE: InstantScale = mech::JULIAN_DATE.into();
    const MODIFIED_JULIAN_DATE: InstantScale = mech::MODIFIED_JULIAN_DATE.into();
    const UNIX_MILLISECONDS: InstantScale = mech::UNIX_MILLISECONDS.into();
    const UNIX_SECONDS: InstantScale = mech::UNIX_SECONDS.into();

    // Point algebra.
    let launch: Instant<f64> = UNIX_SECONDS.quantity(1_700_000_000.0);
    let landing = launch + 2.0 * HOUR;
    let flight: Time<f64> = landing - launch;
    close(HOUR.value(flight), 2.0);
    close(UNIX_SECONDS.value(landing - 30.0 * SECOND), 1_700_007_170.0);
    assert!(launch < landing);
    let mut t = launch;
    t += DAY;
    t -= HOUR;
    close(HOUR.value(t - launch), 23.0);
    assert_eq!(dim_of(launch).kind, Kind::Affine);
    assert_eq!(dim_of(flight), dim_of(SECOND));

    // One line, many scales: the zeros are constants on it.
    close(UNIX_SECONDS.value(GPS_EPOCH), 315_964_800.0);
    close(GPS_SECONDS.value(UNIX_EPOCH), -315_964_800.0);
    close(JULIAN_DATE.value(UNIX_EPOCH), 2_440_587.5);
    close(MODIFIED_JULIAN_DATE.value(UNIX_EPOCH), 40_587.0);
    close(MODIFIED_JULIAN_DATE.value(GPS_EPOCH), 44_244.0);
    // An instant built on one scale reads on any other: J2000.0, noon on
    // 1 January 2000, is JD 2451545.0 and Unix 946728000.
    let j2000 = JULIAN_DATE.quantity(2_451_545.0);
    close(UNIX_SECONDS.value(j2000), 946_728_000.0);
    close(GPS_SECONDS.value(j2000), 946_728_000.0 - 315_964_800.0);
    close(DAY.value(j2000 - GPS_SECONDS.quantity(0.0)), 7_300.5);
    // Stored as `f64` seconds from the Unix epoch: today's instants resolve
    // to a fraction of a microsecond, so a millisecond count survives.
    let precise: Instant<f64> = UNIX_MILLISECONDS.quantity(1_700_000_000_123.0);
    assert!((UNIX_SECONDS.value(precise) - 1_700_000_000.123).abs() < 1e-6);
    assert!((UNIX_MILLISECONDS.value(precise) - 1_700_000_000_123.0).abs() < 1e-3);
    // A scale made at run time: seconds since launch.
    let mission = Scale(launch, SECOND);
    close(mission.value(landing), 7_200.0);
    close(UNIX_SECONDS.value(mission.quantity(60.0)), 1_700_000_060.0);
    const EPOCH_JD: f64 = JULIAN_DATE.value(UNIX_EPOCH);
    close(EPOCH_JD, 2_440_587.5);

    // The unit enum: a menu of scales.
    use InstantUnit::{GpsSecond, JulianDate, ModifiedJulianDate, UnixMillisecond, UnixSecond};
    close(UnixSecond.value(GpsSecond.quantity(0.0)), 315_964_800.0);
    close(ModifiedJulianDate.value(JulianDate.quantity(2_400_000.5)), 0.0);
    close(UnixMillisecond.value(UNIX_SECONDS.quantity(1.5)), 1_500.0);
    assert_eq!(format!("{UnixSecond} / {JulianDate:#}"), "s Unix / Julian date");
    assert_eq!("MJD".parse(), Ok(ModifiedJulianDate));
    assert_eq!(InstantUnit::default(), UnixSecond);

    // At run time: the same algebra, `None` where there is no result.
    let (a, b) = (DynamicSIQuantity::from(launch), DynamicSIQuantity::from(landing));
    let two_hours = DynamicSIQuantity::from(flight);
    assert_eq!((b - a).and_then(|d| Time::try_from(d).ok()), Some(flight));
    assert_eq!((a + two_hours).and_then(|d| Instant::try_from(d).ok()), Some(landing));
    assert_eq!(a + b, None);
    assert_eq!(two_hours + a, None);
    assert_eq!(a * 2.0, None);
    assert_eq!(a * two_hours, None);
    assert_eq!(a / two_hours, None);
    assert_eq!(a.partial_powi(2), None);
    assert!(Time::try_from(a).is_err());
    assert_eq!(a.partial_cmp(&b), Some(core::cmp::Ordering::Less));
    assert_eq!(a.partial_cmp(&two_hours), None);
    assert_eq!(format!("{}", UNIX_SECONDS.quantity(12.5)), "12.5 s{Affine}");
}

fn number_of<Q, U: Unit<Q>>(unit: &U, q: Q) -> f64 {
    unit.value(q)
}
fn from_number<Q, U: Unit<Q>>(unit: &U, n: f64) -> Q {
    unit.quantity(n)
}

#[test]
fn generic_code_reads_and_builds_through_a_unit() {
    use ThermodynamicTemperatureUnit::{Celsius, Kelvin};
    let t: ThermodynamicTemperature<f64> = from_number(&Kelvin, 300.0);
    close(number_of(&Celsius, t), 26.85);
    close(number_of(&Kelvin, t), 300.0);
    let m: Mass<f64> = from_number(&MassUnit::Kilogram, 2.5);
    close(number_of(&MassUnit::Gram, m), 2500.0);
    // A unit picked at run time: `&dyn Unit<Q>`.
    let units: [&dyn Unit<Mass<f64>>; 2] = [&MassUnit::Kilogram, &MassUnit::Pound];
    close(number_of(&units[0], m), 2.5);
}

// --- Units without a home ------------------------------------------------------

#[test]
fn units_picked_at_run_time() {
    use LengthUnit::{Metre, Mile};
    use MassUnit::{Gram, Kilogram, Pound};

    // A unit need not print, so a list of units that do is your own trait:
    // a unit of `Q` that is also `Display` (`{}` the symbol, `{:#}` the
    // name).
    trait Shown<Q>: Unit<Q> + core::fmt::Display {}
    impl<Q, U: Unit<Q> + core::fmt::Display> Shown<Q> for U {}

    // Some unit of length: chosen at run time, still checked at compile time
    // (a mass unit would not fit in this array).
    let kilometre = Prefixed { prefix: Some(Prefix::Kilo), unit: Metre };
    let picks: [&dyn Shown<Length<f64>>; 3] = [&Metre, &kilometre, &Mile];
    let d = picks[1].quantity(3.0);
    let shown: Vec<String> = picks
        .iter()
        .map(|u| format!("{:.1} {}", u.value(d), u))
        .collect();
    assert_eq!(shown, ["3000.0 m", "3.0 km", "1.9 mi"]);

    // Some quantity, decided at run time: the open type, read back through a
    // static quantity. (Some unit of some quantity is a unit set: see
    // `tests/sets.rs`.)
    let masses: [&dyn Shown<Mass<f64>>; 2] = [&Kilogram, &Gram];
    let dq = DynamicSIQuantity::from(Pound.quantity(10.0));
    let m = Mass::try_from(dq).unwrap();
    // A unit prints its symbol with `{}` (its `Display`).
    let labels: Vec<String> = masses.iter().map(|u| u.to_string()).collect();
    assert_eq!(labels, ["kg", "g"]);
    assert_eq!(
        format!("{:.1} {}", masses[1].value(m), masses[1]),
        "4535.9 g"
    );
    // The wrong quantity hands the open-set quantity back.
    assert_eq!(Length::try_from(dq), Err(dq));
    let built = DynamicSIQuantity::from(picks[0].quantity(3.0));
    close(Metre.value(Length::try_from(built).unwrap()), 3.0);

    // Labels only: any `Display`, units of different quantities side by side.
    let labels: [&dyn core::fmt::Display; 2] = [&Kilogram, &Metre];
    assert_eq!(labels.map(|u| format!("{u:#}")), ["kilograms", "metres"]);
}

#[test]
fn a_unit_struct_measuring_two_quantities() {
    // One zero-sized unit, a unit of two quantities: an absolute temperature
    // and a temperature difference are both measured in kelvin. Neither is
    // its "home" - each is one `Unit` impl. The derive writes the interval's
    // (`by = KELVIN`); the absolute temperature's counts the same steps from
    // absolute zero, another unit value, so it is written by hand.
    const ABSOLUTE_ZERO: ThermodynamicTemperature<f64> = therm::ABSOLUTE_ZERO.into();
    const KELVIN: TemperatureInterval<f64> = therm::KELVIN.into();
    #[derive(Copy, Clone, yadc::Unit)]
    #[unit("K", "kelvin", of = TemperatureInterval<f64>, by = KELVIN)]
    struct Kelvins;
    impl Unit<ThermodynamicTemperature<f64>> for Kelvins {
        fn quantity(&self, n: f64) -> ThermodynamicTemperature<f64> {
            ABSOLUTE_ZERO + KELVIN.quantity(n)
        }
        fn value(&self, t: ThermodynamicTemperature<f64>) -> f64 {
            KELVIN.value(t - ABSOLUTE_ZERO)
        }
    }

    // Building names the target, since `Kelvins` alone does not say which.
    let t: ThermodynamicTemperature<f64> = Kelvins.quantity(300.0);
    let step: TemperatureInterval<f64> = Kelvins.quantity(5.0);
    // Reading never needs it: the quantity's type picks the impl.
    close(Kelvins.value(t + step), 305.0);
    close(
        ThermodynamicTemperatureUnit::Celsius.value(t),
        300.0 - 273.15,
    );
    close(
        Kelvins.value(t - ThermodynamicTemperatureUnit::Celsius.quantity(0.0)),
        26.85,
    );
    assert_eq!(format!("{} {:#}", Kelvins.value(step), Kelvins), "5 kelvin");

    // The open type is reached through the static quantity, which names the
    // target: an interval comes back as an interval, not as a temperature.
    let d = DynamicSIQuantity::from(step);
    assert_eq!(
        TemperatureInterval::try_from(d).map(|q| Kelvins.value(q)),
        Ok(5.0)
    );
    assert!(TemperatureInterval::try_from(DynamicSIQuantity::from(t)).is_err());
}

#[test]
fn custom_function_units_mix_with_linear_ones() {
    // The Beaufort wind scale is a formula, not a factor: `v = 0.836 B^1.5`,
    // written in quantity terms.
    const METRE_PER_SECOND: Velocity<f64> = mech::METRE_PER_SECOND.into();
    /// A unit that is a pair of functions: a type of ours.
    #[derive(Copy, Clone)]
    struct Bft;
    impl Unit<Velocity<f64>> for Bft {
        fn quantity(&self, b: f64) -> Velocity<f64> {
            METRE_PER_SECOND.quantity(0.836 * b.powf(1.5))
        }
        fn value(&self, v: Velocity<f64>) -> f64 {
            (METRE_PER_SECOND.value(v) / 0.836).powf(2.0 / 3.0)
        }
    }
    /// Named next to a linear one; `Bft`'s impl is not `const`, so neither
    /// is the enum's.
    #[derive(Copy, Clone, yadc::Unit)]
    #[unit(of = Velocity<f64>, non_const)]
    enum WindUnit {
        #[unit("m/s", "metres per second", by = METRE_PER_SECOND)]
        MetrePerSecond,
        #[unit("Bft", "Beaufort", by = Bft)]
        Beaufort,
    }
    use WindUnit::{Beaufort, MetrePerSecond};

    let gale = Beaufort.quantity(5.0);
    close(MetrePerSecond.value(gale), 0.836 * 5f64.powf(1.5));
    close(Beaufort.value(gale), 5.0);
    // Any unit of `Velocity` reads it, including the generated ones.
    let kmh = VelocityUnit::KilometrePerHour.value(gale);
    close(
        Beaufort.value(VelocityUnit::KilometrePerHour.quantity(kmh)),
        5.0,
    );
    // ... and from the open type, through `Velocity`.
    let d = DynamicSIQuantity::from(MetrePerSecond.quantity(20.0));
    close(
        Beaufort.value(Velocity::try_from(d).unwrap()),
        (20.0f64 / 0.836).powf(2.0 / 3.0),
    );
    let mass = DynamicSIQuantity::from(MassUnit::Kilogram.quantity(1.0));
    assert!(Velocity::try_from(mass).map(|v| Beaufort.value(v)).is_err());
}

#[test]
fn every_generated_unit_reaches_the_open_set() {
    use ThermodynamicTemperatureUnit::{Celsius, Kelvin};

    let dq = DynamicSIQuantity::from(MassUnit::Pound.quantity(10.0));
    // Exactly the static path, read back through the static quantity.
    let kg = MassUnit::Kilogram.value(MassUnit::Pound.quantity(10.0));
    assert_eq!(
        Mass::try_from(dq).map(|m| MassUnit::Kilogram.value(m)),
        Ok(kg)
    );

    // A mismatch hands the original back.
    let err = Force::try_from(dq).unwrap_err();
    assert_eq!(err, dq);

    // Scales: the zero point is the unit's business on the dynamic side too.
    let t = DynamicSIQuantity::from(Celsius.quantity(25.0));
    close(
        Kelvin.value(ThermodynamicTemperature::try_from(t).unwrap()),
        298.15,
    );
    assert!(TemperatureInterval::try_from(t).is_err());
}

// --- Reciprocals ----------------------------------------------------------------

#[test]
fn inv_is_one_over() {
    use yadc::num_traits::Inv;
    use yadc_si::mechanical::{Frequency, FrequencyUnit};

    let f: Frequency<f64> = TimeUnit::Second.quantity(2.0).inv();
    close(FrequencyUnit::Hertz.value(f), 0.5);
    // "Per hour": one hour, inverted, times an hour is one.
    let per_hour = TimeUnit::Hour.quantity(1.0).inv();
    close(f64::from(per_hour * TimeUnit::Hour.quantity(1.0)), 1.0);
    close(FrequencyUnit::Hertz.value(per_hour), 1.0 / 3600.0);

    // The open type's reciprocal is its power -1.
    let d = DynamicSIQuantity::from(LengthUnit::Metre.quantity(4.0))
        .partial_powi(-1)
        .unwrap();
    close(
        f64::try_from((d * DynamicSIQuantity::from(LengthUnit::Metre.quantity(1.0))).unwrap())
            .unwrap(),
        0.25,
    );
    // Every amount has a reciprocal: one over an angle is `rad: -1`, so
    // times an angle it is a plain number.
    let per_rad = DynamicSIQuantity::from(AngleUnit::Radian.quantity(2.0))
        .partial_powi(-1)
        .unwrap();
    assert!(f64::try_from(per_rad).is_err());
    close(
        f64::try_from((per_rad * DynamicSIQuantity::from(AngleUnit::Radian.quantity(1.0))).unwrap())
            .unwrap(),
        0.5,
    );
    let per_rad = AngleUnit::Radian.quantity(2.0).inv();
    close(f64::from(per_rad * AngleUnit::Radian.quantity(1.0)), 0.5);
}

// --- Compile-time constants ---------------------------------------------------

// Each family has a `consts` module: the constants its units are defined
// from - one per plain unit, including the physical constants, which are
// units too (as in UCUM), plus the zeros of the scales (`ICE_POINT`). They
// are exact: arithmetic among them is exact, and std `From` rounds the
// result to a stored number type.
use yadc_si::chemistry::consts as chem;
use yadc_si::electromagnetic::consts as em;
use yadc_si::mechanical::consts as mech;
use yadc_si::thermal::consts as therm;

// The units this file builds and reads `f64` quantities with, each rounded
// once, by the compiler.
const METRE: Length<f64> = mech::METRE.into();
const KILOMETRE: Length<f64> = (KILO * mech::METRE).into();
const WATT: Power<f64> = mech::WATT.into();
const MILLIWATT: Power<f64> = (MILLI * mech::WATT).into();
const BEL: Gain<f64> = mech::BEL.into();
const DECIBEL: Gain<f64> = mech::DECIBEL.into();
const NEPER: Gain<f64> = mech::NEPER.into();
const DBM: Scale<PowerLevel<f64>, Gain<f64>> = mech::DBM.into();

// Exact arithmetic is `const`: these compose in `const` items and round
// once, evaluated by the compiler.
const LIMIT: Velocity<f64> = (KILO * mech::METRE * Exact::int(50) / mech::HOUR).into();
const ELECTRON_REST_ENERGY: Energy<f64> =
    (mech::ELECTRON_MASS * mech::SPEED_OF_LIGHT * mech::SPEED_OF_LIGHT).into();
// A unit with a step, or a step and a zero, is a `const impl`, so building
// with it is const too, zero point and all.
const BOILING: ThermodynamicTemperature<f64> =
    ThermodynamicTemperatureUnit::Celsius.quantity(100.0);
const BOILING_F: f64 = ThermodynamicTemperatureUnit::Fahrenheit.value(BOILING);

#[test]
fn const_items_compose_unit_constants() {
    close(VelocityUnit::KilometrePerHour.value(LIMIT), 50.0);
    // E = mc² for the electron: 8.187e-14 J (0.511 MeV).
    let joules = EnergyUnit::Joule.value(ELECTRON_REST_ENERGY);
    assert!((joules - 8.1871057769e-14).abs() < 1e-23, "got {joules}");
    close(ThermodynamicTemperatureUnit::Kelvin.value(BOILING), 373.15);
    close(BOILING_F, 212.0);
}

#[test]
fn unit_constants_are_one_of_the_unit() {
    use yadc_si::angular::consts as ang;
    use yadc_si::dimensionless::consts as dimless;
    use yadc_si::fluid::consts as fluid;
    use yadc_si::fluid::{DiffusionCoefficientUnit, KinematicViscosityUnit};

    close(LengthUnit::Metre.value(KILOMETRE), 1000.0);
    // Exact, by the 1959 agreement.
    assert_eq!(mech::KILOGRAM.value(mech::POUND), Exact::dec(45359237, -8));
    close(
        MassUnit::Kilogram.value(Mass::from(mech::POUND)),
        0.45359237,
    );
    assert_eq!(
        mech::JOULE.value(KILO * mech::WATT * mech::HOUR),
        Exact::int(3_600_000)
    );
    close(f64::from(Dimensionless::from(dimless::PERCENT)), 0.01);
    // A unit beside a log unit in its enum is still a constant, and takes a
    // prefix (`KILO * WATT`).
    close(
        PowerUnit::Watt.value(Power::from(KILO * mech::WATT)),
        1000.0,
    );
    // A step (a temperature difference) is an amount, so it has a constant;
    // a scale is a step and a zero (there is no "one °C" reading): °C counts
    // `KELVIN` steps from `ICE_POINT`, °F `DEGREE_FAHRENHEIT` steps from
    // `FAHRENHEIT_ZERO`.
    let step: TemperatureInterval<f64> = therm::KELVIN.into();
    close(TemperatureIntervalUnit::Celsius.value(step), 1.0);
    assert_eq!(
        therm::KELVIN.value(therm::DEGREE_FAHRENHEIT),
        Exact::int(5) / Exact::int(9)
    );
    assert_eq!(
        therm::KELVIN_SCALE.value(therm::ICE_POINT),
        Exact::dec(27315, -2)
    );
    assert_eq!(
        therm::FAHRENHEIT.value(therm::FAHRENHEIT_ZERO),
        Exact::int(0)
    );
    assert_eq!(
        therm::FAHRENHEIT.value(therm::ABSOLUTE_ZERO),
        Exact::dec(-45967, -2)
    );
    close(
        ThermodynamicTemperatureUnit::Fahrenheit.value(therm::ABSOLUTE_ZERO.into()),
        -459.67,
    );
    // Modules are namespaces: a minute of time and a minute of arc coexist.
    assert_eq!(mech::SECOND.value(mech::MINUTE), Exact::int(60));
    assert_eq!(
        ang::DEGREE.value(ang::MINUTE),
        Exact::int(1) / Exact::int(60)
    );
    // Within one module, a shared name is prefixed with the quantity.
    close(
        DiffusionCoefficientUnit::SquareMetrePerSecond
            .value(fluid::DIFFUSION_COEFFICIENT_STOKES.into()),
        1e-4,
    );
    close(
        KinematicViscosityUnit::SquareMetrePerSecond
            .value(fluid::KINEMATIC_VISCOSITY_STOKES.into()),
        1e-4,
    );
}

#[test]
fn physical_constants_are_units_with_their_quantity_types() {
    // Standard gravity times a mass is a force (a weight).
    let weight: Force<f64> =
        Acceleration::from(mech::STANDARD_GRAVITY) * MassUnit::Kilogram.quantity(70.0);
    close(ForceUnit::Newton.value(weight), 686.4655);
    // k·T with T an interval is an energy: kT at 300 K ≈ 4.14e-21 J.
    let kt: Energy<f64> = HeatCapacity::from(therm::BOLTZMANN_CONSTANT)
        * TemperatureIntervalUnit::Kelvin.quantity(300.0);
    close(EnergyUnit::Joule.value(kt) * 1e21, 4.141947);
    // R = N_A · k, within the CODATA rounding.
    // (The type annotation is the dimension check: mol⁻¹ · J/K is J/(mol·K).)
    type MolarHeatCapacity = yadc_si::chemistry::MolarHeatCapacity<f64>;
    let r = MolarHeatCapacity::from(chem::AVOGADRO_CONSTANT * therm::BOLTZMANN_CONSTANT);
    // Compare the two numbers, each read in J/(K·mol).
    let joule_per_kelvin_mole = MolarHeatCapacity::from(mech::JOULE / (therm::KELVIN * chem::MOLE));
    let number = |q: MolarHeatCapacity| joule_per_kelvin_mole.value(q);
    assert!((number(r) - number(chem::MOLAR_GAS_CONSTANT.into())).abs() < 1e-8);
    // ħ = h / 2π rad: an angular momentum (J·s per radian), not an action.
    let hbar = AngularMomentum::<f64>::from(
        mech::PLANCK_CONSTANT / (yadc_si::angular::consts::RADIAN * (Exact::int(2) * Exact::PI)),
    );
    assert!((AngularMomentumUnit::ReducedPlanckConstant.value(hbar) - 1.0).abs() < 1e-9);
    // 1 atm in kPa, and the elementary charge in coulombs.
    assert_eq!(
        (KILO * mech::PASCAL).value(mech::STANDARD_ATMOSPHERE),
        Exact::dec(101_325, -3)
    );
    close(
        yadc_si::electromagnetic::ElectricChargeUnit::Coulomb
            .value(ElectricCharge::<f64>::from(em::ELEMENTARY_CHARGE))
            * 1e19,
        1.602176634,
    );
    // ε₀ μ₀ c² = 1, to the precision of the CODATA values. (Their exact
    // product has more digits than `Exact` holds: multiply in `f64`.)
    let c = Velocity::<f64>::from(mech::SPEED_OF_LIGHT);
    let one: Dimensionless<f64> = ElectricPermittivity::<f64>::from(em::VACUUM_PERMITTIVITY)
        * MagneticPermeability::<f64>::from(em::VACUUM_PERMEABILITY)
        * c
        * c;
    assert!((f64::from(one) - 1.0).abs() < 1e-9);
    // G M / r² at the Earth's surface is about standard gravity.
    let earth_mass = MassUnit::Kilogram.quantity(5.9722e24);
    let earth_radius = KILOMETRE.quantity(6371.0);
    let g: Acceleration<f64> = SpecificGravitationalParameter::from(mech::GRAVITATIONAL_CONSTANT)
        * earth_mass
        / (earth_radius * earth_radius);
    assert!((AccelerationUnit::StandardGravity.value(g) - 1.0).abs() < 0.01);
    // σ T⁴: a black body at 300 K radiates about 459 W/m².
    let t = TemperatureIntervalUnit::Kelvin.quantity(300.0);
    let exitance =
        StefanBoltzmannCoefficient::<f64>::from(therm::STEFAN_BOLTZMANN_CONSTANT) * t * t * t * t;
    let w_per_m2 = PowerUnit::Watt
        .value(exitance * LengthUnit::Metre.quantity(1.0) * LengthUnit::Metre.quantity(1.0));
    assert!((w_per_m2 - 459.3).abs() < 0.1, "got {w_per_m2}");
}

#[test]
fn velocities_read_in_units_of_c() {
    // A physical constant is a unit: read a speed as a fraction of c.
    let v = Velocity::<f64>::from(KILO * mech::METRE_PER_SECOND).quantity(29979.2458);
    close(VelocityUnit::SpeedOfLight.value(v), 0.1);
    // ... and build one from it.
    let half_c = VelocityUnit::SpeedOfLight.quantity(0.5);
    close(VelocityUnit::MetrePerSecond.value(half_c), 149_896_229.0);
    let c = VelocityUnit::SpeedOfLight;
    assert_eq!(format!("{:.2} {}", c.value(half_c), c), "0.50 c");
    // The Avogadro constant: one mole of things, counted.
    let per_mole = yadc_si::chemistry::InverseAmountOfSubstanceUnit::PerMole.value(
        InverseAmountOfSubstance::<f64>::from(chem::AVOGADRO_CONSTANT),
    );
    close(per_mole / 1e23, 6.02214076);
}

// --- Powers and roots -----------------------------------------------------------

const CUBIC_METRE_BOX: Volume<f64> = METRE.powi::<3>();

#[test]
fn integer_powers_and_exact_roots() {
    let centimetre = Length::<f64>::from(CENTI * mech::METRE);
    let edge = centimetre.quantity(20.0);
    let face: Area<f64> = edge.powi::<2>();
    let cube: Volume<f64> = edge.powi::<3>();
    close((centimetre * centimetre).value(face), 400.0);
    close(VolumeUnit::Litre.value(cube), 8.0);
    // A root is the exact inverse of a power, dimension and all.
    let side: Length<f64> = face.root::<2>();
    close(centimetre.value(side), 20.0);
    let side3: Length<f64> = cube.root::<3>();
    close(centimetre.value(side3), 20.0);
    // Negative powers: one over a time is a frequency, one over an area ...
    let millisecond = Time::<f64>::from(MILLI * mech::SECOND);
    let f: Frequency<f64> = millisecond.quantity(2.0).powi::<-1>();
    close(FrequencyUnit::Hertz.value(f), 500.0);
    let per_area = edge.powi::<-2>();
    close(f64::from(per_area * face), 1.0);
    // ... and powers are `const`.
    close(VolumeUnit::CubicMetre.value(CUBIC_METRE_BOX), 1.0);
    // `edge.root::<2>()` does not compile: with integer exponents a length
    // has no square root.
}

#[test]
fn powers_and_roots_follow_the_kinds() {
    // Angle² is a solid angle, and back.
    let a = AngleUnit::Radian.quantity(0.5);
    let sr: SolidAngle<f64> = a.powi::<2>();
    close(SolidAngleUnit::Steradian.value(sr), 0.25);
    let back: Angle<f64> = sr.root::<2>();
    close(AngleUnit::Radian.value(back), 0.5);

    // An amount has every power, a temperature among them - the label drops
    // out - so σT⁴ reads as written: about 459 W/m² at 300 K.
    let t = ThermodynamicTemperatureUnit::Kelvin.quantity(300.0);
    let flux: HeatFluxDensity<f64> =
        StefanBoltzmannCoefficient::from(therm::STEFAN_BOLTZMANN_CONSTANT) * t.powi::<4>();
    close(
        HeatFluxDensityUnit::WattPerSquareMetre.value(flux),
        5.670374419e-8 * 300f64.powi(4),
    );
    // Only an amount's roots can fail: `t.root::<2>()` does not compile
    // (K^½).
}

#[test]
fn dynamic_powers_and_roots_are_checked_at_run_time() {
    let area = DynamicSIQuantity::from(AreaUnit::SquareMetre.quantity(9.0));
    let side = area.partial_root(2).unwrap();
    close(
        LengthUnit::Metre.value(Length::try_from(side).unwrap()),
        3.0,
    );
    let cube = DynamicSIQuantity::from(LengthUnit::Metre.quantity(2.0))
        .partial_powi(3)
        .unwrap();
    close(
        VolumeUnit::CubicMetre.value(Volume::try_from(cube).unwrap()),
        8.0,
    );
    // No exact root: the cube root of an area, the square root of a length.
    assert_eq!(area.partial_root(3), None);
    assert_eq!(side.partial_root(2), None);
    // An amount has every power, but not every root (K^½ has no integer
    // exponent); a temperature's power is an ordinary K², the same as an
    // interval's.
    let t = DynamicSIQuantity::from(ThermodynamicTemperatureUnit::Kelvin.quantity(300.0));
    let dt = DynamicSIQuantity::from(TemperatureIntervalUnit::Kelvin.quantity(3.0));
    let dt2 = dt.partial_powi(2).unwrap();
    assert!(f64::try_from((dt2 / dt2).unwrap()).is_ok());
    assert_eq!(dt.partial_root(2), None);
    let t2 = t.partial_powi(2).unwrap();
    assert!(f64::try_from((t2 / dt2).unwrap()).is_ok());
    assert_eq!(t.partial_root(2), None);
    // A level has neither.
    let level = DynamicSIQuantity::from(PowerLevel::from(mech::WATT_LEVEL));
    assert_eq!(level.partial_powi(2), None);
    assert_eq!(level.partial_root(2), None);
}

#[test]
fn dimension_powers_and_roots_directly() {
    // A power is repeated `*`; it may not exist (a level has none), so it
    // is an `Option`.
    assert_eq!(
        dim_of(mech::METRE).partial_powi(2),
        Some(dim_of(mech::SQUARE_METRE))
    );
    assert_eq!(
        dim_of(mech::METRE).partial_powi(-1),
        ISQ::DIMENSIONLESS / dim_of(mech::METRE)
    );
    assert_eq!(
        dim_of(mech::SQUARE_METRE).partial_root(2),
        Some(dim_of(mech::METRE))
    );
    assert_eq!(dim_of(mech::METRE).partial_root(2), None);
    assert_eq!(
        dim_of(yadc_si::angular::consts::RADIAN).partial_powi(2),
        Some(dim_of(yadc_si::angular::consts::STERADIAN))
    );
    assert_eq!(
        dim_of(yadc_si::angular::consts::STERADIAN).partial_root(2),
        Some(dim_of(yadc_si::angular::consts::RADIAN))
    );
    // A temperature's power is an ordinary one; a logarithm has no powers
    // and no roots.
    assert_eq!(
        dim_of(therm::ABSOLUTE_ZERO).partial_powi(2),
        dim_of(therm::KELVIN).partial_powi(2)
    );
    assert_eq!(dim_of(therm::ABSOLUTE_ZERO).partial_root(2), None);
    assert_eq!(dim_of(mech::WATT_LEVEL).partial_powi(2), None);
    assert_eq!(dim_of(mech::BEL).partial_root(2), None);
}

// --- Unit values: every constant is a unit; a scale is a pair of constants ---------

mod unit_values {
    use super::close;
    use yadc::{CastAs, Unit};
    use yadc_si::mechanical::consts as mech;
    use yadc_si::{KILO, MILLI, Scale};
    use yadc_si::mechanical::{Gain, Length, Power, PowerLevel, PowerUnit, Pressure};
    use yadc_si::thermal::consts as therm;
    use yadc_si::thermal::{
        TemperatureInterval, ThermodynamicTemperature, ThermodynamicTemperatureUnit,
    };

    type TemperatureScale = Scale<ThermodynamicTemperature<f64>, TemperatureInterval<f64>>;

    /// The exact constants, rounded to `f64` units once.
    const FOOT: Length<f64> = mech::FOOT.into();
    const METRE: Length<f64> = mech::METRE.into();
    const PASCAL: Pressure<f64> = mech::PASCAL.into();
    const KILOPASCAL: Pressure<f64> = (KILO * mech::PASCAL).into();
    const PSI: Pressure<f64> = mech::PSI.into();
    const WATT: Power<f64> = mech::WATT.into();
    const MILLIWATT: Power<f64> = (MILLI * mech::WATT).into();
    const DBM: Scale<PowerLevel<f64>, Gain<f64>> = mech::DBM.into();
    const ICE_POINT: ThermodynamicTemperature<f64> = therm::ICE_POINT.into();
    const KELVIN: TemperatureInterval<f64> = therm::KELVIN.into();
    const CELSIUS: TemperatureScale = therm::CELSIUS.into();
    const FAHRENHEIT: TemperatureScale = therm::FAHRENHEIT.into();
    const KELVIN_SCALE: TemperatureScale = therm::KELVIN_SCALE.into();
    const RANKINE: TemperatureScale = therm::RANKINE.into();

    /// Quantity constants and scales (a zero and a step) convert at compile
    /// time through `Unit`.
    const BODY: ThermodynamicTemperature<f64> = CELSIUS.quantity(37.0);
    const BODY_F: f64 = FAHRENHEIT.value(BODY);
    const ATM: Pressure<f64> = KILOPASCAL.quantity(101.325);
    const ATM_PSI: f64 = PSI.value(ATM);
    /// A scale is a constant like any other: so is one of yours.
    const REAUMUR: Scale<ThermodynamicTemperature<f64>, TemperatureInterval<f64>> =
        Scale(ICE_POINT, 1.25 * KELVIN);
    const BODY_RE: f64 = REAUMUR.value(BODY);

    #[test]
    fn constants_and_scales_are_units() {
        close(BODY_F, 98.6);
        close(BODY_RE, 29.6);
        close(KELVIN_SCALE.value(BODY), 310.15);
        close(RANKINE.value(BODY), 310.15 * 1.8);
        assert!((ATM_PSI - 14.6959).abs() < 1e-4, "{ATM_PSI}");
        // A scale made at run time: degrees above this morning's reading.
        let since_morning = Scale(CELSIUS.quantity(12.0), KELVIN);
        close(since_morning.value(CELSIUS.quantity(19.5)), 7.5);
        close(CELSIUS.value(since_morning.quantity(3.0)), 15.0);
        // A level, on the logarithmic line: 30 dBm is a watt.
        let p = Power::from(DBM.quantity(30.0));
        close(WATT.value(p), 1.0);
        close(DBM.value(PowerLevel::from(MILLIWATT.quantity(100.0))), 20.0);
        // The unit enum's variant prints: `{}` its symbol and `{:#}` its
        // name.
        use ThermodynamicTemperatureUnit::{Celsius, Fahrenheit};
        assert_eq!(
            format!("{Celsius} {Fahrenheit:#}"),
            "°C degrees Fahrenheit"
        );
        // Picked at run time, still checked: any unit of a pressure, of a
        // temperature.
        let any: [&dyn Unit<Pressure<f64>>; 2] = [&KILOPASCAL, &PASCAL];
        let q = KILOPASCAL.quantity(2.0);
        close(any[0].value(q) * 1000.0, any[1].value(q));
        let scales: [&dyn Unit<ThermodynamicTemperature<f64>>; 3] =
            [&CELSIUS, &FAHRENHEIT, &Celsius];
        close(scales[0].value(BODY), scales[2].value(BODY));
        close(scales[1].value(BODY), 98.6);
    }

    /// Every unit builds and reads through `Unit`: a quantity constant, a
    /// value of a unit enum generic over its number type, and a value of
    /// `PowerUnit` (whose number type is `f64`). A constant scaled by a
    /// number is that many of it.
    #[test]
    fn units_build_and_read() {
        // Quantity constants.
        let d: Length<f64> = FOOT.quantity(12.0);
        close(METRE.value(d), 12.0 * 0.3048);
        assert_eq!(d, FOOT * 12.0);
        close(PASCAL.value(ATM), 101_325.0);
        // A generic unit enum's value.
        let fahrenheit = ThermodynamicTemperatureUnit::Fahrenheit;
        close(fahrenheit.value(BODY), 98.6);
        close(CELSIUS.value(fahrenheit.quantity(212.0)), 100.0);
        // A level in the watts' unit enum: `via` its logarithmic line.
        let dbm = PowerUnit::DecibelMilliwatt;
        let p: Power<f64> = dbm.quantity(30.0);
        close(WATT.value(p), 1.0);
        close(dbm.value(MILLIWATT.quantity(100.0)), 20.0);
        close(DBM.value(PowerLevel::from(p)), 30.0);
        // Generic code asks for `Unit`.
        fn convert<Q>(n: f64, from: impl Unit<Q>, to: impl Unit<Q>) -> f64 {
            to.value(from.quantity(n))
        }
        close(convert(1.0, FOOT, METRE), 0.3048);
        use ThermodynamicTemperatureUnit::{Celsius, Fahrenheit};
        close(convert(100.0, Celsius, Fahrenheit), 212.0);
        close(convert(98.6, Fahrenheit, Celsius), 37.0);
        close(convert(30.0, dbm, WATT), 1.0);
    }

    /// The number type is a parameter: an exact constant rounds to the one
    /// it is converted to, a cast changes it, and std `From` widens it
    /// losslessly.
    #[test]
    fn other_value_types() {
        const KPA32: Pressure<f32> = (KILO * mech::PASCAL).into();
        let p32 = KPA32.quantity(2.5f32);
        assert_eq!(KPA32.value(p32), 2.5f32);
        let p: Pressure<f64> = Pressure::from(p32);
        close(KILOPASCAL.value(p), 2.5);
        assert_eq!(p.cast::<f32>(), p32);
    }
}
