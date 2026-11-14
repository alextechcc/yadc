use crate::Dim;
use yadc::Dimension;

/// The dimension of `AmountOfSubstance`: `Qty<AMOUNT_OF_SUBSTANCE, N>` is one.
pub const AMOUNT_OF_SUBSTANCE: Dim = Dim::DIMENSIONLESS;

/// The dimension of `AmountOfSubstanceDissolvedParticles`: `Qty<AMOUNT_OF_SUBSTANCE_DISSOLVED_PARTICLES, N>` is one.
pub const AMOUNT_OF_SUBSTANCE_DISSOLVED_PARTICLES: Dim = Dim::DIMENSIONLESS;

/// The dimension of `CatalyticActivity`: `Qty<CATALYTIC_ACTIVITY, N>` is one.
pub const CATALYTIC_ACTIVITY: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `FluidResistance`: `Qty<FLUID_RESISTANCE, N>` is one.
pub const FLUID_RESISTANCE: Dim = Dim {
    m: -4,
    s: -1,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `GaugeOfCatheters`: `Qty<GAUGE_OF_CATHETERS, N>` is one.
pub const GAUGE_OF_CATHETERS: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `LineicNumber`: `Qty<LINEIC_NUMBER, N>` is one.
pub const LINEIC_NUMBER: Dim = Dim {
    m: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `MassConcentration`: `Qty<MASS_CONCENTRATION, N>` is one.
pub const MASS_CONCENTRATION: Dim = Dim {
    m: -3,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `MetabolicCostOfPhysicalActivity`: `Qty<METABOLIC_COST_OF_PHYSICAL_ACTIVITY, N>` is one.
pub const METABOLIC_COST_OF_PHYSICAL_ACTIVITY: Dim = Dim {
    m: 3,
    s: -1,
    g: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Pressure`: `Qty<PRESSURE, N>` is one.
pub const PRESSURE: Dim = Dim {
    m: -1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `RefractionOfALens`: `Qty<REFRACTION_OF_A_LENS, N>` is one.
pub const REFRACTION_OF_A_LENS: Dim = Dim {
    m: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `SedimentationCoefficient`: `Qty<SEDIMENTATION_COEFFICIENT, N>` is one.
pub const SEDIMENTATION_COEFFICIENT: Dim = Dim {
    s: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ViewAreaInMicroscope`: `Qty<VIEW_AREA_IN_MICROSCOPE, N>` is one.
pub const VIEW_AREA_IN_MICROSCOPE: Dim = Dim::DIMENSIONLESS;

/// The dimension of `Volume`: `Qty<VOLUME, N>` is one.
pub const VOLUME: Dim = Dim {
    m: 3,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `XRayAttenuation`: `Qty<X_RAY_ATTENUATION, N>` is one.
pub const X_RAY_ATTENUATION: Dim = Dim::DIMENSIONLESS;

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Equivalents (`eq`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` coherent units.
    pub const EQUIVALENTS: UcumUnit<
        AMOUNT_OF_SUBSTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "eq",
                name: "equivalents",
            }
        },
    > = UcumUnit;

    /// Osmole (`osm`), a unit of `AmountOfSubstanceDissolvedParticles`: `n` of them is `n · 602214076e15` coherent units.
    pub const OSMOLE: UcumUnit<
        AMOUNT_OF_SUBSTANCE_DISSOLVED_PARTICLES,
        {
            UcumUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "osm",
                name: "osmole",
            }
        },
    > = UcumUnit;

    /// Katal (`kat`), a unit of `CatalyticActivity`: `n` of them is `n · 602214076e15` coherent units.
    pub const KATAL: UcumUnit<
        CATALYTIC_ACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "kat",
                name: "katal",
            }
        },
    > = UcumUnit;

    /// Unit (`U`), a unit of `CatalyticActivity`: `n` of them is `n · 301107038e8 / 3` coherent units.
    pub const UNIT: UcumUnit<
        CATALYTIC_ACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(30110703800000000) / Exact::int(3),
                zero: Exact::ZERO,
                symbol: "U",
                name: "Unit",
            }
        },
    > = UcumUnit;

    /// Peripheral vascular resistance unit (`[PRU]`), a unit of `FluidResistance`: `n` of them is `n · 133322e6` coherent units.
    pub const PERIPHERAL_VASCULAR_RESISTANCE_UNIT: UcumUnit<
        FLUID_RESISTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(133322000000),
                zero: Exact::ZERO,
                symbol: "[PRU]",
                name: "peripheral vascular resistance unit",
            }
        },
    > = UcumUnit;

    /// Wood unit (`[wood'U]`), a unit of `FluidResistance`: `n` of them is `n · 799932e4` coherent units.
    pub const WOOD_UNIT: UcumUnit<
        FLUID_RESISTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(7999320000),
                zero: Exact::ZERO,
                symbol: "[wood'U]",
                name: "Wood unit",
            }
        },
    > = UcumUnit;

    /// French (`[Ch]`), a unit of `GaugeOfCatheters`: `n` of them is `n · 1 / 3000` coherent units.
    pub const FRENCH: UcumUnit<
        GAUGE_OF_CATHETERS,
        {
            UcumUnitConsts {
                size: Exact::int(1) / Exact::int(3000),
                zero: Exact::ZERO,
                symbol: "[Ch]",
                name: "french",
            }
        },
    > = UcumUnit;

    /// Mesh (`[mesh_i]`), a unit of `LineicNumber`: `n` of them is `n · 5e3 / 127` coherent units.
    pub const MESH: UcumUnit<
        LINEIC_NUMBER,
        {
            UcumUnitConsts {
                size: Exact::int(5000) / Exact::int(127),
                zero: Exact::ZERO,
                symbol: "[mesh_i]",
                name: "mesh",
            }
        },
    > = UcumUnit;

    /// Gram percent (`g%`), a unit of `MassConcentration`: `n` of them is `n · 1e4` coherent units.
    pub const GRAM_PERCENT: UcumUnit<
        MASS_CONCENTRATION,
        {
            UcumUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "g%",
                name: "gram percent",
            }
        },
    > = UcumUnit;

    /// Metabolic equivalent (`[MET]`), a unit of `MetabolicCostOfPhysicalActivity`: `n` of them is `n · 7 / 120000000000` coherent units.
    pub const METABOLIC_EQUIVALENT: UcumUnit<
        METABOLIC_COST_OF_PHYSICAL_ACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(7) / Exact::int(120000000000),
                zero: Exact::ZERO,
                symbol: "[MET]",
                name: "metabolic equivalent",
            }
        },
    > = UcumUnit;

    /// Meter of water column (`m[H2O]`), a unit of `Pressure`: `n` of them is `n · 980665e1` coherent units.
    pub const METER_OF_WATER_COLUMN: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(9806650),
                zero: Exact::ZERO,
                symbol: "m[H2O]",
                name: "meter of water column",
            }
        },
    > = UcumUnit;

    /// Meter of mercury column (`m[Hg]`), a unit of `Pressure`: `n` of them is `n · 133322e3` coherent units.
    pub const METER_OF_MERCURY_COLUMN: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(133322000),
                zero: Exact::ZERO,
                symbol: "m[Hg]",
                name: "meter of mercury column",
            }
        },
    > = UcumUnit;

    /// Inch of water column (`[in_i'H2O]`), a unit of `Pressure`: `n` of them is `n · 24908891e-2` coherent units.
    pub const INCH_OF_WATER_COLUMN: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::dec(24908891, -2),
                zero: Exact::ZERO,
                symbol: "[in_i'H2O]",
                name: "inch of water column",
            }
        },
    > = UcumUnit;

    /// Inch of mercury column (`[in_i'Hg]`), a unit of `Pressure`: `n` of them is `n · 33863788e-1` coherent units.
    pub const INCH_OF_MERCURY_COLUMN: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::dec(33863788, -1),
                zero: Exact::ZERO,
                symbol: "[in_i'Hg]",
                name: "inch of mercury column",
            }
        },
    > = UcumUnit;

    /// Diopter (`[diop]`), a unit of `RefractionOfALens`: `n` of them is `n · 1` coherent units.
    pub const DIOPTER: UcumUnit<
        REFRACTION_OF_A_LENS,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[diop]",
                name: "diopter",
            }
        },
    > = UcumUnit;

    /// Svedberg unit (`[S]`), a unit of `SedimentationCoefficient`: `n` of them is `n · 1e-13` coherent units.
    pub const SVEDBERG_UNIT: UcumUnit<
        SEDIMENTATION_COEFFICIENT,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -13),
                zero: Exact::ZERO,
                symbol: "[S]",
                name: "Svedberg unit",
            }
        },
    > = UcumUnit;

    /// High power field (`[HPF]`), a unit of `ViewAreaInMicroscope`: `n` of them is `n · 1` coherent units.
    pub const HIGH_POWER_FIELD: UcumUnit<
        VIEW_AREA_IN_MICROSCOPE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[HPF]",
                name: "high power field",
            }
        },
    > = UcumUnit;

    /// Low power field (`[LPF]`), a unit of `ViewAreaInMicroscope`: `n` of them is `n · 1e2` coherent units.
    pub const LOW_POWER_FIELD: UcumUnit<
        VIEW_AREA_IN_MICROSCOPE,
        {
            UcumUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "[LPF]",
                name: "low power field",
            }
        },
    > = UcumUnit;

    /// Drop (`[drp]`), a unit of `Volume`: `n` of them is `n · 5e-8` coherent units.
    pub const DROP: UcumUnit<
        VOLUME,
        {
            UcumUnitConsts {
                size: Exact::dec(5, -8),
                zero: Exact::ZERO,
                symbol: "[drp]",
                name: "drop",
            }
        },
    > = UcumUnit;

    /// Hounsfield unit (`[hnsf'U]`), a unit of `XRayAttenuation`: `n` of them is `n · 1` coherent units.
    pub const HOUNSFIELD_UNIT: UcumUnit<
        X_RAY_ATTENUATION,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "[hnsf'U]",
                name: "Hounsfield unit",
            }
        },
    > = UcumUnit;
}
