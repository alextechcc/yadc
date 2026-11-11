//! Blanket [`From`] / [`Into`] impls bridging yadc-si's const-generic [`ISQ`]
//! and uom's typenum-based ISQ dimension system.
//!
//! Each typenum exponent type (`P1`, `Z0`, `N1`, ...) carries a `const I8: i8`
//! via [`typenum::Integer`], so the const-generic ISQ value can be computed
//! directly from the type-level exponents without any runtime overhead.
//!
//! uom stores quantities in SI-base units in its `pub value: f64` field
//! regardless of which unit was used to construct them, so conversion is exact
//! in both directions.
//!
//! # Directionality
//!
//! | Direction | Unit system of result |
//! |-----------|-----------------------|
//! | `uom -> yadc-si` | any (`U: Units<f64>`) - value is already SI-base |
//! | `yadc-si -> uom` | [`uom::si::SI<f64>`] (standard SI) |
//!
//! `Into` impls are provided for free by the standard blanket
//! `impl<T, U: From<T>> Into<U> for T`.

use core::convert::TryFrom;
use core::marker::PhantomData;
use typenum::Integer;
use uom::si::{Quantity, Units, marker};

use crate::{DynamicSIQuantity, ISQ, Kind, StaticSIQuantity};

// --- Kind mapping -------------------------------------------------------------

/// Maps a uom kind marker (as a `dyn` trait object type) to the corresponding
/// yadc [`Kind`] variant, and says how the kind stands for radians.
///
/// uom follows the SI: angles are dimensionless, told apart by a kind. yadc-si
/// counts radians on an exponent axis instead, so a uom angle kind becomes a
/// `rad` exponent - which one depends on the quantity (see [`rad_of`]).
pub trait UomKindMapping {
    const YADC_KIND: Kind;
    /// How this kind stands for radians.
    const RADIANS: Radians = Radians::None;
}

/// How a uom kind stands for radians, which yadc-si counts on its `rad` axis.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Radians {
    /// The kind says nothing about angles.
    None,
    /// uom's angle kind: rad¹ on an angle and its rates, rad⁻¹ on torque and
    /// angular momentum (uom marks those with the angle kind to tell them from
    /// energy and action).
    Angle,
    /// A kind that stands for rad² (the steradian): solid angle, and
    /// illuminance (lux = cd·sr/m²).
    Squared,
}

/// The `rad` exponent of a uom quantity, from its exponents and its kind.
/// Moment of inertia is the one plain-kind quantity with radians: kg·m²/rad²,
/// so that E = ½·I·ω² is in joules.
#[allow(clippy::too_many_arguments)]
pub const fn rad_of(m: i8, kg: i8, s: i8, a: i8, k: i8, mol: i8, cd: i8, radians: Radians) -> i8 {
    match radians {
        Radians::Angle => {
            if m == 0 && kg == 0 {
                1
            } else {
                -1
            }
        }
        Radians::Squared => 2,
        Radians::None => {
            if m == 2 && kg == 1 && s == 0 && a == 0 && k == 0 && mol == 0 && cd == 0 {
                -2
            } else {
                0
            }
        }
    }
}

impl UomKindMapping for dyn uom::Kind {
    const YADC_KIND: Kind = Kind::Linear;
}
impl UomKindMapping for dyn marker::AngleKind {
    const YADC_KIND: Kind = Kind::Linear;
    const RADIANS: Radians = Radians::Angle;
}
impl UomKindMapping for dyn marker::SolidAngleKind {
    const YADC_KIND: Kind = Kind::Linear;
    const RADIANS: Radians = Radians::Squared;
}
impl UomKindMapping for dyn marker::TemperatureKind {
    const YADC_KIND: Kind = Kind::Absolute;
}
impl UomKindMapping for dyn marker::InformationKind {
    const YADC_KIND: Kind = Kind::Linear;
}
impl UomKindMapping for dyn marker::ConstituentConcentrationKind {
    const YADC_KIND: Kind = Kind::Linear;
}
impl UomKindMapping for dyn marker::SurfaceTensionKind {
    const YADC_KIND: Kind = Kind::Linear;
}
impl UomKindMapping for dyn marker::KinematicViscosityKind {
    const YADC_KIND: Kind = Kind::Linear;
}
impl UomKindMapping for dyn marker::IlluminanceKind {
    const YADC_KIND: Kind = Kind::Linear;
    const RADIANS: Radians = Radians::Squared;
}

// --- Const-generic helper -----------------------------------------------------

// Direct struct literals are not yet supported in generic-const positions
// (rustc: "overly complex generic constant"), but const-fn calls are.
#[allow(non_snake_case, clippy::too_many_arguments)]
const fn isq_of(
    m: i8,
    kg: i8,
    s: i8,
    a: i8,
    k: i8,
    mol: i8,
    cd: i8,
    kind: Kind,
    radians: Radians,
) -> ISQ {
    ISQ {
        m,
        kg,
        s,
        A: a,
        K: k,
        mol,
        cd,
        rad: rad_of(m, kg, s, a, k, mol, cd, radians),
        kind,
    }
}

// --- uom -> yadc-si -----------------------------------------------------------

impl<Ld, Md, Td, Id, Thd, Nd, Jd, Kd, U>
    From<Quantity<uom::si::ISQ<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>, U, f64>>
    for StaticSIQuantity<
        {
            isq_of(
                <Ld as Integer>::I8,
                <Md as Integer>::I8,
                <Td as Integer>::I8,
                <Id as Integer>::I8,
                <Thd as Integer>::I8,
                <Nd as Integer>::I8,
                <Jd as Integer>::I8,
                <Kd as UomKindMapping>::YADC_KIND,
                <Kd as UomKindMapping>::RADIANS,
            )
        },
    >
where
    Ld: Integer,
    Md: Integer,
    Td: Integer,
    Id: Integer,
    Thd: Integer,
    Nd: Integer,
    Jd: Integer,
    Kd: UomKindMapping + ?Sized,
    U: Units<f64> + ?Sized,
{
    fn from(q: Quantity<uom::si::ISQ<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>, U, f64>) -> Self {
        // This crate owns `StaticSIQuantity`, so it may build one directly:
        // generic over every uom dimension, where no unit constant applies.
        // Both store the coherent SI number, so it simply moves.
        StaticSIQuantity(q.value)
    }
}

// --- yadc-si -> uom -----------------------------------------------------------

impl<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>
    From<
        StaticSIQuantity<
            {
                isq_of(
                    <Ld as Integer>::I8,
                    <Md as Integer>::I8,
                    <Td as Integer>::I8,
                    <Id as Integer>::I8,
                    <Thd as Integer>::I8,
                    <Nd as Integer>::I8,
                    <Jd as Integer>::I8,
                    <Kd as UomKindMapping>::YADC_KIND,
                    <Kd as UomKindMapping>::RADIANS,
                )
            },
        >,
    > for Quantity<uom::si::ISQ<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>, uom::si::SI<f64>, f64>
where
    Ld: Integer,
    Md: Integer,
    Td: Integer,
    Id: Integer,
    Thd: Integer,
    Nd: Integer,
    Jd: Integer,
    Kd: UomKindMapping + ?Sized,
{
    fn from(
        sq: StaticSIQuantity<
            {
                isq_of(
                    <Ld as Integer>::I8,
                    <Md as Integer>::I8,
                    <Td as Integer>::I8,
                    <Id as Integer>::I8,
                    <Thd as Integer>::I8,
                    <Nd as Integer>::I8,
                    <Jd as Integer>::I8,
                    <Kd as UomKindMapping>::YADC_KIND,
                    <Kd as UomKindMapping>::RADIANS,
                )
            },
        >,
    ) -> Self {
        Quantity {
            value: sq.0,
            dimension: PhantomData,
            units: PhantomData,
        }
    }
}

// --- uom -> DynamicSIQuantity -------------------------------------------------

// No const-generic position involved, so the ISQ struct literal is fine here.
impl<Ld, Md, Td, Id, Thd, Nd, Jd, Kd, U>
    From<Quantity<uom::si::ISQ<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>, U, f64>> for DynamicSIQuantity
where
    Ld: Integer,
    Md: Integer,
    Td: Integer,
    Id: Integer,
    Thd: Integer,
    Nd: Integer,
    Jd: Integer,
    Kd: UomKindMapping + ?Sized,
    U: Units<f64> + ?Sized,
{
    fn from(q: Quantity<uom::si::ISQ<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>, U, f64>) -> Self {
        DynamicSIQuantity {
            value: q.value,
            dim: isq_of(
                Ld::I8,
                Md::I8,
                Td::I8,
                Id::I8,
                Thd::I8,
                Nd::I8,
                Jd::I8,
                <Kd as UomKindMapping>::YADC_KIND,
                <Kd as UomKindMapping>::RADIANS,
            ),
        }
    }
}

// --- DynamicSIQuantity -> uom -------------------------------------------------

// `TryFrom` because the runtime dim may not match the target uom type.
// Returns the original `DynamicSIQuantity` as the error so callers can recover it.
impl<Ld, Md, Td, Id, Thd, Nd, Jd, Kd> TryFrom<DynamicSIQuantity>
    for Quantity<uom::si::ISQ<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>, uom::si::SI<f64>, f64>
where
    Ld: Integer,
    Md: Integer,
    Td: Integer,
    Id: Integer,
    Thd: Integer,
    Nd: Integer,
    Jd: Integer,
    Kd: UomKindMapping + ?Sized,
{
    type Error = DynamicSIQuantity;

    fn try_from(dq: DynamicSIQuantity) -> Result<Self, DynamicSIQuantity> {
        let expected = isq_of(
            Ld::I8,
            Md::I8,
            Td::I8,
            Id::I8,
            Thd::I8,
            Nd::I8,
            Jd::I8,
            <Kd as UomKindMapping>::YADC_KIND,
            <Kd as UomKindMapping>::RADIANS,
        );
        if dq.dim == expected {
            Ok(Quantity {
                value: dq.value,
                dimension: PhantomData,
                units: PhantomData,
            })
        } else {
            Err(dq)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::angular::*;
    use crate::mechanical::*;
    use std::convert::TryFrom;
    use uom::si::angle::radian;
    use uom::si::f64::{Angle, Force, Length, Mass, SolidAngle, Velocity};
    use uom::si::force::newton;
    use uom::si::length::{kilometer, meter};
    use uom::si::mass::kilogram;
    use uom::si::solid_angle::steradian;
    use uom::si::velocity::meter_per_second;
    use yadc::Unit;

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
    }

    #[test]
    fn static_uom_to_yadc_preserves_si_base_value() {
        let q: StaticSIQuantity<_> = Length::new::<kilometer>(1.0).into();
        close(LengthUnit::Metre.value(q), 1_000.0);
    }

    #[test]
    fn static_yadc_to_uom_preserves_si_base_value() {
        let q = LengthUnit::Kilometre.quantity(2.5);
        let uom: Length = q.into();
        close(uom.get::<meter>(), 2_500.0);
    }

    #[test]
    fn static_round_trip_is_exact() {
        let original = Length::new::<meter>(1_234.5);
        let q: StaticSIQuantity<_> = original.into();
        let back: Length = q.into();
        close(back.get::<meter>(), 1_234.5);
    }

    #[test]
    fn static_derived_force_round_trip() {
        let original = Force::new::<newton>(9.81);
        let q: StaticSIQuantity<_> = original.into();
        let back: Force = q.into();
        close(back.get::<newton>(), 9.81);
    }

    #[test]
    fn static_angle_round_trip() {
        let original = Angle::new::<radian>(core::f64::consts::FRAC_PI_2);
        let q: StaticSIQuantity<_> = original.into();
        close(AngleUnit::Radian.value(q), core::f64::consts::FRAC_PI_2);
        let back: Angle = q.into();
        close(back.get::<radian>(), core::f64::consts::FRAC_PI_2);
    }

    #[test]
    fn static_solid_angle_round_trip() {
        let original = SolidAngle::new::<steradian>(2.5);
        let q: StaticSIQuantity<_> = original.into();
        close(SolidAngleUnit::Steradian.value(q), 2.5);
        let back: SolidAngle = q.into();
        close(back.get::<steradian>(), 2.5);
    }

    #[test]
    fn dynamic_from_uom_dim_is_correct() {
        let dq: DynamicSIQuantity = Length::new::<meter>(42.0).into();
        close(
            LengthUnit::Metre.value(crate::mechanical::Length::try_from(dq).unwrap()),
            42.0,
        );
        assert!(crate::mechanical::Mass::try_from(dq).is_err());
    }

    #[test]
    fn dynamic_angle_kind_is_preserved() {
        let dq: DynamicSIQuantity = Angle::new::<radian>(1.0).into();
        close(
            AngleUnit::Radian.value(crate::angular::Angle::try_from(dq).unwrap()),
            1.0,
        );
        // A plain dimensionless quantity has no radians: no match.
        assert!(crate::dimensionless::Dimensionless::try_from(dq).is_err());
    }

    #[test]
    fn dynamic_solid_angle_kind_is_preserved() {
        let dq: DynamicSIQuantity = SolidAngle::new::<steradian>(4.0).into();
        close(
            SolidAngleUnit::Steradian.value(crate::angular::SolidAngle::try_from(dq).unwrap()),
            4.0,
        );
        assert!(crate::angular::Angle::try_from(dq).is_err());
    }

    #[test]
    fn dynamic_try_from_success() {
        let dq: DynamicSIQuantity = Velocity::new::<meter_per_second>(10.0).into();
        let back = Velocity::try_from(dq).expect("dim matches");
        close(back.get::<meter_per_second>(), 10.0);
    }

    #[test]
    fn dynamic_try_from_failure_returns_original() {
        let dq: DynamicSIQuantity = Mass::new::<kilogram>(3.0).into();
        let err = Length::try_from(dq).unwrap_err();
        close(
            MassUnit::Kilogram.value(crate::mechanical::Mass::try_from(err).unwrap()),
            3.0,
        );
    }

    #[test]
    fn dynamic_try_from_angle_succeeds() {
        let dq: DynamicSIQuantity = Angle::new::<radian>(0.5).into();
        let back = Angle::try_from(dq).expect("angle dim matches");
        close(back.get::<radian>(), 0.5);
    }

    #[test]
    fn dynamic_try_from_angle_rejects_dimensionless() {
        // A plain dimensionless quantity (no radians) should not match uom's Angle.
        let dimensionless =
            DynamicSIQuantity::from(crate::dimensionless::DimensionlessUnit::Ratio.quantity(1.0));
        let err = Angle::try_from(dimensionless).unwrap_err();
        close(
            crate::dimensionless::DimensionlessUnit::Ratio
                .value(crate::dimensionless::Dimensionless::try_from(err).unwrap()),
            1.0,
        );
    }
}
