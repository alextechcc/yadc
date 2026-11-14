use crate::{Dim, Qty};
use yadc::Dimension;

pub type AmountOfSubstance<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

/// The dimension of `DoseEquivalent`.
pub const DOSE_EQUIVALENT: Dim = Dim {
    m: 2,
    s: -2,
    ..Dim::DIMENSIONLESS
};
pub type DoseEquivalent<N> = Qty<DOSE_EQUIVALENT, N>;

/// The dimension of `ElectricCapacitance`.
pub const ELECTRIC_CAPACITANCE: Dim = Dim {
    m: -2,
    s: 2,
    g: -1,
    C: 2,
    ..Dim::DIMENSIONLESS
};
pub type ElectricCapacitance<N> = Qty<ELECTRIC_CAPACITANCE, N>;

/// The dimension of `ElectricCharge`.
pub const ELECTRIC_CHARGE: Dim = Dim {
    C: 1,
    ..Dim::DIMENSIONLESS
};
pub type ElectricCharge<N> = Qty<ELECTRIC_CHARGE, N>;

/// The dimension of `ElectricConductance`.
pub const ELECTRIC_CONDUCTANCE: Dim = Dim {
    m: -2,
    s: 1,
    g: -1,
    C: 2,
    ..Dim::DIMENSIONLESS
};
pub type ElectricConductance<N> = Qty<ELECTRIC_CONDUCTANCE, N>;

/// The dimension of `ElectricCurrent`.
pub const ELECTRIC_CURRENT: Dim = Dim {
    s: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};
pub type ElectricCurrent<N> = Qty<ELECTRIC_CURRENT, N>;

/// The dimension of `ElectricPotential`.
pub const ELECTRIC_POTENTIAL: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};
pub type ElectricPotential<N> = Qty<ELECTRIC_POTENTIAL, N>;

/// The dimension of `ElectricResistance`.
pub const ELECTRIC_RESISTANCE: Dim = Dim {
    m: 2,
    s: -1,
    g: 1,
    C: -2,
    ..Dim::DIMENSIONLESS
};
pub type ElectricResistance<N> = Qty<ELECTRIC_RESISTANCE, N>;

/// The dimension of `Energy`.
pub const ENERGY: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Energy<N> = Qty<ENERGY, N>;

/// The dimension of `EnergyDose`.
pub const ENERGY_DOSE: Dim = Dim {
    m: 2,
    s: -2,
    ..Dim::DIMENSIONLESS
};
pub type EnergyDose<N> = Qty<ENERGY_DOSE, N>;

/// The dimension of `Force`.
pub const FORCE: Dim = Dim {
    m: 1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Force<N> = Qty<FORCE, N>;

/// The dimension of `Frequency`.
pub const FREQUENCY: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};
pub type Frequency<N> = Qty<FREQUENCY, N>;

/// The dimension of `Illuminance`.
pub const ILLUMINANCE: Dim = Dim {
    m: -2,
    rad: 2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};
pub type Illuminance<N> = Qty<ILLUMINANCE, N>;

/// The dimension of `Inductance`.
pub const INDUCTANCE: Dim = Dim {
    m: 2,
    g: 1,
    C: -2,
    ..Dim::DIMENSIONLESS
};
pub type Inductance<N> = Qty<INDUCTANCE, N>;

/// The dimension of `Length`.
pub const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};
pub type Length<N> = Qty<LENGTH, N>;

/// The dimension of `LuminousFlux`.
pub const LUMINOUS_FLUX: Dim = Dim {
    rad: 2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};
pub type LuminousFlux<N> = Qty<LUMINOUS_FLUX, N>;

/// The dimension of `LuminousIntensity`.
pub const LUMINOUS_INTENSITY: Dim = Dim {
    cd: 1,
    ..Dim::DIMENSIONLESS
};
pub type LuminousIntensity<N> = Qty<LUMINOUS_INTENSITY, N>;

/// The dimension of `MagneticFlux`.
pub const MAGNETIC_FLUX: Dim = Dim {
    m: 2,
    s: -1,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};
pub type MagneticFlux<N> = Qty<MAGNETIC_FLUX, N>;

/// The dimension of `MagneticFluxDensity`.
pub const MAGNETIC_FLUX_DENSITY: Dim = Dim {
    s: -1,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};
pub type MagneticFluxDensity<N> = Qty<MAGNETIC_FLUX_DENSITY, N>;

/// The dimension of `Mass`.
pub const MASS: Dim = Dim {
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Mass<N> = Qty<MASS, N>;

/// The dimension of `PlaneAngle`.
pub const PLANE_ANGLE: Dim = Dim {
    rad: 1,
    ..Dim::DIMENSIONLESS
};
pub type PlaneAngle<N> = Qty<PLANE_ANGLE, N>;

/// The dimension of `Power`.
pub const POWER: Dim = Dim {
    m: 2,
    s: -3,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Power<N> = Qty<POWER, N>;

/// The dimension of `Pressure`.
pub const PRESSURE: Dim = Dim {
    m: -1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Pressure<N> = Qty<PRESSURE, N>;

/// The dimension of `Radioactivity`.
pub const RADIOACTIVITY: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};
pub type Radioactivity<N> = Qty<RADIOACTIVITY, N>;

/// The dimension of `SolidAngle`.
pub const SOLID_ANGLE: Dim = Dim {
    rad: 2,
    ..Dim::DIMENSIONLESS
};
pub type SolidAngle<N> = Qty<SOLID_ANGLE, N>;

/// The dimension of `Time`.
pub const TIME: Dim = Dim {
    s: 1,
    ..Dim::DIMENSIONLESS
};
pub type Time<N> = Qty<TIME, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Dim, Exact, UcumUnit, UcumUnitConsts};
    use yadc::Dimension;

    /// Mole (`mol`), a unit of `AmountOfSubstance`: `n` of them is `n · 602214076e15` coherent units.
    pub type Mole = UcumUnit<
        { Dim::DIMENSIONLESS },
        {
            UcumUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "mol",
                name: "mole",
            }
        },
    >;

    /// Sievert (`Sv`), a unit of `DoseEquivalent`: `n` of them is `n · 1` coherent units.
    pub type Sievert = UcumUnit<
        DOSE_EQUIVALENT,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Sv",
                name: "sievert",
            }
        },
    >;

    /// Farad (`F`), a unit of `ElectricCapacitance`: `n` of them is `n · 1e-3` coherent units.
    pub type Farad = UcumUnit<
        ELECTRIC_CAPACITANCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "F",
                name: "farad",
            }
        },
    >;

    /// Coulomb (`C`), a unit of `ElectricCharge`: `n` of them is `n · 1` coherent units.
    pub type Coulomb = UcumUnit<
        ELECTRIC_CHARGE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "C",
                name: "coulomb",
            }
        },
    >;

    /// Siemens (`S`), a unit of `ElectricConductance`: `n` of them is `n · 1e-3` coherent units.
    pub type Siemens = UcumUnit<
        ELECTRIC_CONDUCTANCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "S",
                name: "siemens",
            }
        },
    >;

    /// Ampère (`A`), a unit of `ElectricCurrent`: `n` of them is `n · 1` coherent units.
    pub type Ampere = UcumUnit<
        ELECTRIC_CURRENT,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "A",
                name: "ampère",
            }
        },
    >;

    /// Volt (`V`), a unit of `ElectricPotential`: `n` of them is `n · 1e3` coherent units.
    pub type Volt = UcumUnit<
        ELECTRIC_POTENTIAL,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "V",
                name: "volt",
            }
        },
    >;

    /// Ohm (`Ohm`), a unit of `ElectricResistance`: `n` of them is `n · 1e3` coherent units.
    pub type Ohm = UcumUnit<
        ELECTRIC_RESISTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "Ohm",
                name: "ohm",
            }
        },
    >;

    /// Joule (`J`), a unit of `Energy`: `n` of them is `n · 1e3` coherent units.
    pub type Joule = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "J",
                name: "joule",
            }
        },
    >;

    /// Gray (`Gy`), a unit of `EnergyDose`: `n` of them is `n · 1` coherent units.
    pub type Gray = UcumUnit<
        ENERGY_DOSE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Gy",
                name: "gray",
            }
        },
    >;

    /// Newton (`N`), a unit of `Force`: `n` of them is `n · 1e3` coherent units.
    pub type Newton = UcumUnit<
        FORCE,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "N",
                name: "newton",
            }
        },
    >;

    /// Hertz (`Hz`), a unit of `Frequency`: `n` of them is `n · 1` coherent units.
    pub type Hertz = UcumUnit<
        FREQUENCY,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Hz",
                name: "hertz",
            }
        },
    >;

    /// Lux (`lx`), a unit of `Illuminance`: `n` of them is `n · 1` coherent units.
    pub type Lux = UcumUnit<
        ILLUMINANCE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "lx",
                name: "lux",
            }
        },
    >;

    /// Henry (`H`), a unit of `Inductance`: `n` of them is `n · 1e3` coherent units.
    pub type Henry = UcumUnit<
        INDUCTANCE,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "H",
                name: "henry",
            }
        },
    >;

    /// Meter (`m`), a unit of `Length`: `n` of them is `n · 1` coherent units.
    pub type Meter = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m",
                name: "meter",
            }
        },
    >;

    /// Lumen (`lm`), a unit of `LuminousFlux`: `n` of them is `n · 1` coherent units.
    pub type Lumen = UcumUnit<
        LUMINOUS_FLUX,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "lm",
                name: "lumen",
            }
        },
    >;

    /// Candela (`cd`), a unit of `LuminousIntensity`: `n` of them is `n · 1` coherent units.
    pub type Candela = UcumUnit<
        LUMINOUS_INTENSITY,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "cd",
                name: "candela",
            }
        },
    >;

    /// Weber (`Wb`), a unit of `MagneticFlux`: `n` of them is `n · 1e3` coherent units.
    pub type Weber = UcumUnit<
        MAGNETIC_FLUX,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "Wb",
                name: "weber",
            }
        },
    >;

    /// Tesla (`T`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e3` coherent units.
    pub type Tesla = UcumUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "T",
                name: "tesla",
            }
        },
    >;

    /// Gram (`g`), a unit of `Mass`: `n` of them is `n · 1` coherent units.
    pub type Gram = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "g",
                name: "gram",
            }
        },
    >;

    /// Radian (`rad`), a unit of `PlaneAngle`: `n` of them is `n · 1` coherent units.
    pub type Radian = UcumUnit<
        PLANE_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad",
                name: "radian",
            }
        },
    >;

    /// Watt (`W`), a unit of `Power`: `n` of them is `n · 1e3` coherent units.
    pub type Watt = UcumUnit<
        POWER,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "W",
                name: "watt",
            }
        },
    >;

    /// Pascal (`Pa`), a unit of `Pressure`: `n` of them is `n · 1e3` coherent units.
    pub type Pascal = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "Pa",
                name: "pascal",
            }
        },
    >;

    /// Becquerel (`Bq`), a unit of `Radioactivity`: `n` of them is `n · 1` coherent units.
    pub type Becquerel = UcumUnit<
        RADIOACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq",
                name: "becquerel",
            }
        },
    >;

    /// Steradian (`sr`), a unit of `SolidAngle`: `n` of them is `n · 1` coherent units.
    pub type Steradian = UcumUnit<
        SOLID_ANGLE,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "sr",
                name: "steradian",
            }
        },
    >;

    /// Second (`s`), a unit of `Time`: `n` of them is `n · 1` coherent units.
    pub type Second = UcumUnit<
        TIME,
        {
            UcumUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "s",
                name: "second",
            }
        },
    >;
}
