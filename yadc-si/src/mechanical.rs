use crate::{ISQ, Kind, Scale, StaticSIQuantity};
use consts::*;
use yadc::Dimension;

pub type Length = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Mass = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Time = StaticSIQuantity<
    {
        ISQ {
            s: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Instant = StaticSIQuantity<
    {
        ISQ {
            s: 1,
            kind: Kind::Affine,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Area = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Volume = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Velocity = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Acceleration = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Jerk = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Absement = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type InverseVelocity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            s: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Force = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Momentum = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Energy = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type AvailableEnergy = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Power = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Gain = StaticSIQuantity<
    {
        ISQ {
            kind: Kind::Log,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type PowerLevel = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            kind: Kind::Log,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type PowerRate = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -4,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Action = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Torque = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            rad: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type MomentOfInertia = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            rad: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Pressure = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Frequency = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type FrequencyDrift = StaticSIQuantity<
    {
        ISQ {
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type ReciprocalLength = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type Curvature = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type SpecificArea = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type SpecificVolume = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            kg: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type SpecificPower = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

pub type SpecificGravitationalParameter = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            kg: -1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per multiple, each defined from them - `KILOMETRE`
/// is `1000.0 * METRE`, a `Length` and a unit of its own (`KILOMETRE.value(d)`).
/// Quantity arithmetic is `const`, so they compose in `const` items:
/// `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
pub mod consts {
    use super::*;
    use crate::base::{ONE, RADIAN};

    /// The metre, the SI base unit of length.
    pub use crate::base::METRE;

    /// The kilogram, the SI base unit of mass.
    pub use crate::base::KILOGRAM;

    /// The second, the SI base unit of time.
    pub use crate::base::SECOND;

    /// The bel: a gain of ten times in power. The unit of the logarithmic line.
    pub use crate::base::BEL;

    /// The Unix epoch, 1970-01-01T00:00:00Z: the base zero of `Instant`. An instant is stored, printed and serialized as seconds from here; every other zero of the line is defined from it.
    pub use crate::base::UNIX_EPOCH;

    /// The level of one watt, 0 dBW: where the numbers of a power level start.
    pub use crate::base::WATT_LEVEL;

    /// One `Yottametre` (Ym), as a `Length`.
    pub const YOTTAMETRE: Length = 9.9999999999999998e+23 * METRE;

    /// One `Zettametre` (Zm), as a `Length`.
    pub const ZETTAMETRE: Length = 1e+21 * METRE;

    /// One `Exametre` (Em), as a `Length`.
    pub const EXAMETRE: Length = 1e+18 * METRE;

    /// One `Petametre` (Pm), as a `Length`.
    pub const PETAMETRE: Length = 1000000000000000.0 * METRE;

    /// One `Terametre` (Tm), as a `Length`.
    pub const TERAMETRE: Length = 1000000000000.0 * METRE;

    /// One `Gigametre` (Gm), as a `Length`.
    pub const GIGAMETRE: Length = 1000000000.0 * METRE;

    /// One `Megametre` (Mm), as a `Length`.
    pub const MEGAMETRE: Length = 1000000.0 * METRE;

    /// One `Kilometre` (km), as a `Length`.
    pub const KILOMETRE: Length = 1000.0 * METRE;

    /// One `Hectometre` (hm), as a `Length`.
    pub const HECTOMETRE: Length = 100.0 * METRE;

    /// One `Decametre` (dam), as a `Length`.
    pub const DECAMETRE: Length = 10.0 * METRE;

    /// One `Decimetre` (dm), as a `Length`.
    pub const DECIMETRE: Length = 0.10000000000000001 * METRE;

    /// One `Centimetre` (cm), as a `Length`.
    pub const CENTIMETRE: Length = 0.01 * METRE;

    /// One `Millimetre` (mm), as a `Length`.
    pub const MILLIMETRE: Length = 0.001 * METRE;

    /// One `Micrometre` (µm), as a `Length`.
    pub const MICROMETRE: Length = 9.9999999999999995e-07 * METRE;

    /// One `Nanometre` (nm), as a `Length`.
    pub const NANOMETRE: Length = 1.0000000000000001e-09 * METRE;

    /// One `Picometre` (pm), as a `Length`.
    pub const PICOMETRE: Length = 9.9999999999999998e-13 * METRE;

    /// One `Femtometre` (fm), as a `Length`.
    pub const FEMTOMETRE: Length = 1.0000000000000001e-15 * METRE;

    /// One `Attometre` (am), as a `Length`.
    pub const ATTOMETRE: Length = 1.0000000000000001e-18 * METRE;

    /// One `Zeptometre` (zm), as a `Length`.
    pub const ZEPTOMETRE: Length = 9.9999999999999991e-22 * METRE;

    /// One `Yoctometre` (ym), as a `Length`.
    pub const YOCTOMETRE: Length = 9.9999999999999992e-25 * METRE;

    /// One `Angstrom` (Å), as a `Length`.
    pub const ANGSTROM: Length = 1e-10 * METRE;

    /// One `BohrRadius` (a₀), as a `Length`.
    pub const BOHR_RADIUS: Length = 5.2917721090299998e-11 * METRE;

    /// One `AtomicUnitOfLength` (a.u. of length), as a `Length`.
    pub const ATOMIC_UNIT_OF_LENGTH: Length = 5.2917721090299998e-11 * METRE;

    /// One `AstronomicalUnit` (ua), as a `Length`.
    pub const ASTRONOMICAL_UNIT: Length = 149597870700.0 * METRE;

    /// One `Chain` (ch), as a `Length`.
    pub const CHAIN: Length = (66.0 * 1200.0 / 3937.0) * METRE;

    /// One `DataMile` (DM), as a `Length`.
    pub const DATA_MILE: Length = 1828.8 * METRE;

    /// One `Fathom` (fathom), as a `Length`.
    pub const FATHOM: Length = (6.0 * 1200.0 / 3937.0) * METRE;

    /// One `Fermi` (fermi), as a `Length`.
    pub const FERMI: Length = 1.0000000000000001e-15 * METRE;

    /// One `Foot` (ft), as a `Length`.
    pub const FOOT: Length = 0.30480000000000002 * METRE;

    /// One `FootSurvey` (ft (U.S. survey)), as a `Length`.
    pub const FOOT_SURVEY: Length = (1200.0 / 3937.0) * METRE;

    /// One `Inch` (in), as a `Length`.
    pub const INCH: Length = 0.025399999999999999 * METRE;

    /// One `LightYear` (l. y.), as a `Length`.
    pub const LIGHT_YEAR: Length = 9460730472580800.0 * METRE;

    /// One `Microinch` (μin), as a `Length`.
    pub const MICROINCH: Length = 2.5399999999999999e-08 * METRE;

    /// One `Micron` (μ), as a `Length`.
    pub const MICRON: Length = 9.9999999999999995e-07 * METRE;

    /// One `Mil` (0.001 in), as a `Length`.
    pub const MIL: Length = 2.5400000000000001e-05 * METRE;

    /// One `Mile` (mi), as a `Length`.
    pub const MILE: Length = 1609.3440000000001 * METRE;

    /// One `MileSurvey` (mi (U.S. survey)), as a `Length`.
    pub const MILE_SURVEY: Length = (6336000.0 / 3937.0) * METRE;

    /// One `NauticalMile` (M), as a `Length`.
    pub const NAUTICAL_MILE: Length = 1852.0 * METRE;

    /// One `Parsec` (pc), as a `Length`.
    pub const PARSEC: Length = (149597870700.0 * 648000.0 / core::f64::consts::PI) * METRE;

    /// One `PicaComputer` (1/6 in (computer)), as a `Length`.
    pub const PICA_COMPUTER: Length = 0.0042333333333333329 * METRE;

    /// One `PicaPrinters` (1/6 in), as a `Length`.
    pub const PICA_PRINTERS: Length = 0.004217518 * METRE;

    /// One `PointComputer` (1/72 in (computer)), as a `Length`.
    pub const POINT_COMPUTER: Length = (0.0254 / 72.0) * METRE;

    /// One `PointPrinters` (1/72 in), as a `Length`.
    pub const POINT_PRINTERS: Length = 0.00035145979999999999 * METRE;

    /// One `Rod` (rd), as a `Length`.
    pub const ROD: Length = (16.5 * 1200.0 / 3937.0) * METRE;

    /// One `Yard` (yd), as a `Length`.
    pub const YARD: Length = 0.91439999999999999 * METRE;

    /// One `Leuga` (leuga), as a `Length`.
    pub const LEUGA: Length = 2220.0 * METRE;

    /// One `MillePassus` (mille passus), as a `Length`.
    pub const MILLE_PASSUS: Length = 1480.0 * METRE;

    /// One `Stadium` (stadium), as a `Length`.
    pub const STADIUM: Length = 185.0 * METRE;

    /// One `Actus` (actus), as a `Length`.
    pub const ACTUS: Length = 35.520000000000003 * METRE;

    /// One `Decempeda` (decempeda), as a `Length`.
    pub const DECEMPEDA: Length = 2.96 * METRE;

    /// One `Passus` (passus), as a `Length`.
    pub const PASSUS: Length = 1.48 * METRE;

    /// One `Gradus` (gradus), as a `Length`.
    pub const GRADUS: Length = 0.73999999999999999 * METRE;

    /// One `Cubitum` (cubitum), as a `Length`.
    pub const CUBITUM: Length = 0.44400000000000001 * METRE;

    /// One `Palmipes` (palmipes), as a `Length`.
    pub const PALMIPES: Length = 0.37 * METRE;

    /// One `Pes` (pes), as a `Length`.
    pub const PES: Length = 0.29599999999999999 * METRE;

    /// One `PalmusMaior` (palmus maior), as a `Length`.
    pub const PALMUS_MAIOR: Length = 0.222 * METRE;

    /// One `Palmus` (palmus), as a `Length`.
    pub const PALMUS: Length = 0.073999999999999996 * METRE;

    /// One `Uncia` (uncia), as a `Length`.
    pub const LENGTH_UNCIA: Length = 0.024666666666666663 * METRE;

    /// One `Digitus` (digitus), as a `Length`.
    pub const DIGITUS: Length = 0.018499999999999999 * METRE;

    /// One `Yottagram` (Yg), as a `Mass`.
    pub const YOTTAGRAM: Mass = 1e+21 * KILOGRAM;

    /// One `Zettagram` (Zg), as a `Mass`.
    pub const ZETTAGRAM: Mass = 1e+18 * KILOGRAM;

    /// One `Exagram` (Eg), as a `Mass`.
    pub const EXAGRAM: Mass = 1000000000000000.0 * KILOGRAM;

    /// One `Petagram` (Pg), as a `Mass`.
    pub const PETAGRAM: Mass = 1000000000000.0 * KILOGRAM;

    /// One `Teragram` (Tg), as a `Mass`.
    pub const TERAGRAM: Mass = 1000000000.0 * KILOGRAM;

    /// One `Gigagram` (Gg), as a `Mass`.
    pub const GIGAGRAM: Mass = 1000000.0 * KILOGRAM;

    /// One `Megagram` (Mg), as a `Mass`.
    pub const MEGAGRAM: Mass = 1000.0 * KILOGRAM;

    /// One `Hectogram` (hg), as a `Mass`.
    pub const HECTOGRAM: Mass = 0.10000000000000001 * KILOGRAM;

    /// One `Decagram` (dag), as a `Mass`.
    pub const DECAGRAM: Mass = 0.01 * KILOGRAM;

    /// One `Gram` (g), as a `Mass`.
    pub const GRAM: Mass = 0.001 * KILOGRAM;

    /// One `Decigram` (dg), as a `Mass`.
    pub const DECIGRAM: Mass = 0.0001 * KILOGRAM;

    /// One `Centigram` (cg), as a `Mass`.
    pub const CENTIGRAM: Mass = 1.0000000000000001e-05 * KILOGRAM;

    /// One `Milligram` (mg), as a `Mass`.
    pub const MILLIGRAM: Mass = 9.9999999999999995e-07 * KILOGRAM;

    /// One `Microgram` (µg), as a `Mass`.
    pub const MICROGRAM: Mass = 9.9999999999999986e-10 * KILOGRAM;

    /// One `Nanogram` (ng), as a `Mass`.
    pub const NANOGRAM: Mass = 9.9999999999999998e-13 * KILOGRAM;

    /// One `Picogram` (pg), as a `Mass`.
    pub const PICOGRAM: Mass = 1.0000000000000001e-15 * KILOGRAM;

    /// One `Femtogram` (fg), as a `Mass`.
    pub const FEMTOGRAM: Mass = 1.0000000000000001e-18 * KILOGRAM;

    /// One `Attogram` (ag), as a `Mass`.
    pub const ATTOGRAM: Mass = 1.0000000000000001e-21 * KILOGRAM;

    /// One `Zeptogram` (zg), as a `Mass`.
    pub const ZEPTOGRAM: Mass = 9.9999999999999992e-25 * KILOGRAM;

    /// One `Yoctogram` (yg), as a `Mass`.
    pub const YOCTOGRAM: Mass = 9.9999999999999986e-28 * KILOGRAM;

    /// One `Carat` (ct), as a `Mass`.
    pub const CARAT: Mass = 0.00020000000000000001 * KILOGRAM;

    /// One `Dalton` (Da), as a `Mass`.
    pub const DALTON: Mass = 1.6605390666e-27 * KILOGRAM;

    /// One `ElectronMass` (mₑ), as a `Mass`.
    pub const ELECTRON_MASS: Mass = 9.1093837015e-31 * KILOGRAM;

    /// One `ProtonMass` (mₚ), as a `Mass`.
    pub const PROTON_MASS: Mass = 1.67262192369e-27 * KILOGRAM;

    /// One `Grain` (gr), as a `Mass`.
    pub const GRAIN: Mass = 6.4798909999999995e-05 * KILOGRAM;

    /// One `HundredweightLong` (cwt long), as a `Mass`.
    pub const HUNDREDWEIGHT_LONG: Mass = 50.80234544 * KILOGRAM;

    /// One `HundredweightShort` (cwt short), as a `Mass`.
    pub const HUNDREDWEIGHT_SHORT: Mass = 45.359237 * KILOGRAM;

    /// One `Ounce` (oz), as a `Mass`.
    pub const OUNCE: Mass = 0.028349523125 * KILOGRAM;

    /// One `OunceTroy` (oz t), as a `Mass`.
    pub const OUNCE_TROY: Mass = 0.0311034768 * KILOGRAM;

    /// One `Pennyweight` (dwt), as a `Mass`.
    pub const PENNYWEIGHT: Mass = 0.00155517384 * KILOGRAM;

    /// One `Pound` (lb), as a `Mass`.
    pub const POUND: Mass = 0.45359237 * KILOGRAM;

    /// One `PoundTroy` (lb t), as a `Mass`.
    pub const POUND_TROY: Mass = 0.3732417216 * KILOGRAM;

    /// One `Slug` (slug), as a `Mass`.
    pub const SLUG: Mass = (0.45359237 * 9.80665 / 0.3048) * KILOGRAM;

    /// One `TonAssay` (AT), as a `Mass`.
    pub const TON_ASSAY: Mass = (0.175 / 6.0) * KILOGRAM;

    /// One `TonLong` (2240 lb), as a `Mass`.
    pub const TON_LONG: Mass = 1016.0469088 * KILOGRAM;

    /// One `TonShort` (2000 lb), as a `Mass`.
    pub const TON_SHORT: Mass = 907.18474 * KILOGRAM;

    /// One `Ton` (t), as a `Mass`.
    pub const TON: Mass = 1000.0 * KILOGRAM;

    /// One `Libra` (libra), as a `Mass`.
    pub const LIBRA: Mass = 0.32890000000000003 * KILOGRAM;

    /// One `Deunx` (deunx), as a `Mass`.
    pub const MASS_DEUNX: Mass = 0.30149166666666666 * KILOGRAM;

    /// One `Dextans` (dextans), as a `Mass`.
    pub const MASS_DEXTANS: Mass = 0.27408333333333335 * KILOGRAM;

    /// One `Dodrans` (dodrans), as a `Mass`.
    pub const MASS_DODRANS: Mass = 0.24667500000000001 * KILOGRAM;

    /// One `Bes` (bes), as a `Mass`.
    pub const MASS_BES: Mass = 0.21926666666666667 * KILOGRAM;

    /// One `Septunx` (septunx), as a `Mass`.
    pub const MASS_SEPTUNX: Mass = 0.19185833333333335 * KILOGRAM;

    /// One `Semis` (semis), as a `Mass`.
    pub const MASS_SEMIS: Mass = 0.16445000000000001 * KILOGRAM;

    /// One `Quincunx` (quincunx), as a `Mass`.
    pub const MASS_QUINCUNX: Mass = 0.13704166666666667 * KILOGRAM;

    /// One `Triens` (triens), as a `Mass`.
    pub const MASS_TRIENS: Mass = 0.10963333333333333 * KILOGRAM;

    /// One `QuadransTeruncius` (quadrans teruncius), as a `Mass`.
    pub const QUADRANS_TERUNCIUS: Mass = 0.082225000000000006 * KILOGRAM;

    /// One `Sextans` (sextans), as a `Mass`.
    pub const MASS_SEXTANS: Mass = 0.054816666666666701 * KILOGRAM;

    /// One `Sescuncia` (sescuncia), as a `Mass`.
    pub const SESCUNCIA: Mass = 0.041112500000000003 * KILOGRAM;

    /// One `Uncia` (uncia), as a `Mass`.
    pub const MASS_UNCIA: Mass = 0.027408333333333337 * KILOGRAM;

    /// One `Semuncia` (semuncia), as a `Mass`.
    pub const SEMUNCIA: Mass = 0.0137041666666667 * KILOGRAM;

    /// One `Duella` (duella), as a `Mass`.
    pub const DUELLA: Mass = 0.0091361111111111105 * KILOGRAM;

    /// One `Sicilicus` (sicilicus), as a `Mass`.
    pub const MASS_SICILICUS: Mass = 0.0068520833333333298 * KILOGRAM;

    /// One `Sextula` (sextula), as a `Mass`.
    pub const MASS_SEXTULA: Mass = 0.0045680555555555596 * KILOGRAM;

    /// One `Semisextula` (semisextula), as a `Mass`.
    pub const SEMISEXTULA: Mass = 0.0022840277777777798 * KILOGRAM;

    /// One `Scrupulum` (scrupulum), as a `Mass`.
    pub const MASS_SCRUPULUM: Mass = 0.0011420138888888899 * KILOGRAM;

    /// One `Obolus` (obolus), as a `Mass`.
    pub const OBOLUS: Mass = 0.00057100694444444397 * KILOGRAM;

    /// One `Siliqua` (siliqua), as a `Mass`.
    pub const SILIQUA: Mass = 0.000190335648148148 * KILOGRAM;

    /// One `Yottasecond` (Ys), as a `Time`.
    pub const YOTTASECOND: Time = 9.9999999999999998e+23 * SECOND;

    /// One `Zettasecond` (Zs), as a `Time`.
    pub const ZETTASECOND: Time = 1e+21 * SECOND;

    /// One `Exasecond` (Es), as a `Time`.
    pub const EXASECOND: Time = 1e+18 * SECOND;

    /// One `Petasecond` (Ps), as a `Time`.
    pub const PETASECOND: Time = 1000000000000000.0 * SECOND;

    /// One `Terasecond` (Ts), as a `Time`.
    pub const TERASECOND: Time = 1000000000000.0 * SECOND;

    /// One `Gigasecond` (Gs), as a `Time`.
    pub const GIGASECOND: Time = 1000000000.0 * SECOND;

    /// One `Megasecond` (Ms), as a `Time`.
    pub const MEGASECOND: Time = 1000000.0 * SECOND;

    /// One `Kilosecond` (ks), as a `Time`.
    pub const KILOSECOND: Time = 1000.0 * SECOND;

    /// One `Hectosecond` (hs), as a `Time`.
    pub const HECTOSECOND: Time = 100.0 * SECOND;

    /// One `Decasecond` (das), as a `Time`.
    pub const DECASECOND: Time = 10.0 * SECOND;

    /// One `Decisecond` (ds), as a `Time`.
    pub const DECISECOND: Time = 0.10000000000000001 * SECOND;

    /// One `Centisecond` (cs), as a `Time`.
    pub const CENTISECOND: Time = 0.01 * SECOND;

    /// One `Millisecond` (ms), as a `Time`.
    pub const MILLISECOND: Time = 0.001 * SECOND;

    /// One `Microsecond` (µs), as a `Time`.
    pub const MICROSECOND: Time = 9.9999999999999995e-07 * SECOND;

    /// One `Nanosecond` (ns), as a `Time`.
    pub const NANOSECOND: Time = 1.0000000000000001e-09 * SECOND;

    /// One `Picosecond` (ps), as a `Time`.
    pub const PICOSECOND: Time = 9.9999999999999998e-13 * SECOND;

    /// One `Femtosecond` (fs), as a `Time`.
    pub const FEMTOSECOND: Time = 1.0000000000000001e-15 * SECOND;

    /// One `Attosecond` (as), as a `Time`.
    pub const ATTOSECOND: Time = 1.0000000000000001e-18 * SECOND;

    /// One `Zeptosecond` (zs), as a `Time`.
    pub const ZEPTOSECOND: Time = 9.9999999999999991e-22 * SECOND;

    /// One `Yoctosecond` (ys), as a `Time`.
    pub const YOCTOSECOND: Time = 9.9999999999999992e-25 * SECOND;

    /// One `SecondSidereal` (s (sidereal)), as a `Time`.
    pub const SECOND_SIDEREAL: Time = 0.99726959999999998 * SECOND;

    /// One `Day` (d), as a `Time`.
    pub const DAY: Time = 86400.0 * SECOND;

    /// One `DaySidereal` (d (sidereal)), as a `Time`.
    pub const DAY_SIDEREAL: Time = 86164.089999999997 * SECOND;

    /// One `Hour` (h), as a `Time`.
    pub const HOUR: Time = 3600.0 * SECOND;

    /// One `HourSidereal` (h (sidereal)), as a `Time`.
    pub const HOUR_SIDEREAL: Time = 3590.1700000000001 * SECOND;

    /// One `Minute` (min), as a `Time`.
    pub const MINUTE: Time = 60.0 * SECOND;

    /// One `Shake` (10.0 ns), as a `Time`.
    pub const SHAKE: Time = 1e-08 * SECOND;

    /// One `Year` (a), as a `Time`.
    pub const YEAR: Time = 31536000.0 * SECOND;

    /// One `YearSidereal` (a (sidereal)), as a `Time`.
    pub const YEAR_SIDEREAL: Time = 31558150.0 * SECOND;

    /// One `YearTropical` (a (tropical)), as a `Time`.
    pub const YEAR_TROPICAL: Time = 31556930.0 * SECOND;

    /// The GPS epoch, 1980-01-06T00:00:00Z: 3657 days after the Unix epoch.
    pub const GPS_EPOCH: Instant = UNIX_EPOCH + 315964800.0 * SECOND;

    /// The zero of the Julian date: noon on 1 January 4713 BC (proleptic Julian), 2 440 587.5 days before the Unix epoch.
    pub const JULIAN_EPOCH: Instant = UNIX_EPOCH - 2440587.5 * DAY;

    /// The zero of the modified Julian date, 1858-11-17T00:00:00Z: 40 587 days before the Unix epoch.
    pub const MODIFIED_JULIAN_EPOCH: Instant = UNIX_EPOCH - 40587.0 * DAY;

    /// Unix time: seconds from the Unix epoch.
    pub const UNIX_SECONDS: Scale<Instant, Time> = Scale(UNIX_EPOCH, SECOND);

    /// Unix time in milliseconds.
    pub const UNIX_MILLISECONDS: Scale<Instant, Time> = Scale(UNIX_EPOCH, MILLISECOND);

    /// GPS time: seconds from the GPS epoch.
    pub const GPS_SECONDS: Scale<Instant, Time> = Scale(GPS_EPOCH, SECOND);

    /// The Julian date (JD): days from the Julian epoch.
    pub const JULIAN_DATE: Scale<Instant, Time> = Scale(JULIAN_EPOCH, DAY);

    /// The modified Julian date (MJD): days from the modified Julian epoch.
    pub const MODIFIED_JULIAN_DATE: Scale<Instant, Time> = Scale(MODIFIED_JULIAN_EPOCH, DAY);

    /// One `SquareYottametre` (Ym²), as an `Area`.
    pub const SQUARE_YOTTAMETRE: Area = 1e+48 * SQUARE_METRE;

    /// One `SquareZettametre` (Zm²), as an `Area`.
    pub const SQUARE_ZETTAMETRE: Area = 1e+42 * SQUARE_METRE;

    /// One `SquareExametre` (Em²), as an `Area`.
    pub const SQUARE_EXAMETRE: Area = 1e+36 * SQUARE_METRE;

    /// One `SquarePetametre` (Pm²), as an `Area`.
    pub const SQUARE_PETAMETRE: Area = 1e+30 * SQUARE_METRE;

    /// One `SquareTerametre` (Tm²), as an `Area`.
    pub const SQUARE_TERAMETRE: Area = 9.9999999999999998e+23 * SQUARE_METRE;

    /// One `SquareGigametre` (Gm²), as an `Area`.
    pub const SQUARE_GIGAMETRE: Area = 1e+18 * SQUARE_METRE;

    /// One `SquareMegametre` (Mm²), as an `Area`.
    pub const SQUARE_MEGAMETRE: Area = 1000000000000.0 * SQUARE_METRE;

    /// One `SquareKilometre` (km²), as an `Area`.
    pub const SQUARE_KILOMETRE: Area = 1000000.0 * SQUARE_METRE;

    /// One `SquareHectometre` (hm²), as an `Area`.
    pub const SQUARE_HECTOMETRE: Area = 10000.0 * SQUARE_METRE;

    /// One `SquareDecametre` (dam²), as an `Area`.
    pub const SQUARE_DECAMETRE: Area = 100.0 * SQUARE_METRE;

    /// One `SquareMetre` (m²), as an `Area`.
    pub const SQUARE_METRE: Area = METRE * METRE;

    /// One `SquareDecimetre` (dm²), as an `Area`.
    pub const SQUARE_DECIMETRE: Area = 0.010000000000000002 * SQUARE_METRE;

    /// One `SquareCentimetre` (cm²), as an `Area`.
    pub const SQUARE_CENTIMETRE: Area = 0.0001 * SQUARE_METRE;

    /// One `SquareMillimetre` (mm²), as an `Area`.
    pub const SQUARE_MILLIMETRE: Area = 9.9999999999999995e-07 * SQUARE_METRE;

    /// One `SquareMicrometre` (µm²), as an `Area`.
    pub const SQUARE_MICROMETRE: Area = 9.9999999999999998e-13 * SQUARE_METRE;

    /// One `SquareNanometre` (nm²), as an `Area`.
    pub const SQUARE_NANOMETRE: Area = 1.0000000000000001e-18 * SQUARE_METRE;

    /// One `SquarePicometre` (pm²), as an `Area`.
    pub const SQUARE_PICOMETRE: Area = 9.9999999999999992e-25 * SQUARE_METRE;

    /// One `SquareFemtometre` (fm²), as an `Area`.
    pub const SQUARE_FEMTOMETRE: Area = 1.0000000000000001e-30 * SQUARE_METRE;

    /// One `SquareAttometre` (am²), as an `Area`.
    pub const SQUARE_ATTOMETRE: Area = 1.0000000000000001e-36 * SQUARE_METRE;

    /// One `SquareZeptometre` (zm²), as an `Area`.
    pub const SQUARE_ZEPTOMETRE: Area = 9.9999999999999988e-43 * SQUARE_METRE;

    /// One `SquareYoctometre` (ym²), as an `Area`.
    pub const SQUARE_YOCTOMETRE: Area = 9.9999999999999982e-49 * SQUARE_METRE;

    /// One `Acre` (ac), as an `Area`.
    pub const ACRE: Area = (43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0)) * SQUARE_METRE;

    /// One `Are` (a), as an `Area`.
    pub const ARE: Area = 100.0 * SQUARE_METRE;

    /// One `Barn` (b), as an `Area`.
    pub const BARN: Area = 9.9999999999999997e-29 * SQUARE_METRE;

    /// One `CircularMil` (cmil), as an `Area`.
    pub const CIRCULAR_MIL: Area =
        (core::f64::consts::PI / 4.0 * 0.0000254 * 0.0000254) * SQUARE_METRE;

    /// One `Hectare` (ha), as an `Area`.
    pub const HECTARE: Area = 10000.0 * SQUARE_METRE;

    /// One `SquareFoot` (ft²), as an `Area`.
    pub const SQUARE_FOOT: Area = 0.092903040000000006 * SQUARE_METRE;

    /// One `SquareInch` (in²), as an `Area`.
    pub const SQUARE_INCH: Area = 0.00064515999999999998 * SQUARE_METRE;

    /// One `SquareMile` (mi²), as an `Area`.
    pub const SQUARE_MILE: Area = 2589988.1103360001 * SQUARE_METRE;

    /// One `SquareYard` (yd²), as an `Area`.
    pub const SQUARE_YARD: Area = 0.83612735999999999 * SQUARE_METRE;

    /// One `Saltus` (saltus), as an `Area`.
    pub const SALTUS: Area = 2018672.6399999999 * SQUARE_METRE;

    /// One `Centuria` (centuria), as an `Area`.
    pub const CENTURIA: Area = 504668.15999999997 * SQUARE_METRE;

    /// One `Heredium` (heredium), as an `Area`.
    pub const HEREDIUM: Area = 5046.6815999999999 * SQUARE_METRE;

    /// One `Jugerum` (jugerum), as an `Area`.
    pub const JUGERUM: Area = 2523.3407999999999 * SQUARE_METRE;

    /// One `Deunx` (deunx), as an `Area`.
    pub const AREA_DEUNX: Area = 2313.0623999999998 * SQUARE_METRE;

    /// One `Dextans` (dextans), as an `Area`.
    pub const AREA_DEXTANS: Area = 2102.7840000000001 * SQUARE_METRE;

    /// One `Dodrans` (dodrans), as an `Area`.
    pub const AREA_DODRANS: Area = 1892.5056 * SQUARE_METRE;

    /// One `Bes` (bes), as an `Area`.
    pub const AREA_BES: Area = 1682.2272 * SQUARE_METRE;

    /// One `Septunx` (septunx), as an `Area`.
    pub const AREA_SEPTUNX: Area = 1471.9487999999999 * SQUARE_METRE;

    /// One `Semis` (semis), as an `Area`.
    pub const AREA_SEMIS: Area = 1261.6704 * SQUARE_METRE;

    /// One `ActusQuadratus` (actus quadratus), as an `Area`.
    pub const ACTUS_QUADRATUS: Area = 1261.6704 * SQUARE_METRE;

    /// One `Quincunx` (quincunx), as an `Area`.
    pub const AREA_QUINCUNX: Area = 1051.3920000000001 * SQUARE_METRE;

    /// One `Triens` (triens), as an `Area`.
    pub const AREA_TRIENS: Area = 841.11360000000002 * SQUARE_METRE;

    /// One `Quadrans` (quadrans), as an `Area`.
    pub const QUADRANS: Area = 630.83519999999999 * SQUARE_METRE;

    /// One `Sextans` (sextans), as an `Area`.
    pub const AREA_SEXTANS: Area = 420.55680000000001 * SQUARE_METRE;

    /// One `Clima` (clima), as an `Area`.
    pub const CLIMA: Area = 315.41759999999999 * SQUARE_METRE;

    /// One `Uncia` (uncia), as an `Area`.
    pub const AREA_UNCIA: Area = 210.2784 * SQUARE_METRE;

    /// One `Semiuncia` (semiuncia), as an `Area`.
    pub const SEMIUNCIA: Area = 105.1392 * SQUARE_METRE;

    /// One `Sicilicus` (sicilicus), as an `Area`.
    pub const AREA_SICILICUS: Area = 52.569600000000001 * SQUARE_METRE;

    /// One `ActusSimplex` (actus simplex), as an `Area`.
    pub const ACTUS_SIMPLEX: Area = 42.055680000000002 * SQUARE_METRE;

    /// One `Sextula` (sextula), as an `Area`.
    pub const AREA_SEXTULA: Area = 35.046399999999998 * SQUARE_METRE;

    /// One `DuoScrupulum` (duo scrupulum), as an `Area`.
    pub const DUO_SCRUPULUM: Area = 17.523199999999999 * SQUARE_METRE;

    /// One `Scrupulum` (scrupulum), as an `Area`.
    pub const AREA_SCRUPULUM: Area = 8.7615999999999996 * SQUARE_METRE;

    /// One `DimidiumScrupulum` (dimidium scrupulum), as an `Area`.
    pub const DIMIDIUM_SCRUPULUM: Area = 4.3807999999999998 * SQUARE_METRE;

    /// One `PesQuadratus` (pes quadratus), as an `Area`.
    pub const PES_QUADRATUS: Area = 0.087615999999999999 * SQUARE_METRE;

    /// One `CubicMetre` (m³), as a `Volume`.
    pub const CUBIC_METRE: Volume = METRE * METRE * METRE;

    /// One `AcreFoot` (ac · ft), as a `Volume`.
    pub const ACRE_FOOT: Volume =
        (43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0) * (1200.0 / 3937.0)) * CUBIC_METRE;

    /// One `Barrel` (bbl), as a `Volume`.
    pub const BARREL: Volume = 0.158987294928 * CUBIC_METRE;

    /// One `Bushel` (bu), as a `Volume`.
    pub const BUSHEL: Volume = 0.03523907016688 * CUBIC_METRE;

    /// One `Cord` (cords), as a `Volume`.
    pub const CORD: Volume = (128.0 * 0.3048 * 0.3048 * 0.3048) * CUBIC_METRE;

    /// One `CubicFoot` (ft³), as a `Volume`.
    pub const CUBIC_FOOT: Volume = (0.3048 * 0.3048 * 0.3048) * CUBIC_METRE;

    /// One `CubicInch` (in³), as a `Volume`.
    pub const CUBIC_INCH: Volume = (0.0254 * 0.0254 * 0.0254) * CUBIC_METRE;

    /// One `CubicMile` (mi³), as a `Volume`.
    pub const CUBIC_MILE: Volume = (1609.344 * 1609.344 * 1609.344) * CUBIC_METRE;

    /// One `CubicYard` (yd³), as a `Volume`.
    pub const CUBIC_YARD: Volume = (0.9144 * 0.9144 * 0.9144) * CUBIC_METRE;

    /// One `Cup` (cup), as a `Volume`.
    pub const CUP: Volume = 0.0002365882365 * CUBIC_METRE;

    /// One `FluidOunce` (fl oz), as a `Volume`.
    pub const FLUID_OUNCE: Volume = 0.0000295735295625 * CUBIC_METRE;

    /// One `FluidOunceImperial` (fl oz (UK)), as a `Volume`.
    pub const FLUID_OUNCE_IMPERIAL: Volume = 0.0000284130625 * CUBIC_METRE;

    /// One `GallonImperial` (gal (UK)), as a `Volume`.
    pub const GALLON_IMPERIAL: Volume = 0.00454609 * CUBIC_METRE;

    /// One `Gallon` (gal), as a `Volume`.
    pub const GALLON: Volume = 0.003785411784 * CUBIC_METRE;

    /// One `GillImperial` (gi (UK)), as a `Volume`.
    pub const GILL_IMPERIAL: Volume = 0.0001420653125 * CUBIC_METRE;

    /// One `Gill` (gi), as a `Volume`.
    pub const GILL: Volume = 0.00011829411825 * CUBIC_METRE;

    /// One `Yottalitre` (YL), as a `Volume`.
    pub const YOTTALITRE: Volume = 1e+21 * CUBIC_METRE;

    /// One `Zettalitre` (ZL), as a `Volume`.
    pub const ZETTALITRE: Volume = 1e+18 * CUBIC_METRE;

    /// One `Exalitre` (EL), as a `Volume`.
    pub const EXALITRE: Volume = 1000000000000000.0 * CUBIC_METRE;

    /// One `Petalitre` (PL), as a `Volume`.
    pub const PETALITRE: Volume = 1000000000000.0 * CUBIC_METRE;

    /// One `Teralitre` (TL), as a `Volume`.
    pub const TERALITRE: Volume = 1000000000.0 * CUBIC_METRE;

    /// One `Gigalitre` (GL), as a `Volume`.
    pub const GIGALITRE: Volume = 1000000.0 * CUBIC_METRE;

    /// One `Megalitre` (ML), as a `Volume`.
    pub const MEGALITRE: Volume = 1000.0 * CUBIC_METRE;

    /// One `Kilolitre` (kL), as a `Volume`.
    pub const KILOLITRE: Volume = CUBIC_METRE;

    /// One `Hectolitre` (hL), as a `Volume`.
    pub const HECTOLITRE: Volume = 0.10000000000000001 * CUBIC_METRE;

    /// One `Decalitre` (daL), as a `Volume`.
    pub const DECALITRE: Volume = 0.01 * CUBIC_METRE;

    /// One `Litre` (L), as a `Volume`.
    pub const LITRE: Volume = 0.001 * CUBIC_METRE;

    /// One `Decilitre` (dL), as a `Volume`.
    pub const DECILITRE: Volume = 0.0001 * CUBIC_METRE;

    /// One `Centilitre` (cL), as a `Volume`.
    pub const CENTILITRE: Volume = 1.0000000000000001e-05 * CUBIC_METRE;

    /// One `Millilitre` (mL), as a `Volume`.
    pub const MILLILITRE: Volume = 9.9999999999999995e-07 * CUBIC_METRE;

    /// One `Microlitre` (µL), as a `Volume`.
    pub const MICROLITRE: Volume = 1.0000000000000001e-09 * CUBIC_METRE;

    /// One `Nanolitre` (nL), as a `Volume`.
    pub const NANOLITRE: Volume = 1.0000000000000002e-12 * CUBIC_METRE;

    /// One `Picolitre` (pL), as a `Volume`.
    pub const PICOLITRE: Volume = 1.0000000000000001e-15 * CUBIC_METRE;

    /// One `Femtolitre` (fL), as a `Volume`.
    pub const FEMTOLITRE: Volume = 1.0000000000000001e-18 * CUBIC_METRE;

    /// One `Attolitre` (aL), as a `Volume`.
    pub const ATTOLITRE: Volume = 1.0000000000000001e-21 * CUBIC_METRE;

    /// One `Zeptolitre` (zL), as a `Volume`.
    pub const ZEPTOLITRE: Volume = 9.9999999999999992e-25 * CUBIC_METRE;

    /// One `Yoctolitre` (yL), as a `Volume`.
    pub const YOCTOLITRE: Volume = 9.9999999999999986e-28 * CUBIC_METRE;

    /// One `Peck` (pk), as a `Volume`.
    pub const PECK: Volume = 0.00880976754172 * CUBIC_METRE;

    /// One `PintDry` (dry pt), as a `Volume`.
    pub const PINT_DRY: Volume = 0.0005506104713575 * CUBIC_METRE;

    /// One `PintLiquid` (liq pt), as a `Volume`.
    pub const PINT_LIQUID: Volume = 0.000473176473 * CUBIC_METRE;

    /// One `QuartDry` (dry qt), as a `Volume`.
    pub const QUART_DRY: Volume = 0.001101220942715 * CUBIC_METRE;

    /// One `QuartLiquid` (liq qt), as a `Volume`.
    pub const QUART_LIQUID: Volume = 0.000946352946 * CUBIC_METRE;

    /// One `Stere` (st), as a `Volume`.
    pub const STERE: Volume = CUBIC_METRE;

    /// One `Tablespoon` (tbsp), as a `Volume`.
    pub const TABLESPOON: Volume = 0.00001478676478125 * CUBIC_METRE;

    /// One `Teaspoon` (tsp), as a `Volume`.
    pub const TEASPOON: Volume = 0.00000492892159375 * CUBIC_METRE;

    /// One `RegisterTon` (RT), as a `Volume`.
    pub const REGISTER_TON: Volume = (100.0 * 0.3048 * 0.3048 * 0.3048) * CUBIC_METRE;

    /// One `Culeus` (culeus), as a `Volume`.
    pub const CULEUS: Volume = 0.51868672000000005 * CUBIC_METRE;

    /// One `AmphoraQuadrantal` (amphora quadrantal), as a `Volume`.
    pub const AMPHORA_QUADRANTAL: Volume = 0.025934335999999999 * CUBIC_METRE;

    /// One `Urna` (urna), as a `Volume`.
    pub const URNA: Volume = 0.012967167999999999 * CUBIC_METRE;

    /// One `ModiusCastrensis` (modius castrensis), as a `Volume`.
    pub const MODIUS_CASTRENSIS: Volume = 0.012967167999999999 * CUBIC_METRE;

    /// One `Modius` (modius), as a `Volume`.
    pub const MODIUS: Volume = 0.0086447786666666599 * CUBIC_METRE;

    /// One `Semimodius` (semimodius), as a `Volume`.
    pub const SEMIMODIUS: Volume = 0.00432238933333333 * CUBIC_METRE;

    /// One `Congius` (congius), as a `Volume`.
    pub const CONGIUS: Volume = 0.0032417919999999999 * CUBIC_METRE;

    /// One `Sextarius` (sextarius), as a `Volume`.
    pub const SEXTARIUS: Volume = 0.000540298666666667 * CUBIC_METRE;

    /// One `Hemina` (hemina), as a `Volume`.
    pub const HEMINA: Volume = 0.00027014933333333301 * CUBIC_METRE;

    /// One `Quartarius` (quartarius), as a `Volume`.
    pub const QUARTARIUS: Volume = 0.00013507466666666699 * CUBIC_METRE;

    /// One `Acetabulum` (acetabulum), as a `Volume`.
    pub const ACETABULUM: Volume = 6.7537333333333294e-05 * CUBIC_METRE;

    /// One `Cyathus` (cyathus), as a `Volume`.
    pub const CYATHUS: Volume = 4.5024888888888901e-05 * CUBIC_METRE;

    /// One `Ligula` (ligula), as a `Volume`.
    pub const LIGULA: Volume = 1.1256222222222222e-05 * CUBIC_METRE;

    /// One `YottametrePerSecond` (Ym/s), as a `Velocity`.
    pub const YOTTAMETRE_PER_SECOND: Velocity = 9.9999999999999998e+23 * METRE_PER_SECOND;

    /// One `ZettametrePerSecond` (Zm/s), as a `Velocity`.
    pub const ZETTAMETRE_PER_SECOND: Velocity = 1e+21 * METRE_PER_SECOND;

    /// One `ExametrePerSecond` (Em/s), as a `Velocity`.
    pub const EXAMETRE_PER_SECOND: Velocity = 1e+18 * METRE_PER_SECOND;

    /// One `PetametrePerSecond` (Pm/s), as a `Velocity`.
    pub const PETAMETRE_PER_SECOND: Velocity = 1000000000000000.0 * METRE_PER_SECOND;

    /// One `TerametrePerSecond` (Tm/s), as a `Velocity`.
    pub const TERAMETRE_PER_SECOND: Velocity = 1000000000000.0 * METRE_PER_SECOND;

    /// One `GigametrePerSecond` (Gm/s), as a `Velocity`.
    pub const GIGAMETRE_PER_SECOND: Velocity = 1000000000.0 * METRE_PER_SECOND;

    /// One `MegametrePerSecond` (Mm/s), as a `Velocity`.
    pub const MEGAMETRE_PER_SECOND: Velocity = 1000000.0 * METRE_PER_SECOND;

    /// One `KilometrePerSecond` (km/s), as a `Velocity`.
    pub const KILOMETRE_PER_SECOND: Velocity = 1000.0 * METRE_PER_SECOND;

    /// One `HectometrePerSecond` (hm/s), as a `Velocity`.
    pub const HECTOMETRE_PER_SECOND: Velocity = 100.0 * METRE_PER_SECOND;

    /// One `DecametrePerSecond` (dam/s), as a `Velocity`.
    pub const DECAMETRE_PER_SECOND: Velocity = 10.0 * METRE_PER_SECOND;

    /// One `MetrePerSecond` (m/s), as a `Velocity`.
    pub const METRE_PER_SECOND: Velocity = METRE / SECOND;

    /// One `DecimetrePerSecond` (dm/s), as a `Velocity`.
    pub const DECIMETRE_PER_SECOND: Velocity = 0.10000000000000001 * METRE_PER_SECOND;

    /// One `CentimetrePerSecond` (cm/s), as a `Velocity`.
    pub const CENTIMETRE_PER_SECOND: Velocity = 0.01 * METRE_PER_SECOND;

    /// One `MillimetrePerSecond` (mm/s), as a `Velocity`.
    pub const MILLIMETRE_PER_SECOND: Velocity = 0.001 * METRE_PER_SECOND;

    /// One `MicrometrePerSecond` (µm/s), as a `Velocity`.
    pub const MICROMETRE_PER_SECOND: Velocity = 9.9999999999999995e-07 * METRE_PER_SECOND;

    /// One `NanometrePerSecond` (nm/s), as a `Velocity`.
    pub const NANOMETRE_PER_SECOND: Velocity = 1.0000000000000001e-09 * METRE_PER_SECOND;

    /// One `PicometrePerSecond` (pm/s), as a `Velocity`.
    pub const PICOMETRE_PER_SECOND: Velocity = 9.9999999999999998e-13 * METRE_PER_SECOND;

    /// One `FemtometrePerSecond` (fm/s), as a `Velocity`.
    pub const FEMTOMETRE_PER_SECOND: Velocity = 1.0000000000000001e-15 * METRE_PER_SECOND;

    /// One `AttometrePerSecond` (am/s), as a `Velocity`.
    pub const ATTOMETRE_PER_SECOND: Velocity = 1.0000000000000001e-18 * METRE_PER_SECOND;

    /// One `ZeptometrePerSecond` (zm/s), as a `Velocity`.
    pub const ZEPTOMETRE_PER_SECOND: Velocity = 9.9999999999999991e-22 * METRE_PER_SECOND;

    /// One `YoctometrePerSecond` (ym/s), as a `Velocity`.
    pub const YOCTOMETRE_PER_SECOND: Velocity = 9.9999999999999992e-25 * METRE_PER_SECOND;

    /// One `FootPerHour` (ft/h), as a `Velocity`.
    pub const FOOT_PER_HOUR: Velocity = 8.4666666666666674e-05 * METRE_PER_SECOND;

    /// One `FootPerMinute` (ft/min), as a `Velocity`.
    pub const FOOT_PER_MINUTE: Velocity = 0.0050800000000000003 * METRE_PER_SECOND;

    /// One `FootPerSecond` (ft/s), as a `Velocity`.
    pub const FOOT_PER_SECOND: Velocity = 0.30480000000000002 * METRE_PER_SECOND;

    /// One `InchPerSecond` (in/s), as a `Velocity`.
    pub const INCH_PER_SECOND: Velocity = 0.025399999999999999 * METRE_PER_SECOND;

    /// One `InchPerMinute` (in/min), as a `Velocity`.
    pub const INCH_PER_MINUTE: Velocity = 0.00042333333333333334 * METRE_PER_SECOND;

    /// One `KilometrePerHour` (km/h), as a `Velocity`.
    pub const KILOMETRE_PER_HOUR: Velocity = 0.27777777777777779 * METRE_PER_SECOND;

    /// One `Knot` (kn), as a `Velocity`.
    pub const KNOT: Velocity = 0.51444444444444448 * METRE_PER_SECOND;

    /// One `MilePerHour` (mi/h), as a `Velocity`.
    pub const MILE_PER_HOUR: Velocity = 0.44703999999999999 * METRE_PER_SECOND;

    /// One `MilePerMinute` (mi/min), as a `Velocity`.
    pub const MILE_PER_MINUTE: Velocity = 26.822399999999998 * METRE_PER_SECOND;

    /// One `MilePerSecond` (mi/s), as a `Velocity`.
    pub const MILE_PER_SECOND: Velocity = 1609.3440000000001 * METRE_PER_SECOND;

    /// One `MillimetrePerMinute` (mm/min), as a `Velocity`.
    pub const MILLIMETRE_PER_MINUTE: Velocity = 1.6666666666666667e-05 * METRE_PER_SECOND;

    /// One `AtomicUnitOfVelocity` (a₀ · Eₕ/ħ), as a `Velocity`.
    pub const ATOMIC_UNIT_OF_VELOCITY: Velocity = 2187691.26364 * METRE_PER_SECOND;

    /// One `NaturalUnitOfVelocity` (c), as a `Velocity`.
    pub const NATURAL_UNIT_OF_VELOCITY: Velocity = 299792458.0 * METRE_PER_SECOND;

    /// One `SpeedOfLight` (c), as a `Velocity`.
    pub const SPEED_OF_LIGHT: Velocity = 299792458.0 * METRE_PER_SECOND;

    /// One `YottametrePerSecondSquared` (Ym/s²), as an `Acceleration`.
    pub const YOTTAMETRE_PER_SECOND_SQUARED: Acceleration =
        9.9999999999999998e+23 * METRE_PER_SECOND_SQUARED;

    /// One `ZettametrePerSecondSquared` (Zm/s²), as an `Acceleration`.
    pub const ZETTAMETRE_PER_SECOND_SQUARED: Acceleration = 1e+21 * METRE_PER_SECOND_SQUARED;

    /// One `ExametrePerSecondSquared` (Em/s²), as an `Acceleration`.
    pub const EXAMETRE_PER_SECOND_SQUARED: Acceleration = 1e+18 * METRE_PER_SECOND_SQUARED;

    /// One `PetametrePerSecondSquared` (Pm/s²), as an `Acceleration`.
    pub const PETAMETRE_PER_SECOND_SQUARED: Acceleration =
        1000000000000000.0 * METRE_PER_SECOND_SQUARED;

    /// One `TerametrePerSecondSquared` (Tm/s²), as an `Acceleration`.
    pub const TERAMETRE_PER_SECOND_SQUARED: Acceleration =
        1000000000000.0 * METRE_PER_SECOND_SQUARED;

    /// One `GigametrePerSecondSquared` (Gm/s²), as an `Acceleration`.
    pub const GIGAMETRE_PER_SECOND_SQUARED: Acceleration = 1000000000.0 * METRE_PER_SECOND_SQUARED;

    /// One `MegametrePerSecondSquared` (Mm/s²), as an `Acceleration`.
    pub const MEGAMETRE_PER_SECOND_SQUARED: Acceleration = 1000000.0 * METRE_PER_SECOND_SQUARED;

    /// One `KilometrePerSecondSquared` (km/s²), as an `Acceleration`.
    pub const KILOMETRE_PER_SECOND_SQUARED: Acceleration = 1000.0 * METRE_PER_SECOND_SQUARED;

    /// One `HectometrePerSecondSquared` (hm/s²), as an `Acceleration`.
    pub const HECTOMETRE_PER_SECOND_SQUARED: Acceleration = 100.0 * METRE_PER_SECOND_SQUARED;

    /// One `DecametrePerSecondSquared` (dam/s²), as an `Acceleration`.
    pub const DECAMETRE_PER_SECOND_SQUARED: Acceleration = 10.0 * METRE_PER_SECOND_SQUARED;

    /// One `MetrePerSecondSquared` (m/s²), as an `Acceleration`.
    pub const METRE_PER_SECOND_SQUARED: Acceleration = METRE / (SECOND * SECOND);

    /// One `DecimetrePerSecondSquared` (dm/s²), as an `Acceleration`.
    pub const DECIMETRE_PER_SECOND_SQUARED: Acceleration =
        0.10000000000000001 * METRE_PER_SECOND_SQUARED;

    /// One `CentimetrePerSecondSquared` (cm/s²), as an `Acceleration`.
    pub const CENTIMETRE_PER_SECOND_SQUARED: Acceleration = 0.01 * METRE_PER_SECOND_SQUARED;

    /// One `MillimetrePerSecondSquared` (mm/s²), as an `Acceleration`.
    pub const MILLIMETRE_PER_SECOND_SQUARED: Acceleration = 0.001 * METRE_PER_SECOND_SQUARED;

    /// One `MicrometrePerSecondSquared` (µm/s²), as an `Acceleration`.
    pub const MICROMETRE_PER_SECOND_SQUARED: Acceleration =
        9.9999999999999995e-07 * METRE_PER_SECOND_SQUARED;

    /// One `NanometrePerSecondSquared` (nm/s²), as an `Acceleration`.
    pub const NANOMETRE_PER_SECOND_SQUARED: Acceleration =
        1.0000000000000001e-09 * METRE_PER_SECOND_SQUARED;

    /// One `PicometrePerSecondSquared` (pm/s²), as an `Acceleration`.
    pub const PICOMETRE_PER_SECOND_SQUARED: Acceleration =
        9.9999999999999998e-13 * METRE_PER_SECOND_SQUARED;

    /// One `FemtometrePerSecondSquared` (fm/s²), as an `Acceleration`.
    pub const FEMTOMETRE_PER_SECOND_SQUARED: Acceleration =
        1.0000000000000001e-15 * METRE_PER_SECOND_SQUARED;

    /// One `AttometrePerSecondSquared` (am/s²), as an `Acceleration`.
    pub const ATTOMETRE_PER_SECOND_SQUARED: Acceleration =
        1.0000000000000001e-18 * METRE_PER_SECOND_SQUARED;

    /// One `ZeptometrePerSecondSquared` (zm/s²), as an `Acceleration`.
    pub const ZEPTOMETRE_PER_SECOND_SQUARED: Acceleration =
        9.9999999999999991e-22 * METRE_PER_SECOND_SQUARED;

    /// One `YoctometrePerSecondSquared` (ym/s²), as an `Acceleration`.
    pub const YOCTOMETRE_PER_SECOND_SQUARED: Acceleration =
        9.9999999999999992e-25 * METRE_PER_SECOND_SQUARED;

    /// One `FootPerSecondSquared` (ft/s²), as an `Acceleration`.
    pub const FOOT_PER_SECOND_SQUARED: Acceleration =
        0.30480000000000002 * METRE_PER_SECOND_SQUARED;

    /// One `Galileo` (Gal), as an `Acceleration`.
    pub const GALILEO: Acceleration = 0.01 * METRE_PER_SECOND_SQUARED;

    /// One `InchPerSecondSquared` (in/s²), as an `Acceleration`.
    pub const INCH_PER_SECOND_SQUARED: Acceleration =
        0.025399999999999999 * METRE_PER_SECOND_SQUARED;

    /// One `MillimetrePerMinuteSquared` (mm/min²), as an `Acceleration`.
    pub const MILLIMETRE_PER_MINUTE_SQUARED: Acceleration =
        2.7777777777777776e-07 * METRE_PER_SECOND_SQUARED;

    /// One `StandardGravity` (g₀), as an `Acceleration`.
    pub const STANDARD_GRAVITY: Acceleration = 9.80665 * METRE_PER_SECOND_SQUARED;

    /// One `YottametrePerSecondCubed` (Ym/s³), as a `Jerk`.
    pub const YOTTAMETRE_PER_SECOND_CUBED: Jerk = 9.9999999999999998e+23 * METRE_PER_SECOND_CUBED;

    /// One `ZettametrePerSecondCubed` (Zm/s³), as a `Jerk`.
    pub const ZETTAMETRE_PER_SECOND_CUBED: Jerk = 1e+21 * METRE_PER_SECOND_CUBED;

    /// One `ExametrePerSecondCubed` (Em/s³), as a `Jerk`.
    pub const EXAMETRE_PER_SECOND_CUBED: Jerk = 1e+18 * METRE_PER_SECOND_CUBED;

    /// One `PetametrePerSecondCubed` (Pm/s³), as a `Jerk`.
    pub const PETAMETRE_PER_SECOND_CUBED: Jerk = 1000000000000000.0 * METRE_PER_SECOND_CUBED;

    /// One `TerametrePerSecondCubed` (Tm/s³), as a `Jerk`.
    pub const TERAMETRE_PER_SECOND_CUBED: Jerk = 1000000000000.0 * METRE_PER_SECOND_CUBED;

    /// One `GigametrePerSecondCubed` (Gm/s³), as a `Jerk`.
    pub const GIGAMETRE_PER_SECOND_CUBED: Jerk = 1000000000.0 * METRE_PER_SECOND_CUBED;

    /// One `MegametrePerSecondCubed` (Mm/s³), as a `Jerk`.
    pub const MEGAMETRE_PER_SECOND_CUBED: Jerk = 1000000.0 * METRE_PER_SECOND_CUBED;

    /// One `KilometrePerSecondCubed` (km/s³), as a `Jerk`.
    pub const KILOMETRE_PER_SECOND_CUBED: Jerk = 1000.0 * METRE_PER_SECOND_CUBED;

    /// One `HectometrePerSecondCubed` (hm/s³), as a `Jerk`.
    pub const HECTOMETRE_PER_SECOND_CUBED: Jerk = 100.0 * METRE_PER_SECOND_CUBED;

    /// One `DecametrePerSecondCubed` (dam/s³), as a `Jerk`.
    pub const DECAMETRE_PER_SECOND_CUBED: Jerk = 10.0 * METRE_PER_SECOND_CUBED;

    /// One `MetrePerSecondCubed` (m/s³), as a `Jerk`.
    pub const METRE_PER_SECOND_CUBED: Jerk = METRE / (SECOND * SECOND * SECOND);

    /// One `DecimetrePerSecondCubed` (dm/s³), as a `Jerk`.
    pub const DECIMETRE_PER_SECOND_CUBED: Jerk = 0.10000000000000001 * METRE_PER_SECOND_CUBED;

    /// One `CentimetrePerSecondCubed` (cm/s³), as a `Jerk`.
    pub const CENTIMETRE_PER_SECOND_CUBED: Jerk = 0.01 * METRE_PER_SECOND_CUBED;

    /// One `MillimetrePerSecondCubed` (mm/s³), as a `Jerk`.
    pub const MILLIMETRE_PER_SECOND_CUBED: Jerk = 0.001 * METRE_PER_SECOND_CUBED;

    /// One `MicrometrePerSecondCubed` (µm/s³), as a `Jerk`.
    pub const MICROMETRE_PER_SECOND_CUBED: Jerk = 9.9999999999999995e-07 * METRE_PER_SECOND_CUBED;

    /// One `NanometrePerSecondCubed` (nm/s³), as a `Jerk`.
    pub const NANOMETRE_PER_SECOND_CUBED: Jerk = 1.0000000000000001e-09 * METRE_PER_SECOND_CUBED;

    /// One `PicometrePerSecondCubed` (pm/s³), as a `Jerk`.
    pub const PICOMETRE_PER_SECOND_CUBED: Jerk = 9.9999999999999998e-13 * METRE_PER_SECOND_CUBED;

    /// One `FemtometrePerSecondCubed` (fm/s³), as a `Jerk`.
    pub const FEMTOMETRE_PER_SECOND_CUBED: Jerk = 1.0000000000000001e-15 * METRE_PER_SECOND_CUBED;

    /// One `AttometrePerSecondCubed` (am/s³), as a `Jerk`.
    pub const ATTOMETRE_PER_SECOND_CUBED: Jerk = 1.0000000000000001e-18 * METRE_PER_SECOND_CUBED;

    /// One `ZeptometrePerSecondCubed` (zm/s³), as a `Jerk`.
    pub const ZEPTOMETRE_PER_SECOND_CUBED: Jerk = 9.9999999999999991e-22 * METRE_PER_SECOND_CUBED;

    /// One `YoctometrePerSecondCubed` (ym/s³), as a `Jerk`.
    pub const YOCTOMETRE_PER_SECOND_CUBED: Jerk = 9.9999999999999992e-25 * METRE_PER_SECOND_CUBED;

    /// One `FootPerSecondCubed` (ft/s³), as a `Jerk`.
    pub const FOOT_PER_SECOND_CUBED: Jerk = 0.30480000000000002 * METRE_PER_SECOND_CUBED;

    /// One `InchPerSecondCubed` (in/s³), as a `Jerk`.
    pub const INCH_PER_SECOND_CUBED: Jerk = 0.025399999999999999 * METRE_PER_SECOND_CUBED;

    /// One `KilometrePerMinuteCubed` (km/min³), as a `Jerk`.
    pub const KILOMETRE_PER_MINUTE_CUBED: Jerk = 0.0046296296296296294 * METRE_PER_SECOND_CUBED;

    /// One `YottametreSecond` (Ym · s), as an `Absement`.
    pub const YOTTAMETRE_SECOND: Absement = 9.9999999999999998e+23 * METRE_SECOND;

    /// One `ZettametreSecond` (Zm · s), as an `Absement`.
    pub const ZETTAMETRE_SECOND: Absement = 1e+21 * METRE_SECOND;

    /// One `ExametreSecond` (Em · s), as an `Absement`.
    pub const EXAMETRE_SECOND: Absement = 1e+18 * METRE_SECOND;

    /// One `PetametreSecond` (Pm · s), as an `Absement`.
    pub const PETAMETRE_SECOND: Absement = 1000000000000000.0 * METRE_SECOND;

    /// One `TerametreSecond` (Tm · s), as an `Absement`.
    pub const TERAMETRE_SECOND: Absement = 1000000000000.0 * METRE_SECOND;

    /// One `GigametreSecond` (Gm · s), as an `Absement`.
    pub const GIGAMETRE_SECOND: Absement = 1000000000.0 * METRE_SECOND;

    /// One `MegametreSecond` (Mm · s), as an `Absement`.
    pub const MEGAMETRE_SECOND: Absement = 1000000.0 * METRE_SECOND;

    /// One `KilometreSecond` (km · s), as an `Absement`.
    pub const KILOMETRE_SECOND: Absement = 1000.0 * METRE_SECOND;

    /// One `HectometreSecond` (hm · s), as an `Absement`.
    pub const HECTOMETRE_SECOND: Absement = 100.0 * METRE_SECOND;

    /// One `DecametreSecond` (dam · s), as an `Absement`.
    pub const DECAMETRE_SECOND: Absement = 10.0 * METRE_SECOND;

    /// One `MetreSecond` (m · s), as an `Absement`.
    pub const METRE_SECOND: Absement = METRE * SECOND;

    /// One `DecimetreSecond` (dm · s), as an `Absement`.
    pub const DECIMETRE_SECOND: Absement = 0.10000000000000001 * METRE_SECOND;

    /// One `CentimetreSecond` (cm · s), as an `Absement`.
    pub const CENTIMETRE_SECOND: Absement = 0.01 * METRE_SECOND;

    /// One `MillimetreSecond` (mm · s), as an `Absement`.
    pub const MILLIMETRE_SECOND: Absement = 0.001 * METRE_SECOND;

    /// One `MicrometreSecond` (µm · s), as an `Absement`.
    pub const MICROMETRE_SECOND: Absement = 9.9999999999999995e-07 * METRE_SECOND;

    /// One `NanometreSecond` (nm · s), as an `Absement`.
    pub const NANOMETRE_SECOND: Absement = 1.0000000000000001e-09 * METRE_SECOND;

    /// One `PicometreSecond` (pm · s), as an `Absement`.
    pub const PICOMETRE_SECOND: Absement = 9.9999999999999998e-13 * METRE_SECOND;

    /// One `FemtometreSecond` (fm · s), as an `Absement`.
    pub const FEMTOMETRE_SECOND: Absement = 1.0000000000000001e-15 * METRE_SECOND;

    /// One `AttometreSecond` (am · s), as an `Absement`.
    pub const ATTOMETRE_SECOND: Absement = 1.0000000000000001e-18 * METRE_SECOND;

    /// One `ZeptometreSecond` (zm · s), as an `Absement`.
    pub const ZEPTOMETRE_SECOND: Absement = 9.9999999999999991e-22 * METRE_SECOND;

    /// One `YoctometreSecond` (ym · s), as an `Absement`.
    pub const YOCTOMETRE_SECOND: Absement = 9.9999999999999992e-25 * METRE_SECOND;

    /// One `FootSecond` (ft · s), as an `Absement`.
    pub const FOOT_SECOND: Absement = 0.30480000000000002 * METRE_SECOND;

    /// One `InchSecond` (in · s), as an `Absement`.
    pub const INCH_SECOND: Absement = 0.025399999999999999 * METRE_SECOND;

    /// One `KilometreHour` (km · h), as an `Absement`.
    pub const KILOMETRE_HOUR: Absement = 3600000.0 * METRE_SECOND;

    /// One `YottasecondPerMetre` (Ys/m), as an `InverseVelocity`.
    pub const YOTTASECOND_PER_METRE: InverseVelocity = 9.9999999999999998e+23 * SECOND_PER_METRE;

    /// One `ZettasecondPerMetre` (Zs/m), as an `InverseVelocity`.
    pub const ZETTASECOND_PER_METRE: InverseVelocity = 1e+21 * SECOND_PER_METRE;

    /// One `ExasecondPerMetre` (Es/m), as an `InverseVelocity`.
    pub const EXASECOND_PER_METRE: InverseVelocity = 1e+18 * SECOND_PER_METRE;

    /// One `PetasecondPerMetre` (Ps/m), as an `InverseVelocity`.
    pub const PETASECOND_PER_METRE: InverseVelocity = 1000000000000000.0 * SECOND_PER_METRE;

    /// One `TerasecondPerMetre` (Ts/m), as an `InverseVelocity`.
    pub const TERASECOND_PER_METRE: InverseVelocity = 1000000000000.0 * SECOND_PER_METRE;

    /// One `GigasecondPerMetre` (Gs/m), as an `InverseVelocity`.
    pub const GIGASECOND_PER_METRE: InverseVelocity = 1000000000.0 * SECOND_PER_METRE;

    /// One `MegasecondPerMetre` (Ms/m), as an `InverseVelocity`.
    pub const MEGASECOND_PER_METRE: InverseVelocity = 1000000.0 * SECOND_PER_METRE;

    /// One `KilosecondPerMetre` (ks/m), as an `InverseVelocity`.
    pub const KILOSECOND_PER_METRE: InverseVelocity = 1000.0 * SECOND_PER_METRE;

    /// One `HectosecondPerMetre` (hs/m), as an `InverseVelocity`.
    pub const HECTOSECOND_PER_METRE: InverseVelocity = 100.0 * SECOND_PER_METRE;

    /// One `DecasecondPerMetre` (das/m), as an `InverseVelocity`.
    pub const DECASECOND_PER_METRE: InverseVelocity = 10.0 * SECOND_PER_METRE;

    /// One `SecondPerMetre` (s/m), as an `InverseVelocity`.
    pub const SECOND_PER_METRE: InverseVelocity = SECOND / METRE;

    /// One `DecisecondPerMetre` (ds/m), as an `InverseVelocity`.
    pub const DECISECOND_PER_METRE: InverseVelocity = 0.10000000000000001 * SECOND_PER_METRE;

    /// One `CentisecondPerMetre` (cs/m), as an `InverseVelocity`.
    pub const CENTISECOND_PER_METRE: InverseVelocity = 0.01 * SECOND_PER_METRE;

    /// One `MillisecondPerMetre` (ms/m), as an `InverseVelocity`.
    pub const MILLISECOND_PER_METRE: InverseVelocity = 0.001 * SECOND_PER_METRE;

    /// One `MicrosecondPerMetre` (µs/m), as an `InverseVelocity`.
    pub const MICROSECOND_PER_METRE: InverseVelocity = 9.9999999999999995e-07 * SECOND_PER_METRE;

    /// One `NanosecondPerMetre` (ns/m), as an `InverseVelocity`.
    pub const NANOSECOND_PER_METRE: InverseVelocity = 1.0000000000000001e-09 * SECOND_PER_METRE;

    /// One `PicosecondPerMetre` (ps/m), as an `InverseVelocity`.
    pub const PICOSECOND_PER_METRE: InverseVelocity = 9.9999999999999998e-13 * SECOND_PER_METRE;

    /// One `FemtosecondPerMetre` (fs/m), as an `InverseVelocity`.
    pub const FEMTOSECOND_PER_METRE: InverseVelocity = 1.0000000000000001e-15 * SECOND_PER_METRE;

    /// One `AttosecondPerMetre` (as/m), as an `InverseVelocity`.
    pub const ATTOSECOND_PER_METRE: InverseVelocity = 1.0000000000000001e-18 * SECOND_PER_METRE;

    /// One `ZeptosecondPerMetre` (zs/m), as an `InverseVelocity`.
    pub const ZEPTOSECOND_PER_METRE: InverseVelocity = 9.9999999999999991e-22 * SECOND_PER_METRE;

    /// One `YoctosecondPerMetre` (ys/m), as an `InverseVelocity`.
    pub const YOCTOSECOND_PER_METRE: InverseVelocity = 9.9999999999999992e-25 * SECOND_PER_METRE;

    /// One `HourPerFoot` (h/ft), as an `InverseVelocity`.
    pub const HOUR_PER_FOOT: InverseVelocity = 11811.023622047243 * SECOND_PER_METRE;

    /// One `MinutePerFoot` (min/ft), as an `InverseVelocity`.
    pub const MINUTE_PER_FOOT: InverseVelocity = 196.85039370078738 * SECOND_PER_METRE;

    /// One `SecondPerFoot` (s/ft), as an `InverseVelocity`.
    pub const SECOND_PER_FOOT: InverseVelocity = 3.280839895013123 * SECOND_PER_METRE;

    /// One `SecondPerInch` (s/in), as an `InverseVelocity`.
    pub const SECOND_PER_INCH: InverseVelocity = 39.370078740157481 * SECOND_PER_METRE;

    /// One `MinutePerKilometre` (min/km), as an `InverseVelocity`.
    pub const MINUTE_PER_KILOMETRE: InverseVelocity = 0.059999999999999998 * SECOND_PER_METRE;

    /// One `HourPerKilometre` (h/km), as an `InverseVelocity`.
    pub const HOUR_PER_KILOMETRE: InverseVelocity = 3.6000000000000001 * SECOND_PER_METRE;

    /// One `HourPerMile` (h/mi), as an `InverseVelocity`.
    pub const HOUR_PER_MILE: InverseVelocity = 2.236936292054402 * SECOND_PER_METRE;

    /// One `MinutePerMile` (min/mi), as an `InverseVelocity`.
    pub const MINUTE_PER_MILE: InverseVelocity = (60.0 / 1609.344) * SECOND_PER_METRE;

    /// One `SecondPerMile` (s/mi), as an `InverseVelocity`.
    pub const SECOND_PER_MILE: InverseVelocity = 0.00062137119223733392 * SECOND_PER_METRE;

    /// One `Yottanewton` (YN), as a `Force`.
    pub const YOTTANEWTON: Force = 9.9999999999999998e+23 * NEWTON;

    /// One `Zettanewton` (ZN), as a `Force`.
    pub const ZETTANEWTON: Force = 1e+21 * NEWTON;

    /// One `Exanewton` (EN), as a `Force`.
    pub const EXANEWTON: Force = 1e+18 * NEWTON;

    /// One `Petanewton` (PN), as a `Force`.
    pub const PETANEWTON: Force = 1000000000000000.0 * NEWTON;

    /// One `Teranewton` (TN), as a `Force`.
    pub const TERANEWTON: Force = 1000000000000.0 * NEWTON;

    /// One `Giganewton` (GN), as a `Force`.
    pub const GIGANEWTON: Force = 1000000000.0 * NEWTON;

    /// One `Meganewton` (MN), as a `Force`.
    pub const MEGANEWTON: Force = 1000000.0 * NEWTON;

    /// One `Kilonewton` (kN), as a `Force`.
    pub const KILONEWTON: Force = 1000.0 * NEWTON;

    /// One `Hectonewton` (hN), as a `Force`.
    pub const HECTONEWTON: Force = 100.0 * NEWTON;

    /// One `Decanewton` (daN), as a `Force`.
    pub const DECANEWTON: Force = 10.0 * NEWTON;

    /// One `Newton` (N), as a `Force`.
    pub const NEWTON: Force = METRE * KILOGRAM / (SECOND * SECOND);

    /// One `Decinewton` (dN), as a `Force`.
    pub const DECINEWTON: Force = 0.10000000000000001 * NEWTON;

    /// One `Centinewton` (cN), as a `Force`.
    pub const CENTINEWTON: Force = 0.01 * NEWTON;

    /// One `Millinewton` (mN), as a `Force`.
    pub const MILLINEWTON: Force = 0.001 * NEWTON;

    /// One `Micronewton` (µN), as a `Force`.
    pub const MICRONEWTON: Force = 9.9999999999999995e-07 * NEWTON;

    /// One `Nanonewton` (nN), as a `Force`.
    pub const NANONEWTON: Force = 1.0000000000000001e-09 * NEWTON;

    /// One `Piconewton` (pN), as a `Force`.
    pub const PICONEWTON: Force = 9.9999999999999998e-13 * NEWTON;

    /// One `Femtonewton` (fN), as a `Force`.
    pub const FEMTONEWTON: Force = 1.0000000000000001e-15 * NEWTON;

    /// One `Attonewton` (aN), as a `Force`.
    pub const ATTONEWTON: Force = 1.0000000000000001e-18 * NEWTON;

    /// One `Zeptonewton` (zN), as a `Force`.
    pub const ZEPTONEWTON: Force = 9.9999999999999991e-22 * NEWTON;

    /// One `Yoctonewton` (yN), as a `Force`.
    pub const YOCTONEWTON: Force = 9.9999999999999992e-25 * NEWTON;

    /// One `Dyne` (dyn), as a `Force`.
    pub const DYNE: Force = 1.0000000000000001e-05 * NEWTON;

    /// One `KilogramForce` (kgf), as a `Force`.
    pub const KILOGRAM_FORCE: Force = 9.8066499999999994 * NEWTON;

    /// One `Kip` (kip), as a `Force`.
    pub const KIP: Force = (1000.0 * 0.45359237 * 9.80665) * NEWTON;

    /// One `OunceForce` (ozf), as a `Force`.
    pub const OUNCE_FORCE: Force = (0.45359237 * 9.80665 / 16.0) * NEWTON;

    /// One `Poundal` (pdl), as a `Force`.
    pub const POUNDAL: Force = (0.45359237 * 0.3048) * NEWTON;

    /// One `PoundForce` (lbf), as a `Force`.
    pub const POUND_FORCE: Force = (0.45359237 * 9.80665) * NEWTON;

    /// One `TonForce` (2000 lbf), as a `Force`.
    pub const TON_FORCE: Force = (2000.0 * 0.45359237 * 9.80665) * NEWTON;

    /// One `YottagramMetrePerSecond` (Yg · m/s), as a `Momentum`.
    pub const YOTTAGRAM_METRE_PER_SECOND: Momentum = 1e+21 * KILOGRAM_METRE_PER_SECOND;

    /// One `ZettagramMetrePerSecond` (Zg · m/s), as a `Momentum`.
    pub const ZETTAGRAM_METRE_PER_SECOND: Momentum = 1e+18 * KILOGRAM_METRE_PER_SECOND;

    /// One `ExagramMetrePerSecond` (Eg · m/s), as a `Momentum`.
    pub const EXAGRAM_METRE_PER_SECOND: Momentum = 1000000000000000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `PetagramMetrePerSecond` (Pg · m/s), as a `Momentum`.
    pub const PETAGRAM_METRE_PER_SECOND: Momentum = 1000000000000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `TeragramMetrePerSecond` (Tg · m/s), as a `Momentum`.
    pub const TERAGRAM_METRE_PER_SECOND: Momentum = 1000000000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `GigagramMetrePerSecond` (Gg · m/s), as a `Momentum`.
    pub const GIGAGRAM_METRE_PER_SECOND: Momentum = 1000000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `MegagramMetrePerSecond` (Mg · m/s), as a `Momentum`.
    pub const MEGAGRAM_METRE_PER_SECOND: Momentum = 1000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramMetrePerSecond` (kg · m/s), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_SECOND: Momentum = METRE * KILOGRAM / SECOND;

    /// One `HectogramMetrePerSecond` (hg · m/s), as a `Momentum`.
    pub const HECTOGRAM_METRE_PER_SECOND: Momentum =
        0.10000000000000001 * KILOGRAM_METRE_PER_SECOND;

    /// One `DecagramMetrePerSecond` (dag · m/s), as a `Momentum`.
    pub const DECAGRAM_METRE_PER_SECOND: Momentum = 0.01 * KILOGRAM_METRE_PER_SECOND;

    /// One `GramMetrePerSecond` (g · m/s), as a `Momentum`.
    pub const GRAM_METRE_PER_SECOND: Momentum = 0.001 * KILOGRAM_METRE_PER_SECOND;

    /// One `DecigramMetrePerSecond` (dg · m/s), as a `Momentum`.
    pub const DECIGRAM_METRE_PER_SECOND: Momentum = 0.0001 * KILOGRAM_METRE_PER_SECOND;

    /// One `CentigramMetrePerSecond` (cg · m/s), as a `Momentum`.
    pub const CENTIGRAM_METRE_PER_SECOND: Momentum =
        1.0000000000000001e-05 * KILOGRAM_METRE_PER_SECOND;

    /// One `MilligramMetrePerSecond` (mg · m/s), as a `Momentum`.
    pub const MILLIGRAM_METRE_PER_SECOND: Momentum =
        9.9999999999999995e-07 * KILOGRAM_METRE_PER_SECOND;

    /// One `MicrogramMetrePerSecond` (µg · m/s), as a `Momentum`.
    pub const MICROGRAM_METRE_PER_SECOND: Momentum =
        9.9999999999999986e-10 * KILOGRAM_METRE_PER_SECOND;

    /// One `NanogramMetrePerSecond` (ng · m/s), as a `Momentum`.
    pub const NANOGRAM_METRE_PER_SECOND: Momentum =
        9.9999999999999998e-13 * KILOGRAM_METRE_PER_SECOND;

    /// One `PicogramMetrePerSecond` (pg · m/s), as a `Momentum`.
    pub const PICOGRAM_METRE_PER_SECOND: Momentum =
        1.0000000000000001e-15 * KILOGRAM_METRE_PER_SECOND;

    /// One `FemtogramMetrePerSecond` (fg · m/s), as a `Momentum`.
    pub const FEMTOGRAM_METRE_PER_SECOND: Momentum =
        1.0000000000000001e-18 * KILOGRAM_METRE_PER_SECOND;

    /// One `AttogramMetrePerSecond` (ag · m/s), as a `Momentum`.
    pub const ATTOGRAM_METRE_PER_SECOND: Momentum =
        1.0000000000000001e-21 * KILOGRAM_METRE_PER_SECOND;

    /// One `ZeptogramMetrePerSecond` (zg · m/s), as a `Momentum`.
    pub const ZEPTOGRAM_METRE_PER_SECOND: Momentum =
        9.9999999999999992e-25 * KILOGRAM_METRE_PER_SECOND;

    /// One `YoctogramMetrePerSecond` (yg · m/s), as a `Momentum`.
    pub const YOCTOGRAM_METRE_PER_SECOND: Momentum =
        9.9999999999999986e-28 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramYottametrePerSecond` (kg · Ym/s), as a `Momentum`.
    pub const KILOGRAM_YOTTAMETRE_PER_SECOND: Momentum =
        9.9999999999999998e+23 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramZettametrePerSecond` (kg · Zm/s), as a `Momentum`.
    pub const KILOGRAM_ZETTAMETRE_PER_SECOND: Momentum = 1e+21 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramExametrePerSecond` (kg · Em/s), as a `Momentum`.
    pub const KILOGRAM_EXAMETRE_PER_SECOND: Momentum = 1e+18 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramPetametrePerSecond` (kg · Pm/s), as a `Momentum`.
    pub const KILOGRAM_PETAMETRE_PER_SECOND: Momentum =
        1000000000000000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramTerametrePerSecond` (kg · Tm/s), as a `Momentum`.
    pub const KILOGRAM_TERAMETRE_PER_SECOND: Momentum = 1000000000000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramGigametrePerSecond` (kg · Gm/s), as a `Momentum`.
    pub const KILOGRAM_GIGAMETRE_PER_SECOND: Momentum = 1000000000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramMegametrePerSecond` (kg · Mm/s), as a `Momentum`.
    pub const KILOGRAM_MEGAMETRE_PER_SECOND: Momentum = 1000000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramKilometrePerSecond` (kg · km/s), as a `Momentum`.
    pub const KILOGRAM_KILOMETRE_PER_SECOND: Momentum = 1000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramHectometrePerSecond` (kg · hm/s), as a `Momentum`.
    pub const KILOGRAM_HECTOMETRE_PER_SECOND: Momentum = 100.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramDecametrePerSecond` (kg · dam/s), as a `Momentum`.
    pub const KILOGRAM_DECAMETRE_PER_SECOND: Momentum = 10.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramDecimetrePerSecond` (kg · dm/s), as a `Momentum`.
    pub const KILOGRAM_DECIMETRE_PER_SECOND: Momentum =
        0.10000000000000001 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramCentimetrePerSecond` (kg · cm/s), as a `Momentum`.
    pub const KILOGRAM_CENTIMETRE_PER_SECOND: Momentum = 0.01 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramMillimetrePerSecond` (kg · mm/s), as a `Momentum`.
    pub const KILOGRAM_MILLIMETRE_PER_SECOND: Momentum = 0.001 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramMicrometrePerSecond` (kg · µm/s), as a `Momentum`.
    pub const KILOGRAM_MICROMETRE_PER_SECOND: Momentum =
        9.9999999999999995e-07 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramNanometrePerSecond` (kg · nm/s), as a `Momentum`.
    pub const KILOGRAM_NANOMETRE_PER_SECOND: Momentum =
        1.0000000000000001e-09 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramPicometrePerSecond` (kg · pm/s), as a `Momentum`.
    pub const KILOGRAM_PICOMETRE_PER_SECOND: Momentum =
        9.9999999999999998e-13 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramFemtometrePerSecond` (kg · fm/s), as a `Momentum`.
    pub const KILOGRAM_FEMTOMETRE_PER_SECOND: Momentum =
        1.0000000000000001e-15 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramAttometrePerSecond` (kg · am/s), as a `Momentum`.
    pub const KILOGRAM_ATTOMETRE_PER_SECOND: Momentum =
        1.0000000000000001e-18 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramZeptometrePerSecond` (kg · zm/s), as a `Momentum`.
    pub const KILOGRAM_ZEPTOMETRE_PER_SECOND: Momentum =
        9.9999999999999991e-22 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramYoctometrePerSecond` (kg · ym/s), as a `Momentum`.
    pub const KILOGRAM_YOCTOMETRE_PER_SECOND: Momentum =
        9.9999999999999992e-25 * KILOGRAM_METRE_PER_SECOND;

    /// One `TonMetrePerSecond` (t · m/s), as a `Momentum`.
    pub const TON_METRE_PER_SECOND: Momentum = 1000.0 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramMetrePerMinute` (kg · m/min), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_MINUTE: Momentum =
        0.016666666666666666 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramMetrePerHour` (kg · m/h), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_HOUR: Momentum =
        0.00027777777777777778 * KILOGRAM_METRE_PER_SECOND;

    /// One `KilogramMetrePerDay` (kg · m/d), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_DAY: Momentum = 1.1574074074074073e-05 * KILOGRAM_METRE_PER_SECOND;

    /// One `SlugFootPerSecond` (slug · ft/s), as a `Momentum`.
    pub const SLUG_FOOT_PER_SECOND: Momentum =
        ((0.45359237 * 9.80665 / 0.3048) * 0.3048) * KILOGRAM_METRE_PER_SECOND;

    /// One `SlugInchPerSecond` (slug · in/s), as a `Momentum`.
    pub const SLUG_INCH_PER_SECOND: Momentum =
        ((0.45359237 * 9.80665 / 0.3048) * 0.0254) * KILOGRAM_METRE_PER_SECOND;

    /// One `PoundFootPerSecond` (lb · ft/s), as a `Momentum`.
    pub const POUND_FOOT_PER_SECOND: Momentum = 0.138254954376 * KILOGRAM_METRE_PER_SECOND;

    /// One `PoundInchPerSecond` (lb · in/s), as a `Momentum`.
    pub const POUND_INCH_PER_SECOND: Momentum = 0.011521246198 * KILOGRAM_METRE_PER_SECOND;

    /// One `Yottajoule` (YJ), as an `Energy`.
    pub const YOTTAJOULE: Energy = 9.9999999999999998e+23 * JOULE;

    /// One `Zettajoule` (ZJ), as an `Energy`.
    pub const ZETTAJOULE: Energy = 1e+21 * JOULE;

    /// One `Exajoule` (EJ), as an `Energy`.
    pub const EXAJOULE: Energy = 1e+18 * JOULE;

    /// One `Petajoule` (PJ), as an `Energy`.
    pub const PETAJOULE: Energy = 1000000000000000.0 * JOULE;

    /// One `Terajoule` (TJ), as an `Energy`.
    pub const TERAJOULE: Energy = 1000000000000.0 * JOULE;

    /// One `Gigajoule` (GJ), as an `Energy`.
    pub const GIGAJOULE: Energy = 1000000000.0 * JOULE;

    /// One `Megajoule` (MJ), as an `Energy`.
    pub const MEGAJOULE: Energy = 1000000.0 * JOULE;

    /// One `Kilojoule` (kJ), as an `Energy`.
    pub const KILOJOULE: Energy = 1000.0 * JOULE;

    /// One `Hectojoule` (hJ), as an `Energy`.
    pub const HECTOJOULE: Energy = 100.0 * JOULE;

    /// One `Decajoule` (daJ), as an `Energy`.
    pub const DECAJOULE: Energy = 10.0 * JOULE;

    /// One `Joule` (J), as an `Energy`.
    pub const JOULE: Energy = METRE * METRE * KILOGRAM / (SECOND * SECOND);

    /// One `Decijoule` (dJ), as an `Energy`.
    pub const DECIJOULE: Energy = 0.10000000000000001 * JOULE;

    /// One `Centijoule` (cJ), as an `Energy`.
    pub const CENTIJOULE: Energy = 0.01 * JOULE;

    /// One `Millijoule` (mJ), as an `Energy`.
    pub const MILLIJOULE: Energy = 0.001 * JOULE;

    /// One `Microjoule` (µJ), as an `Energy`.
    pub const MICROJOULE: Energy = 9.9999999999999995e-07 * JOULE;

    /// One `Nanojoule` (nJ), as an `Energy`.
    pub const NANOJOULE: Energy = 1.0000000000000001e-09 * JOULE;

    /// One `Picojoule` (pJ), as an `Energy`.
    pub const PICOJOULE: Energy = 9.9999999999999998e-13 * JOULE;

    /// One `Femtojoule` (fJ), as an `Energy`.
    pub const FEMTOJOULE: Energy = 1.0000000000000001e-15 * JOULE;

    /// One `Attojoule` (aJ), as an `Energy`.
    pub const ATTOJOULE: Energy = 1.0000000000000001e-18 * JOULE;

    /// One `Zeptojoule` (zJ), as an `Energy`.
    pub const ZEPTOJOULE: Energy = 9.9999999999999991e-22 * JOULE;

    /// One `Yoctojoule` (yJ), as an `Energy`.
    pub const YOCTOJOULE: Energy = 9.9999999999999992e-25 * JOULE;

    /// One `PetawattHour` (PW · h), as an `Energy`.
    pub const PETAWATT_HOUR: Energy = 3.6e+18 * JOULE;

    /// One `TerawattHour` (TW · h), as an `Energy`.
    pub const TERAWATT_HOUR: Energy = 3600000000000000.0 * JOULE;

    /// One `GigawattHour` (GW · h), as an `Energy`.
    pub const GIGAWATT_HOUR: Energy = 3600000000000.0 * JOULE;

    /// One `MegawattHour` (MW · h), as an `Energy`.
    pub const MEGAWATT_HOUR: Energy = 3600000000.0 * JOULE;

    /// One `KilowattHour` (kW · h), as an `Energy`.
    pub const KILOWATT_HOUR: Energy = 3600000.0 * JOULE;

    /// One `HectowattHour` (hW · h), as an `Energy`.
    pub const HECTOWATT_HOUR: Energy = 360000.0 * JOULE;

    /// One `DecawattHour` (daW · h), as an `Energy`.
    pub const DECAWATT_HOUR: Energy = 36000.0 * JOULE;

    /// One `WattHour` (W · h), as an `Energy`.
    pub const WATT_HOUR: Energy = 3600.0 * JOULE;

    /// One `MilliwattHour` (mW · h), as an `Energy`.
    pub const MILLIWATT_HOUR: Energy = 3.6000000000000001 * JOULE;

    /// One `MicrowattHour` (µW · h), as an `Energy`.
    pub const MICROWATT_HOUR: Energy = 0.0035999999999999999 * JOULE;

    /// One `Petaelectronvolt` (PeV), as an `Energy`.
    pub const PETAELECTRONVOLT: Energy = 0.00016021766339999999 * JOULE;

    /// One `Teraelectronvolt` (TeV), as an `Energy`.
    pub const TERAELECTRONVOLT: Energy = 1.6021766339999999e-07 * JOULE;

    /// One `Gigaelectronvolt` (GeV), as an `Energy`.
    pub const GIGAELECTRONVOLT: Energy = 1.6021766340000001e-10 * JOULE;

    /// One `Megaelectronvolt` (MeV), as an `Energy`.
    pub const MEGAELECTRONVOLT: Energy = 1.6021766340000001e-13 * JOULE;

    /// One `Kiloelectronvolt` (keV), as an `Energy`.
    pub const KILOELECTRONVOLT: Energy = 1.6021766339999999e-16 * JOULE;

    /// One `Hectoelectronvolt` (heV), as an `Energy`.
    pub const HECTOELECTRONVOLT: Energy = 1.6021766339999999e-17 * JOULE;

    /// One `Decaelectronvolt` (daeV), as an `Energy`.
    pub const DECAELECTRONVOLT: Energy = 1.6021766339999999e-18 * JOULE;

    /// One `Electronvolt` (eV), as an `Energy`.
    pub const ELECTRONVOLT: Energy = 1.6021766339999999e-19 * JOULE;

    /// One `Hartree` (Eₕ), as an `Energy`.
    pub const HARTREE: Energy = 4.3597447222071e-18 * JOULE;

    /// One `BtuIt` (Btu (IT)), as an `Energy`.
    pub const BTU_IT: Energy = 1055.05585262 * JOULE;

    /// One `Btu` (Btu), as an `Energy`.
    pub const BTU: Energy = (4184.0 * 0.45359237 / 1.8) * JOULE;

    /// One `CalorieIt` (cal (IT)), as an `Energy`.
    pub const CALORIE_IT: Energy = 4.1867999999999999 * JOULE;

    /// One `Calorie` (cal), as an `Energy`.
    pub const CALORIE: Energy = 4.1840000000000002 * JOULE;

    /// One `CalorieItNutrition` (Cal (IT)), as an `Energy`.
    pub const CALORIE_IT_NUTRITION: Energy = 4186.8000000000002 * JOULE;

    /// One `CalorieNutrition` (Cal), as an `Energy`.
    pub const CALORIE_NUTRITION: Energy = 4184.0 * JOULE;

    /// One `Erg` (erg), as an `Energy`.
    pub const ERG: Energy = 9.9999999999999995e-08 * JOULE;

    /// One `FootPoundal` (ft · pdl), as an `Energy`.
    pub const FOOT_POUNDAL: Energy = (0.3048 * 0.45359237 * 0.3048) * JOULE;

    /// One `FootPound` (ft · lbf), as an `Energy`.
    pub const FOOT_POUND: Energy = (0.3048 * 0.45359237 * 9.80665) * JOULE;

    /// One `KilocalorieIt` (kcal (IT)), as an `Energy`.
    pub const KILOCALORIE_IT: Energy = 4186.8000000000002 * JOULE;

    /// One `Kilocalorie` (kcal), as an `Energy`.
    pub const KILOCALORIE: Energy = 4184.0 * JOULE;

    /// One `Quad` (10¹⁵ Btu (IT)), as an `Energy`.
    pub const QUAD: Energy = 1.05505585262e18 * JOULE;

    /// One `ThermEc` (thm (EC)), as an `Energy`.
    pub const THERM_EC: Energy = 105506000.0 * JOULE;

    /// One `ThermUs` (thm), as an `Energy`.
    pub const THERM_US: Energy = 105480400.0 * JOULE;

    /// One `TonTnt` (t of TNT), as an `Energy`.
    pub const TON_TNT: Energy = 4184000000.0 * JOULE;

    /// One `WattSecond` (W · s), as an `Energy`.
    pub const WATT_SECOND: Energy = JOULE;

    /// One `YottajoulePerKilogram` (YJ/kg), as an `AvailableEnergy`.
    pub const YOTTAJOULE_PER_KILOGRAM: AvailableEnergy =
        9.9999999999999998e+23 * JOULE_PER_KILOGRAM;

    /// One `ZettajoulePerKilogram` (ZJ/kg), as an `AvailableEnergy`.
    pub const ZETTAJOULE_PER_KILOGRAM: AvailableEnergy = 1e+21 * JOULE_PER_KILOGRAM;

    /// One `ExajoulePerKilogram` (EJ/kg), as an `AvailableEnergy`.
    pub const EXAJOULE_PER_KILOGRAM: AvailableEnergy = 1e+18 * JOULE_PER_KILOGRAM;

    /// One `PetajoulePerKilogram` (PJ/kg), as an `AvailableEnergy`.
    pub const PETAJOULE_PER_KILOGRAM: AvailableEnergy = 1000000000000000.0 * JOULE_PER_KILOGRAM;

    /// One `TerajoulePerKilogram` (TJ/kg), as an `AvailableEnergy`.
    pub const TERAJOULE_PER_KILOGRAM: AvailableEnergy = 1000000000000.0 * JOULE_PER_KILOGRAM;

    /// One `GigajoulePerKilogram` (GJ/kg), as an `AvailableEnergy`.
    pub const GIGAJOULE_PER_KILOGRAM: AvailableEnergy = 1000000000.0 * JOULE_PER_KILOGRAM;

    /// One `MegajoulePerKilogram` (MJ/kg), as an `AvailableEnergy`.
    pub const MEGAJOULE_PER_KILOGRAM: AvailableEnergy = 1000000.0 * JOULE_PER_KILOGRAM;

    /// One `KilojoulePerKilogram` (kJ/kg), as an `AvailableEnergy`.
    pub const KILOJOULE_PER_KILOGRAM: AvailableEnergy = 1000.0 * JOULE_PER_KILOGRAM;

    /// One `HectojoulePerKilogram` (hJ/kg), as an `AvailableEnergy`.
    pub const HECTOJOULE_PER_KILOGRAM: AvailableEnergy = 100.0 * JOULE_PER_KILOGRAM;

    /// One `DecajoulePerKilogram` (daJ/kg), as an `AvailableEnergy`.
    pub const DECAJOULE_PER_KILOGRAM: AvailableEnergy = 10.0 * JOULE_PER_KILOGRAM;

    /// One `JoulePerKilogram` (J/kg), as an `AvailableEnergy`.
    pub const JOULE_PER_KILOGRAM: AvailableEnergy = METRE * METRE / (SECOND * SECOND);

    /// One `DecijoulePerKilogram` (dJ/kg), as an `AvailableEnergy`.
    pub const DECIJOULE_PER_KILOGRAM: AvailableEnergy = 0.10000000000000001 * JOULE_PER_KILOGRAM;

    /// One `CentijoulePerKilogram` (cJ/kg), as an `AvailableEnergy`.
    pub const CENTIJOULE_PER_KILOGRAM: AvailableEnergy = 0.01 * JOULE_PER_KILOGRAM;

    /// One `MillijoulePerKilogram` (mJ/kg), as an `AvailableEnergy`.
    pub const MILLIJOULE_PER_KILOGRAM: AvailableEnergy = 0.001 * JOULE_PER_KILOGRAM;

    /// One `MicrojoulePerKilogram` (µJ/kg), as an `AvailableEnergy`.
    pub const MICROJOULE_PER_KILOGRAM: AvailableEnergy =
        9.9999999999999995e-07 * JOULE_PER_KILOGRAM;

    /// One `NanojoulePerKilogram` (nJ/kg), as an `AvailableEnergy`.
    pub const NANOJOULE_PER_KILOGRAM: AvailableEnergy = 1.0000000000000001e-09 * JOULE_PER_KILOGRAM;

    /// One `PicojoulePerKilogram` (pJ/kg), as an `AvailableEnergy`.
    pub const PICOJOULE_PER_KILOGRAM: AvailableEnergy = 9.9999999999999998e-13 * JOULE_PER_KILOGRAM;

    /// One `FemtojoulePerKilogram` (fJ/kg), as an `AvailableEnergy`.
    pub const FEMTOJOULE_PER_KILOGRAM: AvailableEnergy =
        1.0000000000000001e-15 * JOULE_PER_KILOGRAM;

    /// One `AttojoulePerKilogram` (aJ/kg), as an `AvailableEnergy`.
    pub const ATTOJOULE_PER_KILOGRAM: AvailableEnergy = 1.0000000000000001e-18 * JOULE_PER_KILOGRAM;

    /// One `ZeptojoulePerKilogram` (zJ/kg), as an `AvailableEnergy`.
    pub const ZEPTOJOULE_PER_KILOGRAM: AvailableEnergy =
        9.9999999999999991e-22 * JOULE_PER_KILOGRAM;

    /// One `YoctojoulePerKilogram` (yJ/kg), as an `AvailableEnergy`.
    pub const YOCTOJOULE_PER_KILOGRAM: AvailableEnergy =
        9.9999999999999992e-25 * JOULE_PER_KILOGRAM;

    /// One `JoulePerZeptogram` (J/zg), as an `AvailableEnergy`.
    pub const JOULE_PER_ZEPTOGRAM: AvailableEnergy = 9.9999999999999998e+23 * JOULE_PER_KILOGRAM;

    /// One `JoulePerAttogram` (J/ag), as an `AvailableEnergy`.
    pub const JOULE_PER_ATTOGRAM: AvailableEnergy = 1e+21 * JOULE_PER_KILOGRAM;

    /// One `JoulePerFemtogram` (J/fg), as an `AvailableEnergy`.
    pub const JOULE_PER_FEMTOGRAM: AvailableEnergy = 1e+18 * JOULE_PER_KILOGRAM;

    /// One `JoulePerPicogram` (J/pg), as an `AvailableEnergy`.
    pub const JOULE_PER_PICOGRAM: AvailableEnergy = 1000000000000000.0 * JOULE_PER_KILOGRAM;

    /// One `JoulePerNanogram` (J/ng), as an `AvailableEnergy`.
    pub const JOULE_PER_NANOGRAM: AvailableEnergy = 1000000000000.0 * JOULE_PER_KILOGRAM;

    /// One `JoulePerMicrogram` (J/µg), as an `AvailableEnergy`.
    pub const JOULE_PER_MICROGRAM: AvailableEnergy = 1000000000.0 * JOULE_PER_KILOGRAM;

    /// One `JoulePerMilligram` (J/mg), as an `AvailableEnergy`.
    pub const JOULE_PER_MILLIGRAM: AvailableEnergy = 1000000.0 * JOULE_PER_KILOGRAM;

    /// One `JoulePerGram` (J/g), as an `AvailableEnergy`.
    pub const JOULE_PER_GRAM: AvailableEnergy = 1000.0 * JOULE_PER_KILOGRAM;

    /// One `JoulePerMegagram` (J/Mg), as an `AvailableEnergy`.
    pub const JOULE_PER_MEGAGRAM: AvailableEnergy = 0.001 * JOULE_PER_KILOGRAM;

    /// One `JoulePerGigagram` (J/Gg), as an `AvailableEnergy`.
    pub const JOULE_PER_GIGAGRAM: AvailableEnergy = 9.9999999999999995e-07 * JOULE_PER_KILOGRAM;

    /// One `JoulePerTeragram` (J/Tg), as an `AvailableEnergy`.
    pub const JOULE_PER_TERAGRAM: AvailableEnergy = 1.0000000000000001e-09 * JOULE_PER_KILOGRAM;

    /// One `JoulePerPetagram` (J/Pg), as an `AvailableEnergy`.
    pub const JOULE_PER_PETAGRAM: AvailableEnergy = 9.9999999999999998e-13 * JOULE_PER_KILOGRAM;

    /// One `JoulePerExagram` (J/Eg), as an `AvailableEnergy`.
    pub const JOULE_PER_EXAGRAM: AvailableEnergy = 1.0000000000000001e-15 * JOULE_PER_KILOGRAM;

    /// One `JoulePerZettagram` (J/Zg), as an `AvailableEnergy`.
    pub const JOULE_PER_ZETTAGRAM: AvailableEnergy = 1.0000000000000001e-18 * JOULE_PER_KILOGRAM;

    /// One `JoulePerYottagram` (J/Yg), as an `AvailableEnergy`.
    pub const JOULE_PER_YOTTAGRAM: AvailableEnergy = 9.9999999999999991e-22 * JOULE_PER_KILOGRAM;

    /// One `BtuItPerPound` (Btu (IT)/lb), as an `AvailableEnergy`.
    pub const BTU_IT_PER_POUND: AvailableEnergy = 2326.0 * JOULE_PER_KILOGRAM;

    /// One `BtuPerPound` (Btu/lb), as an `AvailableEnergy`.
    pub const BTU_PER_POUND: AvailableEnergy = (4184.0 / 1.8) * JOULE_PER_KILOGRAM;

    /// One `CalorieItPerGram` (cal (IT)/g), as an `AvailableEnergy`.
    pub const CALORIE_IT_PER_GRAM: AvailableEnergy = 4186.8000000000002 * JOULE_PER_KILOGRAM;

    /// One `CaloriePerGram` (cal/g), as an `AvailableEnergy`.
    pub const CALORIE_PER_GRAM: AvailableEnergy = 4184.0 * JOULE_PER_KILOGRAM;

    /// One `Yottawatt` (YW), as a `Power`.
    pub const YOTTAWATT: Power = 9.9999999999999998e+23 * WATT;

    /// One `Zettawatt` (ZW), as a `Power`.
    pub const ZETTAWATT: Power = 1e+21 * WATT;

    /// One `Exawatt` (EW), as a `Power`.
    pub const EXAWATT: Power = 1e+18 * WATT;

    /// One `Petawatt` (PW), as a `Power`.
    pub const PETAWATT: Power = 1000000000000000.0 * WATT;

    /// One `Terawatt` (TW), as a `Power`.
    pub const TERAWATT: Power = 1000000000000.0 * WATT;

    /// One `Gigawatt` (GW), as a `Power`.
    pub const GIGAWATT: Power = 1000000000.0 * WATT;

    /// One `Megawatt` (MW), as a `Power`.
    pub const MEGAWATT: Power = 1000000.0 * WATT;

    /// One `Kilowatt` (kW), as a `Power`.
    pub const KILOWATT: Power = 1000.0 * WATT;

    /// One `Hectowatt` (hW), as a `Power`.
    pub const HECTOWATT: Power = 100.0 * WATT;

    /// One `Decawatt` (daW), as a `Power`.
    pub const DECAWATT: Power = 10.0 * WATT;

    /// One `Watt` (W), as a `Power`.
    pub const WATT: Power = METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND);

    /// One `Deciwatt` (dW), as a `Power`.
    pub const DECIWATT: Power = 0.10000000000000001 * WATT;

    /// One `Centiwatt` (cW), as a `Power`.
    pub const CENTIWATT: Power = 0.01 * WATT;

    /// One `Milliwatt` (mW), as a `Power`.
    pub const MILLIWATT: Power = 0.001 * WATT;

    /// One `Microwatt` (µW), as a `Power`.
    pub const MICROWATT: Power = 9.9999999999999995e-07 * WATT;

    /// One `Nanowatt` (nW), as a `Power`.
    pub const NANOWATT: Power = 1.0000000000000001e-09 * WATT;

    /// One `Picowatt` (pW), as a `Power`.
    pub const PICOWATT: Power = 9.9999999999999998e-13 * WATT;

    /// One `Femtowatt` (fW), as a `Power`.
    pub const FEMTOWATT: Power = 1.0000000000000001e-15 * WATT;

    /// One `Attowatt` (aW), as a `Power`.
    pub const ATTOWATT: Power = 1.0000000000000001e-18 * WATT;

    /// One `Zeptowatt` (zW), as a `Power`.
    pub const ZEPTOWATT: Power = 9.9999999999999991e-22 * WATT;

    /// One `Yoctowatt` (yW), as a `Power`.
    pub const YOCTOWATT: Power = 9.9999999999999992e-25 * WATT;

    /// One `ErgPerSecond` (erg/s), as a `Power`.
    pub const ERG_PER_SECOND: Power = 9.9999999999999995e-08 * WATT;

    /// One `FootPoundPerHour` (ft · lbf/h), as a `Power`.
    pub const FOOT_POUND_PER_HOUR: Power = ((0.3048 * 0.45359237 * 9.80665) / 3600.0) * WATT;

    /// One `FootPoundPerMinute` (ft · lbf/min), as a `Power`.
    pub const FOOT_POUND_PER_MINUTE: Power = ((0.3048 * 0.45359237 * 9.80665) / 60.0) * WATT;

    /// One `FootPoundPerSecond` (ft · lbf/s), as a `Power`.
    pub const FOOT_POUND_PER_SECOND: Power = (0.3048 * 0.45359237 * 9.80665) * WATT;

    /// One `Horsepower` (hp), as a `Power`.
    pub const HORSEPOWER: Power = (550.0 * 0.3048 * 0.45359237 * 9.80665) * WATT;

    /// One `HorsepowerBoiler` (hp (S)), as a `Power`.
    pub const HORSEPOWER_BOILER: Power = 9809.5 * WATT;

    /// One `HorsepowerElectric` (hp (E)), as a `Power`.
    pub const HORSEPOWER_ELECTRIC: Power = 746.0 * WATT;

    /// One `HorsepowerMetric` (hp (M)), as a `Power`.
    pub const HORSEPOWER_METRIC: Power = (75.0 * 9.80665) * WATT;

    /// One `HorsepowerImperial` (hp (I)), as a `Power`.
    pub const HORSEPOWER_IMPERIAL: Power = 745.70000000000005 * WATT;

    /// One `HydraulicHorsepower` (hp (hydraulic)), as a `Power`.
    pub const HYDRAULIC_HORSEPOWER: Power = 746.04300000000001 * WATT;

    /// One `Decibel` (dB), as a `Gain`.
    pub const DECIBEL: Gain = 0.1 * BEL;

    /// One `Neper` (Np), as a `Gain`.
    pub const NEPER: Gain = 0.8685889638065036553 * BEL;

    /// The level of one milliwatt, 0 dBm: 30 dB below a watt.
    pub const MILLIWATT_LEVEL: PowerLevel = WATT_LEVEL - 30.0 * DECIBEL;

    /// Decibels-watt (dBW): decibels from one watt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBW: Scale<PowerLevel, Gain> = Scale(WATT_LEVEL, DECIBEL);

    /// Decibels-milliwatt (dBm): decibels from one milliwatt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBM: Scale<PowerLevel, Gain> = Scale(MILLIWATT_LEVEL, DECIBEL);

    /// One `YottawattPerSecond` (YW/s), as a `PowerRate`.
    pub const YOTTAWATT_PER_SECOND: PowerRate = 9.9999999999999998e+23 * WATT_PER_SECOND;

    /// One `ZettawattPerSecond` (ZW/s), as a `PowerRate`.
    pub const ZETTAWATT_PER_SECOND: PowerRate = 1e+21 * WATT_PER_SECOND;

    /// One `ExawattPerSecond` (EW/s), as a `PowerRate`.
    pub const EXAWATT_PER_SECOND: PowerRate = 1e+18 * WATT_PER_SECOND;

    /// One `PetawattPerSecond` (PW/s), as a `PowerRate`.
    pub const PETAWATT_PER_SECOND: PowerRate = 1000000000000000.0 * WATT_PER_SECOND;

    /// One `TerawattPerSecond` (TW/s), as a `PowerRate`.
    pub const TERAWATT_PER_SECOND: PowerRate = 1000000000000.0 * WATT_PER_SECOND;

    /// One `GigawattPerSecond` (GW/s), as a `PowerRate`.
    pub const GIGAWATT_PER_SECOND: PowerRate = 1000000000.0 * WATT_PER_SECOND;

    /// One `MegawattPerSecond` (MW/s), as a `PowerRate`.
    pub const MEGAWATT_PER_SECOND: PowerRate = 1000000.0 * WATT_PER_SECOND;

    /// One `KilowattPerSecond` (kW/s), as a `PowerRate`.
    pub const KILOWATT_PER_SECOND: PowerRate = 1000.0 * WATT_PER_SECOND;

    /// One `HectowattPerSecond` (hW/s), as a `PowerRate`.
    pub const HECTOWATT_PER_SECOND: PowerRate = 100.0 * WATT_PER_SECOND;

    /// One `DecawattPerSecond` (daW/s), as a `PowerRate`.
    pub const DECAWATT_PER_SECOND: PowerRate = 10.0 * WATT_PER_SECOND;

    /// One `WattPerSecond` (W/s), as a `PowerRate`.
    pub const WATT_PER_SECOND: PowerRate =
        METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND);

    /// One `DeciwattPerSecond` (dW/s), as a `PowerRate`.
    pub const DECIWATT_PER_SECOND: PowerRate = 0.10000000000000001 * WATT_PER_SECOND;

    /// One `CentiwattPerSecond` (cW/s), as a `PowerRate`.
    pub const CENTIWATT_PER_SECOND: PowerRate = 0.01 * WATT_PER_SECOND;

    /// One `MilliwattPerSecond` (mW/s), as a `PowerRate`.
    pub const MILLIWATT_PER_SECOND: PowerRate = 0.001 * WATT_PER_SECOND;

    /// One `MicrowattPerSecond` (µW/s), as a `PowerRate`.
    pub const MICROWATT_PER_SECOND: PowerRate = 9.9999999999999995e-07 * WATT_PER_SECOND;

    /// One `NanowattPerSecond` (nW/s), as a `PowerRate`.
    pub const NANOWATT_PER_SECOND: PowerRate = 1.0000000000000001e-09 * WATT_PER_SECOND;

    /// One `PicowattPerSecond` (pW/s), as a `PowerRate`.
    pub const PICOWATT_PER_SECOND: PowerRate = 9.9999999999999998e-13 * WATT_PER_SECOND;

    /// One `FemtowattPerSecond` (fW/s), as a `PowerRate`.
    pub const FEMTOWATT_PER_SECOND: PowerRate = 1.0000000000000001e-15 * WATT_PER_SECOND;

    /// One `AttowattPerSecond` (aW/s), as a `PowerRate`.
    pub const ATTOWATT_PER_SECOND: PowerRate = 1.0000000000000001e-18 * WATT_PER_SECOND;

    /// One `ZeptowattPerSecond` (zW/s), as a `PowerRate`.
    pub const ZEPTOWATT_PER_SECOND: PowerRate = 9.9999999999999991e-22 * WATT_PER_SECOND;

    /// One `YoctowattPerSecond` (yW/s), as a `PowerRate`.
    pub const YOCTOWATT_PER_SECOND: PowerRate = 9.9999999999999992e-25 * WATT_PER_SECOND;

    /// One `ErgPerSecondSquared` (erg/s²), as a `PowerRate`.
    pub const ERG_PER_SECOND_SQUARED: PowerRate = 9.9999999999999995e-08 * WATT_PER_SECOND;

    /// One `FootPoundPerSecondSquared` (ft · lbf/s²), as a `PowerRate`.
    pub const FOOT_POUND_PER_SECOND_SQUARED: PowerRate =
        (0.3048 * 0.45359237 * 9.80665) * WATT_PER_SECOND;

    /// One `HorsepowerPerSecond` (hp/s), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND: PowerRate =
        (550.0 * 0.3048 * 0.45359237 * 9.80665) * WATT_PER_SECOND;

    /// One `HorsepowerPerSecondBoiler` (hp/s (S)), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND_BOILER: PowerRate = 9809.5 * WATT_PER_SECOND;

    /// One `HorsepowerPerSecondElectric` (hp/s (E)), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND_ELECTRIC: PowerRate = 746.0 * WATT_PER_SECOND;

    /// One `HorsepowerPerSecondMetric` (hp/s (M)), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND_METRIC: PowerRate = (75.0 * 9.80665) * WATT_PER_SECOND;

    /// One `HorsepowerPerSecondImperial` (hp/s (I)), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND_IMPERIAL: PowerRate = 745.70000000000005 * WATT_PER_SECOND;

    /// One `HydraulicHorsepowerPerSecond` (hp/s (hydraulic)), as a `PowerRate`.
    pub const HYDRAULIC_HORSEPOWER_PER_SECOND: PowerRate = 746.04300000000001 * WATT_PER_SECOND;

    /// One `JouleSecond` (J · s), as an `Action`.
    pub const JOULE_SECOND: Action = METRE * METRE * KILOGRAM / SECOND;

    /// One `AtomicUnitOfAction` (ħ), as an `Action`.
    pub const ATOMIC_UNIT_OF_ACTION: Action =
        (6.62607015e-34 / (2.0 * core::f64::consts::PI)) * JOULE_SECOND;

    /// One `PlanckConstant` (h), as an `Action`.
    pub const PLANCK_CONSTANT: Action = 6.62607015e-34 * JOULE_SECOND;

    /// One `ErgSecond` (erg · s), as an `Action`.
    pub const ERG_SECOND: Action = 9.9999999999999995e-08 * JOULE_SECOND;

    /// One `ElectronvoltSecond` (eV · s), as an `Action`.
    pub const ELECTRONVOLT_SECOND: Action = 1.6021766339999999e-19 * JOULE_SECOND;

    /// One `YottanewtonMetre` (YN · m), as a `Torque`.
    pub const YOTTANEWTON_METRE: Torque = 9.9999999999999998e+23 * NEWTON_METRE;

    /// One `ZettanewtonMetre` (ZN · m), as a `Torque`.
    pub const ZETTANEWTON_METRE: Torque = 1e+21 * NEWTON_METRE;

    /// One `ExanewtonMetre` (EN · m), as a `Torque`.
    pub const EXANEWTON_METRE: Torque = 1e+18 * NEWTON_METRE;

    /// One `PetanewtonMetre` (PN · m), as a `Torque`.
    pub const PETANEWTON_METRE: Torque = 1000000000000000.0 * NEWTON_METRE;

    /// One `TeranewtonMetre` (TN · m), as a `Torque`.
    pub const TERANEWTON_METRE: Torque = 1000000000000.0 * NEWTON_METRE;

    /// One `GiganewtonMetre` (GN · m), as a `Torque`.
    pub const GIGANEWTON_METRE: Torque = 1000000000.0 * NEWTON_METRE;

    /// One `MeganewtonMetre` (MN · m), as a `Torque`.
    pub const MEGANEWTON_METRE: Torque = 1000000.0 * NEWTON_METRE;

    /// One `KilonewtonMetre` (kN · m), as a `Torque`.
    pub const KILONEWTON_METRE: Torque = 1000.0 * NEWTON_METRE;

    /// One `HectonewtonMetre` (hN · m), as a `Torque`.
    pub const HECTONEWTON_METRE: Torque = 100.0 * NEWTON_METRE;

    /// One `DecanewtonMetre` (daN · m), as a `Torque`.
    pub const DECANEWTON_METRE: Torque = 10.0 * NEWTON_METRE;

    /// One `NewtonMetre` (N · m), as a `Torque`.
    pub const NEWTON_METRE: Torque = METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN);

    /// One `DecinewtonMetre` (dN · m), as a `Torque`.
    pub const DECINEWTON_METRE: Torque = 0.10000000000000001 * NEWTON_METRE;

    /// One `CentinewtonMetre` (cN · m), as a `Torque`.
    pub const CENTINEWTON_METRE: Torque = 0.01 * NEWTON_METRE;

    /// One `MillinewtonMetre` (mN · m), as a `Torque`.
    pub const MILLINEWTON_METRE: Torque = 0.001 * NEWTON_METRE;

    /// One `MicronewtonMetre` (µN · m), as a `Torque`.
    pub const MICRONEWTON_METRE: Torque = 9.9999999999999995e-07 * NEWTON_METRE;

    /// One `NanonewtonMetre` (nN · m), as a `Torque`.
    pub const NANONEWTON_METRE: Torque = 1.0000000000000001e-09 * NEWTON_METRE;

    /// One `PiconewtonMetre` (pN · m), as a `Torque`.
    pub const PICONEWTON_METRE: Torque = 9.9999999999999998e-13 * NEWTON_METRE;

    /// One `FemtonewtonMetre` (fN · m), as a `Torque`.
    pub const FEMTONEWTON_METRE: Torque = 1.0000000000000001e-15 * NEWTON_METRE;

    /// One `AttonewtonMetre` (aN · m), as a `Torque`.
    pub const ATTONEWTON_METRE: Torque = 1.0000000000000001e-18 * NEWTON_METRE;

    /// One `ZeptonewtonMetre` (zN · m), as a `Torque`.
    pub const ZEPTONEWTON_METRE: Torque = 9.9999999999999991e-22 * NEWTON_METRE;

    /// One `YoctonewtonMetre` (yN · m), as a `Torque`.
    pub const YOCTONEWTON_METRE: Torque = 9.9999999999999992e-25 * NEWTON_METRE;

    /// One `NewtonYottametre` (N · Ym), as a `Torque`.
    pub const NEWTON_YOTTAMETRE: Torque = 9.9999999999999998e+23 * NEWTON_METRE;

    /// One `NewtonZettametre` (N · Zm), as a `Torque`.
    pub const NEWTON_ZETTAMETRE: Torque = 1e+21 * NEWTON_METRE;

    /// One `NewtonExametre` (N · Em), as a `Torque`.
    pub const NEWTON_EXAMETRE: Torque = 1e+18 * NEWTON_METRE;

    /// One `NewtonPetametre` (N · Pm), as a `Torque`.
    pub const NEWTON_PETAMETRE: Torque = 1000000000000000.0 * NEWTON_METRE;

    /// One `NewtonTerametre` (N · Tm), as a `Torque`.
    pub const NEWTON_TERAMETRE: Torque = 1000000000000.0 * NEWTON_METRE;

    /// One `NewtonGigametre` (N · Gm), as a `Torque`.
    pub const NEWTON_GIGAMETRE: Torque = 1000000000.0 * NEWTON_METRE;

    /// One `NewtonMegametre` (N · Mm), as a `Torque`.
    pub const NEWTON_MEGAMETRE: Torque = 1000000.0 * NEWTON_METRE;

    /// One `NewtonKilometre` (N · km), as a `Torque`.
    pub const NEWTON_KILOMETRE: Torque = 1000.0 * NEWTON_METRE;

    /// One `NewtonHectometre` (N · hm), as a `Torque`.
    pub const NEWTON_HECTOMETRE: Torque = 100.0 * NEWTON_METRE;

    /// One `NewtonDecametre` (N · dam), as a `Torque`.
    pub const NEWTON_DECAMETRE: Torque = 10.0 * NEWTON_METRE;

    /// One `NewtonDecimetre` (N · dm), as a `Torque`.
    pub const NEWTON_DECIMETRE: Torque = 0.10000000000000001 * NEWTON_METRE;

    /// One `NewtonCentimetre` (N · cm), as a `Torque`.
    pub const NEWTON_CENTIMETRE: Torque = 0.01 * NEWTON_METRE;

    /// One `NewtonMillimetre` (N · mm), as a `Torque`.
    pub const NEWTON_MILLIMETRE: Torque = 0.001 * NEWTON_METRE;

    /// One `NewtonMicrometre` (N · µm), as a `Torque`.
    pub const NEWTON_MICROMETRE: Torque = 9.9999999999999995e-07 * NEWTON_METRE;

    /// One `NewtonNanometre` (N · nm), as a `Torque`.
    pub const NEWTON_NANOMETRE: Torque = 1.0000000000000001e-09 * NEWTON_METRE;

    /// One `NewtonPicometre` (N · pm), as a `Torque`.
    pub const NEWTON_PICOMETRE: Torque = 9.9999999999999998e-13 * NEWTON_METRE;

    /// One `NewtonFemtometre` (N · fm), as a `Torque`.
    pub const NEWTON_FEMTOMETRE: Torque = 1.0000000000000001e-15 * NEWTON_METRE;

    /// One `NewtonAttometre` (N · am), as a `Torque`.
    pub const NEWTON_ATTOMETRE: Torque = 1.0000000000000001e-18 * NEWTON_METRE;

    /// One `NewtonZeptometre` (N · zm), as a `Torque`.
    pub const NEWTON_ZEPTOMETRE: Torque = 9.9999999999999991e-22 * NEWTON_METRE;

    /// One `NewtonYoctometre` (N · ym), as a `Torque`.
    pub const NEWTON_YOCTOMETRE: Torque = 9.9999999999999992e-25 * NEWTON_METRE;

    /// One `DyneMetre` (dyn · m), as a `Torque`.
    pub const DYNE_METRE: Torque = 1.0000000000000001e-05 * NEWTON_METRE;

    /// One `DyneCentimetre` (dyn · cm), as a `Torque`.
    pub const DYNE_CENTIMETRE: Torque = 9.9999999999999995e-08 * NEWTON_METRE;

    /// One `KilogramForceMetre` (kgf · m), as a `Torque`.
    pub const KILOGRAM_FORCE_METRE: Torque = 9.8066499999999994 * NEWTON_METRE;

    /// One `OunceForceInch` (ozf · in), as a `Torque`.
    pub const OUNCE_FORCE_INCH: Torque = ((0.45359237 * 9.80665 / 16.0) * 0.0254) * NEWTON_METRE;

    /// One `PoundForceFoot` (lbf · ft), as a `Torque`.
    pub const POUND_FORCE_FOOT: Torque = ((0.45359237 * 9.80665) * 0.3048) * NEWTON_METRE;

    /// One `PoundForceInch` (lbf · in), as a `Torque`.
    pub const POUND_FORCE_INCH: Torque = ((0.45359237 * 9.80665) * 0.0254) * NEWTON_METRE;

    /// One `KilogramSquareMetre` (kg · m²), as a `MomentOfInertia`.
    pub const KILOGRAM_SQUARE_METRE: MomentOfInertia = METRE * METRE * KILOGRAM / (RADIAN * RADIAN);

    /// One `DaltonSquareAngstrom` (Da · Å²), as a `MomentOfInertia`.
    pub const DALTON_SQUARE_ANGSTROM: MomentOfInertia = 1.6605390666e-47 * KILOGRAM_SQUARE_METRE;

    /// One `Yottapascal` (YPa), as a `Pressure`.
    pub const YOTTAPASCAL: Pressure = 9.9999999999999998e+23 * PASCAL;

    /// One `Zettapascal` (ZPa), as a `Pressure`.
    pub const ZETTAPASCAL: Pressure = 1e+21 * PASCAL;

    /// One `Exapascal` (EPa), as a `Pressure`.
    pub const EXAPASCAL: Pressure = 1e+18 * PASCAL;

    /// One `Petapascal` (PPa), as a `Pressure`.
    pub const PETAPASCAL: Pressure = 1000000000000000.0 * PASCAL;

    /// One `Terapascal` (TPa), as a `Pressure`.
    pub const TERAPASCAL: Pressure = 1000000000000.0 * PASCAL;

    /// One `Gigapascal` (GPa), as a `Pressure`.
    pub const GIGAPASCAL: Pressure = 1000000000.0 * PASCAL;

    /// One `Megapascal` (MPa), as a `Pressure`.
    pub const MEGAPASCAL: Pressure = 1000000.0 * PASCAL;

    /// One `Kilopascal` (kPa), as a `Pressure`.
    pub const KILOPASCAL: Pressure = 1000.0 * PASCAL;

    /// One `Hectopascal` (hPa), as a `Pressure`.
    pub const HECTOPASCAL: Pressure = 100.0 * PASCAL;

    /// One `Decapascal` (daPa), as a `Pressure`.
    pub const DECAPASCAL: Pressure = 10.0 * PASCAL;

    /// One `Pascal` (Pa), as a `Pressure`.
    pub const PASCAL: Pressure = KILOGRAM / (METRE * SECOND * SECOND);

    /// One `Decipascal` (dPa), as a `Pressure`.
    pub const DECIPASCAL: Pressure = 0.10000000000000001 * PASCAL;

    /// One `Centipascal` (cPa), as a `Pressure`.
    pub const CENTIPASCAL: Pressure = 0.01 * PASCAL;

    /// One `Millipascal` (mPa), as a `Pressure`.
    pub const MILLIPASCAL: Pressure = 0.001 * PASCAL;

    /// One `Micropascal` (µPa), as a `Pressure`.
    pub const MICROPASCAL: Pressure = 9.9999999999999995e-07 * PASCAL;

    /// One `Nanopascal` (nPa), as a `Pressure`.
    pub const NANOPASCAL: Pressure = 1.0000000000000001e-09 * PASCAL;

    /// One `Picopascal` (pPa), as a `Pressure`.
    pub const PICOPASCAL: Pressure = 9.9999999999999998e-13 * PASCAL;

    /// One `Femtopascal` (fPa), as a `Pressure`.
    pub const FEMTOPASCAL: Pressure = 1.0000000000000001e-15 * PASCAL;

    /// One `Attopascal` (aPa), as a `Pressure`.
    pub const ATTOPASCAL: Pressure = 1.0000000000000001e-18 * PASCAL;

    /// One `Zeptopascal` (zPa), as a `Pressure`.
    pub const ZEPTOPASCAL: Pressure = 9.9999999999999991e-22 * PASCAL;

    /// One `Yoctopascal` (yPa), as a `Pressure`.
    pub const YOCTOPASCAL: Pressure = 9.9999999999999992e-25 * PASCAL;

    /// One `StandardAtmosphere` (atm), as a `Pressure`.
    pub const STANDARD_ATMOSPHERE: Pressure = 101325.0 * PASCAL;

    /// One `AtmosphereTechnical` (at), as a `Pressure`.
    pub const ATMOSPHERE_TECHNICAL: Pressure = 98066.5 * PASCAL;

    /// One `Bar` (bar), as a `Pressure`.
    pub const BAR: Pressure = 100000.0 * PASCAL;

    /// One `CentimetreOfMercury` (cm Hg), as a `Pressure`.
    pub const CENTIMETRE_OF_MERCURY: Pressure = (133.322387415 * 10.0) * PASCAL;

    /// One `CentimetreOfWater` (cm H₂O), as a `Pressure`.
    pub const CENTIMETRE_OF_WATER: Pressure = 98.066500000000005 * PASCAL;

    /// One `DynePerSquareCentimetre` (dyn/cm²), as a `Pressure`.
    pub const DYNE_PER_SQUARE_CENTIMETRE: Pressure = 0.10000000000000001 * PASCAL;

    /// One `FootOfMercury` (ft Hg), as a `Pressure`.
    pub const FOOT_OF_MERCURY: Pressure = (133.322387415 * 304.8) * PASCAL;

    /// One `FootOfWater` (ft H₂O), as a `Pressure`.
    pub const FOOT_OF_WATER: Pressure = (9.80665 * 304.8) * PASCAL;

    /// One `InchOfMercury` (in Hg), as a `Pressure`.
    pub const INCH_OF_MERCURY: Pressure = (133.322387415 * 25.4) * PASCAL;

    /// One `InchOfWater` (in H₂O), as a `Pressure`.
    pub const INCH_OF_WATER: Pressure = (9.80665 * 25.4) * PASCAL;

    /// One `NewtonPerSquareMillimetre` (N/mm²), as a `Pressure`.
    pub const NEWTON_PER_SQUARE_MILLIMETRE: Pressure = 1000000.0 * PASCAL;

    /// One `KipPerSquareInch` (kip/in²), as a `Pressure`.
    pub const KIP_PER_SQUARE_INCH: Pressure =
        ((1000.0 * 0.45359237 * 9.80665) / (0.0254 * 0.0254)) * PASCAL;

    /// One `Millibar` (mbar), as a `Pressure`.
    pub const MILLIBAR: Pressure = 100.0 * PASCAL;

    /// One `MillimetreOfMercury` (mm Hg), as a `Pressure`.
    pub const MILLIMETRE_OF_MERCURY: Pressure = 133.322387415 * PASCAL;

    /// One `MillimetreOfWater` (mm H₂O), as a `Pressure`.
    pub const MILLIMETRE_OF_WATER: Pressure = 9.8066499999999994 * PASCAL;

    /// One `Millitorr` (mTorr), as a `Pressure`.
    pub const MILLITORR: Pressure = (101325.0 / 760000.0) * PASCAL;

    /// One `PoundalPerSquareFoot` (pdl/ft²), as a `Pressure`.
    pub const POUNDAL_PER_SQUARE_FOOT: Pressure =
        ((0.45359237 * 0.3048) / (0.3048 * 0.3048)) * PASCAL;

    /// One `PoundForcePerSquareFoot` (lbf/ft²), as a `Pressure`.
    pub const POUND_FORCE_PER_SQUARE_FOOT: Pressure =
        ((0.45359237 * 9.80665) / (0.3048 * 0.3048)) * PASCAL;

    /// One `PoundForcePerSquareInch` (lbf/in²), as a `Pressure`.
    pub const POUND_FORCE_PER_SQUARE_INCH: Pressure =
        ((0.45359237 * 9.80665) / (0.0254 * 0.0254)) * PASCAL;

    /// One `Psi` (psi), as a `Pressure`.
    pub const PSI: Pressure = (0.45359237 * 9.80665 / (0.0254 * 0.0254)) * PASCAL;

    /// One `Torr` (Torr), as a `Pressure`.
    pub const TORR: Pressure = (101325.0 / 760.0) * PASCAL;

    /// One `Yottahertz` (YHz), as a `Frequency`.
    pub const YOTTAHERTZ: Frequency = 9.9999999999999998e+23 * HERTZ;

    /// One `Zettahertz` (ZHz), as a `Frequency`.
    pub const ZETTAHERTZ: Frequency = 1e+21 * HERTZ;

    /// One `Exahertz` (EHz), as a `Frequency`.
    pub const EXAHERTZ: Frequency = 1e+18 * HERTZ;

    /// One `Petahertz` (PHz), as a `Frequency`.
    pub const PETAHERTZ: Frequency = 1000000000000000.0 * HERTZ;

    /// One `Terahertz` (THz), as a `Frequency`.
    pub const TERAHERTZ: Frequency = 1000000000000.0 * HERTZ;

    /// One `Gigahertz` (GHz), as a `Frequency`.
    pub const GIGAHERTZ: Frequency = 1000000000.0 * HERTZ;

    /// One `Megahertz` (MHz), as a `Frequency`.
    pub const MEGAHERTZ: Frequency = 1000000.0 * HERTZ;

    /// One `Kilohertz` (kHz), as a `Frequency`.
    pub const KILOHERTZ: Frequency = 1000.0 * HERTZ;

    /// One `Hectohertz` (hHz), as a `Frequency`.
    pub const HECTOHERTZ: Frequency = 100.0 * HERTZ;

    /// One `Decahertz` (daHz), as a `Frequency`.
    pub const DECAHERTZ: Frequency = 10.0 * HERTZ;

    /// One `Hertz` (Hz), as a `Frequency`.
    pub const HERTZ: Frequency = ONE / SECOND;

    /// One `Decihertz` (dHz), as a `Frequency`.
    pub const DECIHERTZ: Frequency = 0.10000000000000001 * HERTZ;

    /// One `Centihertz` (cHz), as a `Frequency`.
    pub const CENTIHERTZ: Frequency = 0.01 * HERTZ;

    /// One `Millihertz` (mHz), as a `Frequency`.
    pub const MILLIHERTZ: Frequency = 0.001 * HERTZ;

    /// One `Microhertz` (µHz), as a `Frequency`.
    pub const MICROHERTZ: Frequency = 9.9999999999999995e-07 * HERTZ;

    /// One `Nanohertz` (nHz), as a `Frequency`.
    pub const NANOHERTZ: Frequency = 1.0000000000000001e-09 * HERTZ;

    /// One `Picohertz` (pHz), as a `Frequency`.
    pub const PICOHERTZ: Frequency = 9.9999999999999998e-13 * HERTZ;

    /// One `Femtohertz` (fHz), as a `Frequency`.
    pub const FEMTOHERTZ: Frequency = 1.0000000000000001e-15 * HERTZ;

    /// One `Attohertz` (aHz), as a `Frequency`.
    pub const ATTOHERTZ: Frequency = 1.0000000000000001e-18 * HERTZ;

    /// One `Zeptohertz` (zHz), as a `Frequency`.
    pub const ZEPTOHERTZ: Frequency = 9.9999999999999991e-22 * HERTZ;

    /// One `Yoctohertz` (yHz), as a `Frequency`.
    pub const YOCTOHERTZ: Frequency = 9.9999999999999992e-25 * HERTZ;

    /// One `CyclePerDay` (1/d), as a `Frequency`.
    pub const CYCLE_PER_DAY: Frequency = 1.1574074074074073e-05 * HERTZ;

    /// One `CyclePerHour` (1/h), as a `Frequency`.
    pub const CYCLE_PER_HOUR: Frequency = 0.00027777777777777772 * HERTZ;

    /// One `CyclePerMinute` (1/min), as a `Frequency`.
    pub const CYCLE_PER_MINUTE: Frequency = 0.016666666666666666 * HERTZ;

    /// One `CyclePerShake` (100 MHz), as a `Frequency`.
    pub const CYCLE_PER_SHAKE: Frequency = 100000000.0 * HERTZ;

    /// One `CyclePerYear` (1/a), as a `Frequency`.
    pub const CYCLE_PER_YEAR: Frequency = 3.1709791983764579e-08 * HERTZ;

    /// One `TerahertzPerSecond` (THz/s), as a `FrequencyDrift`.
    pub const TERAHERTZ_PER_SECOND: FrequencyDrift = 1000000000000.0 * HERTZ_PER_SECOND;

    /// One `GigahertzPerSecond` (GHz/s), as a `FrequencyDrift`.
    pub const GIGAHERTZ_PER_SECOND: FrequencyDrift = 1000000000.0 * HERTZ_PER_SECOND;

    /// One `MegahertzPerSecond` (MHz/s), as a `FrequencyDrift`.
    pub const MEGAHERTZ_PER_SECOND: FrequencyDrift = 1000000.0 * HERTZ_PER_SECOND;

    /// One `KilohertzPerSecond` (kHz/s), as a `FrequencyDrift`.
    pub const KILOHERTZ_PER_SECOND: FrequencyDrift = 1000.0 * HERTZ_PER_SECOND;

    /// One `HertzPerSecond` (Hz/s), as a `FrequencyDrift`.
    pub const HERTZ_PER_SECOND: FrequencyDrift = ONE / (SECOND * SECOND);

    /// One `ReciprocalKilometre` (km⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_KILOMETRE: ReciprocalLength = 0.001 * RECIPROCAL_METRE;

    /// One `ReciprocalMetre` (m⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_METRE: ReciprocalLength = ONE / METRE;

    /// One `ReciprocalDecimetre` (dm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_DECIMETRE: ReciprocalLength = 10.0 * RECIPROCAL_METRE;

    /// One `ReciprocalCentimetre` (cm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_CENTIMETRE: ReciprocalLength = 100.0 * RECIPROCAL_METRE;

    /// One `ReciprocalMillimetre` (mm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_MILLIMETRE: ReciprocalLength = 1000.0 * RECIPROCAL_METRE;

    /// One `ReciprocalMicrometre` (µm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_MICROMETRE: ReciprocalLength = 1000000.0 * RECIPROCAL_METRE;

    /// One `ReciprocalNanometre` (nm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_NANOMETRE: ReciprocalLength = 999999999.99999988 * RECIPROCAL_METRE;

    /// One `ReciprocalAngstrom` (Å⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_ANGSTROM: ReciprocalLength = 10000000000.0 * RECIPROCAL_METRE;

    /// One `Diopter` (dpt), as a `ReciprocalLength`.
    pub const DIOPTER: ReciprocalLength = RECIPROCAL_METRE;

    /// One `RydbergConstant` (R∞), as a `ReciprocalLength`.
    pub const RYDBERG_CONSTANT: ReciprocalLength = 10973731.568159999 * RECIPROCAL_METRE;

    /// One `RadianPerMetre` (rad/m), as a `Curvature`.
    pub const RADIAN_PER_METRE: Curvature = RADIAN / METRE;

    /// One `DegreePerMetre` (°/m), as a `Curvature`.
    pub const DEGREE_PER_METRE: Curvature = 0.017453292519943295 * RADIAN_PER_METRE;

    /// One `RadianPerMillimetre` (rad/mm), as a `Curvature`.
    pub const RADIAN_PER_MILLIMETRE: Curvature = 1000.0 * RADIAN_PER_METRE;

    /// One `DegreePerMillimetre` (°/mm), as a `Curvature`.
    pub const DEGREE_PER_MILLIMETRE: Curvature = 17.453292519943297 * RADIAN_PER_METRE;

    /// One `SquareMetrePerKilogram` (m²/kg), as a `SpecificArea`.
    pub const SQUARE_METRE_PER_KILOGRAM: SpecificArea = METRE * METRE / KILOGRAM;

    /// One `SquareCentimetrePerKilogram` (cm²/kg), as a `SpecificArea`.
    pub const SQUARE_CENTIMETRE_PER_KILOGRAM: SpecificArea = 0.0001 * SQUARE_METRE_PER_KILOGRAM;

    /// One `SquareMetrePerGram` (m²/g), as a `SpecificArea`.
    pub const SQUARE_METRE_PER_GRAM: SpecificArea = 1000.0 * SQUARE_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerYottagram` (m³/Yg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_YOTTAGRAM: SpecificVolume =
        1.0000000000000001e-21 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerZettagram` (m³/Zg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_ZETTAGRAM: SpecificVolume =
        1.0000000000000001e-18 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerExagram` (m³/Eg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_EXAGRAM: SpecificVolume =
        1.0000000000000001e-15 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerPetagram` (m³/Pg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_PETAGRAM: SpecificVolume =
        9.9999999999999998e-13 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerTeragram` (m³/Tg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TERAGRAM: SpecificVolume =
        1.0000000000000001e-09 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerGigagram` (m³/Gg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_GIGAGRAM: SpecificVolume =
        9.9999999999999995e-07 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerMegagram` (m³/Mg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_MEGAGRAM: SpecificVolume = 0.001 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerKilogram` (m³/kg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_KILOGRAM: SpecificVolume = METRE * METRE * METRE / KILOGRAM;

    /// One `CubicMetrePerHectogram` (m³/hg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_HECTOGRAM: SpecificVolume = 10.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerDecagram` (m³/dag), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_DECAGRAM: SpecificVolume = 100.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerGram` (m³/g), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_GRAM: SpecificVolume = 1000.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerDecigram` (m³/dg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_DECIGRAM: SpecificVolume = 10000.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerCentigram` (m³/cg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_CENTIGRAM: SpecificVolume = 100000.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerMilligram` (m³/mg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_MILLIGRAM: SpecificVolume = 1000000.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerMicrogram` (m³/µg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_MICROGRAM: SpecificVolume = 1000000000.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerNanogram` (m³/ng), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_NANOGRAM: SpecificVolume =
        999999999999.99988 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerPicogram` (m³/pg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_PICOGRAM: SpecificVolume =
        1000000000000000.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerFemtogram` (m³/fg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_FEMTOGRAM: SpecificVolume =
        9.9999999999999987e+17 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerAttogram` (m³/ag), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_ATTOGRAM: SpecificVolume =
        9.9999999999999987e+20 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerZeptogram` (m³/zg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_ZEPTOGRAM: SpecificVolume =
        1.0000000000000001e+24 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerYoctogram` (m³/yg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_YOCTOGRAM: SpecificVolume = 1e+27 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerCarat` (m³/ct), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_CARAT: SpecificVolume = 5000.0 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerGrain` (m³/gr), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_GRAIN: SpecificVolume = 15432.358352941432 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerHundredweightLong` (m³/cwt long), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_HUNDREDWEIGHT_LONG: SpecificVolume =
        (1.0 / 50.80234544) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerHundredweightShort` (m³/cwt short), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_HUNDREDWEIGHT_SHORT: SpecificVolume =
        (1.0 / 45.359237) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerOunce` (m³/oz), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_OUNCE: SpecificVolume =
        (1.0 / 0.028349523125) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerOunceTroy` (m³/oz t), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_OUNCE_TROY: SpecificVolume =
        (1.0 / 0.0311034768) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerPennyweight` (m³/dwt), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_PENNYWEIGHT: SpecificVolume =
        (1.0 / 0.00155517384) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerPound` (m³/lb), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_POUND: SpecificVolume = 2.2046226218487757 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerPoundTroy` (m³/lb t), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_POUND_TROY: SpecificVolume =
        (1.0 / 0.3732417216) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerSlug` (m³/slug), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_SLUG: SpecificVolume =
        (1.0 / (0.45359237 * 9.80665 / 0.3048)) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerTonAssay` (m³/AT), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TON_ASSAY: SpecificVolume =
        (1.0 / (0.175 / 6.0)) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerTonLong` (m³/2240 lb), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TON_LONG: SpecificVolume =
        0.0009842065276110606 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerTonShort` (m³/2000 lb), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TON_SHORT: SpecificVolume =
        0.001102311310924388 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicMetrePerTon` (m³/t), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TON: SpecificVolume = 0.001 * CUBIC_METRE_PER_KILOGRAM;

    /// One `GallonPerGrain` (gal/gr), as a `SpecificVolume`.
    pub const GALLON_PER_GRAIN: SpecificVolume =
        (0.003785411784 / 0.00006479891) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicInchPerOunce` (in³/oz), as a `SpecificVolume`.
    pub const CUBIC_INCH_PER_OUNCE: SpecificVolume =
        (0.0254 * 0.0254 * 0.0254 / 0.028349523125) * CUBIC_METRE_PER_KILOGRAM;

    /// One `GallonImperialPerOunce` (gal (UK)/oz), as a `SpecificVolume`.
    pub const GALLON_IMPERIAL_PER_OUNCE: SpecificVolume =
        (0.00454609 / 0.028349523125) * CUBIC_METRE_PER_KILOGRAM;

    /// One `GallonPerOunce` (gal/oz), as a `SpecificVolume`.
    pub const GALLON_PER_OUNCE: SpecificVolume =
        (0.003785411784 / 0.028349523125) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicFootPerPound` (ft³/lb), as a `SpecificVolume`.
    pub const CUBIC_FOOT_PER_POUND: SpecificVolume = 0.06242796057614461 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicInchPerPound` (in³/lb), as a `SpecificVolume`.
    pub const CUBIC_INCH_PER_POUND: SpecificVolume =
        3.612729200008369e-05 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicYardPerPound` (yd³/lb), as a `SpecificVolume`.
    pub const CUBIC_YARD_PER_POUND: SpecificVolume = 1.6855549355559045 * CUBIC_METRE_PER_KILOGRAM;

    /// One `GallonImperialPerPound` (gal (UK)/lb), as a `SpecificVolume`.
    pub const GALLON_IMPERIAL_PER_POUND: SpecificVolume =
        0.010022412854960501 * CUBIC_METRE_PER_KILOGRAM;

    /// One `GallonPerPound` (gal/lb), as a `SpecificVolume`.
    pub const GALLON_PER_POUND: SpecificVolume = 0.008345404452019332 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicFootPerSlug` (ft³/slug), as a `SpecificVolume`.
    pub const CUBIC_FOOT_PER_SLUG: SpecificVolume =
        (0.3048 * 0.3048 * 0.3048 / (0.45359237 * 9.80665 / 0.3048)) * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicYardPerTonLong` (yd³/2240 lb), as a `SpecificVolume`.
    pub const CUBIC_YARD_PER_TON_LONG: SpecificVolume =
        0.0007524798819446003 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicYardPerTonShort` (yd³/2000 lb), as a `SpecificVolume`.
    pub const CUBIC_YARD_PER_TON_SHORT: SpecificVolume =
        0.0008427774677779523 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicFootPerTonLong` (ft³/2240 lb), as a `SpecificVolume`.
    pub const CUBIC_FOOT_PER_TON_LONG: SpecificVolume =
        2.7869625257207416e-05 * CUBIC_METRE_PER_KILOGRAM;

    /// One `CubicFootPerTonShort` (ft³/2000 lb), as a `SpecificVolume`.
    pub const CUBIC_FOOT_PER_TON_SHORT: SpecificVolume =
        3.121398028807231e-05 * CUBIC_METRE_PER_KILOGRAM;

    /// One `YottawattPerKilogram` (YW/kg), as a `SpecificPower`.
    pub const YOTTAWATT_PER_KILOGRAM: SpecificPower = 9.9999999999999998e+23 * WATT_PER_KILOGRAM;

    /// One `ZettawattPerKilogram` (ZW/kg), as a `SpecificPower`.
    pub const ZETTAWATT_PER_KILOGRAM: SpecificPower = 1e+21 * WATT_PER_KILOGRAM;

    /// One `ExawattPerKilogram` (EW/kg), as a `SpecificPower`.
    pub const EXAWATT_PER_KILOGRAM: SpecificPower = 1e+18 * WATT_PER_KILOGRAM;

    /// One `PetawattPerKilogram` (PW/kg), as a `SpecificPower`.
    pub const PETAWATT_PER_KILOGRAM: SpecificPower = 1000000000000000.0 * WATT_PER_KILOGRAM;

    /// One `TerawattPerKilogram` (TW/kg), as a `SpecificPower`.
    pub const TERAWATT_PER_KILOGRAM: SpecificPower = 1000000000000.0 * WATT_PER_KILOGRAM;

    /// One `GigawattPerKilogram` (GW/kg), as a `SpecificPower`.
    pub const GIGAWATT_PER_KILOGRAM: SpecificPower = 1000000000.0 * WATT_PER_KILOGRAM;

    /// One `MegawattPerKilogram` (MW/kg), as a `SpecificPower`.
    pub const MEGAWATT_PER_KILOGRAM: SpecificPower = 1000000.0 * WATT_PER_KILOGRAM;

    /// One `KilowattPerKilogram` (kW/kg), as a `SpecificPower`.
    pub const KILOWATT_PER_KILOGRAM: SpecificPower = 1000.0 * WATT_PER_KILOGRAM;

    /// One `HectowattPerKilogram` (hW/kg), as a `SpecificPower`.
    pub const HECTOWATT_PER_KILOGRAM: SpecificPower = 100.0 * WATT_PER_KILOGRAM;

    /// One `DecawattPerKilogram` (daW/kg), as a `SpecificPower`.
    pub const DECAWATT_PER_KILOGRAM: SpecificPower = 10.0 * WATT_PER_KILOGRAM;

    /// One `WattPerKilogram` (W/kg), as a `SpecificPower`.
    pub const WATT_PER_KILOGRAM: SpecificPower = METRE * METRE / (SECOND * SECOND * SECOND);

    /// One `DeciwattPerKilogram` (dW/kg), as a `SpecificPower`.
    pub const DECIWATT_PER_KILOGRAM: SpecificPower = 0.10000000000000001 * WATT_PER_KILOGRAM;

    /// One `CentiwattPerKilogram` (cW/kg), as a `SpecificPower`.
    pub const CENTIWATT_PER_KILOGRAM: SpecificPower = 0.01 * WATT_PER_KILOGRAM;

    /// One `MilliwattPerKilogram` (mW/kg), as a `SpecificPower`.
    pub const MILLIWATT_PER_KILOGRAM: SpecificPower = 0.001 * WATT_PER_KILOGRAM;

    /// One `MicrowattPerKilogram` (µW/kg), as a `SpecificPower`.
    pub const MICROWATT_PER_KILOGRAM: SpecificPower = 9.9999999999999995e-07 * WATT_PER_KILOGRAM;

    /// One `NanowattPerKilogram` (nW/kg), as a `SpecificPower`.
    pub const NANOWATT_PER_KILOGRAM: SpecificPower = 1.0000000000000001e-09 * WATT_PER_KILOGRAM;

    /// One `PicowattPerKilogram` (pW/kg), as a `SpecificPower`.
    pub const PICOWATT_PER_KILOGRAM: SpecificPower = 9.9999999999999998e-13 * WATT_PER_KILOGRAM;

    /// One `FemtowattPerKilogram` (fW/kg), as a `SpecificPower`.
    pub const FEMTOWATT_PER_KILOGRAM: SpecificPower = 1.0000000000000001e-15 * WATT_PER_KILOGRAM;

    /// One `AttowattPerKilogram` (aW/kg), as a `SpecificPower`.
    pub const ATTOWATT_PER_KILOGRAM: SpecificPower = 1.0000000000000001e-18 * WATT_PER_KILOGRAM;

    /// One `ZeptowattPerKilogram` (zW/kg), as a `SpecificPower`.
    pub const ZEPTOWATT_PER_KILOGRAM: SpecificPower = 9.9999999999999991e-22 * WATT_PER_KILOGRAM;

    /// One `YoctowattPerKilogram` (yW/kg), as a `SpecificPower`.
    pub const YOCTOWATT_PER_KILOGRAM: SpecificPower = 9.9999999999999992e-25 * WATT_PER_KILOGRAM;

    /// One `SquareMetrePerCubicSecond` (m² · s⁻³), as a `SpecificPower`.
    pub const SQUARE_METRE_PER_CUBIC_SECOND: SpecificPower = WATT_PER_KILOGRAM;

    /// One `ErgPerSecondPerKilogram` ((erg/s)/kg), as a `SpecificPower`.
    pub const ERG_PER_SECOND_PER_KILOGRAM: SpecificPower =
        9.9999999999999995e-08 * WATT_PER_KILOGRAM;

    /// One `FootPoundPerHourPerPound` ((ft · lbf/h)/lb), as a `SpecificPower`.
    pub const FOOT_POUND_PER_HOUR_PER_POUND: SpecificPower =
        ((0.3048 * 0.45359237 * 9.80665) / (3600.0 * 0.45359237)) * WATT_PER_KILOGRAM;

    /// One `FootPoundPerMinutePerPound` ((ft · lbf/min)/lb), as a `SpecificPower`.
    pub const FOOT_POUND_PER_MINUTE_PER_POUND: SpecificPower =
        ((0.3048 * 0.45359237 * 9.80665) / (60.0 * 0.45359237)) * WATT_PER_KILOGRAM;

    /// One `FootPoundPerSecondPerPound` ((ft · lbf/s)/lb), as a `SpecificPower`.
    pub const FOOT_POUND_PER_SECOND_PER_POUND: SpecificPower =
        ((0.3048 * 0.45359237 * 9.80665) / 0.45359237) * WATT_PER_KILOGRAM;

    /// One `HorsepowerPerPound` (hp/lb), as a `SpecificPower`.
    pub const HORSEPOWER_PER_POUND: SpecificPower =
        ((550.0 * 0.3048 * 0.45359237 * 9.80665) / 0.45359237) * WATT_PER_KILOGRAM;

    /// One `HorsepowerBoilerPerPound` ((hp (S))/lb), as a `SpecificPower`.
    pub const HORSEPOWER_BOILER_PER_POUND: SpecificPower = 21626.245609025565 * WATT_PER_KILOGRAM;

    /// One `HorsepowerElectricPerPound` ((hp (E))/lb), as a `SpecificPower`.
    pub const HORSEPOWER_ELECTRIC_PER_POUND: SpecificPower = 1644.6484758991867 * WATT_PER_KILOGRAM;

    /// One `HorsepowerMetricPerPound` ((hp (M))/lb), as a `SpecificPower`.
    pub const HORSEPOWER_METRIC_PER_POUND: SpecificPower =
        ((75.0 * 9.80665) / 0.45359237) * WATT_PER_KILOGRAM;

    /// One `HorsepowerImperialPerPound` ((hp (I))/lb), as a `SpecificPower`.
    pub const HORSEPOWER_IMPERIAL_PER_POUND: SpecificPower = 1643.9870891126322 * WATT_PER_KILOGRAM;

    /// One `HydraulicHorsepowerPerPound` ((hp (hydraulic))/lb), as a `SpecificPower`.
    pub const HYDRAULIC_HORSEPOWER_PER_POUND: SpecificPower =
        1644.7432746719262 * WATT_PER_KILOGRAM;

    /// One `CubicMetrePerKilogramSecondSquared` (m³/(kg · s²)), as a `SpecificGravitationalParameter`.
    pub const CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED: SpecificGravitationalParameter =
        METRE * METRE * METRE / (KILOGRAM * SECOND * SECOND);

    /// One `GravitationalConstant` (G), as a `SpecificGravitationalParameter`.
    pub const GRAVITATIONAL_CONSTANT: SpecificGravitationalParameter =
        6.67430e-11 * CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED;
}

/// The units of `Length`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length)]
pub enum LengthUnit {
    #[unit("Ym", "yottametres", by = YOTTAMETRE)]
    Yottametre,
    #[unit("Zm", "zettametres", by = ZETTAMETRE)]
    Zettametre,
    #[unit("Em", "exametres", by = EXAMETRE)]
    Exametre,
    #[unit("Pm", "petametres", by = PETAMETRE)]
    Petametre,
    #[unit("Tm", "terametres", by = TERAMETRE)]
    Terametre,
    #[unit("Gm", "gigametres", by = GIGAMETRE)]
    Gigametre,
    #[unit("Mm", "megametres", by = MEGAMETRE)]
    Megametre,
    #[unit("km", "kilometres", by = KILOMETRE)]
    Kilometre,
    #[unit("hm", "hectometres", by = HECTOMETRE)]
    Hectometre,
    #[unit("dam", "decametres", by = DECAMETRE)]
    Decametre,
    #[default]
    #[unit("m", "metres", by = METRE)]
    Metre,
    #[unit("dm", "decimetres", by = DECIMETRE)]
    Decimetre,
    #[unit("cm", "centimetres", by = CENTIMETRE)]
    Centimetre,
    #[unit("mm", "millimetres", by = MILLIMETRE)]
    Millimetre,
    #[unit("µm", "micrometres", by = MICROMETRE)]
    Micrometre,
    #[unit("nm", "nanometres", by = NANOMETRE)]
    Nanometre,
    #[unit("pm", "picometres", by = PICOMETRE)]
    Picometre,
    #[unit("fm", "femtometres", by = FEMTOMETRE)]
    Femtometre,
    #[unit("am", "attometres", by = ATTOMETRE)]
    Attometre,
    #[unit("zm", "zeptometres", by = ZEPTOMETRE)]
    Zeptometre,
    #[unit("ym", "yoctometres", by = YOCTOMETRE)]
    Yoctometre,
    #[unit("Å", "ångströms", by = ANGSTROM)]
    Angstrom,
    #[unit("a₀", "bohr radiuses", by = BOHR_RADIUS)]
    BohrRadius,
    #[unit("a.u. of length", "atomic units of length", by = ATOMIC_UNIT_OF_LENGTH)]
    AtomicUnitOfLength,
    #[unit("ua", "astronomical units", by = ASTRONOMICAL_UNIT)]
    AstronomicalUnit,
    #[unit("ch", "chains", by = CHAIN)]
    Chain,
    #[unit("DM", "data miles", by = DATA_MILE)]
    DataMile,
    #[unit("fathom", "fathoms", by = FATHOM)]
    Fathom,
    #[unit("fermi", "fermis", by = FERMI)]
    Fermi,
    #[unit("ft", "feet", by = FOOT)]
    Foot,
    #[unit("ft (U.S. survey)", "feet (U.S. survey)", by = FOOT_SURVEY)]
    FootSurvey,
    #[unit("in", "inches", by = INCH)]
    Inch,
    #[unit("l. y.", "light years", by = LIGHT_YEAR)]
    LightYear,
    #[unit("μin", "microinches", by = MICROINCH)]
    Microinch,
    #[unit("μ", "microns", by = MICRON)]
    Micron,
    #[unit("0.001 in", "mils", by = MIL)]
    Mil,
    #[unit("mi", "miles", by = MILE)]
    Mile,
    #[unit("mi (U.S. survey)", "miles (U.S. survey)", by = MILE_SURVEY)]
    MileSurvey,
    #[unit("M", "nautical miles", by = NAUTICAL_MILE)]
    NauticalMile,
    #[unit("pc", "parsecs", by = PARSEC)]
    Parsec,
    #[unit("1/6 in (computer)", "picas (computer)", by = PICA_COMPUTER)]
    PicaComputer,
    #[unit("1/6 in", "picas (printer's)", by = PICA_PRINTERS)]
    PicaPrinters,
    #[unit("1/72 in (computer)", "points (computer)", by = POINT_COMPUTER)]
    PointComputer,
    #[unit("1/72 in", "points (printer's)", by = POINT_PRINTERS)]
    PointPrinters,
    #[unit("rd", "rods", by = ROD)]
    Rod,
    #[unit("yd", "yards", by = YARD)]
    Yard,
    #[unit("leuga", "leugae", by = LEUGA)]
    Leuga,
    #[unit("mille passus", "mille passa", by = MILLE_PASSUS)]
    MillePassus,
    #[unit("stadium", "stadia", by = STADIUM)]
    Stadium,
    #[unit("actus", "acta", by = ACTUS)]
    Actus,
    #[unit("decempeda", "decempedae", by = DECEMPEDA)]
    Decempeda,
    #[unit("passus", "passa", by = PASSUS)]
    Passus,
    #[unit("gradus", "gradus", by = GRADUS)]
    Gradus,
    #[unit("cubitum", "cubita", by = CUBITUM)]
    Cubitum,
    #[unit("palmipes", "palmipedes", by = PALMIPES)]
    Palmipes,
    #[unit("pes", "pedes", by = PES)]
    Pes,
    #[unit("palmus maior", "palmas maior", by = PALMUS_MAIOR)]
    PalmusMaior,
    #[unit("palmus", "palmas", by = PALMUS)]
    Palmus,
    #[unit("uncia", "unciae", by = LENGTH_UNCIA)]
    Uncia,
    #[unit("digitus", "digiti", by = DIGITUS)]
    Digitus,
}

/// The units of `Mass`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass)]
pub enum MassUnit {
    #[unit("Yg", "yottagrams", by = YOTTAGRAM)]
    Yottagram,
    #[unit("Zg", "zettagrams", by = ZETTAGRAM)]
    Zettagram,
    #[unit("Eg", "exagrams", by = EXAGRAM)]
    Exagram,
    #[unit("Pg", "petagrams", by = PETAGRAM)]
    Petagram,
    #[unit("Tg", "teragrams", by = TERAGRAM)]
    Teragram,
    #[unit("Gg", "gigagrams", by = GIGAGRAM)]
    Gigagram,
    #[unit("Mg", "megagrams", by = MEGAGRAM)]
    Megagram,
    #[default]
    #[unit("kg", "kilograms", by = KILOGRAM)]
    Kilogram,
    #[unit("hg", "hectograms", by = HECTOGRAM)]
    Hectogram,
    #[unit("dag", "decagrams", by = DECAGRAM)]
    Decagram,
    #[unit("g", "grams", by = GRAM)]
    Gram,
    #[unit("dg", "decigrams", by = DECIGRAM)]
    Decigram,
    #[unit("cg", "centigrams", by = CENTIGRAM)]
    Centigram,
    #[unit("mg", "milligrams", by = MILLIGRAM)]
    Milligram,
    #[unit("µg", "micrograms", by = MICROGRAM)]
    Microgram,
    #[unit("ng", "nanograms", by = NANOGRAM)]
    Nanogram,
    #[unit("pg", "picograms", by = PICOGRAM)]
    Picogram,
    #[unit("fg", "femtograms", by = FEMTOGRAM)]
    Femtogram,
    #[unit("ag", "attograms", by = ATTOGRAM)]
    Attogram,
    #[unit("zg", "zeptograms", by = ZEPTOGRAM)]
    Zeptogram,
    #[unit("yg", "yoctograms", by = YOCTOGRAM)]
    Yoctogram,
    #[unit("ct", "carats", by = CARAT)]
    Carat,
    #[unit("Da", "daltons", by = DALTON)]
    Dalton,
    #[unit("mₑ", "electron masses", by = ELECTRON_MASS)]
    ElectronMass,
    #[unit("mₚ", "proton masses", by = PROTON_MASS)]
    ProtonMass,
    #[unit("gr", "grains", by = GRAIN)]
    Grain,
    #[unit("cwt long", "hundredweight (long)", by = HUNDREDWEIGHT_LONG)]
    HundredweightLong,
    #[unit("cwt short", "hundredweight (short)", by = HUNDREDWEIGHT_SHORT)]
    HundredweightShort,
    #[unit("oz", "ounces", by = OUNCE)]
    Ounce,
    #[unit("oz t", "troy ounces", by = OUNCE_TROY)]
    OunceTroy,
    #[unit("dwt", "pennyweight", by = PENNYWEIGHT)]
    Pennyweight,
    #[unit("lb", "pounds", by = POUND)]
    Pound,
    #[unit("lb t", "troy pounds", by = POUND_TROY)]
    PoundTroy,
    #[unit("slug", "slugs", by = SLUG)]
    Slug,
    #[unit("AT", "assay tons", by = TON_ASSAY)]
    TonAssay,
    #[unit("2240 lb", "long tons", by = TON_LONG)]
    TonLong,
    #[unit("2000 lb", "short tons", by = TON_SHORT)]
    TonShort,
    #[unit("t", "tons", by = TON)]
    Ton,
    #[unit("libra", "librae", by = LIBRA)]
    Libra,
    #[unit("deunx", "deunces", by = MASS_DEUNX)]
    Deunx,
    #[unit("dextans", "dextantes", by = MASS_DEXTANS)]
    Dextans,
    #[unit("dodrans", "dodrantes", by = MASS_DODRANS)]
    Dodrans,
    #[unit("bes", "bessis", by = MASS_BES)]
    Bes,
    #[unit("septunx", "septunces", by = MASS_SEPTUNX)]
    Septunx,
    #[unit("semis", "semisses", by = MASS_SEMIS)]
    Semis,
    #[unit("quincunx", "quincunx", by = MASS_QUINCUNX)]
    Quincunx,
    #[unit("triens", "trientes", by = MASS_TRIENS)]
    Triens,
    #[unit("quadrans teruncius", "quadrantes teruncius", by = QUADRANS_TERUNCIUS)]
    QuadransTeruncius,
    #[unit("sextans", "sextantes", by = MASS_SEXTANS)]
    Sextans,
    #[unit("sescuncia", "sescunciae", by = SESCUNCIA)]
    Sescuncia,
    #[unit("uncia", "unciae", by = MASS_UNCIA)]
    Uncia,
    #[unit("semuncia", "semunciae", by = SEMUNCIA)]
    Semuncia,
    #[unit("duella", "duella", by = DUELLA)]
    Duella,
    #[unit("sicilicus", "scilici", by = MASS_SICILICUS)]
    Sicilicus,
    #[unit("sextula", "sextulae", by = MASS_SEXTULA)]
    Sextula,
    #[unit("semisextula", "semisextulae", by = SEMISEXTULA)]
    Semisextula,
    #[unit("scrupulum", "scrupula", by = MASS_SCRUPULUM)]
    Scrupulum,
    #[unit("obolus", "oboli", by = OBOLUS)]
    Obolus,
    #[unit("siliqua", "siliquae", by = SILIQUA)]
    Siliqua,
}

/// The units of `Time`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time)]
pub enum TimeUnit {
    #[unit("Ys", "yottaseconds", by = YOTTASECOND)]
    Yottasecond,
    #[unit("Zs", "zettaseconds", by = ZETTASECOND)]
    Zettasecond,
    #[unit("Es", "exaseconds", by = EXASECOND)]
    Exasecond,
    #[unit("Ps", "petaseconds", by = PETASECOND)]
    Petasecond,
    #[unit("Ts", "teraseconds", by = TERASECOND)]
    Terasecond,
    #[unit("Gs", "gigaseconds", by = GIGASECOND)]
    Gigasecond,
    #[unit("Ms", "megaseconds", by = MEGASECOND)]
    Megasecond,
    #[unit("ks", "kiloseconds", by = KILOSECOND)]
    Kilosecond,
    #[unit("hs", "hectoseconds", by = HECTOSECOND)]
    Hectosecond,
    #[unit("das", "decaseconds", by = DECASECOND)]
    Decasecond,
    #[default]
    #[unit("s", "seconds", by = SECOND)]
    Second,
    #[unit("ds", "deciseconds", by = DECISECOND)]
    Decisecond,
    #[unit("cs", "centiseconds", by = CENTISECOND)]
    Centisecond,
    #[unit("ms", "milliseconds", by = MILLISECOND)]
    Millisecond,
    #[unit("µs", "microseconds", by = MICROSECOND)]
    Microsecond,
    #[unit("ns", "nanoseconds", by = NANOSECOND)]
    Nanosecond,
    #[unit("ps", "picoseconds", by = PICOSECOND)]
    Picosecond,
    #[unit("fs", "femtoseconds", by = FEMTOSECOND)]
    Femtosecond,
    #[unit("as", "attoseconds", by = ATTOSECOND)]
    Attosecond,
    #[unit("zs", "zeptoseconds", by = ZEPTOSECOND)]
    Zeptosecond,
    #[unit("ys", "yoctoseconds", by = YOCTOSECOND)]
    Yoctosecond,
    #[unit("s (sidereal)", "seconds (sidereal)", by = SECOND_SIDEREAL)]
    SecondSidereal,
    #[unit("d", "days", by = DAY)]
    Day,
    #[unit("d (sidereal)", "days (sidereal)", by = DAY_SIDEREAL)]
    DaySidereal,
    #[unit("h", "hours", by = HOUR)]
    Hour,
    #[unit("h (sidereal)", "hours (sidereal)", by = HOUR_SIDEREAL)]
    HourSidereal,
    #[unit("min", "minutes", by = MINUTE)]
    Minute,
    #[unit("10.0 ns", "shakes", by = SHAKE)]
    Shake,
    #[unit("a", "years", by = YEAR)]
    Year,
    #[unit("a (sidereal)", "years (sidereal)", by = YEAR_SIDEREAL)]
    YearSidereal,
    #[unit("a (tropical)", "years (tropical)", by = YEAR_TROPICAL)]
    YearTropical,
}

/// The units of `Instant`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Instant)]
pub enum InstantUnit {
    #[default]
    #[unit("s Unix", "seconds of Unix time", by = UNIX_SECONDS)]
    UnixSecond,
    #[unit("ms Unix", "milliseconds of Unix time", by = UNIX_MILLISECONDS)]
    UnixMillisecond,
    #[unit("s GPS", "seconds of GPS time", by = GPS_SECONDS)]
    GpsSecond,
    #[unit("JD", "Julian date", by = JULIAN_DATE)]
    JulianDate,
    #[unit("MJD", "modified Julian date", by = MODIFIED_JULIAN_DATE)]
    ModifiedJulianDate,
}

/// The units of `Area`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area)]
pub enum AreaUnit {
    #[unit("Ym²", "square yottametres", by = SQUARE_YOTTAMETRE)]
    SquareYottametre,
    #[unit("Zm²", "square zettametres", by = SQUARE_ZETTAMETRE)]
    SquareZettametre,
    #[unit("Em²", "square exametres", by = SQUARE_EXAMETRE)]
    SquareExametre,
    #[unit("Pm²", "square petametres", by = SQUARE_PETAMETRE)]
    SquarePetametre,
    #[unit("Tm²", "square terametres", by = SQUARE_TERAMETRE)]
    SquareTerametre,
    #[unit("Gm²", "square gigametres", by = SQUARE_GIGAMETRE)]
    SquareGigametre,
    #[unit("Mm²", "square megametres", by = SQUARE_MEGAMETRE)]
    SquareMegametre,
    #[unit("km²", "square kilometres", by = SQUARE_KILOMETRE)]
    SquareKilometre,
    #[unit("hm²", "square hectometres", by = SQUARE_HECTOMETRE)]
    SquareHectometre,
    #[unit("dam²", "square decametres", by = SQUARE_DECAMETRE)]
    SquareDecametre,
    #[default]
    #[unit("m²", "square metres", by = SQUARE_METRE)]
    SquareMetre,
    #[unit("dm²", "square decimetres", by = SQUARE_DECIMETRE)]
    SquareDecimetre,
    #[unit("cm²", "square centimetres", by = SQUARE_CENTIMETRE)]
    SquareCentimetre,
    #[unit("mm²", "square millimetres", by = SQUARE_MILLIMETRE)]
    SquareMillimetre,
    #[unit("µm²", "square micrometres", by = SQUARE_MICROMETRE)]
    SquareMicrometre,
    #[unit("nm²", "square nanometres", by = SQUARE_NANOMETRE)]
    SquareNanometre,
    #[unit("pm²", "square picometres", by = SQUARE_PICOMETRE)]
    SquarePicometre,
    #[unit("fm²", "square femtometres", by = SQUARE_FEMTOMETRE)]
    SquareFemtometre,
    #[unit("am²", "square attometres", by = SQUARE_ATTOMETRE)]
    SquareAttometre,
    #[unit("zm²", "square zeptometres", by = SQUARE_ZEPTOMETRE)]
    SquareZeptometre,
    #[unit("ym²", "square yoctometres", by = SQUARE_YOCTOMETRE)]
    SquareYoctometre,
    #[unit("ac", "acres", by = ACRE)]
    Acre,
    #[unit("a", "ares", by = ARE)]
    Are,
    #[unit("b", "barns", by = BARN)]
    Barn,
    #[unit("cmil", "circular mils", by = CIRCULAR_MIL)]
    CircularMil,
    #[unit("ha", "hectares", by = HECTARE)]
    Hectare,
    #[unit("ft²", "square feet", by = SQUARE_FOOT)]
    SquareFoot,
    #[unit("in²", "square inches", by = SQUARE_INCH)]
    SquareInch,
    #[unit("mi²", "square miles", by = SQUARE_MILE)]
    SquareMile,
    #[unit("yd²", "square yards", by = SQUARE_YARD)]
    SquareYard,
    #[unit("saltus", "saltus", by = SALTUS)]
    Saltus,
    #[unit("centuria", "centuriae", by = CENTURIA)]
    Centuria,
    #[unit("heredium", "heredia", by = HEREDIUM)]
    Heredium,
    #[unit("jugerum", "jugera", by = JUGERUM)]
    Jugerum,
    #[unit("deunx", "deunces", by = AREA_DEUNX)]
    Deunx,
    #[unit("dextans", "dextantes", by = AREA_DEXTANS)]
    Dextans,
    #[unit("dodrans", "dodrantes", by = AREA_DODRANS)]
    Dodrans,
    #[unit("bes", "besses", by = AREA_BES)]
    Bes,
    #[unit("septunx", "septunces", by = AREA_SEPTUNX)]
    Septunx,
    #[unit("semis", "semisses", by = AREA_SEMIS)]
    Semis,
    #[unit("actus quadratus", "acta quadratus", by = ACTUS_QUADRATUS)]
    ActusQuadratus,
    #[unit("quincunx", "quincunces", by = AREA_QUINCUNX)]
    Quincunx,
    #[unit("triens", "trientes", by = AREA_TRIENS)]
    Triens,
    #[unit("quadrans", "quadrantes", by = QUADRANS)]
    Quadrans,
    #[unit("sextans", "sextantes", by = AREA_SEXTANS)]
    Sextans,
    #[unit("clima", "climata", by = CLIMA)]
    Clima,
    #[unit("uncia", "unciae", by = AREA_UNCIA)]
    Uncia,
    #[unit("semiuncia", "semiunciae", by = SEMIUNCIA)]
    Semiuncia,
    #[unit("sicilicus", "siculi", by = AREA_SICILICUS)]
    Sicilicus,
    #[unit("actus simplex", "acta simplex", by = ACTUS_SIMPLEX)]
    ActusSimplex,
    #[unit("sextula", "sextulae", by = AREA_SEXTULA)]
    Sextula,
    #[unit("duo scrupulum", "duo scrupula", by = DUO_SCRUPULUM)]
    DuoScrupulum,
    #[unit("scrupulum", "scrupula", by = AREA_SCRUPULUM)]
    Scrupulum,
    #[unit("dimidium scrupulum", "dimidium scupula", by = DIMIDIUM_SCRUPULUM)]
    DimidiumScrupulum,
    #[unit("pes quadratus", "pedes quadratus", by = PES_QUADRATUS)]
    PesQuadratus,
}

/// The units of `Volume`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume)]
pub enum VolumeUnit {
    #[default]
    #[unit("m³", "cubic metres", by = CUBIC_METRE)]
    CubicMetre,
    #[unit("ac · ft", "acre-feet", by = ACRE_FOOT)]
    AcreFoot,
    #[unit("bbl", "barrels", by = BARREL)]
    Barrel,
    #[unit("bu", "bushels", by = BUSHEL)]
    Bushel,
    #[unit("cords", "cords", by = CORD)]
    Cord,
    #[unit("ft³", "cubic feet", by = CUBIC_FOOT)]
    CubicFoot,
    #[unit("in³", "cubic inches", by = CUBIC_INCH)]
    CubicInch,
    #[unit("mi³", "cubic miles", by = CUBIC_MILE)]
    CubicMile,
    #[unit("yd³", "cubic yards", by = CUBIC_YARD)]
    CubicYard,
    #[unit("cup", "cups", by = CUP)]
    Cup,
    #[unit("fl oz", "fluid ounces", by = FLUID_OUNCE)]
    FluidOunce,
    #[unit("fl oz (UK)", "Imperial fluid ounces", by = FLUID_OUNCE_IMPERIAL)]
    FluidOunceImperial,
    #[unit("gal (UK)", "Imperial gallons", by = GALLON_IMPERIAL)]
    GallonImperial,
    #[unit("gal", "gallons", by = GALLON)]
    Gallon,
    #[unit("gi (UK)", "Imperial gills", by = GILL_IMPERIAL)]
    GillImperial,
    #[unit("gi", "gills", by = GILL)]
    Gill,
    #[unit("YL", "yottalitres", by = YOTTALITRE)]
    Yottalitre,
    #[unit("ZL", "zettalitres", by = ZETTALITRE)]
    Zettalitre,
    #[unit("EL", "exalitres", by = EXALITRE)]
    Exalitre,
    #[unit("PL", "petalitres", by = PETALITRE)]
    Petalitre,
    #[unit("TL", "teralitres", by = TERALITRE)]
    Teralitre,
    #[unit("GL", "gigalitres", by = GIGALITRE)]
    Gigalitre,
    #[unit("ML", "megalitres", by = MEGALITRE)]
    Megalitre,
    #[unit("kL", "kilolitres", by = KILOLITRE)]
    Kilolitre,
    #[unit("hL", "hectolitres", by = HECTOLITRE)]
    Hectolitre,
    #[unit("daL", "decalitres", by = DECALITRE)]
    Decalitre,
    #[unit("L", "litres", by = LITRE)]
    Litre,
    #[unit("dL", "decilitres", by = DECILITRE)]
    Decilitre,
    #[unit("cL", "centilitres", by = CENTILITRE)]
    Centilitre,
    #[unit("mL", "millilitres", by = MILLILITRE)]
    Millilitre,
    #[unit("µL", "microlitres", by = MICROLITRE)]
    Microlitre,
    #[unit("nL", "nanolitres", by = NANOLITRE)]
    Nanolitre,
    #[unit("pL", "picolitres", by = PICOLITRE)]
    Picolitre,
    #[unit("fL", "femtolitres", by = FEMTOLITRE)]
    Femtolitre,
    #[unit("aL", "attolitres", by = ATTOLITRE)]
    Attolitre,
    #[unit("zL", "zeptolitres", by = ZEPTOLITRE)]
    Zeptolitre,
    #[unit("yL", "yoctolitres", by = YOCTOLITRE)]
    Yoctolitre,
    #[unit("pk", "pecks", by = PECK)]
    Peck,
    #[unit("dry pt", "dry pints", by = PINT_DRY)]
    PintDry,
    #[unit("liq pt", "liquid pints", by = PINT_LIQUID)]
    PintLiquid,
    #[unit("dry qt", "dry quarts", by = QUART_DRY)]
    QuartDry,
    #[unit("liq qt", "liquid quarts", by = QUART_LIQUID)]
    QuartLiquid,
    #[unit("st", "steres", by = STERE)]
    Stere,
    #[unit("tbsp", "tablespoons", by = TABLESPOON)]
    Tablespoon,
    #[unit("tsp", "teaspoons", by = TEASPOON)]
    Teaspoon,
    #[unit("RT", "register tons", by = REGISTER_TON)]
    RegisterTon,
    #[unit("culeus", "culei", by = CULEUS)]
    Culeus,
    #[unit("amphora quadrantal", "amphora quadrantal", by = AMPHORA_QUADRANTAL)]
    AmphoraQuadrantal,
    #[unit("urna", "urnae", by = URNA)]
    Urna,
    #[unit("modius castrensis", "modii castrensis", by = MODIUS_CASTRENSIS)]
    ModiusCastrensis,
    #[unit("modius", "modii", by = MODIUS)]
    Modius,
    #[unit("semimodius", "semimodii", by = SEMIMODIUS)]
    Semimodius,
    #[unit("congius", "congii", by = CONGIUS)]
    Congius,
    #[unit("sextarius", "sextarii", by = SEXTARIUS)]
    Sextarius,
    #[unit("hemina", "heminae", by = HEMINA)]
    Hemina,
    #[unit("quartarius", "quartarii", by = QUARTARIUS)]
    Quartarius,
    #[unit("acetabulum", "acetabula", by = ACETABULUM)]
    Acetabulum,
    #[unit("cyathus", "cyathi", by = CYATHUS)]
    Cyathus,
    #[unit("ligula", "ligula", by = LIGULA)]
    Ligula,
}

/// The units of `Velocity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity)]
pub enum VelocityUnit {
    #[unit("Ym/s", "yottametres per second", by = YOTTAMETRE_PER_SECOND)]
    YottametrePerSecond,
    #[unit("Zm/s", "zettametres per second", by = ZETTAMETRE_PER_SECOND)]
    ZettametrePerSecond,
    #[unit("Em/s", "exametres per second", by = EXAMETRE_PER_SECOND)]
    ExametrePerSecond,
    #[unit("Pm/s", "petametres per second", by = PETAMETRE_PER_SECOND)]
    PetametrePerSecond,
    #[unit("Tm/s", "terametres per second", by = TERAMETRE_PER_SECOND)]
    TerametrePerSecond,
    #[unit("Gm/s", "gigametres per second", by = GIGAMETRE_PER_SECOND)]
    GigametrePerSecond,
    #[unit("Mm/s", "megametres per second", by = MEGAMETRE_PER_SECOND)]
    MegametrePerSecond,
    #[unit("km/s", "kilometres per second", by = KILOMETRE_PER_SECOND)]
    KilometrePerSecond,
    #[unit("hm/s", "hectometres per second", by = HECTOMETRE_PER_SECOND)]
    HectometrePerSecond,
    #[unit("dam/s", "decametres per second", by = DECAMETRE_PER_SECOND)]
    DecametrePerSecond,
    #[default]
    #[unit("m/s", "metres per second", by = METRE_PER_SECOND)]
    MetrePerSecond,
    #[unit("dm/s", "decimetres per second", by = DECIMETRE_PER_SECOND)]
    DecimetrePerSecond,
    #[unit("cm/s", "centimetres per second", by = CENTIMETRE_PER_SECOND)]
    CentimetrePerSecond,
    #[unit("mm/s", "millimetres per second", by = MILLIMETRE_PER_SECOND)]
    MillimetrePerSecond,
    #[unit("µm/s", "micrometres per second", by = MICROMETRE_PER_SECOND)]
    MicrometrePerSecond,
    #[unit("nm/s", "nanometres per second", by = NANOMETRE_PER_SECOND)]
    NanometrePerSecond,
    #[unit("pm/s", "picometres per second", by = PICOMETRE_PER_SECOND)]
    PicometrePerSecond,
    #[unit("fm/s", "femtometres per second", by = FEMTOMETRE_PER_SECOND)]
    FemtometrePerSecond,
    #[unit("am/s", "attometres per second", by = ATTOMETRE_PER_SECOND)]
    AttometrePerSecond,
    #[unit("zm/s", "zeptometres per second", by = ZEPTOMETRE_PER_SECOND)]
    ZeptometrePerSecond,
    #[unit("ym/s", "yoctometres per second", by = YOCTOMETRE_PER_SECOND)]
    YoctometrePerSecond,
    #[unit("ft/h", "feet per hour", by = FOOT_PER_HOUR)]
    FootPerHour,
    #[unit("ft/min", "feet per minute", by = FOOT_PER_MINUTE)]
    FootPerMinute,
    #[unit("ft/s", "feet per second", by = FOOT_PER_SECOND)]
    FootPerSecond,
    #[unit("in/s", "inches per second", by = INCH_PER_SECOND)]
    InchPerSecond,
    #[unit("in/min", "inches per minute", by = INCH_PER_MINUTE)]
    InchPerMinute,
    #[unit("km/h", "kilometres per hour", by = KILOMETRE_PER_HOUR)]
    KilometrePerHour,
    #[unit("kn", "knots", by = KNOT)]
    Knot,
    #[unit("mi/h", "miles per hour", by = MILE_PER_HOUR)]
    MilePerHour,
    #[unit("mi/min", "miles per minute", by = MILE_PER_MINUTE)]
    MilePerMinute,
    #[unit("mi/s", "miles per second", by = MILE_PER_SECOND)]
    MilePerSecond,
    #[unit("mm/min", "millimetres per minute", by = MILLIMETRE_PER_MINUTE)]
    MillimetrePerMinute,
    #[unit("a₀ · Eₕ/ħ", "atomic units of velocity", by = ATOMIC_UNIT_OF_VELOCITY)]
    AtomicUnitOfVelocity,
    #[unit("c", "natural units of velocity", by = NATURAL_UNIT_OF_VELOCITY)]
    NaturalUnitOfVelocity,
    #[unit("c", "speeds of light in vacuum", by = SPEED_OF_LIGHT)]
    SpeedOfLight,
}

/// The units of `Acceleration`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration)]
pub enum AccelerationUnit {
    #[unit("Ym/s²", "yottametres per second squared", by = YOTTAMETRE_PER_SECOND_SQUARED)]
    YottametrePerSecondSquared,
    #[unit("Zm/s²", "zettametres per second squared", by = ZETTAMETRE_PER_SECOND_SQUARED)]
    ZettametrePerSecondSquared,
    #[unit("Em/s²", "exametres per second squared", by = EXAMETRE_PER_SECOND_SQUARED)]
    ExametrePerSecondSquared,
    #[unit("Pm/s²", "petametres per second squared", by = PETAMETRE_PER_SECOND_SQUARED)]
    PetametrePerSecondSquared,
    #[unit("Tm/s²", "terametres per second squared", by = TERAMETRE_PER_SECOND_SQUARED)]
    TerametrePerSecondSquared,
    #[unit("Gm/s²", "gigametres per second squared", by = GIGAMETRE_PER_SECOND_SQUARED)]
    GigametrePerSecondSquared,
    #[unit("Mm/s²", "megametres per second squared", by = MEGAMETRE_PER_SECOND_SQUARED)]
    MegametrePerSecondSquared,
    #[unit("km/s²", "kilometres per second squared", by = KILOMETRE_PER_SECOND_SQUARED)]
    KilometrePerSecondSquared,
    #[unit("hm/s²", "hectometres per second squared", by = HECTOMETRE_PER_SECOND_SQUARED)]
    HectometrePerSecondSquared,
    #[unit("dam/s²", "decametres per second squared", by = DECAMETRE_PER_SECOND_SQUARED)]
    DecametrePerSecondSquared,
    #[default]
    #[unit("m/s²", "metres per second squared", by = METRE_PER_SECOND_SQUARED)]
    MetrePerSecondSquared,
    #[unit("dm/s²", "decimetres per second squared", by = DECIMETRE_PER_SECOND_SQUARED)]
    DecimetrePerSecondSquared,
    #[unit("cm/s²", "centimetres per second squared", by = CENTIMETRE_PER_SECOND_SQUARED)]
    CentimetrePerSecondSquared,
    #[unit("mm/s²", "millimetres per second squared", by = MILLIMETRE_PER_SECOND_SQUARED)]
    MillimetrePerSecondSquared,
    #[unit("µm/s²", "micrometres per second squared", by = MICROMETRE_PER_SECOND_SQUARED)]
    MicrometrePerSecondSquared,
    #[unit("nm/s²", "nanometres per second squared", by = NANOMETRE_PER_SECOND_SQUARED)]
    NanometrePerSecondSquared,
    #[unit("pm/s²", "picometres per second squared", by = PICOMETRE_PER_SECOND_SQUARED)]
    PicometrePerSecondSquared,
    #[unit("fm/s²", "femtometres per second squared", by = FEMTOMETRE_PER_SECOND_SQUARED)]
    FemtometrePerSecondSquared,
    #[unit("am/s²", "attometres per second squared", by = ATTOMETRE_PER_SECOND_SQUARED)]
    AttometrePerSecondSquared,
    #[unit("zm/s²", "zeptometres per second squared", by = ZEPTOMETRE_PER_SECOND_SQUARED)]
    ZeptometrePerSecondSquared,
    #[unit("ym/s²", "yoctometres per second squared", by = YOCTOMETRE_PER_SECOND_SQUARED)]
    YoctometrePerSecondSquared,
    #[unit("ft/s²", "feet per second squared", by = FOOT_PER_SECOND_SQUARED)]
    FootPerSecondSquared,
    #[unit("Gal", "galileos", by = GALILEO)]
    Galileo,
    #[unit("in/s²", "inches per second squared", by = INCH_PER_SECOND_SQUARED)]
    InchPerSecondSquared,
    #[unit("mm/min²", "millimetres per minute squared", by = MILLIMETRE_PER_MINUTE_SQUARED)]
    MillimetrePerMinuteSquared,
    #[unit("g₀", "standard accelerations of gravity", by = STANDARD_GRAVITY)]
    StandardGravity,
}

/// The units of `Jerk`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Jerk)]
pub enum JerkUnit {
    #[unit("Ym/s³", "yottametres per second cubed", by = YOTTAMETRE_PER_SECOND_CUBED)]
    YottametrePerSecondCubed,
    #[unit("Zm/s³", "zettametres per second cubed", by = ZETTAMETRE_PER_SECOND_CUBED)]
    ZettametrePerSecondCubed,
    #[unit("Em/s³", "exametres per second cubed", by = EXAMETRE_PER_SECOND_CUBED)]
    ExametrePerSecondCubed,
    #[unit("Pm/s³", "petametres per second cubed", by = PETAMETRE_PER_SECOND_CUBED)]
    PetametrePerSecondCubed,
    #[unit("Tm/s³", "terametres per second cubed", by = TERAMETRE_PER_SECOND_CUBED)]
    TerametrePerSecondCubed,
    #[unit("Gm/s³", "gigametres per second cubed", by = GIGAMETRE_PER_SECOND_CUBED)]
    GigametrePerSecondCubed,
    #[unit("Mm/s³", "megametres per second cubed", by = MEGAMETRE_PER_SECOND_CUBED)]
    MegametrePerSecondCubed,
    #[unit("km/s³", "kilometres per second cubed", by = KILOMETRE_PER_SECOND_CUBED)]
    KilometrePerSecondCubed,
    #[unit("hm/s³", "hectometres per second cubed", by = HECTOMETRE_PER_SECOND_CUBED)]
    HectometrePerSecondCubed,
    #[unit("dam/s³", "decametres per second cubed", by = DECAMETRE_PER_SECOND_CUBED)]
    DecametrePerSecondCubed,
    #[default]
    #[unit("m/s³", "metres per second cubed", by = METRE_PER_SECOND_CUBED)]
    MetrePerSecondCubed,
    #[unit("dm/s³", "decimetres per second cubed", by = DECIMETRE_PER_SECOND_CUBED)]
    DecimetrePerSecondCubed,
    #[unit("cm/s³", "centimetres per second cubed", by = CENTIMETRE_PER_SECOND_CUBED)]
    CentimetrePerSecondCubed,
    #[unit("mm/s³", "millimetres per second cubed", by = MILLIMETRE_PER_SECOND_CUBED)]
    MillimetrePerSecondCubed,
    #[unit("µm/s³", "micrometres per second cubed", by = MICROMETRE_PER_SECOND_CUBED)]
    MicrometrePerSecondCubed,
    #[unit("nm/s³", "nanometres per second cubed", by = NANOMETRE_PER_SECOND_CUBED)]
    NanometrePerSecondCubed,
    #[unit("pm/s³", "picometres per second cubed", by = PICOMETRE_PER_SECOND_CUBED)]
    PicometrePerSecondCubed,
    #[unit("fm/s³", "femtometres per second cubed", by = FEMTOMETRE_PER_SECOND_CUBED)]
    FemtometrePerSecondCubed,
    #[unit("am/s³", "attometres per second cubed", by = ATTOMETRE_PER_SECOND_CUBED)]
    AttometrePerSecondCubed,
    #[unit("zm/s³", "zeptometres per second cubed", by = ZEPTOMETRE_PER_SECOND_CUBED)]
    ZeptometrePerSecondCubed,
    #[unit("ym/s³", "yoctometres per second cubed", by = YOCTOMETRE_PER_SECOND_CUBED)]
    YoctometrePerSecondCubed,
    #[unit("ft/s³", "feet per second cubed", by = FOOT_PER_SECOND_CUBED)]
    FootPerSecondCubed,
    #[unit("in/s³", "inches per second cubed", by = INCH_PER_SECOND_CUBED)]
    InchPerSecondCubed,
    #[unit("km/min³", "kilometres per minute cubed", by = KILOMETRE_PER_MINUTE_CUBED)]
    KilometrePerMinuteCubed,
}

/// The units of `Absement`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Absement)]
pub enum AbsementUnit {
    #[unit("Ym · s", "yottametre seconds", by = YOTTAMETRE_SECOND)]
    YottametreSecond,
    #[unit("Zm · s", "zettametre seconds", by = ZETTAMETRE_SECOND)]
    ZettametreSecond,
    #[unit("Em · s", "exametre seconds", by = EXAMETRE_SECOND)]
    ExametreSecond,
    #[unit("Pm · s", "petametre seconds", by = PETAMETRE_SECOND)]
    PetametreSecond,
    #[unit("Tm · s", "terametre seconds", by = TERAMETRE_SECOND)]
    TerametreSecond,
    #[unit("Gm · s", "gigametre seconds", by = GIGAMETRE_SECOND)]
    GigametreSecond,
    #[unit("Mm · s", "megametre seconds", by = MEGAMETRE_SECOND)]
    MegametreSecond,
    #[unit("km · s", "kilometre seconds", by = KILOMETRE_SECOND)]
    KilometreSecond,
    #[unit("hm · s", "hectometre seconds", by = HECTOMETRE_SECOND)]
    HectometreSecond,
    #[unit("dam · s", "decametre seconds", by = DECAMETRE_SECOND)]
    DecametreSecond,
    #[default]
    #[unit("m · s", "metre seconds", by = METRE_SECOND)]
    MetreSecond,
    #[unit("dm · s", "decimetre seconds", by = DECIMETRE_SECOND)]
    DecimetreSecond,
    #[unit("cm · s", "centimetre seconds", by = CENTIMETRE_SECOND)]
    CentimetreSecond,
    #[unit("mm · s", "millimetre seconds", by = MILLIMETRE_SECOND)]
    MillimetreSecond,
    #[unit("µm · s", "micrometre seconds", by = MICROMETRE_SECOND)]
    MicrometreSecond,
    #[unit("nm · s", "nanometre seconds", by = NANOMETRE_SECOND)]
    NanometreSecond,
    #[unit("pm · s", "picometre seconds", by = PICOMETRE_SECOND)]
    PicometreSecond,
    #[unit("fm · s", "femtometre seconds", by = FEMTOMETRE_SECOND)]
    FemtometreSecond,
    #[unit("am · s", "attometre seconds", by = ATTOMETRE_SECOND)]
    AttometreSecond,
    #[unit("zm · s", "zeptometre seconds", by = ZEPTOMETRE_SECOND)]
    ZeptometreSecond,
    #[unit("ym · s", "yoctometre seconds", by = YOCTOMETRE_SECOND)]
    YoctometreSecond,
    #[unit("ft · s", "foot seconds", by = FOOT_SECOND)]
    FootSecond,
    #[unit("in · s", "inch seconds", by = INCH_SECOND)]
    InchSecond,
    #[unit("km · h", "kilometre hours", by = KILOMETRE_HOUR)]
    KilometreHour,
}

/// The units of `InverseVelocity`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseVelocity)]
pub enum InverseVelocityUnit {
    #[unit("Ys/m", "yottaseconds per metre", by = YOTTASECOND_PER_METRE)]
    YottasecondPerMetre,
    #[unit("Zs/m", "zettaseconds per metre", by = ZETTASECOND_PER_METRE)]
    ZettasecondPerMetre,
    #[unit("Es/m", "exaseconds per metre", by = EXASECOND_PER_METRE)]
    ExasecondPerMetre,
    #[unit("Ps/m", "petaseconds per metre", by = PETASECOND_PER_METRE)]
    PetasecondPerMetre,
    #[unit("Ts/m", "teraseconds per metre", by = TERASECOND_PER_METRE)]
    TerasecondPerMetre,
    #[unit("Gs/m", "gigaseconds per metre", by = GIGASECOND_PER_METRE)]
    GigasecondPerMetre,
    #[unit("Ms/m", "megaseconds per metre", by = MEGASECOND_PER_METRE)]
    MegasecondPerMetre,
    #[unit("ks/m", "kiloseconds per metre", by = KILOSECOND_PER_METRE)]
    KilosecondPerMetre,
    #[unit("hs/m", "hectoseconds per metre", by = HECTOSECOND_PER_METRE)]
    HectosecondPerMetre,
    #[unit("das/m", "decaseconds per metre", by = DECASECOND_PER_METRE)]
    DecasecondPerMetre,
    #[default]
    #[unit("s/m", "seconds per metre", by = SECOND_PER_METRE)]
    SecondPerMetre,
    #[unit("ds/m", "deciseconds per metre", by = DECISECOND_PER_METRE)]
    DecisecondPerMetre,
    #[unit("cs/m", "centiseconds per metre", by = CENTISECOND_PER_METRE)]
    CentisecondPerMetre,
    #[unit("ms/m", "milliseconds per metre", by = MILLISECOND_PER_METRE)]
    MillisecondPerMetre,
    #[unit("µs/m", "microseconds per metre", by = MICROSECOND_PER_METRE)]
    MicrosecondPerMetre,
    #[unit("ns/m", "nanoseconds per metre", by = NANOSECOND_PER_METRE)]
    NanosecondPerMetre,
    #[unit("ps/m", "picoseconds per metre", by = PICOSECOND_PER_METRE)]
    PicosecondPerMetre,
    #[unit("fs/m", "femtoseconds per metre", by = FEMTOSECOND_PER_METRE)]
    FemtosecondPerMetre,
    #[unit("as/m", "attoseconds per metre", by = ATTOSECOND_PER_METRE)]
    AttosecondPerMetre,
    #[unit("zs/m", "zeptoseconds per metre", by = ZEPTOSECOND_PER_METRE)]
    ZeptosecondPerMetre,
    #[unit("ys/m", "yoctoseconds per metre", by = YOCTOSECOND_PER_METRE)]
    YoctosecondPerMetre,
    #[unit("h/ft", "hours per foot", by = HOUR_PER_FOOT)]
    HourPerFoot,
    #[unit("min/ft", "minutes per foot", by = MINUTE_PER_FOOT)]
    MinutePerFoot,
    #[unit("s/ft", "seconds per foot", by = SECOND_PER_FOOT)]
    SecondPerFoot,
    #[unit("s/in", "seconds per inch", by = SECOND_PER_INCH)]
    SecondPerInch,
    #[unit("min/km", "minutes per kilometre", by = MINUTE_PER_KILOMETRE)]
    MinutePerKilometre,
    #[unit("h/km", "hours per kilometre", by = HOUR_PER_KILOMETRE)]
    HourPerKilometre,
    #[unit("h/mi", "hours per mile", by = HOUR_PER_MILE)]
    HourPerMile,
    #[unit("min/mi", "minutes per mile", by = MINUTE_PER_MILE)]
    MinutePerMile,
    #[unit("s/mi", "seconds per mile", by = SECOND_PER_MILE)]
    SecondPerMile,
}

/// The units of `Force`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force)]
pub enum ForceUnit {
    #[unit("YN", "yottanewtons", by = YOTTANEWTON)]
    Yottanewton,
    #[unit("ZN", "zettanewtons", by = ZETTANEWTON)]
    Zettanewton,
    #[unit("EN", "exanewtons", by = EXANEWTON)]
    Exanewton,
    #[unit("PN", "petanewtons", by = PETANEWTON)]
    Petanewton,
    #[unit("TN", "teranewtons", by = TERANEWTON)]
    Teranewton,
    #[unit("GN", "giganewtons", by = GIGANEWTON)]
    Giganewton,
    #[unit("MN", "meganewtons", by = MEGANEWTON)]
    Meganewton,
    #[unit("kN", "kilonewtons", by = KILONEWTON)]
    Kilonewton,
    #[unit("hN", "hectonewtons", by = HECTONEWTON)]
    Hectonewton,
    #[unit("daN", "decanewtons", by = DECANEWTON)]
    Decanewton,
    #[default]
    #[unit("N", "newtons", by = NEWTON)]
    Newton,
    #[unit("dN", "decinewtons", by = DECINEWTON)]
    Decinewton,
    #[unit("cN", "centinewtons", by = CENTINEWTON)]
    Centinewton,
    #[unit("mN", "millinewtons", by = MILLINEWTON)]
    Millinewton,
    #[unit("µN", "micronewtons", by = MICRONEWTON)]
    Micronewton,
    #[unit("nN", "nanonewtons", by = NANONEWTON)]
    Nanonewton,
    #[unit("pN", "piconewtons", by = PICONEWTON)]
    Piconewton,
    #[unit("fN", "femtonewtons", by = FEMTONEWTON)]
    Femtonewton,
    #[unit("aN", "attonewtons", by = ATTONEWTON)]
    Attonewton,
    #[unit("zN", "zeptonewtons", by = ZEPTONEWTON)]
    Zeptonewton,
    #[unit("yN", "yoctonewtons", by = YOCTONEWTON)]
    Yoctonewton,
    #[unit("dyn", "dynes", by = DYNE)]
    Dyne,
    #[unit("kgf", "kilograms-force", by = KILOGRAM_FORCE)]
    KilogramForce,
    #[unit("kip", "kips", by = KIP)]
    Kip,
    #[unit("ozf", "ounces-force", by = OUNCE_FORCE)]
    OunceForce,
    #[unit("pdl", "poundals", by = POUNDAL)]
    Poundal,
    #[unit("lbf", "pounds-force", by = POUND_FORCE)]
    PoundForce,
    #[unit("2000 lbf", "tons-force", by = TON_FORCE)]
    TonForce,
}

/// The units of `Momentum`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Momentum)]
pub enum MomentumUnit {
    #[unit("Yg · m/s", "yottagram metres per second", by = YOTTAGRAM_METRE_PER_SECOND)]
    YottagramMetrePerSecond,
    #[unit("Zg · m/s", "zettagram metres per second", by = ZETTAGRAM_METRE_PER_SECOND)]
    ZettagramMetrePerSecond,
    #[unit("Eg · m/s", "exagram metres per second", by = EXAGRAM_METRE_PER_SECOND)]
    ExagramMetrePerSecond,
    #[unit("Pg · m/s", "petagram metres per second", by = PETAGRAM_METRE_PER_SECOND)]
    PetagramMetrePerSecond,
    #[unit("Tg · m/s", "teragram metres per second", by = TERAGRAM_METRE_PER_SECOND)]
    TeragramMetrePerSecond,
    #[unit("Gg · m/s", "gigagram metres per second", by = GIGAGRAM_METRE_PER_SECOND)]
    GigagramMetrePerSecond,
    #[unit("Mg · m/s", "megagram metres per second", by = MEGAGRAM_METRE_PER_SECOND)]
    MegagramMetrePerSecond,
    #[default]
    #[unit("kg · m/s", "kilogram metres per second", by = KILOGRAM_METRE_PER_SECOND)]
    KilogramMetrePerSecond,
    #[unit("hg · m/s", "hectogram metres per second", by = HECTOGRAM_METRE_PER_SECOND)]
    HectogramMetrePerSecond,
    #[unit("dag · m/s", "decagram metres per second", by = DECAGRAM_METRE_PER_SECOND)]
    DecagramMetrePerSecond,
    #[unit("g · m/s", "gram metres per second", by = GRAM_METRE_PER_SECOND)]
    GramMetrePerSecond,
    #[unit("dg · m/s", "decigram metres per second", by = DECIGRAM_METRE_PER_SECOND)]
    DecigramMetrePerSecond,
    #[unit("cg · m/s", "centigram metres per second", by = CENTIGRAM_METRE_PER_SECOND)]
    CentigramMetrePerSecond,
    #[unit("mg · m/s", "milligram metres per second", by = MILLIGRAM_METRE_PER_SECOND)]
    MilligramMetrePerSecond,
    #[unit("µg · m/s", "microgram metres per second", by = MICROGRAM_METRE_PER_SECOND)]
    MicrogramMetrePerSecond,
    #[unit("ng · m/s", "nanogram metres per second", by = NANOGRAM_METRE_PER_SECOND)]
    NanogramMetrePerSecond,
    #[unit("pg · m/s", "picogram metres per second", by = PICOGRAM_METRE_PER_SECOND)]
    PicogramMetrePerSecond,
    #[unit("fg · m/s", "femtogram metres per second", by = FEMTOGRAM_METRE_PER_SECOND)]
    FemtogramMetrePerSecond,
    #[unit("ag · m/s", "attogram metres per second", by = ATTOGRAM_METRE_PER_SECOND)]
    AttogramMetrePerSecond,
    #[unit("zg · m/s", "zeptogram metres per second", by = ZEPTOGRAM_METRE_PER_SECOND)]
    ZeptogramMetrePerSecond,
    #[unit("yg · m/s", "yoctogram metres per second", by = YOCTOGRAM_METRE_PER_SECOND)]
    YoctogramMetrePerSecond,
    #[unit("kg · Ym/s", "kilogram yottametres per second", by = KILOGRAM_YOTTAMETRE_PER_SECOND)]
    KilogramYottametrePerSecond,
    #[unit("kg · Zm/s", "kilogram zettametres per second", by = KILOGRAM_ZETTAMETRE_PER_SECOND)]
    KilogramZettametrePerSecond,
    #[unit("kg · Em/s", "kilogram exametres per second", by = KILOGRAM_EXAMETRE_PER_SECOND)]
    KilogramExametrePerSecond,
    #[unit("kg · Pm/s", "kilogram petametres per second", by = KILOGRAM_PETAMETRE_PER_SECOND)]
    KilogramPetametrePerSecond,
    #[unit("kg · Tm/s", "kilogram terametres per second", by = KILOGRAM_TERAMETRE_PER_SECOND)]
    KilogramTerametrePerSecond,
    #[unit("kg · Gm/s", "kilogram gigametres per second", by = KILOGRAM_GIGAMETRE_PER_SECOND)]
    KilogramGigametrePerSecond,
    #[unit("kg · Mm/s", "kilogram megametres per second", by = KILOGRAM_MEGAMETRE_PER_SECOND)]
    KilogramMegametrePerSecond,
    #[unit("kg · km/s", "kilogram kilometres per second", by = KILOGRAM_KILOMETRE_PER_SECOND)]
    KilogramKilometrePerSecond,
    #[unit("kg · hm/s", "kilogram hectometres per second", by = KILOGRAM_HECTOMETRE_PER_SECOND)]
    KilogramHectometrePerSecond,
    #[unit("kg · dam/s", "kilogram decametres per second", by = KILOGRAM_DECAMETRE_PER_SECOND)]
    KilogramDecametrePerSecond,
    #[unit("kg · dm/s", "kilogram decimetres per second", by = KILOGRAM_DECIMETRE_PER_SECOND)]
    KilogramDecimetrePerSecond,
    #[unit("kg · cm/s", "kilogram centimetres per second", by = KILOGRAM_CENTIMETRE_PER_SECOND)]
    KilogramCentimetrePerSecond,
    #[unit("kg · mm/s", "kilogram millimetres per second", by = KILOGRAM_MILLIMETRE_PER_SECOND)]
    KilogramMillimetrePerSecond,
    #[unit("kg · µm/s", "kilogram micrometres per second", by = KILOGRAM_MICROMETRE_PER_SECOND)]
    KilogramMicrometrePerSecond,
    #[unit("kg · nm/s", "kilogram nanometres per second", by = KILOGRAM_NANOMETRE_PER_SECOND)]
    KilogramNanometrePerSecond,
    #[unit("kg · pm/s", "kilogram picometres per second", by = KILOGRAM_PICOMETRE_PER_SECOND)]
    KilogramPicometrePerSecond,
    #[unit("kg · fm/s", "kilogram femtometres per second", by = KILOGRAM_FEMTOMETRE_PER_SECOND)]
    KilogramFemtometrePerSecond,
    #[unit("kg · am/s", "kilogram attometres per second", by = KILOGRAM_ATTOMETRE_PER_SECOND)]
    KilogramAttometrePerSecond,
    #[unit("kg · zm/s", "kilogram zeptometres per second", by = KILOGRAM_ZEPTOMETRE_PER_SECOND)]
    KilogramZeptometrePerSecond,
    #[unit("kg · ym/s", "kilogram yoctometres per second", by = KILOGRAM_YOCTOMETRE_PER_SECOND)]
    KilogramYoctometrePerSecond,
    #[unit("t · m/s", "ton metres per second", by = TON_METRE_PER_SECOND)]
    TonMetrePerSecond,
    #[unit("kg · m/min", "kilogram metres per minute", by = KILOGRAM_METRE_PER_MINUTE)]
    KilogramMetrePerMinute,
    #[unit("kg · m/h", "kilogram metres per hour", by = KILOGRAM_METRE_PER_HOUR)]
    KilogramMetrePerHour,
    #[unit("kg · m/d", "kilogram metres per day", by = KILOGRAM_METRE_PER_DAY)]
    KilogramMetrePerDay,
    #[unit("slug · ft/s", "slug feet per second", by = SLUG_FOOT_PER_SECOND)]
    SlugFootPerSecond,
    #[unit("slug · in/s", "slug inches per second", by = SLUG_INCH_PER_SECOND)]
    SlugInchPerSecond,
    #[unit("lb · ft/s", "pound feet per second", by = POUND_FOOT_PER_SECOND)]
    PoundFootPerSecond,
    #[unit("lb · in/s", "pound inches per second", by = POUND_INCH_PER_SECOND)]
    PoundInchPerSecond,
}

/// The units of `Energy`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy)]
pub enum EnergyUnit {
    #[unit("YJ", "yottajoules", by = YOTTAJOULE)]
    Yottajoule,
    #[unit("ZJ", "zettajoules", by = ZETTAJOULE)]
    Zettajoule,
    #[unit("EJ", "exajoules", by = EXAJOULE)]
    Exajoule,
    #[unit("PJ", "petajoules", by = PETAJOULE)]
    Petajoule,
    #[unit("TJ", "terajoules", by = TERAJOULE)]
    Terajoule,
    #[unit("GJ", "gigajoules", by = GIGAJOULE)]
    Gigajoule,
    #[unit("MJ", "megajoules", by = MEGAJOULE)]
    Megajoule,
    #[unit("kJ", "kilojoules", by = KILOJOULE)]
    Kilojoule,
    #[unit("hJ", "hectojoules", by = HECTOJOULE)]
    Hectojoule,
    #[unit("daJ", "decajoules", by = DECAJOULE)]
    Decajoule,
    #[default]
    #[unit("J", "joules", by = JOULE)]
    Joule,
    #[unit("dJ", "decijoules", by = DECIJOULE)]
    Decijoule,
    #[unit("cJ", "centijoules", by = CENTIJOULE)]
    Centijoule,
    #[unit("mJ", "millijoules", by = MILLIJOULE)]
    Millijoule,
    #[unit("µJ", "microjoules", by = MICROJOULE)]
    Microjoule,
    #[unit("nJ", "nanojoules", by = NANOJOULE)]
    Nanojoule,
    #[unit("pJ", "picojoules", by = PICOJOULE)]
    Picojoule,
    #[unit("fJ", "femtojoules", by = FEMTOJOULE)]
    Femtojoule,
    #[unit("aJ", "attojoules", by = ATTOJOULE)]
    Attojoule,
    #[unit("zJ", "zeptojoules", by = ZEPTOJOULE)]
    Zeptojoule,
    #[unit("yJ", "yoctojoules", by = YOCTOJOULE)]
    Yoctojoule,
    #[unit("PW · h", "petawatt hours", by = PETAWATT_HOUR)]
    PetawattHour,
    #[unit("TW · h", "terawatt hours", by = TERAWATT_HOUR)]
    TerawattHour,
    #[unit("GW · h", "gigawatt hours", by = GIGAWATT_HOUR)]
    GigawattHour,
    #[unit("MW · h", "megawatt hours", by = MEGAWATT_HOUR)]
    MegawattHour,
    #[unit("kW · h", "kilowatt hours", by = KILOWATT_HOUR)]
    KilowattHour,
    #[unit("hW · h", "hectowatt hours", by = HECTOWATT_HOUR)]
    HectowattHour,
    #[unit("daW · h", "decawatt hours", by = DECAWATT_HOUR)]
    DecawattHour,
    #[unit("W · h", "watt hours", by = WATT_HOUR)]
    WattHour,
    #[unit("mW · h", "milliwatt hours", by = MILLIWATT_HOUR)]
    MilliwattHour,
    #[unit("µW · h", "microwatt hours", by = MICROWATT_HOUR)]
    MicrowattHour,
    #[unit("PeV", "petaelectronvolts", by = PETAELECTRONVOLT)]
    Petaelectronvolt,
    #[unit("TeV", "teraelectronvolts", by = TERAELECTRONVOLT)]
    Teraelectronvolt,
    #[unit("GeV", "gigaelectronvolts", by = GIGAELECTRONVOLT)]
    Gigaelectronvolt,
    #[unit("MeV", "megaelectronvolts", by = MEGAELECTRONVOLT)]
    Megaelectronvolt,
    #[unit("keV", "kiloelectronvolts", by = KILOELECTRONVOLT)]
    Kiloelectronvolt,
    #[unit("heV", "hectoelectronvolts", by = HECTOELECTRONVOLT)]
    Hectoelectronvolt,
    #[unit("daeV", "decaelectronvolts", by = DECAELECTRONVOLT)]
    Decaelectronvolt,
    #[unit("eV", "electronvolts", by = ELECTRONVOLT)]
    Electronvolt,
    #[unit("Eₕ", "hartrees", by = HARTREE)]
    Hartree,
    #[unit("Btu (IT)", "British thermal units (IT)", by = BTU_IT)]
    BtuIt,
    #[unit("Btu", "British thermal units", by = BTU)]
    Btu,
    #[unit("cal (IT)", "calories (IT)", by = CALORIE_IT)]
    CalorieIt,
    #[unit("cal", "calories", by = CALORIE)]
    Calorie,
    #[unit("Cal (IT)", "Calories (IT)", by = CALORIE_IT_NUTRITION)]
    CalorieItNutrition,
    #[unit("Cal", "Calories", by = CALORIE_NUTRITION)]
    CalorieNutrition,
    #[unit("erg", "ergs", by = ERG)]
    Erg,
    #[unit("ft · pdl", "foot poundals", by = FOOT_POUNDAL)]
    FootPoundal,
    #[unit("ft · lbf", "foot pounds-force", by = FOOT_POUND)]
    FootPound,
    #[unit("kcal (IT)", "kilocalories (IT)", by = KILOCALORIE_IT)]
    KilocalorieIt,
    #[unit("kcal", "kilocalories", by = KILOCALORIE)]
    Kilocalorie,
    #[unit("10¹⁵ Btu (IT)", "quads", by = QUAD)]
    Quad,
    #[unit("thm (EC)", "therms (EC)", by = THERM_EC)]
    ThermEc,
    #[unit("thm", "therms", by = THERM_US)]
    ThermUs,
    #[unit("t of TNT", "tons of TNT", by = TON_TNT)]
    TonTnt,
    #[unit("W · s", "watt seconds", by = WATT_SECOND)]
    WattSecond,
}

/// The units of `AvailableEnergy`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AvailableEnergy)]
pub enum AvailableEnergyUnit {
    #[unit("YJ/kg", "yottajoules per kilogram", by = YOTTAJOULE_PER_KILOGRAM)]
    YottajoulePerKilogram,
    #[unit("ZJ/kg", "zettajoules per kilogram", by = ZETTAJOULE_PER_KILOGRAM)]
    ZettajoulePerKilogram,
    #[unit("EJ/kg", "exajoules per kilogram", by = EXAJOULE_PER_KILOGRAM)]
    ExajoulePerKilogram,
    #[unit("PJ/kg", "petajoules per kilogram", by = PETAJOULE_PER_KILOGRAM)]
    PetajoulePerKilogram,
    #[unit("TJ/kg", "terajoules per kilogram", by = TERAJOULE_PER_KILOGRAM)]
    TerajoulePerKilogram,
    #[unit("GJ/kg", "gigajoules per kilogram", by = GIGAJOULE_PER_KILOGRAM)]
    GigajoulePerKilogram,
    #[unit("MJ/kg", "megajoules per kilogram", by = MEGAJOULE_PER_KILOGRAM)]
    MegajoulePerKilogram,
    #[unit("kJ/kg", "kilojoules per kilogram", by = KILOJOULE_PER_KILOGRAM)]
    KilojoulePerKilogram,
    #[unit("hJ/kg", "hectojoules per kilogram", by = HECTOJOULE_PER_KILOGRAM)]
    HectojoulePerKilogram,
    #[unit("daJ/kg", "decajoules per kilogram", by = DECAJOULE_PER_KILOGRAM)]
    DecajoulePerKilogram,
    #[default]
    #[unit("J/kg", "joules per kilogram", by = JOULE_PER_KILOGRAM)]
    JoulePerKilogram,
    #[unit("dJ/kg", "decijoules per kilogram", by = DECIJOULE_PER_KILOGRAM)]
    DecijoulePerKilogram,
    #[unit("cJ/kg", "centijoules per kilogram", by = CENTIJOULE_PER_KILOGRAM)]
    CentijoulePerKilogram,
    #[unit("mJ/kg", "millijoules per kilogram", by = MILLIJOULE_PER_KILOGRAM)]
    MillijoulePerKilogram,
    #[unit("µJ/kg", "microjoules per kilogram", by = MICROJOULE_PER_KILOGRAM)]
    MicrojoulePerKilogram,
    #[unit("nJ/kg", "nanojoules per kilogram", by = NANOJOULE_PER_KILOGRAM)]
    NanojoulePerKilogram,
    #[unit("pJ/kg", "picojoules per kilogram", by = PICOJOULE_PER_KILOGRAM)]
    PicojoulePerKilogram,
    #[unit("fJ/kg", "femtojoules per kilogram", by = FEMTOJOULE_PER_KILOGRAM)]
    FemtojoulePerKilogram,
    #[unit("aJ/kg", "attojoules per kilogram", by = ATTOJOULE_PER_KILOGRAM)]
    AttojoulePerKilogram,
    #[unit("zJ/kg", "zeptojoules per kilogram", by = ZEPTOJOULE_PER_KILOGRAM)]
    ZeptojoulePerKilogram,
    #[unit("yJ/kg", "yoctojoules per kilogram", by = YOCTOJOULE_PER_KILOGRAM)]
    YoctojoulePerKilogram,
    #[unit("J/zg", "joules per zeptogram", by = JOULE_PER_ZEPTOGRAM)]
    JoulePerZeptogram,
    #[unit("J/ag", "joules per attogram", by = JOULE_PER_ATTOGRAM)]
    JoulePerAttogram,
    #[unit("J/fg", "joules per femtogram", by = JOULE_PER_FEMTOGRAM)]
    JoulePerFemtogram,
    #[unit("J/pg", "joules per picogram", by = JOULE_PER_PICOGRAM)]
    JoulePerPicogram,
    #[unit("J/ng", "joules per nanogram", by = JOULE_PER_NANOGRAM)]
    JoulePerNanogram,
    #[unit("J/µg", "joules per microgram", by = JOULE_PER_MICROGRAM)]
    JoulePerMicrogram,
    #[unit("J/mg", "joules per milligram", by = JOULE_PER_MILLIGRAM)]
    JoulePerMilligram,
    #[unit("J/g", "joules per gram", by = JOULE_PER_GRAM)]
    JoulePerGram,
    #[unit("J/Mg", "joules per megagram", by = JOULE_PER_MEGAGRAM)]
    JoulePerMegagram,
    #[unit("J/Gg", "joules per gigagram", by = JOULE_PER_GIGAGRAM)]
    JoulePerGigagram,
    #[unit("J/Tg", "joules per teragram", by = JOULE_PER_TERAGRAM)]
    JoulePerTeragram,
    #[unit("J/Pg", "joules per petagram", by = JOULE_PER_PETAGRAM)]
    JoulePerPetagram,
    #[unit("J/Eg", "joules per exagram", by = JOULE_PER_EXAGRAM)]
    JoulePerExagram,
    #[unit("J/Zg", "joules per zettagram", by = JOULE_PER_ZETTAGRAM)]
    JoulePerZettagram,
    #[unit("J/Yg", "joules per yottagram", by = JOULE_PER_YOTTAGRAM)]
    JoulePerYottagram,
    #[unit("Btu (IT)/lb", "British thermal units (IT) per pound", by = BTU_IT_PER_POUND)]
    BtuItPerPound,
    #[unit("Btu/lb", "British thermal units per pound", by = BTU_PER_POUND)]
    BtuPerPound,
    #[unit("cal (IT)/g", "calories (IT) per gram", by = CALORIE_IT_PER_GRAM)]
    CalorieItPerGram,
    #[unit("cal/g", "calories per gram", by = CALORIE_PER_GRAM)]
    CaloriePerGram,
}

/// The units of `Power`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power, non_const)]
pub enum PowerUnit {
    #[unit("YW", "yottawatts", by = YOTTAWATT)]
    Yottawatt,
    #[unit("ZW", "zettawatts", by = ZETTAWATT)]
    Zettawatt,
    #[unit("EW", "exawatts", by = EXAWATT)]
    Exawatt,
    #[unit("PW", "petawatts", by = PETAWATT)]
    Petawatt,
    #[unit("TW", "terawatts", by = TERAWATT)]
    Terawatt,
    #[unit("GW", "gigawatts", by = GIGAWATT)]
    Gigawatt,
    #[unit("MW", "megawatts", by = MEGAWATT)]
    Megawatt,
    #[unit("kW", "kilowatts", by = KILOWATT)]
    Kilowatt,
    #[unit("hW", "hectowatts", by = HECTOWATT)]
    Hectowatt,
    #[unit("daW", "decawatts", by = DECAWATT)]
    Decawatt,
    #[default]
    #[unit("W", "watts", by = WATT)]
    Watt,
    #[unit("dBW", "decibels-watt", by = DBW, via = PowerLevel)]
    DecibelWatt,
    #[unit("dBm", "decibels-milliwatt", by = DBM, via = PowerLevel)]
    DecibelMilliwatt,
    #[unit("dW", "deciwatts", by = DECIWATT)]
    Deciwatt,
    #[unit("cW", "centiwatts", by = CENTIWATT)]
    Centiwatt,
    #[unit("mW", "milliwatts", by = MILLIWATT)]
    Milliwatt,
    #[unit("µW", "microwatts", by = MICROWATT)]
    Microwatt,
    #[unit("nW", "nanowatts", by = NANOWATT)]
    Nanowatt,
    #[unit("pW", "picowatts", by = PICOWATT)]
    Picowatt,
    #[unit("fW", "femtowatts", by = FEMTOWATT)]
    Femtowatt,
    #[unit("aW", "attowatts", by = ATTOWATT)]
    Attowatt,
    #[unit("zW", "zeptowatts", by = ZEPTOWATT)]
    Zeptowatt,
    #[unit("yW", "yoctowatts", by = YOCTOWATT)]
    Yoctowatt,
    #[unit("erg/s", "ergs per second", by = ERG_PER_SECOND)]
    ErgPerSecond,
    #[unit("ft · lbf/h", "foot pounds-force per hour", by = FOOT_POUND_PER_HOUR)]
    FootPoundPerHour,
    #[unit("ft · lbf/min", "foot pounds-force per minute", by = FOOT_POUND_PER_MINUTE)]
    FootPoundPerMinute,
    #[unit("ft · lbf/s", "foot pounds-force per second", by = FOOT_POUND_PER_SECOND)]
    FootPoundPerSecond,
    #[unit("hp", "horsepower", by = HORSEPOWER)]
    Horsepower,
    #[unit("hp (S)", "horsepower (boiler)", by = HORSEPOWER_BOILER)]
    HorsepowerBoiler,
    #[unit("hp (E)", "horsepower (electric)", by = HORSEPOWER_ELECTRIC)]
    HorsepowerElectric,
    #[unit("hp (M)", "metric horsepower", by = HORSEPOWER_METRIC)]
    HorsepowerMetric,
    #[unit("hp (I)", "horsepower (Imperial)", by = HORSEPOWER_IMPERIAL)]
    HorsepowerImperial,
    #[unit("hp (hydraulic)", "hydraulic horsepower", by = HYDRAULIC_HORSEPOWER)]
    HydraulicHorsepower,
}

/// The units of `Gain`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Gain)]
pub enum GainUnit {
    #[default]
    #[unit("B", "bels", by = BEL)]
    Bel,
    #[unit("dB", "decibels", by = DECIBEL)]
    Decibel,
    #[unit("Np", "nepers", by = NEPER)]
    Neper,
}

/// The units of `PowerLevel`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PowerLevel)]
pub enum PowerLevelUnit {
    #[default]
    #[unit("dBW", "decibels-watt", by = DBW)]
    DecibelWatt,
    #[unit("dBm", "decibels-milliwatt", by = DBM)]
    DecibelMilliwatt,
}

/// The units of `PowerRate`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PowerRate)]
pub enum PowerRateUnit {
    #[unit("YW/s", "yottawatts per second", by = YOTTAWATT_PER_SECOND)]
    YottawattPerSecond,
    #[unit("ZW/s", "zettawatts per second", by = ZETTAWATT_PER_SECOND)]
    ZettawattPerSecond,
    #[unit("EW/s", "exawatts per second", by = EXAWATT_PER_SECOND)]
    ExawattPerSecond,
    #[unit("PW/s", "petawatts per second", by = PETAWATT_PER_SECOND)]
    PetawattPerSecond,
    #[unit("TW/s", "terawatts per second", by = TERAWATT_PER_SECOND)]
    TerawattPerSecond,
    #[unit("GW/s", "gigawatts per second", by = GIGAWATT_PER_SECOND)]
    GigawattPerSecond,
    #[unit("MW/s", "megawatts per second", by = MEGAWATT_PER_SECOND)]
    MegawattPerSecond,
    #[unit("kW/s", "kilowatts per second", by = KILOWATT_PER_SECOND)]
    KilowattPerSecond,
    #[unit("hW/s", "hectowatts per second", by = HECTOWATT_PER_SECOND)]
    HectowattPerSecond,
    #[unit("daW/s", "decawatts per second", by = DECAWATT_PER_SECOND)]
    DecawattPerSecond,
    #[default]
    #[unit("W/s", "watts per second", by = WATT_PER_SECOND)]
    WattPerSecond,
    #[unit("dW/s", "deciwatts per second", by = DECIWATT_PER_SECOND)]
    DeciwattPerSecond,
    #[unit("cW/s", "centiwatts per second", by = CENTIWATT_PER_SECOND)]
    CentiwattPerSecond,
    #[unit("mW/s", "milliwatts per second", by = MILLIWATT_PER_SECOND)]
    MilliwattPerSecond,
    #[unit("µW/s", "microwatts per second", by = MICROWATT_PER_SECOND)]
    MicrowattPerSecond,
    #[unit("nW/s", "nanowatts per second", by = NANOWATT_PER_SECOND)]
    NanowattPerSecond,
    #[unit("pW/s", "picowatts per second", by = PICOWATT_PER_SECOND)]
    PicowattPerSecond,
    #[unit("fW/s", "femtowatts per second", by = FEMTOWATT_PER_SECOND)]
    FemtowattPerSecond,
    #[unit("aW/s", "attowatts per second", by = ATTOWATT_PER_SECOND)]
    AttowattPerSecond,
    #[unit("zW/s", "zeptowatts per second", by = ZEPTOWATT_PER_SECOND)]
    ZeptowattPerSecond,
    #[unit("yW/s", "yoctowatts per second", by = YOCTOWATT_PER_SECOND)]
    YoctowattPerSecond,
    #[unit("erg/s²", "ergs per second squared", by = ERG_PER_SECOND_SQUARED)]
    ErgPerSecondSquared,
    #[unit("ft · lbf/s²", "foot pounds-force per second squared", by = FOOT_POUND_PER_SECOND_SQUARED)]
    FootPoundPerSecondSquared,
    #[unit("hp/s", "horsepower per second", by = HORSEPOWER_PER_SECOND)]
    HorsepowerPerSecond,
    #[unit("hp/s (S)", "horsepower per second (boiler)", by = HORSEPOWER_PER_SECOND_BOILER)]
    HorsepowerPerSecondBoiler,
    #[unit("hp/s (E)", "horsepower per second (electric)", by = HORSEPOWER_PER_SECOND_ELECTRIC)]
    HorsepowerPerSecondElectric,
    #[unit("hp/s (M)", "metric horsepower per second", by = HORSEPOWER_PER_SECOND_METRIC)]
    HorsepowerPerSecondMetric,
    #[unit("hp/s (I)", "horsepower per second (Imperial)", by = HORSEPOWER_PER_SECOND_IMPERIAL)]
    HorsepowerPerSecondImperial,
    #[unit("hp/s (hydraulic)", "hydraulic horsepower per second", by = HYDRAULIC_HORSEPOWER_PER_SECOND)]
    HydraulicHorsepowerPerSecond,
}

/// The units of `Action`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Action)]
pub enum ActionUnit {
    #[default]
    #[unit("J · s", "joule seconds", by = JOULE_SECOND)]
    JouleSecond,
    #[unit("ħ", "atomic units of action", by = ATOMIC_UNIT_OF_ACTION)]
    AtomicUnitOfAction,
    #[unit("h", "Planck constants", by = PLANCK_CONSTANT)]
    PlanckConstant,
    #[unit("erg · s", "erg seconds", by = ERG_SECOND)]
    ErgSecond,
    #[unit("eV · s", "electronvolt seconds", by = ELECTRONVOLT_SECOND)]
    ElectronvoltSecond,
}

/// The units of `Torque`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Torque)]
pub enum TorqueUnit {
    #[unit("YN · m", "yottanewton metres", by = YOTTANEWTON_METRE)]
    YottanewtonMetre,
    #[unit("ZN · m", "zettanewton metres", by = ZETTANEWTON_METRE)]
    ZettanewtonMetre,
    #[unit("EN · m", "exanewton metres", by = EXANEWTON_METRE)]
    ExanewtonMetre,
    #[unit("PN · m", "petanewton metres", by = PETANEWTON_METRE)]
    PetanewtonMetre,
    #[unit("TN · m", "teranewton metres", by = TERANEWTON_METRE)]
    TeranewtonMetre,
    #[unit("GN · m", "giganewton metres", by = GIGANEWTON_METRE)]
    GiganewtonMetre,
    #[unit("MN · m", "meganewton metres", by = MEGANEWTON_METRE)]
    MeganewtonMetre,
    #[unit("kN · m", "kilonewton metres", by = KILONEWTON_METRE)]
    KilonewtonMetre,
    #[unit("hN · m", "hectonewton metres", by = HECTONEWTON_METRE)]
    HectonewtonMetre,
    #[unit("daN · m", "decanewton metres", by = DECANEWTON_METRE)]
    DecanewtonMetre,
    #[default]
    #[unit("N · m", "newton metres", by = NEWTON_METRE)]
    NewtonMetre,
    #[unit("dN · m", "decinewton metres", by = DECINEWTON_METRE)]
    DecinewtonMetre,
    #[unit("cN · m", "centinewton metres", by = CENTINEWTON_METRE)]
    CentinewtonMetre,
    #[unit("mN · m", "millinewton metres", by = MILLINEWTON_METRE)]
    MillinewtonMetre,
    #[unit("µN · m", "micronewton metres", by = MICRONEWTON_METRE)]
    MicronewtonMetre,
    #[unit("nN · m", "nanonewton metres", by = NANONEWTON_METRE)]
    NanonewtonMetre,
    #[unit("pN · m", "piconewton metres", by = PICONEWTON_METRE)]
    PiconewtonMetre,
    #[unit("fN · m", "femtonewton metres", by = FEMTONEWTON_METRE)]
    FemtonewtonMetre,
    #[unit("aN · m", "attonewton metres", by = ATTONEWTON_METRE)]
    AttonewtonMetre,
    #[unit("zN · m", "zeptonewton metres", by = ZEPTONEWTON_METRE)]
    ZeptonewtonMetre,
    #[unit("yN · m", "yoctonewton metres", by = YOCTONEWTON_METRE)]
    YoctonewtonMetre,
    #[unit("N · Ym", "newton yottametres", by = NEWTON_YOTTAMETRE)]
    NewtonYottametre,
    #[unit("N · Zm", "newton zettametres", by = NEWTON_ZETTAMETRE)]
    NewtonZettametre,
    #[unit("N · Em", "newton exametres", by = NEWTON_EXAMETRE)]
    NewtonExametre,
    #[unit("N · Pm", "newton petametres", by = NEWTON_PETAMETRE)]
    NewtonPetametre,
    #[unit("N · Tm", "newton terametres", by = NEWTON_TERAMETRE)]
    NewtonTerametre,
    #[unit("N · Gm", "newton gigametres", by = NEWTON_GIGAMETRE)]
    NewtonGigametre,
    #[unit("N · Mm", "newton megametres", by = NEWTON_MEGAMETRE)]
    NewtonMegametre,
    #[unit("N · km", "newton kilometres", by = NEWTON_KILOMETRE)]
    NewtonKilometre,
    #[unit("N · hm", "newton hectometres", by = NEWTON_HECTOMETRE)]
    NewtonHectometre,
    #[unit("N · dam", "newton decametres", by = NEWTON_DECAMETRE)]
    NewtonDecametre,
    #[unit("N · dm", "newton decimetres", by = NEWTON_DECIMETRE)]
    NewtonDecimetre,
    #[unit("N · cm", "newton centimetres", by = NEWTON_CENTIMETRE)]
    NewtonCentimetre,
    #[unit("N · mm", "newton millimetres", by = NEWTON_MILLIMETRE)]
    NewtonMillimetre,
    #[unit("N · µm", "newton micrometres", by = NEWTON_MICROMETRE)]
    NewtonMicrometre,
    #[unit("N · nm", "newton nanometres", by = NEWTON_NANOMETRE)]
    NewtonNanometre,
    #[unit("N · pm", "newton picometres", by = NEWTON_PICOMETRE)]
    NewtonPicometre,
    #[unit("N · fm", "newton femtometres", by = NEWTON_FEMTOMETRE)]
    NewtonFemtometre,
    #[unit("N · am", "newton attometres", by = NEWTON_ATTOMETRE)]
    NewtonAttometre,
    #[unit("N · zm", "newton zeptometres", by = NEWTON_ZEPTOMETRE)]
    NewtonZeptometre,
    #[unit("N · ym", "newton yoctometres", by = NEWTON_YOCTOMETRE)]
    NewtonYoctometre,
    #[unit("dyn · m", "dyne metres", by = DYNE_METRE)]
    DyneMetre,
    #[unit("dyn · cm", "dyne centimetres", by = DYNE_CENTIMETRE)]
    DyneCentimetre,
    #[unit("kgf · m", "kilogram-force metres", by = KILOGRAM_FORCE_METRE)]
    KilogramForceMetre,
    #[unit("ozf · in", "ounces-force inches", by = OUNCE_FORCE_INCH)]
    OunceForceInch,
    #[unit("lbf · ft", "pounds-force feet", by = POUND_FORCE_FOOT)]
    PoundForceFoot,
    #[unit("lbf · in", "pounds-force inches", by = POUND_FORCE_INCH)]
    PoundForceInch,
}

/// The units of `MomentOfInertia`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MomentOfInertia)]
pub enum MomentOfInertiaUnit {
    #[default]
    #[unit("kg · m²", "kilogram square metres", by = KILOGRAM_SQUARE_METRE)]
    KilogramSquareMetre,
    #[unit("Da · Å²", "dalton square ångströms", by = DALTON_SQUARE_ANGSTROM)]
    DaltonSquareAngstrom,
}

/// The units of `Pressure`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure)]
pub enum PressureUnit {
    #[unit("YPa", "yottapascals", by = YOTTAPASCAL)]
    Yottapascal,
    #[unit("ZPa", "zettapascals", by = ZETTAPASCAL)]
    Zettapascal,
    #[unit("EPa", "exapascals", by = EXAPASCAL)]
    Exapascal,
    #[unit("PPa", "petapascals", by = PETAPASCAL)]
    Petapascal,
    #[unit("TPa", "terapascals", by = TERAPASCAL)]
    Terapascal,
    #[unit("GPa", "gigapascals", by = GIGAPASCAL)]
    Gigapascal,
    #[unit("MPa", "megapascals", by = MEGAPASCAL)]
    Megapascal,
    #[unit("kPa", "kilopascals", by = KILOPASCAL)]
    Kilopascal,
    #[unit("hPa", "hectopascals", by = HECTOPASCAL)]
    Hectopascal,
    #[unit("daPa", "decapascals", by = DECAPASCAL)]
    Decapascal,
    #[default]
    #[unit("Pa", "pascals", by = PASCAL)]
    Pascal,
    #[unit("dPa", "decipascals", by = DECIPASCAL)]
    Decipascal,
    #[unit("cPa", "centipascals", by = CENTIPASCAL)]
    Centipascal,
    #[unit("mPa", "millipascals", by = MILLIPASCAL)]
    Millipascal,
    #[unit("µPa", "micropascals", by = MICROPASCAL)]
    Micropascal,
    #[unit("nPa", "nanopascals", by = NANOPASCAL)]
    Nanopascal,
    #[unit("pPa", "picopascals", by = PICOPASCAL)]
    Picopascal,
    #[unit("fPa", "femtopascals", by = FEMTOPASCAL)]
    Femtopascal,
    #[unit("aPa", "attopascals", by = ATTOPASCAL)]
    Attopascal,
    #[unit("zPa", "zeptopascals", by = ZEPTOPASCAL)]
    Zeptopascal,
    #[unit("yPa", "yoctopascals", by = YOCTOPASCAL)]
    Yoctopascal,
    #[unit("atm", "standard atmospheres", by = STANDARD_ATMOSPHERE)]
    StandardAtmosphere,
    #[unit("at", "atmospheres (technical)", by = ATMOSPHERE_TECHNICAL)]
    AtmosphereTechnical,
    #[unit("bar", "bar", by = BAR)]
    Bar,
    #[unit("cm Hg", "centimetres of mercury", by = CENTIMETRE_OF_MERCURY)]
    CentimetreOfMercury,
    #[unit("cm H₂O", "centimetres of water", by = CENTIMETRE_OF_WATER)]
    CentimetreOfWater,
    #[unit("dyn/cm²", "dynes per square centimetre", by = DYNE_PER_SQUARE_CENTIMETRE)]
    DynePerSquareCentimetre,
    #[unit("ft Hg", "feet of mercury", by = FOOT_OF_MERCURY)]
    FootOfMercury,
    #[unit("ft H₂O", "feet of water", by = FOOT_OF_WATER)]
    FootOfWater,
    #[unit("in Hg", "inches of mercury", by = INCH_OF_MERCURY)]
    InchOfMercury,
    #[unit("in H₂O", "inches of water", by = INCH_OF_WATER)]
    InchOfWater,
    #[unit("N/mm²", "newtons per square millimetre", by = NEWTON_PER_SQUARE_MILLIMETRE)]
    NewtonPerSquareMillimetre,
    #[unit("kip/in²", "kips per square inch", by = KIP_PER_SQUARE_INCH)]
    KipPerSquareInch,
    #[unit("mbar", "millibar", by = MILLIBAR)]
    Millibar,
    #[unit("mm Hg", "millimetres of mercury", by = MILLIMETRE_OF_MERCURY)]
    MillimetreOfMercury,
    #[unit("mm H₂O", "millimetres of water", by = MILLIMETRE_OF_WATER)]
    MillimetreOfWater,
    #[unit("mTorr", "millitorr", by = MILLITORR)]
    Millitorr,
    #[unit("pdl/ft²", "poundals per square foot", by = POUNDAL_PER_SQUARE_FOOT)]
    PoundalPerSquareFoot,
    #[unit("lbf/ft²", "pounds-force per square foot", by = POUND_FORCE_PER_SQUARE_FOOT)]
    PoundForcePerSquareFoot,
    #[unit("lbf/in²", "pounds-force per square inch", by = POUND_FORCE_PER_SQUARE_INCH)]
    PoundForcePerSquareInch,
    #[unit("psi", "pounds-force per square inch", by = PSI)]
    Psi,
    #[unit("Torr", "torr", by = TORR)]
    Torr,
}

/// The units of `Frequency`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Frequency)]
pub enum FrequencyUnit {
    #[unit("YHz", "yottahertz", by = YOTTAHERTZ)]
    Yottahertz,
    #[unit("ZHz", "zettahertz", by = ZETTAHERTZ)]
    Zettahertz,
    #[unit("EHz", "exahertz", by = EXAHERTZ)]
    Exahertz,
    #[unit("PHz", "petahertz", by = PETAHERTZ)]
    Petahertz,
    #[unit("THz", "terahertz", by = TERAHERTZ)]
    Terahertz,
    #[unit("GHz", "gigahertz", by = GIGAHERTZ)]
    Gigahertz,
    #[unit("MHz", "megahertz", by = MEGAHERTZ)]
    Megahertz,
    #[unit("kHz", "kilohertz", by = KILOHERTZ)]
    Kilohertz,
    #[unit("hHz", "hectohertz", by = HECTOHERTZ)]
    Hectohertz,
    #[unit("daHz", "decahertz", by = DECAHERTZ)]
    Decahertz,
    #[default]
    #[unit("Hz", "hertz", by = HERTZ)]
    Hertz,
    #[unit("dHz", "decihertz", by = DECIHERTZ)]
    Decihertz,
    #[unit("cHz", "centihertz", by = CENTIHERTZ)]
    Centihertz,
    #[unit("mHz", "millihertz", by = MILLIHERTZ)]
    Millihertz,
    #[unit("µHz", "microhertz", by = MICROHERTZ)]
    Microhertz,
    #[unit("nHz", "nanohertz", by = NANOHERTZ)]
    Nanohertz,
    #[unit("pHz", "picohertz", by = PICOHERTZ)]
    Picohertz,
    #[unit("fHz", "femtohertz", by = FEMTOHERTZ)]
    Femtohertz,
    #[unit("aHz", "attohertz", by = ATTOHERTZ)]
    Attohertz,
    #[unit("zHz", "zeptohertz", by = ZEPTOHERTZ)]
    Zeptohertz,
    #[unit("yHz", "yoctohertz", by = YOCTOHERTZ)]
    Yoctohertz,
    #[unit("1/d", "cycles per day", by = CYCLE_PER_DAY)]
    CyclePerDay,
    #[unit("1/h", "cycles per hour", by = CYCLE_PER_HOUR)]
    CyclePerHour,
    #[unit("1/min", "cycles per minute", by = CYCLE_PER_MINUTE)]
    CyclePerMinute,
    #[unit("100 MHz", "cycles per shake", by = CYCLE_PER_SHAKE)]
    CyclePerShake,
    #[unit("1/a", "cycles per year", by = CYCLE_PER_YEAR)]
    CyclePerYear,
}

/// The units of `FrequencyDrift`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FrequencyDrift)]
pub enum FrequencyDriftUnit {
    #[unit("THz/s", "terahertz per second", by = TERAHERTZ_PER_SECOND)]
    TerahertzPerSecond,
    #[unit("GHz/s", "gigahertz per second", by = GIGAHERTZ_PER_SECOND)]
    GigahertzPerSecond,
    #[unit("MHz/s", "megahertz per second", by = MEGAHERTZ_PER_SECOND)]
    MegahertzPerSecond,
    #[unit("kHz/s", "kilohertz per second", by = KILOHERTZ_PER_SECOND)]
    KilohertzPerSecond,
    #[default]
    #[unit("Hz/s", "hertz per second", by = HERTZ_PER_SECOND)]
    HertzPerSecond,
}

/// The units of `ReciprocalLength`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReciprocalLength)]
pub enum ReciprocalLengthUnit {
    #[unit("km⁻¹", "reciprocal kilometres", by = RECIPROCAL_KILOMETRE)]
    ReciprocalKilometre,
    #[default]
    #[unit("m⁻¹", "reciprocal metres", by = RECIPROCAL_METRE)]
    ReciprocalMetre,
    #[unit("dm⁻¹", "reciprocal decimetres", by = RECIPROCAL_DECIMETRE)]
    ReciprocalDecimetre,
    #[unit("cm⁻¹", "reciprocal centimetres", by = RECIPROCAL_CENTIMETRE)]
    ReciprocalCentimetre,
    #[unit("mm⁻¹", "reciprocal millimetres", by = RECIPROCAL_MILLIMETRE)]
    ReciprocalMillimetre,
    #[unit("µm⁻¹", "reciprocal micrometres", by = RECIPROCAL_MICROMETRE)]
    ReciprocalMicrometre,
    #[unit("nm⁻¹", "reciprocal nanometres", by = RECIPROCAL_NANOMETRE)]
    ReciprocalNanometre,
    #[unit("Å⁻¹", "reciprocal ångströms", by = RECIPROCAL_ANGSTROM)]
    ReciprocalAngstrom,
    #[unit("dpt", "diopters", by = DIOPTER)]
    Diopter,
    #[unit("R∞", "Rydberg constants", by = RYDBERG_CONSTANT)]
    RydbergConstant,
}

/// The units of `Curvature`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Curvature)]
pub enum CurvatureUnit {
    #[default]
    #[unit("rad/m", "radians per metre", by = RADIAN_PER_METRE)]
    RadianPerMetre,
    #[unit("°/m", "degrees per metre", by = DEGREE_PER_METRE)]
    DegreePerMetre,
    #[unit("rad/mm", "radians per millimetre", by = RADIAN_PER_MILLIMETRE)]
    RadianPerMillimetre,
    #[unit("°/mm", "degrees per millimetre", by = DEGREE_PER_MILLIMETRE)]
    DegreePerMillimetre,
}

/// The units of `SpecificArea`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificArea)]
pub enum SpecificAreaUnit {
    #[default]
    #[unit("m²/kg", "square metres per kilogram", by = SQUARE_METRE_PER_KILOGRAM)]
    SquareMetrePerKilogram,
    #[unit("cm²/kg", "square centimetres per kilogram", by = SQUARE_CENTIMETRE_PER_KILOGRAM)]
    SquareCentimetrePerKilogram,
    #[unit("m²/g", "square metres per gram", by = SQUARE_METRE_PER_GRAM)]
    SquareMetrePerGram,
}

/// The units of `SpecificVolume`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificVolume)]
pub enum SpecificVolumeUnit {
    #[unit("m³/Yg", "cubic metres per yottagram", by = CUBIC_METRE_PER_YOTTAGRAM)]
    CubicMetrePerYottagram,
    #[unit("m³/Zg", "cubic metres per zettagram", by = CUBIC_METRE_PER_ZETTAGRAM)]
    CubicMetrePerZettagram,
    #[unit("m³/Eg", "cubic metres per exagram", by = CUBIC_METRE_PER_EXAGRAM)]
    CubicMetrePerExagram,
    #[unit("m³/Pg", "cubic metres per petagram", by = CUBIC_METRE_PER_PETAGRAM)]
    CubicMetrePerPetagram,
    #[unit("m³/Tg", "cubic metres per teragram", by = CUBIC_METRE_PER_TERAGRAM)]
    CubicMetrePerTeragram,
    #[unit("m³/Gg", "cubic metres per gigagram", by = CUBIC_METRE_PER_GIGAGRAM)]
    CubicMetrePerGigagram,
    #[unit("m³/Mg", "cubic metres per megagram", by = CUBIC_METRE_PER_MEGAGRAM)]
    CubicMetrePerMegagram,
    #[default]
    #[unit("m³/kg", "cubic metres per kilogram", by = CUBIC_METRE_PER_KILOGRAM)]
    CubicMetrePerKilogram,
    #[unit("m³/hg", "cubic metres per hectogram", by = CUBIC_METRE_PER_HECTOGRAM)]
    CubicMetrePerHectogram,
    #[unit("m³/dag", "cubic metres per decagram", by = CUBIC_METRE_PER_DECAGRAM)]
    CubicMetrePerDecagram,
    #[unit("m³/g", "cubic metres per gram", by = CUBIC_METRE_PER_GRAM)]
    CubicMetrePerGram,
    #[unit("m³/dg", "cubic metres per decigram", by = CUBIC_METRE_PER_DECIGRAM)]
    CubicMetrePerDecigram,
    #[unit("m³/cg", "cubic metres per centigram", by = CUBIC_METRE_PER_CENTIGRAM)]
    CubicMetrePerCentigram,
    #[unit("m³/mg", "cubic metres per milligram", by = CUBIC_METRE_PER_MILLIGRAM)]
    CubicMetrePerMilligram,
    #[unit("m³/µg", "cubic metres per microgram", by = CUBIC_METRE_PER_MICROGRAM)]
    CubicMetrePerMicrogram,
    #[unit("m³/ng", "cubic metres per nanogram", by = CUBIC_METRE_PER_NANOGRAM)]
    CubicMetrePerNanogram,
    #[unit("m³/pg", "cubic metres per picogram", by = CUBIC_METRE_PER_PICOGRAM)]
    CubicMetrePerPicogram,
    #[unit("m³/fg", "cubic metres per femtogram", by = CUBIC_METRE_PER_FEMTOGRAM)]
    CubicMetrePerFemtogram,
    #[unit("m³/ag", "cubic metres per attogram", by = CUBIC_METRE_PER_ATTOGRAM)]
    CubicMetrePerAttogram,
    #[unit("m³/zg", "cubic metres per zeptogram", by = CUBIC_METRE_PER_ZEPTOGRAM)]
    CubicMetrePerZeptogram,
    #[unit("m³/yg", "cubic metres per yoctogram", by = CUBIC_METRE_PER_YOCTOGRAM)]
    CubicMetrePerYoctogram,
    #[unit("m³/ct", "cubic metres per carat", by = CUBIC_METRE_PER_CARAT)]
    CubicMetrePerCarat,
    #[unit("m³/gr", "cubic metres per grain", by = CUBIC_METRE_PER_GRAIN)]
    CubicMetrePerGrain,
    #[unit("m³/cwt long", "cubic metres per hundredweight (long)", by = CUBIC_METRE_PER_HUNDREDWEIGHT_LONG)]
    CubicMetrePerHundredweightLong,
    #[unit("m³/cwt short", "cubic metres per hundredweight (short)", by = CUBIC_METRE_PER_HUNDREDWEIGHT_SHORT)]
    CubicMetrePerHundredweightShort,
    #[unit("m³/oz", "cubic metres per ounce", by = CUBIC_METRE_PER_OUNCE)]
    CubicMetrePerOunce,
    #[unit("m³/oz t", "cubic metres per troy ounce", by = CUBIC_METRE_PER_OUNCE_TROY)]
    CubicMetrePerOunceTroy,
    #[unit("m³/dwt", "cubic metres per pennyweight", by = CUBIC_METRE_PER_PENNYWEIGHT)]
    CubicMetrePerPennyweight,
    #[unit("m³/lb", "cubic metres per pound", by = CUBIC_METRE_PER_POUND)]
    CubicMetrePerPound,
    #[unit("m³/lb t", "cubic metres per troy pound", by = CUBIC_METRE_PER_POUND_TROY)]
    CubicMetrePerPoundTroy,
    #[unit("m³/slug", "cubic metres per slug", by = CUBIC_METRE_PER_SLUG)]
    CubicMetrePerSlug,
    #[unit("m³/AT", "cubic metres per assay ton", by = CUBIC_METRE_PER_TON_ASSAY)]
    CubicMetrePerTonAssay,
    #[unit("m³/2240 lb", "cubic metres per long ton", by = CUBIC_METRE_PER_TON_LONG)]
    CubicMetrePerTonLong,
    #[unit("m³/2000 lb", "cubic metres per short ton", by = CUBIC_METRE_PER_TON_SHORT)]
    CubicMetrePerTonShort,
    #[unit("m³/t", "cubic metres per ton", by = CUBIC_METRE_PER_TON)]
    CubicMetrePerTon,
    #[unit("gal/gr", "gallons per grain", by = GALLON_PER_GRAIN)]
    GallonPerGrain,
    #[unit("in³/oz", "cubic inches per ounce", by = CUBIC_INCH_PER_OUNCE)]
    CubicInchPerOunce,
    #[unit("gal (UK)/oz", "Imperial gallons per ounce", by = GALLON_IMPERIAL_PER_OUNCE)]
    GallonImperialPerOunce,
    #[unit("gal/oz", "gallons per ounce", by = GALLON_PER_OUNCE)]
    GallonPerOunce,
    #[unit("ft³/lb", "cubic feet per pound", by = CUBIC_FOOT_PER_POUND)]
    CubicFootPerPound,
    #[unit("in³/lb", "cubic inches per pound", by = CUBIC_INCH_PER_POUND)]
    CubicInchPerPound,
    #[unit("yd³/lb", "cubic yards per pound", by = CUBIC_YARD_PER_POUND)]
    CubicYardPerPound,
    #[unit("gal (UK)/lb", "Imperial gallons per pound", by = GALLON_IMPERIAL_PER_POUND)]
    GallonImperialPerPound,
    #[unit("gal/lb", "gallons per pound", by = GALLON_PER_POUND)]
    GallonPerPound,
    #[unit("ft³/slug", "cubic feett per slug", by = CUBIC_FOOT_PER_SLUG)]
    CubicFootPerSlug,
    #[unit("yd³/2240 lb", "cubic yards per long ton", by = CUBIC_YARD_PER_TON_LONG)]
    CubicYardPerTonLong,
    #[unit("yd³/2000 lb", "cubic yards per short ton", by = CUBIC_YARD_PER_TON_SHORT)]
    CubicYardPerTonShort,
    #[unit("ft³/2240 lb", "cubic feet per long ton", by = CUBIC_FOOT_PER_TON_LONG)]
    CubicFootPerTonLong,
    #[unit("ft³/2000 lb", "cubic feet per short ton", by = CUBIC_FOOT_PER_TON_SHORT)]
    CubicFootPerTonShort,
}

/// The units of `SpecificPower`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificPower)]
pub enum SpecificPowerUnit {
    #[unit("YW/kg", "yottawatts per kilogram", by = YOTTAWATT_PER_KILOGRAM)]
    YottawattPerKilogram,
    #[unit("ZW/kg", "zettawatts per kilogram", by = ZETTAWATT_PER_KILOGRAM)]
    ZettawattPerKilogram,
    #[unit("EW/kg", "exawatts per kilogram", by = EXAWATT_PER_KILOGRAM)]
    ExawattPerKilogram,
    #[unit("PW/kg", "petawatts per kilogram", by = PETAWATT_PER_KILOGRAM)]
    PetawattPerKilogram,
    #[unit("TW/kg", "terawatts per kilogram", by = TERAWATT_PER_KILOGRAM)]
    TerawattPerKilogram,
    #[unit("GW/kg", "gigawatts per kilogram", by = GIGAWATT_PER_KILOGRAM)]
    GigawattPerKilogram,
    #[unit("MW/kg", "megawatts per kilogram", by = MEGAWATT_PER_KILOGRAM)]
    MegawattPerKilogram,
    #[unit("kW/kg", "kilowatts per kilogram", by = KILOWATT_PER_KILOGRAM)]
    KilowattPerKilogram,
    #[unit("hW/kg", "hectowatts per kilogram", by = HECTOWATT_PER_KILOGRAM)]
    HectowattPerKilogram,
    #[unit("daW/kg", "decawatts per kilogram", by = DECAWATT_PER_KILOGRAM)]
    DecawattPerKilogram,
    #[default]
    #[unit("W/kg", "watts per kilogram", by = WATT_PER_KILOGRAM)]
    WattPerKilogram,
    #[unit("dW/kg", "deciwatts per kilogram", by = DECIWATT_PER_KILOGRAM)]
    DeciwattPerKilogram,
    #[unit("cW/kg", "centiwatts per kilogram", by = CENTIWATT_PER_KILOGRAM)]
    CentiwattPerKilogram,
    #[unit("mW/kg", "milliwatts per kilogram", by = MILLIWATT_PER_KILOGRAM)]
    MilliwattPerKilogram,
    #[unit("µW/kg", "microwatts per kilogram", by = MICROWATT_PER_KILOGRAM)]
    MicrowattPerKilogram,
    #[unit("nW/kg", "nanowatts per kilogram", by = NANOWATT_PER_KILOGRAM)]
    NanowattPerKilogram,
    #[unit("pW/kg", "picowatts per kilogram", by = PICOWATT_PER_KILOGRAM)]
    PicowattPerKilogram,
    #[unit("fW/kg", "femtowatts per kilogram", by = FEMTOWATT_PER_KILOGRAM)]
    FemtowattPerKilogram,
    #[unit("aW/kg", "attowatts per kilogram", by = ATTOWATT_PER_KILOGRAM)]
    AttowattPerKilogram,
    #[unit("zW/kg", "zeptowatts per kilogram", by = ZEPTOWATT_PER_KILOGRAM)]
    ZeptowattPerKilogram,
    #[unit("yW/kg", "yoctowatts per kilogram", by = YOCTOWATT_PER_KILOGRAM)]
    YoctowattPerKilogram,
    #[unit("m² · s⁻³", "square metres per cubic second", by = SQUARE_METRE_PER_CUBIC_SECOND)]
    SquareMetrePerCubicSecond,
    #[unit("(erg/s)/kg", "ergs per second per kilogram", by = ERG_PER_SECOND_PER_KILOGRAM)]
    ErgPerSecondPerKilogram,
    #[unit("(ft · lbf/h)/lb", "foot pounds-force per hour per pound", by = FOOT_POUND_PER_HOUR_PER_POUND)]
    FootPoundPerHourPerPound,
    #[unit("(ft · lbf/min)/lb", "foot pounds-force per minute per pound", by = FOOT_POUND_PER_MINUTE_PER_POUND)]
    FootPoundPerMinutePerPound,
    #[unit("(ft · lbf/s)/lb", "foot pounds-force per second per pound", by = FOOT_POUND_PER_SECOND_PER_POUND)]
    FootPoundPerSecondPerPound,
    #[unit("hp/lb", "horsepower per pound", by = HORSEPOWER_PER_POUND)]
    HorsepowerPerPound,
    #[unit("(hp (S))/lb", "horsepower (boiler) per pound", by = HORSEPOWER_BOILER_PER_POUND)]
    HorsepowerBoilerPerPound,
    #[unit("(hp (E))/lb", "horsepower (electric) per pound", by = HORSEPOWER_ELECTRIC_PER_POUND)]
    HorsepowerElectricPerPound,
    #[unit("(hp (M))/lb", "metric horsepower per pound", by = HORSEPOWER_METRIC_PER_POUND)]
    HorsepowerMetricPerPound,
    #[unit("(hp (I))/lb", "horsepower (Imperial) per pound", by = HORSEPOWER_IMPERIAL_PER_POUND)]
    HorsepowerImperialPerPound,
    #[unit("(hp (hydraulic))/lb", "hydraulic horsepower per pound", by = HYDRAULIC_HORSEPOWER_PER_POUND)]
    HydraulicHorsepowerPerPound,
}

/// The units of `SpecificGravitationalParameter`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificGravitationalParameter)]
pub enum SpecificGravitationalParameterUnit {
    #[default]
    #[unit("m³/(kg · s²)", "cubic metres per kilogram second squared", by = CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED)]
    CubicMetrePerKilogramSecondSquared,
    #[unit("G", "Newtonian constants of gravitation", by = GRAVITATIONAL_CONSTANT)]
    GravitationalConstant,
}
