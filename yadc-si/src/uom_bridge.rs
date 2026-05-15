//! Blanket [`From`] / [`Into`] impls bridging yadc-si's const-generic [`ISQ`]
//! and uom's typenum-based ISQ dimension system.
//!
//! Each typenum exponent type (`P1`, `Z0`, `N1`, …) carries a `const I8: i8`
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
//! | `uom → yadc-si` | any (`U: Units<f64>`) — value is already SI-base |
//! | `yadc-si → uom` | [`uom::si::SI<f64>`] (standard SI) |
//!
//! `Into` impls are provided for free by the standard blanket
//! `impl<T, U: From<T>> Into<U> for T`.

use core::convert::TryFrom;
use core::marker::PhantomData;
use typenum::Integer;
use uom::si::{Quantity, Units, marker};

use crate::{DynamicSIQuantity, ISQ, Kind, StaticSIQuantity};

// ─── Kind mapping ─────────────────────────────────────────────────────────────

/// Maps a uom kind marker (as a `dyn` trait object type) to the corresponding
/// yadc [`Kind`] variant.
pub trait UomKindMapping {
    const YADC_KIND: Kind;
}

impl UomKindMapping for dyn uom::Kind {
    const YADC_KIND: Kind = Kind::Linear;
}
impl UomKindMapping for dyn marker::AngleKind {
    const YADC_KIND: Kind = Kind::Angle;
}
impl UomKindMapping for dyn marker::SolidAngleKind {
    const YADC_KIND: Kind = Kind::SolidAngle;
}
impl UomKindMapping for dyn marker::TemperatureKind {
    const YADC_KIND: Kind = Kind::Affine;
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
}

// ─── Const-generic helper ─────────────────────────────────────────────────────

// Direct struct literals are not yet supported in generic-const positions
// (rustc: "overly complex generic constant"), but const-fn calls are.
#[allow(non_snake_case)]
const fn isq_of(m: i8, kg: i8, s: i8, a: i8, k: i8, mol: i8, cd: i8, kind: Kind) -> ISQ {
    ISQ {
        m,
        kg,
        s,
        A: a,
        K: k,
        mol,
        cd,
        kind,
    }
}

// ─── uom → yadc-si ───────────────────────────────────────────────────────────

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
        StaticSIQuantity(q.value)
    }
}

// ─── yadc-si → uom ───────────────────────────────────────────────────────────

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

// ─── uom → DynamicSIQuantity ─────────────────────────────────────────────────

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
            dim: ISQ {
                m: Ld::I8,
                kg: Md::I8,
                s: Td::I8,
                A: Id::I8,
                K: Thd::I8,
                mol: Nd::I8,
                cd: Jd::I8,
                kind: <Kd as UomKindMapping>::YADC_KIND,
            },
        }
    }
}

// ─── DynamicSIQuantity → uom ─────────────────────────────────────────────────

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
        let expected = ISQ {
            m: Ld::I8,
            kg: Md::I8,
            s: Td::I8,
            A: Id::I8,
            K: Thd::I8,
            mol: Nd::I8,
            cd: Jd::I8,
            kind: <Kd as UomKindMapping>::YADC_KIND,
        };
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
    use yadc::StaticQuantity;

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "expected {b}, got {a}");
    }

    #[test]
    fn static_uom_to_yadc_preserves_si_base_value() {
        let q: StaticSIQuantity<_> = Length::new::<kilometer>(1.0).into();
        close(q.as_unit(LengthUnit::Meter), 1_000.0);
    }

    #[test]
    fn static_yadc_to_uom_preserves_si_base_value() {
        let q = crate::mechanical::Length::from_unit(2.5, LengthUnit::Kilometer);
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
        let original = Angle::new::<radian>(1.5707963);
        let q: StaticSIQuantity<_> = original.into();
        close(q.as_unit(AngleUnit::Radian), 1.5707963);
        let back: Angle = q.into();
        close(back.get::<radian>(), 1.5707963);
    }

    #[test]
    fn static_solid_angle_round_trip() {
        let original = SolidAngle::new::<steradian>(2.5);
        let q: StaticSIQuantity<_> = original.into();
        close(q.as_unit(SolidAngleUnit::Steradian), 2.5);
        let back: SolidAngle = q.into();
        close(back.get::<steradian>(), 2.5);
    }

    #[test]
    fn dynamic_from_uom_dim_is_correct() {
        use yadc::DynamicQuantity;
        let dq: DynamicSIQuantity = Length::new::<meter>(42.0).into();
        assert_eq!(dq.as_unit(LengthUnit::Meter), Some(42.0));
        assert_eq!(dq.as_unit(MassUnit::Kilogram), None);
    }

    #[test]
    fn dynamic_angle_kind_is_preserved() {
        use yadc::DynamicQuantity;
        let dq: DynamicSIQuantity = Angle::new::<radian>(1.0).into();
        assert_eq!(dq.as_unit(AngleUnit::Radian), Some(1.0));
        // Dimensionless (Kind::Linear) does not match Kind::Angle
        assert_eq!(
            dq.as_unit(crate::dimensionless::DimensionlessUnit::Ratio),
            None
        );
    }

    #[test]
    fn dynamic_solid_angle_kind_is_preserved() {
        use yadc::DynamicQuantity;
        let dq: DynamicSIQuantity = SolidAngle::new::<steradian>(4.0).into();
        assert_eq!(dq.as_unit(SolidAngleUnit::Steradian), Some(4.0));
        assert_eq!(dq.as_unit(AngleUnit::Radian), None);
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
        use yadc::DynamicQuantity;
        assert_eq!(err.as_unit(MassUnit::Kilogram), Some(3.0));
    }

    #[test]
    fn dynamic_try_from_angle_succeeds() {
        let dq: DynamicSIQuantity = Angle::new::<radian>(0.5).into();
        let back = Angle::try_from(dq).expect("angle dim matches");
        close(back.get::<radian>(), 0.5);
    }

    #[test]
    fn dynamic_try_from_angle_rejects_dimensionless() {
        use yadc::DynamicQuantity;
        // A dimensionless (Kind::Linear) quantity should not match the Angle (Kind::Angle) uom type.
        let dimensionless =
            DynamicSIQuantity::from_unit(1.0, crate::dimensionless::DimensionlessUnit::Ratio);
        let err = Angle::try_from(dimensionless).unwrap_err();
        assert_eq!(
            err.as_unit(crate::dimensionless::DimensionlessUnit::Ratio),
            Some(1.0)
        );
    }
}
