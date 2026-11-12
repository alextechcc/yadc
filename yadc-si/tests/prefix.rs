//! A prefix is an exact number scaling a unit (`KILO * METRE`), and a unit
//! enum's values take one when parsed (`Prefixed<LengthUnit>`).
#![feature(generic_const_exprs, const_trait_impl, const_convert)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_si::mechanical::consts::{self, SQUARE_METRE};
use yadc_si::mechanical::{
    Area, AreaUnit, Length, LengthUnit, Pressure, PressureUnit, Time, TimeUnit,
};
use yadc_si::{Exact, KILO, MEGA, MICRO, MILLI, Prefix, Prefixed};

const METRE: Length<f64> = consts::METRE.into();
const PASCAL: Pressure<f64> = consts::PASCAL.into();
const SECOND: Time<f64> = consts::SECOND.into();

#[test]
fn a_prefix_scales_a_unit() {
    let km: Length<f64> = (KILO * consts::METRE).into();
    assert_eq!(METRE.value(km), 1000.0);
    let road = km.quantity(3.0);
    assert_eq!(METRE.value(road), 3000.0);
    let um: Length<f32> = (MICRO * consts::METRE).into();
    assert_eq!(um, Length::<f32>::from(consts::METRE) * 1e-6f32);
    assert_eq!(KILO * consts::METRE, consts::METRE * Exact::int(1000));
    let kpa: Pressure<f64> = (KILO * consts::PASCAL).into();
    assert_eq!(PASCAL.value(kpa), 1000.0);
}

#[test]
fn prefixed_units_parse_and_print() {
    let parse = |s: &str| s.parse::<Prefixed<LengthUnit>>();
    let km = parse("km").unwrap();
    assert_eq!(
        km,
        Prefixed {
            prefix: Some(Prefix::Kilo),
            unit: LengthUnit::Metre
        }
    );
    assert_eq!(format!("{km} {km:#}"), "km kilometres");
    assert_eq!(km.value(METRE * 2500.0), 2.5);
    // A unit's own symbol wins over a prefix: `M` is the nautical mile, `Mm`
    // a megametre.
    assert_eq!(parse("M").unwrap().prefix, None);
    assert_eq!(parse("Mm").unwrap().prefix, Some(Prefix::Mega));
    // Micro has three spellings.
    for s in ["µm", "μm", "um"] {
        assert_eq!(parse(s).unwrap().prefix, Some(Prefix::Micro));
    }
    // Any unit takes a prefix: a kilofoot is 304.8 m exactly.
    let kft = parse("kft").unwrap();
    assert_eq!(METRE.value(kft.quantity(1.0)), 304.8);
    assert!(parse("xm").is_err());
    assert_eq!(parse("m").unwrap(), Prefixed::from(LengthUnit::Metre));

    let pa = "mPa".parse::<Prefixed<PressureUnit>>().unwrap();
    let p: Pressure<f64> = pa.quantity(2.0);
    assert_eq!(p, Pressure::<f64>::from(MILLI * consts::PASCAL) * 2.0);
    let min = "min".parse::<Prefixed<TimeUnit>>().unwrap();
    assert_eq!(min.prefix, None);
    let ms = "ms".parse::<Prefixed<TimeUnit>>().unwrap();
    let t: Time<f64> = ms.quantity(5.0);
    assert!((SECOND.value(t) - 0.005).abs() < 1e-15);
}

/// A prefix scales a unit's first symbol with its power: `km²` is a million
/// square metres, and is `MEGA * SQUARE_METRE`.
#[test]
fn a_prefix_scales_a_power() {
    let km2 = "km²".parse::<Prefixed<AreaUnit>>().unwrap();
    assert_eq!(km2.prefix, Some(Prefix::Kilo));
    let a: Area<f64> = km2.quantity(1.0);
    let mega: Area<f64> = (MEGA * SQUARE_METRE).into();
    assert_eq!(a, mega);
    assert_eq!(format!("{km2} {km2:#}"), "km² square kilometres");
    assert_eq!(
        format!("[{:>5}]", "km".parse::<Prefixed<LengthUnit>>().unwrap()),
        "[   km]"
    );
}
