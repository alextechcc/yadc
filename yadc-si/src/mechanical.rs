use crate::{ISQ, Kind, SIQuantity};
use yadc::Dimension;

/// The dimension of `Length`.
pub const LENGTH: ISQ = ISQ {
    m: 1,
    ..ISQ::DIMENSIONLESS
};
pub type Length<N> = SIQuantity<LENGTH, N>;

/// The dimension of `Mass`.
pub const MASS: ISQ = ISQ {
    kg: 1,
    ..ISQ::DIMENSIONLESS
};
pub type Mass<N> = SIQuantity<MASS, N>;

/// The dimension of `Time`.
pub const TIME: ISQ = ISQ {
    s: 1,
    ..ISQ::DIMENSIONLESS
};
pub type Time<N> = SIQuantity<TIME, N>;

/// The dimension of `Instant`.
pub const INSTANT: ISQ = ISQ {
    s: 1,
    kind: Kind::Affine,
    ..ISQ::DIMENSIONLESS
};
pub type Instant<N> = SIQuantity<INSTANT, N>;

/// The dimension of `Area`.
pub const AREA: ISQ = ISQ {
    m: 2,
    ..ISQ::DIMENSIONLESS
};
pub type Area<N> = SIQuantity<AREA, N>;

/// The dimension of `Volume`.
pub const VOLUME: ISQ = ISQ {
    m: 3,
    ..ISQ::DIMENSIONLESS
};
pub type Volume<N> = SIQuantity<VOLUME, N>;

/// The dimension of `Velocity`.
pub const VELOCITY: ISQ = ISQ {
    m: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type Velocity<N> = SIQuantity<VELOCITY, N>;

/// The dimension of `Acceleration`.
pub const ACCELERATION: ISQ = ISQ {
    m: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type Acceleration<N> = SIQuantity<ACCELERATION, N>;

/// The dimension of `Jerk`.
pub const JERK: ISQ = ISQ {
    m: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};
pub type Jerk<N> = SIQuantity<JERK, N>;

/// The dimension of `Absement`.
pub const ABSEMENT: ISQ = ISQ {
    m: 1,
    s: 1,
    ..ISQ::DIMENSIONLESS
};
pub type Absement<N> = SIQuantity<ABSEMENT, N>;

/// The dimension of `InverseVelocity`.
pub const INVERSE_VELOCITY: ISQ = ISQ {
    m: -1,
    s: 1,
    ..ISQ::DIMENSIONLESS
};
pub type InverseVelocity<N> = SIQuantity<INVERSE_VELOCITY, N>;

/// The dimension of `Force`.
pub const FORCE: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type Force<N> = SIQuantity<FORCE, N>;

/// The dimension of `Momentum`.
pub const MOMENTUM: ISQ = ISQ {
    m: 1,
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type Momentum<N> = SIQuantity<MOMENTUM, N>;

/// The dimension of `Energy`.
pub const ENERGY: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type Energy<N> = SIQuantity<ENERGY, N>;

/// The dimension of `AvailableEnergy`.
pub const AVAILABLE_ENERGY: ISQ = ISQ {
    m: 2,
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type AvailableEnergy<N> = SIQuantity<AVAILABLE_ENERGY, N>;

/// The dimension of `Power`.
pub const POWER: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    ..ISQ::DIMENSIONLESS
};
pub type Power<N> = SIQuantity<POWER, N>;

/// The dimension of `Gain`.
pub const GAIN: ISQ = ISQ {
    kind: Kind::Log,
    ..ISQ::DIMENSIONLESS
};
pub type Gain<N> = SIQuantity<GAIN, N>;

/// The dimension of `PowerLevel`.
pub const POWER_LEVEL: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -3,
    kind: Kind::Log,
    ..ISQ::DIMENSIONLESS
};
pub type PowerLevel<N> = SIQuantity<POWER_LEVEL, N>;

/// The dimension of `PowerRate`.
pub const POWER_RATE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -4,
    ..ISQ::DIMENSIONLESS
};
pub type PowerRate<N> = SIQuantity<POWER_RATE, N>;

/// The dimension of `Action`.
pub const ACTION: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type Action<N> = SIQuantity<ACTION, N>;

/// The dimension of `Torque`.
pub const TORQUE: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    rad: -1,
    ..ISQ::DIMENSIONLESS
};
pub type Torque<N> = SIQuantity<TORQUE, N>;

/// The dimension of `MomentOfInertia`.
pub const MOMENT_OF_INERTIA: ISQ = ISQ {
    m: 2,
    kg: 1,
    rad: -2,
    ..ISQ::DIMENSIONLESS
};
pub type MomentOfInertia<N> = SIQuantity<MOMENT_OF_INERTIA, N>;

/// The dimension of `Pressure`.
pub const PRESSURE: ISQ = ISQ {
    m: -1,
    kg: 1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type Pressure<N> = SIQuantity<PRESSURE, N>;

/// The dimension of `Frequency`.
pub const FREQUENCY: ISQ = ISQ {
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type Frequency<N> = SIQuantity<FREQUENCY, N>;

/// The dimension of `FrequencyDrift`.
pub const FREQUENCY_DRIFT: ISQ = ISQ {
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type FrequencyDrift<N> = SIQuantity<FREQUENCY_DRIFT, N>;

/// The dimension of `ReciprocalLength`.
pub const RECIPROCAL_LENGTH: ISQ = ISQ {
    m: -1,
    ..ISQ::DIMENSIONLESS
};
pub type ReciprocalLength<N> = SIQuantity<RECIPROCAL_LENGTH, N>;

/// The dimension of `Curvature`.
pub const CURVATURE: ISQ = ISQ {
    m: -1,
    rad: 1,
    ..ISQ::DIMENSIONLESS
};
pub type Curvature<N> = SIQuantity<CURVATURE, N>;

/// The dimension of `SpecificArea`.
pub const SPECIFIC_AREA: ISQ = ISQ {
    m: 2,
    kg: -1,
    ..ISQ::DIMENSIONLESS
};
pub type SpecificArea<N> = SIQuantity<SPECIFIC_AREA, N>;

/// The dimension of `SpecificVolume`.
pub const SPECIFIC_VOLUME: ISQ = ISQ {
    m: 3,
    kg: -1,
    ..ISQ::DIMENSIONLESS
};
pub type SpecificVolume<N> = SIQuantity<SPECIFIC_VOLUME, N>;

/// The dimension of `SpecificPower`.
pub const SPECIFIC_POWER: ISQ = ISQ {
    m: 2,
    s: -3,
    ..ISQ::DIMENSIONLESS
};
pub type SpecificPower<N> = SIQuantity<SPECIFIC_POWER, N>;

/// The dimension of `SpecificGravitationalParameter`.
pub const SPECIFIC_GRAVITATIONAL_PARAMETER: ISQ = ISQ {
    m: 3,
    kg: -1,
    s: -2,
    ..ISQ::DIMENSIONLESS
};
pub type SpecificGravitationalParameter<N> = SIQuantity<SPECIFIC_GRAVITATIONAL_PARAMETER, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Metres (`m`), a unit of `Length`: `n` of them is `n · 1.0` coherent units.
    pub type Metre = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m",
                name: "metres",
            }
        },
    >;

    /// Ångströms (`Å`), a unit of `Length`: `n` of them is `n · 1e-10` coherent units.
    pub type Angstrom = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1, -10),
                zero: Exact::ZERO,
                symbol: "Å",
                name: "ångströms",
            }
        },
    >;

    /// Bohr radiuses (`a₀`), a unit of `Length`: `n` of them is `n · 5.29177210903e-11` coherent units.
    pub type BohrRadius = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(529177210903, -22),
                zero: Exact::ZERO,
                symbol: "a₀",
                name: "bohr radiuses",
            }
        },
    >;

    /// Atomic units of length (`a.u. of length`), a unit of `Length`: `n` of them is `n · 5.29177210903e-11` coherent units.
    pub type AtomicUnitOfLength = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(529177210903, -22),
                zero: Exact::ZERO,
                symbol: "a.u. of length",
                name: "atomic units of length",
            }
        },
    >;

    /// Astronomical units (`ua`), a unit of `Length`: `n` of them is `n · 149597870700.0` coherent units.
    pub type AstronomicalUnit = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(149597870700),
                zero: Exact::ZERO,
                symbol: "ua",
                name: "astronomical units",
            }
        },
    >;

    /// Chains (`ch`), a unit of `Length`: `n` of them is `n · 66.0 * 1200.0 / 3937.0` coherent units.
    pub type Chain = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(66) * Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "ch",
                name: "chains",
            }
        },
    >;

    /// Data miles (`DM`), a unit of `Length`: `n` of them is `n · 1828.8` coherent units.
    pub type DataMile = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(18288, -1),
                zero: Exact::ZERO,
                symbol: "DM",
                name: "data miles",
            }
        },
    >;

    /// Fathoms (`fathom`), a unit of `Length`: `n` of them is `n · 6.0 * 1200.0 / 3937.0` coherent units.
    pub type Fathom = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(6) * Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "fathom",
                name: "fathoms",
            }
        },
    >;

    /// Fermis (`fermi`), a unit of `Length`: `n` of them is `n · 1e-15` coherent units.
    pub type Fermi = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1, -15),
                zero: Exact::ZERO,
                symbol: "fermi",
                name: "fermis",
            }
        },
    >;

    /// Feet (`ft`), a unit of `Length`: `n` of them is `n · 0.3048` coherent units.
    pub type Foot = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft",
                name: "feet",
            }
        },
    >;

    /// Feet (U.S. survey) (`ft (U.S. survey)`), a unit of `Length`: `n` of them is `n · 1200.0 / 3937.0` coherent units.
    pub type FootSurvey = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "ft (U.S. survey)",
                name: "feet (U.S. survey)",
            }
        },
    >;

    /// Inches (`in`), a unit of `Length`: `n` of them is `n · 0.0254` coherent units.
    pub type Inch = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in",
                name: "inches",
            }
        },
    >;

    /// Light years (`l. y.`), a unit of `Length`: `n` of them is `n · 9460730472580800.0` coherent units.
    pub type LightYear = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(9460730472580800),
                zero: Exact::ZERO,
                symbol: "l. y.",
                name: "light years",
            }
        },
    >;

    /// Microns (`μ`), a unit of `Length`: `n` of them is `n · 1e-06` coherent units.
    pub type Micron = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1, -6),
                zero: Exact::ZERO,
                symbol: "μ",
                name: "microns",
            }
        },
    >;

    /// Mils (`0.001 in`), a unit of `Length`: `n` of them is `n · 2.54e-05` coherent units.
    pub type Mil = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(254, -7),
                zero: Exact::ZERO,
                symbol: "0.001 in",
                name: "mils",
            }
        },
    >;

    /// Miles (`mi`), a unit of `Length`: `n` of them is `n · 1609.344` coherent units.
    pub type Mile = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi",
                name: "miles",
            }
        },
    >;

    /// Miles (U.S. survey) (`mi (U.S. survey)`), a unit of `Length`: `n` of them is `n · 6336000.0 / 3937.0` coherent units.
    pub type MileSurvey = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(6336000) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "mi (U.S. survey)",
                name: "miles (U.S. survey)",
            }
        },
    >;

    /// Nautical miles (`M`), a unit of `Length`: `n` of them is `n · 1852.0` coherent units.
    pub type NauticalMile = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1852),
                zero: Exact::ZERO,
                symbol: "M",
                name: "nautical miles",
            }
        },
    >;

    /// Parsecs (`pc`), a unit of `Length`: `n` of them is `n · 149597870700.0 * 648000.0 / core::f64::consts::PI` coherent units.
    pub type Parsec = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(149597870700) * Exact::int(648000) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "pc",
                name: "parsecs",
            }
        },
    >;

    /// Picas (computer) (`1/6 in (computer)`), a unit of `Length`: `n` of them is `n · 0.0254 / 6.0` coherent units.
    pub type PicaComputer = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "1/6 in (computer)",
                name: "picas (computer)",
            }
        },
    >;

    /// Picas (printer's) (`1/6 in`), a unit of `Length`: `n` of them is `n · 0.004217518` coherent units.
    pub type PicaPrinters = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(4217518, -9),
                zero: Exact::ZERO,
                symbol: "1/6 in",
                name: "picas (printer's)",
            }
        },
    >;

    /// Points (computer) (`1/72 in (computer)`), a unit of `Length`: `n` of them is `n · 0.0254 / 72.0` coherent units.
    pub type PointComputer = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) / Exact::int(72),
                zero: Exact::ZERO,
                symbol: "1/72 in (computer)",
                name: "points (computer)",
            }
        },
    >;

    /// Points (printer's) (`1/72 in`), a unit of `Length`: `n` of them is `n · 0.0003514598` coherent units.
    pub type PointPrinters = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(3514598, -10),
                zero: Exact::ZERO,
                symbol: "1/72 in",
                name: "points (printer's)",
            }
        },
    >;

    /// Rods (`rd`), a unit of `Length`: `n` of them is `n · 16.5 * 1200.0 / 3937.0` coherent units.
    pub type Rod = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(165, -1) * Exact::int(1200) / Exact::int(3937),
                zero: Exact::ZERO,
                symbol: "rd",
                name: "rods",
            }
        },
    >;

    /// Yards (`yd`), a unit of `Length`: `n` of them is `n · 0.9144` coherent units.
    pub type Yard = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd",
                name: "yards",
            }
        },
    >;

    /// Leugae (`leuga`), a unit of `Length`: `n` of them is `n · 2220.0` coherent units.
    pub type Leuga = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(2220),
                zero: Exact::ZERO,
                symbol: "leuga",
                name: "leugae",
            }
        },
    >;

    /// Mille passa (`mille passus`), a unit of `Length`: `n` of them is `n · 1480.0` coherent units.
    pub type MillePassus = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1480),
                zero: Exact::ZERO,
                symbol: "mille passus",
                name: "mille passa",
            }
        },
    >;

    /// Stadia (`stadium`), a unit of `Length`: `n` of them is `n · 185.0` coherent units.
    pub type Stadium = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(185),
                zero: Exact::ZERO,
                symbol: "stadium",
                name: "stadia",
            }
        },
    >;

    /// Acta (`actus`), a unit of `Length`: `n` of them is `n · 35.52` coherent units.
    pub type Actus = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(3552, -2),
                zero: Exact::ZERO,
                symbol: "actus",
                name: "acta",
            }
        },
    >;

    /// Decempedae (`decempeda`), a unit of `Length`: `n` of them is `n · 2.96` coherent units.
    pub type Decempeda = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(296, -2),
                zero: Exact::ZERO,
                symbol: "decempeda",
                name: "decempedae",
            }
        },
    >;

    /// Passa (`passus`), a unit of `Length`: `n` of them is `n · 1.48` coherent units.
    pub type Passus = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(148, -2),
                zero: Exact::ZERO,
                symbol: "passus",
                name: "passa",
            }
        },
    >;

    /// Gradus (`gradus`), a unit of `Length`: `n` of them is `n · 0.74` coherent units.
    pub type Gradus = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(74, -2),
                zero: Exact::ZERO,
                symbol: "gradus",
                name: "gradus",
            }
        },
    >;

    /// Cubita (`cubitum`), a unit of `Length`: `n` of them is `n · 0.444` coherent units.
    pub type Cubitum = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(444, -3),
                zero: Exact::ZERO,
                symbol: "cubitum",
                name: "cubita",
            }
        },
    >;

    /// Palmipedes (`palmipes`), a unit of `Length`: `n` of them is `n · 0.37` coherent units.
    pub type Palmipes = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(37, -2),
                zero: Exact::ZERO,
                symbol: "palmipes",
                name: "palmipedes",
            }
        },
    >;

    /// Pedes (`pes`), a unit of `Length`: `n` of them is `n · 0.296` coherent units.
    pub type Pes = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(296, -3),
                zero: Exact::ZERO,
                symbol: "pes",
                name: "pedes",
            }
        },
    >;

    /// Palmas maior (`palmus maior`), a unit of `Length`: `n` of them is `n · 0.222` coherent units.
    pub type PalmusMaior = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(222, -3),
                zero: Exact::ZERO,
                symbol: "palmus maior",
                name: "palmas maior",
            }
        },
    >;

    /// Palmas (`palmus`), a unit of `Length`: `n` of them is `n · 0.074` coherent units.
    pub type Palmus = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(74, -3),
                zero: Exact::ZERO,
                symbol: "palmus",
                name: "palmas",
            }
        },
    >;

    /// Unciae (`uncia`), a unit of `Length`: `n` of them is `n · 0.296 / 12.0` coherent units.
    pub type LengthUncia = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(296, -3) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "uncia",
                name: "unciae",
            }
        },
    >;

    /// Digiti (`digitus`), a unit of `Length`: `n` of them is `n · 0.0185` coherent units.
    pub type Digitus = SIUnit<
        LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(185, -4),
                zero: Exact::ZERO,
                symbol: "digitus",
                name: "digiti",
            }
        },
    >;

    /// Kilograms (`kg`), a unit of `Mass`: `n` of them is `n · 1.0` coherent units.
    pub type Kilogram = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg",
                name: "kilograms",
            }
        },
    >;

    /// Grams (`g`), a unit of `Mass`: `n` of them is `n · 0.001` coherent units.
    pub type Gram = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g",
                name: "grams",
            }
        },
    >;

    /// Carats (`ct`), a unit of `Mass`: `n` of them is `n · 0.0002` coherent units.
    pub type Carat = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(2, -4),
                zero: Exact::ZERO,
                symbol: "ct",
                name: "carats",
            }
        },
    >;

    /// Daltons (`Da`), a unit of `Mass`: `n` of them is `n · 1.6605390666e-27` coherent units.
    pub type Dalton = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(16605390666, -37),
                zero: Exact::ZERO,
                symbol: "Da",
                name: "daltons",
            }
        },
    >;

    /// Electron masses (`mₑ`), a unit of `Mass`: `n` of them is `n · 9.1093837015e-31` coherent units.
    pub type ElectronMass = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(91093837015, -41),
                zero: Exact::ZERO,
                symbol: "mₑ",
                name: "electron masses",
            }
        },
    >;

    /// Proton masses (`mₚ`), a unit of `Mass`: `n` of them is `n · 1.67262192369e-27` coherent units.
    pub type ProtonMass = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(167262192369, -38),
                zero: Exact::ZERO,
                symbol: "mₚ",
                name: "proton masses",
            }
        },
    >;

    /// Grains (`gr`), a unit of `Mass`: `n` of them is `n · 6.479891e-05` coherent units.
    pub type Grain = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "gr",
                name: "grains",
            }
        },
    >;

    /// Hundredweight (long) (`cwt long`), a unit of `Mass`: `n` of them is `n · 50.80234544` coherent units.
    pub type HundredweightLong = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(5080234544, -8),
                zero: Exact::ZERO,
                symbol: "cwt long",
                name: "hundredweight (long)",
            }
        },
    >;

    /// Hundredweight (short) (`cwt short`), a unit of `Mass`: `n` of them is `n · 45.359237` coherent units.
    pub type HundredweightShort = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -6),
                zero: Exact::ZERO,
                symbol: "cwt short",
                name: "hundredweight (short)",
            }
        },
    >;

    /// Ounces (`oz`), a unit of `Mass`: `n` of them is `n · 0.028349523125` coherent units.
    pub type Ounce = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "oz",
                name: "ounces",
            }
        },
    >;

    /// Troy ounces (`oz t`), a unit of `Mass`: `n` of them is `n · 0.0311034768` coherent units.
    pub type OunceTroy = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(311034768, -10),
                zero: Exact::ZERO,
                symbol: "oz t",
                name: "troy ounces",
            }
        },
    >;

    /// Pennyweight (`dwt`), a unit of `Mass`: `n` of them is `n · 0.00155517384` coherent units.
    pub type Pennyweight = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(155517384, -11),
                zero: Exact::ZERO,
                symbol: "dwt",
                name: "pennyweight",
            }
        },
    >;

    /// Pounds (`lb`), a unit of `Mass`: `n` of them is `n · 0.45359237` coherent units.
    pub type Pound = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "lb",
                name: "pounds",
            }
        },
    >;

    /// Troy pounds (`lb t`), a unit of `Mass`: `n` of them is `n · 0.3732417216` coherent units.
    pub type PoundTroy = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3732417216, -10),
                zero: Exact::ZERO,
                symbol: "lb t",
                name: "troy pounds",
            }
        },
    >;

    /// Slugs (`slug`), a unit of `Mass`: `n` of them is `n · 0.45359237 * 9.80665 / 0.3048` coherent units.
    pub type Slug = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "slug",
                name: "slugs",
            }
        },
    >;

    /// Assay tons (`AT`), a unit of `Mass`: `n` of them is `n · 0.175 / 6.0` coherent units.
    pub type TonAssay = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(175, -3) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "AT",
                name: "assay tons",
            }
        },
    >;

    /// Long tons (`2240 lb`), a unit of `Mass`: `n` of them is `n · 1016.0469088` coherent units.
    pub type TonLong = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "2240 lb",
                name: "long tons",
            }
        },
    >;

    /// Short tons (`2000 lb`), a unit of `Mass`: `n` of them is `n · 907.18474` coherent units.
    pub type TonShort = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "2000 lb",
                name: "short tons",
            }
        },
    >;

    /// Tons (`t`), a unit of `Mass`: `n` of them is `n · 1000.0` coherent units.
    pub type Ton = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "t",
                name: "tons",
            }
        },
    >;

    /// Librae (`libra`), a unit of `Mass`: `n` of them is `n · 0.3289` coherent units.
    pub type Libra = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4),
                zero: Exact::ZERO,
                symbol: "libra",
                name: "librae",
            }
        },
    >;

    /// Deunces (`deunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 11.0 / 12.0` coherent units.
    pub type MassDeunx = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(11) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "deunx",
                name: "deunces",
            }
        },
    >;

    /// Dextantes (`dextans`), a unit of `Mass`: `n` of them is `n · 0.3289 * 10.0 / 12.0` coherent units.
    pub type MassDextans = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(10) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "dextans",
                name: "dextantes",
            }
        },
    >;

    /// Dodrantes (`dodrans`), a unit of `Mass`: `n` of them is `n · 0.246675` coherent units.
    pub type MassDodrans = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(246675, -6),
                zero: Exact::ZERO,
                symbol: "dodrans",
                name: "dodrantes",
            }
        },
    >;

    /// Bessis (`bes`), a unit of `Mass`: `n` of them is `n · 0.3289 * 8.0 / 12.0` coherent units.
    pub type MassBes = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(8) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "bes",
                name: "bessis",
            }
        },
    >;

    /// Septunces (`septunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 7.0 / 12.0` coherent units.
    pub type MassSeptunx = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(7) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "septunx",
                name: "septunces",
            }
        },
    >;

    /// Semisses (`semis`), a unit of `Mass`: `n` of them is `n · 0.16445` coherent units.
    pub type MassSemis = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(16445, -5),
                zero: Exact::ZERO,
                symbol: "semis",
                name: "semisses",
            }
        },
    >;

    /// Quincunx (`quincunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 5.0 / 12.0` coherent units.
    pub type MassQuincunx = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) * Exact::int(5) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "quincunx",
                name: "quincunx",
            }
        },
    >;

    /// Trientes (`triens`), a unit of `Mass`: `n` of them is `n · 0.3289 / 3.0` coherent units.
    pub type MassTriens = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(3),
                zero: Exact::ZERO,
                symbol: "triens",
                name: "trientes",
            }
        },
    >;

    /// Quadrantes teruncius (`quadrans teruncius`), a unit of `Mass`: `n` of them is `n · 0.082225` coherent units.
    pub type QuadransTeruncius = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(82225, -6),
                zero: Exact::ZERO,
                symbol: "quadrans teruncius",
                name: "quadrantes teruncius",
            }
        },
    >;

    /// Sextantes (`sextans`), a unit of `Mass`: `n` of them is `n · 0.3289 / 6.0` coherent units.
    pub type MassSextans = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "sextans",
                name: "sextantes",
            }
        },
    >;

    /// Sescunciae (`sescuncia`), a unit of `Mass`: `n` of them is `n · 0.0411125` coherent units.
    pub type Sescuncia = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(411125, -7),
                zero: Exact::ZERO,
                symbol: "sescuncia",
                name: "sescunciae",
            }
        },
    >;

    /// Unciae (`uncia`), a unit of `Mass`: `n` of them is `n · 0.3289 / 12.0` coherent units.
    pub type MassUncia = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "uncia",
                name: "unciae",
            }
        },
    >;

    /// Semunciae (`semuncia`), a unit of `Mass`: `n` of them is `n · 0.3289 / 24.0` coherent units.
    pub type Semuncia = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(24),
                zero: Exact::ZERO,
                symbol: "semuncia",
                name: "semunciae",
            }
        },
    >;

    /// Duella (`duella`), a unit of `Mass`: `n` of them is `n · 0.3289 / 36.0` coherent units.
    pub type Duella = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(36),
                zero: Exact::ZERO,
                symbol: "duella",
                name: "duella",
            }
        },
    >;

    /// Scilici (`sicilicus`), a unit of `Mass`: `n` of them is `n · 0.3289 / 48.0` coherent units.
    pub type MassSicilicus = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(48),
                zero: Exact::ZERO,
                symbol: "sicilicus",
                name: "scilici",
            }
        },
    >;

    /// Sextulae (`sextula`), a unit of `Mass`: `n` of them is `n · 0.3289 / 72.0` coherent units.
    pub type MassSextula = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(72),
                zero: Exact::ZERO,
                symbol: "sextula",
                name: "sextulae",
            }
        },
    >;

    /// Semisextulae (`semisextula`), a unit of `Mass`: `n` of them is `n · 0.3289 / 144.0` coherent units.
    pub type Semisextula = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(144),
                zero: Exact::ZERO,
                symbol: "semisextula",
                name: "semisextulae",
            }
        },
    >;

    /// Scrupula (`scrupulum`), a unit of `Mass`: `n` of them is `n · 0.3289 / 288.0` coherent units.
    pub type MassScrupulum = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(288),
                zero: Exact::ZERO,
                symbol: "scrupulum",
                name: "scrupula",
            }
        },
    >;

    /// Oboli (`obolus`), a unit of `Mass`: `n` of them is `n · 0.3289 / 576.0` coherent units.
    pub type Obolus = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(576),
                zero: Exact::ZERO,
                symbol: "obolus",
                name: "oboli",
            }
        },
    >;

    /// Siliquae (`siliqua`), a unit of `Mass`: `n` of them is `n · 0.3289 / 1728.0` coherent units.
    pub type Siliqua = SIUnit<
        MASS,
        {
            SIUnitConsts {
                size: Exact::dec(3289, -4) / Exact::int(1728),
                zero: Exact::ZERO,
                symbol: "siliqua",
                name: "siliquae",
            }
        },
    >;

    /// Seconds (`s`), a unit of `Time`: `n` of them is `n · 1.0` coherent units.
    pub type Second = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "s",
                name: "seconds",
            }
        },
    >;

    /// Seconds (sidereal) (`s (sidereal)`), a unit of `Time`: `n` of them is `n · 0.9972696` coherent units.
    pub type SecondSidereal = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::dec(9972696, -7),
                zero: Exact::ZERO,
                symbol: "s (sidereal)",
                name: "seconds (sidereal)",
            }
        },
    >;

    /// Days (`d`), a unit of `Time`: `n` of them is `n · 86400.0` coherent units.
    pub type Day = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "d",
                name: "days",
            }
        },
    >;

    /// Days (sidereal) (`d (sidereal)`), a unit of `Time`: `n` of them is `n · 86164.09` coherent units.
    pub type DaySidereal = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::dec(8616409, -2),
                zero: Exact::ZERO,
                symbol: "d (sidereal)",
                name: "days (sidereal)",
            }
        },
    >;

    /// Hours (`h`), a unit of `Time`: `n` of them is `n · 3600.0` coherent units.
    pub type Hour = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "h",
                name: "hours",
            }
        },
    >;

    /// Hours (sidereal) (`h (sidereal)`), a unit of `Time`: `n` of them is `n · 3590.17` coherent units.
    pub type HourSidereal = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::dec(359017, -2),
                zero: Exact::ZERO,
                symbol: "h (sidereal)",
                name: "hours (sidereal)",
            }
        },
    >;

    /// Minutes (`min`), a unit of `Time`: `n` of them is `n · 60.0` coherent units.
    pub type Minute = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(60),
                zero: Exact::ZERO,
                symbol: "min",
                name: "minutes",
            }
        },
    >;

    /// Shakes (`10.0 ns`), a unit of `Time`: `n` of them is `n · 1e-08` coherent units.
    pub type Shake = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::dec(1, -8),
                zero: Exact::ZERO,
                symbol: "10.0 ns",
                name: "shakes",
            }
        },
    >;

    /// Years (`a`), a unit of `Time`: `n` of them is `n · 31536000.0` coherent units.
    pub type Year = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(31536000),
                zero: Exact::ZERO,
                symbol: "a",
                name: "years",
            }
        },
    >;

    /// Years (sidereal) (`a (sidereal)`), a unit of `Time`: `n` of them is `n · 31558150.0` coherent units.
    pub type YearSidereal = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(31558150),
                zero: Exact::ZERO,
                symbol: "a (sidereal)",
                name: "years (sidereal)",
            }
        },
    >;

    /// Years (tropical) (`a (tropical)`), a unit of `Time`: `n` of them is `n · 31556930.0` coherent units.
    pub type YearTropical = SIUnit<
        TIME,
        {
            SIUnitConsts {
                size: Exact::int(31556930),
                zero: Exact::ZERO,
                symbol: "a (tropical)",
                name: "years (tropical)",
            }
        },
    >;

    /// Seconds of Unix time (`s Unix`), a unit of `Instant`: `n` of them is `n · 1.0` coherent units.
    pub type UnixSecond = SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "s Unix",
                name: "seconds of Unix time",
            }
        },
    >;

    /// Milliseconds of Unix time (`ms Unix`), a unit of `Instant`: `n` of them is `n · 0.001` coherent units.
    pub type UnixMillisecond = SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "ms Unix",
                name: "milliseconds of Unix time",
            }
        },
    >;

    /// Seconds of GPS time (`s GPS`), a unit of `Instant`: `n` of them is `n · 1.0 + 315964800.0` coherent units.
    pub type GpsSecond = SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::int(315964800),
                symbol: "s GPS",
                name: "seconds of GPS time",
            }
        },
    >;

    /// Julian date (`JD`), a unit of `Instant`: `n` of them is `n · 86400.0 + -2440587.5 * 86400.0` coherent units.
    pub type JulianDate = SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::int(86400),
                zero: -Exact::dec(24405875, -1) * Exact::int(86400),
                symbol: "JD",
                name: "Julian date",
            }
        },
    >;

    /// Modified Julian date (`MJD`), a unit of `Instant`: `n` of them is `n · 86400.0 + -40587.0 * 86400.0` coherent units.
    pub type ModifiedJulianDate = SIUnit<
        INSTANT,
        {
            SIUnitConsts {
                size: Exact::int(86400),
                zero: -Exact::int(40587) * Exact::int(86400),
                symbol: "MJD",
                name: "modified Julian date",
            }
        },
    >;

    /// Square metres (`m²`), a unit of `Area`: `n` of them is `n · 1.0` coherent units.
    pub type SquareMetre = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²",
                name: "square metres",
            }
        },
    >;

    /// Acres (`ac`), a unit of `Area`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0)` coherent units.
    pub type Acre = SIUnit<
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
    >;

    /// Ares (`a`), a unit of `Area`: `n` of them is `n · 100.0` coherent units.
    pub type Are = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "a",
                name: "ares",
            }
        },
    >;

    /// Barns (`b`), a unit of `Area`: `n` of them is `n · 1e-28` coherent units.
    pub type Barn = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(1, -28),
                zero: Exact::ZERO,
                symbol: "b",
                name: "barns",
            }
        },
    >;

    /// Circular mils (`cmil`), a unit of `Area`: `n` of them is `n · core::f64::consts::PI / 4.0 * 2.54e-05 * 2.54e-05` coherent units.
    pub type CircularMil = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(4) * Exact::dec(254, -7) * Exact::dec(254, -7),
                zero: Exact::ZERO,
                symbol: "cmil",
                name: "circular mils",
            }
        },
    >;

    /// Hectares (`ha`), a unit of `Area`: `n` of them is `n · 10000.0` coherent units.
    pub type Hectare = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "ha",
                name: "hectares",
            }
        },
    >;

    /// Square feet (`ft²`), a unit of `Area`: `n` of them is `n · 0.09290304` coherent units.
    pub type SquareFoot = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(9290304, -8),
                zero: Exact::ZERO,
                symbol: "ft²",
                name: "square feet",
            }
        },
    >;

    /// Square inches (`in²`), a unit of `Area`: `n` of them is `n · 0.00064516` coherent units.
    pub type SquareInch = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(64516, -8),
                zero: Exact::ZERO,
                symbol: "in²",
                name: "square inches",
            }
        },
    >;

    /// Square miles (`mi²`), a unit of `Area`: `n` of them is `n · 2589988.110336` coherent units.
    pub type SquareMile = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(2589988110336, -6),
                zero: Exact::ZERO,
                symbol: "mi²",
                name: "square miles",
            }
        },
    >;

    /// Square yards (`yd²`), a unit of `Area`: `n` of them is `n · 0.83612736` coherent units.
    pub type SquareYard = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(83612736, -8),
                zero: Exact::ZERO,
                symbol: "yd²",
                name: "square yards",
            }
        },
    >;

    /// Saltus (`saltus`), a unit of `Area`: `n` of them is `n · 2018672.64` coherent units.
    pub type Saltus = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(201867264, -2),
                zero: Exact::ZERO,
                symbol: "saltus",
                name: "saltus",
            }
        },
    >;

    /// Centuriae (`centuria`), a unit of `Area`: `n` of them is `n · 504668.16` coherent units.
    pub type Centuria = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(50466816, -2),
                zero: Exact::ZERO,
                symbol: "centuria",
                name: "centuriae",
            }
        },
    >;

    /// Heredia (`heredium`), a unit of `Area`: `n` of them is `n · 5046.6816` coherent units.
    pub type Heredium = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(50466816, -4),
                zero: Exact::ZERO,
                symbol: "heredium",
                name: "heredia",
            }
        },
    >;

    /// Jugera (`jugerum`), a unit of `Area`: `n` of them is `n · 2523.3408` coherent units.
    pub type Jugerum = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(25233408, -4),
                zero: Exact::ZERO,
                symbol: "jugerum",
                name: "jugera",
            }
        },
    >;

    /// Deunces (`deunx`), a unit of `Area`: `n` of them is `n · 2313.0624` coherent units.
    pub type AreaDeunx = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(23130624, -4),
                zero: Exact::ZERO,
                symbol: "deunx",
                name: "deunces",
            }
        },
    >;

    /// Dextantes (`dextans`), a unit of `Area`: `n` of them is `n · 2102.784` coherent units.
    pub type AreaDextans = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(2102784, -3),
                zero: Exact::ZERO,
                symbol: "dextans",
                name: "dextantes",
            }
        },
    >;

    /// Dodrantes (`dodrans`), a unit of `Area`: `n` of them is `n · 1892.5056` coherent units.
    pub type AreaDodrans = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(18925056, -4),
                zero: Exact::ZERO,
                symbol: "dodrans",
                name: "dodrantes",
            }
        },
    >;

    /// Besses (`bes`), a unit of `Area`: `n` of them is `n · 1682.2272` coherent units.
    pub type AreaBes = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(16822272, -4),
                zero: Exact::ZERO,
                symbol: "bes",
                name: "besses",
            }
        },
    >;

    /// Septunces (`septunx`), a unit of `Area`: `n` of them is `n · 1471.9488` coherent units.
    pub type AreaSeptunx = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(14719488, -4),
                zero: Exact::ZERO,
                symbol: "septunx",
                name: "septunces",
            }
        },
    >;

    /// Semisses (`semis`), a unit of `Area`: `n` of them is `n · 1261.6704` coherent units.
    pub type AreaSemis = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(12616704, -4),
                zero: Exact::ZERO,
                symbol: "semis",
                name: "semisses",
            }
        },
    >;

    /// Acta quadratus (`actus quadratus`), a unit of `Area`: `n` of them is `n · 1261.6704` coherent units.
    pub type ActusQuadratus = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(12616704, -4),
                zero: Exact::ZERO,
                symbol: "actus quadratus",
                name: "acta quadratus",
            }
        },
    >;

    /// Quincunces (`quincunx`), a unit of `Area`: `n` of them is `n · 1051.392` coherent units.
    pub type AreaQuincunx = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(1051392, -3),
                zero: Exact::ZERO,
                symbol: "quincunx",
                name: "quincunces",
            }
        },
    >;

    /// Trientes (`triens`), a unit of `Area`: `n` of them is `n · 841.1136` coherent units.
    pub type AreaTriens = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(8411136, -4),
                zero: Exact::ZERO,
                symbol: "triens",
                name: "trientes",
            }
        },
    >;

    /// Quadrantes (`quadrans`), a unit of `Area`: `n` of them is `n · 630.8352` coherent units.
    pub type Quadrans = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(6308352, -4),
                zero: Exact::ZERO,
                symbol: "quadrans",
                name: "quadrantes",
            }
        },
    >;

    /// Sextantes (`sextans`), a unit of `Area`: `n` of them is `n · 420.5568` coherent units.
    pub type AreaSextans = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(4205568, -4),
                zero: Exact::ZERO,
                symbol: "sextans",
                name: "sextantes",
            }
        },
    >;

    /// Climata (`clima`), a unit of `Area`: `n` of them is `n · 315.4176` coherent units.
    pub type Clima = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(3154176, -4),
                zero: Exact::ZERO,
                symbol: "clima",
                name: "climata",
            }
        },
    >;

    /// Unciae (`uncia`), a unit of `Area`: `n` of them is `n · 210.2784` coherent units.
    pub type AreaUncia = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(2102784, -4),
                zero: Exact::ZERO,
                symbol: "uncia",
                name: "unciae",
            }
        },
    >;

    /// Semiunciae (`semiuncia`), a unit of `Area`: `n` of them is `n · 105.1392` coherent units.
    pub type Semiuncia = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(1051392, -4),
                zero: Exact::ZERO,
                symbol: "semiuncia",
                name: "semiunciae",
            }
        },
    >;

    /// Siculi (`sicilicus`), a unit of `Area`: `n` of them is `n · 52.5696` coherent units.
    pub type AreaSicilicus = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(525696, -4),
                zero: Exact::ZERO,
                symbol: "sicilicus",
                name: "siculi",
            }
        },
    >;

    /// Acta simplex (`actus simplex`), a unit of `Area`: `n` of them is `n · 42.05568` coherent units.
    pub type ActusSimplex = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(4205568, -5),
                zero: Exact::ZERO,
                symbol: "actus simplex",
                name: "acta simplex",
            }
        },
    >;

    /// Sextulae (`sextula`), a unit of `Area`: `n` of them is `n · 35.0464` coherent units.
    pub type AreaSextula = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(350464, -4),
                zero: Exact::ZERO,
                symbol: "sextula",
                name: "sextulae",
            }
        },
    >;

    /// Duo scrupula (`duo scrupulum`), a unit of `Area`: `n` of them is `n · 17.5232` coherent units.
    pub type DuoScrupulum = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(175232, -4),
                zero: Exact::ZERO,
                symbol: "duo scrupulum",
                name: "duo scrupula",
            }
        },
    >;

    /// Scrupula (`scrupulum`), a unit of `Area`: `n` of them is `n · 8.7616` coherent units.
    pub type AreaScrupulum = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(87616, -4),
                zero: Exact::ZERO,
                symbol: "scrupulum",
                name: "scrupula",
            }
        },
    >;

    /// Dimidium scupula (`dimidium scrupulum`), a unit of `Area`: `n` of them is `n · 4.3808` coherent units.
    pub type DimidiumScrupulum = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(43808, -4),
                zero: Exact::ZERO,
                symbol: "dimidium scrupulum",
                name: "dimidium scupula",
            }
        },
    >;

    /// Pedes quadratus (`pes quadratus`), a unit of `Area`: `n` of them is `n · 0.087616` coherent units.
    pub type PesQuadratus = SIUnit<
        AREA,
        {
            SIUnitConsts {
                size: Exact::dec(87616, -6),
                zero: Exact::ZERO,
                symbol: "pes quadratus",
                name: "pedes quadratus",
            }
        },
    >;

    /// Cubic metres (`m³`), a unit of `Volume`: `n` of them is `n · 1.0` coherent units.
    pub type CubicMetre = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³",
                name: "cubic metres",
            }
        },
    >;

    /// Acre-feet (`ac · ft`), a unit of `Volume`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0) * (1200.0 / 3937.0)` coherent units.
    pub type AcreFoot = SIUnit<
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
    >;

    /// Barrels (`bbl`), a unit of `Volume`: `n` of them is `n · 0.158987294928` coherent units.
    pub type Barrel = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(158987294928, -12),
                zero: Exact::ZERO,
                symbol: "bbl",
                name: "barrels",
            }
        },
    >;

    /// Bushels (`bu`), a unit of `Volume`: `n` of them is `n · 0.03523907016688` coherent units.
    pub type Bushel = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3523907016688, -14),
                zero: Exact::ZERO,
                symbol: "bu",
                name: "bushels",
            }
        },
    >;

    /// Cords (`cords`), a unit of `Volume`: `n` of them is `n · 128.0 * 0.3048 * 0.3048 * 0.3048` coherent units.
    pub type Cord = SIUnit<
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
    >;

    /// Cubic feet (`ft³`), a unit of `Volume`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048` coherent units.
    pub type CubicFoot = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft³",
                name: "cubic feet",
            }
        },
    >;

    /// Cubic inches (`in³`), a unit of `Volume`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254` coherent units.
    pub type CubicInch = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in³",
                name: "cubic inches",
            }
        },
    >;

    /// Cubic miles (`mi³`), a unit of `Volume`: `n` of them is `n · 1609.344 * 1609.344 * 1609.344` coherent units.
    pub type CubicMile = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi³",
                name: "cubic miles",
            }
        },
    >;

    /// Cubic yards (`yd³`), a unit of `Volume`: `n` of them is `n · 0.9144 * 0.9144 * 0.9144` coherent units.
    pub type CubicYard = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4),
                zero: Exact::ZERO,
                symbol: "yd³",
                name: "cubic yards",
            }
        },
    >;

    /// Cups (`cup`), a unit of `Volume`: `n` of them is `n · 0.0002365882365` coherent units.
    pub type Cup = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(2365882365, -13),
                zero: Exact::ZERO,
                symbol: "cup",
                name: "cups",
            }
        },
    >;

    /// Fluid ounces (`fl oz`), a unit of `Volume`: `n` of them is `n · 2.95735295625e-05` coherent units.
    pub type FluidOunce = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(295735295625, -16),
                zero: Exact::ZERO,
                symbol: "fl oz",
                name: "fluid ounces",
            }
        },
    >;

    /// Imperial fluid ounces (`fl oz (UK)`), a unit of `Volume`: `n` of them is `n · 2.84130625e-05` coherent units.
    pub type FluidOunceImperial = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(284130625, -13),
                zero: Exact::ZERO,
                symbol: "fl oz (UK)",
                name: "Imperial fluid ounces",
            }
        },
    >;

    /// Imperial gallons (`gal (UK)`), a unit of `Volume`: `n` of them is `n · 0.00454609` coherent units.
    pub type GallonImperial = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(454609, -8),
                zero: Exact::ZERO,
                symbol: "gal (UK)",
                name: "Imperial gallons",
            }
        },
    >;

    /// Gallons (`gal`), a unit of `Volume`: `n` of them is `n · 0.003785411784` coherent units.
    pub type Gallon = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12),
                zero: Exact::ZERO,
                symbol: "gal",
                name: "gallons",
            }
        },
    >;

    /// Imperial gills (`gi (UK)`), a unit of `Volume`: `n` of them is `n · 0.0001420653125` coherent units.
    pub type GillImperial = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1420653125, -13),
                zero: Exact::ZERO,
                symbol: "gi (UK)",
                name: "Imperial gills",
            }
        },
    >;

    /// Gills (`gi`), a unit of `Volume`: `n` of them is `n · 0.00011829411825` coherent units.
    pub type Gill = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(11829411825, -14),
                zero: Exact::ZERO,
                symbol: "gi",
                name: "gills",
            }
        },
    >;

    /// Litres (`L`), a unit of `Volume`: `n` of them is `n · 0.001` coherent units.
    pub type Litre = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "L",
                name: "litres",
            }
        },
    >;

    /// Pecks (`pk`), a unit of `Volume`: `n` of them is `n · 0.00880976754172` coherent units.
    pub type Peck = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(880976754172, -14),
                zero: Exact::ZERO,
                symbol: "pk",
                name: "pecks",
            }
        },
    >;

    /// Dry pints (`dry pt`), a unit of `Volume`: `n` of them is `n · 0.0005506104713575` coherent units.
    pub type PintDry = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(5506104713575, -16),
                zero: Exact::ZERO,
                symbol: "dry pt",
                name: "dry pints",
            }
        },
    >;

    /// Liquid pints (`liq pt`), a unit of `Volume`: `n` of them is `n · 0.000473176473` coherent units.
    pub type PintLiquid = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(473176473, -12),
                zero: Exact::ZERO,
                symbol: "liq pt",
                name: "liquid pints",
            }
        },
    >;

    /// Dry quarts (`dry qt`), a unit of `Volume`: `n` of them is `n · 0.001101220942715` coherent units.
    pub type QuartDry = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1101220942715, -15),
                zero: Exact::ZERO,
                symbol: "dry qt",
                name: "dry quarts",
            }
        },
    >;

    /// Liquid quarts (`liq qt`), a unit of `Volume`: `n` of them is `n · 0.000946352946` coherent units.
    pub type QuartLiquid = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(946352946, -12),
                zero: Exact::ZERO,
                symbol: "liq qt",
                name: "liquid quarts",
            }
        },
    >;

    /// Steres (`st`), a unit of `Volume`: `n` of them is `n · 1.0` coherent units.
    pub type Stere = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "st",
                name: "steres",
            }
        },
    >;

    /// Tablespoons (`tbsp`), a unit of `Volume`: `n` of them is `n · 1.478676478125e-05` coherent units.
    pub type Tablespoon = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1478676478125, -17),
                zero: Exact::ZERO,
                symbol: "tbsp",
                name: "tablespoons",
            }
        },
    >;

    /// Teaspoons (`tsp`), a unit of `Volume`: `n` of them is `n · 4.92892159375e-06` coherent units.
    pub type Teaspoon = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(492892159375, -17),
                zero: Exact::ZERO,
                symbol: "tsp",
                name: "teaspoons",
            }
        },
    >;

    /// Register tons (`RT`), a unit of `Volume`: `n` of them is `n · 100.0 * 0.3048 * 0.3048 * 0.3048` coherent units.
    pub type RegisterTon = SIUnit<
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
    >;

    /// Culei (`culeus`), a unit of `Volume`: `n` of them is `n · 0.51868672` coherent units.
    pub type Culeus = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(51868672, -8),
                zero: Exact::ZERO,
                symbol: "culeus",
                name: "culei",
            }
        },
    >;

    /// Amphora quadrantal (`amphora quadrantal`), a unit of `Volume`: `n` of them is `n · 0.025934336` coherent units.
    pub type AmphoraQuadrantal = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(25934336, -9),
                zero: Exact::ZERO,
                symbol: "amphora quadrantal",
                name: "amphora quadrantal",
            }
        },
    >;

    /// Urnae (`urna`), a unit of `Volume`: `n` of them is `n · 0.012967168` coherent units.
    pub type Urna = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(12967168, -9),
                zero: Exact::ZERO,
                symbol: "urna",
                name: "urnae",
            }
        },
    >;

    /// Modii castrensis (`modius castrensis`), a unit of `Volume`: `n` of them is `n · 0.012967168` coherent units.
    pub type ModiusCastrensis = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(12967168, -9),
                zero: Exact::ZERO,
                symbol: "modius castrensis",
                name: "modii castrensis",
            }
        },
    >;

    /// Modii (`modius`), a unit of `Volume`: `n` of them is `n · 0.003241792 * 16.0 / 6.0` coherent units.
    pub type Modius = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) * Exact::int(16) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "modius",
                name: "modii",
            }
        },
    >;

    /// Semimodii (`semimodius`), a unit of `Volume`: `n` of them is `n · 0.003241792 * 8.0 / 6.0` coherent units.
    pub type Semimodius = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) * Exact::int(8) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "semimodius",
                name: "semimodii",
            }
        },
    >;

    /// Congii (`congius`), a unit of `Volume`: `n` of them is `n · 0.003241792` coherent units.
    pub type Congius = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9),
                zero: Exact::ZERO,
                symbol: "congius",
                name: "congii",
            }
        },
    >;

    /// Sextarii (`sextarius`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 6.0` coherent units.
    pub type Sextarius = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(6),
                zero: Exact::ZERO,
                symbol: "sextarius",
                name: "sextarii",
            }
        },
    >;

    /// Heminae (`hemina`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 12.0` coherent units.
    pub type Hemina = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(12),
                zero: Exact::ZERO,
                symbol: "hemina",
                name: "heminae",
            }
        },
    >;

    /// Quartarii (`quartarius`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 24.0` coherent units.
    pub type Quartarius = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(24),
                zero: Exact::ZERO,
                symbol: "quartarius",
                name: "quartarii",
            }
        },
    >;

    /// Acetabula (`acetabulum`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 48.0` coherent units.
    pub type Acetabulum = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(48),
                zero: Exact::ZERO,
                symbol: "acetabulum",
                name: "acetabula",
            }
        },
    >;

    /// Cyathi (`cyathus`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 72.0` coherent units.
    pub type Cyathus = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(72),
                zero: Exact::ZERO,
                symbol: "cyathus",
                name: "cyathi",
            }
        },
    >;

    /// Ligula (`ligula`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 288.0` coherent units.
    pub type Ligula = SIUnit<
        VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3241792, -9) / Exact::int(288),
                zero: Exact::ZERO,
                symbol: "ligula",
                name: "ligula",
            }
        },
    >;

    /// Metres per second (`m/s`), a unit of `Velocity`: `n` of them is `n · 1.0` coherent units.
    pub type MetrePerSecond = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m/s",
                name: "metres per second",
            }
        },
    >;

    /// Feet per hour (`ft/h`), a unit of `Velocity`: `n` of them is `n · 0.3048 / 3600.0` coherent units.
    pub type FootPerHour = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "ft/h",
                name: "feet per hour",
            }
        },
    >;

    /// Feet per minute (`ft/min`), a unit of `Velocity`: `n` of them is `n · 0.00508` coherent units.
    pub type FootPerMinute = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(508, -5),
                zero: Exact::ZERO,
                symbol: "ft/min",
                name: "feet per minute",
            }
        },
    >;

    /// Feet per second (`ft/s`), a unit of `Velocity`: `n` of them is `n · 0.3048` coherent units.
    pub type FootPerSecond = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft/s",
                name: "feet per second",
            }
        },
    >;

    /// Inches per second (`in/s`), a unit of `Velocity`: `n` of them is `n · 0.0254` coherent units.
    pub type InchPerSecond = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in/s",
                name: "inches per second",
            }
        },
    >;

    /// Inches per minute (`in/min`), a unit of `Velocity`: `n` of them is `n · 0.0254 / 60.0` coherent units.
    pub type InchPerMinute = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "in/min",
                name: "inches per minute",
            }
        },
    >;

    /// Knots (`kn`), a unit of `Velocity`: `n` of them is `n · 1852.0 / 3600.0` coherent units.
    pub type Knot = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1852) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "kn",
                name: "knots",
            }
        },
    >;

    /// Miles per hour (`mi/h`), a unit of `Velocity`: `n` of them is `n · 0.44704` coherent units.
    pub type MilePerHour = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(44704, -5),
                zero: Exact::ZERO,
                symbol: "mi/h",
                name: "miles per hour",
            }
        },
    >;

    /// Miles per minute (`mi/min`), a unit of `Velocity`: `n` of them is `n · 26.8224` coherent units.
    pub type MilePerMinute = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(268224, -4),
                zero: Exact::ZERO,
                symbol: "mi/min",
                name: "miles per minute",
            }
        },
    >;

    /// Miles per second (`mi/s`), a unit of `Velocity`: `n` of them is `n · 1609.344` coherent units.
    pub type MilePerSecond = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "mi/s",
                name: "miles per second",
            }
        },
    >;

    /// Atomic units of velocity (`a₀ · Eₕ/ħ`), a unit of `Velocity`: `n` of them is `n · 2187691.26364` coherent units.
    pub type AtomicUnitOfVelocity = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(218769126364, -5),
                zero: Exact::ZERO,
                symbol: "a₀ · Eₕ/ħ",
                name: "atomic units of velocity",
            }
        },
    >;

    /// Natural units of velocity (`c`), a unit of `Velocity`: `n` of them is `n · 299792458.0` coherent units.
    pub type NaturalUnitOfVelocity = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(299792458),
                zero: Exact::ZERO,
                symbol: "c",
                name: "natural units of velocity",
            }
        },
    >;

    /// Speeds of light in vacuum (`c`), a unit of `Velocity`: `n` of them is `n · 299792458.0` coherent units.
    pub type SpeedOfLight = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(299792458),
                zero: Exact::ZERO,
                symbol: "c",
                name: "speeds of light in vacuum",
            }
        },
    >;

    /// Kilometres per hour (`km/h`), a unit of `Velocity`: `n` of them is `n · 1000.0 / 3600.0` coherent units.
    pub type KilometrePerHour = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1000) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "km/h",
                name: "kilometres per hour",
            }
        },
    >;

    /// Millimetres per minute (`mm/min`), a unit of `Velocity`: `n` of them is `n · 0.001 / 60.0` coherent units.
    pub type MillimetrePerMinute = SIUnit<
        VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "mm/min",
                name: "millimetres per minute",
            }
        },
    >;

    /// Metres per second squared (`m/s²`), a unit of `Acceleration`: `n` of them is `n · 1.0` coherent units.
    pub type MetrePerSecondSquared = SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m/s²",
                name: "metres per second squared",
            }
        },
    >;

    /// Feet per second squared (`ft/s²`), a unit of `Acceleration`: `n` of them is `n · 0.3048` coherent units.
    pub type FootPerSecondSquared = SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft/s²",
                name: "feet per second squared",
            }
        },
    >;

    /// Galileos (`Gal`), a unit of `Acceleration`: `n` of them is `n · 0.01` coherent units.
    pub type Galileo = SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "Gal",
                name: "galileos",
            }
        },
    >;

    /// Inches per second squared (`in/s²`), a unit of `Acceleration`: `n` of them is `n · 0.0254` coherent units.
    pub type InchPerSecondSquared = SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in/s²",
                name: "inches per second squared",
            }
        },
    >;

    /// Standard accelerations of gravity (`g₀`), a unit of `Acceleration`: `n` of them is `n · 9.80665` coherent units.
    pub type StandardGravity = SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "g₀",
                name: "standard accelerations of gravity",
            }
        },
    >;

    /// Millimetres per minute squared (`mm/min²`), a unit of `Acceleration`: `n` of them is `n · 0.001 / (60.0 * 60.0)` coherent units.
    pub type MillimetrePerMinuteSquared = SIUnit<
        ACCELERATION,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3) / (Exact::int(60) * Exact::int(60)),
                zero: Exact::ZERO,
                symbol: "mm/min²",
                name: "millimetres per minute squared",
            }
        },
    >;

    /// Metres per second cubed (`m/s³`), a unit of `Jerk`: `n` of them is `n · 1.0` coherent units.
    pub type MetrePerSecondCubed = SIUnit<
        JERK,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m/s³",
                name: "metres per second cubed",
            }
        },
    >;

    /// Feet per second cubed (`ft/s³`), a unit of `Jerk`: `n` of them is `n · 0.3048` coherent units.
    pub type FootPerSecondCubed = SIUnit<
        JERK,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft/s³",
                name: "feet per second cubed",
            }
        },
    >;

    /// Inches per second cubed (`in/s³`), a unit of `Jerk`: `n` of them is `n · 0.0254` coherent units.
    pub type InchPerSecondCubed = SIUnit<
        JERK,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in/s³",
                name: "inches per second cubed",
            }
        },
    >;

    /// Kilometres per minute cubed (`km/min³`), a unit of `Jerk`: `n` of them is `n · 1000.0 / (60.0 * 60.0 * 60.0)` coherent units.
    pub type KilometrePerMinuteCubed = SIUnit<
        JERK,
        {
            SIUnitConsts {
                size: Exact::int(1000) / (Exact::int(60) * Exact::int(60) * Exact::int(60)),
                zero: Exact::ZERO,
                symbol: "km/min³",
                name: "kilometres per minute cubed",
            }
        },
    >;

    /// Metre seconds (`m · s`), a unit of `Absement`: `n` of them is `n · 1.0` coherent units.
    pub type MetreSecond = SIUnit<
        ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m · s",
                name: "metre seconds",
            }
        },
    >;

    /// Foot seconds (`ft · s`), a unit of `Absement`: `n` of them is `n · 0.3048` coherent units.
    pub type FootSecond = SIUnit<
        ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft · s",
                name: "foot seconds",
            }
        },
    >;

    /// Inch seconds (`in · s`), a unit of `Absement`: `n` of them is `n · 0.0254` coherent units.
    pub type InchSecond = SIUnit<
        ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "in · s",
                name: "inch seconds",
            }
        },
    >;

    /// Kilometre hours (`km · h`), a unit of `Absement`: `n` of them is `n · 3600000.0` coherent units.
    pub type KilometreHour = SIUnit<
        ABSEMENT,
        {
            SIUnitConsts {
                size: Exact::int(3600000),
                zero: Exact::ZERO,
                symbol: "km · h",
                name: "kilometre hours",
            }
        },
    >;

    /// Seconds per metre (`s/m`), a unit of `InverseVelocity`: `n` of them is `n · 1.0` coherent units.
    pub type SecondPerMetre = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "s/m",
                name: "seconds per metre",
            }
        },
    >;

    /// Hours per foot (`h/ft`), a unit of `InverseVelocity`: `n` of them is `n · 3600.0 / 0.3048` coherent units.
    pub type HourPerFoot = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(3600) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "h/ft",
                name: "hours per foot",
            }
        },
    >;

    /// Minutes per foot (`min/ft`), a unit of `InverseVelocity`: `n` of them is `n · 60.0 / 0.3048` coherent units.
    pub type MinutePerFoot = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(60) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "min/ft",
                name: "minutes per foot",
            }
        },
    >;

    /// Seconds per foot (`s/ft`), a unit of `InverseVelocity`: `n` of them is `n · 1.0 / 0.3048` coherent units.
    pub type SecondPerFoot = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "s/ft",
                name: "seconds per foot",
            }
        },
    >;

    /// Seconds per inch (`s/in`), a unit of `InverseVelocity`: `n` of them is `n · 1.0 / 0.0254` coherent units.
    pub type SecondPerInch = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "s/in",
                name: "seconds per inch",
            }
        },
    >;

    /// Hours per mile (`h/mi`), a unit of `InverseVelocity`: `n` of them is `n · 3600.0 / 1609.344` coherent units.
    pub type HourPerMile = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(3600) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "h/mi",
                name: "hours per mile",
            }
        },
    >;

    /// Minutes per mile (`min/mi`), a unit of `InverseVelocity`: `n` of them is `n · 60.0 / 1609.344` coherent units.
    pub type MinutePerMile = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(60) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "min/mi",
                name: "minutes per mile",
            }
        },
    >;

    /// Seconds per mile (`s/mi`), a unit of `InverseVelocity`: `n` of them is `n · 1.0 / 1609.344` coherent units.
    pub type SecondPerMile = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(1609344, -3),
                zero: Exact::ZERO,
                symbol: "s/mi",
                name: "seconds per mile",
            }
        },
    >;

    /// Minutes per kilometre (`min/km`), a unit of `InverseVelocity`: `n` of them is `n · 0.06` coherent units.
    pub type MinutePerKilometre = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(6, -2),
                zero: Exact::ZERO,
                symbol: "min/km",
                name: "minutes per kilometre",
            }
        },
    >;

    /// Hours per kilometre (`h/km`), a unit of `InverseVelocity`: `n` of them is `n · 3.6` coherent units.
    pub type HourPerKilometre = SIUnit<
        INVERSE_VELOCITY,
        {
            SIUnitConsts {
                size: Exact::dec(36, -1),
                zero: Exact::ZERO,
                symbol: "h/km",
                name: "hours per kilometre",
            }
        },
    >;

    /// Newtons (`N`), a unit of `Force`: `n` of them is `n · 1.0` coherent units.
    pub type Newton = SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N",
                name: "newtons",
            }
        },
    >;

    /// Dynes (`dyn`), a unit of `Force`: `n` of them is `n · 1e-05` coherent units.
    pub type Dyne = SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -5),
                zero: Exact::ZERO,
                symbol: "dyn",
                name: "dynes",
            }
        },
    >;

    /// Kilograms-force (`kgf`), a unit of `Force`: `n` of them is `n · 9.80665` coherent units.
    pub type KilogramForce = SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "kgf",
                name: "kilograms-force",
            }
        },
    >;

    /// Kips (`kip`), a unit of `Force`: `n` of them is `n · 1000.0 * 0.45359237 * 9.80665` coherent units.
    pub type Kip = SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "kip",
                name: "kips",
            }
        },
    >;

    /// Ounces-force (`ozf`), a unit of `Force`: `n` of them is `n · 0.45359237 * 9.80665 / 16.0` coherent units.
    pub type OunceForce = SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16),
                zero: Exact::ZERO,
                symbol: "ozf",
                name: "ounces-force",
            }
        },
    >;

    /// Poundals (`pdl`), a unit of `Force`: `n` of them is `n · 0.45359237 * 0.3048` coherent units.
    pub type Poundal = SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "pdl",
                name: "poundals",
            }
        },
    >;

    /// Pounds-force (`lbf`), a unit of `Force`: `n` of them is `n · 0.45359237 * 9.80665` coherent units.
    pub type PoundForce = SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "lbf",
                name: "pounds-force",
            }
        },
    >;

    /// Tons-force (`2000 lbf`), a unit of `Force`: `n` of them is `n · 2000.0 * 0.45359237 * 9.80665` coherent units.
    pub type TonForce = SIUnit<
        FORCE,
        {
            SIUnitConsts {
                size: Exact::int(2000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "2000 lbf",
                name: "tons-force",
            }
        },
    >;

    /// Kilogram metres per second (`kg · m/s`), a unit of `Momentum`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramMetrePerSecond = SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m/s",
                name: "kilogram metres per second",
            }
        },
    >;

    /// Gram metres per second (`g · m/s`), a unit of `Momentum`: `n` of them is `n · 0.001` coherent units.
    pub type GramMetrePerSecond = SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g · m/s",
                name: "gram metres per second",
            }
        },
    >;

    /// Ton metres per second (`t · m/s`), a unit of `Momentum`: `n` of them is `n · 1000.0` coherent units.
    pub type TonMetrePerSecond = SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "t · m/s",
                name: "ton metres per second",
            }
        },
    >;

    /// Kilogram metres per minute (`kg · m/min`), a unit of `Momentum`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub type KilogramMetrePerMinute = SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "kg · m/min",
                name: "kilogram metres per minute",
            }
        },
    >;

    /// Kilogram metres per hour (`kg · m/h`), a unit of `Momentum`: `n` of them is `n · 1.0 / 3600.0` coherent units.
    pub type KilogramMetrePerHour = SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "kg · m/h",
                name: "kilogram metres per hour",
            }
        },
    >;

    /// Kilogram metres per day (`kg · m/d`), a unit of `Momentum`: `n` of them is `n · 1.0 / 86400.0` coherent units.
    pub type KilogramMetrePerDay = SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "kg · m/d",
                name: "kilogram metres per day",
            }
        },
    >;

    /// Slug feet per second (`slug · ft/s`), a unit of `Momentum`: `n` of them is `n · (0.45359237 * 9.80665 / 0.3048) * 0.3048` coherent units.
    pub type SlugFootPerSecond = SIUnit<
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
    >;

    /// Slug inches per second (`slug · in/s`), a unit of `Momentum`: `n` of them is `n · (0.45359237 * 9.80665 / 0.3048) * 0.0254` coherent units.
    pub type SlugInchPerSecond = SIUnit<
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
    >;

    /// Pound feet per second (`lb · ft/s`), a unit of `Momentum`: `n` of them is `n · 0.138254954376` coherent units.
    pub type PoundFootPerSecond = SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::dec(138254954376, -12),
                zero: Exact::ZERO,
                symbol: "lb · ft/s",
                name: "pound feet per second",
            }
        },
    >;

    /// Pound inches per second (`lb · in/s`), a unit of `Momentum`: `n` of them is `n · 0.011521246198` coherent units.
    pub type PoundInchPerSecond = SIUnit<
        MOMENTUM,
        {
            SIUnitConsts {
                size: Exact::dec(11521246198, -12),
                zero: Exact::ZERO,
                symbol: "lb · in/s",
                name: "pound inches per second",
            }
        },
    >;

    /// Joules (`J`), a unit of `Energy`: `n` of them is `n · 1.0` coherent units.
    pub type Joule = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J",
                name: "joules",
            }
        },
    >;

    /// Watt hours (`W · h`), a unit of `Energy`: `n` of them is `n · 3600.0` coherent units.
    pub type WattHour = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "W · h",
                name: "watt hours",
            }
        },
    >;

    /// Electronvolts (`eV`), a unit of `Energy`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub type Electronvolt = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "eV",
                name: "electronvolts",
            }
        },
    >;

    /// Hartrees (`Eₕ`), a unit of `Energy`: `n` of them is `n · 4.3597447222071e-18` coherent units.
    pub type Hartree = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(43597447222071, -31),
                zero: Exact::ZERO,
                symbol: "Eₕ",
                name: "hartrees",
            }
        },
    >;

    /// British thermal units (IT) (`Btu (IT)`), a unit of `Energy`: `n` of them is `n · 1055.05585262` coherent units.
    pub type BtuIt = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8),
                zero: Exact::ZERO,
                symbol: "Btu (IT)",
                name: "British thermal units (IT)",
            }
        },
    >;

    /// British thermal units (`Btu`), a unit of `Energy`: `n` of them is `n · 4184.0 * 0.45359237 / 1.8` coherent units.
    pub type Btu = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1),
                zero: Exact::ZERO,
                symbol: "Btu",
                name: "British thermal units",
            }
        },
    >;

    /// Calories (IT) (`cal (IT)`), a unit of `Energy`: `n` of them is `n · 4.1868` coherent units.
    pub type CalorieIt = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -4),
                zero: Exact::ZERO,
                symbol: "cal (IT)",
                name: "calories (IT)",
            }
        },
    >;

    /// Calories (`cal`), a unit of `Energy`: `n` of them is `n · 4.184` coherent units.
    pub type Calorie = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal",
                name: "calories",
            }
        },
    >;

    /// Calories (IT) (`Cal (IT)`), a unit of `Energy`: `n` of them is `n · 4186.8` coherent units.
    pub type CalorieItNutrition = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Cal (IT)",
                name: "Calories (IT)",
            }
        },
    >;

    /// Calories (`Cal`), a unit of `Energy`: `n` of them is `n · 4184.0` coherent units.
    pub type CalorieNutrition = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Cal",
                name: "Calories",
            }
        },
    >;

    /// Ergs (`erg`), a unit of `Energy`: `n` of them is `n · 1e-07` coherent units.
    pub type Erg = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg",
                name: "ergs",
            }
        },
    >;

    /// Foot poundals (`ft · pdl`), a unit of `Energy`: `n` of them is `n · 0.3048 * 0.45359237 * 0.3048` coherent units.
    pub type FootPoundal = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft · pdl",
                name: "foot poundals",
            }
        },
    >;

    /// Foot pounds-force (`ft · lbf`), a unit of `Energy`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub type FootPound = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "ft · lbf",
                name: "foot pounds-force",
            }
        },
    >;

    /// Quads (`10¹⁵ Btu (IT)`), a unit of `Energy`: `n` of them is `n · 1.05505585262e18` coherent units.
    pub type Quad = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1055055852620000000),
                zero: Exact::ZERO,
                symbol: "10¹⁵ Btu (IT)",
                name: "quads",
            }
        },
    >;

    /// Therms (EC) (`thm (EC)`), a unit of `Energy`: `n` of them is `n · 105506000.0` coherent units.
    pub type ThermEc = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(105506000),
                zero: Exact::ZERO,
                symbol: "thm (EC)",
                name: "therms (EC)",
            }
        },
    >;

    /// Therms (`thm`), a unit of `Energy`: `n` of them is `n · 105480400.0` coherent units.
    pub type ThermUs = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(105480400),
                zero: Exact::ZERO,
                symbol: "thm",
                name: "therms",
            }
        },
    >;

    /// Tons of TNT (`t of TNT`), a unit of `Energy`: `n` of them is `n · 4184000000.0` coherent units.
    pub type TonTnt = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184000000),
                zero: Exact::ZERO,
                symbol: "t of TNT",
                name: "tons of TNT",
            }
        },
    >;

    /// Watt seconds (`W · s`), a unit of `Energy`: `n` of them is `n · 1.0` coherent units.
    pub type WattSecond = SIUnit<
        ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W · s",
                name: "watt seconds",
            }
        },
    >;

    /// Joules per kilogram (`J/kg`), a unit of `AvailableEnergy`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerKilogram = SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/kg",
                name: "joules per kilogram",
            }
        },
    >;

    /// Joules per gram (`J/g`), a unit of `AvailableEnergy`: `n` of them is `n · 1000.0` coherent units.
    pub type JoulePerGram = SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "J/g",
                name: "joules per gram",
            }
        },
    >;

    /// British thermal units (IT) per pound (`Btu (IT)/lb`), a unit of `AvailableEnergy`: `n` of them is `n · 2326.0` coherent units.
    pub type BtuItPerPound = SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(2326),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/lb",
                name: "British thermal units (IT) per pound",
            }
        },
    >;

    /// British thermal units per pound (`Btu/lb`), a unit of `AvailableEnergy`: `n` of them is `n · 4184.0 / 1.8` coherent units.
    pub type BtuPerPound = SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184) / Exact::dec(18, -1),
                zero: Exact::ZERO,
                symbol: "Btu/lb",
                name: "British thermal units per pound",
            }
        },
    >;

    /// Calories (IT) per gram (`cal (IT)/g`), a unit of `AvailableEnergy`: `n` of them is `n · 4186.8` coherent units.
    pub type CalorieItPerGram = SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "cal (IT)/g",
                name: "calories (IT) per gram",
            }
        },
    >;

    /// Calories per gram (`cal/g`), a unit of `AvailableEnergy`: `n` of them is `n · 4184.0` coherent units.
    pub type CaloriePerGram = SIUnit<
        AVAILABLE_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "cal/g",
                name: "calories per gram",
            }
        },
    >;

    /// Watts (`W`), a unit of `Power`: `n` of them is `n · 1.0` coherent units.
    pub type Watt = SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W",
                name: "watts",
            }
        },
    >;

    /// Ergs per second (`erg/s`), a unit of `Power`: `n` of them is `n · 1e-07` coherent units.
    pub type ErgPerSecond = SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg/s",
                name: "ergs per second",
            }
        },
    >;

    /// Foot pounds-force per hour (`ft · lbf/h`), a unit of `Power`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / 3600.0` coherent units.
    pub type FootPoundPerHour = SIUnit<
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
    >;

    /// Foot pounds-force per minute (`ft · lbf/min`), a unit of `Power`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / 60.0` coherent units.
    pub type FootPoundPerMinute = SIUnit<
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
    >;

    /// Foot pounds-force per second (`ft · lbf/s`), a unit of `Power`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub type FootPoundPerSecond = SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "ft · lbf/s",
                name: "foot pounds-force per second",
            }
        },
    >;

    /// Horsepower (`hp`), a unit of `Power`: `n` of them is `n · 550.0 * 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub type Horsepower = SIUnit<
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
    >;

    /// Horsepower (boiler) (`hp (S)`), a unit of `Power`: `n` of them is `n · 9809.5` coherent units.
    pub type HorsepowerBoiler = SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(98095, -1),
                zero: Exact::ZERO,
                symbol: "hp (S)",
                name: "horsepower (boiler)",
            }
        },
    >;

    /// Horsepower (electric) (`hp (E)`), a unit of `Power`: `n` of them is `n · 746.0` coherent units.
    pub type HorsepowerElectric = SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::int(746),
                zero: Exact::ZERO,
                symbol: "hp (E)",
                name: "horsepower (electric)",
            }
        },
    >;

    /// Metric horsepower (`hp (M)`), a unit of `Power`: `n` of them is `n · 75.0 * 9.80665` coherent units.
    pub type HorsepowerMetric = SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::int(75) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "hp (M)",
                name: "metric horsepower",
            }
        },
    >;

    /// Horsepower (Imperial) (`hp (I)`), a unit of `Power`: `n` of them is `n · 745.7` coherent units.
    pub type HorsepowerImperial = SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(7457, -1),
                zero: Exact::ZERO,
                symbol: "hp (I)",
                name: "horsepower (Imperial)",
            }
        },
    >;

    /// Hydraulic horsepower (`hp (hydraulic)`), a unit of `Power`: `n` of them is `n · 746.043` coherent units.
    pub type HydraulicHorsepower = SIUnit<
        POWER,
        {
            SIUnitConsts {
                size: Exact::dec(746043, -3),
                zero: Exact::ZERO,
                symbol: "hp (hydraulic)",
                name: "hydraulic horsepower",
            }
        },
    >;

    /// Bels (`B`), a unit of `Gain`: `n` of them is `n · 1.0` coherent units.
    pub type Bel = SIUnit<
        GAIN,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "B",
                name: "bels",
            }
        },
    >;

    /// Decibels (`dB`), a unit of `Gain`: `n` of them is `n · 0.1` coherent units.
    pub type Decibel = SIUnit<
        GAIN,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "dB",
                name: "decibels",
            }
        },
    >;

    /// Nepers (`Np`), a unit of `Gain`: `n` of them is `n · 0.8685889638065036` coherent units.
    pub type Neper = SIUnit<
        GAIN,
        {
            SIUnitConsts {
                size: Exact::dec(8685889638065036, -16),
                zero: Exact::ZERO,
                symbol: "Np",
                name: "nepers",
            }
        },
    >;

    /// Decibels-watt (`dBW`), a unit of `PowerLevel`: `n` of them is `n · 0.1` coherent units.
    pub type DecibelWatt = SIUnit<
        POWER_LEVEL,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "dBW",
                name: "decibels-watt",
            }
        },
    >;

    /// Decibels-milliwatt (`dBm`), a unit of `PowerLevel`: `n` of them is `n · 0.1 + -3.0` coherent units.
    pub type DecibelMilliwatt = SIUnit<
        POWER_LEVEL,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: -Exact::int(3),
                symbol: "dBm",
                name: "decibels-milliwatt",
            }
        },
    >;

    /// Watts per second (`W/s`), a unit of `PowerRate`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerSecond = SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/s",
                name: "watts per second",
            }
        },
    >;

    /// Ergs per second squared (`erg/s²`), a unit of `PowerRate`: `n` of them is `n · 1e-07` coherent units.
    pub type ErgPerSecondSquared = SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg/s²",
                name: "ergs per second squared",
            }
        },
    >;

    /// Foot pounds-force per second squared (`ft · lbf/s²`), a unit of `PowerRate`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub type FootPoundPerSecondSquared = SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "ft · lbf/s²",
                name: "foot pounds-force per second squared",
            }
        },
    >;

    /// Horsepower per second (`hp/s`), a unit of `PowerRate`: `n` of them is `n · 550.0 * 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub type HorsepowerPerSecond = SIUnit<
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
    >;

    /// Horsepower per second (boiler) (`hp/s (S)`), a unit of `PowerRate`: `n` of them is `n · 9809.5` coherent units.
    pub type HorsepowerPerSecondBoiler = SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(98095, -1),
                zero: Exact::ZERO,
                symbol: "hp/s (S)",
                name: "horsepower per second (boiler)",
            }
        },
    >;

    /// Horsepower per second (electric) (`hp/s (E)`), a unit of `PowerRate`: `n` of them is `n · 746.0` coherent units.
    pub type HorsepowerPerSecondElectric = SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(746),
                zero: Exact::ZERO,
                symbol: "hp/s (E)",
                name: "horsepower per second (electric)",
            }
        },
    >;

    /// Metric horsepower per second (`hp/s (M)`), a unit of `PowerRate`: `n` of them is `n · 75.0 * 9.80665` coherent units.
    pub type HorsepowerPerSecondMetric = SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::int(75) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "hp/s (M)",
                name: "metric horsepower per second",
            }
        },
    >;

    /// Horsepower per second (Imperial) (`hp/s (I)`), a unit of `PowerRate`: `n` of them is `n · 745.7` coherent units.
    pub type HorsepowerPerSecondImperial = SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(7457, -1),
                zero: Exact::ZERO,
                symbol: "hp/s (I)",
                name: "horsepower per second (Imperial)",
            }
        },
    >;

    /// Hydraulic horsepower per second (`hp/s (hydraulic)`), a unit of `PowerRate`: `n` of them is `n · 746.043` coherent units.
    pub type HydraulicHorsepowerPerSecond = SIUnit<
        POWER_RATE,
        {
            SIUnitConsts {
                size: Exact::dec(746043, -3),
                zero: Exact::ZERO,
                symbol: "hp/s (hydraulic)",
                name: "hydraulic horsepower per second",
            }
        },
    >;

    /// Joule seconds (`J · s`), a unit of `Action`: `n` of them is `n · 1.0` coherent units.
    pub type JouleSecond = SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J · s",
                name: "joule seconds",
            }
        },
    >;

    /// Atomic units of action (`ħ`), a unit of `Action`: `n` of them is `n · 6.62607015e-34 / (2.0 * core::f64::consts::PI)` coherent units.
    pub type AtomicUnitOfAction = SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI),
                zero: Exact::ZERO,
                symbol: "ħ",
                name: "atomic units of action",
            }
        },
    >;

    /// Planck constants (`h`), a unit of `Action`: `n` of them is `n · 6.62607015e-34` coherent units.
    pub type PlanckConstant = SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::dec(662607015, -42),
                zero: Exact::ZERO,
                symbol: "h",
                name: "Planck constants",
            }
        },
    >;

    /// Erg seconds (`erg · s`), a unit of `Action`: `n` of them is `n · 1e-07` coherent units.
    pub type ErgSecond = SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg · s",
                name: "erg seconds",
            }
        },
    >;

    /// Electronvolt seconds (`eV · s`), a unit of `Action`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub type ElectronvoltSecond = SIUnit<
        ACTION,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "eV · s",
                name: "electronvolt seconds",
            }
        },
    >;

    /// Newton metres (`N · m`), a unit of `Torque`: `n` of them is `n · 1.0` coherent units.
    pub type NewtonMetre = SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "N · m",
                name: "newton metres",
            }
        },
    >;

    /// Dyne metres (`dyn · m`), a unit of `Torque`: `n` of them is `n · 1e-05` coherent units.
    pub type DyneMetre = SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -5),
                zero: Exact::ZERO,
                symbol: "dyn · m",
                name: "dyne metres",
            }
        },
    >;

    /// Kilogram-force metres (`kgf · m`), a unit of `Torque`: `n` of them is `n · 9.80665` coherent units.
    pub type KilogramForceMetre = SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "kgf · m",
                name: "kilogram-force metres",
            }
        },
    >;

    /// Ounces-force inches (`ozf · in`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665 / 16.0) * 0.0254` coherent units.
    pub type OunceForceInch = SIUnit<
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
    >;

    /// Pounds-force feet (`lbf · ft`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665) * 0.3048` coherent units.
    pub type PoundForceFoot = SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "lbf · ft",
                name: "pounds-force feet",
            }
        },
    >;

    /// Pounds-force inches (`lbf · in`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665) * 0.0254` coherent units.
    pub type PoundForceInch = SIUnit<
        TORQUE,
        {
            SIUnitConsts {
                size: (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(254, -4),
                zero: Exact::ZERO,
                symbol: "lbf · in",
                name: "pounds-force inches",
            }
        },
    >;

    /// Kilogram square metres (`kg · m²`), a unit of `MomentOfInertia`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramSquareMetre = SIUnit<
        MOMENT_OF_INERTIA,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²",
                name: "kilogram square metres",
            }
        },
    >;

    /// Dalton square ångströms (`Da · Å²`), a unit of `MomentOfInertia`: `n` of them is `n · 1.6605390666e-47` coherent units.
    pub type DaltonSquareAngstrom = SIUnit<
        MOMENT_OF_INERTIA,
        {
            SIUnitConsts {
                size: Exact::dec(16605390666, -57),
                zero: Exact::ZERO,
                symbol: "Da · Å²",
                name: "dalton square ångströms",
            }
        },
    >;

    /// Pascals (`Pa`), a unit of `Pressure`: `n` of them is `n · 1.0` coherent units.
    pub type Pascal = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Pa",
                name: "pascals",
            }
        },
    >;

    /// Standard atmospheres (`atm`), a unit of `Pressure`: `n` of them is `n · 101325.0` coherent units.
    pub type StandardAtmosphere = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(101325),
                zero: Exact::ZERO,
                symbol: "atm",
                name: "standard atmospheres",
            }
        },
    >;

    /// Atmospheres (technical) (`at`), a unit of `Pressure`: `n` of them is `n · 98066.5` coherent units.
    pub type AtmosphereTechnical = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -1),
                zero: Exact::ZERO,
                symbol: "at",
                name: "atmospheres (technical)",
            }
        },
    >;

    /// Bar (`bar`), a unit of `Pressure`: `n` of them is `n · 100000.0` coherent units.
    pub type Bar = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(100000),
                zero: Exact::ZERO,
                symbol: "bar",
                name: "bar",
            }
        },
    >;

    /// Centimetres of mercury (`cm Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 10.0` coherent units.
    pub type CentimetreOfMercury = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(133322387415, -9) * Exact::int(10),
                zero: Exact::ZERO,
                symbol: "cm Hg",
                name: "centimetres of mercury",
            }
        },
    >;

    /// Centimetres of water (`cm H₂O`), a unit of `Pressure`: `n` of them is `n · 98.0665` coherent units.
    pub type CentimetreOfWater = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -4),
                zero: Exact::ZERO,
                symbol: "cm H₂O",
                name: "centimetres of water",
            }
        },
    >;

    /// Feet of mercury (`ft Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 304.8` coherent units.
    pub type FootOfMercury = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(133322387415, -9) * Exact::dec(3048, -1),
                zero: Exact::ZERO,
                symbol: "ft Hg",
                name: "feet of mercury",
            }
        },
    >;

    /// Feet of water (`ft H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665 * 304.8` coherent units.
    pub type FootOfWater = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5) * Exact::dec(3048, -1),
                zero: Exact::ZERO,
                symbol: "ft H₂O",
                name: "feet of water",
            }
        },
    >;

    /// Inches of mercury (`in Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 25.4` coherent units.
    pub type InchOfMercury = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(133322387415, -9) * Exact::dec(254, -1),
                zero: Exact::ZERO,
                symbol: "in Hg",
                name: "inches of mercury",
            }
        },
    >;

    /// Inches of water (`in H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665 * 25.4` coherent units.
    pub type InchOfWater = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5) * Exact::dec(254, -1),
                zero: Exact::ZERO,
                symbol: "in H₂O",
                name: "inches of water",
            }
        },
    >;

    /// Kips per square inch (`kip/in²`), a unit of `Pressure`: `n` of them is `n · (1000.0 * 0.45359237 * 9.80665) / (0.0254 * 0.0254)` coherent units.
    pub type KipPerSquareInch = SIUnit<
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
    >;

    /// Millimetres of mercury (`mm Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415` coherent units.
    pub type MillimetreOfMercury = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(133322387415, -9),
                zero: Exact::ZERO,
                symbol: "mm Hg",
                name: "millimetres of mercury",
            }
        },
    >;

    /// Millimetres of water (`mm H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665` coherent units.
    pub type MillimetreOfWater = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "mm H₂O",
                name: "millimetres of water",
            }
        },
    >;

    /// Poundals per square foot (`pdl/ft²`), a unit of `Pressure`: `n` of them is `n · (0.45359237 * 0.3048) / (0.3048 * 0.3048)` coherent units.
    pub type PoundalPerSquareFoot = SIUnit<
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
    >;

    /// Pounds-force per square foot (`lbf/ft²`), a unit of `Pressure`: `n` of them is `n · (0.45359237 * 9.80665) / (0.3048 * 0.3048)` coherent units.
    pub type PoundForcePerSquareFoot = SIUnit<
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
    >;

    /// Pounds-force per square inch (`lbf/in²`), a unit of `Pressure`: `n` of them is `n · (0.45359237 * 9.80665) / (0.0254 * 0.0254)` coherent units.
    pub type PoundForcePerSquareInch = SIUnit<
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
    >;

    /// Pounds-force per square inch (`psi`), a unit of `Pressure`: `n` of them is `n · 0.45359237 * 9.80665 / (0.0254 * 0.0254)` coherent units.
    pub type Psi = SIUnit<
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
    >;

    /// Torr (`Torr`), a unit of `Pressure`: `n` of them is `n · 101325.0 / 760.0` coherent units.
    pub type Torr = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(101325) / Exact::int(760),
                zero: Exact::ZERO,
                symbol: "Torr",
                name: "torr",
            }
        },
    >;

    /// Dynes per square centimetre (`dyn/cm²`), a unit of `Pressure`: `n` of them is `n · 0.1` coherent units.
    pub type DynePerSquareCentimetre = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "dyn/cm²",
                name: "dynes per square centimetre",
            }
        },
    >;

    /// Newtons per square millimetre (`N/mm²`), a unit of `Pressure`: `n` of them is `n · 1000000.0` coherent units.
    pub type NewtonPerSquareMillimetre = SIUnit<
        PRESSURE,
        {
            SIUnitConsts {
                size: Exact::int(1000000),
                zero: Exact::ZERO,
                symbol: "N/mm²",
                name: "newtons per square millimetre",
            }
        },
    >;

    /// Hertz (`Hz`), a unit of `Frequency`: `n` of them is `n · 1.0` coherent units.
    pub type Hertz = SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Hz",
                name: "hertz",
            }
        },
    >;

    /// Cycles per day (`1/d`), a unit of `Frequency`: `n` of them is `n · 1.0 / 86400.0` coherent units.
    pub type CyclePerDay = SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(86400),
                zero: Exact::ZERO,
                symbol: "1/d",
                name: "cycles per day",
            }
        },
    >;

    /// Cycles per hour (`1/h`), a unit of `Frequency`: `n` of them is `n · 1.0 / 3600.0` coherent units.
    pub type CyclePerHour = SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "1/h",
                name: "cycles per hour",
            }
        },
    >;

    /// Cycles per minute (`1/min`), a unit of `Frequency`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub type CyclePerMinute = SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "1/min",
                name: "cycles per minute",
            }
        },
    >;

    /// Cycles per shake (`100 MHz`), a unit of `Frequency`: `n` of them is `n · 100000000.0` coherent units.
    pub type CyclePerShake = SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(100000000),
                zero: Exact::ZERO,
                symbol: "100 MHz",
                name: "cycles per shake",
            }
        },
    >;

    /// Cycles per year (`1/a`), a unit of `Frequency`: `n` of them is `n · 1.0 / 31536000.0` coherent units.
    pub type CyclePerYear = SIUnit<
        FREQUENCY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(31536000),
                zero: Exact::ZERO,
                symbol: "1/a",
                name: "cycles per year",
            }
        },
    >;

    /// Hertz per second (`Hz/s`), a unit of `FrequencyDrift`: `n` of them is `n · 1.0` coherent units.
    pub type HertzPerSecond = SIUnit<
        FREQUENCY_DRIFT,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Hz/s",
                name: "hertz per second",
            }
        },
    >;

    /// Reciprocal metres (`m⁻¹`), a unit of `ReciprocalLength`: `n` of them is `n · 1.0` coherent units.
    pub type ReciprocalMetre = SIUnit<
        RECIPROCAL_LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m⁻¹",
                name: "reciprocal metres",
            }
        },
    >;

    /// Reciprocal ångströms (`Å⁻¹`), a unit of `ReciprocalLength`: `n` of them is `n · 10000000000.0` coherent units.
    pub type ReciprocalAngstrom = SIUnit<
        RECIPROCAL_LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(10000000000),
                zero: Exact::ZERO,
                symbol: "Å⁻¹",
                name: "reciprocal ångströms",
            }
        },
    >;

    /// Diopters (`dpt`), a unit of `ReciprocalLength`: `n` of them is `n · 1.0` coherent units.
    pub type Diopter = SIUnit<
        RECIPROCAL_LENGTH,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "dpt",
                name: "diopters",
            }
        },
    >;

    /// Rydberg constants (`R∞`), a unit of `ReciprocalLength`: `n` of them is `n · 10973731.56816` coherent units.
    pub type RydbergConstant = SIUnit<
        RECIPROCAL_LENGTH,
        {
            SIUnitConsts {
                size: Exact::dec(1097373156816, -5),
                zero: Exact::ZERO,
                symbol: "R∞",
                name: "Rydberg constants",
            }
        },
    >;

    /// Radians per metre (`rad/m`), a unit of `Curvature`: `n` of them is `n · 1.0` coherent units.
    pub type RadianPerMetre = SIUnit<
        CURVATURE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "rad/m",
                name: "radians per metre",
            }
        },
    >;

    /// Degrees per metre (`°/m`), a unit of `Curvature`: `n` of them is `n · core::f64::consts::PI / 180.0` coherent units.
    pub type DegreePerMetre = SIUnit<
        CURVATURE,
        {
            SIUnitConsts {
                size: Exact::PI / Exact::int(180),
                zero: Exact::ZERO,
                symbol: "°/m",
                name: "degrees per metre",
            }
        },
    >;

    /// Square metres per kilogram (`m²/kg`), a unit of `SpecificArea`: `n` of them is `n · 1.0` coherent units.
    pub type SquareMetrePerKilogram = SIUnit<
        SPECIFIC_AREA,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m²/kg",
                name: "square metres per kilogram",
            }
        },
    >;

    /// Square metres per gram (`m²/g`), a unit of `SpecificArea`: `n` of them is `n · 1000.0` coherent units.
    pub type SquareMetrePerGram = SIUnit<
        SPECIFIC_AREA,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "m²/g",
                name: "square metres per gram",
            }
        },
    >;

    /// Cubic metres per kilogram (`m³/kg`), a unit of `SpecificVolume`: `n` of them is `n · 1.0` coherent units.
    pub type CubicMetrePerKilogram = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³/kg",
                name: "cubic metres per kilogram",
            }
        },
    >;

    /// Cubic metres per gram (`m³/g`), a unit of `SpecificVolume`: `n` of them is `n · 1000.0` coherent units.
    pub type CubicMetrePerGram = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "m³/g",
                name: "cubic metres per gram",
            }
        },
    >;

    /// Cubic metres per carat (`m³/ct`), a unit of `SpecificVolume`: `n` of them is `n · 5000.0` coherent units.
    pub type CubicMetrePerCarat = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(5000),
                zero: Exact::ZERO,
                symbol: "m³/ct",
                name: "cubic metres per carat",
            }
        },
    >;

    /// Cubic metres per grain (`m³/gr`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 6.479891e-05` coherent units.
    pub type CubicMetrePerGrain = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "m³/gr",
                name: "cubic metres per grain",
            }
        },
    >;

    /// Cubic metres per hundredweight (long) (`m³/cwt long`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 50.80234544` coherent units.
    pub type CubicMetrePerHundredweightLong = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(5080234544, -8),
                zero: Exact::ZERO,
                symbol: "m³/cwt long",
                name: "cubic metres per hundredweight (long)",
            }
        },
    >;

    /// Cubic metres per hundredweight (short) (`m³/cwt short`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 45.359237` coherent units.
    pub type CubicMetrePerHundredweightShort = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(45359237, -6),
                zero: Exact::ZERO,
                symbol: "m³/cwt short",
                name: "cubic metres per hundredweight (short)",
            }
        },
    >;

    /// Cubic metres per ounce (`m³/oz`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.028349523125` coherent units.
    pub type CubicMetrePerOunce = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "m³/oz",
                name: "cubic metres per ounce",
            }
        },
    >;

    /// Cubic metres per troy ounce (`m³/oz t`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.0311034768` coherent units.
    pub type CubicMetrePerOunceTroy = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(311034768, -10),
                zero: Exact::ZERO,
                symbol: "m³/oz t",
                name: "cubic metres per troy ounce",
            }
        },
    >;

    /// Cubic metres per pennyweight (`m³/dwt`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.00155517384` coherent units.
    pub type CubicMetrePerPennyweight = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(155517384, -11),
                zero: Exact::ZERO,
                symbol: "m³/dwt",
                name: "cubic metres per pennyweight",
            }
        },
    >;

    /// Cubic metres per pound (`m³/lb`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.45359237` coherent units.
    pub type CubicMetrePerPound = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "m³/lb",
                name: "cubic metres per pound",
            }
        },
    >;

    /// Cubic metres per troy pound (`m³/lb t`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 0.3732417216` coherent units.
    pub type CubicMetrePerPoundTroy = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(3732417216, -10),
                zero: Exact::ZERO,
                symbol: "m³/lb t",
                name: "cubic metres per troy pound",
            }
        },
    >;

    /// Cubic metres per slug (`m³/slug`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / (0.45359237 * 9.80665 / 0.3048)` coherent units.
    pub type CubicMetrePerSlug = SIUnit<
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
    >;

    /// Cubic metres per assay ton (`m³/AT`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / (0.175 / 6.0)` coherent units.
    pub type CubicMetrePerTonAssay = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / (Exact::dec(175, -3) / Exact::int(6)),
                zero: Exact::ZERO,
                symbol: "m³/AT",
                name: "cubic metres per assay ton",
            }
        },
    >;

    /// Cubic metres per long ton (`m³/2240 lb`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 1016.0469088` coherent units.
    pub type CubicMetrePerTonLong = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(10160469088, -7),
                zero: Exact::ZERO,
                symbol: "m³/2240 lb",
                name: "cubic metres per long ton",
            }
        },
    >;

    /// Cubic metres per short ton (`m³/2000 lb`), a unit of `SpecificVolume`: `n` of them is `n · 1.0 / 907.18474` coherent units.
    pub type CubicMetrePerTonShort = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::dec(90718474, -5),
                zero: Exact::ZERO,
                symbol: "m³/2000 lb",
                name: "cubic metres per short ton",
            }
        },
    >;

    /// Cubic metres per ton (`m³/t`), a unit of `SpecificVolume`: `n` of them is `n · 0.001` coherent units.
    pub type CubicMetrePerTon = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "m³/t",
                name: "cubic metres per ton",
            }
        },
    >;

    /// Gallons per grain (`gal/gr`), a unit of `SpecificVolume`: `n` of them is `n · 0.003785411784 / 6.479891e-05` coherent units.
    pub type GallonPerGrain = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::dec(6479891, -11),
                zero: Exact::ZERO,
                symbol: "gal/gr",
                name: "gallons per grain",
            }
        },
    >;

    /// Cubic inches per ounce (`in³/oz`), a unit of `SpecificVolume`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254 / 0.028349523125` coherent units.
    pub type CubicInchPerOunce = SIUnit<
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
    >;

    /// Imperial gallons per ounce (`gal (UK)/oz`), a unit of `SpecificVolume`: `n` of them is `n · 0.00454609 / 0.028349523125` coherent units.
    pub type GallonImperialPerOunce = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(454609, -8) / Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "gal (UK)/oz",
                name: "Imperial gallons per ounce",
            }
        },
    >;

    /// Gallons per ounce (`gal/oz`), a unit of `SpecificVolume`: `n` of them is `n · 0.003785411784 / 0.028349523125` coherent units.
    pub type GallonPerOunce = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::dec(28349523125, -12),
                zero: Exact::ZERO,
                symbol: "gal/oz",
                name: "gallons per ounce",
            }
        },
    >;

    /// Cubic feet per pound (`ft³/lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.3048 * 0.3048 * 0.3048) / 0.45359237` coherent units.
    pub type CubicFootPerPound = SIUnit<
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
    >;

    /// Cubic inches per pound (`in³/lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.0254 * 0.0254 * 0.0254) / 0.45359237` coherent units.
    pub type CubicInchPerPound = SIUnit<
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
    >;

    /// Cubic yards per pound (`yd³/lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.9144 * 0.9144 * 0.9144) / 0.45359237` coherent units.
    pub type CubicYardPerPound = SIUnit<
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
    >;

    /// Imperial gallons per pound (`gal (UK)/lb`), a unit of `SpecificVolume`: `n` of them is `n · 0.00454609 / 0.45359237` coherent units.
    pub type GallonImperialPerPound = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(454609, -8) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "gal (UK)/lb",
                name: "Imperial gallons per pound",
            }
        },
    >;

    /// Gallons per pound (`gal/lb`), a unit of `SpecificVolume`: `n` of them is `n · 0.003785411784 / 0.45359237` coherent units.
    pub type GallonPerPound = SIUnit<
        SPECIFIC_VOLUME,
        {
            SIUnitConsts {
                size: Exact::dec(3785411784, -12) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "gal/lb",
                name: "gallons per pound",
            }
        },
    >;

    /// Cubic feett per slug (`ft³/slug`), a unit of `SpecificVolume`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048 / (0.45359237 * 9.80665 / 0.3048)` coherent units.
    pub type CubicFootPerSlug = SIUnit<
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
    >;

    /// Cubic yards per long ton (`yd³/2240 lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.9144 * 0.9144 * 0.9144) / 1016.0469088` coherent units.
    pub type CubicYardPerTonLong = SIUnit<
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
    >;

    /// Cubic yards per short ton (`yd³/2000 lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.9144 * 0.9144 * 0.9144) / 907.18474` coherent units.
    pub type CubicYardPerTonShort = SIUnit<
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
    >;

    /// Cubic feet per long ton (`ft³/2240 lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.3048 * 0.3048 * 0.3048) / 1016.0469088` coherent units.
    pub type CubicFootPerTonLong = SIUnit<
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
    >;

    /// Cubic feet per short ton (`ft³/2000 lb`), a unit of `SpecificVolume`: `n` of them is `n · (0.3048 * 0.3048 * 0.3048) / 907.18474` coherent units.
    pub type CubicFootPerTonShort = SIUnit<
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
    >;

    /// Watts per kilogram (`W/kg`), a unit of `SpecificPower`: `n` of them is `n · 1.0` coherent units.
    pub type WattPerKilogram = SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W/kg",
                name: "watts per kilogram",
            }
        },
    >;

    /// Square metres per cubic second (`m² · s⁻³`), a unit of `SpecificPower`: `n` of them is `n · 1.0` coherent units.
    pub type SquareMetrePerCubicSecond = SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m² · s⁻³",
                name: "square metres per cubic second",
            }
        },
    >;

    /// Ergs per second per kilogram (`(erg/s)/kg`), a unit of `SpecificPower`: `n` of them is `n · 1e-07` coherent units.
    pub type ErgPerSecondPerKilogram = SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "(erg/s)/kg",
                name: "ergs per second per kilogram",
            }
        },
    >;

    /// Foot pounds-force per hour per pound (`(ft · lbf/h)/lb`), a unit of `SpecificPower`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / (3600.0 * 0.45359237)` coherent units.
    pub type FootPoundPerHourPerPound = SIUnit<
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
    >;

    /// Foot pounds-force per minute per pound (`(ft · lbf/min)/lb`), a unit of `SpecificPower`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / (60.0 * 0.45359237)` coherent units.
    pub type FootPoundPerMinutePerPound = SIUnit<
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
    >;

    /// Foot pounds-force per second per pound (`(ft · lbf/s)/lb`), a unit of `SpecificPower`: `n` of them is `n · (0.3048 * 0.45359237 * 9.80665) / 0.45359237` coherent units.
    pub type FootPoundPerSecondPerPound = SIUnit<
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
    >;

    /// Horsepower per pound (`hp/lb`), a unit of `SpecificPower`: `n` of them is `n · (550.0 * 0.3048 * 0.45359237 * 9.80665) / 0.45359237` coherent units.
    pub type HorsepowerPerPound = SIUnit<
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
    >;

    /// Horsepower (boiler) per pound (`(hp (S))/lb`), a unit of `SpecificPower`: `n` of them is `n · 9809.5 / 0.45359237` coherent units.
    pub type HorsepowerBoilerPerPound = SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::dec(98095, -1) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (S))/lb",
                name: "horsepower (boiler) per pound",
            }
        },
    >;

    /// Horsepower (electric) per pound (`(hp (E))/lb`), a unit of `SpecificPower`: `n` of them is `n · 746.0 / 0.45359237` coherent units.
    pub type HorsepowerElectricPerPound = SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::int(746) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (E))/lb",
                name: "horsepower (electric) per pound",
            }
        },
    >;

    /// Metric horsepower per pound (`(hp (M))/lb`), a unit of `SpecificPower`: `n` of them is `n · (75.0 * 9.80665) / 0.45359237` coherent units.
    pub type HorsepowerMetricPerPound = SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: (Exact::int(75) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (M))/lb",
                name: "metric horsepower per pound",
            }
        },
    >;

    /// Horsepower (Imperial) per pound (`(hp (I))/lb`), a unit of `SpecificPower`: `n` of them is `n · 745.7 / 0.45359237` coherent units.
    pub type HorsepowerImperialPerPound = SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::dec(7457, -1) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (I))/lb",
                name: "horsepower (Imperial) per pound",
            }
        },
    >;

    /// Hydraulic horsepower per pound (`(hp (hydraulic))/lb`), a unit of `SpecificPower`: `n` of them is `n · 746.043 / 0.45359237` coherent units.
    pub type HydraulicHorsepowerPerPound = SIUnit<
        SPECIFIC_POWER,
        {
            SIUnitConsts {
                size: Exact::dec(746043, -3) / Exact::dec(45359237, -8),
                zero: Exact::ZERO,
                symbol: "(hp (hydraulic))/lb",
                name: "hydraulic horsepower per pound",
            }
        },
    >;

    /// Cubic metres per kilogram second squared (`m³/(kg · s²)`), a unit of `SpecificGravitationalParameter`: `n` of them is `n · 1.0` coherent units.
    pub type CubicMetrePerKilogramSecondSquared = SIUnit<
        SPECIFIC_GRAVITATIONAL_PARAMETER,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³/(kg · s²)",
                name: "cubic metres per kilogram second squared",
            }
        },
    >;

    /// Newtonian constants of gravitation (`G`), a unit of `SpecificGravitationalParameter`: `n` of them is `n · 6.6743e-11` coherent units.
    pub type GravitationalConstant = SIUnit<
        SPECIFIC_GRAVITATIONAL_PARAMETER,
        {
            SIUnitConsts {
                size: Exact::dec(66743, -15),
                zero: Exact::ZERO,
                symbol: "G",
                name: "Newtonian constants of gravitation",
            }
        },
    >;
}
