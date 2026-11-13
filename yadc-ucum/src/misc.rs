use crate::{Dim, Exact, Qty};
use yadc::Dimension;

pub type ActionArea<N> = Qty<
    {
        Dim {
            m: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricConductance<N> = Qty<
    {
        Dim {
            m: -2,
            s: 1,
            g: -1,
            C: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Length<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Mass<N> = Qty<
    {
        Dim {
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassFraction<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type PlaneAngle<N> = Qty<
    {
        Dim {
            rad: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Pressure<N> = Qty<
    {
        Dim {
            m: -1,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type SolidAngle<N> = Qty<
    {
        Dim {
            rad: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Turbidity<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type Volume<N> = Qty<
    {
        Dim {
            m: 3,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH.quantity(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::{COULOMB, SECOND};

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// The gram, UCUM's base unit of mass.
    pub use crate::base::GRAM;

    /// The radian, UCUM's base unit of plane angle.
    pub use crate::base::RADIAN;

    /// One `Barn` (`b`), as an `ActionArea`.
    pub const BARN: ActionArea<Exact> = (METRE * METRE) * Exact::dec(1, -28);

    /// One `Mho` (`mho`), as an `ElectricConductance`.
    pub const MHO: ElectricConductance<Exact> =
        (SECOND * COULOMB * COULOMB / (METRE * METRE * GRAM)) * Exact::dec(1, -3);

    /// One `Line` (`[lne]`), as a `Length`.
    pub const LINE: Length<Exact> = METRE * (Exact::int(127) / Exact::int(60000));

    /// One `Point` (`[pnt]`), as a `Length`.
    pub const POINT: Length<Exact> = METRE * (Exact::int(127) / Exact::int(360000));

    /// One `Pica` (`[pca]`), as a `Length`.
    pub const PICA: Length<Exact> = METRE * (Exact::int(127) / Exact::int(30000));

    /// One `PrinterSPoint` (`[pnt_pr]`), as a `Length`.
    pub const PRINTER_S_POINT: Length<Exact> = METRE * Exact::dec(3514598, -10);

    /// One `PrinterSPica` (`[pca_pr]`), as a `Length`.
    pub const PRINTER_S_PICA: Length<Exact> = METRE * Exact::dec(42175176, -10);

    /// One `FrenchFoot` (`[pied]`), as a `Length`.
    pub const FRENCH_FOOT: Length<Exact> = METRE * Exact::dec(3248, -4);

    /// One `FrenchInch` (`[pouce]`), as a `Length`.
    pub const FRENCH_INCH: Length<Exact> = METRE * (Exact::int(203) / Exact::int(7500));

    /// One `FrenchLine` (`[ligne]`), as a `Length`.
    pub const FRENCH_LINE: Length<Exact> = METRE * (Exact::int(203) / Exact::int(90000));

    /// One `DidotSPoint` (`[didot]`), as a `Length`.
    pub const DIDOT_S_POINT: Length<Exact> = METRE * (Exact::int(203) / Exact::int(540000));

    /// One `DidotSPica` (`[cicero]`), as a `Length`.
    pub const DIDOT_S_PICA: Length<Exact> = METRE * (Exact::int(203) / Exact::int(45000));

    /// One `Angstrom` (`Ao`), as a `Length`.
    pub const ANGSTROM: Length<Exact> = METRE * Exact::dec(1, -10);

    /// One `Smoot` (`[smoot]`), as a `Length`.
    pub const SMOOT: Length<Exact> = METRE * Exact::dec(17018, -4);

    /// One `MetricCarat` (`[car_m]`), as a `Mass`.
    pub const METRIC_CARAT: Mass<Exact> = GRAM * Exact::dec(2, -1);

    /// One `CaratOfGoldAlloys` (`[car_Au]`), as a `MassFraction`.
    pub const CARAT_OF_GOLD_ALLOYS: MassFraction<Exact> = ONE * (Exact::int(1) / Exact::int(24));

    /// One `Circle` (`circ`), as a `PlaneAngle`.
    pub const CIRCLE: PlaneAngle<Exact> = RADIAN * (Exact::int(2) * Exact::PI);

    /// One `TechnicalAtmosphere` (`att`), as a `Pressure`.
    pub const TECHNICAL_ATMOSPHERE: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::int(98066500);

    /// One `PoundPerSquareInch` (`[psi]`), as a `Pressure`.
    pub const POUND_PER_SQUARE_INCH: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * (Exact::int(8896443230521) / Exact::int(1290320));

    /// One `Sphere` (`sph`), as a `SolidAngle`.
    pub const SPHERE: SolidAngle<Exact> = (RADIAN * RADIAN) * (Exact::int(4) * Exact::PI);

    /// One `NephelometricTurbidityUnit` (`[NTU]`), as a `Turbidity`.
    pub const NEPHELOMETRIC_TURBIDITY_UNIT: Turbidity<Exact> = ONE;

    /// One `FormazinNephelometricUnit` (`[FNU]`), as a `Turbidity`.
    pub const FORMAZIN_NEPHELOMETRIC_UNIT: Turbidity<Exact> = NEPHELOMETRIC_TURBIDITY_UNIT;

    /// One `Stere` (`st`), as a `Volume`.
    pub const STERE: Volume<Exact> = METRE * METRE * METRE;
}
