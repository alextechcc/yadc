//! Rotations about a known reference frame, with real units.
//!
//! The question: can we place things in 3-D, turn them about an axis, and move
//! between "where it is on the vehicle" and "where it is in the world" - with
//! every number still carrying its unit? Yes, and nothing in yadc needs to
//! know about 3-D for it. You write the small vector type yourself; yadc's
//! quantities supply the units.
//!
//! The pieces, in plain words:
//!
//! - **A vector** (`Vec3<Q>`) is three quantities of the same type: an x, a y
//!   and a z. A displacement is three lengths, a velocity three speeds. Adding
//!   two vectors adds each part; scaling one scales each part. Multiplying a
//!   vector by a quantity multiplies each part, so the result's unit follows:
//!   a velocity vector times a time is a displacement vector - the compiler
//!   works out `Vec3<Velocity> * Time = Vec3<Length>` from `Velocity * Time =
//!   Length`.
//! - **A place** is not an amount: like an altitude (see the `points`
//!   example) its zero - the *origin* - is a choice. Each of its three
//!   coordinates is a point: a length from the survey marker, whose dimension
//!   is of the affine kind. A coordinate minus a coordinate is a length, a
//!   coordinate plus a length is a coordinate, and nothing else.
//!   `Vec3`'s `+` and `-` do whatever their parts' do, so a place
//!   (`Vec3<Coordinate>`) minus a place is a displacement (`Vec3<Length>`),
//!   and a displacement moves a place - with no code about points at all.
//! - **The origin is a constant**, `SURVEY_MARKER`, and reading a place as
//!   numbers is choosing a *unit*. A unit whose number is three numbers is a
//!   `Unit<Place, [f64; 3]>` - a coordinate chart, which is all a unit is. A
//!   grid is one: an origin and a spacing, reading a place as how many
//!   spacings east, north and up it is from the origin.
//! - **A frame is another unit of the same places**: the vehicle's own axes
//!   (x forward, y left, z up) sit at the vehicle, turned by its heading.
//!   Building a place from body coordinates turns them by the heading and
//!   moves them to the vehicle; reading one undoes both. "Body" and "world"
//!   are not two kinds of place, just two ways of writing one down.
//!
//! The constants and the derived units are called like functions -
//! `DEGREE(30.0)` builds, `METRE(reach)` reads. The grid and the frame are
//! units written by hand, so they are their two methods:
//! `SITE_GRID.quantity([10.0, 5.0, 0.0])`, `vehicle.value(sensor)`.
//!
//! **Angles and radians.** `cos` and `sin` want a plain number, and an angle
//! is not one here: the radian is a unit with its own exponent, like the
//! metre. Reading an angle in radians gives the plain number
//! (`RADIAN(theta)`).
//!
//! Run with: cargo run -p yadc --example rotations

// This file defines its own kind, dimension and quantity type, so it needs
// the definer's feature set.
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(fn_traits, unboxed_closures, const_destruct)]
#![feature(
    const_cmp,
    const_default,
    derive_const,
    const_ops,
    const_convert,
    const_try
)]
#![allow(incomplete_features)]

use core::f64::consts::PI;
use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

use yadc::{Dimension, Root, RootAs, Unit};

// --- The system ---------------------------------------------------------------------

/// How a quantity behaves: an amount, or a point - a coordinate on the site,
/// counted from a zero somebody chose.
#[derive_const(PartialEq, Default)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub enum Kind {
    #[default]
    Linear,
    #[kind(affine)]
    Affine,
}

/// Metres, seconds and radians, and the kind.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
    pub rad: i8,
    #[kind]
    pub kind: Kind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
pub struct Qty<const D: Dim>(f64);

pub type Length = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
/// A position along one axis of the site: a length from the survey marker.
pub type Coordinate = Qty<
    {
        Dim {
            m: 1,
            kind: Kind::Affine,
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
pub type Velocity = Qty<
    {
        Dim {
            m: 1,
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type Angle = Qty<
    {
        Dim {
            rad: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
pub type AngularVelocity = Qty<
    {
        Dim {
            s: -1,
            rad: 1,
            ..Dim::DIMENSIONLESS
        }
    },
>;
// --- Constants: the base ones, and the rest from them ---------------------------

pub const METRE: Length = Qty(1.0);
pub const SECOND: Time = Qty(1.0);
pub const RADIAN: Angle = Qty(1.0);
/// The survey marker's coordinate, on every axis: the zero of the line of
/// coordinates.
pub const MARKER: Coordinate = Qty(0.0);

pub const NANOMETRE: Length = 1e-9 * METRE;
pub const METRE_PER_SECOND: Velocity = METRE / SECOND;
pub const DEGREE: Angle = (PI / 180.0) * RADIAN;
pub const DEGREE_PER_SECOND: AngularVelocity = DEGREE / SECOND;

// Unit types, for printing: each names one of the values above, with
// its labels.

#[derive(Copy, Clone, yadc::Unit)]
#[unit("m", "metres", of = Length, by = METRE)]
struct Metre;

#[derive(Copy, Clone, yadc::Unit)]
#[unit("°", "degrees", of = Angle, by = DEGREE)]
struct Degree;

#[derive(Copy, Clone, yadc::Unit)]
#[unit("rad", "radians", of = Angle, by = RADIAN)]
struct Radian;
// --- A vector of any quantity -----------------------------------------------------

/// Three quantities of one type: x, y and z.
#[derive(Copy, Clone, PartialEq, Debug)]
struct Vec3<Q> {
    x: Q,
    y: Q,
    z: Q,
}

impl<Q> Vec3<Q> {
    const fn new(x: Q, y: Q, z: Q) -> Self {
        Self { x, y, z }
    }
}

/// Each part plus the other's: the result is whatever `A + B` is - a length
/// for two lengths, a coordinate for a coordinate plus a length, and nothing
/// for two coordinates.
impl<A: Add<B>, B> Add<Vec3<B>> for Vec3<A> {
    type Output = Vec3<A::Output>;
    fn add(self, r: Vec3<B>) -> Vec3<A::Output> {
        Vec3::new(self.x + r.x, self.y + r.y, self.z + r.z)
    }
}

/// Each part minus the other's, the same way: two coordinates differ by a
/// length, so two places differ by a displacement.
impl<A: Sub<B>, B> Sub<Vec3<B>> for Vec3<A> {
    type Output = Vec3<A::Output>;
    fn sub(self, r: Vec3<B>) -> Vec3<A::Output> {
        Vec3::new(self.x - r.x, self.y - r.y, self.z - r.z)
    }
}

/// Each part times `k`: the result's type is whatever `A * B` is. With `B` a
/// plain `f64` this is scaling (`Length * f64` is a `Length`); with `B` a
/// quantity the unit changes (`Velocity * Time` is a `Length`).
impl<A: Mul<B>, B: Copy> Mul<B> for Vec3<A> {
    type Output = Vec3<A::Output>;
    fn mul(self, k: B) -> Vec3<A::Output> {
        Vec3::new(self.x * k, self.y * k, self.z * k)
    }
}

/// Each part divided by `k`, the same way.
impl<A: Div<B>, B: Copy> Div<B> for Vec3<A> {
    type Output = Vec3<A::Output>;
    fn div(self, k: B) -> Vec3<A::Output> {
        Vec3::new(self.x / k, self.y / k, self.z / k)
    }
}

impl<Q: Copy> Vec3<Q> {
    /// How long the vector is: the square root of x² + y² + z², in `Q`'s own
    /// unit (a length's squares are an area, whose root is a length again).
    fn norm<S>(self) -> Q
    where
        Q: Mul<Output = S>,
        S: Add<Output = S> + Root<2, Output = Q>,
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).root::<2>()
    }

    /// Turned by `angle` about the z axis (anticlockwise seen from above).
    /// Only an amount turns: a place has no `* f64`.
    fn rotated_z(self, angle: Angle) -> Self
    where
        Q: Mul<f64, Output = Q> + Add<Output = Q> + Sub<Output = Q>,
    {
        // `cos` and `sin` want a plain number: the angle counted in radians.
        let (sin, cos) = RADIAN(angle).sin_cos();
        Vec3::new(
            self.x * cos - self.y * sin,
            self.x * sin + self.y * cos,
            self.z,
        )
    }
}

// --- Places, and the units that read them -----------------------------------------

/// A place on the site: three coordinates, each a point.
type Place = Vec3<Coordinate>;

/// The origin: the survey marker every map of the site is drawn from.
const SURVEY_MARKER: Place = Vec3::new(MARKER, MARKER, MARKER);

/// A grid: an origin and a spacing. Its numbers are three numbers - how many
/// spacings east, north and up a place is from the origin.
#[derive(Copy, Clone, Debug)]
struct Grid {
    origin: Place,
    spacing: Length,
}

impl Unit<Place, [f64; 3]> for Grid {
    fn quantity(&self, [x, y, z]: [f64; 3]) -> Place {
        self.origin + Vec3::new(x, y, z) * self.spacing
    }
    fn value(&self, p: Place) -> [f64; 3] {
        let d = p - self.origin;
        // The spacing is a length, and every length is a unit of lengths.
        [d.x, d.y, d.z].map(|part| self.spacing.value(part))
    }
}

/// The site's survey grid: metres east, north and up from the marker.
const SITE_GRID: Grid = Grid {
    origin: SURVEY_MARKER,
    spacing: METRE,
};

/// A displacement's three numbers, in metres.
fn metres(d: Vec3<Length>) -> [f64; 3] {
    [d.x, d.y, d.z].map(|part| METRE(part))
}

/// The vehicle's own axes: where it is and which way it faces. Another unit
/// of the same places - metres forward, left and up from the vehicle.
#[derive(Copy, Clone, Debug)]
struct Frame {
    origin: Place,
    heading: Angle,
}

impl Unit<Place, [f64; 3]> for Frame {
    /// Turn by the heading, then move to the origin.
    fn quantity(&self, [x, y, z]: [f64; 3]) -> Place {
        self.origin + (Vec3::new(x, y, z) * METRE).rotated_z(self.heading)
    }
    /// Undo both: move back from the origin, then turn by minus the heading.
    fn value(&self, p: Place) -> [f64; 3] {
        // Naming the difference's type lets the compiler work it out before
        // `rotated_z` asks what it can do.
        let d: Vec3<Length> = p - self.origin;
        metres(d.rotated_z(-self.heading))
    }
}

impl Frame {
    /// A direction (a velocity, a force) only turns: it has no position.
    fn direction_to_world<Q>(self, v: Vec3<Q>) -> Vec3<Q>
    where
        Q: Copy + Mul<f64, Output = Q> + Add<Output = Q> + Sub<Output = Q>,
    {
        v.rotated_z(self.heading)
    }
}

/// Generic code over any point: halfway between two of them. `P` is anything
/// whose difference `L` can be halved and moves it back - a coordinate and a
/// length, a place and a displacement.
fn midpoint<P, L>(a: P, b: P) -> P
where
    P: Copy + Sub<Output = L> + Add<L, Output = P>,
    L: Mul<f64, Output = L>,
{
    a + (b - a) * 0.5
}

fn show(label: &str, [x, y, z]: [f64; 3]) {
    println!("{label}: ({x:.3}, {y:.3}, {z:.3})");
}

fn main() {
    // --- 1. Body -> world and back --------------------------------------------
    // The vehicle stands at (10, 5, 0) m on the site grid, facing 30° to the
    // left of the grid's x axis. A sensor is mounted 2 m ahead, 0.5 m to the
    // left and 1 m up on the vehicle: three numbers in the vehicle's frame.
    let vehicle = Frame {
        origin: SITE_GRID.quantity([10.0, 5.0, 0.0]),
        heading: DEGREE(30.0),
    };
    let sensor: Place = vehicle.quantity([2.0, 0.5, 1.0]);
    show("sensor, on the vehicle (m)", vehicle.value(sensor));
    show("sensor, on the grid (m)   ", SITE_GRID.value(sensor));

    // Back and forth through the vehicle's frame is the same place.
    let back = vehicle.quantity(vehicle.value(sensor));
    let error: Length = (back - sensor).norm();
    assert!(error < NANOMETRE(1.0));
    println!("round trip error: {:.1e} {Metre}", Metre(error));

    // Turning never changes a distance: place - place is a displacement,
    // whichever unit built the places.
    let reach: Length = (sensor - vehicle.origin).norm();
    let [x, y, z] = vehicle.value(sensor);
    let reach_on_vehicle = (x * x + y * y + z * z).sqrt();
    assert!((METRE(reach) - reach_on_vehicle).abs() < 1e-9);
    println!("sensor reach: {:.4} {Metre}", Metre(reach));

    // A landmark on the grid, seen from the vehicle - and on a coarser grid
    // of 5 m cells whose corner is at (-5, -5, 0) m: the same place, other
    // numbers.
    let tree: Place = SITE_GRID.quantity([20.0, 15.0, 0.0]);
    let cells = Grid {
        origin: SITE_GRID.quantity([-5.0, -5.0, 0.0]),
        spacing: 5.0 * METRE,
    };
    show("tree, from the vehicle (m)", vehicle.value(tree));
    show("tree, in 5 m cells        ", cells.value(tree));
    // Halfway between the vehicle and the tree, in either unit.
    let halfway = midpoint(vehicle.origin, tree);
    show("halfway, on the grid (m)  ", SITE_GRID.value(halfway));

    // --- 2. Vectors carry units through products --------------------------------
    // Driving forward at 3 m/s for 4 s: the velocity is along the body's x
    // axis; turned into the world and multiplied by a time, it is a
    // displacement.
    let zero = METRE_PER_SECOND(0.0);
    let forward: Vec3<Velocity> = Vec3::new(METRE_PER_SECOND(3.0), zero, zero);
    let moved: Vec3<Length> = vehicle.direction_to_world(forward) * SECOND(4.0);
    show("moved in 4 s (m)          ", metres(moved));
    let half: Vec3<Length> = moved * 0.5;
    show("half of that (m)          ", metres(half));

    // --- 3. Turning: angular speed × time is an angle ---------------------------
    let yaw_rate: AngularVelocity = DEGREE_PER_SECOND(15.0);
    let turned: Angle = yaw_rate * SECOND(4.0);
    let after = Frame {
        origin: vehicle.origin + moved,
        heading: vehicle.heading + turned,
    };
    println!(
        "turned {:.0} {Degree} in 4 s; heading now {:.0} {Degree} = {:.4} {Radian}",
        Degree(turned),
        Degree(after.heading),
        Radian(after.heading),
    );
    // The sensor rode along: the same body coordinates, read in the new frame.
    let sensor_now = after.quantity([2.0, 0.5, 1.0]);
    show("sensor, on the grid (m)   ", SITE_GRID.value(sensor_now));

    // --- 4. What does not compile -------------------------------------------------
    // A place is made of points: two of them differ by a displacement, and
    // that is all. Adding or scaling places would depend on where the marker
    // was put:
    //
    // let _: Place = sensor + tree;
    //     error: cannot add a point to a point
    // let _ = sensor * 2.0;
    //     error: cannot multiply `Qty<{ .. kind: Kind::Affine .. }>` by `f64`
    // (`Vec3`'s `*` is this file's own, so the message is rustc's plain
    // one, with no reason; its `help:` line names the trait the coordinate
    // lacks, `ScaleNotPoint`.)
    let _ = tree - sensor; // a displacement: fine
}
