//! The dynamic type over a family with a *const* parameter, whose fact has
//! arithmetic of its own: lengths, areas and volumes.
//!
//! `Extent<const N: Dims>` is a size in `N` dimensions, in metres: a length
//! is 1-D, an area 2-D, a volume 3-D. The count is in the type, so
//! `Length * Length` is an `Area` and `Length + Area` does not compile.
//!
//! `AnyExtent` holds any of them, the count in a field. Lifting does the
//! same thing to every field with the field's own operator - so the count
//! needs operators that say what a product and a sum do to it: multiplying
//! sizes adds the counts (a 1-D width times a 2-D floor is a 3-D room);
//! adding sizes needs equal counts. That is `Dims`'s own `*` and `+`,
//! written once, below.
#![feature(
    adt_const_params,
    const_trait_impl,
    const_cmp,
    const_ops,
    const_convert
)]
#![feature(const_destruct, derive_const)]
#![allow(incomplete_features)]

use core::marker::ConstParamTy;
use core::ops::{Add, Mul};

/// How many dimensions a size has: 1 for a length, 2 for an area, 3 for a
/// volume.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Dims(pub i8);

/// A product of sizes: the counts add (1-D × 2-D = 3-D).
const impl Mul for Dims {
    type Output = Dims;
    fn mul(self, other: Dims) -> Dims {
        Dims(self.0 + other.0)
    }
}

/// A sum of sizes: only of the same count.
const impl Add for Dims {
    type Output = Option<Dims>;
    fn add(self, other: Dims) -> Option<Dims> {
        if self == other { Some(self) } else { None }
    }
}

/// The family: one static type per count.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Extent<const N: Dims>(pub f64);

pub type Length = Extent<{ Dims(1) }>;
pub type Area = Extent<{ Dims(2) }>;
pub type Volume = Extent<{ Dims(3) }>;

#[derive(Copy, Clone, Debug, PartialEq, dynamickit::Dynamic)]
#[dynamic(Extent<const N: Dims> => { dims: N, metres: .0 })]
#[dynamic(impl(Mul -> Self, Add -> Option<Self>))]
pub struct AnyExtent {
    pub dims: Dims,
    pub metres: f64,
}

// What the derive writes, by hand:
//
// impl<const N: Dims> From<Extent<N>> for AnyExtent {
//     fn from(e: Extent<N>) -> Self { AnyExtent { dims: N, metres: e.0 } }
// }
// impl<const N: Dims> TryFrom<AnyExtent> for Extent<N> {
//     type Error = AnyExtent;
//     fn try_from(a: AnyExtent) -> Result<Self, AnyExtent> {
//         if a.dims == N { Ok(Extent(a.metres)) } else { Err(a) }
//     }
// }
// impl Mul for AnyExtent {
//     type Output = AnyExtent;
//     fn mul(self, other: AnyExtent) -> AnyExtent {
//         AnyExtent { dims: self.dims * other.dims, metres: self.metres * other.metres }
//     }
// }
// impl Add for AnyExtent {
//     type Output = Option<AnyExtent>;
//     fn add(self, other: AnyExtent) -> Option<AnyExtent> {
//         Some(AnyExtent { dims: (self.dims + other.dims)?, metres: self.metres + other.metres })
//     }
// }

/// The operators are `const`.
const FLOOR: AnyExtent = {
    let (width, depth): (Length, Length) = (Extent(4.0), Extent(2.5));
    AnyExtent::from(width) * AnyExtent::from(depth)
};

fn main() {
    let height = AnyExtent::from(Extent::<{ Dims(1) }>(3.0));
    println!("floor         = {FLOOR:?}");
    println!("floor × height = {:?}", FLOOR * height);
    println!("floor + floor  = {:?}", FLOOR + FLOOR);
    println!("floor + height = {:?}", FLOOR + height);

    // Back to a static type: checked.
    println!("as Area:   {:?}", Area::try_from(FLOOR));
    println!("as Volume: {:?}", Volume::try_from(FLOOR * height));
    println!("as Length: {:?}", Length::try_from(FLOOR).is_ok());
}
