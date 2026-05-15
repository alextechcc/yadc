//! serde round-trip with the yadc-si quantities.
//!
//! `Qty<const D: Dim>` serializes transparently as just its inner `f64` -
//! the dimension is determined by the target type at deserialize time.
//! `DynamicSIQuantity` carries `dim: Dim` at runtime, and that field is serialized
//! as a UCUM unit string (e.g. `"m.kg.s-2"` for force, `"K"` for temperature,
//! `"rad"` for angle, `"1"` for dimensionless).

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::{Format, Unit};
use yadc_si::DynamicSIQuantity;
use yadc_si::electromagnetic::*;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;

fn main() {
    // 1. Typed Qty<D> -> JSON -> typed Qty<D>. Wire is just the number.
    let m = MassUnit::Kilogram.quantity(1.5);
    let s = serde_json::to_string(&m).expect("serialize Mass");
    println!("Mass(1.5 kg) -> {s}");

    let parsed: Mass = serde_json::from_str(&s).expect("round-trip Mass");
    println!(
        "round-trip:    {}\n",
        MassUnit::Kilogram.format_quantity(parsed, Format::Long)
    );

    // 2. DynamicSIQuantity carries its dims at runtime - they're serialized as a UCUM
    //    string via `UcumSerialize`, which extends the string with a `{KindName}`
    //    annotation for kinds that have no UCUM prefix (i.e. everything except Angle
    //    and SolidAngle). This lets Affine
    //    temperature survive a DynamicSIQuantity round-trip.
    let body = DynamicSIQuantity::from(ThermodynamicTemperatureUnit::Fahrenheit.quantity(98.6));
    let s = serde_json::to_string_pretty(&body).unwrap();
    println!("DynamicSIQuantity(98.6 °F) ->\n{s}\n");

    let parsed: DynamicSIQuantity = serde_json::from_str(&s).unwrap();
    println!(
        "round-trip: {} ≡ {:.2}\n",
        ThermodynamicTemperatureUnit::Fahrenheit
            .try_format_quantity(parsed, Format::Short)
            .unwrap(),
        ThermodynamicTemperatureUnit::Celsius
            .try_format_quantity(parsed, Format::Short)
            .unwrap(),
    );

    // DeltaT (Linear K) serializes without annotation; absolute T gets {Affine}.
    let interval = DynamicSIQuantity::from(TemperatureIntervalUnit::Kelvin.quantity(15.0));
    println!(
        "DynamicSIQuantity(DeltaT 15 K) -> {}\n",
        serde_json::to_string(&interval).unwrap(),
    );

    // 3. A more interesting Dim: Force has m*kg*s⁻².
    let force = DynamicSIQuantity::from(ForceUnit::Newton.quantity(9.81));
    println!(
        "DynamicSIQuantity(9.81 N) -> {}\n",
        serde_json::to_string(&force).unwrap()
    );

    // 4. Holding typed quantities in a serde-derived struct.
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Reading {
        sensor: String,
        temperature: ThermodynamicTemperature,
        voltage: ElectricPotential,
    }

    let r = Reading {
        sensor: "thermistor-7".into(),
        temperature: ThermodynamicTemperatureUnit::Celsius.quantity(25.0),
        voltage: ElectricPotentialUnit::Volt.quantity(3.30),
    };
    let s = serde_json::to_string_pretty(&r).unwrap();
    println!("Reading ->\n{s}");

    let _back: Reading = serde_json::from_str(&s).unwrap();
}
