//! Tour of the `yadc-si` quantity system, one feature per section.
//!
//! Run with: cargo run --example basic_use --manifest-path yadc/Cargo.toml

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use yadc::{Format, Unit};
use yadc_si::DynamicSIQuantity;
use yadc_si::dimensionless::*;
use yadc_si::electromagnetic::*;
use yadc_si::mechanical::*;
use yadc_si::thermal::*;

fn main() {
    // --- 1. Construction from a unit ----------------------------------------
    // Conversion is unit-led: `unit.quantity(value)` builds the quantity (stored
    // in SI base units), `unit.from_quantity(q)` / `unit.format_quantity(q, fmt)`
    // read it back.

    let mass = MassUnit::Kilogram.quantity(70.0);
    let accel = AccelerationUnit::StandardGravity.quantity(1.0);
    println!("mass  = {}", MassUnit::Kilogram.format_quantity(mass, Format::Long));
    println!(
        "accel = {}",
        AccelerationUnit::StandardGravity.format_quantity(accel, Format::Long)
    );

    let mass_array = [
        MassUnit::Kilogram.quantity(1.0),
        MassUnit::Gram.quantity(500.0),
        MassUnit::Gram.quantity(250.0),
    ];
    let total = mass_array.iter().sum::<Mass>();
    println!("sum   = {}", MassUnit::Carat.format_quantity(total, Format::Short));

    // --- 2. Mul / Div compose dimensions ------------------------------------
    // `mass * accel` is type-checked to be Force.

    let force: Force = mass * accel;
    println!("force = {:.2}", ForceUnit::Newton.format_quantity(force, Format::Short));

    let voltage = ElectricPotentialUnit::Volt.quantity(12.0);
    let current = ElectricCurrentUnit::Milliampere.quantity(500.0);
    let power: Power = voltage * current;
    let resistance: ElectricalResistance = voltage / current;
    println!(
        "{} x {} -> {:.2}",
        ElectricPotentialUnit::Volt.format_quantity(voltage, Format::Short),
        ElectricCurrentUnit::Milliampere.format_quantity(current, Format::Short),
        PowerUnit::Watt.format_quantity(power, Format::Long),
    );
    println!(
        "{} ÷ {} -> {:.1}",
        ElectricPotentialUnit::Volt.format_quantity(voltage, Format::Short),
        ElectricCurrentUnit::Milliampere.format_quantity(current, Format::Short),
        ElectricalResistanceUnit::Ohm.format_quantity(resistance, Format::Short),
    );

    // --- 3. Same-dim division -> Dimensionless -------------------------------
    // `Force / Force` is type-checked to be `Dimensionless`.

    let a = ForceUnit::Newton.quantity(9.0);
    let b = ForceUnit::Newton.quantity(3.0);
    let ratio: Dimensionless = a / b;
    println!(
        "9 N / 3 N = {} (or {})",
        DimensionlessUnit::Ratio.format_quantity(ratio, Format::Short),
        DimensionlessUnit::Percent.format_quantity(ratio, Format::Short),
    );

    // --- 4. Affine temperature: T - T = DeltaT ------------------------------
    // `ThermodynamicTemperature` carries `Kind::Affine` so the type system
    // distinguishes it from a plain temperature interval. Subtracting two
    // absolute temperatures produces an *interval*, not another absolute T.

    let body = ThermodynamicTemperatureUnit::Celsius.quantity(37.0);
    let room = ThermodynamicTemperatureUnit::Celsius.quantity(22.0);
    let fever: TemperatureInterval = body - room;
    println!(
        "body - room = {}",
        TemperatureIntervalUnit::Celsius.format_quantity(fever, Format::Long),
    );

    // Affine + Linear stays absolute: `T + DeltaT = T`.
    let warmer: ThermodynamicTemperature = body + fever;
    println!(
        "body + fever = {:.1}",
        ThermodynamicTemperatureUnit::Celsius.format_quantity(warmer, Format::Short),
    );

    // --- 5. Display passthrough via format flags ----------------------------
    // `format_quantity` returns a `UnitDisplay` that respects `{:.2}`, `{:>10.3}`,
    // `{:+}`, etc.

    let g = ForceUnit::Newton.format_quantity(force, Format::Long);
    println!("plain   : {g}");
    println!("2 dp    : {g:.2}");
    println!("width 12: {g:>12.3}");

    // --- 6. Raw scalar via from_quantity ------------------------------------
    // No `Display` wrapper - just an `f64` you can do math with.

    let kg = MassUnit::Kilogram.from_quantity(MassUnit::Gram.quantity(1500.0));
    println!("1500 g as kg (raw): {kg}");

    // --- 7. Scalar arithmetic (Qty * f64, etc.) -----------------------------
    // A quantity can be multiplied or divided by a plain scalar. The result
    // keeps the same dimension. `*=` / `/=` work in place.

    let m1 = MassUnit::Kilogram.quantity(10.0);
    let doubled = m1 * 2.0;
    let halved = m1 / 2.0;
    let from_left: Mass = 3.0 * m1;
    println!(
        "10 kg x 2.0 = {}, 10 kg / 2.0 = {}, 3.0 x 10 kg = {}",
        MassUnit::Kilogram.format_quantity(doubled, Format::Short),
        MassUnit::Kilogram.format_quantity(halved, Format::Short),
        MassUnit::Kilogram.format_quantity(from_left, Format::Short),
    );

    let mut tank = MassUnit::Kilogram.quantity(1.0);
    tank *= 4.0;
    tank /= 2.0;
    println!(
        "1 kg *= 4.0 /= 2.0 -> {}",
        MassUnit::Kilogram.format_quantity(tank, Format::Short),
    );

    // --- 8. Dynamic dimensions with DynamicSIQuantity -----------------------
    // `DynamicSIQuantity` carries its dimension at runtime. Build one by
    // converting a static quantity (`unit.quantity(v).into()`); read it back with
    // the unit's `try_*` methods (`None` on a dim mismatch). Arithmetic that
    // combines dims (`*`, `/`) always succeeds; matching-dim ops (`+`, `-`)
    // return `Option`.

    let dyn_v = DynamicSIQuantity::from(ElectricPotentialUnit::Volt.quantity(12.0));
    let dyn_i = DynamicSIQuantity::from(ElectricCurrentUnit::Ampere.quantity(2.0));
    // `*` / `/` on DynamicSIQuantity return `Option` (`None` only on an invalid
    // kind result); V x A composes to a valid Power, so unwrap.
    let dyn_p = (dyn_v * dyn_i).unwrap();
    println!(
        "DynamicSIQuantity: {} x {} = {:.0}",
        ElectricPotentialUnit::Volt.try_format_quantity(dyn_v, Format::Short).unwrap(),
        ElectricCurrentUnit::Ampere.try_format_quantity(dyn_i, Format::Short).unwrap(),
        PowerUnit::Watt.try_format_quantity(dyn_p, Format::Long).unwrap(),
    );

    let dyn_mass = DynamicSIQuantity::from(MassUnit::Kilogram.quantity(2.5));
    let dyn_force = DynamicSIQuantity::from(ForceUnit::Newton.quantity(1.0));
    assert!((dyn_mass + dyn_force).is_none()); // dims mismatch -> None
    println!("DynamicSIQuantity: mass + force is None (mismatched dims)");

    // --- 9. Logarithmic units: dB as a unit of Power ------------------------
    // dBW / dBm are nonlinear *units* of `Power` (no separate log type): the
    // stored value is watts, and the unit converts via 10^(x/10).
    let p = PowerUnit::DecibelWatt.quantity(20.0); // 20 dBW = 100 W
    println!("20 dBW = {:.0}", PowerUnit::Watt.format_quantity(p, Format::Short)); // 100 W
    println!(
        "       = {:.0}",
        PowerUnit::DecibelMilliwatt.format_quantity(p, Format::Short)
    ); // 50 dBm

    // --- 10. Round-trip between typed and dynamic ---------------------------
    // `From<Qty<D>>` for `DynamicSIQuantity` always succeeds; `TryFrom<DynamicSIQuantity>`
    // for `Qty<D>` succeeds only if dims match.

    let typed = MassUnit::Kilogram.quantity(2.0);
    let dyn_: DynamicSIQuantity = typed.into();
    let back: Mass = dyn_.try_into().expect("dims match");
    println!(
        "typed -> DynamicSIQuantity -> typed: {}",
        MassUnit::Kilogram.format_quantity(back, Format::Long)
    );
}
