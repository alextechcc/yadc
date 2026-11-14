use crate::Dim;
use yadc::Dimension;

/// The dimension of `AmountOfSubstance`: `Qty<AMOUNT_OF_SUBSTANCE, N>` is one.
pub const AMOUNT_OF_SUBSTANCE: Dim = Dim::DIMENSIONLESS;

/// The dimension of `DoseEquivalent`: `Qty<DOSE_EQUIVALENT, N>` is one.
pub const DOSE_EQUIVALENT: Dim = Dim {
    m: 2,
    s: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricCapacitance`: `Qty<ELECTRIC_CAPACITANCE, N>` is one.
pub const ELECTRIC_CAPACITANCE: Dim = Dim {
    m: -2,
    s: 2,
    g: -1,
    C: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricCharge`: `Qty<ELECTRIC_CHARGE, N>` is one.
pub const ELECTRIC_CHARGE: Dim = Dim {
    C: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricConductance`: `Qty<ELECTRIC_CONDUCTANCE, N>` is one.
pub const ELECTRIC_CONDUCTANCE: Dim = Dim {
    m: -2,
    s: 1,
    g: -1,
    C: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricCurrent`: `Qty<ELECTRIC_CURRENT, N>` is one.
pub const ELECTRIC_CURRENT: Dim = Dim {
    s: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricPotential`: `Qty<ELECTRIC_POTENTIAL, N>` is one.
pub const ELECTRIC_POTENTIAL: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricResistance`: `Qty<ELECTRIC_RESISTANCE, N>` is one.
pub const ELECTRIC_RESISTANCE: Dim = Dim {
    m: 2,
    s: -1,
    g: 1,
    C: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Energy`: `Qty<ENERGY, N>` is one.
pub const ENERGY: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `EnergyDose`: `Qty<ENERGY_DOSE, N>` is one.
pub const ENERGY_DOSE: Dim = Dim {
    m: 2,
    s: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Force`: `Qty<FORCE, N>` is one.
pub const FORCE: Dim = Dim {
    m: 1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Frequency`: `Qty<FREQUENCY, N>` is one.
pub const FREQUENCY: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Illuminance`: `Qty<ILLUMINANCE, N>` is one.
pub const ILLUMINANCE: Dim = Dim {
    m: -2,
    rad: 2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Inductance`: `Qty<INDUCTANCE, N>` is one.
pub const INDUCTANCE: Dim = Dim {
    m: 2,
    g: 1,
    C: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Length`: `Qty<LENGTH, N>` is one.
pub const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `LuminousFlux`: `Qty<LUMINOUS_FLUX, N>` is one.
pub const LUMINOUS_FLUX: Dim = Dim {
    rad: 2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `LuminousIntensity`: `Qty<LUMINOUS_INTENSITY, N>` is one.
pub const LUMINOUS_INTENSITY: Dim = Dim {
    cd: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `MagneticFlux`: `Qty<MAGNETIC_FLUX, N>` is one.
pub const MAGNETIC_FLUX: Dim = Dim {
    m: 2,
    s: -1,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `MagneticFluxDensity`: `Qty<MAGNETIC_FLUX_DENSITY, N>` is one.
pub const MAGNETIC_FLUX_DENSITY: Dim = Dim {
    s: -1,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Mass`: `Qty<MASS, N>` is one.
pub const MASS: Dim = Dim {
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `PlaneAngle`: `Qty<PLANE_ANGLE, N>` is one.
pub const PLANE_ANGLE: Dim = Dim {
    rad: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Power`: `Qty<POWER, N>` is one.
pub const POWER: Dim = Dim {
    m: 2,
    s: -3,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Pressure`: `Qty<PRESSURE, N>` is one.
pub const PRESSURE: Dim = Dim {
    m: -1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Radioactivity`: `Qty<RADIOACTIVITY, N>` is one.
pub const RADIOACTIVITY: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `SolidAngle`: `Qty<SOLID_ANGLE, N>` is one.
pub const SOLID_ANGLE: Dim = Dim {
    rad: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Time`: `Qty<TIME, N>` is one.
pub const TIME: Dim = Dim {
    s: 1,
    ..Dim::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Mole (`mol`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` coherent units.
    pub const MOLE: UcumUnit<
        AMOUNT_OF_SUBSTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "mol",
                name: "mole",
            }
        },
    > = UcumUnit;

    /// Sievert (`Sv`), a unit of `DoseEquivalent`: `n` of them is `n · 1` coherent units.
    pub const SIEVERT: UcumUnit<
        DOSE_EQUIVALENT,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Sv",
                name: "sievert",
            }
        },
    > = UcumUnit;

    /// Farad (`F`), a unit of `ElectricCapacitance`: `n` of them is `n · 1e-3` coherent units.
    pub const FARAD: UcumUnit<
        ELECTRIC_CAPACITANCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "F",
                name: "farad",
            }
        },
    > = UcumUnit;

    /// Coulomb (`C`), a unit of `ElectricCharge`: `n` of them is `n · 1` coherent units.
    pub const COULOMB: UcumUnit<
        ELECTRIC_CHARGE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C",
                name: "coulomb",
            }
        },
    > = UcumUnit;

    /// Siemens (`S`), a unit of `ElectricConductance`: `n` of them is `n · 1e-3` coherent units.
    pub const SIEMENS: UcumUnit<
        ELECTRIC_CONDUCTANCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "S",
                name: "siemens",
            }
        },
    > = UcumUnit;

    /// Ampère (`A`), a unit of `ElectricCurrent`: `n` of them is `n · 1` coherent units.
    pub const AMPERE: UcumUnit<
        ELECTRIC_CURRENT,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A",
                name: "ampère",
            }
        },
    > = UcumUnit;

    /// Volt (`V`), a unit of `ElectricPotential`: `n` of them is `n · 1e3` coherent units.
    pub const VOLT: UcumUnit<
        ELECTRIC_POTENTIAL,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "V",
                name: "volt",
            }
        },
    > = UcumUnit;

    /// Ohm (`Ohm`), a unit of `ElectricResistance`: `n` of them is `n · 1e3` coherent units.
    pub const OHM: UcumUnit<
        ELECTRIC_RESISTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "Ohm",
                name: "ohm",
            }
        },
    > = UcumUnit;

    /// Joule (`J`), a unit of `Energy`: `n` of them is `n · 1e3` coherent units.
    pub const JOULE: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "J",
                name: "joule",
            }
        },
    > = UcumUnit;

    /// Gray (`Gy`), a unit of `EnergyDose`: `n` of them is `n · 1` coherent units.
    pub const GRAY: UcumUnit<
        ENERGY_DOSE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Gy",
                name: "gray",
            }
        },
    > = UcumUnit;

    /// Newton (`N`), a unit of `Force`: `n` of them is `n · 1e3` coherent units.
    pub const NEWTON: UcumUnit<
        FORCE,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "N",
                name: "newton",
            }
        },
    > = UcumUnit;

    /// Hertz (`Hz`), a unit of `Frequency`: `n` of them is `n · 1` coherent units.
    pub const HERTZ: UcumUnit<
        FREQUENCY,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Hz",
                name: "hertz",
            }
        },
    > = UcumUnit;

    /// Lux (`lx`), a unit of `Illuminance`: `n` of them is `n · 1` coherent units.
    pub const LUX: UcumUnit<
        ILLUMINANCE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "lx",
                name: "lux",
            }
        },
    > = UcumUnit;

    /// Henry (`H`), a unit of `Inductance`: `n` of them is `n · 1e3` coherent units.
    pub const HENRY: UcumUnit<
        INDUCTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "H",
                name: "henry",
            }
        },
    > = UcumUnit;

    /// Meter (`m`), a unit of `Length`: `n` of them is `n · 1` coherent units.
    pub const METER: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m",
                name: "meter",
            }
        },
    > = UcumUnit;

    /// Lumen (`lm`), a unit of `LuminousFlux`: `n` of them is `n · 1` coherent units.
    pub const LUMEN: UcumUnit<
        LUMINOUS_FLUX,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "lm",
                name: "lumen",
            }
        },
    > = UcumUnit;

    /// Candela (`cd`), a unit of `LuminousIntensity`: `n` of them is `n · 1` coherent units.
    pub const CANDELA: UcumUnit<
        LUMINOUS_INTENSITY,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "cd",
                name: "candela",
            }
        },
    > = UcumUnit;

    /// Weber (`Wb`), a unit of `MagneticFlux`: `n` of them is `n · 1e3` coherent units.
    pub const WEBER: UcumUnit<
        MAGNETIC_FLUX,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "Wb",
                name: "weber",
            }
        },
    > = UcumUnit;

    /// Tesla (`T`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e3` coherent units.
    pub const TESLA: UcumUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "T",
                name: "tesla",
            }
        },
    > = UcumUnit;

    /// Gram (`g`), a unit of `Mass`: `n` of them is `n · 1` coherent units.
    pub const GRAM: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "g",
                name: "gram",
            }
        },
    > = UcumUnit;

    /// Radian (`rad`), a unit of `PlaneAngle`: `n` of them is `n · 1` coherent units.
    pub const RADIAN: UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad",
                name: "radian",
            }
        },
    > = UcumUnit;

    /// Watt (`W`), a unit of `Power`: `n` of them is `n · 1e3` coherent units.
    pub const WATT: UcumUnit<
        POWER,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "W",
                name: "watt",
            }
        },
    > = UcumUnit;

    /// Pascal (`Pa`), a unit of `Pressure`: `n` of them is `n · 1e3` coherent units.
    pub const PASCAL: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "Pa",
                name: "pascal",
            }
        },
    > = UcumUnit;

    /// Becquerel (`Bq`), a unit of `Radioactivity`: `n` of them is `n · 1` coherent units.
    pub const BECQUEREL: UcumUnit<
        RADIOACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq",
                name: "becquerel",
            }
        },
    > = UcumUnit;

    /// Steradian (`sr`), a unit of `SolidAngle`: `n` of them is `n · 1` coherent units.
    pub const STERADIAN: UcumUnit<
        SOLID_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "sr",
                name: "steradian",
            }
        },
    > = UcumUnit;

    /// Second (`s`), a unit of `Time`: `n` of them is `n · 1` coherent units.
    pub const SECOND: UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "s",
                name: "second",
            }
        },
    > = UcumUnit;
}
