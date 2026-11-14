//! Tour of the `yadc-si` quantity system, one feature per section.
//!
//! Run with: cargo run -p yadc --example basic_use

// `generic_const_exprs` is all a crate using `yadc-si` needs; `const_trait_impl`
// and `const_ops` only because this one builds quantities and does their
// arithmetic in `const` items (section 11).
#![feature(generic_const_exprs, const_trait_impl, const_ops)]
#![allow(incomplete_features)]

use yadc::{CastAs, PowiAs, Unit};
use yadc_si::dimensionless::*;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::dynamic::dimensionless::*;
use yadc_si::dynamic::electromagnetic::*;
use yadc_si::dynamic::mechanical::*;
use yadc_si::dynamic::thermal::*;
use yadc_si::electromagnetic::*;
use yadc_si::mechanical::units::{
    DECIBEL, DECIBEL_MILLIWATT, ELECTRON_MASS, GRAM, HOUR, INCH, KILOGRAM, KILOMETRE_PER_HOUR,
    METRE, METRE_PER_SECOND, POUND, SPEED_OF_LIGHT, STANDARD_GRAVITY, WATT,
};
use yadc_si::mechanical::*;
use yadc_si::thermal::units::{CELSIUS, KELVIN};
use yadc_si::thermal::*;
use yadc_si::{Prefix, Prefixed, SIQuantity};

// Compile-time quantities. A unit of `yadc-si` is a constant, and its
// conversion is `const`: `KILOMETRE_PER_HOUR.quantity(50.0)` is folded by the
// compiler, and so is arithmetic on the quantities it builds.
const SPEED_LIMIT: SIQuantity<VELOCITY, f64> = KILOMETRE_PER_HOUR.quantity(50.0);
const C: SIQuantity<VELOCITY, f64> = SPEED_OF_LIGHT.quantity(1.0);
const ELECTRON_REST_ENERGY: SIQuantity<ENERGY, f64> = ELECTRON_MASS.quantity(1.0) * C * C; // E = mc²
// A pressure defined the way a standards document writes it: a pound-force
// per square inch is a pound under standard gravity, on a square inch.
const G_N: SIQuantity<ACCELERATION, f64> = STANDARD_GRAVITY.quantity(1.0);
const PSI: SIQuantity<PRESSURE, f64> =
    POUND.quantity(1.0) * G_N / (INCH.quantity(1.0) * INCH.quantity(1.0));

fn main() {
    // --- 1. A unit turns a number into a quantity and back ------------------
    // A unit is the `Unit` trait: `quantity` builds, `value` reads. In
    // `yadc-si` each unit is a constant whose type holds its definition:
    // `KILOGRAM.quantity(70.0)` *is* 70 kg (stored in SI base units), and
    // `KILOGRAM.value(m)` reads a mass back in kilograms. A unit enum
    // names units, for printing and for picking at run time:
    // `MassUnit::Kilogram.quantity(70.0)` builds the same mass. A unit
    // prints itself: `{}` is its symbol, `{:#}` its long name. So a quantity
    // shown in a unit is the number the unit reads, then the unit.

    let kg = KILOGRAM;
    let mass = kg.quantity(70.0_f64);
    assert_eq!(MassUnit::Kilogram.quantity(70.0), mass);
    println!("mass  = {} {kg:#}", kg.value(mass));
    let g0 = AccelerationUnit::StandardGravity;
    let accel = g0.quantity(1.0_f64);
    println!("accel = {} {g0:#}", g0.value(accel));

    // A quantity is a unit of its own kind: `one_kg * 0.5` is half a kilogram.
    let gram = GRAM;
    let one_kg = kg.quantity(1.0);
    let masses = [one_kg, one_kg * 0.5, gram.quantity(250.0)];
    let total: SIQuantity<MASS, f64> = masses.iter().sum();
    println!("sum   = {} {gram}", gram.value(total));

    // --- 2. Maths is on quantities: km/h is km divided by h -----------------
    // Units never take part in arithmetic. Build quantities, compute with
    // them, and read the result in whichever unit you like: 90 km in 1 h is a
    // velocity, read in km/h or m/s.

    // A prefix scales a unit: `km` is the metre with the prefix kilo.
    let km = Prefixed {
        prefix: Some(Prefix::Kilo),
        unit: METRE,
    };
    let hour = HOUR;
    let (kmh, mps) = (KILOMETRE_PER_HOUR, METRE_PER_SECOND);
    let v: SIQuantity<VELOCITY, f64> = km.quantity(90.0) / hour.quantity(1.0);
    println!("v     = {} {kmh} = {} {mps}", kmh.value(v), mps.value(v));

    let trip: SIQuantity<LENGTH, f64> = v * hour.quantity(2.5);
    println!("2.5 h at {} {kmh} = {} {km}", kmh.value(v), km.value(trip));

    // --- 3. Mul / Div compose dimensions ------------------------------------
    // `mass * accel` is type-checked to be Force.

    let force: SIQuantity<FORCE, f64> = mass * accel;
    let newton = ForceUnit::Newton;
    println!("force = {:.2} {newton}", newton.value(force));

    let volt = ElectricPotentialUnit::Volt;
    let ma = Prefixed {
        prefix: Some(Prefix::Milli),
        unit: ElectricCurrentUnit::Ampere,
    };
    let voltage = volt.quantity(12.0);
    let current = ma.quantity(500.0);
    let power: SIQuantity<POWER, f64> = voltage * current;
    let resistance: SIQuantity<ELECTRICAL_RESISTANCE, f64> = voltage / current;
    let (watt, ohm) = (PowerUnit::Watt, ElectricalResistanceUnit::Ohm);
    println!("12 V x 500 mA -> {:.2} {watt:#}", watt.value(power));
    println!("12 V ÷ 500 mA -> {:.1} {ohm}", ohm.value(resistance));

    // --- 4. Same-dim division -> a plain number -----------------------------
    // A force over a force is `SIQuantity<DIMENSIONLESS, _>`, read like any quantity: in the unit
    // `Ratio` (one), or in percent.

    let ratio: SIQuantity<DIMENSIONLESS, f64> = newton.quantity(9.0) / newton.quantity(3.0);
    let (one, percent) = (DimensionlessUnit::Ratio, DimensionlessUnit::Percent);
    println!(
        "9 N / 3 N = {} = {} {percent}",
        one.value(ratio),
        percent.value(ratio),
    );

    // --- 5. Temperature: a unit with a zero ---------------------------------
    // °C counts steps of one kelvin from the ice point, 273.15 K - which is
    // the unit's business: `CELSIUS.quantity(37.0)` is 310.15 K. A unit
    // of `yadc-si` is a size and a zero, so `FAHRENHEIT` and `KELVIN_SCALE`
    // are the same, and `ThermodynamicTemperatureUnit` names them all, for a
    // menu and for printing. A temperature has the `Absolute` label: two of
    // them differ by a temperature interval (an ordinary amount, with its own
    // units - `KELVIN`, `DEGREE_CELSIUS`) and an interval moves one, but
    // `fever + body` does not compile (a sum keeps its left operand's type),
    // nor does a temperature unit read an interval. Absolute zero is a fact,
    // so a temperature is still an amount: it averages and scales, in kelvins.

    use ThermodynamicTemperatureUnit::{Fahrenheit, Kelvin as KelvinScale};
    let celsius = CELSIUS;
    let body = celsius.quantity(37.0_f64);
    let room = ThermodynamicTemperatureUnit::Celsius.quantity(22.0);
    let fever: SIQuantity<TEMPERATURE_INTERVAL, f64> = body - room;
    let degrees = TemperatureIntervalUnit::Celsius;
    println!("body - room = {} {degrees:#}", degrees.value(fever));
    let warmer: SIQuantity<THERMODYNAMIC_TEMPERATURE, f64> = body + fever;
    assert!((Fahrenheit.value(warmer) - 125.6).abs() < 1e-9);
    let between: SIQuantity<THERMODYNAMIC_TEMPERATURE, f64> = (body + room) / 2.0;
    println!("between them = {:.1} {celsius}", celsius.value(between));
    println!(
        "body + fever = {:.1} {celsius} = {:.1} {Fahrenheit} = {:.2} {KelvinScale}",
        celsius.value(warmer),
        Fahrenheit.value(warmer),
        KelvinScale.value(warmer),
    );
    // Kelvins above room temperature: the difference, read in kelvins.
    println!("body is {:.0} K above room", KELVIN.value(body - room));

    // --- 6. Format flags ----------------------------------------------------
    // The number takes the number's flags, the unit its own (`{:#}` is the long
    // name). A quantity also prints itself, as its number and base dimension.

    println!("{:>10.3} {newton:#}", newton.value(force));
    println!("{force:.2}");

    // --- 7. Levels: dBm is a unit on a logarithmic line ---------------------
    // dBm is ten times the base-10 logarithm of the power over 1 mW, so
    // 30 dBm is 1 W. On that line multiplying is adding: a `PowerLevel` is a
    // place on it, a `Gain` a step (`DECIBEL`), and `DECIBEL_MILLIWATT` a
    // unit with a zero like °C. It reads levels; the watts are one std
    // `From` away - the one place a logarithm is taken.

    let dbm = DECIBEL_MILLIWATT;
    let db = DECIBEL;
    let level = dbm.quantity(30.0_f64);
    let p = SIQuantity::<POWER, _>::from(level);
    let dbw = PowerLevelUnit::DecibelWatt;
    println!(
        "30 dBm = {:.0} {watt} = {:.0} {dbw}",
        watt.value(p),
        dbw.value(level)
    );
    let amplified = p * 100.0;
    println!(
        "× 100   = {:.0} {dbm}",
        dbm.value(SIQuantity::<POWER_LEVEL, _>::from(amplified))
    );
    // The same on the line: a level moves by gains, with `+` and `-`.
    let received = level - db.quantity(3.0) + db.quantity(23.0);
    println!(
        "30 dBm - 3 dB + 23 dB = {:.0} {dbm} = {:.0} {}",
        dbm.value(received),
        WATT.value(SIQuantity::<POWER, _>::from(received)),
        WATT,
    );
    // Two levels differ by a gain, and a gain is the logarithm of a ratio:
    // 20 dB is a hundred times.
    let gain: SIQuantity<GAIN, f64> = received - level;
    println!("gain    = {:.0} {db}", db.value(gain));
    let hundredfold = SIQuantity::<DIMENSIONLESS, _>::from(db.quantity(20.0));
    println!("20 dB   = × {:.0}", one.value(hundredfold));

    // --- 8. Instants: points on the line of time ----------------------------
    // An `Instant` is a point: measured from a zero somebody chose, so two of
    // them differ by a `Time` and a `Time` moves one, and nothing else -
    // `launch + landing` and `2.0 * launch` do not compile. Which zero is
    // not in the type: Unix seconds, GPS seconds and Julian dates are units
    // with their own zeros on the one line, so an instant built on one reads
    // on any other.

    use InstantUnit::{GpsSecond, JulianDate as Jd, UnixSecond};
    let launch = GpsSecond.quantity(1_400_000_000.0_f64);
    println!("launch  = {:.0} {UnixSecond}", UnixSecond.value(launch));
    let landing = launch + hour.quantity(2.5);
    let flight: SIQuantity<TIME, f64> = landing - launch;
    let minute = TimeUnit::Minute;
    println!("flight  = {:.0} {minute}", minute.value(flight));
    println!("landing = {:.4} {Jd} ({Jd:#})", Jd.value(landing));

    // --- 9. Dynamic dimensions with DynamicSIQuantity -----------------------
    // `DynamicSIQuantity` carries its dimension at run time. Any static
    // quantity converts in with `From`; its arithmetic composes dimensions as
    // it goes. Any step may have no result - a sum of a mass and a force, a
    // product of a power level - so every operator returns an `Option`, and a
    // calculation chains with `?`. The way back, `TryFrom`, checks the
    // dimension, and a mismatch hands the original back instead of losing it.

    let dyn_v = DynamicSIQuantity::from(volt.quantity(12.0));
    let dyn_i = DynamicSIQuantity::from(ElectricCurrentUnit::Ampere.quantity(2.0));
    let dyn_p = (dyn_v * dyn_i).expect("amounts have a product");
    let p = SIQuantity::<POWER, _>::try_from(dyn_p).expect("V x A is a power");
    println!("12 V x 2 A = {:.0} {watt:#}", watt.value(p));
    assert_eq!(SIQuantity::<MASS, _>::try_from(dyn_p), Err(dyn_p));

    let dyn_mass = DynamicSIQuantity::from(kg.quantity(2.5));
    let dyn_force = DynamicSIQuantity::from(newton.quantity(1.0));
    let half_kg = DynamicSIQuantity::from(kg.quantity(0.5));
    let nonsense = || -> Option<DynamicSIQuantity> { (dyn_mass + dyn_force)? * dyn_mass };
    let fine = || -> Option<f64> {
        let m = SIQuantity::<MASS, _>::try_from(((dyn_mass + half_kg)? * 2.0)?).ok()?;
        Some(kg.value(m))
    };
    println!(
        "(kg + N) * kg -> {:?}; (kg + kg) * 2 in kg -> {:?}",
        nonsense(),
        fine()
    );

    // --- 10. Runtime unit selection ----------------------------------------
    // A unit picked at run time is a `&dyn Unit<SIQuantity<MASS, f64>, f64>`: some unit
    // of mass counted in `f64`, still checked at compile time (a force unit
    // does not fit). A choice across *several* quantities is a unit set -
    // your own enum of unit enums (`yadc-si/examples/sets.rs`).
    let choices: [&dyn Unit<SIQuantity<MASS, f64>, f64>; 3] = [&kg, &MassUnit::Pound, &gram];
    let m = kg.quantity(2.5);
    let shown: Vec<String> = choices
        .iter()
        .map(|u| format!("{:.2}", u.value(m)))
        .collect();
    println!("2.5 kg in kg, lb, g: {}", shown.join(", "));

    // A unit enum is also std `Display` and `FromStr`: `{}` prints its
    // symbol, `{:#}` its long name, and `.parse()` reads the symbol back -
    // from a config file, a command line, a form field.
    let picked: MassUnit = "lb".parse().expect("a mass unit symbol");
    println!(
        "parsed \"lb\" -> {picked} / {picked:#}: 2.5 kg = {:.3} {picked}",
        picked.value(m)
    );
    let unknown = "furlong".parse::<MassUnit>().map(|u| u.to_string());
    println!("\"furlong\" as a mass unit: {unknown:?}");

    // --- 11. Constants, at compile time -------------------------------------
    // Every family has a `units` module of unit constants -
    // `mechanical::units::METRE`, `LengthUnit::Metre` names it - and a
    // prefix scales one (`Prefixed`, `km`). The physical constants are units
    // too (as in UCUM): `SPEED_OF_LIGHT`, `ELECTRON_MASS`, `STANDARD_GRAVITY`.
    // Their conversions are `const`, so the quantities they build compose in
    // `const` items (see the top of this file), checked and evaluated by the
    // compiler.
    println!("speed limit = {:.0} {kmh}", kmh.value(SPEED_LIMIT));
    let joule = EnergyUnit::Joule;
    let mev = Prefixed {
        prefix: Some(Prefix::Mega),
        unit: EnergyUnit::Electronvolt,
    };
    println!(
        "electron rest energy = {:.4e} {joule} = {:.4} {mev}",
        joule.value(ELECTRON_REST_ENERGY),
        mev.value(ELECTRON_REST_ENERGY),
    );
    // A weight is standard gravity times a mass.
    let weight: SIQuantity<FORCE, f64> = G_N * mass;
    println!("70 kg weighs {:.1} {newton}", newton.value(weight));
    // `PSI`, defined at the top from the pound, gravity and the inch, is the
    // generated psi unit.
    let psi = PressureUnit::PoundForcePerSquareInch;
    println!("lb · g_n / in² = {:.6} {psi}", psi.value(PSI));
    // A quantity is a unit, so speeds read in units of c.
    let probe = Prefixed {
        prefix: Some(Prefix::Kilo),
        unit: METRE_PER_SECOND,
    }
    .quantity(17.0);
    println!("17 km/s = {:.3e} c", C.value(probe));
    // Squares and other powers are typed: c² is a velocity squared.
    let c2 = C.powi::<2>();
    println!("E / c² = {:.4e} {kg}", kg.value(ELECTRON_REST_ENERGY / c2));

    // --- 12. Another number type --------------------------------------------
    // `SIQuantity` is generic over its number type (`SIQuantity<MASS, f64>`, `SIQuantity<MASS, f32>`).
    // `cast` changes it - for a compact record, say. A unit of `yadc-si`
    // counts in any number type its exact size converts to, so `KILOGRAM`
    // reads an `f32` mass as an `f32`. Widening `f32` to `f64` loses
    // nothing, so it is also a plain `From`.
    let m32 = mass.cast::<f32>();
    let widened: SIQuantity<MASS, f64> = m32.into();
    println!(
        "70 kg as f32: {} {kg} (an f32); widened back: {} {kg}",
        kg.value(m32),
        kg.value(widened),
    );
    // Integers too: a kilometre counted in metres as an `i32`, exactly, with
    // no floating point anywhere.
    let km_i32: SIQuantity<LENGTH, i32> = km.quantity(1);
    assert_eq!(METRE.value(km_i32), 1000);
    println!("1 km = {} m (an i32)", METRE.value(km_i32));
}
