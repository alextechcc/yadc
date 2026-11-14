//! The SI system at work: units build and read quantities, the static
//! quantities compose with typed operators, and the open type
//! (`DynamicSIQuantity`) decides at run time. A unit is a type of its own
//! (`mech::Metre`), a unit enum a choice among them (`LengthUnit`), and both
//! build and read in `const` items.
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![feature(const_trait_impl, const_ops)]
#![allow(incomplete_features)]

use yadc::{CastAs, Dimension, PartialPowi, PartialRoot, PowiAs, RootAs, Unit};
use yadc_si::angular::units as ang;
use yadc_si::angular::*;
use yadc_si::chemistry::units as chem;
use yadc_si::chemistry::{InverseAmountOfSubstance, MolarHeatCapacity};
use yadc_si::dimensionless::*;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::angular::*;
use yadc_si::dynamic::dimensionless::*;
use yadc_si::dynamic::mechanical::*;
use yadc_si::dynamic::thermal::*;
use yadc_si::electromagnetic::units as em;
use yadc_si::electromagnetic::{ElectricCharge, ElectricPermittivity, MagneticPermeability};
use yadc_si::mechanical::units as mech;
use yadc_si::mechanical::*;
use yadc_si::thermal::units as therm;
use yadc_si::thermal::*;
use yadc_si::{Exact, ISQ, Kind, Prefix, Prefixed, SIQuantity, SIUnit, SIUnitConsts};

/// A static quantity's dimension: its const parameter.
fn dim_of<const D: ISQ, N>(_: SIQuantity<D, N>) -> ISQ {
    D
}

/// A unit's exact size and zero, in coherent units of its quantity.
fn size_and_zero<const D: ISQ, const C: SIUnitConsts>(_: SIUnit<D, C>) -> (Exact, Exact) {
    (C.size, C.zero)
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

const KILOMETRE: Prefixed<LengthUnit> = Prefixed {
    prefix: Some(Prefix::Kilo),
    unit: LengthUnit::Metre,
};
const MILLIWATT: Prefixed<PowerUnit> = Prefixed {
    prefix: Some(Prefix::Milli),
    unit: PowerUnit::Watt,
};

#[test]
fn quantity_and_value_round_trip() {
    use MassUnit::{Gram, Kilogram, Pound};
    let mass = Gram.quantity(1500.0);
    close(Kilogram.value(mass), 1.5);
    close(Gram.value(mass), 1500.0);

    // Naming the target quantity.
    let m: Mass<f64> = Pound.quantity(10.0);
    close(Kilogram.value(m), 4.5359237); // the international pound, exactly
}

#[test]
fn units_build_quantities_and_quantities_compose() {
    use LengthUnit::Metre;
    use TimeUnit::{Hour, Second};
    use VelocityUnit::{KilometrePerHour, MetrePerSecond};

    // Units never take part in arithmetic: build quantities, then compute.
    let d: Length<f64> = KILOMETRE.quantity(90.0);
    let v: Velocity<f64> = d / Hour.quantity(1.0);
    close(MetrePerSecond.value(v), 25.0);
    close(KilometrePerHour.value(v), 90.0);
    close(
        MetrePerSecond.value(Metre.quantity(1.0) / Second.quantity(1.0)),
        1.0,
    );
    close(KILOMETRE.value(v * Hour.quantity(1.0)), 90.0);
    // A unit type and a unit enum's value are units alike.
    close(mech::Mile {}.value(d), 90.0 / 1.609344);
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

    // Same-dim division yields Dimensionless, which converts to its number.
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
    let radian: Angle<f64> = ang::Radian {}.quantity(1.0);
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
    let arc: Length<f64> = r * AngleUnit::Radian.quantity(1.5) / radian;
    close(LengthUnit::Metre.value(arc), 3.0);
    let rim: Velocity<f64> = AngularVelocityUnit::RadianPerSecond.quantity(3.0) * r / radian;
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
fn gains_are_logarithms_of_ratios() {
    use GainUnit::{Bel, Decibel, Neper};

    // A gain is a quantity of its own, with plain units: decibels add.
    let g: Gain<f64> = Decibel.quantity(3.0) + Decibel.quantity(7.0);
    close(Decibel.value(g), 10.0);
    close(Bel.value(g), 1.0);
    close(Bel.value(Decibel.quantity(20.0)), 2.0);
    // 1 Np = 20 / ln 10 dB.
    let neper: Gain<f64> = Neper.quantity(1.0);
    close(Decibel.value(neper), 20.0 / std::f64::consts::LN_10);
    close(Neper.value(Neper.quantity(2.0)), 2.0);
    assert_eq!(format!("{Decibel} {Decibel:#}"), "dB decibels");

    // The plain ratio a gain is, and back: a power ratio, as for the bel.
    let r = Dimensionless::from(Decibel.quantity(3.0)); // ~2x power
    close(f64::from(r), 10f64.powf(0.3));
    close(Decibel.value(Gain::from(r)), 3.0);
    close(f64::from(Dimensionless::from(Bel.quantity(1.0))), 10.0);
    close(
        f64::from(Dimensionless::from(neper)),
        std::f64::consts::E.powi(2),
    );
    close(
        Decibel.value(Gain::from(DimensionlessUnit::Ratio.quantity(1000.0))),
        30.0,
    );
}

#[test]
fn power_levels_are_on_the_logarithmic_line() {
    use GainUnit::Decibel;
    use PowerLevelUnit::{DecibelMilliwatt, DecibelWatt};
    use PowerUnit::Watt;

    // A level's units count decibels from a zero; the watts are one std
    // `From` away, either way.
    let level = DecibelMilliwatt.quantity(30.0);
    let p = Power::from(level);
    close(Watt.value(p), 1.0);
    close(DecibelWatt.value(level), 0.0);
    close(
        DecibelMilliwatt.value(PowerLevel::from(MILLIWATT.quantity(100.0))),
        20.0,
    );
    close(
        DecibelMilliwatt.value(PowerLevel::from(MILLIWATT.quantity(1.0))),
        0.0,
    );

    // A link budget, in decibels: on the line, multiplying is adding.
    let received = level - Decibel.quantity(3.0) + Decibel.quantity(13.0);
    close(DecibelMilliwatt.value(received), 40.0);
    close(Watt.value(Power::from(received)), 10.0);
    // Two levels differ by a gain; a gain moves a level.
    let gain: Gain<f64> = received - level;
    close(Decibel.value(gain), 10.0);
    close(DecibelMilliwatt.value(level + gain), 40.0);
    // The linear side and the line agree: doubling the watts is 3.01 dB.
    close(
        Decibel.value(PowerLevel::from(2.0 * p) - level),
        10.0 * 2f64.log10(),
    );
    // The unit types are the enum's units.
    close(
        mech::DecibelWatt {}.value(mech::DecibelMilliwatt {}.quantity(30.0_f64)),
        0.0,
    );
    assert_eq!(
        format!("{DecibelMilliwatt} {DecibelWatt:#}"),
        "dBm decibels-watt"
    );

    // The dynamic type.
    let d = DynamicSIQuantity::from(level);
    assert_eq!(PowerLevel::try_from(d), Ok(level));
    assert!(Power::try_from(d).is_err());
    // A level and a gain have a sum; two levels sum to the logarithm of watts
    // squared, which is no power level; and a level cannot be scaled.
    assert!((d + d).is_some_and(|sum| PowerLevel::try_from(sum).is_err()));
    assert_eq!(d * 2.0, None);
    let moved = (d + DynamicSIQuantity::from(gain)).unwrap();
    close(
        DecibelMilliwatt.value(PowerLevel::try_from(moved).unwrap()),
        40.0,
    );
    assert_eq!((moved - d).map(|x| Gain::try_from(x).is_ok()), Some(true));
}

#[test]
fn display_quantity_via_unit() {
    use PowerLevelUnit::DecibelWatt;
    use PowerUnit::Watt;
    let p = Watt.quantity(100.0);
    let level = PowerLevel::from(p);
    // The number read in the unit, then the unit's own `Display`: `{}` is
    // its symbol ...
    assert_eq!(
        format!("{:.0} {}", DecibelWatt.value(level), DecibelWatt),
        "20 dBW"
    );
    // ... and `{:#}` its long name.
    assert_eq!(
        format!("{:.0} {:#}", DecibelWatt.value(level), DecibelWatt),
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

    let distance: Length<f64> = KILOMETRE.quantity(90.0);
    let one_km: Length<f64> = KILOMETRE.quantity(1.0);
    let d = DynamicSIQuantity::from(distance);
    let hour = DynamicSIQuantity::from(Hour.quantity(1.0));
    let km = DynamicSIQuantity::from(one_km);
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

/// A unit of any dimension, chosen at run time, builds and reads the open
/// type directly, and reads `None` for another dimension.
#[test]
fn a_dynamic_unit_reads_the_open_type() {
    use yadc_si::dynamic::DynamicSIUnit;
    let mile = DynamicSIUnit::from(mech::Mile {});
    let hour = DynamicSIUnit::from(TimeUnit::Hour);
    let mph = DynamicSIUnit::from(VelocityUnit::MilePerHour);
    let v = mile
        .quantity(Some(60.0))
        .zip(hour.quantity(Some(1.5)))
        .and_then(|(d, t)| d / t);
    close(mph.value(v).unwrap(), 40.0);
    assert_eq!(mile.value(v), None);
    assert_eq!(mph.value(None), None);
    assert_eq!(format!("{mph} {mph:#}"), "mi/h miles per hour");
    assert_eq!(mech::Mile::try_from(mile), Ok(mech::Mile {}));
    assert!(mech::Mile::try_from(hour).is_err());
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
    let ice_point = Celsius.quantity(0.0);
    close(ice_point.value(doubled), 666.3 / 273.15);
    // It multiplies, and the label drops out of the product: the ideal gas
    // law, p V = n R T, and a fourth power for radiation.
    let n_r: HeatCapacity<f64> =
        chem::Mole {}.quantity(1.0) * chem::MolarGasConstant {}.quantity(1.0);
    let energy = n_r * warmer;
    close(EnergyUnit::Joule.value(energy), 8.314462618 * 333.15);
    let fourth = warmer.powi::<4>();
    assert_eq!(dim_of(fourth).kind, Kind::Linear);
    assert_eq!(dim_of(fourth).K, 4);
    let ratio = hot / cold;
    close(f64::from(ratio), 373.15 / 293.15);
}

#[test]
fn kinds_decide_sums_and_products() {
    // A quantity's dimension is its type's const parameter; each module
    // names its quantities' dimensions.
    let absolute = THERMODYNAMIC_TEMPERATURE;
    let step = TEMPERATURE_INTERVAL;
    let metre = LENGTH;
    assert_eq!(
        dim_of(ThermodynamicTemperatureUnit::Kelvin.quantity(1.0)),
        absolute
    );
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
    assert_eq!(MASS * ACCELERATION, Some(FORCE));
    assert_eq!(MASS + metre, None);
    // One over an angle exists: `rad: -1`.
    assert_eq!(
        ISQ::DIMENSIONLESS / ANGLE,
        Some(ISQ {
            rad: -1,
            ..ISQ::DIMENSIONLESS
        })
    );
    assert_eq!(
        ISQ::DIMENSIONLESS,
        dim_of(DimensionlessUnit::Ratio.quantity(1.0))
    );
    // Logarithmic: `+` and `-` add and subtract the exponents - adding
    // logarithms multiplies what they are the logarithms of.
    let (level, gain, watt) = (POWER_LEVEL, GAIN, POWER);
    assert_eq!(
        level,
        ISQ {
            kind: Kind::Log,
            ..watt
        }
    );
    assert_eq!(level - level, Some(gain));
    assert_eq!(level + gain, Some(level));
    assert_eq!(level - gain, Some(level));
    assert_eq!(gain + gain, Some(gain));
    assert_eq!(
        level + level,
        Some(ISQ {
            kind: Kind::Log,
            ..(watt * watt).unwrap()
        })
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
    let one_km: Length<f64> = KILOMETRE.quantity(1.0);
    let attenuation: yadc_si::density::LinearNumberDensity<f64> =
        GainUnit::Decibel.quantity(0.2) / one_km;
    // Over 15 km it is a plain number again - of bels; that many bels is the
    // gain.
    let span: Length<f64> = KILOMETRE.quantity(15.0);
    let bels: Dimensionless<f64> = attenuation * span;
    let loss: Gain<f64> = bels * GainUnit::Bel.quantity(1.0);
    close(GainUnit::Decibel.value(loss), 3.0);
    // Launched at 0 dBm, the level after the fibre.
    let launched: PowerLevel<f64> = PowerLevelUnit::DecibelMilliwatt.quantity(0.0);
    let received: PowerLevel<f64> = launched - loss;
    close(PowerLevelUnit::DecibelMilliwatt.value(received), -3.0);
    let power: Power<f64> = received.into();
    close(MILLIWATT.value(power), 10f64.powf(-0.3));
}

/// An instant is a point on the one line of times (the affine kind): two
/// differ by a `Time`, a `Time` moves one, and every unit of it - Unix
/// seconds, GPS seconds, Julian dates - reads the same line. Not compiled,
/// because refused: `a + b` ("cannot add a point to a point"), `2.0 * a`
/// ("a point cannot be scaled"), `a * hour` ("these have no product"),
/// `hour + a` ("cannot add these in this order").
#[test]
fn an_instant_is_a_point_in_time() {
    use InstantUnit::{GpsSecond, JulianDate, ModifiedJulianDate, UnixMillisecond, UnixSecond};
    use TimeUnit::{Day, Hour, Second};

    // Point algebra.
    let launch: Instant<f64> = UnixSecond.quantity(1_700_000_000.0);
    let landing = launch + Hour.quantity(2.0);
    let flight: Time<f64> = landing - launch;
    close(Hour.value(flight), 2.0);
    close(
        UnixSecond.value(landing - Second.quantity(30.0)),
        1_700_007_170.0,
    );
    assert!(launch < landing);
    let mut t = launch;
    t += Day.quantity(1.0);
    t -= Hour.quantity(1.0);
    close(Hour.value(t - launch), 23.0);
    assert_eq!(dim_of(launch).kind, Kind::Affine);
    assert_eq!(dim_of(flight), TIME);

    // One line, many units: each counts from its own zero on it.
    let gps_epoch: Instant<f64> = GpsSecond.quantity(0.0);
    let unix_epoch: Instant<f64> = UnixSecond.quantity(0.0);
    close(UnixSecond.value(gps_epoch), 315_964_800.0);
    close(GpsSecond.value(unix_epoch), -315_964_800.0);
    close(JulianDate.value(unix_epoch), 2_440_587.5);
    close(ModifiedJulianDate.value(unix_epoch), 40_587.0);
    close(ModifiedJulianDate.value(gps_epoch), 44_244.0);
    // An instant built in one unit reads in any other: J2000.0, noon on
    // 1 January 2000, is JD 2451545.0 and Unix 946728000.
    let j2000 = JulianDate.quantity(2_451_545.0);
    close(UnixSecond.value(j2000), 946_728_000.0);
    close(GpsSecond.value(j2000), 946_728_000.0 - 315_964_800.0);
    close(Day.value(j2000 - gps_epoch), 7_300.5);
    // Stored as `f64` seconds from the Unix epoch: today's instants resolve
    // to a fraction of a microsecond, so a millisecond count survives.
    let precise: Instant<f64> = UnixMillisecond.quantity(1_700_000_000_123.0);
    assert!((UnixSecond.value(precise) - 1_700_000_000.123).abs() < 1e-6);
    assert!((UnixMillisecond.value(precise) - 1_700_000_000_123.0).abs() < 1e-3);
    // The unit types read at compile time.
    const EPOCH_JD: f64 = mech::JulianDate {}.value(mech::UnixSecond {}.quantity(0.0));
    close(EPOCH_JD, 2_440_587.5);

    // The unit enum: a menu of them.
    close(UnixSecond.value(GpsSecond.quantity(0.0)), 315_964_800.0);
    close(
        ModifiedJulianDate.value(JulianDate.quantity(2_400_000.5)),
        0.0,
    );
    close(UnixMillisecond.value(UnixSecond.quantity(1.5)), 1_500.0);
    assert_eq!(
        format!("{UnixSecond} / {JulianDate:#}"),
        "s Unix / Julian date"
    );
    assert_eq!("MJD".parse(), Ok(ModifiedJulianDate));
    assert_eq!(InstantUnit::default(), UnixSecond);

    // At run time: the same algebra, `None` where there is no result.
    let (a, b) = (
        DynamicSIQuantity::from(launch),
        DynamicSIQuantity::from(landing),
    );
    let two_hours = DynamicSIQuantity::from(flight);
    assert_eq!((b - a).and_then(|d| Time::try_from(d).ok()), Some(flight));
    assert_eq!(
        (a + two_hours).and_then(|d| Instant::try_from(d).ok()),
        Some(landing)
    );
    assert_eq!(a + b, None);
    assert_eq!(two_hours + a, None);
    assert_eq!(a * 2.0, None);
    assert_eq!(a * two_hours, None);
    assert_eq!(a / two_hours, None);
    assert_eq!(a.partial_powi(2), None);
    assert!(Time::try_from(a).is_err());
    assert_eq!(a.partial_cmp(&b), Some(core::cmp::Ordering::Less));
    assert_eq!(a.partial_cmp(&two_hours), None);
    let shown: Instant<f64> = UnixSecond.quantity(12.5);
    assert_eq!(format!("{shown}"), "12.5 s{Affine}");
}

/// Generic code reads and builds a value through any unit it is handed: the
/// unit is the only way between numbers and quantities, so it is all the
/// code needs - offset units included.
fn number_of<Q, U: Unit<Q, f64>>(unit: &U, q: Q) -> f64 {
    unit.value(q)
}
fn from_number<Q, U: Unit<Q, f64>>(unit: &U, n: f64) -> Q {
    unit.quantity(n)
}

#[test]
fn generic_code_reads_and_builds_through_a_unit() {
    use ThermodynamicTemperatureUnit::{Celsius, Kelvin};
    let t: ThermodynamicTemperature<f64> = from_number(&Kelvin, 300.0);
    close(number_of(&Celsius, t), 26.85);
    close(number_of(&Kelvin, t), 300.0);
    close(number_of(&therm::Fahrenheit {}, t), 80.33);
    let m: Mass<f64> = from_number(&MassUnit::Kilogram, 2.5);
    close(number_of(&MassUnit::Gram, m), 2500.0);
    // A unit picked at run time: `&dyn Unit<Q, N>`.
    let units: [&dyn Unit<Mass<f64>, f64>; 3] =
        [&MassUnit::Kilogram, &MassUnit::Pound, &mech::Ounce {}];
    close(number_of(&units[0], m), 2.5);
    close(number_of(&units[2], m) / number_of(&units[1], m), 16.0);
}

// --- Units without a home ------------------------------------------------------

#[test]
fn units_picked_at_run_time() {
    use LengthUnit::{Metre, Mile};
    use MassUnit::{Gram, Kilogram, Pound};

    // A unit need not print, so a list of units that do is your own trait:
    // a unit of `Q` that is also `Display` (`{}` the symbol, `{:#}` the
    // name).
    trait Shown<Q>: Unit<Q, f64> + core::fmt::Display {}
    impl<Q, U: Unit<Q, f64> + core::fmt::Display> Shown<Q> for U {}

    // Some unit of length: chosen at run time, still checked at compile time
    // (a mass unit would not fit in this array).
    let picks: [&dyn Shown<Length<f64>>; 4] = [&Metre, &KILOMETRE, &Mile, &mech::Foot {}];
    let d = picks[1].quantity(3.0);
    let shown: Vec<String> = picks
        .iter()
        .map(|u| format!("{:.1} {}", u.value(d), u))
        .collect();
    assert_eq!(shown, ["3000.0 m", "3.0 km", "1.9 mi", "9842.5 ft"]);

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
    // its "home" - each is one `Unit` impl, and its `Display` is its own.
    #[derive(Copy, Clone)]
    struct Kelvins;
    impl Unit<TemperatureInterval<f64>, f64> for Kelvins {
        fn quantity(&self, n: f64) -> TemperatureInterval<f64> {
            therm::Kelvin {}.quantity(n)
        }
        fn value(&self, q: TemperatureInterval<f64>) -> f64 {
            therm::Kelvin {}.value(q)
        }
    }
    impl Unit<ThermodynamicTemperature<f64>, f64> for Kelvins {
        fn quantity(&self, n: f64) -> ThermodynamicTemperature<f64> {
            therm::KelvinScale {}.quantity(n)
        }
        fn value(&self, t: ThermodynamicTemperature<f64>) -> f64 {
            therm::KelvinScale {}.value(t)
        }
    }
    impl core::fmt::Display for Kelvins {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.pad(if f.alternate() { "kelvin" } else { "K" })
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
    /// A unit that is a pair of functions: a type of ours.
    #[derive(Copy, Clone)]
    struct Beaufort;
    impl Unit<Velocity<f64>, f64> for Beaufort {
        fn quantity(&self, b: f64) -> Velocity<f64> {
            mech::MetrePerSecond {}.quantity(0.836 * b.powf(1.5))
        }
        fn value(&self, v: Velocity<f64>) -> f64 {
            (mech::MetrePerSecond {}.value(v) / 0.836).powf(2.0 / 3.0)
        }
    }

    let gale = Beaufort.quantity(5.0);
    close(mech::MetrePerSecond {}.value(gale), 0.836 * 5f64.powf(1.5));
    close(Beaufort.value(gale), 5.0);
    // Any unit of `Velocity` reads it, and it reads any.
    let kmh = VelocityUnit::KilometrePerHour.value(gale);
    close(
        Beaufort.value(VelocityUnit::KilometrePerHour.quantity(kmh)),
        5.0,
    );
    // Picked at run time beside the linear ones.
    let menu: [&dyn Unit<Velocity<f64>, f64>; 3] =
        [&mech::MetrePerSecond {}, &VelocityUnit::Knot, &Beaufort];
    close(
        menu[2].value(menu[0].quantity(20.0)),
        (20.0f64 / 0.836).powf(2.0 / 3.0),
    );
    // ... and from the open type, through `Velocity`.
    let d = DynamicSIQuantity::from(mech::MetrePerSecond {}.quantity(20.0));
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

    // Zeros: the zero point is the unit's business on the dynamic side too.
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
        f64::try_from(
            (per_rad * DynamicSIQuantity::from(AngleUnit::Radian.quantity(1.0))).unwrap(),
        )
        .unwrap(),
        0.5,
    );
    let per_rad = AngleUnit::Radian.quantity(2.0).inv();
    close(f64::from(per_rad * AngleUnit::Radian.quantity(1.0)), 0.5);
}

// --- Compile-time units ---------------------------------------------------------

// A unit's `Unit` impl is `const`, its size and zero rounded to the number
// type once, by the compiler: quantities built in `const` items compose
// there, zero points and all.
const LIMIT: Velocity<f64> = mech::KilometrePerHour {}.quantity(50.0);
const ELECTRON_REST_ENERGY: Energy<f64> = mech::ElectronMass {}.quantity(1.0)
    * mech::SpeedOfLight {}.quantity(1.0)
    * mech::SpeedOfLight {}.quantity(1.0);
const BOILING: ThermodynamicTemperature<f64> =
    ThermodynamicTemperatureUnit::Celsius.quantity(100.0);
const BOILING_F: f64 = ThermodynamicTemperatureUnit::Fahrenheit.value(BOILING);

#[test]
fn const_items_build_and_read() {
    close(VelocityUnit::MetrePerSecond.value(LIMIT), 50.0 / 3.6);
    // E = mc² for the electron: 8.187e-14 J (0.511 MeV).
    let joules = EnergyUnit::Joule.value(ELECTRON_REST_ENERGY);
    assert!((joules - 8.1871057769e-14).abs() < 1e-23, "got {joules}");
    close(ThermodynamicTemperatureUnit::Kelvin.value(BOILING), 373.15);
    close(BOILING_F, 212.0);
}

#[test]
fn units_carry_their_exact_definitions() {
    use yadc_si::dynamic::fluid::{DiffusionCoefficientUnit, KinematicViscosityUnit};
    use yadc_si::fluid::units as fluid;

    // Exact, by the 1959 agreement.
    assert_eq!(
        size_and_zero(mech::Pound {}),
        (Exact::dec(45359237, -8), Exact::ZERO)
    );
    close(
        MassUnit::Kilogram.value(mech::Pound {}.quantity(1.0)),
        0.45359237,
    );
    assert_eq!(size_and_zero(mech::WattHour {}).0, Exact::int(3600));
    close(
        f64::from(yadc_si::dimensionless::units::Percent {}.quantity(1.0_f64)),
        0.01,
    );
    // A temperature unit is a size and a zero: °C counts kelvin-sized steps
    // from the ice point, °F steps of 5/9 K from 459.67 °F below it.
    assert_eq!(
        size_and_zero(therm::Celsius {}),
        (Exact::ONE, Exact::dec(27315, -2))
    );
    assert_eq!(
        size_and_zero(therm::DegreeFahrenheit {}).0,
        Exact::int(5) / Exact::int(9)
    );
    assert_eq!(
        size_and_zero(therm::Fahrenheit {}).1,
        Exact::dec(45967, -2) * Exact::int(5) / Exact::int(9)
    );
    close(
        ThermodynamicTemperatureUnit::Fahrenheit.value(therm::KelvinScale {}.quantity(0.0)),
        -459.67,
    );
    close(
        TemperatureIntervalUnit::Celsius.value(therm::Kelvin {}.quantity(1.0)),
        1.0,
    );
    // Modules are namespaces: a minute of time and a minute of arc coexist.
    close(
        mech::Second {}.value(mech::Minute {}.quantity(1.0_f64)),
        60.0,
    );
    assert_eq!(
        size_and_zero(ang::Minute {}).0 / size_and_zero(ang::Degree {}).0,
        Exact::int(1) / Exact::int(60)
    );
    // Within one module, a shared name is prefixed with the quantity.
    close(
        DiffusionCoefficientUnit::SquareMetrePerSecond
            .value(fluid::DiffusionCoefficientStokes {}.quantity(1.0)),
        1e-4,
    );
    close(
        KinematicViscosityUnit::SquareMetrePerSecond
            .value(fluid::KinematicViscosityStokes {}.quantity(1.0)),
        1e-4,
    );
}

#[test]
fn physical_constants_are_units_with_their_quantity_types() {
    // Standard gravity times a mass is a force (a weight).
    let weight: Force<f64> =
        mech::StandardGravity {}.quantity(1.0) * MassUnit::Kilogram.quantity(70.0);
    close(ForceUnit::Newton.value(weight), 686.4655);
    // k·T with T an interval is an energy: kT at 300 K ≈ 4.14e-21 J.
    let kt: Energy<f64> =
        therm::BoltzmannConstant {}.quantity(1.0) * TemperatureIntervalUnit::Kelvin.quantity(300.0);
    close(EnergyUnit::Joule.value(kt) * 1e21, 4.141947);
    // R = N_A · k, within the CODATA rounding. (The type annotation is the
    // dimension check: mol⁻¹ · J/K is J/(mol·K).)
    let r: MolarHeatCapacity<f64> =
        chem::AvogadroConstant {}.quantity(1.0) * therm::BoltzmannConstant {}.quantity(1.0);
    assert!((chem::MolarGasConstant {}.value(r) - 1.0).abs() < 1e-9);
    // ħ = h / 2π rad: an angular momentum (J·s per radian), not an action.
    let hbar: AngularMomentum<f64> = mech::PlanckConstant {}.quantity(1.0)
        / AngleUnit::Radian.quantity(2.0 * std::f64::consts::PI);
    assert!((AngularMomentumUnit::ReducedPlanckConstant.value(hbar) - 1.0).abs() < 1e-9);
    // 1 atm in pascals, exactly, and the elementary charge in coulombs.
    assert_eq!(
        size_and_zero(mech::StandardAtmosphere {}).0,
        Exact::int(101_325)
    );
    let e: ElectricCharge<f64> = em::ElementaryCharge {}.quantity(1.0);
    close(em::Coulomb {}.value(e) * 1e19, 1.602176634);
    // ε₀ μ₀ c² = 1, to the precision of the CODATA values.
    let c: Velocity<f64> = mech::SpeedOfLight {}.quantity(1.0);
    let eps0: ElectricPermittivity<f64> = em::VacuumPermittivity {}.quantity(1.0);
    let mu0: MagneticPermeability<f64> = em::VacuumPermeability {}.quantity(1.0);
    let one: Dimensionless<f64> = eps0 * mu0 * c * c;
    assert!((f64::from(one) - 1.0).abs() < 1e-9);
    // G M / r² at the Earth's surface is about standard gravity.
    let earth_mass = MassUnit::Kilogram.quantity(5.9722e24);
    let earth_radius: Length<f64> = KILOMETRE.quantity(6371.0);
    let g: Acceleration<f64> =
        mech::GravitationalConstant {}.quantity(1.0) * earth_mass / (earth_radius * earth_radius);
    assert!((AccelerationUnit::StandardGravity.value(g) - 1.0).abs() < 0.01);
    // σ T⁴: a black body at 300 K radiates about 459 W/m².
    let t = TemperatureIntervalUnit::Kelvin.quantity(300.0);
    let sigma: StefanBoltzmannCoefficient<f64> = therm::StefanBoltzmannConstant {}.quantity(1.0);
    let exitance = sigma * t * t * t * t;
    let w_per_m2 = PowerUnit::Watt
        .value(exitance * LengthUnit::Metre.quantity(1.0) * LengthUnit::Metre.quantity(1.0));
    assert!((w_per_m2 - 459.3).abs() < 0.1, "got {w_per_m2}");
}

#[test]
fn velocities_read_in_units_of_c() {
    // A physical constant is a unit: read a speed as a fraction of c.
    let v = VelocityUnit::MetrePerSecond.quantity(29_979_245.8);
    close(VelocityUnit::SpeedOfLight.value(v), 0.1);
    // ... and build one from it.
    let half_c = VelocityUnit::SpeedOfLight.quantity(0.5);
    close(VelocityUnit::MetrePerSecond.value(half_c), 149_896_229.0);
    let c = VelocityUnit::SpeedOfLight;
    assert_eq!(format!("{:.2} {}", c.value(half_c), c), "0.50 c");
    // The Avogadro constant: one mole of things, counted.
    let n_a: InverseAmountOfSubstance<f64> = chem::AvogadroConstant {}.quantity(1.0);
    let per_mole = yadc_si::dynamic::chemistry::InverseAmountOfSubstanceUnit::PerMole.value(n_a);
    close(per_mole / 1e23, 6.02214076);
}

// --- Powers and roots -----------------------------------------------------------

const CUBIC_METRE_BOX: Volume<f64> = mech::Metre {}.quantity(1.0).powi::<3>();

#[test]
fn integer_powers_and_exact_roots() {
    let centimetre = Prefixed {
        prefix: Some(Prefix::Centi),
        unit: LengthUnit::Metre,
    };
    let square_centimetre = Prefixed {
        prefix: Some(Prefix::Centi),
        unit: AreaUnit::SquareMetre,
    };
    let edge: Length<f64> = centimetre.quantity(20.0);
    let face: Area<f64> = edge.powi::<2>();
    let cube: Volume<f64> = edge.powi::<3>();
    close(square_centimetre.value(face), 400.0);
    close(VolumeUnit::Litre.value(cube), 8.0);
    // A root is the exact inverse of a power, dimension and all.
    let side: Length<f64> = face.root::<2>();
    close(centimetre.value(side), 20.0);
    let side3: Length<f64> = cube.root::<3>();
    close(centimetre.value(side3), 20.0);
    // Negative powers: one over a time is a frequency, one over an area ...
    let millisecond = Prefixed {
        prefix: Some(Prefix::Milli),
        unit: TimeUnit::Second,
    };
    let period: Time<f64> = millisecond.quantity(2.0);
    let f: Frequency<f64> = period.powi::<-1>();
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
    let sigma: StefanBoltzmannCoefficient<f64> = therm::StefanBoltzmannConstant {}.quantity(1.0);
    let flux: HeatFluxDensity<f64> = sigma * t.powi::<4>();
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
    let level = DynamicSIQuantity::from(PowerLevelUnit::DecibelWatt.quantity(0.0));
    assert_eq!(level.partial_powi(2), None);
    assert_eq!(level.partial_root(2), None);
}

#[test]
fn dimension_powers_and_roots_directly() {
    // A power is repeated `*`; it may not exist (a level has none), so it
    // is an `Option`.
    assert_eq!(LENGTH.partial_powi(2), Some(AREA));
    assert_eq!(LENGTH.partial_powi(-1), ISQ::DIMENSIONLESS / LENGTH);
    assert_eq!(AREA.partial_root(2), Some(LENGTH));
    assert_eq!(LENGTH.partial_root(2), None);
    assert_eq!(ANGLE.partial_powi(2), Some(SOLID_ANGLE));
    assert_eq!(SOLID_ANGLE.partial_root(2), Some(ANGLE));
    // A temperature's power is an ordinary one; a logarithm has no powers
    // and no roots.
    assert_eq!(
        THERMODYNAMIC_TEMPERATURE.partial_powi(2),
        TEMPERATURE_INTERVAL.partial_powi(2)
    );
    assert_eq!(THERMODYNAMIC_TEMPERATURE.partial_root(2), None);
    assert_eq!(POWER_LEVEL.partial_powi(2), None);
    assert_eq!(GAIN.partial_root(2), None);
}

// --- Units of your own, in `const` items ------------------------------------------

/// The degree Réaumur: a unit of yours, a size and a zero - 1.25 K steps
/// from the ice point.
type Reaumur = SIUnit<
    THERMODYNAMIC_TEMPERATURE,
    {
        SIUnitConsts {
            size: Exact::dec(125, -2),
            zero: Exact::dec(27315, -2),
            symbol: "°Ré",
            name: "degrees Réaumur",
        }
    },
>;

/// Built and read at compile time through `Unit`.
const BODY: ThermodynamicTemperature<f64> = therm::Celsius {}.quantity(37.0);
const BODY_F: f64 = therm::Fahrenheit {}.value(BODY);
const BODY_RE: f64 = Reaumur {}.value(BODY);
const ATM: Pressure<f64> = mech::StandardAtmosphere {}.quantity(1.0);
const ATM_PSI: f64 = mech::Psi {}.value(ATM);

#[test]
fn units_build_and_read_in_const_items() {
    close(BODY_F, 98.6);
    close(BODY_RE, 29.6);
    close(therm::KelvinScale {}.value(BODY), 310.15);
    close(therm::Rankine {}.value(BODY), 310.15 * 1.8);
    assert!((ATM_PSI - 14.6959).abs() < 1e-4, "{ATM_PSI}");
    close(mech::Pascal {}.value(ATM), 101_325.0);
    assert_eq!(
        format!("{} {:#}", Reaumur {}, Reaumur {}),
        "°Ré degrees Réaumur"
    );
    // Picked at run time, still checked: any unit of a temperature.
    let scales: [&dyn Unit<ThermodynamicTemperature<f64>, f64>; 3] = [
        &therm::Celsius {},
        &Reaumur {},
        &ThermodynamicTemperatureUnit::Fahrenheit,
    ];
    close(scales[0].value(BODY), 37.0);
    close(scales[1].value(BODY), 29.6);
    close(scales[2].value(BODY), 98.6);
    // Generic code asks for `Unit`.
    fn convert<Q>(n: f64, from: impl Unit<Q, f64>, to: impl Unit<Q, f64>) -> f64 {
        to.value(from.quantity(n))
    }
    close(convert(1.0, mech::Foot {}, mech::Metre {}), 0.3048);
    close(
        convert(100.0, therm::Celsius {}, therm::Fahrenheit {}),
        212.0,
    );
    close(convert(80.0, Reaumur {}, therm::Celsius {}), 100.0);
}

/// The number type is a parameter: a unit rounds its size to the one it
/// counts in, a cast changes it, and std `From` widens it losslessly.
#[test]
fn other_number_types() {
    let p32: Pressure<f32> = mech::Pascal {}.quantity(2500.0_f32);
    assert_eq!(mech::Bar {}.value(p32), 0.025_f32);
    let p: Pressure<f64> = Pressure::from(p32);
    close(mech::Pascal {}.value(p), 2500.0);
    assert_eq!(p.cast::<f32>(), p32);
}
