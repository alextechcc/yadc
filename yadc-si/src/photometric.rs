use crate::{Exact, ISQ, SIQuantity};
use yadc::Dimension;

pub type Luminance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type LuminousIntensity<N> = SIQuantity<
    {
        ISQ {
            cd: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Illuminance<N> = SIQuantity<
    {
        ISQ {
            m: -2,
            cd: 1,
            rad: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type RadiantExposure<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{METRE, RADIAN};

    /// The candela, the SI base unit of luminous intensity.
    pub use crate::base::CANDELA;

    /// One `Footlambert` (`fl`), as a `Luminance`.
    pub const FOOTLAMBERT: Luminance<Exact> = (CANDELA / (METRE * METRE))
        * (Exact::int(1) / (Exact::PI * Exact::dec(3048, -4) * Exact::dec(3048, -4)));

    /// One `Lambert` (`la`), as a `Luminance`.
    pub const LAMBERT: Luminance<Exact> =
        (CANDELA / (METRE * METRE)) * (Exact::int(10000) / Exact::PI);

    /// One `Stilb` (`sb`), as a `Luminance`.
    pub const STILB: Luminance<Exact> = (CANDELA / (METRE * METRE)) * Exact::int(10000);

    /// One `Lux` (`lx`), as an `Illuminance`.
    pub const LUX: Illuminance<Exact> = CANDELA * RADIAN * RADIAN / (METRE * METRE);

    /// One `Phot` (`ph`), as an `Illuminance`.
    pub const PHOT: Illuminance<Exact> = LUX * Exact::int(10000);

    /// One `Footcandle` (`fc`), as an `Illuminance`.
    pub const FOOTCANDLE: Illuminance<Exact> =
        LUX * (Exact::int(1) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)));
}
