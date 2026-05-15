//! Tour of the `yadc-si` quantity system, one feature per section.
//!
//! Run with: cargo run --example basic_use --manifest-path yadc/Cargo.toml

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::{DynamicQuantity, Format, StaticQuantity};
use yadc_si::DynamicSIQuantity;
use yadc_si::dimensionless::*;
use yadc_si::electromagnetic::*;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;

fn main() {
    // ─── 1. Construction from a unit ────────────────────────────────────────
    // `Qty::from_unit` takes a numeric value plus a unit enum variant.

    let mass = Mass::from_unit(70.0, MassUnit::Kilogram);
    let accel = Acceleration::from_unit(1.0, AccelerationUnit::StandardGravity);
    println!(
        "mass  = {}",
        mass.fmt_as_unit(MassUnit::Kilogram, Format::Long)
    );
    println!(
        "accel = {}",
        accel.fmt_as_unit(AccelerationUnit::StandardGravity, Format::Long)
    );

    let mass_array = [
        Mass::from_unit(1.0, MassUnit::Kilogram),
        Mass::from_unit(500.0, MassUnit::Gram),
        Mass::from_unit(250.0, MassUnit::Gram),
    ];
    let total = mass_array.iter().sum::<Mass>();
    println!(
        "sum   = {}",
        total.fmt_as_unit(MassUnit::Carat, Format::Short)
    );

    // ─── 2. Mul / Div compose dimensions ────────────────────────────────────
    // `mass * accel` is type-checked to be Force.

    let force: Force = mass * accel;
    println!(
        "force = {:.2}",
        force.fmt_as_unit(ForceUnit::Newton, Format::Short)
    );

    let voltage = ElectricPotential::from_unit(12.0, ElectricPotentialUnit::Volt);
    let current = ElectricCurrent::from_unit(500.0, ElectricCurrentUnit::Milliampere);
    let power: Power = voltage * current;
    let resistance: ElectricalResistance = voltage / current;
    println!(
        "{} × {} → {:.2}",
        voltage.fmt_as_unit(ElectricPotentialUnit::Volt, Format::Short),
        current.fmt_as_unit(ElectricCurrentUnit::Milliampere, Format::Short),
        power.fmt_as_unit(PowerUnit::Watt, Format::Long),
    );
    println!(
        "{} ÷ {} → {:.1}",
        voltage.fmt_as_unit(ElectricPotentialUnit::Volt, Format::Short),
        current.fmt_as_unit(ElectricCurrentUnit::Milliampere, Format::Short),
        resistance.fmt_as_unit(ElectricalResistanceUnit::Ohm, Format::Short),
    );

    // ─── 3. Same-dim division → Dimensionless ───────────────────────────────
    // `Force / Force` is type-checked to be `Dimensionless`. The result also
    // implements `Deref<Target = f64>` so `*ratio` gives the raw scalar.

    let a = Force::from_unit(9.0, ForceUnit::Newton);
    let b = Force::from_unit(3.0, ForceUnit::Newton);
    let ratio: Dimensionless = a / b;
    println!(
        "9 N / 3 N = {} (or {})",
        ratio.fmt_as_unit(DimensionlessUnit::Ratio, Format::Short),
        ratio.fmt_as_unit(DimensionlessUnit::Percent, Format::Short),
    );

    // ─── 4. Affine temperature: T - T = ΔT ──────────────────────────────────
    // `ThermodynamicTemperature` carries `Kind::Affine` so the type system
    // distinguishes it from a plain temperature interval. Subtracting two
    // absolute temperatures produces an *interval*, not another absolute T.

    let body = ThermodynamicTemperature::from_unit(37.0, ThermodynamicTemperatureUnit::Celsius);
    let room = ThermodynamicTemperature::from_unit(22.0, ThermodynamicTemperatureUnit::Celsius);
    let fever: TemperatureInterval = body - room;
    println!(
        "body − room = {}",
        fever.fmt_as_unit(TemperatureIntervalUnit::Celsius, Format::Long),
    );

    // Affine + Linear stays absolute: `T + ΔT = T`.
    let warmer: ThermodynamicTemperature = body + fever;
    println!(
        "body + fever = {:.1}",
        warmer.fmt_as_unit(ThermodynamicTemperatureUnit::Celsius, Format::Short),
    );

    // ─── 5. Display passthrough via format flags ────────────────────────────
    // `fmt_as_unit` returns a `UnitDisplay` that respects `{:.2}`, `{:>10.3}`,
    // `{:+}`, etc.

    let g = force.fmt_as_unit(ForceUnit::Newton, Format::Long);
    println!("plain   : {g}");
    println!("2 dp    : {g:.2}");
    println!("width 12: {g:>12.3}");

    // ─── 6. Raw scalar via .as_unit() ───────────────────────────────────────
    // No `Display` wrapper — just an `f64` you can do math with.

    let kg = Mass::from_unit(1500.0, MassUnit::Gram).as_unit(MassUnit::Kilogram);
    println!("1500 g as kg (raw): {kg}");

    // ─── 7. Scalar arithmetic (Qty * f64, etc.) ─────────────────────────────
    // A quantity can be multiplied or divided by a plain scalar. The result
    // keeps the same dimension. `*=` / `/=` work in place. The scalar can
    // appear on either side of `*`.

    let m1 = Mass::from_unit(10.0, MassUnit::Kilogram);
    let doubled = m1 * 2.0;
    let halved = m1 / 2.0;
    let from_left: Mass = 3.0 * m1;
    println!(
        "10 kg × 2.0 = {}, 10 kg / 2.0 = {}, 3.0 × 10 kg = {}",
        doubled.fmt_as_unit(MassUnit::Kilogram, Format::Short),
        halved.fmt_as_unit(MassUnit::Kilogram, Format::Short),
        from_left.fmt_as_unit(MassUnit::Kilogram, Format::Short),
    );

    let mut tank = Mass::from_unit(1.0, MassUnit::Kilogram);
    tank *= 4.0;
    tank /= 2.0;
    println!(
        "1 kg *= 4.0 /= 2.0 → {}",
        tank.fmt_as_unit(MassUnit::Kilogram, Format::Short),
    );

    // ─── 8. Dynamic dimensions with DynamicSIQuantity ──────────────────────────────────
    // `DynamicSIQuantity` carries its dimension at runtime. Arithmetic that combines
    // dims (`*`, `/`) always succeeds; arithmetic that requires matching dims
    // (`+`, `-`) returns `Option`.

    let dyn_v = DynamicSIQuantity::from_unit(12.0, ElectricPotentialUnit::Volt);
    let dyn_i = DynamicSIQuantity::from_unit(2.0, ElectricCurrentUnit::Ampere);
    let dyn_p = dyn_v * dyn_i;
    println!(
        "DynamicSIQuantity: {} × {} = {:.0}",
        dyn_v
            .fmt_as_unit(ElectricPotentialUnit::Volt, Format::Short)
            .unwrap(),
        dyn_i
            .fmt_as_unit(ElectricCurrentUnit::Ampere, Format::Short)
            .unwrap(),
        dyn_p.fmt_as_unit(PowerUnit::Watt, Format::Long).unwrap(),
    );

    let dyn_mass = DynamicSIQuantity::from_unit(2.5, MassUnit::Kilogram);
    let dyn_force = DynamicSIQuantity::from_unit(1.0, ForceUnit::Newton);
    assert!((dyn_mass + dyn_force).is_none()); // dims mismatch → None
    println!("DynamicSIQuantity: mass + force is None (mismatched dims)");

    // ─── 9. Round-trip between typed and dynamic ────────────────────────────
    // `From<Qty<D>>` for `DynamicSIQuantity` always succeeds; `TryFrom<DynamicSIQuantity>` for
    // `Qty<D>` succeeds only if dims match.

    let typed = Mass::from_unit(2.0, MassUnit::Kilogram);
    let dyn_: DynamicSIQuantity = typed.into();
    let back: Mass = dyn_.try_into().expect("dims match");
    println!(
        "typed → DynamicSIQuantity → typed: {}",
        back.fmt_as_unit(MassUnit::Kilogram, Format::Long)
    );
}
