use crate::{Dim, Qty};
use yadc::Dimension;

pub type AmountOfSubstance<N> = Qty<{ Dim::DIMENSIONLESS }, N>;
pub type AmountOfSubstanceDissolvedParticles<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

/// The dimension of `CatalyticActivity`.
pub const CATALYTIC_ACTIVITY: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};
pub type CatalyticActivity<N> = Qty<CATALYTIC_ACTIVITY, N>;

/// The dimension of `FluidResistance`.
pub const FLUID_RESISTANCE: Dim = Dim {
    m: -4,
    s: -1,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type FluidResistance<N> = Qty<FLUID_RESISTANCE, N>;

/// The dimension of `GaugeOfCatheters`.
pub const GAUGE_OF_CATHETERS: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};
pub type GaugeOfCatheters<N> = Qty<GAUGE_OF_CATHETERS, N>;

/// The dimension of `LineicNumber`.
pub const LINEIC_NUMBER: Dim = Dim {
    m: -1,
    ..Dim::DIMENSIONLESS
};
pub type LineicNumber<N> = Qty<LINEIC_NUMBER, N>;

/// The dimension of `MassConcentration`.
pub const MASS_CONCENTRATION: Dim = Dim {
    m: -3,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type MassConcentration<N> = Qty<MASS_CONCENTRATION, N>;

/// The dimension of `MetabolicCostOfPhysicalActivity`.
pub const METABOLIC_COST_OF_PHYSICAL_ACTIVITY: Dim = Dim {
    m: 3,
    s: -1,
    g: -1,
    ..Dim::DIMENSIONLESS
};
pub type MetabolicCostOfPhysicalActivity<N> = Qty<METABOLIC_COST_OF_PHYSICAL_ACTIVITY, N>;

/// The dimension of `Pressure`.
pub const PRESSURE: Dim = Dim {
    m: -1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Pressure<N> = Qty<PRESSURE, N>;

/// The dimension of `RefractionOfALens`.
pub const REFRACTION_OF_A_LENS: Dim = Dim {
    m: -1,
    ..Dim::DIMENSIONLESS
};
pub type RefractionOfALens<N> = Qty<REFRACTION_OF_A_LENS, N>;

/// The dimension of `SedimentationCoefficient`.
pub const SEDIMENTATION_COEFFICIENT: Dim = Dim {
    s: 1,
    ..Dim::DIMENSIONLESS
};
pub type SedimentationCoefficient<N> = Qty<SEDIMENTATION_COEFFICIENT, N>;
pub type ViewAreaInMicroscope<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

/// The dimension of `Volume`.
pub const VOLUME: Dim = Dim {
    m: 3,
    ..Dim::DIMENSIONLESS
};
pub type Volume<N> = Qty<VOLUME, N>;
pub type XRayAttenuation<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Dim, Exact, UcumUnit, UcumUnitConsts};
    use yadc::Dimension;

    /// Equivalents (`eq`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` coherent units.
    pub type Equivalents = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "eq",
                name: "equivalents",
            }
        },
    >;

    /// Osmole (`osm`), a unit of `AmountOfSubstanceDissolvedParticles`: `n` of them is `n · 602214076e15` coherent units.
    pub type Osmole = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "osm",
                name: "osmole",
            }
        },
    >;

    /// Katal (`kat`), a unit of `CatalyticActivity`: `n` of them is `n · 602214076e15` coherent units.
    pub type Katal = UcumUnit<
        CATALYTIC_ACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "kat",
                name: "katal",
            }
        },
    >;

    /// Unit (`U`), a unit of `CatalyticActivity`: `n` of them is `n · 301107038e8 / 3` coherent units.
    pub type Unit = UcumUnit<
        CATALYTIC_ACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(30110703800000000) / Exact::int(3),
                zero: Exact::ZERO,
                symbol: "U",
                name: "Unit",
            }
        },
    >;

    /// Peripheral vascular resistance unit (`[PRU]`), a unit of `FluidResistance`: `n` of them is `n · 133322e6` coherent units.
    pub type PeripheralVascularResistanceUnit = UcumUnit<
        FLUID_RESISTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(133322000000),
                zero: Exact::ZERO,
                symbol: "[PRU]",
                name: "peripheral vascular resistance unit",
            }
        },
    >;

    /// Wood unit (`[wood'U]`), a unit of `FluidResistance`: `n` of them is `n · 799932e4` coherent units.
    pub type WoodUnit = UcumUnit<
        FLUID_RESISTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(7999320000),
                zero: Exact::ZERO,
                symbol: "[wood'U]",
                name: "Wood unit",
            }
        },
    >;

    /// French (`[Ch]`), a unit of `GaugeOfCatheters`: `n` of them is `n · 1 / 3000` coherent units.
    pub type French = UcumUnit<
        GAUGE_OF_CATHETERS,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(3000),
                zero: Exact::ZERO,
                symbol: "[Ch]",
                name: "french",
            }
        },
    >;

    /// Mesh (`[mesh_i]`), a unit of `LineicNumber`: `n` of them is `n · 5e3 / 127` coherent units.
    pub type Mesh = UcumUnit<
        LINEIC_NUMBER,
        {
            UcumUnitConsts {
                size: Exact::int(5000) / Exact::int(127),
                zero: Exact::ZERO,
                symbol: "[mesh_i]",
                name: "mesh",
            }
        },
    >;

    /// Gram percent (`g%`), a unit of `MassConcentration`: `n` of them is `n · 1e4` coherent units.
    pub type GramPercent = UcumUnit<
        MASS_CONCENTRATION,
        {
            UcumUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "g%",
                name: "gram percent",
            }
        },
    >;

    /// Metabolic equivalent (`[MET]`), a unit of `MetabolicCostOfPhysicalActivity`: `n` of them is `n · 7 / 120000000000` coherent units.
    pub type MetabolicEquivalent = UcumUnit<
        METABOLIC_COST_OF_PHYSICAL_ACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(7) / Exact::int(120000000000),
                zero: Exact::ZERO,
                symbol: "[MET]",
                name: "metabolic equivalent",
            }
        },
    >;

    /// Meter of water column (`m[H2O]`), a unit of `Pressure`: `n` of them is `n · 980665e1` coherent units.
    pub type MeterOfWaterColumn = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(9806650),
                zero: Exact::ZERO,
                symbol: "m[H2O]",
                name: "meter of water column",
            }
        },
    >;

    /// Meter of mercury column (`m[Hg]`), a unit of `Pressure`: `n` of them is `n · 133322e3` coherent units.
    pub type MeterOfMercuryColumn = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(133322000),
                zero: Exact::ZERO,
                symbol: "m[Hg]",
                name: "meter of mercury column",
            }
        },
    >;

    /// Inch of water column (`[in_i'H2O]`), a unit of `Pressure`: `n` of them is `n · 24908891e-2` coherent units.
    pub type InchOfWaterColumn = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::dec(24908891, -2),
                zero: Exact::ZERO,
                symbol: "[in_i'H2O]",
                name: "inch of water column",
            }
        },
    >;

    /// Inch of mercury column (`[in_i'Hg]`), a unit of `Pressure`: `n` of them is `n · 33863788e-1` coherent units.
    pub type InchOfMercuryColumn = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::dec(33863788, -1),
                zero: Exact::ZERO,
                symbol: "[in_i'Hg]",
                name: "inch of mercury column",
            }
        },
    >;

    /// Diopter (`[diop]`), a unit of `RefractionOfALens`: `n` of them is `n · 1` coherent units.
    pub type Diopter = UcumUnit<
        REFRACTION_OF_A_LENS,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[diop]",
                name: "diopter",
            }
        },
    >;

    /// Svedberg unit (`[S]`), a unit of `SedimentationCoefficient`: `n` of them is `n · 1e-13` coherent units.
    pub type SvedbergUnit = UcumUnit<
        SEDIMENTATION_COEFFICIENT,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -13),
                zero: Exact::ZERO,
                symbol: "[S]",
                name: "Svedberg unit",
            }
        },
    >;

    /// High power field (`[HPF]`), a unit of `ViewAreaInMicroscope`: `n` of them is `n · 1` coherent units.
    pub type HighPowerField = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[HPF]",
                name: "high power field",
            }
        },
    >;

    /// Low power field (`[LPF]`), a unit of `ViewAreaInMicroscope`: `n` of them is `n · 1e2` coherent units.
    pub type LowPowerField = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "[LPF]",
                name: "low power field",
            }
        },
    >;

    /// Drop (`[drp]`), a unit of `Volume`: `n` of them is `n · 5e-8` coherent units.
    pub type Drop = UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(5, -8),
                zero: Exact::ZERO,
                symbol: "[drp]",
                name: "drop",
            }
        },
    >;

    /// Hounsfield unit (`[hnsf'U]`), a unit of `XRayAttenuation`: `n` of them is `n · 1` coherent units.
    pub type HounsfieldUnit = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[hnsf'U]",
                name: "Hounsfield unit",
            }
        },
    >;
}
