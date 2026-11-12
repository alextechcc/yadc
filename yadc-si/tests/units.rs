//! Units print and parse through std traits; dimensions may leave out the
//! label; a quantity's number type may be a generic parameter.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(
    const_cmp,
    const_default,
    derive_const,
    const_ops,
    const_convert,
    const_try
)]
#![feature(const_destruct)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use yadc::{Dimension, PartialPowi, PowiAs, RootAs, Unit, UnknownUnit};
use yadc_si::mechanical::{Length, consts};
use yadc_si::dynamic::mechanical::{LengthUnit, MassUnit, TimeUnit};
use yadc_si::{KILO, Prefix, Prefixed};

const METRE: Length<f64> = consts::METRE.into();

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
}

// --- Display and FromStr ------------------------------------------------------

#[test]
fn units_print_their_symbol_and_long_name() {
    // `{}` is the symbol, `{:#}` the long name: the unit's own `Display`.
    assert_eq!(format!("{}", LengthUnit::Mile), "mi");
    assert_eq!(format!("{:#}", LengthUnit::Mile), "miles");
    assert_eq!(LengthUnit::Mile.to_string(), "mi");
    // A prefixed unit prints the prefix's symbol or name before the unit's.
    let km = Prefixed { prefix: Some(Prefix::Kilo), unit: LengthUnit::Metre };
    assert_eq!(format!("{}", km), "km");
    assert_eq!(format!("{:#}", km), "kilometres");
    assert_eq!(km.to_string(), "km");
    // Width, fill and alignment apply, as for `str`.
    assert_eq!(format!("{:>6}", LengthUnit::Mile), "    mi");
    assert_eq!(format!("{:-<#8}", LengthUnit::Mile), "miles---");
    assert_eq!(
        format!("{} / {:#}", MassUnit::Kilogram, MassUnit::Kilogram),
        "kg / kilograms"
    );
    // A quantity shown in a unit: the number the unit reads, with its own
    // flags, then the unit.
    let d = km.quantity(1.5);
    let m = LengthUnit::Metre;
    assert_eq!(format!("{:.2} {}", km.value(d), km), "1.50 km");
    assert_eq!(format!("{} {:#}", m.value(d), m), "1500 metres");
    assert_eq!(format!("{:e} {}", m.value(d), m), "1.5e3 m");
    // A quantity prints itself as its number and its dimension.
    assert_eq!(format!("{d}"), "1500 m");
}

#[test]
fn units_parse_from_their_symbol() {
    let km: Prefixed<LengthUnit> = "km".parse().unwrap();
    assert_eq!(km.to_string(), "km");
    close(km.value(LengthUnit::Metre.quantity(2500.0)), 2.5);
    // Every unit round-trips through its symbol.
    for unit in [LengthUnit::Metre, LengthUnit::Mile, LengthUnit::Angstrom] {
        let back: LengthUnit = unit.to_string().parse().unwrap();
        assert_eq!(back.to_string(), unit.to_string());
    }
    // An unknown symbol - or a long name - is an error.
    assert_eq!(
        "furlongs per fortnight".parse::<LengthUnit>().err(),
        Some(UnknownUnit)
    );
    assert!("kilometres".parse::<LengthUnit>().is_err());
    assert!("kg".parse::<LengthUnit>().is_err());
    assert_eq!(UnknownUnit.to_string(), "unknown unit symbol");
    let _: Result<TimeUnit, UnknownUnit> = "h".parse();
}

// A unit may leave out the derived `FromStr` - `impl(Display)` writes only
// `Display` - and write its own: here one that also accepts the long names.
#[derive(Copy, Clone, Debug, PartialEq, yadc::Unit)]
#[unit(of = Length<f64>, impl(Display))]
enum Distance {
    #[unit("m", "metres", by = METRE)]
    Metre,
    #[unit("km", "kilometres", by = Length::<f64>::from(KILO * consts::METRE))]
    Kilometre,
}

impl core::str::FromStr for Distance {
    type Err = UnknownUnit;
    fn from_str(s: &str) -> Result<Self, UnknownUnit> {
        match s {
            "m" | "metre" | "metres" => Ok(Distance::Metre),
            "km" | "kilometre" | "kilometres" => Ok(Distance::Kilometre),
            _ => Err(UnknownUnit),
        }
    }
}

#[test]
fn a_hand_written_from_str() {
    assert_eq!("kilometres".parse(), Ok(Distance::Kilometre));
    assert_eq!("m".parse(), Ok(Distance::Metre));
    assert_eq!("mile".parse::<Distance>(), Err(UnknownUnit));
    assert_eq!(format!("{:#}", Distance::Kilometre), "kilometres");
    close(
        Distance::Kilometre.value(Distance::Metre.quantity(500.0)),
        0.5,
    );
}

// --- A dimension without a label ---------------------------------------------------

mod kindless {
    use super::*;

    /// No `#[kind]` field: there is no label, so only the exponents decide.
    #[derive_const(PartialEq)]
    #[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
    pub struct Dim {
        pub m: i8,
        pub s: i8,
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
    pub struct Qty<const D: Dim>(f64);

    /// Its dynamic version, with dynamickit; only `+` is offered here: the
    /// dimension's own, which may refuse.
    #[derive(Copy, Clone, PartialEq, Debug, dynamickit::Dynamic)]
    #[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]
    #[dynamic(impl(Add -> Option<Self>))]
    pub struct DynQty {
        dim: Dim,
        value: f64,
    }

    pub type Length = Qty<
        {
            Dim {
                m: 1,
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Area = Qty<
        {
            Dim {
                m: 2,
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Time = Qty<
        {
            Dim {
                s: 1,
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Speed = Qty<{ Dim { m: 1, s: -1 } }>;

    /// The base constants: the only raw numbers.
    pub const METRE: Length = Qty(1.0);
    pub const SECOND: Time = Qty(1.0);

    #[derive(Copy, Clone, yadc::Unit)]
    #[unit(of = Length)]
    pub enum LengthUnit {
        #[unit("m", "metres", by = METRE)]
        Metre,
    }
}

#[test]
fn a_kindless_dimension() {
    use kindless::*;
    // A number becomes a quantity in steps of a constant, and reads back as
    // the ratio to it - what a unit does.
    let a = 3.0 * METRE;
    let b: Length = LengthUnit::Metre.quantity(4.0);
    let t = 2.0 * SECOND;
    let area: Area = a * b;
    close((METRE * METRE).value(area), 12.0);
    let v: Speed = (a + b) / t;
    close((METRE / SECOND).value(v), 3.5);
    close(LengthUnit::Metre.value(area.root::<2>()), 12f64.sqrt());
    // Only exponents are checked: lengths add, a length and a time do not.
    // A static quantity's dimension is its const parameter.
    fn dim_of<const D: Dim>(_: Qty<D>) -> Dim {
        D
    }
    assert_eq!(dim_of(METRE) + dim_of(SECOND), None);
    assert_eq!(dim_of(METRE) + dim_of(METRE), Some(dim_of(METRE)));
    assert_eq!(dim_of(METRE).partial_powi(2), Some(dim_of(METRE * METRE)));
    assert_eq!(format!("{v}"), "3.5 m·s⁻¹");
    assert_eq!(DynQty::from(a) + DynQty::from(t), None);
    let d = (DynQty::from(a) + DynQty::from(b)).unwrap();
    close(LengthUnit::Metre.value(Length::try_from(d).unwrap()), 7.0);
    assert_eq!(Dim::DIMENSIONLESS, Dim { m: 0, s: 0 });
}

// --- A generic number type ------------------------------------------------------

mod generic {
    use super::*;

    #[derive_const(PartialEq)]
    #[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
    pub struct Dim {
        pub m: i8,
        pub s: i8,
    }

    /// One quantity type for every number type: `Qty<f32, D>`, `Qty<f64, D>`.
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
    pub struct Qty<V, #[dim] const D: Dim> {
        #[value]
        pub v: V,
    }

    pub type Length<V> = Qty<
        V,
        {
            Dim {
                m: 1,
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Area<V> = Qty<
        V,
        {
            Dim {
                m: 2,
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Time<V> = Qty<
        V,
        {
            Dim {
                s: 1,
                ..Dim::DIMENSIONLESS
            }
        },
    >;
}

#[test]
fn a_generic_number_type() {
    use generic::*;
    let a32: Length<f32> = Qty { v: 3.0f32 };
    let b32: Length<f32> = Qty { v: 4.0f32 };
    let area32: Area<f32> = a32 * b32;
    assert_eq!(area32.v, 12.0f32);
    assert_eq!((a32 + b32).v, 7.0f32);

    let a64: Length<f64> = Qty { v: 1.5 };
    let t64: Time<f64> = Qty { v: 0.5 };
    let speed = a64 / t64;
    close(speed.v, 3.0);
    close((a64 * 2.0).v, 3.0);
    let sq: Area<f64> = a64.powi::<2>();
    close(sq.v, 2.25);
}

#[test]
fn exponents_parse_what_they_print() {
    use yadc::Exponent;
    for e in [Exponent::int(3), Exponent::new(-5, 2), Exponent::new(4, 6)] {
        assert_eq!(e.to_string().parse::<Exponent>(), Ok(e));
    }
    assert_eq!("2/4".parse::<Exponent>(), Ok(Exponent::new(1, 2)));
    assert_eq!("3/-6".parse::<Exponent>(), Ok(Exponent::new(-1, 2)));
    assert!("1/0".parse::<Exponent>().is_err());
    assert!("300".parse::<Exponent>().is_err());
    assert!("half".parse::<Exponent>().is_err());
    // Exponent arithmetic is checked: a sum that does not fit is `None`.
    assert_eq!(Exponent::int(100) + Exponent::int(100), None);
}
