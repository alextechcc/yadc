//! Tour of the `yadc-si` quantity system, one feature per section.
//!
//! Run with: cargo run -p yadc --example basic_use

// `generic_const_exprs` is all a crate using `yadc-si` needs; `const_trait_impl`
// and `const_ops` only because this one does quantity arithmetic in `const`
// items (section 11).
#![feature(generic_const_exprs, const_trait_impl, const_ops)]
#![allow(incomplete_features)]

use yadc::{CastAs, Unit};
use yadc_si::dimensionless::*;
use yadc_si::electromagnetic::*;
use yadc_si::mechanical::consts::*;
use yadc_si::mechanical::*;
use yadc_si::thermal::consts::{CELSIUS, FAHRENHEIT, KELVIN, KELVIN_SCALE};
use yadc_si::thermal::*;
use yadc_si::{DynamicSIQuantity, Scale};

// Compile-time quantities: unit and physical constants compose in `const`
// items, because quantity arithmetic is `const`.
const SPEED_LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;
const ELECTRON_REST_ENERGY: Energy = ELECTRON_MASS::<f64> * SPEED_OF_LIGHT * SPEED_OF_LIGHT; // E = mc²
// A unit defined the way a standards document writes it: a pound-force per
// square inch is a pound under standard gravity, on a square inch.
const PSI: Pressure = POUND::<f64> * STANDARD_GRAVITY / (INCH::<f64> * INCH);

fn main() {
    // --- 1. A unit turns a number into a quantity and back ------------------
    // A quantity constant or a unit enum's value is called like a function:
    // on a number it builds, on a quantity it reads. `Kilogram(70.0)` *is* 70 kg (stored in SI base
    // units), and `Kilogram(m)` reads a mass back in kilograms. A unit is a
    // value: every constant is one (`KILOGRAM(m)` is the same number), and
    // `MassUnit::Kilogram` is an enum variant that names it, for printing and
    // for picking at run time.
    // A unit prints itself: `{}` is its symbol, `{:#}` its long name. So a
    // quantity shown in a unit is the number the unit reads, then the unit.

    use MassUnit::{Gram, Kilogram};
    let mass = Kilogram(70.0);
    let accel = AccelerationUnit::StandardGravity(1.0);
    println!("mass  = {} {:#}", Kilogram(mass), Kilogram);
    assert_eq!(Kilogram(mass), KILOGRAM(mass));
    let g0 = AccelerationUnit::StandardGravity;
    println!("accel = {} {:#}", g0(accel), g0);

    let masses = [
        Kilogram(1.0),
        Gram(500.0),
        Gram(250.0),
    ];
    let total: Mass = masses.iter().sum();
    println!("sum   = {} {}", Gram(total), Gram);

    // --- 2. Maths is on quantities: km/h is km divided by h -----------------
    // Units never take part in arithmetic. Build quantities, compute with
    // them, and read the result in whichever unit you like: 90 km in 1 h is a
    // `Velocity`, read in km/h or m/s.

    use LengthUnit::Kilometre;
    use TimeUnit::Hour;
    use VelocityUnit::{KilometrePerHour, MetrePerSecond};
    let v: Velocity = Kilometre(90.0) / Hour(1.0);
    let kmh = KilometrePerHour(v);
    let mps = MetrePerSecond(v);
    println!("v     = {kmh} km/h = {mps} m/s");

    let t = Hour(2.5);
    let trip: Length = v * t;
    println!("2.5 h at {kmh} km/h = {} km", KILOMETRE(trip));

    // --- 3. Mul / Div compose dimensions ------------------------------------
    // `mass * accel` is type-checked to be Force.

    let force: Force = mass * accel;
    println!(
        "force = {:.2} {}",
        ForceUnit::Newton(force),
        ForceUnit::Newton
    );

    use ElectricCurrentUnit::Milliampere;
    use ElectricPotentialUnit::Volt;
    let voltage = Volt(12.0);
    let current = Milliampere(500.0);
    let power: Power = voltage * current;
    let resistance: ElectricalResistance = voltage / current;
    let (watt, ohm) = (PowerUnit::Watt, ElectricalResistanceUnit::Ohm);
    println!("12 V x 500 mA -> {:.2} {:#}", watt(power), watt);
    println!("12 V ÷ 500 mA -> {:.1} {}", ohm(resistance), ohm);

    // --- 4. Same-dim division -> a plain number -----------------------------
    // `Force / Force` is `Dimensionless`, read like any quantity: in the unit
    // `Ratio` (one), or in percent.

    let ratio: Dimensionless = ForceUnit::Newton(9.0) / ForceUnit::Newton(3.0);
    let (one, percent) = (DimensionlessUnit::Ratio, DimensionlessUnit::Percent);
    println!(
        "9 N / 3 N = {} = {} {}",
        one(ratio),
        percent(ratio),
        percent
    );

    // --- 5. Temperature: read on a scale ------------------------------------
    // °C counts steps of one kelvin from the ice point, 273.15 K - which is
    // the scale's business: `CELSIUS.quantity(37.0)` is 310.15 K. `CELSIUS` is a
    // constant - a zero and a step, `Scale(ICE_POINT, KELVIN)` - and so are
    // `FAHRENHEIT` and `KELVIN_SCALE`; a scale with a zero known only at run
    // time is the same expression, `Scale(room, KELVIN)`.
    // `ThermodynamicTemperatureUnit` is the enum that names them all, for a
    // menu and for printing. A temperature has the `Absolute` label: two of
    // them differ by a temperature interval (an ordinary amount, with its own
    // units) and an interval moves one, but `fever + body` does not compile
    // (a sum keeps its left operand's type), nor does a scale read an
    // interval. Absolute zero is a fact, so a temperature is still an
    // amount: it averages and scales, in kelvins.

    use ThermodynamicTemperatureUnit::{Celsius, Fahrenheit, Kelvin};
    let body = CELSIUS.quantity(37.0);
    let room = Celsius(22.0);
    let fever: TemperatureInterval = body - room;
    let degrees = TemperatureIntervalUnit::Celsius;
    println!("body - room = {} {:#}", degrees(fever), degrees);
    let warmer: ThermodynamicTemperature = body + fever;
    assert!((FAHRENHEIT.value(warmer) - 125.6).abs() < 1e-9);
    let between: ThermodynamicTemperature = (body + room) / 2.0;
    println!("between them = {:.1} {}", CELSIUS.value(between), Celsius);
    println!(
        "body + fever = {:.1} {} = {:.1} {} = {:.2} {}",
        CELSIUS.value(warmer),
        Celsius,
        FAHRENHEIT.value(warmer),
        Fahrenheit,
        KELVIN_SCALE.value(warmer),
        Kelvin,
    );
    // Kelvins above room temperature: a scale made on the spot.
    let above_room = Scale(room, KELVIN);
    println!("body is {:.0} K above room", above_room.value(body));

    // --- 6. Format flags ----------------------------------------------------
    // The number takes the number's flags, the unit its own (`{:#}` is the long
    // name). A quantity also prints itself, as its number and base dimension.

    let newton = ForceUnit::Newton;
    println!("{:>10.3} {newton:#}", newton(force));
    println!("{force:.2}");

    // --- 7. Levels: dBm is a scale on a logarithmic line --------------------
    // dBm is ten times the base-10 logarithm of the power over 1 mW, so
    // 30 dBm is 1 W. On that line multiplying is adding: a `PowerLevel` is a
    // place on it, a `Gain` a step, and `DBM` a scale like °C - a zero and a
    // step, `Scale(MILLIWATT_LEVEL, DECIBEL)`. It reads levels; the watts are
    // one std `From` away - the one place a logarithm is taken. The
    // `PowerUnit` enum lists dBm among the units of power, converting for
    // you.

    use PowerUnit::{DecibelWatt as Dbw, Watt};
    let level = DBM.quantity(30.0);
    let p = Power::from(level);
    println!("30 dBm = {:.0} {} = {:.0} {}", Watt(p), Watt, Dbw(p), Dbw);
    let amplified = p * 100.0;
    let dbm = PowerUnit::DecibelMilliwatt;
    println!("× 100   = {:.0} {}", dbm(amplified), dbm);
    // The same on the line: a level moves by gains, with `+` and `-`.
    let received = level - DECIBEL(3.0) + DECIBEL(23.0);
    println!(
        "30 dBm - 3 dB + 23 dB = {:.0} {dbm} = {:.0} {Watt}",
        DBM.value(received),
        Watt(Power::from(received))
    );
    // Two levels differ by a gain, and a gain is the logarithm of a ratio:
    // 20 dB is a hundred times.
    let gain: Gain = received - level;
    println!("gain    = {:.0} dB", DECIBEL(gain));
    let hundredfold = Dimensionless::from(DECIBEL(20.0));
    println!("20 dB   = × {:.0}", one(hundredfold));

    // --- 8. Instants: points on the line of time ----------------------------
    // An `Instant` is a point: measured from a zero somebody chose, so two of
    // them differ by a `Time` and a `Time` moves one, and nothing else -
    // `launch + landing` and `2.0 * launch` do not compile. Which zero is
    // not in the type: Unix seconds, GPS seconds and Julian dates are scales
    // on the one line, so an instant built on one reads on any other.

    let launch = GPS_SECONDS.quantity(1_400_000_000.0);
    println!("launch  = {:.0} s Unix", UNIX_SECONDS.value(launch));
    let landing = launch + Hour(2.5);
    let flight: Time = landing - launch;
    println!("flight  = {:.0} {}", TimeUnit::Minute(flight), TimeUnit::Minute);
    let jd = InstantUnit::JulianDate;
    println!("landing = {:.4} {jd} ({jd:#})", jd(landing));

    // --- 9. Dynamic dimensions with DynamicSIQuantity -----------------------
    // `DynamicSIQuantity` carries its dimension at run time. Any static
    // quantity converts in with `From`; its arithmetic composes dimensions as
    // it goes. Any step may have no result - a sum of a mass and a force, a
    // product of a power level - so every operator returns an `Option`, and a
    // calculation chains with `?`. The way back, `TryFrom`, checks the
    // dimension, and a mismatch hands the original back instead of losing it.

    let dyn_v = DynamicSIQuantity::from(Volt(12.0));
    let dyn_i = DynamicSIQuantity::from(ElectricCurrentUnit::Ampere(2.0));
    let dyn_p = (dyn_v * dyn_i).expect("amounts have a product");
    let p = Power::try_from(dyn_p).expect("V x A is a power");
    println!("12 V x 2 A = {:.0} {:#}", Watt(p), Watt);
    assert_eq!(Mass::try_from(dyn_p), Err(dyn_p));

    let dyn_mass = DynamicSIQuantity::from(Kilogram(2.5));
    let dyn_force = DynamicSIQuantity::from(ForceUnit::Newton(1.0));
    let half_kg = DynamicSIQuantity::from(Kilogram(0.5));
    let nonsense = || -> Option<DynamicSIQuantity> { (dyn_mass + dyn_force)? * dyn_mass };
    let fine = || -> Option<f64> {
        let m = Mass::try_from(((dyn_mass + half_kg)? * 2.0)?).ok()?;
        Some(Kilogram(m))
    };
    println!("(kg + N) * kg -> {:?}; (kg + kg) * 2 in kg -> {:?}", nonsense(), fine());

    // --- 10. Runtime unit selection ----------------------------------------
    // A unit picked at run time is a `&dyn Unit<Mass>`: some unit of mass,
    // still checked at compile time (a force unit does not fit). A choice
    // across *several* quantities is a unit set - your own enum of unit enums
    // (`yadc-si/examples/sets.rs`).
    let choices: [&dyn Unit<Mass>; 3] = [&Kilogram, &MassUnit::Pound, &Gram];
    let m = Kilogram(2.5);
    let shown: Vec<String> = choices.iter().map(|u| format!("{:.2}", u.value(m))).collect();
    println!("2.5 kg in kg, lb, g: {}", shown.join(", "));

    // A unit is also std `Display` and `FromStr`: `{}` prints its symbol,
    // `{:#}` its long name, and `.parse()` reads the symbol back - from a
    // config file, a command line, a form field.
    let picked: MassUnit = "lb".parse().expect("a mass unit symbol");
    println!(
        "parsed \"lb\" -> {picked} / {picked:#}: 2.5 kg = {:.3} {picked}",
        picked(Kilogram(2.5))
    );
    let unknown = "furlong".parse::<MassUnit>().map(|u| u.to_string());
    println!("\"furlong\" as a mass unit: {unknown:?}");

    // --- 11. Constants, at compile time -------------------------------------
    // Units are named constants: every family has a `consts` module with the
    // constants its units count in - `mechanical::consts::KILOMETRE` is one
    // kilometre, as a `Length`, and `LengthUnit::Kilometre` names it. The
    // physical constants are units too (as in UCUM), so they are there as
    // well: `SPEED_OF_LIGHT`, `ELECTRON_MASS`, `STANDARD_GRAVITY`. Quantity
    // arithmetic is `const`, so they compose in `const` items (see the top of
    // this file), checked and evaluated by the compiler.
    println!(
        "speed limit = {:.0} {}",
        KilometrePerHour(SPEED_LIMIT),
        KilometrePerHour
    );
    let (joule, mev) = (EnergyUnit::Joule, EnergyUnit::Megaelectronvolt);
    println!(
        "electron rest energy = {:.4e} {joule} = {:.4} {mev}",
        joule(ELECTRON_REST_ENERGY),
        mev(ELECTRON_REST_ENERGY),
    );
    // A weight is standard gravity times a mass.
    let weight: Force = STANDARD_GRAVITY * mass;
    println!("70 kg weighs {:.1} {newton}", newton(weight));
    // `PSI`, defined at the top from the pound, gravity and the inch, is the
    // generated psi unit's constant to the last bit or so.
    let psi = PressureUnit::PoundForcePerSquareInch;
    println!(
        "POUND * STANDARD_GRAVITY / INCH² = {:.6} {psi}",
        psi(PSI)
    );
    // A constant is a unit, so speeds read in units of c.
    let probe = VelocityUnit::KilometrePerSecond(17.0);
    println!("17 km/s = {:.3e} c", SPEED_OF_LIGHT(probe));

    // --- 12. Another number type --------------------------------------------
    // `SIQuantity` is generic over its number type (`f64` unless you say
    // otherwise). `cast` changes it - for a compact record, say. A constant
    // is exact and takes the number type it is used at, so `KILOGRAM` bound
    // as a `Mass<f32>` is a unit counting in `f32`. Widening `f32` to `f64`
    // loses nothing, so it is also a plain `From`.
    let m32 = mass.cast::<f32>();
    let kilogram32: Mass<f32> = KILOGRAM;
    let widened: Mass = m32.into();
    println!(
        "70 kg as f32: {} kg (an f32); widened back: {} kg",
        kilogram32(m32),
        KILOGRAM(widened),
    );
}
