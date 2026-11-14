//! Blanket [`From`] / [`Into`] / [`TryFrom`] impls bridging yadc-si's
//! const-generic [`SIUnit`] (a quantity's coherent unit) and uom's
//! typenum-based ISQ dimension system.
//!
//! Each typenum exponent type (`P1`, `Z0`, `N1`, ...) carries a `const I8: i8`
//! via [`typenum::Integer`], so the const-generic unit can be computed
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

use crate::dynamic::DynamicSIQuantity;
use crate::{
    AMPERE, CANDELA, KELVIN, KILOGRAM, METRE, MOLE, RADIAN, SECOND, SIQuantity, SIUnit,
};

// --- Kind mapping -------------------------------------------------------------

/// Says how a uom kind marker (as a `dyn` trait object type) stands for
/// radians.
///
/// uom follows the SI: angles are dimensionless, told apart by a kind. yadc-si
/// counts radians on an exponent axis instead, so a uom angle kind becomes a
/// `rad` exponent - which one depends on the quantity (see [`rad_of`]).
pub trait UomKindMapping {
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

impl UomKindMapping for dyn uom::Kind {}
impl UomKindMapping for dyn marker::AngleKind {
    const RADIANS: Radians = Radians::Angle;
}
impl UomKindMapping for dyn marker::SolidAngleKind {
    const RADIANS: Radians = Radians::Squared;
}
impl UomKindMapping for dyn marker::TemperatureKind {}
impl UomKindMapping for dyn marker::InformationKind {}
impl UomKindMapping for dyn marker::ConstituentConcentrationKind {}
impl UomKindMapping for dyn marker::SurfaceTensionKind {}
impl UomKindMapping for dyn marker::KinematicViscosityKind {}
impl UomKindMapping for dyn marker::IlluminanceKind {
    const RADIANS: Radians = Radians::Squared;
}

// --- Const-generic helper -----------------------------------------------------

/// The coherent unit of these exponents: the product of the base units.
#[allow(clippy::too_many_arguments)]
const fn unit_of(
    m: i8,
    kg: i8,
    s: i8,
    a: i8,
    k: i8,
    mol: i8,
    cd: i8,
    radians: Radians,
) -> SIUnit {
    let parts = [
        (METRE.unit(), m),
        (KILOGRAM.unit(), kg),
        (SECOND.unit(), s),
        (AMPERE.unit(), a),
        (KELVIN.unit(), k),
        (MOLE.unit(), mol),
        (CANDELA.unit(), cd),
        (RADIAN.unit(), rad_of(m, kg, s, a, k, mol, cd, radians)),
    ];
    let mut u = SIUnit::ONE;
    let mut i = 0;
    while i < parts.len() {
        if parts[i].1 != 0 {
            u = u.times(parts[i].0, parts[i].1);
        }
        i += 1;
    }
    u
}

// --- uom -> yadc-si -----------------------------------------------------------

impl<Ld, Md, Td, Id, Thd, Nd, Jd, Kd, U>
    From<Quantity<uom::si::ISQ<Ld, Md, Td, Id, Thd, Nd, Jd, Kd>, U, f64>>
    for SIQuantity<
        {
            unit_of(
                <Ld as Integer>::I8,
                <Md as Integer>::I8,
                <Td as Integer>::I8,
                <Id as Integer>::I8,
                <Thd as Integer>::I8,
                <Nd as Integer>::I8,
                <Jd as Integer>::I8,
                <Kd as UomKindMapping>::RADIANS,
            )
        },
        f64,
        true,
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
                unit_of(
                    <Ld as Integer>::I8,
                    <Md as Integer>::I8,
                    <Td as Integer>::I8,
                    <Id as Integer>::I8,
                    <Thd as Integer>::I8,
                    <Nd as Integer>::I8,
                    <Jd as Integer>::I8,
                    <Kd as UomKindMapping>::RADIANS,
                )
            },
            f64,
            true,
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
                unit_of(
                    <Ld as Integer>::I8,
                    <Md as Integer>::I8,
                    <Td as Integer>::I8,
                    <Id as Integer>::I8,
                    <Thd as Integer>::I8,
                    <Nd as Integer>::I8,
                    <Jd as Integer>::I8,
                    <Kd as UomKindMapping>::RADIANS,
                )
            },
            f64,
            true,
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

// No const-generic position involved: the unit is computed at run time.
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
            is_base: true,
            value: q.value,
            unit: unit_of(
                Ld::I8,
                Md::I8,
                Td::I8,
                Id::I8,
                Thd::I8,
                Nd::I8,
                Jd::I8,
                <Kd as UomKindMapping>::RADIANS,
            ),
        }
    }
}

// --- DynamicSIQuantity -> uom -------------------------------------------------

// `TryFrom` because the run-time unit may not be the target uom type's.
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
        let expected = unit_of(
            Ld::I8,
            Md::I8,
            Td::I8,
            Id::I8,
            Thd::I8,
            Nd::I8,
            Jd::I8,
            <Kd as UomKindMapping>::RADIANS,
        );
        if dq.unit == expected && dq.is_base {
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
