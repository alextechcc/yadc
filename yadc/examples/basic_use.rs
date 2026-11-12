//! Tour of the `yadc-si` quantity system, one feature per section.
//!
//! Run with: cargo run -p yadc --example basic_use

// `generic_const_exprs` is all a crate using `yadc-si` needs; `const_trait_impl`,
// `const_ops` and `const_convert` only because this one does quantity
// arithmetic and rounding in `const` items (section 11).
#![feature(generic_const_exprs, const_trait_impl, const_ops, const_convert)]
#![allow(incomplete_features)]

use yadc::{CastAs, Unit};
use yadc_si::dimensionless::*;
use yadc_si::dynamic::dimensionless::*;
use yadc_si::dynamic::DynamicSIQuantity;
use yadc_si::electromagnetic::consts::AMPERE;
use yadc_si::electromagnetic::*;
use yadc_si::dynamic::electromagnetic::*;
use yadc_si::mechanical::consts::{
    DECIBEL, ELECTRON_MASS, HOUR, INCH, KILOGRAM, METRE, METRE_PER_SECOND, POUND,
    SPEED_OF_LIGHT, STANDARD_GRAVITY,
};
use yadc_si::mechanical::*;
use yadc_si::dynamic::mechanical::*;
use yadc_si::thermal::consts::KELVIN;
use yadc_si::thermal::*;
use yadc_si::dynamic::thermal::*;
use yadc_si::{Exact, KILO, MILLI, Prefix, Prefixed, Scale};

// Compile-time quantities. The constants are exact numbers, and so is their
// arithmetic: `KILO * METRE / HOUR` is 5/18 m/s, not a float near it. A
// constant is rounded to the number type a program stores only by std
// `From`/`Into`, once, here - and these are `const` items, so the compiler
// does it.
const SPEED_LIMIT: Velocity<f64> = (KILO * METRE / HOUR * Exact::int(50)).into();
const ELECTRON_REST_ENERGY: Energy<f64> = (ELECTRON_MASS * SPEED_OF_LIGHT * SPEED_OF_LIGHT).into(); // E = mc²
// A unit defined the way a standards document writes it: a pound-force per
// square inch is a pound under standard gravity, on a square inch.
const PSI: Pressure<f64> = (POUND * STANDARD_GRAVITY / (INCH * INCH)).into();

// The units this file counts `f64`s in, each rounded from its exact constant.
// A quantity is a unit of its own kind: `KG.quantity(70.0)` builds 70 kg,
// `KG.value(m)` reads a mass in kilograms.
const KG: Mass<f64> = KILOGRAM.into();
const KM: Length<f64> = (KILO * METRE).into();
const KM_PER_S: Velocity<f64> = (KILO * METRE_PER_SECOND).into();
const MA: ElectricCurrent<f64> = (MILLI * AMPERE).into();
const K: TemperatureInterval<f64> = KELVIN.into();
const DB: Gain<f64> = DECIBEL.into();
const G_N: Acceleration<f64> = STANDARD_GRAVITY.into();
const C: Velocity<f64> = SPEED_OF_LIGHT.into();
// A scale - a zero and a step - rounds the same way, both at once.
const CELSIUS: Scale<ThermodynamicTemperature<f64>, TemperatureInterval<f64>> =
    yadc_si::thermal::consts::CELSIUS.into();
const DBM: Scale<PowerLevel<f64>, Gain<f64>> = yadc_si::mechanical::consts::DBM.into();

fn main() {
    // --- 1. A unit turns a number into a quantity and back ------------------
    // A unit is the `Unit` trait: `quantity` builds, `value` reads.
    // `KG.quantity(70.0)` *is* 70 kg (stored in SI base units), as is
    // `KG * 70.0`, and `KG.value(m)` reads a mass back in kilograms. A unit
    // enum names units, for printing and for picking at run time, and is a
    // unit of any number type: `MassUnit::Kilogram.quantity(70.0)` builds the
    // same mass, `.value(m)` reads it. A unit prints itself: `{}` is its
    // symbol, `{:#}` its long name. So a quantity shown in a unit is the
    // number the unit reads, then the unit.

    use MassUnit::{Gram, Kilogram};
    let mass = KG.quantity(70.0);
    assert_eq!(Kilogram.quantity(70.0), mass);
    println!("mass  = {} {Kilogram:#}", Kilogram.value(mass));
    let g0 = AccelerationUnit::StandardGravity;
    let accel = g0.quantity(1.0);
    println!("accel = {} {g0:#}", g0.value(accel));

    let masses = [KG, Gram.quantity(500.0), Gram.quantity(250.0)];
    let total: Mass<f64> = masses.iter().sum();
    println!("sum   = {} {Gram}", Gram.value(total));

    // --- 2. Maths is on quantities: km/h is km divided by h -----------------
    // Units never take part in arithmetic. Build quantities, compute with
    // them, and read the result in whichever unit you like: 90 km in 1 h is a
    // `Velocity`, read in km/h or m/s.

    // A prefix scales a unit: `KM` is `KILO * METRE`, a kilometre.
    let hour = TimeUnit::Hour;
    let (kmh, mps) = (VelocityUnit::KilometrePerHour, VelocityUnit::MetrePerSecond);
    let v: Velocity<f64> = KM.quantity(90.0) / hour.quantity(1.0);
    println!("v     = {} {kmh} = {} {mps}", kmh.value(v), mps.value(v));

    let trip: Length<f64> = v * hour.quantity(2.5);
    println!("2.5 h at {} {kmh} = {} km", kmh.value(v), KM.value(trip));

    // --- 3. Mul / Div compose dimensions ------------------------------------
    // `mass * accel` is type-checked to be Force.

    let force: Force<f64> = mass * accel;
    let newton = ForceUnit::Newton;
    println!("force = {:.2} {newton}", newton.value(force));

    let volt = ElectricPotentialUnit::Volt;
    let voltage = volt.quantity(12.0);
    let current = MA.quantity(500.0);
    let power: Power<f64> = voltage * current;
    let resistance: ElectricalResistance<f64> = voltage / current;
    let (watt, ohm) = (PowerUnit::Watt, ElectricalResistanceUnit::Ohm);
    println!("12 V x 500 mA -> {:.2} {watt:#}", watt.value(power));
    println!("12 V ÷ 500 mA -> {:.1} {ohm}", ohm.value(resistance));

    // --- 4. Same-dim division -> a plain number -----------------------------
    // `Force / Force` is `Dimensionless`, read like any quantity: in the unit
    // `Ratio` (one), or in percent.

    let ratio: Dimensionless<f64> = newton.quantity(9.0) / newton.quantity(3.0);
    let (one, percent) = (DimensionlessUnit::Ratio, DimensionlessUnit::Percent);
    println!(
        "9 N / 3 N = {} = {} {percent}",
        one.value(ratio),
        percent.value(ratio),
    );

    // --- 5. Temperature: read on a scale ------------------------------------
    // °C counts steps of one kelvin from the ice point, 273.15 K - which is
    // the scale's business: `CELSIUS.quantity(37.0)` is 310.15 K. The exact
    // constant `thermal::consts::CELSIUS` is a zero and a step,
    // `Scale(ICE_POINT, KELVIN)`, rounded to `f64` at the top of this file;
    // so are `FAHRENHEIT` and `KELVIN_SCALE`, and
    // `ThermodynamicTemperatureUnit` names them all, for a menu and for
    // printing. A scale with a zero known only at run time is the same
    // expression, `Scale(room, K)`. A temperature has the `Absolute` label:
    // two of them differ by a temperature interval (an ordinary amount, with
    // its own units) and an interval moves one, but `fever + body` does not
    // compile (a sum keeps its left operand's type), nor does a scale read an
    // interval. Absolute zero is a fact, so a temperature is still an
    // amount: it averages and scales, in kelvins.

    use ThermodynamicTemperatureUnit::{Celsius, Fahrenheit, Kelvin};
    let body = CELSIUS.quantity(37.0);
    let room = Celsius.quantity(22.0);
    let fever: TemperatureInterval<f64> = body - room;
    let degrees = TemperatureIntervalUnit::Celsius;
    println!("body - room = {} {degrees:#}", degrees.value(fever));
    let warmer: ThermodynamicTemperature<f64> = body + fever;
    assert!((Fahrenheit.value(warmer) - 125.6).abs() < 1e-9);
    let between: ThermodynamicTemperature<f64> = (body + room) / 2.0;
    println!("between them = {:.1} {Celsius}", CELSIUS.value(between));
    println!(
        "body + fever = {:.1} {Celsius} = {:.1} {Fahrenheit} = {:.2} {Kelvin}",
        CELSIUS.value(warmer),
        Fahrenheit.value(warmer),
        Kelvin.value(warmer),
    );
    // Kelvins above room temperature: a scale made on the spot.
    let above_room = Scale(room, K);
    println!("body is {:.0} K above room", above_room.value(body));

    // --- 6. Format flags ----------------------------------------------------
    // The number takes the number's flags, the unit its own (`{:#}` is the long
    // name). A quantity also prints itself, as its number and base dimension.

    println!("{:>10.3} {newton:#}", newton.value(force));
    println!("{force:.2}");

    // --- 7. Levels: dBm is a scale on a logarithmic line --------------------
    // dBm is ten times the base-10 logarithm of the power over 1 mW, so
    // 30 dBm is 1 W. On that line multiplying is adding: a `PowerLevel` is a
    // place on it, a `Gain` a step (`DB`), and `DBM` a scale like °C - a zero
    // and a step, `Scale(MILLIWATT_LEVEL, DECIBEL)`. It reads levels; the
    // watts are one std `From` away - the one place a logarithm is taken. The
    // `PowerUnit` enum lists dBm among the units of power, converting for
    // you.

    use PowerUnit::DecibelWatt as Dbw;
    let level = DBM.quantity(30.0);
    let p = Power::from(level);
    println!("30 dBm = {:.0} {watt} = {:.0} {Dbw}", watt.value(p), Dbw.value(p));
    let amplified = p * 100.0;
    let dbm = PowerUnit::DecibelMilliwatt;
    println!("× 100   = {:.0} {dbm}", dbm.value(amplified));
    // The same on the line: a level moves by gains, with `+` and `-`.
    let received = level - DB * 3.0 + DB * 23.0;
    println!(
        "30 dBm - 3 dB + 23 dB = {:.0} {dbm} = {:.0} {watt}",
        DBM.value(received),
        watt.value(Power::from(received))
    );
    // Two levels differ by a gain, and a gain is the logarithm of a ratio:
    // 20 dB is a hundred times.
    let gain: Gain<f64> = received - level;
    println!("gain    = {:.0} dB", DB.value(gain));
    let hundredfold = Dimensionless::from(DB * 20.0);
    println!("20 dB   = × {:.0}", one.value(hundredfold));

    // --- 8. Instants: points on the line of time ----------------------------
    // An `Instant` is a point: measured from a zero somebody chose, so two of
    // them differ by a `Time` and a `Time` moves one, and nothing else -
    // `launch + landing` and `2.0 * launch` do not compile. Which zero is
    // not in the type: Unix seconds, GPS seconds and Julian dates are scales
    // on the one line, so an instant built on one reads on any other.

    use InstantUnit::{GpsSecond, JulianDate as Jd, UnixSecond};
    let launch = GpsSecond.quantity(1_400_000_000.0);
    println!("launch  = {:.0} {UnixSecond}", UnixSecond.value(launch));
    let landing = launch + hour.quantity(2.5);
    let flight: Time<f64> = landing - launch;
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
    let p = Power::try_from(dyn_p).expect("V x A is a power");
    println!("12 V x 2 A = {:.0} {watt:#}", watt.value(p));
    assert_eq!(Mass::try_from(dyn_p), Err(dyn_p));

    let dyn_mass = DynamicSIQuantity::from(KG.quantity(2.5));
    let dyn_force = DynamicSIQuantity::from(newton.quantity(1.0));
    let half_kg = DynamicSIQuantity::from(KG.quantity(0.5));
    let nonsense = || -> Option<DynamicSIQuantity> { (dyn_mass + dyn_force)? * dyn_mass };
    let fine = || -> Option<f64> {
        let m = Mass::try_from(((dyn_mass + half_kg)? * 2.0)?).ok()?;
        Some(KG.value(m))
    };
    println!("(kg + N) * kg -> {:?}; (kg + kg) * 2 in kg -> {:?}", nonsense(), fine());

    // --- 10. Runtime unit selection ----------------------------------------
    // A unit picked at run time is a `&dyn Unit<Mass>`: some unit of mass,
    // still checked at compile time (a force unit does not fit). A choice
    // across *several* quantities is a unit set - your own enum of unit enums
    // (`yadc-si/examples/sets.rs`).
    let choices: [&dyn Unit<Mass<f64>>; 3] = [&KG, &MassUnit::Pound, &Gram];
    let m = KG.quantity(2.5);
    let shown: Vec<String> = choices.iter().map(|u| format!("{:.2}", u.value(m))).collect();
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
    // Units are named constants: every family has a `consts` module with the
    // exact constants its units count in - `mechanical::consts::METRE` is one
    // metre, as a `Length<Exact>`, and `LengthUnit::Metre` names it; a
    // prefix scales one (`KILO * METRE`), and `Prefixed` names that (`km`).
    // The physical constants are units too (as in UCUM), so they are there
    // as well: `SPEED_OF_LIGHT`, `ELECTRON_MASS`, `STANDARD_GRAVITY`. Their
    // arithmetic is exact and `const`, so they compose in `const` items (see
    // the top of this file), checked and evaluated by the compiler, and
    // rounded once, by the `.into()` that picks the number type.
    println!("speed limit = {:.0} {kmh}", kmh.value(SPEED_LIMIT));
    let joule = EnergyUnit::Joule;
    let mev = Prefixed { prefix: Some(Prefix::Mega), unit: EnergyUnit::Electronvolt };
    println!(
        "electron rest energy = {:.4e} {joule} = {:.4} {mev}",
        joule.value(ELECTRON_REST_ENERGY),
        mev.value(ELECTRON_REST_ENERGY),
    );
    // A weight is standard gravity times a mass.
    let weight: Force<f64> = G_N * mass;
    println!("70 kg weighs {:.1} {newton}", newton.value(weight));
    // `PSI`, defined at the top from the pound, gravity and the inch, is the
    // generated psi unit.
    let psi = PressureUnit::PoundForcePerSquareInch;
    println!("POUND * STANDARD_GRAVITY / INCH² = {:.6} {psi}", psi.value(PSI));
    // A constant is a unit, so speeds read in units of c.
    let probe = KM_PER_S.quantity(17.0);
    println!("17 km/s = {:.3e} c", C.value(probe));

    // --- 12. Another number type --------------------------------------------
    // `SIQuantity` is generic over its number type (`Mass<f64>`, `Mass<f32>`).
    // `cast` changes it - for a compact record, say. An exact constant
    // rounds to whichever type its `From` names, so `KILOGRAM` as a
    // `Mass<f32>` is a unit counting in `f32`; a unit enum reads any number
    // type. Widening `f32` to `f64` loses nothing, so it is also a plain
    // `From`.
    const KG_F32: Mass<f32> = KILOGRAM.into();
    let m32 = mass.cast::<f32>();
    let widened: Mass<f64> = m32.into();
    println!(
        "70 kg as f32: {} kg = {} {Kilogram} (an f32); widened back: {} kg",
        KG_F32.value(m32),
        Kilogram.value(m32),
        KG.value(widened),
    );
    // Integers too: the compiler counts a kilometre in metres as an `i32`,
    // exactly, with no floating point anywhere.
    const KM_I32: Length<i32> = (KILO * METRE).into();
    const M_I32: Length<i32> = METRE.into();
    assert_eq!(M_I32.value(KM_I32), 1000);
    println!("1 km = {} m (an i32)", M_I32.value(KM_I32));
}
