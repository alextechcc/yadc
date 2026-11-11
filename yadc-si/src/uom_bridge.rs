//! Blanket [`From`] / [`Into`] / [`TryFrom`] impls bridging yadc-si's
//! const-generic [`ISQ`] and uom's typenum-based ISQ dimension system.
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

use crate::{DynamicSIQuantity, ISQ, Kind, SIQuantity};

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
    /// uom's angle kind: rad¹ on a quantity with no mass (an angle, its
    /// rates, its absement, curvature), rad⁻¹ on one with mass (torque and
    /// angular momentum, which uom marks with the angle kind to tell them
    /// from energy and action).
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
            if kg == 0 {
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
    for SIQuantity<
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
        // This crate owns `SIQuantity`, so it may build one directly:
        // generic over every uom dimension, where no unit constant applies.
        // Both store the coherent SI number, so it simply moves.
        SIQuantity(q.value)
    }
}

// --- yadc-si -> uom -----------------------------------------------------------

impl<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>
    From<
        SIQuantity<
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
        sq: SIQuantity<
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
