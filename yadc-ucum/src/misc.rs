use crate::{Dim, Exact, Qty};
use consts::*;
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
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
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

/// The units of `ActionArea`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ActionArea<N>, number = N)]
#[unit(for<N> where ActionArea<N>: [const] yadc::Unit<ActionArea<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ActionAreaUnit {
    #[default]
    #[unit("b", "barn", by = ActionArea::<N>::from(BARN))]
    Barn,
}

/// The units of `ElectricConductance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricConductance<N>, number = N)]
#[unit(for<N> where ElectricConductance<N>: [const] yadc::Unit<ElectricConductance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricConductanceUnit {
    #[default]
    #[unit("mho", "mho", by = ElectricConductance::<N>::from(MHO))]
    Mho,
}

/// The units of `Length`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N> where Length<N>: [const] yadc::Unit<Length<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LengthUnit {
    #[default]
    #[unit("[lne]", "line", by = Length::<N>::from(LINE))]
    Line,
    #[unit("[pnt]", "point", by = Length::<N>::from(POINT))]
    Point,
    #[unit("[pca]", "pica", by = Length::<N>::from(PICA))]
    Pica,
    #[unit("[pnt_pr]", "Printer's point", by = Length::<N>::from(PRINTER_S_POINT))]
    PrinterSPoint,
    #[unit("[pca_pr]", "Printer's pica", by = Length::<N>::from(PRINTER_S_PICA))]
    PrinterSPica,
    #[unit("[pied]", "French foot", by = Length::<N>::from(FRENCH_FOOT))]
    FrenchFoot,
    #[unit("[pouce]", "French inch", by = Length::<N>::from(FRENCH_INCH))]
    FrenchInch,
    #[unit("[ligne]", "French line", by = Length::<N>::from(FRENCH_LINE))]
    FrenchLine,
    #[unit("[didot]", "Didot's point", by = Length::<N>::from(DIDOT_S_POINT))]
    DidotSPoint,
    #[unit("[cicero]", "Didot's pica", by = Length::<N>::from(DIDOT_S_PICA))]
    DidotSPica,
    #[unit("Ao", "Ångström", by = Length::<N>::from(ANGSTROM))]
    Angstrom,
    #[unit("[smoot]", "Smoot", by = Length::<N>::from(SMOOT))]
    Smoot,
}

/// The units of `Mass`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N> where Mass<N>: [const] yadc::Unit<Mass<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassUnit {
    #[default]
    #[unit("[car_m]", "metric carat", by = Mass::<N>::from(METRIC_CARAT))]
    MetricCarat,
}

/// The units of `MassFraction`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassFraction<N>, number = N)]
#[unit(for<N> where MassFraction<N>: [const] yadc::Unit<MassFraction<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassFractionUnit {
    #[default]
    #[unit("[car_Au]", "carat of gold alloys", by = MassFraction::<N>::from(CARAT_OF_GOLD_ALLOYS))]
    CaratOfGoldAlloys,
}

/// The units of `PlaneAngle`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, PlaneAngle<N>: [const] yadc::Unit<PlaneAngle<N>, N> + [const] core::marker::Destruct)]
pub enum PlaneAngleUnit {
    #[default]
    #[unit("circ", "circle", by = PlaneAngle::<N>::from(CIRCLE))]
    Circle,
}

/// The units of `Pressure`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Pressure<N>: [const] yadc::Unit<Pressure<N>, N> + [const] core::marker::Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("att", "technical atmosphere", by = Pressure::<N>::from(TECHNICAL_ATMOSPHERE))]
    TechnicalAtmosphere,
    #[unit("[psi]", "pound per square inch", by = Pressure::<N>::from(POUND_PER_SQUARE_INCH))]
    PoundPerSquareInch,
}

/// The units of `SolidAngle`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SolidAngle<N>: [const] yadc::Unit<SolidAngle<N>, N> + [const] core::marker::Destruct)]
pub enum SolidAngleUnit {
    #[default]
    #[unit("sph", "sphere", by = SolidAngle::<N>::from(SPHERE))]
    Sphere,
}

/// The units of `Turbidity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Turbidity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Turbidity<N>: [const] yadc::Unit<Turbidity<N>, N> + [const] core::marker::Destruct)]
pub enum TurbidityUnit {
    #[default]
    #[unit("[NTU]", "Nephelometric Turbidity Unit", by = Turbidity::<N>::from(NEPHELOMETRIC_TURBIDITY_UNIT))]
    NephelometricTurbidityUnit,
    #[unit("[FNU]", "Formazin Nephelometric Unit", by = Turbidity::<N>::from(FORMAZIN_NEPHELOMETRIC_UNIT))]
    FormazinNephelometricUnit,
}

/// The units of `Volume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Volume<N>: [const] yadc::Unit<Volume<N>, N> + [const] core::marker::Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("st", "stere", by = Volume::<N>::from(STERE))]
    Stere,
}
