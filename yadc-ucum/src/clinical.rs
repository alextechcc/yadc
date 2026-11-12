use crate::{Dim, Exact, Qty};
use consts::*;
use yadc::Dimension;

pub type AmountOfSubstance<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type AmountOfSubstanceDissolvedParticles<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type CatalyticActivity<N> = Qty<
    {
        Dim {
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type FluidResistance<N> = Qty<
    {
        Dim {
            m: -4,
            s: -1,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type GaugeOfCatheters<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type LineicNumber<N> = Qty<
    {
        Dim {
            m: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MassConcentration<N> = Qty<
    {
        Dim {
            m: -3,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MetabolicCostOfPhysicalActivity<N> = Qty<
    {
        Dim {
            m: 3,
            s: -1,
            g: -1,
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

pub type RefractionOfALens<N> = Qty<
    {
        Dim {
            m: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type SedimentationCoefficient<N> = Qty<
    {
        Dim {
            s: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ViewAreaInMicroscope<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type Volume<N> = Qty<
    {
        Dim {
            m: 3,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type XRayAttenuation<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::GRAM;

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// The second, UCUM's base unit of time.
    pub use crate::base::SECOND;

    /// One `Equivalents` (`eq`), as an `AmountOfSubstance`.
    pub const EQUIVALENTS: AmountOfSubstance<Exact> = ONE * Exact::int(602214076000000000000000);

    /// One `Osmole` (`osm`), as an `AmountOfSubstanceDissolvedParticles`.
    pub const OSMOLE: AmountOfSubstanceDissolvedParticles<Exact> =
        ONE * Exact::int(602214076000000000000000);

    /// One `Katal` (`kat`), as a `CatalyticActivity`.
    pub const KATAL: CatalyticActivity<Exact> =
        (ONE / SECOND) * Exact::int(602214076000000000000000);

    /// One `Unit` (`U`), as a `CatalyticActivity`.
    pub const UNIT: CatalyticActivity<Exact> =
        (ONE / SECOND) * (Exact::int(30110703800000000) / Exact::int(3));

    /// One `PeripheralVascularResistanceUnit` (`[PRU]`), as a `FluidResistance`.
    pub const PERIPHERAL_VASCULAR_RESISTANCE_UNIT: FluidResistance<Exact> =
        (GRAM / (METRE * METRE * METRE * METRE * SECOND)) * Exact::int(133322000000);

    /// One `WoodUnit` (`[wood'U]`), as a `FluidResistance`.
    pub const WOOD_UNIT: FluidResistance<Exact> =
        (GRAM / (METRE * METRE * METRE * METRE * SECOND)) * Exact::int(7999320000);

    /// One `French` (`[Ch]`), as a `GaugeOfCatheters`.
    pub const FRENCH: GaugeOfCatheters<Exact> = METRE * (Exact::int(1) / Exact::int(3000));

    /// One `Mesh` (`[mesh_i]`), as a `LineicNumber`.
    pub const MESH: LineicNumber<Exact> = (ONE / METRE) * (Exact::int(5000) / Exact::int(127));

    /// One `GramPercent` (`g%`), as a `MassConcentration`.
    pub const GRAM_PERCENT: MassConcentration<Exact> =
        (GRAM / (METRE * METRE * METRE)) * Exact::int(10000);

    /// One `MetabolicEquivalent` (`[MET]`), as a `MetabolicCostOfPhysicalActivity`.
    pub const METABOLIC_EQUIVALENT: MetabolicCostOfPhysicalActivity<Exact> =
        (METRE * METRE * METRE / (SECOND * GRAM)) * (Exact::int(7) / Exact::int(120000000000));

    /// One `MeterOfWaterColumn` (`m[H2O]`), as a `Pressure`.
    pub const METER_OF_WATER_COLUMN: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::int(9806650);

    /// One `MeterOfMercuryColumn` (`m[Hg]`), as a `Pressure`.
    pub const METER_OF_MERCURY_COLUMN: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::int(133322000);

    /// One `InchOfWaterColumn` (`[in_i'H2O]`), as a `Pressure`.
    pub const INCH_OF_WATER_COLUMN: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::dec(24908891, -2);

    /// One `InchOfMercuryColumn` (`[in_i'Hg]`), as a `Pressure`.
    pub const INCH_OF_MERCURY_COLUMN: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::dec(33863788, -1);

    /// One `Diopter` (`[diop]`), as a `RefractionOfALens`.
    pub const DIOPTER: RefractionOfALens<Exact> = ONE / METRE;

    /// One `SvedbergUnit` (`[S]`), as a `SedimentationCoefficient`.
    pub const SVEDBERG_UNIT: SedimentationCoefficient<Exact> = SECOND * Exact::dec(1, -13);

    /// One `HighPowerField` (`[HPF]`), as a `ViewAreaInMicroscope`.
    pub const HIGH_POWER_FIELD: ViewAreaInMicroscope<Exact> = ONE;

    /// One `LowPowerField` (`[LPF]`), as a `ViewAreaInMicroscope`.
    pub const LOW_POWER_FIELD: ViewAreaInMicroscope<Exact> = HIGH_POWER_FIELD * Exact::int(100);

    /// One `Drop` (`[drp]`), as a `Volume`.
    pub const DROP: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(5, -8);

    /// One `HounsfieldUnit` (`[hnsf'U]`), as a `XRayAttenuation`.
    pub const HOUNSFIELD_UNIT: XRayAttenuation<Exact> = ONE;
}

/// The units of `AmountOfSubstance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N)]
#[unit(for<N> where AmountOfSubstance<N>: [const] yadc::Unit<AmountOfSubstance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit("eq", "equivalents", by = AmountOfSubstance::<N>::from(EQUIVALENTS))]
    Equivalents,
}

/// The units of `AmountOfSubstanceDissolvedParticles`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstanceDissolvedParticles<N>, number = N)]
#[unit(for<N> where AmountOfSubstanceDissolvedParticles<N>: [const] yadc::Unit<AmountOfSubstanceDissolvedParticles<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AmountOfSubstanceDissolvedParticlesUnit {
    #[default]
    #[unit("osm", "osmole", by = AmountOfSubstanceDissolvedParticles::<N>::from(OSMOLE))]
    Osmole,
}

/// The units of `CatalyticActivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = CatalyticActivity<N>, number = N)]
#[unit(for<N> where CatalyticActivity<N>: [const] yadc::Unit<CatalyticActivity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum CatalyticActivityUnit {
    #[default]
    #[unit("kat", "katal", by = CatalyticActivity::<N>::from(KATAL))]
    Katal,
    #[unit("U", "Unit", by = CatalyticActivity::<N>::from(UNIT))]
    Unit,
}

/// The units of `FluidResistance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FluidResistance<N>, number = N)]
#[unit(for<N> where FluidResistance<N>: [const] yadc::Unit<FluidResistance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum FluidResistanceUnit {
    #[default]
    #[unit("[PRU]", "peripheral vascular resistance unit", by = FluidResistance::<N>::from(PERIPHERAL_VASCULAR_RESISTANCE_UNIT))]
    PeripheralVascularResistanceUnit,
    #[unit("[wood'U]", "Wood unit", by = FluidResistance::<N>::from(WOOD_UNIT))]
    WoodUnit,
}

/// The units of `GaugeOfCatheters`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = GaugeOfCatheters<N>, number = N)]
#[unit(for<N> where GaugeOfCatheters<N>: [const] yadc::Unit<GaugeOfCatheters<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum GaugeOfCathetersUnit {
    #[default]
    #[unit("[Ch]", "french", by = GaugeOfCatheters::<N>::from(FRENCH))]
    French,
}

/// The units of `LineicNumber`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LineicNumber<N>, number = N)]
#[unit(for<N> where LineicNumber<N>: [const] yadc::Unit<LineicNumber<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LineicNumberUnit {
    #[default]
    #[unit("[mesh_i]", "mesh", by = LineicNumber::<N>::from(MESH))]
    Mesh,
}

/// The units of `MassConcentration`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MassConcentration<N>, number = N)]
#[unit(for<N> where MassConcentration<N>: [const] yadc::Unit<MassConcentration<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassConcentrationUnit {
    #[default]
    #[unit("g%", "gram percent", by = MassConcentration::<N>::from(GRAM_PERCENT))]
    GramPercent,
}

/// The units of `MetabolicCostOfPhysicalActivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MetabolicCostOfPhysicalActivity<N>, number = N)]
#[unit(for<N> where MetabolicCostOfPhysicalActivity<N>: [const] yadc::Unit<MetabolicCostOfPhysicalActivity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MetabolicCostOfPhysicalActivityUnit {
    #[default]
    #[unit("[MET]", "metabolic equivalent", by = MetabolicCostOfPhysicalActivity::<N>::from(METABOLIC_EQUIVALENT))]
    MetabolicEquivalent,
}

/// The units of `Pressure`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Pressure<N>: [const] yadc::Unit<Pressure<N>, N> + [const] core::marker::Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("m[H2O]", "meter of water column", by = Pressure::<N>::from(METER_OF_WATER_COLUMN))]
    MeterOfWaterColumn,
    #[unit("m[Hg]", "meter of mercury column", by = Pressure::<N>::from(METER_OF_MERCURY_COLUMN))]
    MeterOfMercuryColumn,
    #[unit("[in_i'H2O]", "inch of water column", by = Pressure::<N>::from(INCH_OF_WATER_COLUMN))]
    InchOfWaterColumn,
    #[unit("[in_i'Hg]", "inch of mercury column", by = Pressure::<N>::from(INCH_OF_MERCURY_COLUMN))]
    InchOfMercuryColumn,
}

/// The units of `RefractionOfALens`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = RefractionOfALens<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, RefractionOfALens<N>: [const] yadc::Unit<RefractionOfALens<N>, N> + [const] core::marker::Destruct)]
pub enum RefractionOfALensUnit {
    #[default]
    #[unit("[diop]", "diopter", by = RefractionOfALens::<N>::from(DIOPTER))]
    Diopter,
}

/// The units of `SedimentationCoefficient`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SedimentationCoefficient<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SedimentationCoefficient<N>: [const] yadc::Unit<SedimentationCoefficient<N>, N> + [const] core::marker::Destruct)]
pub enum SedimentationCoefficientUnit {
    #[default]
    #[unit("[S]", "Svedberg unit", by = SedimentationCoefficient::<N>::from(SVEDBERG_UNIT))]
    SvedbergUnit,
}

/// The units of `ViewAreaInMicroscope`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ViewAreaInMicroscope<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, ViewAreaInMicroscope<N>: [const] yadc::Unit<ViewAreaInMicroscope<N>, N> + [const] core::marker::Destruct)]
pub enum ViewAreaInMicroscopeUnit {
    #[default]
    #[unit("[HPF]", "high power field", by = ViewAreaInMicroscope::<N>::from(HIGH_POWER_FIELD))]
    HighPowerField,
    #[unit("[LPF]", "low power field", by = ViewAreaInMicroscope::<N>::from(LOW_POWER_FIELD))]
    LowPowerField,
}

/// The units of `Volume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Volume<N>: [const] yadc::Unit<Volume<N>, N> + [const] core::marker::Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("[drp]", "drop", by = Volume::<N>::from(DROP))]
    Drop,
}

/// The units of `XRayAttenuation`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = XRayAttenuation<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, XRayAttenuation<N>: [const] yadc::Unit<XRayAttenuation<N>, N> + [const] core::marker::Destruct)]
pub enum XRayAttenuationUnit {
    #[default]
    #[unit("[hnsf'U]", "Hounsfield unit", by = XRayAttenuation::<N>::from(HOUNSFIELD_UNIT))]
    HounsfieldUnit,
}
