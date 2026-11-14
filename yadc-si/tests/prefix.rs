//! A prefix is a constant of the plain number's unit that fuses with the one
//! unprefixed named unit it multiplies: `KILO * GRAM` is the kilogram, its
//! size a thousand grams, printed `kg`. Quantities multiply only where both
//! are counted in base units or neither is, so a prefix (not a base unit)
//! multiplies a constant of another named unit; a prefixed base unit is a
//! constant of its unit's product: `KILO.unit().times(METRE.unit(), 1)`.
#![feature(generic_const_exprs, const_trait_impl)]
#![allow(incomplete_features)]

use yadc_si::mechanical::{FOOT, GRAM, KILOMETRE, Length, MINUTE, Mass, NAUTICAL_MILE, Time};
use yadc_si::{Exact, KILO, KILOGRAM, MEGA, METRE, MICRO, MILLI, SECOND, SIQuantity};

/// A megametre: a prefixed base unit.
const MEGAMETRE: SIQuantity<{ MEGA.unit().times(METRE.unit(), 1) }, Exact, false> = SIQuantity::ONE;
/// A micrometre.
const MICROMETRE: SIQuantity<{ MICRO.unit().times(METRE.unit(), 1) }, Exact, false> =
    SIQuantity::ONE;

#[test]
fn a_prefix_scales_a_unit() {
    let kilo_gram: Mass<Exact> = (KILO * GRAM).into();
    assert_eq!(kilo_gram, KILOGRAM);
    let mg: Mass<f64> = (250.0_f64 * MILLI * GRAM).into();
    assert!((f64::from(mg / (1.0 * KILOGRAM)) - 250e-6).abs() < 1e-18);
    // Any named unit takes a prefix: a kilofoot is 304.8 m exactly.
    let kft: Length<f64> = (1.0_f64 * KILO * FOOT).into();
    assert_eq!(f64::from(kft / (1.0 * METRE)), 304.8);
    // A prefix on a unit with a size of its own: milliminutes.
    let mmin: Time<f64> = (5.0_f64 * MILLI * MINUTE).into();
    assert!((f64::from(mmin / (1.0 * SECOND)) - 0.3).abs() < 1e-15);
}

#[test]
fn a_prefixed_base_unit() {
    assert_eq!(MEGAMETRE.unit().size(), Exact::int(1_000_000));
    assert_eq!(
        KILO.unit().times(METRE.unit(), 1).size(),
        KILOMETRE.unit().size()
    );
    let road: Length<f64> = (3.0_f64 * MEGAMETRE).into();
    assert_eq!(f64::from(road / (1.0 * METRE)), 3e6);
    let hair: Length<f32> = (70.0_f32 * MICROMETRE).into();
    assert!((f32::from(hair / (1.0_f32 * METRE)) - 70e-6).abs() < 1e-10);
    let back: SIQuantity<{ MICROMETRE.unit() }, f32, false> = hair.into();
    assert!((f32::from(back / (1.0_f32 * MICROMETRE)) - 70.0).abs() < 1e-3);
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
