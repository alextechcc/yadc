//! Vectors of quantities, turned about an axis, with `yadc-si`'s units.
//!
//! Nothing in yadc knows about 3-D: the vector type is this file's own.
//! `Vec3<Q>` is three quantities of one type, and its operators are its
//! parts' (`Vec3<A> * B` is a `Vec3<A * B>`), so the units follow: a
//! velocity vector times a time is a displacement vector, because a
//! velocity times a time is a length.
//!
//! `cos` and `sin` want a plain number, and an angle is not one: the radian
//! is a base unit with its own exponent, like the metre. An angle over a
//! radian is the plain number.
//!
//! Run with: cargo run -p yadc --example rotations
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::ops::{Add, Div, Mul, Sub};

use yadc_si::angular::*;
use yadc_si::mechanical::*;
use yadc_si::{METRE, RADIAN, SECOND};

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

/// Each part plus the other's: whatever `A + B` is.
impl<A: Add<B>, B> Add<Vec3<B>> for Vec3<A> {
    type Output = Vec3<A::Output>;
    fn add(self, r: Vec3<B>) -> Vec3<A::Output> {
        Vec3::new(self.x + r.x, self.y + r.y, self.z + r.z)
    }
}

impl<A: Sub<B>, B> Sub<Vec3<B>> for Vec3<A> {
    type Output = Vec3<A::Output>;
    fn sub(self, r: Vec3<B>) -> Vec3<A::Output> {
        Vec3::new(self.x - r.x, self.y - r.y, self.z - r.z)
    }
}

/// Each part times `k`: a scaling for a number `k`, a change of unit for a
/// quantity (`Velocity * Time` is a `Length`).
impl<A: Mul<B>, B: Copy> Mul<B> for Vec3<A> {
    type Output = Vec3<A::Output>;
    fn mul(self, k: B) -> Vec3<A::Output> {
        Vec3::new(self.x * k, self.y * k, self.z * k)
    }
}

impl<A: Div<B>, B: Copy> Div<B> for Vec3<A> {
    type Output = Vec3<A::Output>;
    fn div(self, k: B) -> Vec3<A::Output> {
        Vec3::new(self.x / k, self.y / k, self.z / k)
    }
}

impl<Q: Copy> Vec3<Q> {
    /// The parts as numbers of `unit`.
    fn numbers<R>(self, unit: Q) -> [f64; 3]
    where
        Q: Div<Output = R>,
        f64: From<R>,
    {
        [self.x, self.y, self.z].map(|part| f64::from(part / unit))
    }

    /// How long the vector is, in `Q`.
    fn norm<R>(self, unit: Q) -> Q
    where
        Q: Div<Output = R> + Mul<f64, Output = Q>,
        f64: From<R>,
    {
        let [x, y, z] = self.numbers(unit);
        unit * (x * x + y * y + z * z).sqrt()
    }

    /// Turned by `angle` about the z axis, anticlockwise seen from above.
    fn rotated_z(self, angle: Angle<f64>) -> Self
    where
        Q: Mul<f64, Output = Q> + Add<Output = Q> + Sub<Output = Q>,
    {
        let (sin, cos) = f64::from(angle / (1.0 * RADIAN)).sin_cos();
        Vec3::new(
            self.x * cos - self.y * sin,
            self.x * sin + self.y * cos,
            self.z,
        )
    }
}

fn show(label: &str, [x, y, z]: [f64; 3]) {
    println!("{label}: ({x:.3}, {y:.3}, {z:.3})");
}

fn main() {
    let metre: Length<f64> = 1.0 * METRE;
    let degree: Angle<f64> = (1.0_f64 * DEGREE).into();

    // A sensor 2 m ahead, 0.5 m left and 1 m up on a vehicle facing 30° left
    // of the world's x axis: turned by the heading, it is in world axes.
    let heading = 30.0 * degree;
    let on_vehicle: Vec3<Length<f64>> = Vec3::new(2.0 * metre, 0.5 * metre, 1.0 * metre);
    let in_world = on_vehicle.rotated_z(heading);
    show("sensor, vehicle axes (m)", on_vehicle.numbers(metre));
    show("sensor, world axes (m)  ", in_world.numbers(metre));
    // Turning keeps the length; turning back gives the vector back.
    println!(
        "reach: {:.4} = {:.4}",
        on_vehicle.norm(metre),
        in_world.norm(metre)
    );
    let back = in_world.rotated_z(-heading);
    assert!((back - on_vehicle).norm(metre) < 1e-9 * metre);

    // Driving forward at 3 m/s for 4 s: a velocity vector, turned into the
    // world and multiplied by a time, is a displacement vector.
    let zero: Velocity<f64> = 0.0 * METRE / SECOND;
    let forward = Vec3::new(3.0 * METRE / SECOND, zero, zero);
    let moved: Vec3<Length<f64>> = forward.rotated_z(heading) * (4.0 * SECOND);
    show("moved in 4 s (m)        ", moved.numbers(metre));
    show("half of that (m)        ", (moved * 0.5).numbers(metre));

    // An angular velocity times a time is an angle.
    let yaw_rate: AngularVelocity<f64> = 15.0 * degree / (1.0 * SECOND);
    let turned: Angle<f64> = yaw_rate * (4.0 * SECOND);
    println!(
        "turned {:.0}° in 4 s; heading now {:.4}",
        f64::from(turned / degree),
        heading + turned
    );
}
