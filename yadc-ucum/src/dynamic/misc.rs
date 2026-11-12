//! The units of `misc`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::misc::consts::*;
use crate::misc::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `ActionArea`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ActionArea<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ActionArea<N>: [const] Unit<ActionArea<N>, N> + [const] Destruct)]
pub enum ActionAreaUnit {
    #[default]
    #[unit("b", "barn", by = ActionArea::<N>::from(BARN))]
    Barn,
}

/// The units of `ElectricConductance`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricConductance<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where ElectricConductance<N>: [const] Unit<ElectricConductance<N>, N> + [const] Destruct)]
pub enum ElectricConductanceUnit {
    #[default]
    #[unit("mho", "mho", by = ElectricConductance::<N>::from(MHO))]
    Mho,
}

/// The units of `Length`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Length<N>: [const] Unit<Length<N>, N> + [const] Destruct)]
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

/// The units of `Mass`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Mass<N>: [const] Unit<Mass<N>, N> + [const] Destruct)]
pub enum MassUnit {
    #[default]
    #[unit("[car_m]", "metric carat", by = Mass::<N>::from(METRIC_CARAT))]
    MetricCarat,
}

/// The units of `MassFraction`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassFraction<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where MassFraction<N>: [const] Unit<MassFraction<N>, N> + [const] Destruct)]
pub enum MassFractionUnit {
    #[default]
    #[unit("[car_Au]", "carat of gold alloys", by = MassFraction::<N>::from(CARAT_OF_GOLD_ALLOYS))]
    CaratOfGoldAlloys,
}

/// The units of `PlaneAngle`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where PlaneAngle<N>: [const] Unit<PlaneAngle<N>, N> + [const] Destruct)]
pub enum PlaneAngleUnit {
    #[default]
    #[unit("circ", "circle", by = PlaneAngle::<N>::from(CIRCLE))]
    Circle,
}

/// The units of `Pressure`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Pressure<N>: [const] Unit<Pressure<N>, N> + [const] Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("att", "technical atmosphere", by = Pressure::<N>::from(TECHNICAL_ATMOSPHERE))]
    TechnicalAtmosphere,
    #[unit("[psi]", "pound per square inch", by = Pressure::<N>::from(POUND_PER_SQUARE_INCH))]
    PoundPerSquareInch,
}

/// The units of `SolidAngle`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where SolidAngle<N>: [const] Unit<SolidAngle<N>, N> + [const] Destruct)]
pub enum SolidAngleUnit {
    #[default]
    #[unit("sph", "sphere", by = SolidAngle::<N>::from(SPHERE))]
    Sphere,
}

/// The units of `Turbidity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Turbidity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Turbidity<N>: [const] Unit<Turbidity<N>, N> + [const] Destruct)]
pub enum TurbidityUnit {
    #[default]
    #[unit("[NTU]", "Nephelometric Turbidity Unit", by = Turbidity::<N>::from(NEPHELOMETRIC_TURBIDITY_UNIT))]
    NephelometricTurbidityUnit,
    #[unit("[FNU]", "Formazin Nephelometric Unit", by = Turbidity::<N>::from(FORMAZIN_NEPHELOMETRIC_UNIT))]
    FormazinNephelometricUnit,
}

/// The units of `Volume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Volume<N>: [const] Unit<Volume<N>, N> + [const] Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("st", "stere", by = Volume::<N>::from(STERE))]
    Stere,
}
