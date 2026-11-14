//! Functions generic over quantities, bounded on std operators only.
//!
//! A quantity's operators are std's, so a generic function says what its
//! body does with them (`Q: Add<Output = Q>`, `T: Sub<Output = I>`) and
//! nothing else. The same body takes any system's quantities: `main` calls
//! each function with `yadc-si` and with `nav`, a system written in this
//! file.
//!
//! Run with: cargo run -p yadc --example generic_fns
#![feature(
    generic_const_exprs,
    adt_const_params,
    const_trait_impl,
    const_ops,
    const_cmp
)]
#![feature(derive_const, const_destruct, const_convert)]
#![allow(incomplete_features)]

use std::ops::{Add, Div, Sub};

// --- Generic functions --------------------------------------------------------

/// The average of two values of one quantity.
fn average<Q>(a: Q, b: Q) -> Q
where
    Q: Add<Output = Q> + Div<f64, Output = Q>,
{
    (a + b) / 2.0
}

/// The mean of some values, `None` for none.
fn mean<Q>(values: &[Q]) -> Option<Q>
where
    Q: Copy + Add<Output = Q> + Div<f64, Output = Q>,
{
    let (first, rest) = values.split_first()?;
    let sum = rest.iter().fold(*first, |sum, v| sum + *v);
    Some(sum / values.len() as f64)
}

/// How many times `step` goes into `q`: their quotient is a plain number.
fn how_many<Q, R>(q: Q, step: Q) -> f64
where
    Q: Div<Output = R>,
    f64: From<R>,
{
    f64::from(q / step)
}

/// The change between two readings, in `unit`: the difference may be a
/// quantity of another type than the readings (`T - T = I`).
fn change<T, I, R>(from: T, to: T, unit: I) -> f64
where
    T: Sub<Output = I>,
    I: Div<Output = R>,
    f64: From<R>,
{
    f64::from((to - from) / unit)
}

// --- A system of this file: nautical ------------------------------------------

mod nav {
    use const_derive_more::ConstOps;
    use core::marker::ConstParamTy;
    use core::ops::{Add, Div, Mul, Sub};

    /// The exponents of distance and time.
    #[derive_const(PartialEq)]
    #[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
    pub struct NavDim {
        pub d: i8,
        pub t: i8,
    }

    const impl Mul for NavDim {
        type Output = Option<NavDim>;
        fn mul(self, o: NavDim) -> Option<NavDim> {
            Some(NavDim {
                d: self.d + o.d,
                t: self.t + o.t,
            })
        }
    }
    const impl Div for NavDim {
        type Output = Option<NavDim>;
        fn div(self, o: NavDim) -> Option<NavDim> {
            Some(NavDim {
                d: self.d - o.d,
                t: self.t - o.t,
            })
        }
    }
    const impl Add for NavDim {
        type Output = Option<NavDim>;
        fn add(self, o: NavDim) -> Option<NavDim> {
            if self == o { Some(self) } else { None }
        }
    }
    const impl Sub for NavDim {
        type Output = Option<NavDim>;
        fn sub(self, o: NavDim) -> Option<NavDim> {
            if self == o { Some(self) } else { None }
        }
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, ConstOps)]
    #[const_ops(one = NavDim { d: 0, t: 0 })]
    pub struct NavQty<const D: NavDim, N>(N);

    pub type Distance<N> = NavQty<{ NavDim { d: 1, t: 0 } }, N>;
    pub type Duration<N> = NavQty<{ NavDim { d: 0, t: 1 } }, N>;
    pub type Speed<N> = NavQty<{ NavDim { d: 1, t: -1 } }, N>;

    pub const METRE: Distance<f64> = NavQty(1.0);
    pub const SECOND: Duration<f64> = NavQty(1.0);
    pub const NAUTICAL_MILE: Distance<f64> = 1852.0 * METRE;
    pub const HOUR: Duration<f64> = 3600.0 * SECOND;
    pub const KNOT: Speed<f64> = NAUTICAL_MILE / HOUR;
}

// --- main ---------------------------------------------------------------------

fn main() {
    use yadc_si::mechanical::*;
    use yadc_si::thermal::*;
    use yadc_si::{KELVIN, KILOGRAM, METRE};

    // yadc-si: two masses, one in pounds, made kilograms by `From`.
    let a: Mass<f64> = 60.0 * KILOGRAM;
    let b: Mass<f64> = (176.0_f64 * POUND).into();
    println!("SI   avg(60 kg, 176 lb)     = {:.2}", average(a, b));
    // nav: two distances.
    let (x, y) = (10.0 * nav::NAUTICAL_MILE, 50.0 * nav::NAUTICAL_MILE);
    println!(
        "Nav  avg(10 nmi, 50 nmi)    = {:.1} nmi",
        how_many(average(x, y), nav::NAUTICAL_MILE)
    );

    let loads = [a, b, 100.0 * KILOGRAM];
    println!("SI   mean of three loads    = {:.2}", mean(&loads).unwrap());
    let legs = [3.0 * nav::NAUTICAL_MILE, 500.0 * nav::METRE];
    println!(
        "Nav  mean(3 nmi, 500 m)     = {:.3} nmi",
        how_many(mean(&legs).unwrap(), nav::NAUTICAL_MILE)
    );

    let marathon: Length<f64> = (42.195_f64 * KILOMETRE).into();
    println!(
        "SI   a marathon in 400 m laps = {:.1}",
        how_many(marathon, 400.0 * METRE)
    );
    let speed = 12.0 * nav::NAUTICAL_MILE / nav::HOUR;
    println!(
        "Nav  12 nmi in an hour       = {:.0} kn",
        how_many(speed, nav::KNOT)
    );

    // Two temperatures, in kelvins; their difference read in kelvins.
    let morning: ThermodynamicTemperature<f64> = (37.0_f64 * CELSIUS).into();
    let evening: ThermodynamicTemperature<f64> = (38.5_f64 * CELSIUS).into();
    println!(
        "SI   37 °C -> 38.5 °C        = {:.1} K",
        change(morning, evening, 1.0 * KELVIN)
    );
    println!(
        "Nav  2 nmi -> 5 nmi          = {} m",
        change(2.0 * nav::NAUTICAL_MILE, 5.0 * nav::NAUTICAL_MILE, nav::METRE)
    );
}
