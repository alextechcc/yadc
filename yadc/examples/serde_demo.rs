//! serde round-trip with the yadc-si quantities.
//!
//! `SIQuantity<const D: ISQ>` serializes transparently as just its inner
//! `f64` - the dimension is determined by the target type at deserialize time.
//! `DynamicSIQuantity` carries `dim: ISQ` at runtime, and that field is
//! serialized as a UCUM unit string (e.g. `"m.kg.s-2"` for force, `"K"` for a
//! temperature interval, `"rad"` for angle, `"1"` for dimensionless). A
//! rejected combination is `None` from the operator, so it never reaches the
//! wire as a `DynamicSIQuantity`.
//!
//! Run with: cargo run -p yadc --example serde_demo
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc_si::{DynamicSIQuantity, Unit};
use yadc_si::electromagnetic::consts::VOLT;
use yadc_si::electromagnetic::*;
use yadc_si::mechanical::consts::{KILOGRAM, NEWTON};
use yadc_si::mechanical::*;
use yadc_si::thermal::consts::{CELSIUS, FAHRENHEIT, KELVIN};
use yadc_si::thermal::ThermodynamicTemperature;

fn main() {
    // 1. Typed Qty<D> -> JSON -> typed Qty<D>. Wire is just the number.
    // Every constant is a unit, called like a function: `KILOGRAM(1.5)` is
    // 1.5 kg, and `KILOGRAM(m)` reads a mass back.
    use MassUnit::Kilogram;
    let m = KILOGRAM(1.5);
    let s = serde_json::to_string(&m).expect("serialize Mass");
    println!("Mass(1.5 kg) -> {s}");

    let parsed: Mass = serde_json::from_str(&s).expect("round-trip Mass");
    println!("round-trip:    {} {Kilogram:#}\n", Kilogram(parsed));

    // 2. DynamicSIQuantity carries its dims at runtime - they're serialized as a UCUM
    //    string via `UcumSerialize`, which extends the string with a `{Name}`
    //    annotation for any kind but the ordinary one (`K{Absolute}`). This lets a
    //    temperature survive a DynamicSIQuantity round-trip. Angles need
    //    no annotation: radians are an axis (`rad`, and `rad2` for steradians).
    let body = FAHRENHEIT.quantity(98.6);
    let body = DynamicSIQuantity::from(body);
    let s = serde_json::to_string_pretty(&body).unwrap();
    println!("DynamicSIQuantity(98.6 °F) ->\n{s}\n");

    let parsed: DynamicSIQuantity = serde_json::from_str(&s).unwrap();
    let t = ThermodynamicTemperature::try_from(parsed).expect("a temperature");
    println!(
        "round-trip: {:.1} °F ≡ {:.2} °C\n",
        FAHRENHEIT.value(t),
        CELSIUS.value(t),
    );

    // A temperature step (plain K) serializes without annotation; a
    // temperature gets {Absolute}.
    let interval = DynamicSIQuantity::from(KELVIN(15.0));
    println!(
        "DynamicSIQuantity(a step of 15 K) -> {}\n",
        serde_json::to_string(&interval).unwrap(),
    );

    // 3. A more interesting Dim: Force has m*kg*s⁻².
    let force = DynamicSIQuantity::from(NEWTON(9.81));
    println!(
        "DynamicSIQuantity(9.81 N) -> {}\n",
        serde_json::to_string(&force).unwrap()
    );

    // A rejected combination (kg + N) is `None`: there is no quantity to
    // serialize, and an `Option<DynamicSIQuantity>` goes out as `null`, which
    // reads back as `None`.
    let nonsense = DynamicSIQuantity::from(KILOGRAM(1.0)) + force;
    let s = serde_json::to_string(&nonsense).unwrap();
    println!("DynamicSIQuantity(1 kg + 9.81 N) -> {s}");
    let parsed: Option<DynamicSIQuantity> = serde_json::from_str(&s).unwrap();
    assert_eq!(parsed, None);
    // A well-formed one round-trips with its dimension: it reads back as a
    // `Force`.
    let parsed: DynamicSIQuantity =
        serde_json::from_str(&serde_json::to_string(&force).unwrap()).unwrap();
    let f = Force::try_from(parsed).expect("still a force");
    let newton = ForceUnit::Newton;
    println!("round-trip: {} {newton}\n", newton(f));

    // 4. Holding typed quantities in a serde-derived struct.
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Reading {
        sensor: String,
        temperature: ThermodynamicTemperature,
        voltage: ElectricPotential,
    }

    let r = Reading {
        sensor: "thermistor-7".into(),
        temperature: CELSIUS.quantity(25.0),
        voltage: VOLT(3.30),
    };
    let s = serde_json::to_string_pretty(&r).unwrap();
    println!("Reading ->\n{s}");

    let back: Reading = serde_json::from_str(&s).unwrap();
    println!(
        "read back: {} at {:.1} °C and {:.2} V",
        back.sensor,
        CELSIUS.value(back.temperature),
        VOLT(back.voltage),
    );
}
