//! A prefix scales a unit enum's value, chosen at run time: `Prefixed<U>`
//! parses (`km`), prints and is a unit of what `U` measures.
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use yadc::Unit;
use yadc_si::dynamic::mechanical::{AreaUnit, LengthUnit, PressureUnit, TimeUnit};
use yadc_si::mechanical::units::{Metre, Pascal, Second, SquareMetre};
use yadc_si::mechanical::{Area, Length, Pressure, Time};
use yadc_si::{Prefix, Prefixed};

#[test]
fn a_prefix_scales_a_unit() {
    let km = Prefixed {
        prefix: Some(Prefix::Kilo),
        unit: LengthUnit::Metre,
    };
    let road: Length<f64> = km.quantity(3.0);
    assert_eq!(Metre {}.value(road), 3000.0);
    let um = Prefixed {
        prefix: Some(Prefix::Micro),
        unit: LengthUnit::Metre,
    };
    let hair: Length<f32> = um.quantity(70.0);
    assert!((Metre {}.value(hair) - 70e-6).abs() < 1e-10);
    let kpa = Prefixed {
        prefix: Some(Prefix::Kilo),
        unit: PressureUnit::Pascal,
    };
    assert_eq!(kpa.value(Pascal {}.quantity(1500.0_f64)), 1.5);
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
    assert_eq!(km.value(Metre {}.quantity(2500.0_f64)), 2.5);
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
    assert_eq!(Metre {}.value(kft.quantity(1.0_f64)), 304.8);
    assert!(parse("xm").is_err());
    assert_eq!(parse("m").unwrap(), Prefixed::from(LengthUnit::Metre));

    let mpa = "mPa".parse::<Prefixed<PressureUnit>>().unwrap();
    let p: Pressure<f64> = mpa.quantity(2.0);
    assert_eq!(Pascal {}.value(p), 0.002);
    let min = "min".parse::<Prefixed<TimeUnit>>().unwrap();
    assert_eq!(min.prefix, None);
    let ms = "ms".parse::<Prefixed<TimeUnit>>().unwrap();
    let t: Time<f64> = ms.quantity(5.0);
    assert!((Second {}.value(t) - 0.005).abs() < 1e-15);
}

/// A prefix scales a unit's first symbol with its power: `km²` is a million
/// square metres.
#[test]
fn a_prefix_scales_a_power() {
    let km2 = "km²".parse::<Prefixed<AreaUnit>>().unwrap();
    assert_eq!(km2.prefix, Some(Prefix::Kilo));
    let a: Area<f64> = km2.quantity(1.0);
    assert_eq!(SquareMetre {}.value(a), 1e6);
    assert_eq!(format!("{km2} {km2:#}"), "km² square kilometres");
    assert_eq!(
        format!("[{:>5}]", "km".parse::<Prefixed<LengthUnit>>().unwrap()),
        "[   km]"
    );
}
