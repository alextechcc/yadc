//! The derived impls at run time and at compile time: products of points,
//! powers, units of every form, and `Display`.
#![feature(generic_const_exprs, const_trait_impl, const_ops, const_cmp)]
#![allow(incomplete_features)]
// `a - a` is asked of the algebra on purpose: the difference of two of a kind.
#![allow(clippy::eq_op)]

use yadc::{Dimension, PartialPowi, PartialRoot, Unit};
use yadc_tests::*;

// The dimension's own `*` decides which products exist, at compile time.
const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};
const ALTITUDE: Dim = Dim {
    m: 1,
    kind: Kind::Affine,
    ..Dim::DIMENSIONLESS
};
const POWER: Dim = Dim {
    kg: 1,
    m: 2,
    s: -3,
    ..Dim::DIMENSIONLESS
};
const LEVEL: Dim = Dim {
    kind: Kind::Log,
    ..POWER
};
const RANK: Dim = Dim {
    kind: Kind::Rank,
    ..Dim::DIMENSIONLESS
};
const GAIN: Dim = Dim {
    kind: Kind::Log,
    ..Dim::DIMENSIONLESS
};
const fn is(a: Option<Dim>, b: Dim) -> bool {
    matches!(a, Some(d) if d == b)
}
const _: () = assert!(matches!(LENGTH * LENGTH, Some(d) if d.m == 2));
const _: () = assert!(matches!(LENGTH.partial_powi(-2), Some(d) if d.m == -2));
// A point: no products, powers or roots; two differ by a plain amount, and
// a plain amount moves one.
const _: () = assert!((ALTITUDE * LENGTH).is_none() && (LENGTH / ALTITUDE).is_none());
const _: () = assert!((ALTITUDE * Dim::DIMENSIONLESS).is_none());
const _: () = assert!(ALTITUDE.partial_powi(2).is_none() && ALTITUDE.partial_root(2).is_none());
const _: () = assert!(ALTITUDE.partial_powi(1).is_none() && ALTITUDE.partial_root(1).is_none());
const _: () = assert!(is(ALTITUDE - ALTITUDE, LENGTH) && is(ALTITUDE + LENGTH, ALTITUDE));
const _: () = assert!(is(ALTITUDE - LENGTH, ALTITUDE));
const _: () = assert!((ALTITUDE + ALTITUDE).is_none() && (LENGTH + ALTITUDE).is_none());
const _: () = assert!((LENGTH - ALTITUDE).is_none() && (ALTITUDE + POWER).is_none());
// A logarithm: adding is multiplying what it is the logarithm of, so `+` and
// `-` add and subtract the exponents. A level less a level is a gain, a gain
// moves a level, and two levels sum to the logarithm of watts squared.
const _: () = assert!(is(LEVEL - LEVEL, GAIN) && is(LEVEL + GAIN, LEVEL) && is(LEVEL - GAIN, LEVEL));
const _: () = assert!(is(GAIN + GAIN, GAIN) && is(GAIN + LEVEL, LEVEL));
const _: () = assert!(matches!(LEVEL + LEVEL, Some(d) if d.kg == 2 && d.m == 4 && d.kind == Kind::Log));
const _: () = assert!((LEVEL + POWER).is_none() && (POWER + LEVEL).is_none() && (GAIN + Dim::DIMENSIONLESS).is_none());
// Beside a plain number a product is a scaling: a gain scales (gains add),
// a level does not (two levels sum to another level's square), nor does a
// point or a label with no sum. A gain leaves any other product as a label
// does; a level has no other products.
const _: () = assert!(is(GAIN * Dim::DIMENSIONLESS, GAIN) && is(Dim::DIMENSIONLESS * GAIN, GAIN));
const _: () = assert!((LEVEL * Dim::DIMENSIONLESS).is_none() && (LEVEL / Dim::DIMENSIONLESS).is_none());
const _: () = assert!((RANK * Dim::DIMENSIONLESS).is_none() && is(LENGTH * Dim::DIMENSIONLESS, LENGTH));
const _: () = assert!(matches!(GAIN / LENGTH, Some(d) if d.m == -1 && d.kind == Kind::Linear));
const _: () = assert!((LEVEL * LENGTH).is_none() && (GAIN * GAIN).is_none() && (LENGTH / LEVEL).is_none());
const _: () = assert!(GAIN.partial_root(2).is_none() && matches!(GAIN.partial_root(1), Some(d) if d == GAIN));
// An exponent that does not fit its type has no product.
const BIG: Dim = Dim { m: 100, ..Dim::DIMENSIONLESS };
const _: () = assert!((BIG * BIG).is_none() && (LENGTH.partial_powi(-128)).is_none());
const _: () = assert!(matches!(Kind::Rank * Kind::Linear, Kind::Rank));
const BODY: Temperature = CELSIUS.quantity(37.0); // a scale, in `const`
const _: () = assert!(TemperatureUnit::Kelvin.value(BODY) > 310.0);

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn units_of_every_form() {
    // A scale: a zero and a step.
    let t: Temperature = CELSIUS.quantity(100.0);
    assert!(close(TemperatureUnit::Kelvin.value(t), 373.15));
    assert!(close(TemperatureUnit::Fahrenheit.value(t), 212.0));
    assert!(close(CELSIUS.value(t), 100.0));
    // A quantity constant.
    assert!(close(FOOT.value(3.0 * METRE), 3.0 / 0.3048));
    // A scale of points, and one made at run time.
    let aircraft: Altitude = FEET_AMSL.quantity(150.0);
    let ground = Scale(SEA_LEVEL + 100.0 * FOOT, FOOT);
    assert!(close(ground.value(aircraft), 50.0));
    assert!(close(FeetAboveGround { scale: ground }.value(aircraft), 50.0));
    assert!(close(
        PressureUnit::Kilopascal.value(PressureUnit::Pascal.quantity(2500.0)),
        2.5
    ));
}

#[test]
fn levels() {
    // A level is a scale on the logarithmic line: it reads a `PowerLevel`,
    // and the watts are one `From` away.
    let tx = DBM.quantity(30.0);
    assert!(close(WATT.value(Power::from(tx)), 1.0));
    assert!(close(DBM.value(PowerLevel::from(2.0 * WATT)), 30.0 + 10.0 * 2f64.log10()));
    assert!(close(DBM.value(MILLIWATT.into()), 0.0));
    // On the line, multiplying is adding: a level moves by gains.
    let rx = tx - 3.0 * DECIBEL + 13.0 * DECIBEL;
    assert!(close(DBM.value(rx), 40.0));
    assert!(close(WATT.value(Power::from(rx)), 10.0));
    // Two levels differ by a gain.
    let gain: Gain = rx - tx;
    assert!(close(DECIBEL.value(gain), 10.0));
    // Another zero on the same line.
    let dbw = Scale(WATT_LEVEL, DECIBEL);
    assert!(close(dbw.value(rx), 10.0));
}

/// A unit builds with `quantity` and reads with `value`, in `const` too; a
/// constant times a number builds as well.
const MARATHON: Length = METRE.quantity(42_195.0);
const BOILING: Temperature = CELSIUS.quantity(100.0);
const BOILING_K: f64 = TemperatureUnit::Kelvin.value(BOILING);

#[test]
fn units_build_and_read() {
    assert!(close(FOOT.value(MARATHON), 42_195.0 / 0.3048));
    assert!(close(BOILING_K, 373.15));
    assert!(close(CELSIUS.value(BOILING), 100.0));
    assert!(close(KILOPASCAL.value(PASCAL * 2500.0), 2.5));
    let aircraft = FEET_AMSL.quantity(6500.0);
    let clearance = aircraft - FEET_AMSL.quantity(1200.0);
    assert!(close(FOOT.value(clearance), 5300.0));
    let agl = Scale(FEET_AMSL.quantity(1200.0), METRE);
    assert!(close(agl.value(aircraft), 5300.0 * 0.3048));
    assert!(close(DBM.value(DBM.quantity(30.0) + DECIBEL * 3.0), 33.0));
    // A unit enum's values build and read the same way; a level reads the
    // watts `via` its line.
    let (celsius, dbm) = (TemperatureUnit::Celsius, PowerUnit::DecibelMilliwatt);
    assert!(close(celsius.value(celsius.quantity(21.5)), 21.5));
    assert!(close(WATT.value(dbm.quantity(30.0)), 1.0));
    assert!(close(dbm.value(WATT * 2.0), 30.0 + 10.0 * 2f64.log10()));
    // Generic code asks for `Unit`: constants, scales and unit enums meet it.
    fn convert<Q>(n: f64, from: impl Unit<Q>, to: impl Unit<Q>) -> f64 {
        to.value(from.quantity(n))
    }
    assert!(close(convert(12.0, FOOT, METRE), 12.0 * 0.3048));
    let fahrenheit = TemperatureUnit::Fahrenheit;
    assert!(close(convert(0.0, celsius, fahrenheit), 32.0));
    assert!(close(convert(100.0, CELSIUS, fahrenheit), 212.0));
}

#[test]
fn unit_labels() {
    let celsius = TemperatureUnit::Celsius;
    assert_eq!(format!("{celsius} / {celsius:#}"), "°C / degrees Celsius");
    assert_eq!(format!("[{:>4}]", TemperatureUnit::Kelvin), "[   K]");
    assert_eq!(
        format!("{:.1} {celsius}", celsius.value(ICE_POINT)),
        "0.0 °C"
    );
    assert_eq!(
        "°F".parse::<TemperatureUnit>(),
        Ok(TemperatureUnit::Fahrenheit)
    );
    assert_eq!("x".parse::<TemperatureUnit>(), Err(yadc::UnknownUnit));
    let agl = FeetAboveGround {
        scale: Scale(SEA_LEVEL, FOOT),
    };
    assert_eq!(format!("{agl}"), "ft AGL");
}

#[test]
fn dimension_display() {
    let one = Dim::DIMENSIONLESS;
    let pressure = Dim { kg: 1, m: -1, s: -2, ..one };
    let power = Dim { kg: 1, m: 2, s: -3, ..one };
    let temperature = Dim { k: 1, kind: Kind::Affine, ..one };
    let gain = Dim { kind: Kind::Log, ..one };
    assert_eq!(pressure.to_string(), "kg·m⁻¹·s⁻²");
    assert_eq!(power.to_string(), "kg·m²·s⁻³");
    assert_eq!(temperature.to_string(), "k{Affine}");
    assert_eq!(gain.to_string(), "{Log}");
    assert_eq!(Dim::DIMENSIONLESS.to_string(), "");
    assert_eq!(Kind::Affine.to_string(), "Affine");
}

#[test]
fn quantity_display() {
    assert_eq!(
        format!("{:.1}", 101.325 * KILOPASCAL),
        "101325.0 kg·m⁻¹·s⁻²"
    );
    assert_eq!(format!("{}", ONE / 8.0), "0.125");
    assert_eq!(format!("{:+.2}", ICE_POINT), "+273.15 k{Affine}");
    assert_eq!(format!("{}", SEA_LEVEL + 3.0 * METRE), "3 m{Affine}");
}

#[test]
fn points_have_no_products_but_move() {
    let a: Altitude = SEA_LEVEL + 10.0 * METRE;
    let d: Length = a - SEA_LEVEL;
    assert!(close(METRE.value(d), 10.0));
}

/// An odd root keeps the sign: ∛(-8 m³) is -2 m, in the static type and in
/// a dimension-free float root alike; an even root of a negative is NaN.
#[test]
fn odd_roots_keep_the_sign() {
    use yadc::RootAs;
    let cube = METRE * METRE * METRE * -8.0;
    let side: Length = cube.root::<3>();
    assert!(close(METRE.value(side), -2.0));
    assert!(close((-8.0f64).partial_root(3).unwrap(), -2.0));
    assert!((-4.0f64).partial_root(2).unwrap().is_nan());
}
