//! A prefix is a constant of the plain number's unit that fuses with the one
//! unprefixed named unit it multiplies: `KILO * METRE` is the kilometre,
//! `KILO * GRAM` a thousand grams, printed `kg`. A prefix is not a base unit,
//! so neither is a prefixed unit.
#![feature(generic_const_exprs, const_trait_impl, const_ops)]
#![allow(incomplete_features)]

use yadc_si::mechanical::{FOOT, GRAM, KILOMETRE, Length, MINUTE, Mass, NAUTICAL_MILE, Time};
use yadc_si::{Exact, KILO, KILOGRAM, MEGA, METRE, MICRO, MILLI, NOT_BASE, SECOND, SIQuantity};

/// A megametre: a prefixed base unit.
const MEGAMETRE: SIQuantity<{ MEGA.unit().times(METRE.unit(), 1) }, Exact, NOT_BASE> = MEGA * METRE;
/// A micrometre.
const MICROMETRE: SIQuantity<{ MICRO.unit().times(METRE.unit(), 1) }, Exact, NOT_BASE> =
    MICRO * METRE;

#[test]
fn a_prefix_scales_a_unit() {
    let kilo_gram: Mass<Exact> = (KILO * GRAM).into();
    assert_eq!(kilo_gram, KILOGRAM);
    let mg: Mass<f64> = (250.0_f64 * MILLI * GRAM).into();
    assert!((f64::from(mg / KILOGRAM) - 250e-6).abs() < 1e-18);
    // Any named unit takes a prefix: a kilofoot is 304.8 m exactly.
    let kft: Length<f64> = (1.0_f64 * KILO * FOOT).into();
    assert_eq!(f64::from(kft / METRE), 304.8);
    // A prefix on a unit with a size of its own: milliminutes.
    let mmin: Time<f64> = (5.0_f64 * MILLI * MINUTE).into();
    assert!((f64::from(mmin / SECOND) - 0.3).abs() < 1e-15);
}

#[test]
fn a_prefixed_base_unit() {
    assert_eq!(MEGAMETRE.unit().size(), Exact::int(1_000_000));
    assert_eq!(KILO * METRE, KILOMETRE);
    let road: Length<f64> = (3.0_f64 * MEGAMETRE).into();
    assert_eq!(f64::from(road / METRE), 3e6);
    assert_eq!(f64::from(road / KILOMETRE), 3000.0);
    let hair: Length<f32> = (70.0_f32 * MICROMETRE).into();
    assert!((f32::from(hair / METRE) - 70e-6).abs() < 1e-10);
    let back: SIQuantity<{ MICROMETRE.unit() }, f32, NOT_BASE> = hair.into();
    assert!((f32::from(back / MICROMETRE) - 70.0).abs() < 1e-3);
}

#[test]
fn prefixed_units_print_their_prefix() {
    assert_eq!((2.5_f64 * KILO * GRAM).to_string(), "2.5 kg");
    assert_eq!((1.0_f64 * KILO * FOOT).to_string(), "1 kft");
    assert_eq!((1.0_f64 * MEGAMETRE).to_string(), "1 Mm");
    assert_eq!((1.0_f64 * MICROMETRE).to_string(), "1 μm");
    // A unit's own symbol is not a prefix's: `M` is the nautical mile.
    assert_eq!((1.0_f64 * NAUTICAL_MILE).to_string(), "1 M");
}
