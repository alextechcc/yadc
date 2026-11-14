use crate::{ISQ, Kind};
use yadc::Dimension;

/// The dimension of `Length`: `SIQuantity<LENGTH, N>` is one.
pub const LENGTH: ISQ = ISQ {
    m: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Mass`: `SIQuantity<MASS, N>` is one.
pub const MASS: ISQ = ISQ {
    kg: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Time`: `SIQuantity<TIME, N>` is one.
pub const TIME: ISQ = ISQ {
    s: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Instant`: `SIQuantity<INSTANT, N>` is one.
pub const INSTANT: ISQ = ISQ {
    s: 1,
    kind: Kind::Affine,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Area`: `SIQuantity<AREA, N>` is one.
pub const AREA: ISQ = ISQ {
    m: 2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Volume`: `SIQuantity<VOLUME, N>` is one.
pub const VOLUME: ISQ = ISQ {
    m: 3,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Velocity`: `SIQuantity<VELOCITY, N>` is one.
pub const VELOCITY: ISQ = ISQ {
    m: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Acceleration`: `SIQuantity<ACCELERATION, N>` is one.
pub const ACCELERATION: ISQ = ISQ {
    m: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Jerk`: `SIQuantity<JERK, N>` is one.
pub const JERK: ISQ = ISQ {
    m: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Absement`: `SIQuantity<ABSEMENT, N>` is one.
pub const ABSEMENT: ISQ = ISQ {
    m: 1,
    s: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `InverseVelocity`: `SIQuantity<INVERSE_VELOCITY, N>` is one.
pub const INVERSE_VELOCITY: ISQ = ISQ {
    m: -1,
    s: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Force`: `SIQuantity<FORCE, N>` is one.
pub const FORCE: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Momentum`: `SIQuantity<MOMENTUM, N>` is one.
pub const MOMENTUM: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Energy`: `SIQuantity<ENERGY, N>` is one.
pub const ENERGY: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `AvailableEnergy`: `SIQuantity<AVAILABLE_ENERGY, N>` is one.
pub const AVAILABLE_ENERGY: ISQ = ISQ {
    m: 2,
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Power`: `SIQuantity<POWER, N>` is one.
pub const POWER: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Gain`: `SIQuantity<GAIN, N>` is one.
pub const GAIN: ISQ = ISQ {
    kind: Kind::Log,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `PowerLevel`: `SIQuantity<POWER_LEVEL, N>` is one.
pub const POWER_LEVEL: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    kind: Kind::Log,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `PowerRate`: `SIQuantity<POWER_RATE, N>` is one.
pub const POWER_RATE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -4,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Action`: `SIQuantity<ACTION, N>` is one.
pub const ACTION: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Torque`: `SIQuantity<TORQUE, N>` is one.
pub const TORQUE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    rad: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MomentOfInertia`: `SIQuantity<MOMENT_OF_INERTIA, N>` is one.
pub const MOMENT_OF_INERTIA: ISQ = ISQ {
    m: 2,
    kg: 1,
    rad: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Pressure`: `SIQuantity<PRESSURE, N>` is one.
pub const PRESSURE: ISQ = ISQ {
    m: -1,
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Frequency`: `SIQuantity<FREQUENCY, N>` is one.
pub const FREQUENCY: ISQ = ISQ {
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `FrequencyDrift`: `SIQuantity<FREQUENCY_DRIFT, N>` is one.
pub const FREQUENCY_DRIFT: ISQ = ISQ {
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `ReciprocalLength`: `SIQuantity<RECIPROCAL_LENGTH, N>` is one.
pub const RECIPROCAL_LENGTH: ISQ = ISQ {
    m: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Curvature`: `SIQuantity<CURVATURE, N>` is one.
pub const CURVATURE: ISQ = ISQ {
    m: -1,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SpecificArea`: `SIQuantity<SPECIFIC_AREA, N>` is one.
pub const SPECIFIC_AREA: ISQ = ISQ {
    m: 2,
    kg: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SpecificVolume`: `SIQuantity<SPECIFIC_VOLUME, N>` is one.
pub const SPECIFIC_VOLUME: ISQ = ISQ {
    m: 3,
    kg: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SpecificPower`: `SIQuantity<SPECIFIC_POWER, N>` is one.
pub const SPECIFIC_POWER: ISQ = ISQ {
    m: 2,
    s: -3,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SpecificGravitationalParameter`: `SIQuantity<SPECIFIC_GRAVITATIONAL_PARAMETER, N>` is one.
pub const SPECIFIC_GRAVITATIONAL_PARAMETER: ISQ = ISQ {
    m: 3,
    kg: -1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Metres (`m`), a unit of `Length`: `n` of them is `n · 1.0` coherent units.
    pub const METRE: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m",
                name: "metres",
            }
        },
    > = SIUnit;

    /// Ångströms (`Å`), a unit of `Length`: `n` of them is `n · 1e-10` coherent units.
    pub const ANGSTROM: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1, -10),
                zero: Exact::ZERO,
                symbol: "Å",
                name: "ångströms",
            }
        },
    > = SIUnit;

    /// Bohr radiuses (`a₀`), a unit of `Length`: `n` of them is `n · 5.29177210903e-11` coherent units.
    pub const BOHR_RADIUS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(529177210903, -22),
                zero: Exact::ZERO,
                symbol: "a₀",
                name: "bohr radiuses",
            }
        },
    > = SIUnit;

    /// Atomic units of length (`a.u. of length`), a unit of `Length`: `n` of them is `n · 5.29177210903e-11` coherent units.
    pub const ATOMIC_UNIT_OF_LENGTH: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(529177210903, -22),
                zero: Exact::ZERO,
                symbol: "a.u. of length",
                name: "atomic units of length",
            }
        },
    > = SIUnit;

    /// Astronomical units (`ua`), a unit of `Length`: `n` of them is `n · 149597870700.0` coherent units.
    pub const ASTRONOMICAL_UNIT: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(149597870700),
                zero: Exact::ZERO,
                symbol: "ua",
                name: "astronomical units",
            }
        },
    > = SIUnit;

    /// Chains (`ch`), a unit of `Length`: `n` of them is `n · 66.0 * 1200.0 / 3937.0` coherent units.
    pub const CHAIN: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(66) * Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "ch",
                name: "chains",
            }
        },
    > = SIUnit;

    /// Data miles (`DM`), a unit of `Length`: `n` of them is `n · 1828.8` coherent units.
    pub const DATA_MILE: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(18288, -1),
                zero: Exact::ZERO,
                symbol: "DM",
                name: "data miles",
            }
        },
    > = SIUnit;

    /// Fathoms (`fathom`), a unit of `Length`: `n` of them is `n · 6.0 * 1200.0 / 3937.0` coherent units.
    pub const FATHOM: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(6) * Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "fathom",
                name: "fathoms",
            }
        },
    > = SIUnit;

    /// Fermis (`fermi`), a unit of `Length`: `n` of them is `n · 1e-15` coherent units.
    pub const FERMI: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1, -15),
                zero: Exact::ZERO,
                symbol: "fermi",
                name: "fermis",
            }
        },
    > = SIUnit;

    /// Feet (`ft`), a unit of `Length`: `n` of them is `n · 0.3048` coherent units.
    pub const FOOT: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft",
                name: "feet",
            }
        },
    > = SIUnit;

    /// Feet (U.S. survey) (`ft (U.S. survey)`), a unit of `Length`: `n` of them is `n · 1200.0 / 3937.0` coherent units.
    pub const FOOT_SURVEY: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "ft (U.S. survey)",
                name: "feet (U.S. survey)",
            }
        },
    > = SIUnit;

    /// Inches (`in`), a unit of `Length`: `n` of them is `n · 0.0254` coherent units.
    pub const INCH: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in",
                name: "inches",
            }
        },
    > = SIUnit;

    /// Light years (`l. y.`), a unit of `Length`: `n` of them is `n · 9460730472580800.0` coherent units.
    pub const LIGHT_YEAR: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(9460730472580800),
                zero: Exact::ZERO,
                symbol: "l. y.",
                name: "light years",
            }
        },
    > = SIUnit;

    /// Microns (`μ`), a unit of `Length`: `n` of them is `n · 1e-06` coherent units.
    pub const MICRON: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "μ",
                name: "microns",
            }
        },
    > = SIUnit;

    /// Mils (`0.001 in`), a unit of `Length`: `n` of them is `n · 2.54e-05` coherent units.
    pub const MIL: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(254, -7),
                zero: Exact::ZERO,
                symbol: "0.001 in",
                name: "mils",
            }
        },
    > = SIUnit;

    /// Miles (`mi`), a unit of `Length`: `n` of them is `n · 1609.344` coherent units.
    pub const MILE: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi",
                name: "miles",
            }
        },
    > = SIUnit;

    /// Miles (U.S. survey) (`mi (U.S. survey)`), a unit of `Length`: `n` of them is `n · 6336000.0 / 3937.0` coherent units.
    pub const MILE_SURVEY: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(6336000) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "mi (U.S. survey)",
                name: "miles (U.S. survey)",
            }
        },
    > = SIUnit;

    /// Nautical miles (`M`), a unit of `Length`: `n` of them is `n · 1852.0` coherent units.
    pub const NAUTICAL_MILE: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1852),
                zero: Exact::ZERO,
                symbol: "M",
                name: "nautical miles",
            }
        },
    > = SIUnit;

    /// Parsecs (`pc`), a unit of `Length`: `n` of them is `n · 149597870700.0 * 648000.0 / core::f64::consts::PI` coherent units.
    pub const PARSEC: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(149597870700) * Exact::int(648000) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "pc",
                name: "parsecs",
            }
        },
    > = SIUnit;

    /// Picas (computer) (`1/6 in (computer)`), a unit of `Length`: `n` of them is `n · 0.0254 / 6.0` coherent units.
    pub const PICA_COMPUTER: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "1/6 in (computer)",
                name: "picas (computer)",
            }
        },
    > = SIUnit;

    /// Picas (printer's) (`1/6 in`), a unit of `Length`: `n` of them is `n · 0.004217518` coherent units.
    pub const PICA_PRINTERS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(4217518, -9),
                zero: Exact::ZERO,
                symbol: "1/6 in",
                name: "picas (printer's)",
            }
        },
    > = SIUnit;

    /// Points (computer) (`1/72 in (computer)`), a unit of `Length`: `n` of them is `n · 0.0254 / 72.0` coherent units.
    pub const POINT_COMPUTER: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) / Exact::int(72),
                zero: Exact::ZERO,
                symbol: "1/72 in (computer)",
                name: "points (computer)",
            }
        },
    > = SIUnit;

    /// Points (printer's) (`1/72 in`), a unit of `Length`: `n` of them is `n · 0.0003514598` coherent units.
    pub const POINT_PRINTERS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(3514598, -10),
                zero: Exact::ZERO,
                symbol: "1/72 in",
                name: "points (printer's)",
            }
        },
    > = SIUnit;

    /// Rods (`rd`), a unit of `Length`: `n` of them is `n · 16.5 * 1200.0 / 3937.0` coherent units.
    pub const ROD: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(165, -1) * Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "rd",
                name: "rods",
            }
        },
    > = SIUnit;

    /// Yards (`yd`), a unit of `Length`: `n` of them is `n · 0.9144` coherent units.
    pub const YARD: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd",
                name: "yards",
            }
        },
    > = SIUnit;

    /// Leugae (`leuga`), a unit of `Length`: `n` of them is `n · 2220.0` coherent units.
    pub const LEUGA: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(2220),
                zero: Exact::ZERO,
                symbol: "leuga",
                name: "leugae",
            }
        },
    > = SIUnit;

    /// Mille passa (`mille passus`), a unit of `Length`: `n` of them is `n · 1480.0` coherent units.
    pub const MILLE_PASSUS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1480),
                zero: Exact::ZERO,
                symbol: "mille passus",
                name: "mille passa",
            }
        },
    > = SIUnit;

    /// Stadia (`stadium`), a unit of `Length`: `n` of them is `n · 185.0` coherent units.
    pub const STADIUM: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(185),
                zero: Exact::ZERO,
                symbol: "stadium",
                name: "stadia",
            }
        },
    > = SIUnit;

    /// Acta (`actus`), a unit of `Length`: `n` of them is `n · 35.52` coherent units.
    pub const ACTUS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(3552, -2),
                zero: Exact::ZERO,
                symbol: "actus",
                name: "acta",
            }
        },
    > = SIUnit;

    /// Decempedae (`decempeda`), a unit of `Length`: `n` of them is `n · 2.96` coherent units.
    pub const DECEMPEDA: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(296, -2),
                zero: Exact::ZERO,
                symbol: "decempeda",
                name: "decempedae",
            }
        },
    > = SIUnit;

    /// Passa (`passus`), a unit of `Length`: `n` of them is `n · 1.48` coherent units.
    pub const PASSUS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(148, -2),
                zero: Exact::ZERO,
                symbol: "passus",
                name: "passa",
            }
        },
    > = SIUnit;

    /// Gradus (`gradus`), a unit of `Length`: `n` of them is `n · 0.74` coherent units.
    pub const GRADUS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(74, -2),
                zero: Exact::ZERO,
                symbol: "gradus",
                name: "gradus",
            }
        },
    > = SIUnit;

    /// Cubita (`cubitum`), a unit of `Length`: `n` of them is `n · 0.444` coherent units.
    pub const CUBITUM: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(444, -3),
                zero: Exact::ZERO,
                symbol: "cubitum",
                name: "cubita",
            }
        },
    > = SIUnit;

    /// Palmipedes (`palmipes`), a unit of `Length`: `n` of them is `n · 0.37` coherent units.
    pub const PALMIPES: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(37, -2),
                zero: Exact::ZERO,
                symbol: "palmipes",
                name: "palmipedes",
            }
        },
    > = SIUnit;

    /// Pedes (`pes`), a unit of `Length`: `n` of them is `n · 0.296` coherent units.
    pub const PES: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(296, -3),
                zero: Exact::ZERO,
                symbol: "pes",
                name: "pedes",
            }
        },
    > = SIUnit;

    /// Palmas maior (`palmus maior`), a unit of `Length`: `n` of them is `n · 0.222` coherent units.
    pub const PALMUS_MAIOR: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(222, -3),
                zero: Exact::ZERO,
                symbol: "palmus maior",
                name: "palmas maior",
            }
        },
    > = SIUnit;

    /// Palmas (`palmus`), a unit of `Length`: `n` of them is `n · 0.074` coherent units.
    pub const PALMUS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(74, -3),
                zero: Exact::ZERO,
                symbol: "palmus",
                name: "palmas",
            }
        },
    > = SIUnit;

    /// Unciae (`uncia`), a unit of `Length`: `n` of them is `n · 0.296 / 12.0` coherent units.
    pub const LENGTH_UNCIA: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(296, -3) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "uncia",
                name: "unciae",
            }
        },
    > = SIUnit;

    /// Digiti (`digitus`), a unit of `Length`: `n` of them is `n · 0.0185` coherent units.
    pub const DIGITUS: SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(185, -4),
                zero: Exact::ZERO,
                symbol: "digitus",
                name: "digiti",
            }
        },
    > = SIUnit;

    /// Kilograms (`kg`), a unit of `Mass`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg",
                name: "kilograms",
            }
        },
    > = SIUnit;

    /// Grams (`g`), a unit of `Mass`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g",
                name: "grams",
            }
        },
    > = SIUnit;

    /// Carats (`ct`), a unit of `Mass`: `n` of them is `n · 0.0002` coherent units.
    pub const CARAT: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(2, -4),
                zero: Exact::ZERO,
                symbol: "ct",
                name: "carats",
            }
        },
    > = SIUnit;

    /// Daltons (`Da`), a unit of `Mass`: `n` of them is `n · 1.6605390666e-27` coherent units.
    pub const DALTON: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(16605390666, -37),
                zero: Exact::ZERO,
                symbol: "Da",
                name: "daltons",
            }
        },
    > = SIUnit;

    /// Electron masses (`mₑ`), a unit of `Mass`: `n` of them is `n · 9.1093837015e-31` coherent units.
    pub const ELECTRON_MASS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(91093837015, -41),
                zero: Exact::ZERO,
                symbol: "mₑ",
                name: "electron masses",
            }
        },
    > = SIUnit;

    /// Proton masses (`mₚ`), a unit of `Mass`: `n` of them is `n · 1.67262192369e-27` coherent units.
    pub const PROTON_MASS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(167262192369, -38),
                zero: Exact::ZERO,
                symbol: "mₚ",
                name: "proton masses",
            }
        },
    > = SIUnit;

    /// Grains (`gr`), a unit of `Mass`: `n` of them is `n · 6.479891e-05` coherent units.
    pub const GRAIN: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "gr",
                name: "grains",
            }
        },
    > = SIUnit;

    /// Hundredweight (long) (`cwt long`), a unit of `Mass`: `n` of them is `n · 50.80234544` coherent units.
    pub const HUNDREDWEIGHT_LONG: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(5080234544, -8),
                zero: Exact::ZERO,
                symbol: "cwt long",
                name: "hundredweight (long)",
            }
        },
    > = SIUnit;

    /// Hundredweight (short) (`cwt short`), a unit of `Mass`: `n` of them is `n · 45.359237` coherent units.
    pub const HUNDREDWEIGHT_SHORT: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -6),
                zero: Exact::ZERO,
                symbol: "cwt short",
                name: "hundredweight (short)",
            }
        },
    > = SIUnit;

    /// Ounces (`oz`), a unit of `Mass`: `n` of them is `n · 0.028349523125` coherent units.
    pub const OUNCE: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "oz",
                name: "ounces",
            }
        },
    > = SIUnit;

    /// Troy ounces (`oz t`), a unit of `Mass`: `n` of them is `n · 0.0311034768` coherent units.
    pub const OUNCE_TROY: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(311034768, -10),
                zero: Exact::ZERO,
                symbol: "oz t",
                name: "troy ounces",
            }
        },
    > = SIUnit;

    /// Pennyweight (`dwt`), a unit of `Mass`: `n` of them is `n · 0.00155517384` coherent units.
    pub const PENNYWEIGHT: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(155517384, -11),
                zero: Exact::ZERO,
                symbol: "dwt",
                name: "pennyweight",
            }
        },
    > = SIUnit;

    /// Pounds (`lb`), a unit of `Mass`: `n` of them is `n · 0.45359237` coherent units.
    pub const POUND: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "lb",
                name: "pounds",
            }
        },
    > = SIUnit;

    /// Troy pounds (`lb t`), a unit of `Mass`: `n` of them is `n · 0.3732417216` coherent units.
    pub const POUND_TROY: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3732417216, -10),
                zero: Exact::ZERO,
                symbol: "lb t",
                name: "troy pounds",
            }
        },
    > = SIUnit;

    /// Slugs (`slug`), a unit of `Mass`: `n` of them is `n · 0.45359237 * 9.80665 / 0.3048` coherent units.
    pub const SLUG: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "slug",
                name: "slugs",
            }
        },
    > = SIUnit;

    /// Assay tons (`AT`), a unit of `Mass`: `n` of them is `n · 0.175 / 6.0` coherent units.
    pub const TON_ASSAY: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(175, -3) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "AT",
                name: "assay tons",
            }
        },
    > = SIUnit;

    /// Long tons (`2240 lb`), a unit of `Mass`: `n` of them is `n · 1016.0469088` coherent units.
    pub const TON_LONG: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "2240 lb",
                name: "long tons",
            }
        },
    > = SIUnit;

    /// Short tons (`2000 lb`), a unit of `Mass`: `n` of them is `n · 907.18474` coherent units.
    pub const TON_SHORT: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "2000 lb",
                name: "short tons",
            }
        },
    > = SIUnit;

    /// Tons (`t`), a unit of `Mass`: `n` of them is `n · 1000.0` coherent units.
    pub const TON: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "t",
                name: "tons",
            }
        },
    > = SIUnit;

    /// Librae (`libra`), a unit of `Mass`: `n` of them is `n · 0.3289` coherent units.
    pub const LIBRA: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4),
                zero: Exact::ZERO,
                symbol: "libra",
                name: "librae",
            }
        },
    > = SIUnit;

    /// Deunces (`deunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 11.0 / 12.0` coherent units.
    pub const MASS_DEUNX: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(11) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "deunx",
                name: "deunces",
            }
        },
    > = SIUnit;

    /// Dextantes (`dextans`), a unit of `Mass`: `n` of them is `n · 0.3289 * 10.0 / 12.0` coherent units.
    pub const MASS_DEXTANS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(10) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "dextans",
                name: "dextantes",
            }
        },
    > = SIUnit;

    /// Dodrantes (`dodrans`), a unit of `Mass`: `n` of them is `n · 0.246675` coherent units.
    pub const MASS_DODRANS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(246675, -6),
                zero: Exact::ZERO,
                symbol: "dodrans",
                name: "dodrantes",
            }
        },
    > = SIUnit;

    /// Bessis (`bes`), a unit of `Mass`: `n` of them is `n · 0.3289 * 8.0 / 12.0` coherent units.
    pub const MASS_BES: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(8) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "bes",
                name: "bessis",
            }
        },
    > = SIUnit;

    /// Septunces (`septunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 7.0 / 12.0` coherent units.
    pub const MASS_SEPTUNX: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(7) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "septunx",
                name: "septunces",
            }
        },
    > = SIUnit;

    /// Semisses (`semis`), a unit of `Mass`: `n` of them is `n · 0.16445` coherent units.
    pub const MASS_SEMIS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(16445, -5),
                zero: Exact::ZERO,
                symbol: "semis",
                name: "semisses",
            }
        },
    > = SIUnit;

    /// Quincunx (`quincunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 5.0 / 12.0` coherent units.
    pub const MASS_QUINCUNX: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(5) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "quincunx",
                name: "quincunx",
            }
        },
    > = SIUnit;

    /// Trientes (`triens`), a unit of `Mass`: `n` of them is `n · 0.3289 / 3.0` coherent units.
    pub const MASS_TRIENS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(3),
                zero: Exact::ZERO,
                symbol: "triens",
                name: "trientes",
            }
        },
    > = SIUnit;

    /// Quadrantes teruncius (`quadrans teruncius`), a unit of `Mass`: `n` of them is `n · 0.082225` coherent units.
    pub const QUADRANS_TERUNCIUS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(82225, -6),
                zero: Exact::ZERO,
                symbol: "quadrans teruncius",
                name: "quadrantes teruncius",
            }
        },
    > = SIUnit;

    /// Sextantes (`sextans`), a unit of `Mass`: `n` of them is `n · 0.3289 / 6.0` coherent units.
    pub const MASS_SEXTANS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "sextans",
                name: "sextantes",
            }
        },
    > = SIUnit;

    /// Sescunciae (`sescuncia`), a unit of `Mass`: `n` of them is `n · 0.0411125` coherent units.
    pub const SESCUNCIA: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(411125, -7),
                zero: Exact::ZERO,
                symbol: "sescuncia",
                name: "sescunciae",
            }
        },
    > = SIUnit;

    /// Unciae (`uncia`), a unit of `Mass`: `n` of them is `n · 0.3289 / 12.0` coherent units.
    pub const MASS_UNCIA: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "uncia",
                name: "unciae",
            }
        },
    > = SIUnit;

    /// Semunciae (`semuncia`), a unit of `Mass`: `n` of them is `n · 0.3289 / 24.0` coherent units.
    pub const SEMUNCIA: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(24),
                zero: Exact::ZERO,
                symbol: "semuncia",
                name: "semunciae",
            }
        },
    > = SIUnit;

    /// Duella (`duella`), a unit of `Mass`: `n` of them is `n · 0.3289 / 36.0` coherent units.
    pub const DUELLA: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(36),
                zero: Exact::ZERO,
                symbol: "duella",
                name: "duella",
            }
        },
    > = SIUnit;

    /// Scilici (`sicilicus`), a unit of `Mass`: `n` of them is `n · 0.3289 / 48.0` coherent units.
    pub const MASS_SICILICUS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(48),
                zero: Exact::ZERO,
                symbol: "sicilicus",
                name: "scilici",
            }
        },
    > = SIUnit;

    /// Sextulae (`sextula`), a unit of `Mass`: `n` of them is `n · 0.3289 / 72.0` coherent units.
    pub const MASS_SEXTULA: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(72),
                zero: Exact::ZERO,
                symbol: "sextula",
                name: "sextulae",
            }
        },
    > = SIUnit;

    /// Semisextulae (`semisextula`), a unit of `Mass`: `n` of them is `n · 0.3289 / 144.0` coherent units.
    pub const SEMISEXTULA: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(144),
                zero: Exact::ZERO,
                symbol: "semisextula",
                name: "semisextulae",
            }
        },
    > = SIUnit;

    /// Scrupula (`scrupulum`), a unit of `Mass`: `n` of them is `n · 0.3289 / 288.0` coherent units.
    pub const MASS_SCRUPULUM: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(288),
                zero: Exact::ZERO,
                symbol: "scrupulum",
                name: "scrupula",
            }
        },
    > = SIUnit;

    /// Oboli (`obolus`), a unit of `Mass`: `n` of them is `n · 0.3289 / 576.0` coherent units.
    pub const OBOLUS: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(576),
                zero: Exact::ZERO,
                symbol: "obolus",
                name: "oboli",
            }
        },
    > = SIUnit;

    /// Siliquae (`siliqua`), a unit of `Mass`: `n` of them is `n · 0.3289 / 1728.0` coherent units.
    pub const SILIQUA: SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(1728),
                zero: Exact::ZERO,
                symbol: "siliqua",
                name: "siliquae",
            }
        },
    > = SIUnit;

    /// Seconds (`s`), a unit of `Time`: `n` of them is `n · 1.0` coherent units.
    pub const SECOND: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "s",
                name: "seconds",
            }
        },
    > = SIUnit;

    /// Seconds (sidereal) (`s (sidereal)`), a unit of `Time`: `n` of them is `n · 0.9972696` coherent units.
    pub const SECOND_SIDEREAL: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::dec(9972696, -7),
                zero: Exact::ZERO,
                symbol: "s (sidereal)",
                name: "seconds (sidereal)",
            }
        },
    > = SIUnit;

    /// Days (`d`), a unit of `Time`: `n` of them is `n · 86400.0` coherent units.
    pub const DAY: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "d",
                name: "days",
            }
        },
    > = SIUnit;

    /// Days (sidereal) (`d (sidereal)`), a unit of `Time`: `n` of them is `n · 86164.09` coherent units.
    pub const DAY_SIDEREAL: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::dec(8616409, -2),
                zero: Exact::ZERO,
                symbol: "d (sidereal)",
                name: "days (sidereal)",
            }
        },
    > = SIUnit;

    /// Hours (`h`), a unit of `Time`: `n` of them is `n · 3600.0` coherent units.
    pub const HOUR: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "h",
                name: "hours",
            }
        },
    > = SIUnit;

    /// Hours (sidereal) (`h (sidereal)`), a unit of `Time`: `n` of them is `n · 3590.17` coherent units.
    pub const HOUR_SIDEREAL: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::dec(359017, -2),
                zero: Exact::ZERO,
                symbol: "h (sidereal)",
                name: "hours (sidereal)",
            }
        },
    > = SIUnit;

    /// Minutes (`min`), a unit of `Time`: `n` of them is `n · 60.0` coherent units.
    pub const MINUTE: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(60),
                zero: Exact::ZERO,
                symbol: "min",
                name: "minutes",
            }
        },
    > = SIUnit;

    /// Shakes (`10.0 ns`), a unit of `Time`: `n` of them is `n · 1e-08` coherent units.
    pub const SHAKE: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::dec(1, -8),
                zero: Exact::ZERO,
                symbol: "10.0 ns",
                name: "shakes",
            }
        },
    > = SIUnit;

    /// Years (`a`), a unit of `Time`: `n` of them is `n · 31536000.0` coherent units.
    pub const YEAR: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(31536000),
                zero: Exact::ZERO,
                symbol: "a",
                name: "years",
            }
        },
    > = SIUnit;

    /// Years (sidereal) (`a (sidereal)`), a unit of `Time`: `n` of them is `n · 31558150.0` coherent units.
    pub const YEAR_SIDEREAL: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(31558150),
                zero: Exact::ZERO,
                symbol: "a (sidereal)",
                name: "years (sidereal)",
            }
        },
    > = SIUnit;

    /// Years (tropical) (`a (tropical)`), a unit of `Time`: `n` of them is `n · 31556930.0` coherent units.
    pub const YEAR_TROPICAL: SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(31556930),
                zero: Exact::ZERO,
                symbol: "a (tropical)",
                name: "years (tropical)",
            }
        },
    > = SIUnit;

    /// Seconds of Unix time (`s Unix`), a unit of `Instant`: `n` of them is `n · 1.0` coherent units.
    pub const UNIX_SECOND: SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "s Unix",
                name: "seconds of Unix time",
            }
        },
    > = SIUnit;

    /// Milliseconds of Unix time (`ms Unix`), a unit of `Instant`: `n` of them is `n · 0.001` coherent units.
    pub const UNIX_MILLISECOND: SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "ms Unix",
                name: "milliseconds of Unix time",
            }
        },
    > = SIUnit;

    /// Seconds of GPS time (`s GPS`), a unit of `Instant`: `n` of them is `n · 1.0 + 315964800.0` coherent units.
    pub const GPS_SECOND: SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::int(315964800),
                symbol: "s GPS",
                name: "seconds of GPS time",
            }
        },
    > = SIUnit;

    /// Julian date (`JD`), a unit of `Instant`: `n` of them is `n · 86400.0 + -2440587.5 * 86400.0` coherent units.
    pub const JULIAN_DATE: SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::int(86400),
                zero: -Exact::dec(24405875, -1) * Exact::int(86400),
                symbol: "JD",
                name: "Julian date",
            }
        },
    > = SIUnit;

    /// Modified Julian date (`MJD`), a unit of `Instant`: `n` of them is `n · 86400.0 + -40587.0 * 86400.0` coherent units.
    pub const MODIFIED_JULIAN_DATE: SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::int(86400),
                zero: -Exact::int(40587) * Exact::int(86400),
                symbol: "MJD",
                name: "modified Julian date",
            }
        },
    > = SIUnit;

    /// Square metres (`m²`), a unit of `Area`: `n` of them is `n · 1.0` coherent units.
    pub const SQUARE_METRE: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²",
                name: "square metres",
            }
        },
    > = SIUnit;

    /// Acres (`ac`), a unit of `Area`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0)` coherent units.
    pub const ACRE: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::int(43560)
                    * (Exact::int(1200) / Exact::int(3937))
                    * (Exact::int(1200) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "ac",
                name: "acres",
            }
        },
    > = SIUnit;

    /// Ares (`a`), a unit of `Area`: `n` of them is `n · 100.0` coherent units.
    pub const ARE: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "a",
                name: "ares",
            }
        },
    > = SIUnit;

    /// Barns (`b`), a unit of `Area`: `n` of them is `n · 1e-28` coherent units.
    pub const BARN: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(1, -28),
                zero: Exact::ZERO,
                symbol: "b",
                name: "barns",
            }
        },
    > = SIUnit;

    /// Circular mils (`cmil`), a unit of `Area`: `n` of them is `n · core::f64::consts::PI / 4.0 * 2.54e-05 * 2.54e-05` coherent units.
    pub const CIRCULAR_MIL: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(4) * Exact::dec(254, -7) * Exact::dec(254, -7),
                zero: Exact::ZERO,
                symbol: "cmil",
                name: "circular mils",
            }
        },
    > = SIUnit;

    /// Hectares (`ha`), a unit of `Area`: `n` of them is `n · 10000.0` coherent units.
    pub const HECTARE: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "ha",
                name: "hectares",
            }
        },
    > = SIUnit;

    /// Square feet (`ft²`), a unit of `Area`: `n` of them is `n · 0.09290304` coherent units.
    pub const SQUARE_FOOT: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(9290304, -8),
                zero: Exact::ZERO,
                symbol: "ft²",
                name: "square feet",
            }
        },
    > = SIUnit;

    /// Square inches (`in²`), a unit of `Area`: `n` of them is `n · 0.00064516` coherent units.
    pub const SQUARE_INCH: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(64516, -8),
                zero: Exact::ZERO,
                symbol: "in²",
                name: "square inches",
            }
        },
    > = SIUnit;

    /// Square miles (`mi²`), a unit of `Area`: `n` of them is `n · 2589988.110336` coherent units.
    pub const SQUARE_MILE: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(2589988110336, -6),
                zero: Exact::ZERO,
                symbol: "mi²",
                name: "square miles",
            }
        },
    > = SIUnit;

    /// Square yards (`yd²`), a unit of `Area`: `n` of them is `n · 0.83612736` coherent units.
    pub const SQUARE_YARD: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(83612736, -8),
                zero: Exact::ZERO,
                symbol: "yd²",
                name: "square yards",
            }
        },
    > = SIUnit;

    /// Saltus (`saltus`), a unit of `Area`: `n` of them is `n · 2018672.64` coherent units.
    pub const SALTUS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(201867264, -2),
                zero: Exact::ZERO,
                symbol: "saltus",
                name: "saltus",
            }
        },
    > = SIUnit;

    /// Centuriae (`centuria`), a unit of `Area`: `n` of them is `n · 504668.16` coherent units.
    pub const CENTURIA: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(50466816, -2),
                zero: Exact::ZERO,
                symbol: "centuria",
                name: "centuriae",
            }
        },
    > = SIUnit;

    /// Heredia (`heredium`), a unit of `Area`: `n` of them is `n · 5046.6816` coherent units.
    pub const HEREDIUM: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(50466816, -4),
                zero: Exact::ZERO,
                symbol: "heredium",
                name: "heredia",
            }
        },
    > = SIUnit;

    /// Jugera (`jugerum`), a unit of `Area`: `n` of them is `n · 2523.3408` coherent units.
    pub const JUGERUM: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(25233408, -4),
                zero: Exact::ZERO,
                symbol: "jugerum",
                name: "jugera",
            }
        },
    > = SIUnit;

    /// Deunces (`deunx`), a unit of `Area`: `n` of them is `n · 2313.0624` coherent units.
    pub const AREA_DEUNX: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(23130624, -4),
                zero: Exact::ZERO,
                symbol: "deunx",
                name: "deunces",
            }
        },
    > = SIUnit;

    /// Dextantes (`dextans`), a unit of `Area`: `n` of them is `n · 2102.784` coherent units.
    pub const AREA_DEXTANS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(2102784, -3),
                zero: Exact::ZERO,
                symbol: "dextans",
                name: "dextantes",
            }
        },
    > = SIUnit;

    /// Dodrantes (`dodrans`), a unit of `Area`: `n` of them is `n · 1892.5056` coherent units.
    pub const AREA_DODRANS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(18925056, -4),
                zero: Exact::ZERO,
                symbol: "dodrans",
                name: "dodrantes",
            }
        },
    > = SIUnit;

    /// Besses (`bes`), a unit of `Area`: `n` of them is `n · 1682.2272` coherent units.
    pub const AREA_BES: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(16822272, -4),
                zero: Exact::ZERO,
                symbol: "bes",
                name: "besses",
            }
        },
    > = SIUnit;

    /// Septunces (`septunx`), a unit of `Area`: `n` of them is `n · 1471.9488` coherent units.
    pub const AREA_SEPTUNX: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(14719488, -4),
                zero: Exact::ZERO,
                symbol: "septunx",
                name: "septunces",
            }
        },
    > = SIUnit;

    /// Semisses (`semis`), a unit of `Area`: `n` of them is `n · 1261.6704` coherent units.
    pub const AREA_SEMIS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(12616704, -4),
                zero: Exact::ZERO,
                symbol: "semis",
                name: "semisses",
            }
        },
    > = SIUnit;

    /// Acta quadratus (`actus quadratus`), a unit of `Area`: `n` of them is `n · 1261.6704` coherent units.
    pub const ACTUS_QUADRATUS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(12616704, -4),
                zero: Exact::ZERO,
                symbol: "actus quadratus",
                name: "acta quadratus",
            }
        },
    > = SIUnit;

    /// Quincunces (`quincunx`), a unit of `Area`: `n` of them is `n · 1051.392` coherent units.
    pub const AREA_QUINCUNX: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(1051392, -3),
                zero: Exact::ZERO,
                symbol: "quincunx",
                name: "quincunces",
            }
        },
    > = SIUnit;

    /// Trientes (`triens`), a unit of `Area`: `n` of them is `n · 841.1136` coherent units.
    pub const AREA_TRIENS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(8411136, -4),
                zero: Exact::ZERO,
                symbol: "triens",
                name: "trientes",
            }
        },
    > = SIUnit;

    /// Quadrantes (`quadrans`), a unit of `Area`: `n` of them is `n · 630.8352` coherent units.
    pub const QUADRANS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(6308352, -4),
                zero: Exact::ZERO,
                symbol: "quadrans",
                name: "quadrantes",
            }
        },
    > = SIUnit;

    /// Sextantes (`sextans`), a unit of `Area`: `n` of them is `n · 420.5568` coherent units.
    pub const AREA_SEXTANS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(4205568, -4),
                zero: Exact::ZERO,
                symbol: "sextans",
                name: "sextantes",
            }
        },
    > = SIUnit;

    /// Climata (`clima`), a unit of `Area`: `n` of them is `n · 315.4176` coherent units.
    pub const CLIMA: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(3154176, -4),
                zero: Exact::ZERO,
                symbol: "clima",
                name: "climata",
            }
        },
    > = SIUnit;

    /// Unciae (`uncia`), a unit of `Area`: `n` of them is `n · 210.2784` coherent units.
    pub const AREA_UNCIA: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(2102784, -4),
                zero: Exact::ZERO,
                symbol: "uncia",
                name: "unciae",
            }
        },
    > = SIUnit;

    /// Semiunciae (`semiuncia`), a unit of `Area`: `n` of them is `n · 105.1392` coherent units.
    pub const SEMIUNCIA: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(1051392, -4),
                zero: Exact::ZERO,
                symbol: "semiuncia",
                name: "semiunciae",
            }
        },
    > = SIUnit;

    /// Siculi (`sicilicus`), a unit of `Area`: `n` of them is `n · 52.5696` coherent units.
    pub const AREA_SICILICUS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(525696, -4),
                zero: Exact::ZERO,
                symbol: "sicilicus",
                name: "siculi",
            }
        },
    > = SIUnit;

    /// Acta simplex (`actus simplex`), a unit of `Area`: `n` of them is `n · 42.05568` coherent units.
    pub const ACTUS_SIMPLEX: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(4205568, -5),
                zero: Exact::ZERO,
                symbol: "actus simplex",
                name: "acta simplex",
            }
        },
    > = SIUnit;

    /// Sextulae (`sextula`), a unit of `Area`: `n` of them is `n · 35.0464` coherent units.
    pub const AREA_SEXTULA: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(350464, -4),
                zero: Exact::ZERO,
                symbol: "sextula",
                name: "sextulae",
            }
        },
    > = SIUnit;

    /// Duo scrupula (`duo scrupulum`), a unit of `Area`: `n` of them is `n · 17.5232` coherent units.
    pub const DUO_SCRUPULUM: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(175232, -4),
                zero: Exact::ZERO,
                symbol: "duo scrupulum",
                name: "duo scrupula",
            }
        },
    > = SIUnit;

    /// Scrupula (`scrupulum`), a unit of `Area`: `n` of them is `n · 8.7616` coherent units.
    pub const AREA_SCRUPULUM: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(87616, -4),
                zero: Exact::ZERO,
                symbol: "scrupulum",
                name: "scrupula",
            }
        },
    > = SIUnit;

    /// Dimidium scupula (`dimidium scrupulum`), a unit of `Area`: `n` of them is `n · 4.3808` coherent units.
    pub const DIMIDIUM_SCRUPULUM: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(43808, -4),
                zero: Exact::ZERO,
                symbol: "dimidium scrupulum",
                name: "dimidium scupula",
            }
        },
    > = SIUnit;

    /// Pedes quadratus (`pes quadratus`), a unit of `Area`: `n` of them is `n · 0.087616` coherent units.
    pub const PES_QUADRATUS: SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(87616, -6),
                zero: Exact::ZERO,
                symbol: "pes quadratus",
                name: "pedes quadratus",
            }
        },
    > = SIUnit;

    /// Cubic metres (`m³`), a unit of `Volume`: `n` of them is `n · 1.0` coherent units.
    pub const CUBIC_METRE: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³",
                name: "cubic metres",
            }
        },
    > = SIUnit;

    /// Acre-feet (`ac · ft`), a unit of `Volume`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0) * (1200.0 / 3937.0)` coherent units.
    pub const ACRE_FOOT: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(43560)
                    * (Exact::int(1200) / Exact::int(3937))
                    * (Exact::int(1200) / Exact::int(3937))
                    * (Exact::int(1200) / Exact::int(3937)),
                zero: Exact::ZERO,
                symbol: "ac · ft",
                name: "acre-feet",
            }
        },
    > = SIUnit;

    /// Barrels (`bbl`), a unit of `Volume`: `n` of them is `n · 0.158987294928` coherent units.
    pub const BARREL: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(158987294928, -12),
                zero: Exact::ZERO,
                symbol: "bbl",
                name: "barrels",
            }
        },
    > = SIUnit;

    /// Bushels (`bu`), a unit of `Volume`: `n` of them is `n · 0.03523907016688` coherent units.
    pub const BUSHEL: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3523907016688, -14),
                zero: Exact::ZERO,
                symbol: "bu",
                name: "bushels",
            }
        },
    > = SIUnit;

    /// Cords (`cords`), a unit of `Volume`: `n` of them is `n · 128.0 * 0.3048 * 0.3048 * 0.3048` coherent units.
    pub const CORD: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(128)
                    * Exact::dec(3048, -4)
                    * Exact::dec(3048, -4)
                    * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "cords",
                name: "cords",
            }
        },
    > = SIUnit;

    /// Cubic feet (`ft³`), a unit of `Volume`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048` coherent units.
    pub const CUBIC_FOOT: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft³",
                name: "cubic feet",
            }
        },
    > = SIUnit;

    /// Cubic inches (`in³`), a unit of `Volume`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254` coherent units.
    pub const CUBIC_INCH: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in³",
                name: "cubic inches",
            }
        },
    > = SIUnit;

    /// Cubic miles (`mi³`), a unit of `Volume`: `n` of them is `n · 1609.344 * 1609.344 * 1609.344` coherent units.
    pub const CUBIC_MILE: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi³",
                name: "cubic miles",
            }
        },
    > = SIUnit;

    /// Cubic yards (`yd³`), a unit of `Volume`: `n` of them is `n · 0.9144 * 0.9144 * 0.9144` coherent units.
    pub const CUBIC_YARD: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd³",
                name: "cubic yards",
            }
        },
    > = SIUnit;

    /// Cups (`cup`), a unit of `Volume`: `n` of them is `n · 0.0002365882365` coherent units.
    pub const CUP: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(2365882365, -13),
                zero: Exact::ZERO,
                symbol: "cup",
                name: "cups",
            }
        },
    > = SIUnit;

    /// Fluid ounces (`fl oz`), a unit of `Volume`: `n` of them is `n · 2.95735295625e-05` coherent units.
    pub const FLUID_OUNCE: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "fl oz",
                name: "fluid ounces",
            }
        },
    > = SIUnit;

    /// Imperial fluid ounces (`fl oz (UK)`), a unit of `Volume`: `n` of them is `n · 2.84130625e-05` coherent units.
    pub const FLUID_OUNCE_IMPERIAL: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "fl oz (UK)",
                name: "Imperial fluid ounces",
            }
        },
    > = SIUnit;

    /// Imperial gallons (`gal (UK)`), a unit of `Volume`: `n` of them is `n · 0.00454609` coherent units.
    pub const GALLON_IMPERIAL: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "gal (UK)",
                name: "Imperial gallons",
            }
        },
    > = SIUnit;

    /// Gallons (`gal`), a unit of `Volume`: `n` of them is `n · 0.003785411784` coherent units.
    pub const GALLON: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "gal",
                name: "gallons",
            }
        },
    > = SIUnit;

    /// Imperial gills (`gi (UK)`), a unit of `Volume`: `n` of them is `n · 0.0001420653125` coherent units.
    pub const GILL_IMPERIAL: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1420653125, -13),
                zero: Exact::ZERO,
                symbol: "gi (UK)",
                name: "Imperial gills",
            }
        },
    > = SIUnit;

    /// Gills (`gi`), a unit of `Volume`: `n` of them is `n · 0.00011829411825` coherent units.
    pub const GILL: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(11829411825, -14),
                zero: Exact::ZERO,
                symbol: "gi",
                name: "gills",
            }
        },
    > = SIUnit;

    /// Litres (`L`), a unit of `Volume`: `n` of them is `n · 0.001` coherent units.
    pub const LITRE: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "L",
                name: "litres",
            }
        },
    > = SIUnit;

    /// Pecks (`pk`), a unit of `Volume`: `n` of them is `n · 0.00880976754172` coherent units.
    pub const PECK: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(880976754172, -14),
                zero: Exact::ZERO,
                symbol: "pk",
                name: "pecks",
            }
        },
    > = SIUnit;

    /// Dry pints (`dry pt`), a unit of `Volume`: `n` of them is `n · 0.0005506104713575` coherent units.
    pub const PINT_DRY: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(5506104713575, -16),
                zero: Exact::ZERO,
                symbol: "dry pt",
                name: "dry pints",
            }
        },
    > = SIUnit;

    /// Liquid pints (`liq pt`), a unit of `Volume`: `n` of them is `n · 0.000473176473` coherent units.
    pub const PINT_LIQUID: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(473176473, -12),
                zero: Exact::ZERO,
                symbol: "liq pt",
                name: "liquid pints",
            }
        },
    > = SIUnit;

    /// Dry quarts (`dry qt`), a unit of `Volume`: `n` of them is `n · 0.001101220942715` coherent units.
    pub const QUART_DRY: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1101220942715, -15),
                zero: Exact::ZERO,
                symbol: "dry qt",
                name: "dry quarts",
            }
        },
    > = SIUnit;

    /// Liquid quarts (`liq qt`), a unit of `Volume`: `n` of them is `n · 0.000946352946` coherent units.
    pub const QUART_LIQUID: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(946352946, -12),
                zero: Exact::ZERO,
                symbol: "liq qt",
                name: "liquid quarts",
            }
        },
    > = SIUnit;

    /// Steres (`st`), a unit of `Volume`: `n` of them is `n · 1.0` coherent units.
    pub const STERE: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "st",
                name: "steres",
            }
        },
    > = SIUnit;

    /// Tablespoons (`tbsp`), a unit of `Volume`: `n` of them is `n · 1.478676478125e-05` coherent units.
    pub const TABLESPOON: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1478676478125, -17),
                zero: Exact::ZERO,
                symbol: "tbsp",
                name: "tablespoons",
            }
        },
    > = SIUnit;

    /// Teaspoons (`tsp`), a unit of `Volume`: `n` of them is `n · 4.92892159375e-06` coherent units.
    pub const TEASPOON: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(492892159375, -17),
                zero: Exact::ZERO,
                symbol: "tsp",
                name: "teaspoons",
            }
        },
    > = SIUnit;

    /// Register tons (`RT`), a unit of `Volume`: `n` of them is `n · 100.0 * 0.3048 * 0.3048 * 0.3048` coherent units.
    pub const REGISTER_TON: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(100)
                    * Exact::dec(3048, -4)
                    * Exact::dec(3048, -4)
                    * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "RT",
                name: "register tons",
            }
        },
    > = SIUnit;

    /// Culei (`culeus`), a unit of `Volume`: `n` of them is `n · 0.51868672` coherent units.
    pub const CULEUS: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(51868672, -8),
                zero: Exact::ZERO,
                symbol: "culeus",
                name: "culei",
            }
        },
    > = SIUnit;

    /// Amphora quadrantal (`amphora quadrantal`), a unit of `Volume`: `n` of them is `n · 0.025934336` coherent units.
    pub const AMPHORA_QUADRANTAL: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(25934336, -9),
                zero: Exact::ZERO,
                symbol: "amphora quadrantal",
                name: "amphora quadrantal",
            }
        },
    > = SIUnit;

    /// Urnae (`urna`), a unit of `Volume`: `n` of them is `n · 0.012967168` coherent units.
    pub const URNA: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(12967168, -9),
                zero: Exact::ZERO,
                symbol: "urna",
                name: "urnae",
            }
        },
    > = SIUnit;

    /// Modii castrensis (`modius castrensis`), a unit of `Volume`: `n` of them is `n · 0.012967168` coherent units.
    pub const MODIUS_CASTRENSIS: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(12967168, -9),
                zero: Exact::ZERO,
                symbol: "modius castrensis",
                name: "modii castrensis",
            }
        },
    > = SIUnit;

    /// Modii (`modius`), a unit of `Volume`: `n` of them is `n · 0.003241792 * 16.0 / 6.0` coherent units.
    pub const MODIUS: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) * Exact::int(16) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "modius",
                name: "modii",
            }
        },
    > = SIUnit;

    /// Semimodii (`semimodius`), a unit of `Volume`: `n` of them is `n · 0.003241792 * 8.0 / 6.0` coherent units.
    pub const SEMIMODIUS: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) * Exact::int(8) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "semimodius",
                name: "semimodii",
            }
        },
    > = SIUnit;

    /// Congii (`congius`), a unit of `Volume`: `n` of them is `n · 0.003241792` coherent units.
    pub const CONGIUS: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9),
                zero: Exact::ZERO,
                symbol: "congius",
                name: "congii",
            }
        },
    > = SIUnit;

    /// Sextarii (`sextarius`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 6.0` coherent units.
    pub const SEXTARIUS: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "sextarius",
                name: "sextarii",
            }
        },
    > = SIUnit;

    /// Heminae (`hemina`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 12.0` coherent units.
    pub const HEMINA: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "hemina",
                name: "heminae",
            }
        },
    > = SIUnit;

    /// Quartarii (`quartarius`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 24.0` coherent units.
    pub const QUARTARIUS: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(24),
                zero: Exact::ZERO,
                symbol: "quartarius",
                name: "quartarii",
            }
        },
    > = SIUnit;

    /// Acetabula (`acetabulum`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 48.0` coherent units.
    pub const ACETABULUM: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(48),
                zero: Exact::ZERO,
                symbol: "acetabulum",
                name: "acetabula",
            }
        },
    > = SIUnit;

    /// Cyathi (`cyathus`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 72.0` coherent units.
    pub const CYATHUS: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(72),
                zero: Exact::ZERO,
                symbol: "cyathus",
                name: "cyathi",
            }
        },
    > = SIUnit;

    /// Ligula (`ligula`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 288.0` coherent units.
    pub const LIGULA: SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(288),
                zero: Exact::ZERO,
                symbol: "ligula",
                name: "ligula",
            }
        },
    > = SIUnit;

    /// Metres per second (`m/s`), a unit of `Velocity`: `n` of them is `n · 1.0` coherent units.
    pub const METRE_PER_SECOND: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m/s",
                name: "metres per second",
            }
        },
    > = SIUnit;

    /// Feet per hour (`ft/h`), a unit of `Velocity`: `n` of them is `n · 0.3048 / 3600.0` coherent units.
    pub const FOOT_PER_HOUR: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "ft/h",
                name: "feet per hour",
            }
        },
    > = SIUnit;

    /// Feet per minute (`ft/min`), a unit of `Velocity`: `n` of them is `n · 0.00508` coherent units.
    pub const FOOT_PER_MINUTE: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(508, -5),
                zero: Exact::ZERO,
                symbol: "ft/min",
                name: "feet per minute",
            }
        },
    > = SIUnit;

    /// Feet per second (`ft/s`), a unit of `Velocity`: `n` of them is `n · 0.3048` coherent units.
    pub const FOOT_PER_SECOND: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft/s",
                name: "feet per second",
            }
        },
    > = SIUnit;

    /// Inches per second (`in/s`), a unit of `Velocity`: `n` of them is `n · 0.0254` coherent units.
    pub const INCH_PER_SECOND: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in/s",
                name: "inches per second",
            }
        },
    > = SIUnit;

    /// Inches per minute (`in/min`), a unit of `Velocity`: `n` of them is `n · 0.0254 / 60.0` coherent units.
    pub const INCH_PER_MINUTE: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "in/min",
                name: "inches per minute",
            }
        },
    > = SIUnit;

    /// Knots (`kn`), a unit of `Velocity`: `n` of them is `n · 1852.0 / 3600.0` coherent units.
    pub const KNOT: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1852) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "kn",
                name: "knots",
            }
        },
    > = SIUnit;

    /// Miles per hour (`mi/h`), a unit of `Velocity`: `n` of them is `n · 0.44704` coherent units.
    pub const MILE_PER_HOUR: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(44704, -5),
                zero: Exact::ZERO,
                symbol: "mi/h",
                name: "miles per hour",
            }
        },
    > = SIUnit;

    /// Miles per minute (`mi/min`), a unit of `Velocity`: `n` of them is `n · 26.8224` coherent units.
    pub const MILE_PER_MINUTE: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(268224, -4),
                zero: Exact::ZERO,
                symbol: "mi/min",
                name: "miles per minute",
            }
        },
    > = SIUnit;

    /// Miles per second (`mi/s`), a unit of `Velocity`: `n` of them is `n · 1609.344` coherent units.
    pub const MILE_PER_SECOND: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi/s",
                name: "miles per second",
            }
        },
    > = SIUnit;

    /// Atomic units of velocity (`a₀ · Eₕ/ħ`), a unit of `Velocity`: `n` of them is `n · 2187691.26364` coherent units.
    pub const ATOMIC_UNIT_OF_VELOCITY: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(218769126364, -5),
                zero: Exact::ZERO,
                symbol: "a₀ · Eₕ/ħ",
                name: "atomic units of velocity",
            }
        },
    > = SIUnit;

    /// Natural units of velocity (`c`), a unit of `Velocity`: `n` of them is `n · 299792458.0` coherent units.
    pub const NATURAL_UNIT_OF_VELOCITY: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(299792458),
                zero: Exact::ZERO,
                symbol: "c",
                name: "natural units of velocity",
            }
        },
    > = SIUnit;

    /// Speeds of light in vacuum (`c`), a unit of `Velocity`: `n` of them is `n · 299792458.0` coherent units.
    pub const SPEED_OF_LIGHT: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(299792458),
                zero: Exact::ZERO,
                symbol: "c",
                name: "speeds of light in vacuum",
            }
        },
    > = SIUnit;

    /// Kilometres per hour (`km/h`), a unit of `Velocity`: `n` of them is `n · 1000.0 / 3600.0` coherent units.
    pub const KILOMETRE_PER_HOUR: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1000) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "km/h",
                name: "kilometres per hour",
            }
        },
    > = SIUnit;

    /// Millimetres per minute (`mm/min`), a unit of `Velocity`: `n` of them is `n · 0.001 / 60.0` coherent units.
    pub const MILLIMETRE_PER_MINUTE: SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "mm/min",
                name: "millimetres per minute",
            }
        },
    > = SIUnit;

    /// Metres per second squared (`m/s²`), a unit of `Acceleration`: `n` of them is `n · 1.0` coherent units.
    pub const METRE_PER_SECOND_SQUARED: SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m/s²",
                name: "metres per second squared",
            }
        },
    > = SIUnit;

    /// Feet per second squared (`ft/s²`), a unit of `Acceleration`: `n` of them is `n · 0.3048` coherent units.
    pub const FOOT_PER_SECOND_SQUARED: SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft/s²",
                name: "feet per second squared",
            }
        },
    > = SIUnit;

    /// Galileos (`Gal`), a unit of `Acceleration`: `n` of them is `n · 0.01` coherent units.
    pub const GALILEO: SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "Gal",
                name: "galileos",
            }
        },
    > = SIUnit;

    /// Inches per second squared (`in/s²`), a unit of `Acceleration`: `n` of them is `n · 0.0254` coherent units.
    pub const INCH_PER_SECOND_SQUARED: SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in/s²",
                name: "inches per second squared",
            }
        },
    > = SIUnit;

    /// Standard accelerations of gravity (`g₀`), a unit of `Acceleration`: `n` of them is `n · 9.80665` coherent units.
    pub const STANDARD_GRAVITY: SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "g₀",
                name: "standard accelerations of gravity",
            }
        },
    > = SIUnit;

    /// Millimetres per minute squared (`mm/min²`), a unit of `Acceleration`: `n` of them is `n · 0.001 / (60.0 * 60.0)` coherent units.
    pub const MILLIMETRE_PER_MINUTE_SQUARED: SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / (Exact::int(60) * Exact::int(60)),
                zero: Exact::ZERO,
                symbol: "mm/min²",
                name: "millimetres per minute squared",
            }
        },
    > = SIUnit;

    /// Metres per second cubed (`m/s³`), a unit of `Jerk`: `n` of them is `n · 1.0` coherent units.
    pub const METRE_PER_SECOND_CUBED: SIUnit<
        JERK,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m/s³",
                name: "metres per second cubed",
            }
        },
    > = SIUnit;

    /// Feet per second cubed (`ft/s³`), a unit of `Jerk`: `n` of them is `n · 0.3048` coherent units.
    pub const FOOT_PER_SECOND_CUBED: SIUnit<
        JERK,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft/s³",
                name: "feet per second cubed",
            }
        },
    > = SIUnit;

    /// Inches per second cubed (`in/s³`), a unit of `Jerk`: `n` of them is `n · 0.0254` coherent units.
    pub const INCH_PER_SECOND_CUBED: SIUnit<
        JERK,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in/s³",
                name: "inches per second cubed",
            }
        },
    > = SIUnit;

    /// Kilometres per minute cubed (`km/min³`), a unit of `Jerk`: `n` of them is `n · 1000.0 / (60.0 * 60.0 * 60.0)` coherent units.
    pub const KILOMETRE_PER_MINUTE_CUBED: SIUnit<
        JERK,
        {
            SIUnitConsts {
                size: Exact::int(1000) / (Exact::int(60) * Exact::int(60) * Exact::int(60)),
                zero: Exact::ZERO,
                symbol: "km/min³",
                name: "kilometres per minute cubed",
            }
        },
    > = SIUnit;

    /// Metre seconds (`m · s`), a unit of `Absement`: `n` of them is `n · 1.0` coherent units.
    pub const METRE_SECOND: SIUnit<
        ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m · s",
                name: "metre seconds",
            }
        },
    > = SIUnit;

    /// Foot seconds (`ft · s`), a unit of `Absement`: `n` of them is `n · 0.3048` coherent units.
    pub const FOOT_SECOND: SIUnit<
        ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft · s",
                name: "foot seconds",
            }
        },
    > = SIUnit;

    /// Inch seconds (`in · s`), a unit of `Absement`: `n` of them is `n · 0.0254` coherent units.
    pub const INCH_SECOND: SIUnit<
        ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in · s",
                name: "inch seconds",
            }
        },
    > = SIUnit;

    /// Kilometre hours (`km · h`), a unit of `Absement`: `n` of them is `n · 3600000.0` coherent units.
    pub const KILOMETRE_HOUR: SIUnit<
        ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::int(3600000),
                zero: Exact::ZERO,
                symbol: "km · h",
                name: "kilometre hours",
            }
        },
    > = SIUnit;

    /// Seconds per metre (`s/m`), a unit of `InverseVelocity`: `n` of them is `n · 1.0` coherent units.
    pub const SECOND_PER_METRE: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "s/m",
                name: "seconds per metre",
            }
        },
    > = SIUnit;

    /// Hours per foot (`h/ft`), a unit of `InverseVelocity`: `n` of them is `n · 3600.0 / 0.3048` coherent units.
    pub const HOUR_PER_FOOT: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(3600) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "h/ft",
                name: "hours per foot",
            }
        },
    > = SIUnit;

    /// Minutes per foot (`min/ft`), a unit of `InverseVelocity`: `n` of them is `n · 60.0 / 0.3048` coherent units.
    pub const MINUTE_PER_FOOT: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(60) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "min/ft",
                name: "minutes per foot",
            }
        },
    > = SIUnit;

    /// Seconds per foot (`s/ft`), a unit of `InverseVelocity`: `n` of them is `n · 1.0 / 0.3048` coherent units.
    pub const SECOND_PER_FOOT: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "s/ft",
                name: "seconds per foot",
            }
        },
    > = SIUnit;

    /// Seconds per inch (`s/in`), a unit of `InverseVelocity`: `n` of them is `n · 1.0 / 0.0254` coherent units.
    pub const SECOND_PER_INCH: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "s/in",
                name: "seconds per inch",
            }
        },
    > = SIUnit;

    /// Hours per mile (`h/mi`), a unit of `InverseVelocity`: `n` of them is `n · 3600.0 / 1609.344` coherent units.
    pub const HOUR_PER_MILE: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(3600) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "h/mi",
                name: "hours per mile",
            }
        },
    > = SIUnit;

    /// Minutes per mile (`min/mi`), a unit of `InverseVelocity`: `n` of them is `n · 60.0 / 1609.344` coherent units.
    pub const MINUTE_PER_MILE: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(60) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "min/mi",
                name: "minutes per mile",
            }
        },
    > = SIUnit;

    /// Seconds per mile (`s/mi`), a unit of `InverseVelocity`: `n` of them is `n · 1.0 / 1609.344` coherent units.
    pub const SECOND_PER_MILE: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "s/mi",
                name: "seconds per mile",
            }
        },
    > = SIUnit;

    /// Minutes per kilometre (`min/km`), a unit of `InverseVelocity`: `n` of them is `n · 0.06` coherent units.
    pub const MINUTE_PER_KILOMETRE: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(6, -2),
                zero: Exact::ZERO,
                symbol: "min/km",
                name: "minutes per kilometre",
            }
        },
    > = SIUnit;

    /// Hours per kilometre (`h/km`), a unit of `InverseVelocity`: `n` of them is `n · 3.6` coherent units.
    pub const HOUR_PER_KILOMETRE: SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(36, -1),
                zero: Exact::ZERO,
                symbol: "h/km",
                name: "hours per kilometre",
            }
        },
    > = SIUnit;

    /// Newtons (`N`), a unit of `Force`: `n` of them is `n · 1.0` coherent units.
    pub const NEWTON: SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N",
                name: "newtons",
            }
        },
    > = SIUnit;

    /// Dynes (`dyn`), a unit of `Force`: `n` of them is `n · 1e-05` coherent units.
    pub const DYNE: SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -5),
                zero: Exact::ZERO,
                symbol: "dyn",
                name: "dynes",
            }
        },
    > = SIUnit;

    /// Kilograms-force (`kgf`), a unit of `Force`: `n` of them is `n · 9.80665` coherent units.
    pub const KILOGRAM_FORCE: SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "kgf",
                name: "kilograms-force",
            }
        },
    > = SIUnit;

    /// Kips (`kip`), a unit of `Force`: `n` of them is `n · 1000.0 * 0.45359237 * 9.80665` coherent units.
    pub const KIP: SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "kip",
                name: "kips",
            }
        },
    > = SIUnit;

    /// Ounces-force (`ozf`), a unit of `Force`: `n` of them is `n · 0.45359237 * 9.80665 / 16.0` coherent units.
    pub const OUNCE_FORCE: SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16),
                zero: Exact::ZERO,
                symbol: "ozf",
                name: "ounces-force",
            }
        },
    > = SIUnit;

    /// Poundals (`pdl`), a unit of `Force`: `n` of them is `n · 0.45359237 * 0.3048` coherent units.
    pub const POUNDAL: SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "pdl",
                name: "poundals",
            }
        },
    > = SIUnit;

    /// Pounds-force (`lbf`), a unit of `Force`: `n` of them is `n · 0.45359237 * 9.80665` coherent units.
    pub const POUND_FORCE: SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "lbf",
                name: "pounds-force",
            }
        },
    > = SIUnit;

    /// Tons-force (`2000 lbf`), a unit of `Force`: `n` of them is `n · 2000.0 * 0.45359237 * 9.80665` coherent units.
    pub const TON_FORCE: SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::int(2000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "2000 lbf",
                name: "tons-force",
            }
        },
    > = SIUnit;

    /// Kilogram metres per second (`kg · m/s`), a unit of `Momentum`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_METRE_PER_SECOND: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m/s",
                name: "kilogram metres per second",
            }
        },
    > = SIUnit;

    /// Gram metres per second (`g · m/s`), a unit of `Momentum`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_METRE_PER_SECOND: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g · m/s",
                name: "gram metres per second",
            }
        },
    > = SIUnit;

    /// Ton metres per second (`t · m/s`), a unit of `Momentum`: `n` of them is `n · 1000.0` coherent units.
    pub const TON_METRE_PER_SECOND: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "t · m/s",
                name: "ton metres per second",
            }
        },
    > = SIUnit;

    /// Kilogram metres per minute (`kg · m/min`), a unit of `Momentum`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub const KILOGRAM_METRE_PER_MINUTE: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "kg · m/min",
                name: "kilogram metres per minute",
            }
        },
    > = SIUnit;

    /// Kilogram metres per hour (`kg · m/h`), a unit of `Momentum`: `n` of them is `n · 1.0 / 3600.0` coherent units.
    pub const KILOGRAM_METRE_PER_HOUR: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "kg · m/h",
                name: "kilogram metres per hour",
            }
        },
    > = SIUnit;

    /// Kilogram metres per day (`kg · m/d`), a unit of `Momentum`: `n` of them is `n · 1.0 / 86400.0` coherent units.
    pub const KILOGRAM_METRE_PER_DAY: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "kg · m/d",
                name: "kilogram metres per day",
            }
        },
    > = SIUnit;

    /// Slug feet per second (`slug · ft/s`), a unit of `Momentum`: `n` of them is `n · (0.45359237 * 9.80665 / 0.3048) * 0.3048` coherent units.
    pub const SLUG_FOOT_PER_SECOND: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4))
                    * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "slug · ft/s",
                name: "slug feet per second",
            }
        },
    > = SIUnit;

    /// Slug inches per second (`slug · in/s`), a unit of `Momentum`: `n` of them is `n · (0.45359237 * 9.80665 / 0.3048) * 0.0254` coherent units.
    pub const SLUG_INCH_PER_SECOND: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4))
                    * Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "slug · in/s",
                name: "slug inches per second",
            }
        },
    > = SIUnit;

    /// Pound feet per second (`lb · ft/s`), a unit of `Momentum`: `n` of them is `n · 0.138254954376` coherent units.
    pub const POUND_FOOT_PER_SECOND: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::dec(138254954376, -12),
                zero: Exact::ZERO,
                symbol: "lb · ft/s",
                name: "pound feet per second",
            }
        },
    > = SIUnit;

    /// Pound inches per second (`lb · in/s`), a unit of `Momentum`: `n` of them is `n · 0.011521246198` coherent units.
    pub const POUND_INCH_PER_SECOND: SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::dec(11521246198, -12),
                zero: Exact::ZERO,
                symbol: "lb · in/s",
                name: "pound inches per second",
            }
        },
    > = SIUnit;

    /// Joules (`J`), a unit of `Energy`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J",
                name: "joules",
            }
        },
    > = SIUnit;

    /// Watt hours (`W · h`), a unit of `Energy`: `n` of them is `n · 3600.0` coherent units.
    pub const WATT_HOUR: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "W · h",
                name: "watt hours",
            }
        },
    > = SIUnit;

    /// Electronvolts (`eV`), a unit of `Energy`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub const ELECTRONVOLT: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "eV",
                name: "electronvolts",
            }
        },
    > = SIUnit;

    /// Hartrees (`Eₕ`), a unit of `Energy`: `n` of them is `n · 4.3597447222071e-18` coherent units.
    pub const HARTREE: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(43597447222071, -31),
                zero: Exact::ZERO,
                symbol: "Eₕ",
                name: "hartrees",
            }
        },
    > = SIUnit;

    /// British thermal units (IT) (`Btu (IT)`), a unit of `Energy`: `n` of them is `n · 1055.05585262` coherent units.
    pub const BTU_IT: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8),
                zero: Exact::ZERO,
                symbol: "Btu (IT)",
                name: "British thermal units (IT)",
            }
        },
    > = SIUnit;

    /// British thermal units (`Btu`), a unit of `Energy`: `n` of them is `n · 4184.0 * 0.45359237 / 1.8` coherent units.
    pub const BTU: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1),
                zero: Exact::ZERO,
                symbol: "Btu",
                name: "British thermal units",
            }
        },
    > = SIUnit;

    /// Calories (IT) (`cal (IT)`), a unit of `Energy`: `n` of them is `n · 4.1868` coherent units.
    pub const CALORIE_IT: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -4),
                zero: Exact::ZERO,
                symbol: "cal (IT)",
                name: "calories (IT)",
            }
        },
    > = SIUnit;

    /// Calories (`cal`), a unit of `Energy`: `n` of them is `n · 4.184` coherent units.
    pub const CALORIE: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal",
                name: "calories",
            }
        },
    > = SIUnit;

    /// Calories (IT) (`Cal (IT)`), a unit of `Energy`: `n` of them is `n · 4186.8` coherent units.
    pub const CALORIE_IT_NUTRITION: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Cal (IT)",
                name: "Calories (IT)",
            }
        },
    > = SIUnit;

    /// Calories (`Cal`), a unit of `Energy`: `n` of them is `n · 4184.0` coherent units.
    pub const CALORIE_NUTRITION: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Cal",
                name: "Calories",
            }
        },
    > = SIUnit;

    /// Ergs (`erg`), a unit of `Energy`: `n` of them is `n · 1e-07` coherent units.
    pub const ERG: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg",
                name: "ergs",
            }
        },
    > = SIUnit;

    /// Foot poundals (`ft · pdl`), a unit of `Energy`: `n` of them is `n · 0.3048 * 0.45359237 * 0.3048` coherent units.
    pub const FOOT_POUNDAL: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft · pdl",
                name: "foot poundals",
            }
        },
    > = SIUnit;

    /// Foot pounds-force (`ft · lbf`), a unit of `Energy`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub const FOOT_POUND: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "ft · lbf",
                name: "foot pounds-force",
            }
        },
    > = SIUnit;

    /// Quads (`10¹⁵ Btu (IT)`), a unit of `Energy`: `n` of them is `n · 1.05505585262e18` coherent units.
    pub const QUAD: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1055055852620000000),
                zero: Exact::ZERO,
                symbol: "10¹⁵ Btu (IT)",
                name: "quads",
            }
        },
    > = SIUnit;

    /// Therms (EC) (`thm (EC)`), a unit of `Energy`: `n` of them is `n · 105506000.0` coherent units.
    pub const THERM_EC: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(105506000),
                zero: Exact::ZERO,
                symbol: "thm (EC)",
                name: "therms (EC)",
            }
        },
    > = SIUnit;

    /// Therms (`thm`), a unit of `Energy`: `n` of them is `n · 105480400.0` coherent units.
    pub const THERM_US: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(105480400),
                zero: Exact::ZERO,
                symbol: "thm",
                name: "therms",
            }
        },
    > = SIUnit;

    /// Tons of TNT (`t of TNT`), a unit of `Energy`: `n` of them is `n · 4184000000.0` coherent units.
    pub const TON_TNT: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184000000),
                zero: Exact::ZERO,
                symbol: "t of TNT",
                name: "tons of TNT",
            }
        },
    > = SIUnit;

    /// Watt seconds (`W · s`), a unit of `Energy`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_SECOND: SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W · s",
                name: "watt seconds",
            }
        },
    > = SIUnit;

    /// Joules per kilogram (`J/kg`), a unit of `AvailableEnergy`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_KILOGRAM: SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/kg",
                name: "joules per kilogram",
            }
        },
    > = SIUnit;

    /// Joules per gram (`J/g`), a unit of `AvailableEnergy`: `n` of them is `n · 1000.0` coherent units.
    pub const JOULE_PER_GRAM: SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "J/g",
                name: "joules per gram",
            }
        },
    > = SIUnit;

    /// British thermal units (IT) per pound (`Btu (IT)/lb`), a unit of `AvailableEnergy`: `n` of them is `n · 2326.0` coherent units.
    pub const BTU_IT_PER_POUND: SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(2326),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/lb",
                name: "British thermal units (IT) per pound",
            }
        },
    > = SIUnit;

    /// British thermal units per pound (`Btu/lb`), a unit of `AvailableEnergy`: `n` of them is `n · 4184.0 / 1.8` coherent units.
    pub const BTU_PER_POUND: SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184) / Exact::dec(18, -1),
                zero: Exact::ZERO,
                symbol: "Btu/lb",
                name: "British thermal units per pound",
            }
        },
    > = SIUnit;

    /// Calories (IT) per gram (`cal (IT)/g`), a unit of `AvailableEnergy`: `n` of them is `n · 4186.8` coherent units.
    pub const CALORIE_IT_PER_GRAM: SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "cal (IT)/g",
                name: "calories (IT) per gram",
            }
        },
    > = SIUnit;

    /// Calories per gram (`cal/g`), a unit of `AvailableEnergy`: `n` of them is `n · 4184.0` coherent units.
    pub const CALORIE_PER_GRAM: SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "cal/g",
                name: "calories per gram",
            }
        },
    > = SIUnit;

    /// Watts (`W`), a unit of `Power`: `n` of them is `n · 1.0` coherent units.
    pub const WATT: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W",
                name: "watts",
            }
        },
    > = SIUnit;

    /// Ergs per second (`erg/s`), a unit of `Power`: `n` of them is `n · 1e-07` coherent units.
    pub const ERG_PER_SECOND: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg/s",
                name: "ergs per second",
            }
        },
    > = SIUnit;

    /// Foot pounds-force per hour (`ft · lbf/h`), a unit of `Power`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / 3600.0` coherent units.
    pub const FOOT_POUND_PER_HOUR: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))
                    / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "ft · lbf/h",
                name: "foot pounds-force per hour",
            }
        },
    > = SIUnit;

    /// Foot pounds-force per minute (`ft · lbf/min`), a unit of `Power`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / 60.0` coherent units.
    pub const FOOT_POUND_PER_MINUTE: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))
                    / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "ft · lbf/min",
                name: "foot pounds-force per minute",
            }
        },
    > = SIUnit;

    /// Foot pounds-force per second (`ft · lbf/s`), a unit of `Power`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub const FOOT_POUND_PER_SECOND: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "ft · lbf/s",
                name: "foot pounds-force per second",
            }
        },
    > = SIUnit;

    /// Horsepower (`hp`), a unit of `Power`: `n` of them is `n · 550.0 * 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub const HORSEPOWER: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::int(550)
                    * Exact::dec(3048, -4)
                    * Exact::dec(45359237, -8)
                    * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "hp",
                name: "horsepower",
            }
        },
    > = SIUnit;

    /// Horsepower (boiler) (`hp (S)`), a unit of `Power`: `n` of them is `n · 9809.5` coherent units.
    pub const HORSEPOWER_BOILER: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(98095, -1),
                zero: Exact::ZERO,
                symbol: "hp (S)",
                name: "horsepower (boiler)",
            }
        },
    > = SIUnit;

    /// Horsepower (electric) (`hp (E)`), a unit of `Power`: `n` of them is `n · 746.0` coherent units.
    pub const HORSEPOWER_ELECTRIC: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::int(746),
                zero: Exact::ZERO,
                symbol: "hp (E)",
                name: "horsepower (electric)",
            }
        },
    > = SIUnit;

    /// Metric horsepower (`hp (M)`), a unit of `Power`: `n` of them is `n · 75.0 * 9.80665` coherent units.
    pub const HORSEPOWER_METRIC: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::int(75) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "hp (M)",
                name: "metric horsepower",
            }
        },
    > = SIUnit;

    /// Horsepower (Imperial) (`hp (I)`), a unit of `Power`: `n` of them is `n · 745.7` coherent units.
    pub const HORSEPOWER_IMPERIAL: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(7457, -1),
                zero: Exact::ZERO,
                symbol: "hp (I)",
                name: "horsepower (Imperial)",
            }
        },
    > = SIUnit;

    /// Hydraulic horsepower (`hp (hydraulic)`), a unit of `Power`: `n` of them is `n · 746.043` coherent units.
    pub const HYDRAULIC_HORSEPOWER: SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(746043, -3),
                zero: Exact::ZERO,
                symbol: "hp (hydraulic)",
                name: "hydraulic horsepower",
            }
        },
    > = SIUnit;

    /// Bels (`B`), a unit of `Gain`: `n` of them is `n · 1.0` coherent units.
    pub const BEL: SIUnit<
        GAIN,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "B",
                name: "bels",
            }
        },
    > = SIUnit;

    /// Decibels (`dB`), a unit of `Gain`: `n` of them is `n · 0.1` coherent units.
    pub const DECIBEL: SIUnit<
        GAIN,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "dB",
                name: "decibels",
            }
        },
    > = SIUnit;

    /// Nepers (`Np`), a unit of `Gain`: `n` of them is `n · 0.8685889638065036` coherent units.
    pub const NEPER: SIUnit<
        GAIN,
        {
            SIUnitConsts {
                size: Exact::dec(8685889638065036, -16),
                zero: Exact::ZERO,
                symbol: "Np",
                name: "nepers",
            }
        },
    > = SIUnit;

    /// Decibels-watt (`dBW`), a unit of `PowerLevel`: `n` of them is `n · 0.1` coherent units.
    pub const DECIBEL_WATT: SIUnit<
        POWER_LEVEL,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "dBW",
                name: "decibels-watt",
            }
        },
    > = SIUnit;

    /// Decibels-milliwatt (`dBm`), a unit of `PowerLevel`: `n` of them is `n · 0.1 + -3.0` coherent units.
    pub const DECIBEL_MILLIWATT: SIUnit<
        POWER_LEVEL,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: -Exact::int(3),
                symbol: "dBm",
                name: "decibels-milliwatt",
            }
        },
    > = SIUnit;

    /// Watts per second (`W/s`), a unit of `PowerRate`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_SECOND: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/s",
                name: "watts per second",
            }
        },
    > = SIUnit;

    /// Ergs per second squared (`erg/s²`), a unit of `PowerRate`: `n` of them is `n · 1e-07` coherent units.
    pub const ERG_PER_SECOND_SQUARED: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg/s²",
                name: "ergs per second squared",
            }
        },
    > = SIUnit;

    /// Foot pounds-force per second squared (`ft · lbf/s²`), a unit of `PowerRate`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub const FOOT_POUND_PER_SECOND_SQUARED: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "ft · lbf/s²",
                name: "foot pounds-force per second squared",
            }
        },
    > = SIUnit;

    /// Horsepower per second (`hp/s`), a unit of `PowerRate`: `n` of them is `n · 550.0 * 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub const HORSEPOWER_PER_SECOND: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(550)
                    * Exact::dec(3048, -4)
                    * Exact::dec(45359237, -8)
                    * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "hp/s",
                name: "horsepower per second",
            }
        },
    > = SIUnit;

    /// Horsepower per second (boiler) (`hp/s (S)`), a unit of `PowerRate`: `n` of them is `n · 9809.5` coherent units.
    pub const HORSEPOWER_PER_SECOND_BOILER: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(98095, -1),
                zero: Exact::ZERO,
                symbol: "hp/s (S)",
                name: "horsepower per second (boiler)",
            }
        },
    > = SIUnit;

    /// Horsepower per second (electric) (`hp/s (E)`), a unit of `PowerRate`: `n` of them is `n · 746.0` coherent units.
    pub const HORSEPOWER_PER_SECOND_ELECTRIC: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(746),
                zero: Exact::ZERO,
                symbol: "hp/s (E)",
                name: "horsepower per second (electric)",
            }
        },
    > = SIUnit;

    /// Metric horsepower per second (`hp/s (M)`), a unit of `PowerRate`: `n` of them is `n · 75.0 * 9.80665` coherent units.
    pub const HORSEPOWER_PER_SECOND_METRIC: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(75) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "hp/s (M)",
                name: "metric horsepower per second",
            }
        },
    > = SIUnit;

    /// Horsepower per second (Imperial) (`hp/s (I)`), a unit of `PowerRate`: `n` of them is `n · 745.7` coherent units.
    pub const HORSEPOWER_PER_SECOND_IMPERIAL: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(7457, -1),
                zero: Exact::ZERO,
                symbol: "hp/s (I)",
                name: "horsepower per second (Imperial)",
            }
        },
    > = SIUnit;

    /// Hydraulic horsepower per second (`hp/s (hydraulic)`), a unit of `PowerRate`: `n` of them is `n · 746.043` coherent units.
    pub const HYDRAULIC_HORSEPOWER_PER_SECOND: SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(746043, -3),
                zero: Exact::ZERO,
                symbol: "hp/s (hydraulic)",
                name: "hydraulic horsepower per second",
            }
        },
    > = SIUnit;

    /// Joule seconds (`J · s`), a unit of `Action`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_SECOND: SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J · s",
                name: "joule seconds",
            }
        },
    > = SIUnit;

    /// Atomic units of action (`ħ`), a unit of `Action`: `n` of them is `n · 6.62607015e-34 / (2.0 * core::f64::consts::PI)` coherent units.
    pub const ATOMIC_UNIT_OF_ACTION: SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI),
                zero: Exact::ZERO,
                symbol: "ħ",
                name: "atomic units of action",
            }
        },
    > = SIUnit;

    /// Planck constants (`h`), a unit of `Action`: `n` of them is `n · 6.62607015e-34` coherent units.
    pub const PLANCK_CONSTANT: SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::dec(662607015, -42),
                zero: Exact::ZERO,
                symbol: "h",
                name: "Planck constants",
            }
        },
    > = SIUnit;

    /// Erg seconds (`erg · s`), a unit of `Action`: `n` of them is `n · 1e-07` coherent units.
    pub const ERG_SECOND: SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg · s",
                name: "erg seconds",
            }
        },
    > = SIUnit;

    /// Electronvolt seconds (`eV · s`), a unit of `Action`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub const ELECTRONVOLT_SECOND: SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "eV · s",
                name: "electronvolt seconds",
            }
        },
    > = SIUnit;

    /// Newton metres (`N · m`), a unit of `Torque`: `n` of them is `n · 1.0` coherent units.
    pub const NEWTON_METRE: SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N · m",
                name: "newton metres",
            }
        },
    > = SIUnit;

    /// Dyne metres (`dyn · m`), a unit of `Torque`: `n` of them is `n · 1e-05` coherent units.
    pub const DYNE_METRE: SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -5),
                zero: Exact::ZERO,
                symbol: "dyn · m",
                name: "dyne metres",
            }
        },
    > = SIUnit;

    /// Kilogram-force metres (`kgf · m`), a unit of `Torque`: `n` of them is `n · 9.80665` coherent units.
    pub const KILOGRAM_FORCE_METRE: SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "kgf · m",
                name: "kilogram-force metres",
            }
        },
    > = SIUnit;

    /// Ounces-force inches (`ozf · in`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665 / 16.0) * 0.0254` coherent units.
    pub const OUNCE_FORCE_INCH: SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16))
                    * Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "ozf · in",
                name: "ounces-force inches",
            }
        },
    > = SIUnit;

    /// Pounds-force feet (`lbf · ft`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665) * 0.3048` coherent units.
    pub const POUND_FORCE_FOOT: SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "lbf · ft",
                name: "pounds-force feet",
            }
        },
    > = SIUnit;

    /// Pounds-force inches (`lbf · in`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665) * 0.0254` coherent units.
    pub const POUND_FORCE_INCH: SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "lbf · in",
                name: "pounds-force inches",
            }
        },
    > = SIUnit;

    /// Kilogram square metres (`kg · m²`), a unit of `MomentOfInertia`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_SQUARE_METRE: SIUnit<
        MOMENT_OF_INERTIA,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²",
                name: "kilogram square metres",
            }
        },
    > = SIUnit;

    /// Dalton square ångströms (`Da · Å²`), a unit of `MomentOfInertia`: `n` of them is `n · 1.6605390666e-47` coherent units.
    pub const DALTON_SQUARE_ANGSTROM: SIUnit<
        MOMENT_OF_INERTIA,
        {
            SIUnitConsts {
                size: Exact::dec(16605390666, -57),
                zero: Exact::ZERO,
                symbol: "Da · Å²",
                name: "dalton square ångströms",
            }
        },
    > = SIUnit;

    /// Pascals (`Pa`), a unit of `Pressure`: `n` of them is `n · 1.0` coherent units.
    pub const PASCAL: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Pa",
                name: "pascals",
            }
        },
    > = SIUnit;

    /// Standard atmospheres (`atm`), a unit of `Pressure`: `n` of them is `n · 101325.0` coherent units.
    pub const STANDARD_ATMOSPHERE: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(101325),
                zero: Exact::ZERO,
                symbol: "atm",
                name: "standard atmospheres",
            }
        },
    > = SIUnit;

    /// Atmospheres (technical) (`at`), a unit of `Pressure`: `n` of them is `n · 98066.5` coherent units.
    pub const ATMOSPHERE_TECHNICAL: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -1),
                zero: Exact::ZERO,
                symbol: "at",
                name: "atmospheres (technical)",
            }
        },
    > = SIUnit;

    /// Bar (`bar`), a unit of `Pressure`: `n` of them is `n · 100000.0` coherent units.
    pub const BAR: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(100000),
                zero: Exact::ZERO,
                symbol: "bar",
                name: "bar",
            }
        },
    > = SIUnit;

    /// Centimetres of mercury (`cm Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 10.0` coherent units.
    pub const CENTIMETRE_OF_MERCURY: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(133322387415, -9) * Exact::int(10),
                zero: Exact::ZERO,
                symbol: "cm Hg",
                name: "centimetres of mercury",
            }
        },
    > = SIUnit;

    /// Centimetres of water (`cm H₂O`), a unit of `Pressure`: `n` of them is `n · 98.0665` coherent units.
    pub const CENTIMETRE_OF_WATER: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -4),
                zero: Exact::ZERO,
                symbol: "cm H₂O",
                name: "centimetres of water",
            }
        },
    > = SIUnit;

    /// Feet of mercury (`ft Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 304.8` coherent units.
    pub const FOOT_OF_MERCURY: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(133322387415, -9) * Exact::dec(3048, -1),
                zero: Exact::ZERO,
                symbol: "ft Hg",
                name: "feet of mercury",
            }
        },
    > = SIUnit;

    /// Feet of water (`ft H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665 * 304.8` coherent units.
    pub const FOOT_OF_WATER: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5) * Exact::dec(3048, -1),
                zero: Exact::ZERO,
                symbol: "ft H₂O",
                name: "feet of water",
            }
        },
    > = SIUnit;

    /// Inches of mercury (`in Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 25.4` coherent units.
    pub const INCH_OF_MERCURY: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(133322387415, -9) * Exact::dec(254, -1),
                zero: Exact::ZERO,
                symbol: "in Hg",
                name: "inches of mercury",
            }
        },
    > = SIUnit;

    /// Inches of water (`in H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665 * 25.4` coherent units.
    pub const INCH_OF_WATER: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5) * Exact::dec(254, -1),
                zero: Exact::ZERO,
                symbol: "in H₂O",
                name: "inches of water",
            }
        },
    > = SIUnit;

    /// Kips per square inch (`kip/in²`), a unit of `Pressure`: `n` of them is `n · (1000.0 * 0.45359237 * 9.80665) / (0.0254 * 0.0254)` coherent units.
    pub const KIP_PER_SQUARE_INCH: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: (Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))
                    / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "kip/in²",
                name: "kips per square inch",
            }
        },
    > = SIUnit;

    /// Millimetres of mercury (`mm Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415` coherent units.
    pub const MILLIMETRE_OF_MERCURY: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(133322387415, -9),
                zero: Exact::ZERO,
                symbol: "mm Hg",
                name: "millimetres of mercury",
            }
        },
    > = SIUnit;

    /// Millimetres of water (`mm H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665` coherent units.
    pub const MILLIMETRE_OF_WATER: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "mm H₂O",
                name: "millimetres of water",
            }
        },
    > = SIUnit;

    /// Poundals per square foot (`pdl/ft²`), a unit of `Pressure`: `n` of them is `n · (0.45359237 * 0.3048) / (0.3048 * 0.3048)` coherent units.
    pub const POUNDAL_PER_SQUARE_FOOT: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(3048, -4))
                    / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "pdl/ft²",
                name: "poundals per square foot",
            }
        },
    > = SIUnit;

    /// Pounds-force per square foot (`lbf/ft²`), a unit of `Pressure`: `n` of them is `n · (0.45359237 * 9.80665) / (0.3048 * 0.3048)` coherent units.
    pub const POUND_FORCE_PER_SQUARE_FOOT: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5))
                    / (Exact::dec(3048, -4) * Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "lbf/ft²",
                name: "pounds-force per square foot",
            }
        },
    > = SIUnit;

    /// Pounds-force per square inch (`lbf/in²`), a unit of `Pressure`: `n` of them is `n · (0.45359237 * 9.80665) / (0.0254 * 0.0254)` coherent units.
    pub const POUND_FORCE_PER_SQUARE_INCH: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5))
                    / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "lbf/in²",
                name: "pounds-force per square inch",
            }
        },
    > = SIUnit;

    /// Pounds-force per square inch (`psi`), a unit of `Pressure`: `n` of them is `n · 0.45359237 * 9.80665 / (0.0254 * 0.0254)` coherent units.
    pub const PSI: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5)
                    / (Exact::dec(254, -4) * Exact::dec(254, -4)),
                zero: Exact::ZERO,
                symbol: "psi",
                name: "pounds-force per square inch",
            }
        },
    > = SIUnit;

    /// Torr (`Torr`), a unit of `Pressure`: `n` of them is `n · 101325.0 / 760.0` coherent units.
    pub const TORR: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(101325) / Exact::int(760),
                zero: Exact::ZERO,
                symbol: "Torr",
                name: "torr",
            }
        },
    > = SIUnit;

    /// Dynes per square centimetre (`dyn/cm²`), a unit of `Pressure`: `n` of them is `n · 0.1` coherent units.
    pub const DYNE_PER_SQUARE_CENTIMETRE: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "dyn/cm²",
                name: "dynes per square centimetre",
            }
        },
    > = SIUnit;

    /// Newtons per square millimetre (`N/mm²`), a unit of `Pressure`: `n` of them is `n · 1000000.0` coherent units.
    pub const NEWTON_PER_SQUARE_MILLIMETRE: SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(1000000),
                zero: Exact::ZERO,
                symbol: "N/mm²",
                name: "newtons per square millimetre",
            }
        },
    > = SIUnit;

    /// Hertz (`Hz`), a unit of `Frequency`: `n` of them is `n · 1.0` coherent units.
    pub const HERTZ: SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Hz",
                name: "hertz",
            }
        },
    > = SIUnit;

    /// Cycles per day (`1/d`), a unit of `Frequency`: `n` of them is `n · 1.0 / 86400.0` coherent units.
    pub const CYCLE_PER_DAY: SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "1/d",
                name: "cycles per day",
            }
        },
    > = SIUnit;

    /// Cycles per hour (`1/h`), a unit of `Frequency`: `n` of them is `n · 1.0 / 3600.0` coherent units.
    pub const CYCLE_PER_HOUR: SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "1/h",
                name: "cycles per hour",
            }
        },
    > = SIUnit;

    /// Cycles per minute (`1/min`), a unit of `Frequency`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub const CYCLE_PER_MINUTE: SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "1/min",
                name: "cycles per minute",
            }
        },
    > = SIUnit;

    /// Cycles per shake (`100 MHz`), a unit of `Frequency`: `n` of them is `n · 100000000.0` coherent units.
    pub const CYCLE_PER_SHAKE: SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(100000000),
                zero: Exact::ZERO,
                symbol: "100 MHz",
                name: "cycles per shake",
            }
        },
    > = SIUnit;

    /// Cycles per year (`1/a`), a unit of `Frequency`: `n` of them is `n · 1.0 / 31536000.0` coherent units.
    pub const CYCLE_PER_YEAR: SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(31536000),
                zero: Exact::ZERO,
                symbol: "1/a",
                name: "cycles per year",
            }
        },
    > = SIUnit;

    /// Hertz per second (`Hz/s`), a unit of `FrequencyDrift`: `n` of them is `n · 1.0` coherent units.
    pub const HERTZ_PER_SECOND: SIUnit<
        FREQUENCY_DRIFT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Hz/s",
                name: "hertz per second",
            }
        },
    > = SIUnit;

    /// Reciprocal metres (`m⁻¹`), a unit of `ReciprocalLength`: `n` of them is `n · 1.0` coherent units.
    pub const RECIPROCAL_METRE: SIUnit<
        RECIPROCAL_LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻¹",
                name: "reciprocal metres",
            }
        },
    > = SIUnit;

    /// Reciprocal ångströms (`Å⁻¹`), a unit of `ReciprocalLength`: `n` of them is `n · 10000000000.0` coherent units.
    pub const RECIPROCAL_ANGSTROM: SIUnit<
        RECIPROCAL_LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(10000000000),
                zero: Exact::ZERO,
                symbol: "Å⁻¹",
                name: "reciprocal ångströms",
            }
        },
    > = SIUnit;

    /// Diopters (`dpt`), a unit of `ReciprocalLength`: `n` of them is `n · 1.0` coherent units.
    pub const DIOPTER: SIUnit<
        RECIPROCAL_LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "dpt",
                name: "diopters",
            }
        },
    > = SIUnit;

    /// Rydberg constants (`R∞`), a unit of `ReciprocalLength`: `n` of them is `n · 10973731.56816` coherent units.
    pub const RYDBERG_CONSTANT: SIUnit<
        RECIPROCAL_LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1097373156816, -5),
                zero: Exact::ZERO,
                symbol: "R∞",
                name: "Rydberg constants",
            }
        },
    > = SIUnit;

    /// Radians per metre (`rad/m`), a unit of `Curvature`: `n` of them is `n · 1.0` coherent units.
    pub const RADIAN_PER_METRE: SIUnit<
        CURVATURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/m",
                name: "radians per metre",
            }
        },
    > = SIUnit;

    /// Degrees per metre (`°/m`), a unit of `Curvature`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub const DEGREE_PER_METRE: SIUnit<
        CURVATURE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°/m",
                name: "degrees per metre",
            }
        },
    > = SIUnit;

    /// Square metres per kilogram (`m²/kg`), a unit of `SpecificArea`: `n` of them is `n · 1.0` coherent units.
    pub const SQUARE_METRE_PER_KILOGRAM: SIUnit<
        SPECIFIC_AREA,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/kg",
                name: "square metres per kilogram",
            }
        },
    > = SIUnit;

    /// Square metres per gram (`m²/g`), a unit of `SpecificArea`: `n` of them is `n · 1000.0` coherent units.
    pub const SQUARE_METRE_PER_GRAM: SIUnit<
        SPECIFIC_AREA,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "m²/g",
                name: "square metres per gram",
            }
        },
    > = SIUnit;

    /// Cubic metres per kilogram (`m³/kg`), a unit of `SpecificVolume`: `n` of them is `n · 1.0` coherent units.
    pub const CUBIC_METRE_PER_KILOGRAM: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³/kg",
                name: "cubic metres per kilogram",
            }
        },
    > = SIUnit;

    /// Cubic metres per gram (`m³/g`), a unit of `SpecificVolume`: `n` of them is `n · 1000.0` coherent units.
    pub const CUBIC_METRE_PER_GRAM: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "m³/g",
                name: "cubic metres per gram",
            }
        },
    > = SIUnit;

    /// Cubic metres per carat (`m³/ct`), a unit of `SpecificVolume`: `n` of them is `n · 5000.0` coherent units.
    pub const CUBIC_METRE_PER_CARAT: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(5000),
                zero: Exact::ZERO,
                symbol: "m³/ct",
                name: "cubic metres per carat",
            }
        },
    > = SIUnit;

    /// Cubic metres per grain (`m³/gr`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 6.479891e-05` coherent units.
    pub const CUBIC_METRE_PER_GRAIN: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "m³/gr",
                name: "cubic metres per grain",
            }
        },
    > = SIUnit;

    /// Cubic metres per hundredweight (long) (`m³/cwt long`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 50.80234544` coherent units.
    pub const CUBIC_METRE_PER_HUNDREDWEIGHT_LONG: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(5080234544, -8),
                zero: Exact::ZERO,
                symbol: "m³/cwt long",
                name: "cubic metres per hundredweight (long)",
            }
        },
    > = SIUnit;

    /// Cubic metres per hundredweight (short) (`m³/cwt short`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 45.359237` coherent units.
    pub const CUBIC_METRE_PER_HUNDREDWEIGHT_SHORT: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(45359237, -6),
                zero: Exact::ZERO,
                symbol: "m³/cwt short",
                name: "cubic metres per hundredweight (short)",
            }
        },
    > = SIUnit;

    /// Cubic metres per ounce (`m³/oz`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.028349523125` coherent units.
    pub const CUBIC_METRE_PER_OUNCE: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "m³/oz",
                name: "cubic metres per ounce",
            }
        },
    > = SIUnit;

    /// Cubic metres per troy ounce (`m³/oz t`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.0311034768` coherent units.
    pub const CUBIC_METRE_PER_OUNCE_TROY: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(311034768, -10),
                zero: Exact::ZERO,
                symbol: "m³/oz t",
                name: "cubic metres per troy ounce",
            }
        },
    > = SIUnit;

    /// Cubic metres per pennyweight (`m³/dwt`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.00155517384` coherent units.
    pub const CUBIC_METRE_PER_PENNYWEIGHT: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(155517384, -11),
                zero: Exact::ZERO,
                symbol: "m³/dwt",
                name: "cubic metres per pennyweight",
            }
        },
    > = SIUnit;

    /// Cubic metres per pound (`m³/lb`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.45359237` coherent units.
    pub const CUBIC_METRE_PER_POUND: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "m³/lb",
                name: "cubic metres per pound",
            }
        },
    > = SIUnit;

    /// Cubic metres per troy pound (`m³/lb t`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.3732417216` coherent units.
    pub const CUBIC_METRE_PER_POUND_TROY: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3732417216, -10),
                zero: Exact::ZERO,
                symbol: "m³/lb t",
                name: "cubic metres per troy pound",
            }
        },
    > = SIUnit;

    /// Cubic metres per slug (`m³/slug`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / (0.45359237 * 9.80665 / 0.3048)` coherent units.
    pub const CUBIC_METRE_PER_SLUG: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1)
                    / (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "m³/slug",
                name: "cubic metres per slug",
            }
        },
    > = SIUnit;

    /// Cubic metres per assay ton (`m³/AT`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / (0.175 / 6.0)` coherent units.
    pub const CUBIC_METRE_PER_TON_ASSAY: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(175, -3) / Exact::int(6)),
                zero: Exact::ZERO,
                symbol: "m³/AT",
                name: "cubic metres per assay ton",
            }
        },
    > = SIUnit;

    /// Cubic metres per long ton (`m³/2240 lb`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 1016.0469088` coherent units.
    pub const CUBIC_METRE_PER_TON_LONG: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "m³/2240 lb",
                name: "cubic metres per long ton",
            }
        },
    > = SIUnit;

    /// Cubic metres per short ton (`m³/2000 lb`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 907.18474` coherent units.
    pub const CUBIC_METRE_PER_TON_SHORT: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "m³/2000 lb",
                name: "cubic metres per short ton",
            }
        },
    > = SIUnit;

    /// Cubic metres per ton (`m³/t`), a unit of `SpecificVolume`: `n` of them is `n · 0.001` coherent units.
    pub const CUBIC_METRE_PER_TON: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "m³/t",
                name: "cubic metres per ton",
            }
        },
    > = SIUnit;

    /// Gallons per grain (`gal/gr`), a unit of `SpecificVolume`: `n` of them is `n · 0.003785411784 / 6.479891e-05` coherent units.
    pub const GALLON_PER_GRAIN: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "gal/gr",
                name: "gallons per grain",
            }
        },
    > = SIUnit;

    /// Cubic inches per ounce (`in³/oz`), a unit of `SpecificVolume`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254 / 0.028349523125` coherent units.
    pub const CUBIC_INCH_PER_OUNCE: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)
                    / Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "in³/oz",
                name: "cubic inches per ounce",
            }
        },
    > = SIUnit;

    /// Imperial gallons per ounce (`gal (UK)/oz`), a unit of `SpecificVolume`: `n` of them is `n · 0.00454609 / 0.028349523125` coherent units.
    pub const GALLON_IMPERIAL_PER_OUNCE: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(454609, -8) / Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "gal (UK)/oz",
                name: "Imperial gallons per ounce",
            }
        },
    > = SIUnit;

    /// Gallons per ounce (`gal/oz`), a unit of `SpecificVolume`: `n` of them is `n · 0.003785411784 / 0.028349523125` coherent units.
    pub const GALLON_PER_OUNCE: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "gal/oz",
                name: "gallons per ounce",
            }
        },
    > = SIUnit;

    /// Cubic feet per pound (`ft³/lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.3048 * 0.3048 * 0.3048) / 0.45359237` coherent units.
    pub const CUBIC_FOOT_PER_POUND: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))
                    / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "ft³/lb",
                name: "cubic feet per pound",
            }
        },
    > = SIUnit;

    /// Cubic inches per pound (`in³/lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.0254 * 0.0254 * 0.0254) / 0.45359237` coherent units.
    pub const CUBIC_INCH_PER_POUND: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4))
                    / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "in³/lb",
                name: "cubic inches per pound",
            }
        },
    > = SIUnit;

    /// Cubic yards per pound (`yd³/lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.9144 * 0.9144 * 0.9144) / 0.45359237` coherent units.
    pub const CUBIC_YARD_PER_POUND: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))
                    / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "yd³/lb",
                name: "cubic yards per pound",
            }
        },
    > = SIUnit;

    /// Imperial gallons per pound (`gal (UK)/lb`), a unit of `SpecificVolume`: `n` of them is `n · 0.00454609 / 0.45359237` coherent units.
    pub const GALLON_IMPERIAL_PER_POUND: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(454609, -8) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "gal (UK)/lb",
                name: "Imperial gallons per pound",
            }
        },
    > = SIUnit;

    /// Gallons per pound (`gal/lb`), a unit of `SpecificVolume`: `n` of them is `n · 0.003785411784 / 0.45359237` coherent units.
    pub const GALLON_PER_POUND: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "gal/lb",
                name: "gallons per pound",
            }
        },
    > = SIUnit;

    /// Cubic feett per slug (`ft³/slug`), a unit of `SpecificVolume`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048 / (0.45359237 * 9.80665 / 0.3048)` coherent units.
    pub const CUBIC_FOOT_PER_SLUG: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)
                    / (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)),
                zero: Exact::ZERO,
                symbol: "ft³/slug",
                name: "cubic feett per slug",
            }
        },
    > = SIUnit;

    /// Cubic yards per long ton (`yd³/2240 lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.9144 * 0.9144 * 0.9144) / 1016.0469088` coherent units.
    pub const CUBIC_YARD_PER_TON_LONG: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))
                    / Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "yd³/2240 lb",
                name: "cubic yards per long ton",
            }
        },
    > = SIUnit;

    /// Cubic yards per short ton (`yd³/2000 lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.9144 * 0.9144 * 0.9144) / 907.18474` coherent units.
    pub const CUBIC_YARD_PER_TON_SHORT: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4))
                    / Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "yd³/2000 lb",
                name: "cubic yards per short ton",
            }
        },
    > = SIUnit;

    /// Cubic feet per long ton (`ft³/2240 lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.3048 * 0.3048 * 0.3048) / 1016.0469088` coherent units.
    pub const CUBIC_FOOT_PER_TON_LONG: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))
                    / Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "ft³/2240 lb",
                name: "cubic feet per long ton",
            }
        },
    > = SIUnit;

    /// Cubic feet per short ton (`ft³/2000 lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.3048 * 0.3048 * 0.3048) / 907.18474` coherent units.
    pub const CUBIC_FOOT_PER_TON_SHORT: SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4))
                    / Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "ft³/2000 lb",
                name: "cubic feet per short ton",
            }
        },
    > = SIUnit;

    /// Watts per kilogram (`W/kg`), a unit of `SpecificPower`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_PER_KILOGRAM: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/kg",
                name: "watts per kilogram",
            }
        },
    > = SIUnit;

    /// Square metres per cubic second (`m² · s⁻³`), a unit of `SpecificPower`: `n` of them is `n · 1.0` coherent units.
    pub const SQUARE_METRE_PER_CUBIC_SECOND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m² · s⁻³",
                name: "square metres per cubic second",
            }
        },
    > = SIUnit;

    /// Ergs per second per kilogram (`(erg/s)/kg`), a unit of `SpecificPower`: `n` of them is `n · 1e-07` coherent units.
    pub const ERG_PER_SECOND_PER_KILOGRAM: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "(erg/s)/kg",
                name: "ergs per second per kilogram",
            }
        },
    > = SIUnit;

    /// Foot pounds-force per hour per pound (`(ft · lbf/h)/lb`), a unit of `SpecificPower`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / (3600.0 * 0.45359237)` coherent units.
    pub const FOOT_POUND_PER_HOUR_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))
                    / (Exact::int(3600) * Exact::dec(45359237, -8)),
                zero: Exact::ZERO,
                symbol: "(ft · lbf/h)/lb",
                name: "foot pounds-force per hour per pound",
            }
        },
    > = SIUnit;

    /// Foot pounds-force per minute per pound (`(ft · lbf/min)/lb`), a unit of `SpecificPower`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / (60.0 * 0.45359237)` coherent units.
    pub const FOOT_POUND_PER_MINUTE_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))
                    / (Exact::int(60) * Exact::dec(45359237, -8)),
                zero: Exact::ZERO,
                symbol: "(ft · lbf/min)/lb",
                name: "foot pounds-force per minute per pound",
            }
        },
    > = SIUnit;

    /// Foot pounds-force per second per pound (`(ft · lbf/s)/lb`), a unit of `SpecificPower`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / 0.45359237` coherent units.
    pub const FOOT_POUND_PER_SECOND_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))
                    / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(ft · lbf/s)/lb",
                name: "foot pounds-force per second per pound",
            }
        },
    > = SIUnit;

    /// Horsepower per pound (`hp/lb`), a unit of `SpecificPower`: `n` of them is `n · (550.0 * 0.3048 * 0.45359237 * 9.80665) / 0.45359237` coherent units.
    pub const HORSEPOWER_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: (Exact::int(550)
                    * Exact::dec(3048, -4)
                    * Exact::dec(45359237, -8)
                    * Exact::dec(980665, -5))
                    / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "hp/lb",
                name: "horsepower per pound",
            }
        },
    > = SIUnit;

    /// Horsepower (boiler) per pound (`(hp (S))/lb`), a unit of `SpecificPower`: `n` of them is `n · 9809.5 / 0.45359237` coherent units.
    pub const HORSEPOWER_BOILER_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::dec(98095, -1) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (S))/lb",
                name: "horsepower (boiler) per pound",
            }
        },
    > = SIUnit;

    /// Horsepower (electric) per pound (`(hp (E))/lb`), a unit of `SpecificPower`: `n` of them is `n · 746.0 / 0.45359237` coherent units.
    pub const HORSEPOWER_ELECTRIC_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::int(746) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (E))/lb",
                name: "horsepower (electric) per pound",
            }
        },
    > = SIUnit;

    /// Metric horsepower per pound (`(hp (M))/lb`), a unit of `SpecificPower`: `n` of them is `n · (75.0 * 9.80665) / 0.45359237` coherent units.
    pub const HORSEPOWER_METRIC_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: (Exact::int(75) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (M))/lb",
                name: "metric horsepower per pound",
            }
        },
    > = SIUnit;

    /// Horsepower (Imperial) per pound (`(hp (I))/lb`), a unit of `SpecificPower`: `n` of them is `n · 745.7 / 0.45359237` coherent units.
    pub const HORSEPOWER_IMPERIAL_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::dec(7457, -1) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (I))/lb",
                name: "horsepower (Imperial) per pound",
            }
        },
    > = SIUnit;

    /// Hydraulic horsepower per pound (`(hp (hydraulic))/lb`), a unit of `SpecificPower`: `n` of them is `n · 746.043 / 0.45359237` coherent units.
    pub const HYDRAULIC_HORSEPOWER_PER_POUND: SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::dec(746043, -3) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (hydraulic))/lb",
                name: "hydraulic horsepower per pound",
            }
        },
    > = SIUnit;

    /// Cubic metres per kilogram second squared (`m³/(kg · s²)`), a unit of `SpecificGravitationalParameter`: `n` of them is `n · 1.0` coherent units.
    pub const CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED: SIUnit<
        SPECIFIC_GRAVITATIONAL_PARAMETER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³/(kg · s²)",
                name: "cubic metres per kilogram second squared",
            }
        },
    > = SIUnit;

    /// Newtonian constants of gravitation (`G`), a unit of `SpecificGravitationalParameter`: `n` of them is `n · 6.6743e-11` coherent units.
    pub const GRAVITATIONAL_CONSTANT: SIUnit<
        SPECIFIC_GRAVITATIONAL_PARAMETER,
        {
            SIUnitConsts {
                size: Exact::dec(66743, -15),
                zero: Exact::ZERO,
                symbol: "G",
                name: "Newtonian constants of gravitation",
            }
        },
    > = SIUnit;
}
