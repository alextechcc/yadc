use crate::base::{ONE, RADIAN};
use crate::{Exact, ISQ, Kind, MILLI, SIQuantity, Scale};
use consts::*;
use yadc::Dimension;

pub type Length<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Mass<N> = SIQuantity<
    {
        ISQ {
            kg: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Time<N> = SIQuantity<
    {
        ISQ {
            s: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Instant<N> = SIQuantity<
    {
        ISQ {
            s: 1,
            kind: Kind::Affine,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Area<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Volume<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Velocity<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Acceleration<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Jerk<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Absement<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            s: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type InverseVelocity<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            s: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Force<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Momentum<N> = SIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Energy<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type AvailableEnergy<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Power<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Gain<N> = SIQuantity<
    {
        ISQ {
            kind: Kind::Log,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type PowerLevel<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            kind: Kind::Log,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type PowerRate<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -4,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Action<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Torque<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            rad: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type MomentOfInertia<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            rad: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Pressure<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            kg: 1,
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Frequency<N> = SIQuantity<
    {
        ISQ {
            s: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type FrequencyDrift<N> = SIQuantity<
    {
        ISQ {
            s: -2,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type ReciprocalLength<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type Curvature<N> = SIQuantity<
    {
        ISQ {
            m: -1,
            rad: 1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificArea<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            kg: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificVolume<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            kg: -1,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificPower<N> = SIQuantity<
    {
        ISQ {
            m: 2,
            s: -3,
            ..ISQ::DIMENSIONLESS
        }
    },
    N,
>;

pub type SpecificGravitationalParameter<N> = SIQuantity<
    {
        ISQ {
            m: 3,
            kg: -1,
            s: -2,
            ..ISQ::DIMENSIONLESS
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
    use crate::base::ONE;

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

    /// One `Angstrom` (Å), as a `Length`.
    pub const ANGSTROM: Length<Exact> = METRE * Exact::dec(1, -10);

    /// One `BohrRadius` (a₀), as a `Length`.
    pub const BOHR_RADIUS: Length<Exact> = METRE * Exact::dec(529177210903, -22);

    /// One `AtomicUnitOfLength` (a.u. of length), as a `Length`.
    pub const ATOMIC_UNIT_OF_LENGTH: Length<Exact> = METRE * Exact::dec(529177210903, -22);

    /// One `AstronomicalUnit` (ua), as a `Length`.
    pub const ASTRONOMICAL_UNIT: Length<Exact> = METRE * Exact::int(149597870700);

    /// One `Chain` (ch), as a `Length`.
    pub const CHAIN: Length<Exact> = METRE * (Exact::int(66) * Exact::int(1200) / Exact::int(3937));

    /// One `DataMile` (DM), as a `Length`.
    pub const DATA_MILE: Length<Exact> = METRE * Exact::dec(18288, -1);

    /// One `Fathom` (fathom), as a `Length`.
    pub const FATHOM: Length<Exact> = METRE * (Exact::int(6) * Exact::int(1200) / Exact::int(3937));

    /// One `Fermi` (fermi), as a `Length`.
    pub const FERMI: Length<Exact> = METRE * Exact::dec(1, -15);

    /// One `Foot` (ft), as a `Length`.
    pub const FOOT: Length<Exact> = METRE * Exact::dec(3048, -4);

    /// One `FootSurvey` (ft (U.S. survey)), as a `Length`.
    pub const FOOT_SURVEY: Length<Exact> = METRE * (Exact::int(1200) / Exact::int(3937));

    /// One `Inch` (in), as a `Length`.
    pub const INCH: Length<Exact> = METRE * Exact::dec(254, -4);

    /// One `LightYear` (l. y.), as a `Length`.
    pub const LIGHT_YEAR: Length<Exact> = METRE * Exact::int(9460730472580800);

    /// One `Micron` (μ), as a `Length`.
    pub const MICRON: Length<Exact> = METRE * Exact::dec(1, -6);

    /// One `Mil` (0.001 in), as a `Length`.
    pub const MIL: Length<Exact> = METRE * Exact::dec(254, -7);

    /// One `Mile` (mi), as a `Length`.
    pub const MILE: Length<Exact> = METRE * Exact::dec(1609344, -3);

    /// One `MileSurvey` (mi (U.S. survey)), as a `Length`.
    pub const MILE_SURVEY: Length<Exact> = METRE * (Exact::int(6336000) / Exact::int(3937));

    /// One `NauticalMile` (M), as a `Length`.
    pub const NAUTICAL_MILE: Length<Exact> = METRE * Exact::int(1852);

    /// One `Parsec` (pc), as a `Length`.
    pub const PARSEC: Length<Exact> =
        METRE * (Exact::int(149597870700) * Exact::int(648000) / Exact::PI);

    /// One `PicaComputer` (1/6 in (computer)), as a `Length`.
    pub const PICA_COMPUTER: Length<Exact> = METRE * (Exact::dec(254, -4) / Exact::int(6));

    /// One `PicaPrinters` (1/6 in), as a `Length`.
    pub const PICA_PRINTERS: Length<Exact> = METRE * Exact::dec(4217518, -9);

    /// One `PointComputer` (1/72 in (computer)), as a `Length`.
    pub const POINT_COMPUTER: Length<Exact> = METRE * (Exact::dec(254, -4) / Exact::int(72));

    /// One `PointPrinters` (1/72 in), as a `Length`.
    pub const POINT_PRINTERS: Length<Exact> = METRE * Exact::dec(3514598, -10);

    /// One `Rod` (rd), as a `Length`.
    pub const ROD: Length<Exact> =
        METRE * (Exact::dec(165, -1) * Exact::int(1200) / Exact::int(3937));

    /// One `Yard` (yd), as a `Length`.
    pub const YARD: Length<Exact> = METRE * Exact::dec(9144, -4);

    /// One `Leuga` (leuga), as a `Length`.
    pub const LEUGA: Length<Exact> = METRE * Exact::int(2220);

    /// One `MillePassus` (mille passus), as a `Length`.
    pub const MILLE_PASSUS: Length<Exact> = METRE * Exact::int(1480);

    /// One `Stadium` (stadium), as a `Length`.
    pub const STADIUM: Length<Exact> = METRE * Exact::int(185);

    /// One `Actus` (actus), as a `Length`.
    pub const ACTUS: Length<Exact> = METRE * Exact::dec(3552, -2);

    /// One `Decempeda` (decempeda), as a `Length`.
    pub const DECEMPEDA: Length<Exact> = METRE * Exact::dec(296, -2);

    /// One `Passus` (passus), as a `Length`.
    pub const PASSUS: Length<Exact> = METRE * Exact::dec(148, -2);

    /// One `Gradus` (gradus), as a `Length`.
    pub const GRADUS: Length<Exact> = METRE * Exact::dec(74, -2);

    /// One `Cubitum` (cubitum), as a `Length`.
    pub const CUBITUM: Length<Exact> = METRE * Exact::dec(444, -3);

    /// One `Palmipes` (palmipes), as a `Length`.
    pub const PALMIPES: Length<Exact> = METRE * Exact::dec(37, -2);

    /// One `Pes` (pes), as a `Length`.
    pub const PES: Length<Exact> = METRE * Exact::dec(296, -3);

    /// One `PalmusMaior` (palmus maior), as a `Length`.
    pub const PALMUS_MAIOR: Length<Exact> = METRE * Exact::dec(222, -3);

    /// One `Palmus` (palmus), as a `Length`.
    pub const PALMUS: Length<Exact> = METRE * Exact::dec(74, -3);

    /// One `Uncia` (uncia), as a `Length`.
    pub const LENGTH_UNCIA: Length<Exact> = METRE * (Exact::dec(296, -3) / Exact::int(12));

    /// One `Digitus` (digitus), as a `Length`.
    pub const DIGITUS: Length<Exact> = METRE * Exact::dec(185, -4);

    /// One `Gram` (g), as a `Mass`.
    pub const GRAM: Mass<Exact> = KILOGRAM * Exact::dec(1, -3);

    /// One `Carat` (ct), as a `Mass`.
    pub const CARAT: Mass<Exact> = KILOGRAM * Exact::dec(2, -4);

    /// One `Dalton` (Da), as a `Mass`.
    pub const DALTON: Mass<Exact> = KILOGRAM * Exact::dec(16605390666, -37);

    /// One `ElectronMass` (mₑ), as a `Mass`.
    pub const ELECTRON_MASS: Mass<Exact> = KILOGRAM * Exact::dec(91093837015, -41);

    /// One `ProtonMass` (mₚ), as a `Mass`.
    pub const PROTON_MASS: Mass<Exact> = KILOGRAM * Exact::dec(167262192369, -38);

    /// One `Grain` (gr), as a `Mass`.
    pub const GRAIN: Mass<Exact> = KILOGRAM * Exact::dec(6479891, -11);

    /// One `HundredweightLong` (cwt long), as a `Mass`.
    pub const HUNDREDWEIGHT_LONG: Mass<Exact> = KILOGRAM * Exact::dec(5080234544, -8);

    /// One `HundredweightShort` (cwt short), as a `Mass`.
    pub const HUNDREDWEIGHT_SHORT: Mass<Exact> = KILOGRAM * Exact::dec(45359237, -6);

    /// One `Ounce` (oz), as a `Mass`.
    pub const OUNCE: Mass<Exact> = KILOGRAM * Exact::dec(28349523125, -12);

    /// One `OunceTroy` (oz t), as a `Mass`.
    pub const OUNCE_TROY: Mass<Exact> = KILOGRAM * Exact::dec(311034768, -10);

    /// One `Pennyweight` (dwt), as a `Mass`.
    pub const PENNYWEIGHT: Mass<Exact> = KILOGRAM * Exact::dec(155517384, -11);

    /// One `Pound` (lb), as a `Mass`.
    pub const POUND: Mass<Exact> = KILOGRAM * Exact::dec(45359237, -8);

    /// One `PoundTroy` (lb t), as a `Mass`.
    pub const POUND_TROY: Mass<Exact> = KILOGRAM * Exact::dec(3732417216, -10);

    /// One `Slug` (slug), as a `Mass`.
    pub const SLUG: Mass<Exact> =
        KILOGRAM * (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4));

    /// One `TonAssay` (AT), as a `Mass`.
    pub const TON_ASSAY: Mass<Exact> = KILOGRAM * (Exact::dec(175, -3) / Exact::int(6));

    /// One `TonLong` (2240 lb), as a `Mass`.
    pub const TON_LONG: Mass<Exact> = KILOGRAM * Exact::dec(10160469088, -7);

    /// One `TonShort` (2000 lb), as a `Mass`.
    pub const TON_SHORT: Mass<Exact> = KILOGRAM * Exact::dec(90718474, -5);

    /// One `Ton` (t), as a `Mass`.
    pub const TON: Mass<Exact> = KILOGRAM * Exact::int(1000);

    /// One `Libra` (libra), as a `Mass`.
    pub const LIBRA: Mass<Exact> = KILOGRAM * Exact::dec(3289, -4);

    /// One `Deunx` (deunx), as a `Mass`.
    pub const MASS_DEUNX: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(11) / Exact::int(12));

    /// One `Dextans` (dextans), as a `Mass`.
    pub const MASS_DEXTANS: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(10) / Exact::int(12));

    /// One `Dodrans` (dodrans), as a `Mass`.
    pub const MASS_DODRANS: Mass<Exact> = KILOGRAM * Exact::dec(246675, -6);

    /// One `Bes` (bes), as a `Mass`.
    pub const MASS_BES: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(8) / Exact::int(12));

    /// One `Septunx` (septunx), as a `Mass`.
    pub const MASS_SEPTUNX: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(7) / Exact::int(12));

    /// One `Semis` (semis), as a `Mass`.
    pub const MASS_SEMIS: Mass<Exact> = KILOGRAM * Exact::dec(16445, -5);

    /// One `Quincunx` (quincunx), as a `Mass`.
    pub const MASS_QUINCUNX: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(5) / Exact::int(12));

    /// One `Triens` (triens), as a `Mass`.
    pub const MASS_TRIENS: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(3));

    /// One `QuadransTeruncius` (quadrans teruncius), as a `Mass`.
    pub const QUADRANS_TERUNCIUS: Mass<Exact> = KILOGRAM * Exact::dec(82225, -6);

    /// One `Sextans` (sextans), as a `Mass`.
    pub const MASS_SEXTANS: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(6));

    /// One `Sescuncia` (sescuncia), as a `Mass`.
    pub const SESCUNCIA: Mass<Exact> = KILOGRAM * Exact::dec(411125, -7);

    /// One `Uncia` (uncia), as a `Mass`.
    pub const MASS_UNCIA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(12));

    /// One `Semuncia` (semuncia), as a `Mass`.
    pub const SEMUNCIA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(24));

    /// One `Duella` (duella), as a `Mass`.
    pub const DUELLA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(36));

    /// One `Sicilicus` (sicilicus), as a `Mass`.
    pub const MASS_SICILICUS: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(48));

    /// One `Sextula` (sextula), as a `Mass`.
    pub const MASS_SEXTULA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(72));

    /// One `Semisextula` (semisextula), as a `Mass`.
    pub const SEMISEXTULA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(144));

    /// One `Scrupulum` (scrupulum), as a `Mass`.
    pub const MASS_SCRUPULUM: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(288));

    /// One `Obolus` (obolus), as a `Mass`.
    pub const OBOLUS: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(576));

    /// One `Siliqua` (siliqua), as a `Mass`.
    pub const SILIQUA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(1728));

    /// One `SecondSidereal` (s (sidereal)), as a `Time`.
    pub const SECOND_SIDEREAL: Time<Exact> = SECOND * Exact::dec(9972696, -7);

    /// One `Day` (d), as a `Time`.
    pub const DAY: Time<Exact> = SECOND * Exact::int(86400);

    /// One `DaySidereal` (d (sidereal)), as a `Time`.
    pub const DAY_SIDEREAL: Time<Exact> = SECOND * Exact::dec(8616409, -2);

    /// One `Hour` (h), as a `Time`.
    pub const HOUR: Time<Exact> = SECOND * Exact::int(3600);

    /// One `HourSidereal` (h (sidereal)), as a `Time`.
    pub const HOUR_SIDEREAL: Time<Exact> = SECOND * Exact::dec(359017, -2);

    /// One `Minute` (min), as a `Time`.
    pub const MINUTE: Time<Exact> = SECOND * Exact::int(60);

    /// One `Shake` (10.0 ns), as a `Time`.
    pub const SHAKE: Time<Exact> = SECOND * Exact::dec(1, -8);

    /// One `Year` (a), as a `Time`.
    pub const YEAR: Time<Exact> = SECOND * Exact::int(31536000);

    /// One `YearSidereal` (a (sidereal)), as a `Time`.
    pub const YEAR_SIDEREAL: Time<Exact> = SECOND * Exact::int(31558150);

    /// One `YearTropical` (a (tropical)), as a `Time`.
    pub const YEAR_TROPICAL: Time<Exact> = SECOND * Exact::int(31556930);

    /// The GPS epoch, 1980-01-06T00:00:00Z: 3657 days after the Unix epoch.
    pub const GPS_EPOCH: Instant<Exact> = UNIX_EPOCH + SECOND * Exact::int(315964800);

    /// The zero of the Julian date: noon on 1 January 4713 BC (proleptic Julian), 2 440 587.5 days before the Unix epoch.
    pub const JULIAN_EPOCH: Instant<Exact> = UNIX_EPOCH - DAY * Exact::dec(24405875, -1);

    /// The zero of the modified Julian date, 1858-11-17T00:00:00Z: 40 587 days before the Unix epoch.
    pub const MODIFIED_JULIAN_EPOCH: Instant<Exact> = UNIX_EPOCH - DAY * Exact::int(40587);

    /// Unix time: seconds from the Unix epoch.
    pub const UNIX_SECONDS: Scale<Instant<Exact>, Time<Exact>> = Scale(UNIX_EPOCH, SECOND);

    /// Unix time in milliseconds.
    pub const UNIX_MILLISECONDS: Scale<Instant<Exact>, Time<Exact>> =
        Scale(UNIX_EPOCH, MILLI * SECOND);

    /// GPS time: seconds from the GPS epoch.
    pub const GPS_SECONDS: Scale<Instant<Exact>, Time<Exact>> = Scale(GPS_EPOCH, SECOND);

    /// The Julian date (JD): days from the Julian epoch.
    pub const JULIAN_DATE: Scale<Instant<Exact>, Time<Exact>> = Scale(JULIAN_EPOCH, DAY);

    /// The modified Julian date (MJD): days from the modified Julian epoch.
    pub const MODIFIED_JULIAN_DATE: Scale<Instant<Exact>, Time<Exact>> =
        Scale(MODIFIED_JULIAN_EPOCH, DAY);

    /// One `SquareMetre` (m²), as an `Area`.
    pub const SQUARE_METRE: Area<Exact> = METRE * METRE;

    /// One `Acre` (ac), as an `Area`.
    pub const ACRE: Area<Exact> = SQUARE_METRE
        * (Exact::int(43560)
            * (Exact::int(1200) / Exact::int(3937))
            * (Exact::int(1200) / Exact::int(3937)));

    /// One `Are` (a), as an `Area`.
    pub const ARE: Area<Exact> = SQUARE_METRE * Exact::int(100);

    /// One `Barn` (b), as an `Area`.
    pub const BARN: Area<Exact> = SQUARE_METRE * Exact::dec(1, -28);

    /// One `CircularMil` (cmil), as an `Area`.
    pub const CIRCULAR_MIL: Area<Exact> =
        SQUARE_METRE * (Exact::PI / Exact::int(4) * Exact::dec(254, -7) * Exact::dec(254, -7));

    /// One `Hectare` (ha), as an `Area`.
    pub const HECTARE: Area<Exact> = SQUARE_METRE * Exact::int(10000);

    /// One `SquareFoot` (ft²), as an `Area`.
    pub const SQUARE_FOOT: Area<Exact> = SQUARE_METRE * Exact::dec(9290304, -8);

    /// One `SquareInch` (in²), as an `Area`.
    pub const SQUARE_INCH: Area<Exact> = SQUARE_METRE * Exact::dec(64516, -8);

    /// One `SquareMile` (mi²), as an `Area`.
    pub const SQUARE_MILE: Area<Exact> = SQUARE_METRE * Exact::dec(2589988110336, -6);

    /// One `SquareYard` (yd²), as an `Area`.
    pub const SQUARE_YARD: Area<Exact> = SQUARE_METRE * Exact::dec(83612736, -8);

    /// One `Saltus` (saltus), as an `Area`.
    pub const SALTUS: Area<Exact> = SQUARE_METRE * Exact::dec(201867264, -2);

    /// One `Centuria` (centuria), as an `Area`.
    pub const CENTURIA: Area<Exact> = SQUARE_METRE * Exact::dec(50466816, -2);

    /// One `Heredium` (heredium), as an `Area`.
    pub const HEREDIUM: Area<Exact> = SQUARE_METRE * Exact::dec(50466816, -4);

    /// One `Jugerum` (jugerum), as an `Area`.
    pub const JUGERUM: Area<Exact> = SQUARE_METRE * Exact::dec(25233408, -4);

    /// One `Deunx` (deunx), as an `Area`.
    pub const AREA_DEUNX: Area<Exact> = SQUARE_METRE * Exact::dec(23130624, -4);

    /// One `Dextans` (dextans), as an `Area`.
    pub const AREA_DEXTANS: Area<Exact> = SQUARE_METRE * Exact::dec(2102784, -3);

    /// One `Dodrans` (dodrans), as an `Area`.
    pub const AREA_DODRANS: Area<Exact> = SQUARE_METRE * Exact::dec(18925056, -4);

    /// One `Bes` (bes), as an `Area`.
    pub const AREA_BES: Area<Exact> = SQUARE_METRE * Exact::dec(16822272, -4);

    /// One `Septunx` (septunx), as an `Area`.
    pub const AREA_SEPTUNX: Area<Exact> = SQUARE_METRE * Exact::dec(14719488, -4);

    /// One `Semis` (semis), as an `Area`.
    pub const AREA_SEMIS: Area<Exact> = SQUARE_METRE * Exact::dec(12616704, -4);

    /// One `ActusQuadratus` (actus quadratus), as an `Area`.
    pub const ACTUS_QUADRATUS: Area<Exact> = SQUARE_METRE * Exact::dec(12616704, -4);

    /// One `Quincunx` (quincunx), as an `Area`.
    pub const AREA_QUINCUNX: Area<Exact> = SQUARE_METRE * Exact::dec(1051392, -3);

    /// One `Triens` (triens), as an `Area`.
    pub const AREA_TRIENS: Area<Exact> = SQUARE_METRE * Exact::dec(8411136, -4);

    /// One `Quadrans` (quadrans), as an `Area`.
    pub const QUADRANS: Area<Exact> = SQUARE_METRE * Exact::dec(6308352, -4);

    /// One `Sextans` (sextans), as an `Area`.
    pub const AREA_SEXTANS: Area<Exact> = SQUARE_METRE * Exact::dec(4205568, -4);

    /// One `Clima` (clima), as an `Area`.
    pub const CLIMA: Area<Exact> = SQUARE_METRE * Exact::dec(3154176, -4);

    /// One `Uncia` (uncia), as an `Area`.
    pub const AREA_UNCIA: Area<Exact> = SQUARE_METRE * Exact::dec(2102784, -4);

    /// One `Semiuncia` (semiuncia), as an `Area`.
    pub const SEMIUNCIA: Area<Exact> = SQUARE_METRE * Exact::dec(1051392, -4);

    /// One `Sicilicus` (sicilicus), as an `Area`.
    pub const AREA_SICILICUS: Area<Exact> = SQUARE_METRE * Exact::dec(525696, -4);

    /// One `ActusSimplex` (actus simplex), as an `Area`.
    pub const ACTUS_SIMPLEX: Area<Exact> = SQUARE_METRE * Exact::dec(4205568, -5);

    /// One `Sextula` (sextula), as an `Area`.
    pub const AREA_SEXTULA: Area<Exact> = SQUARE_METRE * Exact::dec(350464, -4);

    /// One `DuoScrupulum` (duo scrupulum), as an `Area`.
    pub const DUO_SCRUPULUM: Area<Exact> = SQUARE_METRE * Exact::dec(175232, -4);

    /// One `Scrupulum` (scrupulum), as an `Area`.
    pub const AREA_SCRUPULUM: Area<Exact> = SQUARE_METRE * Exact::dec(87616, -4);

    /// One `DimidiumScrupulum` (dimidium scrupulum), as an `Area`.
    pub const DIMIDIUM_SCRUPULUM: Area<Exact> = SQUARE_METRE * Exact::dec(43808, -4);

    /// One `PesQuadratus` (pes quadratus), as an `Area`.
    pub const PES_QUADRATUS: Area<Exact> = SQUARE_METRE * Exact::dec(87616, -6);

    /// One `CubicMetre` (m³), as a `Volume`.
    pub const CUBIC_METRE: Volume<Exact> = METRE * METRE * METRE;

    /// One `Barrel` (bbl), as a `Volume`.
    pub const BARREL: Volume<Exact> = CUBIC_METRE * Exact::dec(158987294928, -12);

    /// One `Bushel` (bu), as a `Volume`.
    pub const BUSHEL: Volume<Exact> = CUBIC_METRE * Exact::dec(3523907016688, -14);

    /// One `Cord` (cords), as a `Volume`.
    pub const CORD: Volume<Exact> = CUBIC_METRE
        * (Exact::int(128) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4));

    /// One `CubicFoot` (ft³), as a `Volume`.
    pub const CUBIC_FOOT: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4));

    /// One `CubicInch` (in³), as a `Volume`.
    pub const CUBIC_INCH: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4));

    /// One `CubicMile` (mi³), as a `Volume`.
    pub const CUBIC_MILE: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3));

    /// One `CubicYard` (yd³), as a `Volume`.
    pub const CUBIC_YARD: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4));

    /// One `Cup` (cup), as a `Volume`.
    pub const CUP: Volume<Exact> = CUBIC_METRE * Exact::dec(2365882365, -13);

    /// One `FluidOunce` (fl oz), as a `Volume`.
    pub const FLUID_OUNCE: Volume<Exact> = CUBIC_METRE * Exact::dec(295735295625, -16);

    /// One `FluidOunceImperial` (fl oz (UK)), as a `Volume`.
    pub const FLUID_OUNCE_IMPERIAL: Volume<Exact> = CUBIC_METRE * Exact::dec(284130625, -13);

    /// One `GallonImperial` (gal (UK)), as a `Volume`.
    pub const GALLON_IMPERIAL: Volume<Exact> = CUBIC_METRE * Exact::dec(454609, -8);

    /// One `Gallon` (gal), as a `Volume`.
    pub const GALLON: Volume<Exact> = CUBIC_METRE * Exact::dec(3785411784, -12);

    /// One `GillImperial` (gi (UK)), as a `Volume`.
    pub const GILL_IMPERIAL: Volume<Exact> = CUBIC_METRE * Exact::dec(1420653125, -13);

    /// One `Gill` (gi), as a `Volume`.
    pub const GILL: Volume<Exact> = CUBIC_METRE * Exact::dec(11829411825, -14);

    /// One `Litre` (L), as a `Volume`.
    pub const LITRE: Volume<Exact> = CUBIC_METRE * Exact::dec(1, -3);

    /// One `Peck` (pk), as a `Volume`.
    pub const PECK: Volume<Exact> = CUBIC_METRE * Exact::dec(880976754172, -14);

    /// One `PintDry` (dry pt), as a `Volume`.
    pub const PINT_DRY: Volume<Exact> = CUBIC_METRE * Exact::dec(5506104713575, -16);

    /// One `PintLiquid` (liq pt), as a `Volume`.
    pub const PINT_LIQUID: Volume<Exact> = CUBIC_METRE * Exact::dec(473176473, -12);

    /// One `QuartDry` (dry qt), as a `Volume`.
    pub const QUART_DRY: Volume<Exact> = CUBIC_METRE * Exact::dec(1101220942715, -15);

    /// One `QuartLiquid` (liq qt), as a `Volume`.
    pub const QUART_LIQUID: Volume<Exact> = CUBIC_METRE * Exact::dec(946352946, -12);

    /// One `Stere` (st), as a `Volume`.
    pub const STERE: Volume<Exact> = CUBIC_METRE;

    /// One `Tablespoon` (tbsp), as a `Volume`.
    pub const TABLESPOON: Volume<Exact> = CUBIC_METRE * Exact::dec(1478676478125, -17);

    /// One `Teaspoon` (tsp), as a `Volume`.
    pub const TEASPOON: Volume<Exact> = CUBIC_METRE * Exact::dec(492892159375, -17);

    /// One `RegisterTon` (RT), as a `Volume`.
    pub const REGISTER_TON: Volume<Exact> = CUBIC_METRE
        * (Exact::int(100) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4));

    /// One `Culeus` (culeus), as a `Volume`.
    pub const CULEUS: Volume<Exact> = CUBIC_METRE * Exact::dec(51868672, -8);

    /// One `AmphoraQuadrantal` (amphora quadrantal), as a `Volume`.
    pub const AMPHORA_QUADRANTAL: Volume<Exact> = CUBIC_METRE * Exact::dec(25934336, -9);

    /// One `Urna` (urna), as a `Volume`.
    pub const URNA: Volume<Exact> = CUBIC_METRE * Exact::dec(12967168, -9);

    /// One `ModiusCastrensis` (modius castrensis), as a `Volume`.
    pub const MODIUS_CASTRENSIS: Volume<Exact> = CUBIC_METRE * Exact::dec(12967168, -9);

    /// One `Modius` (modius), as a `Volume`.
    pub const MODIUS: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(3241792, -9) * Exact::int(16) / Exact::int(6));

    /// One `Semimodius` (semimodius), as a `Volume`.
    pub const SEMIMODIUS: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(3241792, -9) * Exact::int(8) / Exact::int(6));

    /// One `Congius` (congius), as a `Volume`.
    pub const CONGIUS: Volume<Exact> = CUBIC_METRE * Exact::dec(3241792, -9);

    /// One `Sextarius` (sextarius), as a `Volume`.
    pub const SEXTARIUS: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(6));

    /// One `Hemina` (hemina), as a `Volume`.
    pub const HEMINA: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(12));

    /// One `Quartarius` (quartarius), as a `Volume`.
    pub const QUARTARIUS: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(24));

    /// One `Acetabulum` (acetabulum), as a `Volume`.
    pub const ACETABULUM: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(48));

    /// One `Cyathus` (cyathus), as a `Volume`.
    pub const CYATHUS: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(72));

    /// One `Ligula` (ligula), as a `Volume`.
    pub const LIGULA: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(288));

    /// One `MetrePerSecond` (m/s), as a `Velocity`.
    pub const METRE_PER_SECOND: Velocity<Exact> = METRE / SECOND;

    /// One `Knot` (kn), as a `Velocity`.
    pub const KNOT: Velocity<Exact> = METRE_PER_SECOND * (Exact::int(1852) / Exact::int(3600));

    /// One `MilePerHour` (mi/h), as a `Velocity`.
    pub const MILE_PER_HOUR: Velocity<Exact> = METRE_PER_SECOND * Exact::dec(44704, -5);

    /// One `NaturalUnitOfVelocity` (c), as a `Velocity`.
    pub const NATURAL_UNIT_OF_VELOCITY: Velocity<Exact> = METRE_PER_SECOND * Exact::int(299792458);

    /// One `SpeedOfLight` (c), as a `Velocity`.
    pub const SPEED_OF_LIGHT: Velocity<Exact> = METRE_PER_SECOND * Exact::int(299792458);

    /// One `MetrePerSecondSquared` (m/s²), as an `Acceleration`.
    pub const METRE_PER_SECOND_SQUARED: Acceleration<Exact> = METRE / (SECOND * SECOND);

    /// One `Galileo` (Gal), as an `Acceleration`.
    pub const GALILEO: Acceleration<Exact> = METRE_PER_SECOND_SQUARED * Exact::dec(1, -2);

    /// One `StandardGravity` (g₀), as an `Acceleration`.
    pub const STANDARD_GRAVITY: Acceleration<Exact> =
        METRE_PER_SECOND_SQUARED * Exact::dec(980665, -5);

    /// One `Newton` (N), as a `Force`.
    pub const NEWTON: Force<Exact> = METRE * KILOGRAM / (SECOND * SECOND);

    /// One `Dyne` (dyn), as a `Force`.
    pub const DYNE: Force<Exact> = NEWTON * Exact::dec(1, -5);

    /// One `KilogramForce` (kgf), as a `Force`.
    pub const KILOGRAM_FORCE: Force<Exact> = NEWTON * Exact::dec(980665, -5);

    /// One `Kip` (kip), as a `Force`.
    pub const KIP: Force<Exact> =
        NEWTON * (Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5));

    /// One `OunceForce` (ozf), as a `Force`.
    pub const OUNCE_FORCE: Force<Exact> =
        NEWTON * (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16));

    /// One `Poundal` (pdl), as a `Force`.
    pub const POUNDAL: Force<Exact> = NEWTON * (Exact::dec(45359237, -8) * Exact::dec(3048, -4));

    /// One `PoundForce` (lbf), as a `Force`.
    pub const POUND_FORCE: Force<Exact> =
        NEWTON * (Exact::dec(45359237, -8) * Exact::dec(980665, -5));

    /// One `TonForce` (2000 lbf), as a `Force`.
    pub const TON_FORCE: Force<Exact> =
        NEWTON * (Exact::int(2000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5));

    /// One `Joule` (J), as an `Energy`.
    pub const JOULE: Energy<Exact> = METRE * METRE * KILOGRAM / (SECOND * SECOND);

    /// One `Electronvolt` (eV), as an `Energy`.
    pub const ELECTRONVOLT: Energy<Exact> = JOULE * Exact::dec(1602176634, -28);

    /// One `Hartree` (Eₕ), as an `Energy`.
    pub const HARTREE: Energy<Exact> = JOULE * Exact::dec(43597447222071, -31);

    /// One `BtuIt` (Btu (IT)), as an `Energy`.
    pub const BTU_IT: Energy<Exact> = JOULE * Exact::dec(105505585262, -8);

    /// One `Btu` (Btu), as an `Energy`.
    pub const BTU: Energy<Exact> =
        JOULE * (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1));

    /// One `CalorieIt` (cal (IT)), as an `Energy`.
    pub const CALORIE_IT: Energy<Exact> = JOULE * Exact::dec(41868, -4);

    /// One `Calorie` (cal), as an `Energy`.
    pub const CALORIE: Energy<Exact> = JOULE * Exact::dec(4184, -3);

    /// One `CalorieItNutrition` (Cal (IT)), as an `Energy`.
    pub const CALORIE_IT_NUTRITION: Energy<Exact> = JOULE * Exact::dec(41868, -1);

    /// One `CalorieNutrition` (Cal), as an `Energy`.
    pub const CALORIE_NUTRITION: Energy<Exact> = JOULE * Exact::int(4184);

    /// One `Erg` (erg), as an `Energy`.
    pub const ERG: Energy<Exact> = JOULE * Exact::dec(1, -7);

    /// One `Quad` (10¹⁵ Btu (IT)), as an `Energy`.
    pub const QUAD: Energy<Exact> = JOULE * Exact::int(1055055852620000000);

    /// One `ThermEc` (thm (EC)), as an `Energy`.
    pub const THERM_EC: Energy<Exact> = JOULE * Exact::int(105506000);

    /// One `ThermUs` (thm), as an `Energy`.
    pub const THERM_US: Energy<Exact> = JOULE * Exact::int(105480400);

    /// One `TonTnt` (t of TNT), as an `Energy`.
    pub const TON_TNT: Energy<Exact> = JOULE * Exact::int(4184000000);

    /// One `Watt` (W), as a `Power`.
    pub const WATT: Power<Exact> = METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND);

    /// One `Horsepower` (hp), as a `Power`.
    pub const HORSEPOWER: Power<Exact> = WATT
        * (Exact::int(550)
            * Exact::dec(3048, -4)
            * Exact::dec(45359237, -8)
            * Exact::dec(980665, -5));

    /// One `HorsepowerBoiler` (hp (S)), as a `Power`.
    pub const HORSEPOWER_BOILER: Power<Exact> = WATT * Exact::dec(98095, -1);

    /// One `HorsepowerElectric` (hp (E)), as a `Power`.
    pub const HORSEPOWER_ELECTRIC: Power<Exact> = WATT * Exact::int(746);

    /// One `HorsepowerMetric` (hp (M)), as a `Power`.
    pub const HORSEPOWER_METRIC: Power<Exact> = WATT * (Exact::int(75) * Exact::dec(980665, -5));

    /// One `HorsepowerImperial` (hp (I)), as a `Power`.
    pub const HORSEPOWER_IMPERIAL: Power<Exact> = WATT * Exact::dec(7457, -1);

    /// One `HydraulicHorsepower` (hp (hydraulic)), as a `Power`.
    pub const HYDRAULIC_HORSEPOWER: Power<Exact> = WATT * Exact::dec(746043, -3);

    /// One `Decibel` (dB), as a `Gain`.
    pub const DECIBEL: Gain<Exact> = BEL * Exact::dec(1, -1);

    /// One `Neper` (Np), as a `Gain`.
    pub const NEPER: Gain<Exact> = BEL * Exact::dec(8685889638065036, -16);

    /// The level of one milliwatt, 0 dBm: 30 dB below a watt.
    pub const MILLIWATT_LEVEL: PowerLevel<Exact> = WATT_LEVEL - DECIBEL * Exact::int(30);

    /// Decibels-watt (dBW): decibels from one watt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBW: Scale<PowerLevel<Exact>, Gain<Exact>> = Scale(WATT_LEVEL, DECIBEL);

    /// Decibels-milliwatt (dBm): decibels from one milliwatt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBM: Scale<PowerLevel<Exact>, Gain<Exact>> = Scale(MILLIWATT_LEVEL, DECIBEL);

    /// One `AtomicUnitOfAction` (ħ), as an `Action`.
    pub const ATOMIC_UNIT_OF_ACTION: Action<Exact> = (METRE * METRE * KILOGRAM / SECOND)
        * (Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI));

    /// One `PlanckConstant` (h), as an `Action`.
    pub const PLANCK_CONSTANT: Action<Exact> =
        (METRE * METRE * KILOGRAM / SECOND) * Exact::dec(662607015, -42);

    /// One `Pascal` (Pa), as a `Pressure`.
    pub const PASCAL: Pressure<Exact> = KILOGRAM / (METRE * SECOND * SECOND);

    /// One `StandardAtmosphere` (atm), as a `Pressure`.
    pub const STANDARD_ATMOSPHERE: Pressure<Exact> = PASCAL * Exact::int(101325);

    /// One `AtmosphereTechnical` (at), as a `Pressure`.
    pub const ATMOSPHERE_TECHNICAL: Pressure<Exact> = PASCAL * Exact::dec(980665, -1);

    /// One `Bar` (bar), as a `Pressure`.
    pub const BAR: Pressure<Exact> = PASCAL * Exact::int(100000);

    /// One `CentimetreOfMercury` (cm Hg), as a `Pressure`.
    pub const CENTIMETRE_OF_MERCURY: Pressure<Exact> =
        PASCAL * (Exact::dec(133322387415, -9) * Exact::int(10));

    /// One `CentimetreOfWater` (cm H₂O), as a `Pressure`.
    pub const CENTIMETRE_OF_WATER: Pressure<Exact> = PASCAL * Exact::dec(980665, -4);

    /// One `FootOfMercury` (ft Hg), as a `Pressure`.
    pub const FOOT_OF_MERCURY: Pressure<Exact> =
        PASCAL * (Exact::dec(133322387415, -9) * Exact::dec(3048, -1));

    /// One `FootOfWater` (ft H₂O), as a `Pressure`.
    pub const FOOT_OF_WATER: Pressure<Exact> =
        PASCAL * (Exact::dec(980665, -5) * Exact::dec(3048, -1));

    /// One `InchOfMercury` (in Hg), as a `Pressure`.
    pub const INCH_OF_MERCURY: Pressure<Exact> =
        PASCAL * (Exact::dec(133322387415, -9) * Exact::dec(254, -1));

    /// One `InchOfWater` (in H₂O), as a `Pressure`.
    pub const INCH_OF_WATER: Pressure<Exact> =
        PASCAL * (Exact::dec(980665, -5) * Exact::dec(254, -1));

    /// One `MillimetreOfMercury` (mm Hg), as a `Pressure`.
    pub const MILLIMETRE_OF_MERCURY: Pressure<Exact> = PASCAL * Exact::dec(133322387415, -9);

    /// One `MillimetreOfWater` (mm H₂O), as a `Pressure`.
    pub const MILLIMETRE_OF_WATER: Pressure<Exact> = PASCAL * Exact::dec(980665, -5);

    /// One `Psi` (psi), as a `Pressure`.
    pub const PSI: Pressure<Exact> = PASCAL
        * (Exact::dec(45359237, -8) * Exact::dec(980665, -5)
            / (Exact::dec(254, -4) * Exact::dec(254, -4)));

    /// One `Torr` (Torr), as a `Pressure`.
    pub const TORR: Pressure<Exact> = PASCAL * (Exact::int(101325) / Exact::int(760));

    /// One `Hertz` (Hz), as a `Frequency`.
    pub const HERTZ: Frequency<Exact> = ONE / SECOND;

    /// One `CyclePerShake` (100 MHz), as a `Frequency`.
    pub const CYCLE_PER_SHAKE: Frequency<Exact> = HERTZ * Exact::int(100000000);

    /// One `ReciprocalMetre` (m⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_METRE: ReciprocalLength<Exact> = ONE / METRE;

    /// One `ReciprocalAngstrom` (Å⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_ANGSTROM: ReciprocalLength<Exact> =
        RECIPROCAL_METRE * Exact::int(10000000000);

    /// One `Diopter` (dpt), as a `ReciprocalLength`.
    pub const DIOPTER: ReciprocalLength<Exact> = RECIPROCAL_METRE;

    /// One `RydbergConstant` (R∞), as a `ReciprocalLength`.
    pub const RYDBERG_CONSTANT: ReciprocalLength<Exact> =
        RECIPROCAL_METRE * Exact::dec(1097373156816, -5);

    /// One `GravitationalConstant` (G), as a `SpecificGravitationalParameter`.
    pub const GRAVITATIONAL_CONSTANT: SpecificGravitationalParameter<Exact> =
        (METRE * METRE * METRE / (KILOGRAM * SECOND * SECOND)) * Exact::dec(66743, -15);
}

/// The units of `Length`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N> where Length<N>: [const] yadc::Unit<Length<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LengthUnit {
    #[default]
    #[unit("m", "metres", by = Length::<N>::from(METRE))]
    Metre,
    #[unit("Å", "ångströms", by = Length::<N>::from(ANGSTROM))]
    Angstrom,
    #[unit("a₀", "bohr radiuses", by = Length::<N>::from(BOHR_RADIUS))]
    BohrRadius,
    #[unit("a.u. of length", "atomic units of length", by = Length::<N>::from(ATOMIC_UNIT_OF_LENGTH))]
    AtomicUnitOfLength,
    #[unit("ua", "astronomical units", by = Length::<N>::from(ASTRONOMICAL_UNIT))]
    AstronomicalUnit,
    #[unit("ch", "chains", by = Length::<N>::from(CHAIN))]
    Chain,
    #[unit("DM", "data miles", by = Length::<N>::from(DATA_MILE))]
    DataMile,
    #[unit("fathom", "fathoms", by = Length::<N>::from(FATHOM))]
    Fathom,
    #[unit("fermi", "fermis", by = Length::<N>::from(FERMI))]
    Fermi,
    #[unit("ft", "feet", by = Length::<N>::from(FOOT))]
    Foot,
    #[unit("ft (U.S. survey)", "feet (U.S. survey)", by = Length::<N>::from(FOOT_SURVEY))]
    FootSurvey,
    #[unit("in", "inches", by = Length::<N>::from(INCH))]
    Inch,
    #[unit("l. y.", "light years", by = Length::<N>::from(LIGHT_YEAR))]
    LightYear,
    #[unit("μ", "microns", by = Length::<N>::from(MICRON))]
    Micron,
    #[unit("0.001 in", "mils", by = Length::<N>::from(MIL))]
    Mil,
    #[unit("mi", "miles", by = Length::<N>::from(MILE))]
    Mile,
    #[unit("mi (U.S. survey)", "miles (U.S. survey)", by = Length::<N>::from(MILE_SURVEY))]
    MileSurvey,
    #[unit("M", "nautical miles", by = Length::<N>::from(NAUTICAL_MILE))]
    NauticalMile,
    #[unit("pc", "parsecs", by = Length::<N>::from(PARSEC))]
    Parsec,
    #[unit("1/6 in (computer)", "picas (computer)", by = Length::<N>::from(PICA_COMPUTER))]
    PicaComputer,
    #[unit("1/6 in", "picas (printer's)", by = Length::<N>::from(PICA_PRINTERS))]
    PicaPrinters,
    #[unit("1/72 in (computer)", "points (computer)", by = Length::<N>::from(POINT_COMPUTER))]
    PointComputer,
    #[unit("1/72 in", "points (printer's)", by = Length::<N>::from(POINT_PRINTERS))]
    PointPrinters,
    #[unit("rd", "rods", by = Length::<N>::from(ROD))]
    Rod,
    #[unit("yd", "yards", by = Length::<N>::from(YARD))]
    Yard,
    #[unit("leuga", "leugae", by = Length::<N>::from(LEUGA))]
    Leuga,
    #[unit("mille passus", "mille passa", by = Length::<N>::from(MILLE_PASSUS))]
    MillePassus,
    #[unit("stadium", "stadia", by = Length::<N>::from(STADIUM))]
    Stadium,
    #[unit("actus", "acta", by = Length::<N>::from(ACTUS))]
    Actus,
    #[unit("decempeda", "decempedae", by = Length::<N>::from(DECEMPEDA))]
    Decempeda,
    #[unit("passus", "passa", by = Length::<N>::from(PASSUS))]
    Passus,
    #[unit("gradus", "gradus", by = Length::<N>::from(GRADUS))]
    Gradus,
    #[unit("cubitum", "cubita", by = Length::<N>::from(CUBITUM))]
    Cubitum,
    #[unit("palmipes", "palmipedes", by = Length::<N>::from(PALMIPES))]
    Palmipes,
    #[unit("pes", "pedes", by = Length::<N>::from(PES))]
    Pes,
    #[unit("palmus maior", "palmas maior", by = Length::<N>::from(PALMUS_MAIOR))]
    PalmusMaior,
    #[unit("palmus", "palmas", by = Length::<N>::from(PALMUS))]
    Palmus,
    #[unit("uncia", "unciae", by = Length::<N>::from(LENGTH_UNCIA))]
    Uncia,
    #[unit("digitus", "digiti", by = Length::<N>::from(DIGITUS))]
    Digitus,
}

/// The units of `Mass`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N> where Mass<N>: [const] yadc::Unit<Mass<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassUnit {
    #[default]
    #[unit("kg", "kilograms", by = Mass::<N>::from(KILOGRAM))]
    Kilogram,
    #[unit("g", "grams", by = Mass::<N>::from(GRAM))]
    Gram,
    #[unit("ct", "carats", by = Mass::<N>::from(CARAT))]
    Carat,
    #[unit("Da", "daltons", by = Mass::<N>::from(DALTON))]
    Dalton,
    #[unit("mₑ", "electron masses", by = Mass::<N>::from(ELECTRON_MASS))]
    ElectronMass,
    #[unit("mₚ", "proton masses", by = Mass::<N>::from(PROTON_MASS))]
    ProtonMass,
    #[unit("gr", "grains", by = Mass::<N>::from(GRAIN))]
    Grain,
    #[unit("cwt long", "hundredweight (long)", by = Mass::<N>::from(HUNDREDWEIGHT_LONG))]
    HundredweightLong,
    #[unit("cwt short", "hundredweight (short)", by = Mass::<N>::from(HUNDREDWEIGHT_SHORT))]
    HundredweightShort,
    #[unit("oz", "ounces", by = Mass::<N>::from(OUNCE))]
    Ounce,
    #[unit("oz t", "troy ounces", by = Mass::<N>::from(OUNCE_TROY))]
    OunceTroy,
    #[unit("dwt", "pennyweight", by = Mass::<N>::from(PENNYWEIGHT))]
    Pennyweight,
    #[unit("lb", "pounds", by = Mass::<N>::from(POUND))]
    Pound,
    #[unit("lb t", "troy pounds", by = Mass::<N>::from(POUND_TROY))]
    PoundTroy,
    #[unit("slug", "slugs", by = Mass::<N>::from(SLUG))]
    Slug,
    #[unit("AT", "assay tons", by = Mass::<N>::from(TON_ASSAY))]
    TonAssay,
    #[unit("2240 lb", "long tons", by = Mass::<N>::from(TON_LONG))]
    TonLong,
    #[unit("2000 lb", "short tons", by = Mass::<N>::from(TON_SHORT))]
    TonShort,
    #[unit("t", "tons", by = Mass::<N>::from(TON))]
    Ton,
    #[unit("libra", "librae", by = Mass::<N>::from(LIBRA))]
    Libra,
    #[unit("deunx", "deunces", by = Mass::<N>::from(MASS_DEUNX))]
    Deunx,
    #[unit("dextans", "dextantes", by = Mass::<N>::from(MASS_DEXTANS))]
    Dextans,
    #[unit("dodrans", "dodrantes", by = Mass::<N>::from(MASS_DODRANS))]
    Dodrans,
    #[unit("bes", "bessis", by = Mass::<N>::from(MASS_BES))]
    Bes,
    #[unit("septunx", "septunces", by = Mass::<N>::from(MASS_SEPTUNX))]
    Septunx,
    #[unit("semis", "semisses", by = Mass::<N>::from(MASS_SEMIS))]
    Semis,
    #[unit("quincunx", "quincunx", by = Mass::<N>::from(MASS_QUINCUNX))]
    Quincunx,
    #[unit("triens", "trientes", by = Mass::<N>::from(MASS_TRIENS))]
    Triens,
    #[unit("quadrans teruncius", "quadrantes teruncius", by = Mass::<N>::from(QUADRANS_TERUNCIUS))]
    QuadransTeruncius,
    #[unit("sextans", "sextantes", by = Mass::<N>::from(MASS_SEXTANS))]
    Sextans,
    #[unit("sescuncia", "sescunciae", by = Mass::<N>::from(SESCUNCIA))]
    Sescuncia,
    #[unit("uncia", "unciae", by = Mass::<N>::from(MASS_UNCIA))]
    Uncia,
    #[unit("semuncia", "semunciae", by = Mass::<N>::from(SEMUNCIA))]
    Semuncia,
    #[unit("duella", "duella", by = Mass::<N>::from(DUELLA))]
    Duella,
    #[unit("sicilicus", "scilici", by = Mass::<N>::from(MASS_SICILICUS))]
    Sicilicus,
    #[unit("sextula", "sextulae", by = Mass::<N>::from(MASS_SEXTULA))]
    Sextula,
    #[unit("semisextula", "semisextulae", by = Mass::<N>::from(SEMISEXTULA))]
    Semisextula,
    #[unit("scrupulum", "scrupula", by = Mass::<N>::from(MASS_SCRUPULUM))]
    Scrupulum,
    #[unit("obolus", "oboli", by = Mass::<N>::from(OBOLUS))]
    Obolus,
    #[unit("siliqua", "siliquae", by = Mass::<N>::from(SILIQUA))]
    Siliqua,
}

/// The units of `Time`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Time<N>: [const] yadc::Unit<Time<N>, N> + [const] core::marker::Destruct)]
pub enum TimeUnit {
    #[default]
    #[unit("s", "seconds", by = Time::<N>::from(SECOND))]
    Second,
    #[unit("s (sidereal)", "seconds (sidereal)", by = Time::<N>::from(SECOND_SIDEREAL))]
    SecondSidereal,
    #[unit("d", "days", by = Time::<N>::from(DAY))]
    Day,
    #[unit("d (sidereal)", "days (sidereal)", by = Time::<N>::from(DAY_SIDEREAL))]
    DaySidereal,
    #[unit("h", "hours", by = Time::<N>::from(HOUR))]
    Hour,
    #[unit("h (sidereal)", "hours (sidereal)", by = Time::<N>::from(HOUR_SIDEREAL))]
    HourSidereal,
    #[unit("min", "minutes", by = Time::<N>::from(MINUTE))]
    Minute,
    #[unit("10.0 ns", "shakes", by = Time::<N>::from(SHAKE))]
    Shake,
    #[unit("a", "years", by = Time::<N>::from(YEAR))]
    Year,
    #[unit("a (sidereal)", "years (sidereal)", by = Time::<N>::from(YEAR_SIDEREAL))]
    YearSidereal,
    #[unit("a (tropical)", "years (tropical)", by = Time::<N>::from(YEAR_TROPICAL))]
    YearTropical,
}

/// The units of `Instant`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Instant<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Scale<Instant<N>, Time<N>>: [const] yadc::Unit<Instant<N>, N> + [const] core::marker::Destruct)]
pub enum InstantUnit {
    #[default]
    #[unit("s Unix", "seconds of Unix time", by = Scale::<Instant<N>, Time<N>>::from(UNIX_SECONDS))]
    UnixSecond,
    #[unit("ms Unix", "milliseconds of Unix time", by = Scale::<Instant<N>, Time<N>>::from(UNIX_MILLISECONDS))]
    UnixMillisecond,
    #[unit("s GPS", "seconds of GPS time", by = Scale::<Instant<N>, Time<N>>::from(GPS_SECONDS))]
    GpsSecond,
    #[unit("JD", "Julian date", by = Scale::<Instant<N>, Time<N>>::from(JULIAN_DATE))]
    JulianDate,
    #[unit("MJD", "modified Julian date", by = Scale::<Instant<N>, Time<N>>::from(MODIFIED_JULIAN_DATE))]
    ModifiedJulianDate,
}

/// The units of `Area`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N> where Area<N>: [const] yadc::Unit<Area<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AreaUnit {
    #[default]
    #[unit("m²", "square metres", by = Area::<N>::from(SQUARE_METRE))]
    SquareMetre,
    #[unit("ac", "acres", by = Area::<N>::from(ACRE))]
    Acre,
    #[unit("a", "ares", by = Area::<N>::from(ARE))]
    Are,
    #[unit("b", "barns", by = Area::<N>::from(BARN))]
    Barn,
    #[unit("cmil", "circular mils", by = Area::<N>::from(CIRCULAR_MIL))]
    CircularMil,
    #[unit("ha", "hectares", by = Area::<N>::from(HECTARE))]
    Hectare,
    #[unit("ft²", "square feet", by = Area::<N>::from(SQUARE_FOOT))]
    SquareFoot,
    #[unit("in²", "square inches", by = Area::<N>::from(SQUARE_INCH))]
    SquareInch,
    #[unit("mi²", "square miles", by = Area::<N>::from(SQUARE_MILE))]
    SquareMile,
    #[unit("yd²", "square yards", by = Area::<N>::from(SQUARE_YARD))]
    SquareYard,
    #[unit("saltus", "saltus", by = Area::<N>::from(SALTUS))]
    Saltus,
    #[unit("centuria", "centuriae", by = Area::<N>::from(CENTURIA))]
    Centuria,
    #[unit("heredium", "heredia", by = Area::<N>::from(HEREDIUM))]
    Heredium,
    #[unit("jugerum", "jugera", by = Area::<N>::from(JUGERUM))]
    Jugerum,
    #[unit("deunx", "deunces", by = Area::<N>::from(AREA_DEUNX))]
    Deunx,
    #[unit("dextans", "dextantes", by = Area::<N>::from(AREA_DEXTANS))]
    Dextans,
    #[unit("dodrans", "dodrantes", by = Area::<N>::from(AREA_DODRANS))]
    Dodrans,
    #[unit("bes", "besses", by = Area::<N>::from(AREA_BES))]
    Bes,
    #[unit("septunx", "septunces", by = Area::<N>::from(AREA_SEPTUNX))]
    Septunx,
    #[unit("semis", "semisses", by = Area::<N>::from(AREA_SEMIS))]
    Semis,
    #[unit("actus quadratus", "acta quadratus", by = Area::<N>::from(ACTUS_QUADRATUS))]
    ActusQuadratus,
    #[unit("quincunx", "quincunces", by = Area::<N>::from(AREA_QUINCUNX))]
    Quincunx,
    #[unit("triens", "trientes", by = Area::<N>::from(AREA_TRIENS))]
    Triens,
    #[unit("quadrans", "quadrantes", by = Area::<N>::from(QUADRANS))]
    Quadrans,
    #[unit("sextans", "sextantes", by = Area::<N>::from(AREA_SEXTANS))]
    Sextans,
    #[unit("clima", "climata", by = Area::<N>::from(CLIMA))]
    Clima,
    #[unit("uncia", "unciae", by = Area::<N>::from(AREA_UNCIA))]
    Uncia,
    #[unit("semiuncia", "semiunciae", by = Area::<N>::from(SEMIUNCIA))]
    Semiuncia,
    #[unit("sicilicus", "siculi", by = Area::<N>::from(AREA_SICILICUS))]
    Sicilicus,
    #[unit("actus simplex", "acta simplex", by = Area::<N>::from(ACTUS_SIMPLEX))]
    ActusSimplex,
    #[unit("sextula", "sextulae", by = Area::<N>::from(AREA_SEXTULA))]
    Sextula,
    #[unit("duo scrupulum", "duo scrupula", by = Area::<N>::from(DUO_SCRUPULUM))]
    DuoScrupulum,
    #[unit("scrupulum", "scrupula", by = Area::<N>::from(AREA_SCRUPULUM))]
    Scrupulum,
    #[unit("dimidium scrupulum", "dimidium scupula", by = Area::<N>::from(DIMIDIUM_SCRUPULUM))]
    DimidiumScrupulum,
    #[unit("pes quadratus", "pedes quadratus", by = Area::<N>::from(PES_QUADRATUS))]
    PesQuadratus,
}

/// The units of `Volume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Volume<N>: [const] yadc::Unit<Volume<N>, N> + [const] core::marker::Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("m³", "cubic metres", by = Volume::<N>::from(CUBIC_METRE))]
    CubicMetre,
    #[unit("ac · ft", "acre-feet", by = Volume::<N>::from(CUBIC_METRE * (Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)))))]
    AcreFoot,
    #[unit("bbl", "barrels", by = Volume::<N>::from(BARREL))]
    Barrel,
    #[unit("bu", "bushels", by = Volume::<N>::from(BUSHEL))]
    Bushel,
    #[unit("cords", "cords", by = Volume::<N>::from(CORD))]
    Cord,
    #[unit("ft³", "cubic feet", by = Volume::<N>::from(CUBIC_FOOT))]
    CubicFoot,
    #[unit("in³", "cubic inches", by = Volume::<N>::from(CUBIC_INCH))]
    CubicInch,
    #[unit("mi³", "cubic miles", by = Volume::<N>::from(CUBIC_MILE))]
    CubicMile,
    #[unit("yd³", "cubic yards", by = Volume::<N>::from(CUBIC_YARD))]
    CubicYard,
    #[unit("cup", "cups", by = Volume::<N>::from(CUP))]
    Cup,
    #[unit("fl oz", "fluid ounces", by = Volume::<N>::from(FLUID_OUNCE))]
    FluidOunce,
    #[unit("fl oz (UK)", "Imperial fluid ounces", by = Volume::<N>::from(FLUID_OUNCE_IMPERIAL))]
    FluidOunceImperial,
    #[unit("gal (UK)", "Imperial gallons", by = Volume::<N>::from(GALLON_IMPERIAL))]
    GallonImperial,
    #[unit("gal", "gallons", by = Volume::<N>::from(GALLON))]
    Gallon,
    #[unit("gi (UK)", "Imperial gills", by = Volume::<N>::from(GILL_IMPERIAL))]
    GillImperial,
    #[unit("gi", "gills", by = Volume::<N>::from(GILL))]
    Gill,
    #[unit("L", "litres", by = Volume::<N>::from(LITRE))]
    Litre,
    #[unit("pk", "pecks", by = Volume::<N>::from(PECK))]
    Peck,
    #[unit("dry pt", "dry pints", by = Volume::<N>::from(PINT_DRY))]
    PintDry,
    #[unit("liq pt", "liquid pints", by = Volume::<N>::from(PINT_LIQUID))]
    PintLiquid,
    #[unit("dry qt", "dry quarts", by = Volume::<N>::from(QUART_DRY))]
    QuartDry,
    #[unit("liq qt", "liquid quarts", by = Volume::<N>::from(QUART_LIQUID))]
    QuartLiquid,
    #[unit("st", "steres", by = Volume::<N>::from(STERE))]
    Stere,
    #[unit("tbsp", "tablespoons", by = Volume::<N>::from(TABLESPOON))]
    Tablespoon,
    #[unit("tsp", "teaspoons", by = Volume::<N>::from(TEASPOON))]
    Teaspoon,
    #[unit("RT", "register tons", by = Volume::<N>::from(REGISTER_TON))]
    RegisterTon,
    #[unit("culeus", "culei", by = Volume::<N>::from(CULEUS))]
    Culeus,
    #[unit("amphora quadrantal", "amphora quadrantal", by = Volume::<N>::from(AMPHORA_QUADRANTAL))]
    AmphoraQuadrantal,
    #[unit("urna", "urnae", by = Volume::<N>::from(URNA))]
    Urna,
    #[unit("modius castrensis", "modii castrensis", by = Volume::<N>::from(MODIUS_CASTRENSIS))]
    ModiusCastrensis,
    #[unit("modius", "modii", by = Volume::<N>::from(MODIUS))]
    Modius,
    #[unit("semimodius", "semimodii", by = Volume::<N>::from(SEMIMODIUS))]
    Semimodius,
    #[unit("congius", "congii", by = Volume::<N>::from(CONGIUS))]
    Congius,
    #[unit("sextarius", "sextarii", by = Volume::<N>::from(SEXTARIUS))]
    Sextarius,
    #[unit("hemina", "heminae", by = Volume::<N>::from(HEMINA))]
    Hemina,
    #[unit("quartarius", "quartarii", by = Volume::<N>::from(QUARTARIUS))]
    Quartarius,
    #[unit("acetabulum", "acetabula", by = Volume::<N>::from(ACETABULUM))]
    Acetabulum,
    #[unit("cyathus", "cyathi", by = Volume::<N>::from(CYATHUS))]
    Cyathus,
    #[unit("ligula", "ligula", by = Volume::<N>::from(LIGULA))]
    Ligula,
}

/// The units of `Velocity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Velocity<N>: [const] yadc::Unit<Velocity<N>, N> + [const] core::marker::Destruct)]
pub enum VelocityUnit {
    #[default]
    #[unit("m/s", "metres per second", by = Velocity::<N>::from(METRE_PER_SECOND))]
    MetrePerSecond,
    #[unit("ft/h", "feet per hour", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::dec(3048, -4) / Exact::int(3600))))]
    FootPerHour,
    #[unit("ft/min", "feet per minute", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(508, -5)))]
    FootPerMinute,
    #[unit("ft/s", "feet per second", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(3048, -4)))]
    FootPerSecond,
    #[unit("in/s", "inches per second", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(254, -4)))]
    InchPerSecond,
    #[unit("in/min", "inches per minute", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::dec(254, -4) / Exact::int(60))))]
    InchPerMinute,
    #[unit("kn", "knots", by = Velocity::<N>::from(KNOT))]
    Knot,
    #[unit("mi/h", "miles per hour", by = Velocity::<N>::from(MILE_PER_HOUR))]
    MilePerHour,
    #[unit("mi/min", "miles per minute", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(268224, -4)))]
    MilePerMinute,
    #[unit("mi/s", "miles per second", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(1609344, -3)))]
    MilePerSecond,
    #[unit("a₀ · Eₕ/ħ", "atomic units of velocity", by = Velocity::<N>::from(METRE_PER_SECOND * Exact::dec(218769126364, -5)))]
    AtomicUnitOfVelocity,
    #[unit("c", "natural units of velocity", by = Velocity::<N>::from(NATURAL_UNIT_OF_VELOCITY))]
    NaturalUnitOfVelocity,
    #[unit("c", "speeds of light in vacuum", by = Velocity::<N>::from(SPEED_OF_LIGHT))]
    SpeedOfLight,
    #[unit("km/h", "kilometres per hour", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::int(1000) / Exact::int(3600))))]
    KilometrePerHour,
    #[unit("mm/min", "millimetres per minute", by = Velocity::<N>::from(METRE_PER_SECOND * (Exact::dec(1, -3) / Exact::int(60))))]
    MillimetrePerMinute,
}

/// The units of `Acceleration`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N)]
#[unit(for<N> where Acceleration<N>: [const] yadc::Unit<Acceleration<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AccelerationUnit {
    #[default]
    #[unit("m/s²", "metres per second squared", by = Acceleration::<N>::from(METRE_PER_SECOND_SQUARED))]
    MetrePerSecondSquared,
    #[unit("ft/s²", "feet per second squared", by = Acceleration::<N>::from(METRE_PER_SECOND_SQUARED * Exact::dec(3048, -4)))]
    FootPerSecondSquared,
    #[unit("Gal", "galileos", by = Acceleration::<N>::from(GALILEO))]
    Galileo,
    #[unit("in/s²", "inches per second squared", by = Acceleration::<N>::from(METRE_PER_SECOND_SQUARED * Exact::dec(254, -4)))]
    InchPerSecondSquared,
    #[unit("g₀", "standard accelerations of gravity", by = Acceleration::<N>::from(STANDARD_GRAVITY))]
    StandardGravity,
    #[unit("mm/min²", "millimetres per minute squared", by = Acceleration::<N>::from(METRE_PER_SECOND_SQUARED * (Exact::dec(1, -3) / (Exact::int(60) * Exact::int(60)))))]
    MillimetrePerMinuteSquared,
}

/// The units of `Jerk`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Jerk<N>, number = N)]
#[unit(for<N> where Jerk<N>: [const] yadc::Unit<Jerk<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum JerkUnit {
    #[default]
    #[unit("m/s³", "metres per second cubed", by = Jerk::<N>::from(METRE / (SECOND * SECOND * SECOND)))]
    MetrePerSecondCubed,
    #[unit("ft/s³", "feet per second cubed", by = Jerk::<N>::from((METRE / (SECOND * SECOND * SECOND)) * Exact::dec(3048, -4)))]
    FootPerSecondCubed,
    #[unit("in/s³", "inches per second cubed", by = Jerk::<N>::from((METRE / (SECOND * SECOND * SECOND)) * Exact::dec(254, -4)))]
    InchPerSecondCubed,
    #[unit("km/min³", "kilometres per minute cubed", by = Jerk::<N>::from((METRE / (SECOND * SECOND * SECOND)) * (Exact::int(1000) / (Exact::int(60) * Exact::int(60) * Exact::int(60)))))]
    KilometrePerMinuteCubed,
}

/// The units of `Absement`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Absement<N>, number = N)]
#[unit(for<N> where Absement<N>: [const] yadc::Unit<Absement<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AbsementUnit {
    #[default]
    #[unit("m · s", "metre seconds", by = Absement::<N>::from(METRE * SECOND))]
    MetreSecond,
    #[unit("ft · s", "foot seconds", by = Absement::<N>::from((METRE * SECOND) * Exact::dec(3048, -4)))]
    FootSecond,
    #[unit("in · s", "inch seconds", by = Absement::<N>::from((METRE * SECOND) * Exact::dec(254, -4)))]
    InchSecond,
    #[unit("km · h", "kilometre hours", by = Absement::<N>::from((METRE * SECOND) * Exact::int(3600000)))]
    KilometreHour,
}

/// The units of `InverseVelocity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = InverseVelocity<N>, number = N)]
#[unit(for<N> where InverseVelocity<N>: [const] yadc::Unit<InverseVelocity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum InverseVelocityUnit {
    #[default]
    #[unit("s/m", "seconds per metre", by = InverseVelocity::<N>::from(SECOND / METRE))]
    SecondPerMetre,
    #[unit("h/ft", "hours per foot", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(3600) / Exact::dec(3048, -4))))]
    HourPerFoot,
    #[unit("min/ft", "minutes per foot", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(60) / Exact::dec(3048, -4))))]
    MinutePerFoot,
    #[unit("s/ft", "seconds per foot", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(1) / Exact::dec(3048, -4))))]
    SecondPerFoot,
    #[unit("s/in", "seconds per inch", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(1) / Exact::dec(254, -4))))]
    SecondPerInch,
    #[unit("h/mi", "hours per mile", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(3600) / Exact::dec(1609344, -3))))]
    HourPerMile,
    #[unit("min/mi", "minutes per mile", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(60) / Exact::dec(1609344, -3))))]
    MinutePerMile,
    #[unit("s/mi", "seconds per mile", by = InverseVelocity::<N>::from((SECOND / METRE) * (Exact::int(1) / Exact::dec(1609344, -3))))]
    SecondPerMile,
    #[unit("min/km", "minutes per kilometre", by = InverseVelocity::<N>::from((SECOND / METRE) * Exact::dec(6, -2)))]
    MinutePerKilometre,
    #[unit("h/km", "hours per kilometre", by = InverseVelocity::<N>::from((SECOND / METRE) * Exact::dec(36, -1)))]
    HourPerKilometre,
}

/// The units of `Force`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N)]
#[unit(for<N> where Force<N>: [const] yadc::Unit<Force<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ForceUnit {
    #[default]
    #[unit("N", "newtons", by = Force::<N>::from(NEWTON))]
    Newton,
    #[unit("dyn", "dynes", by = Force::<N>::from(DYNE))]
    Dyne,
    #[unit("kgf", "kilograms-force", by = Force::<N>::from(KILOGRAM_FORCE))]
    KilogramForce,
    #[unit("kip", "kips", by = Force::<N>::from(KIP))]
    Kip,
    #[unit("ozf", "ounces-force", by = Force::<N>::from(OUNCE_FORCE))]
    OunceForce,
    #[unit("pdl", "poundals", by = Force::<N>::from(POUNDAL))]
    Poundal,
    #[unit("lbf", "pounds-force", by = Force::<N>::from(POUND_FORCE))]
    PoundForce,
    #[unit("2000 lbf", "tons-force", by = Force::<N>::from(TON_FORCE))]
    TonForce,
}

/// The units of `Momentum`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Momentum<N>, number = N)]
#[unit(for<N> where Momentum<N>: [const] yadc::Unit<Momentum<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MomentumUnit {
    #[default]
    #[unit("kg · m/s", "kilogram metres per second", by = Momentum::<N>::from(METRE * KILOGRAM / SECOND))]
    KilogramMetrePerSecond,
    #[unit("g · m/s", "gram metres per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * Exact::dec(1, -3)))]
    GramMetrePerSecond,
    #[unit("t · m/s", "ton metres per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * Exact::int(1000)))]
    TonMetrePerSecond,
    #[unit("kg · m/min", "kilogram metres per minute", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * (Exact::int(1) / Exact::int(60))))]
    KilogramMetrePerMinute,
    #[unit("kg · m/h", "kilogram metres per hour", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * (Exact::int(1) / Exact::int(3600))))]
    KilogramMetrePerHour,
    #[unit("kg · m/d", "kilogram metres per day", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * (Exact::int(1) / Exact::int(86400))))]
    KilogramMetrePerDay,
    #[unit("slug · ft/s", "slug feet per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * Exact::dec(3048, -4))))]
    SlugFootPerSecond,
    #[unit("slug · in/s", "slug inches per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * Exact::dec(254, -4))))]
    SlugInchPerSecond,
    #[unit("lb · ft/s", "pound feet per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * Exact::dec(138254954376, -12)))]
    PoundFootPerSecond,
    #[unit("lb · in/s", "pound inches per second", by = Momentum::<N>::from((METRE * KILOGRAM / SECOND) * Exact::dec(11521246198, -12)))]
    PoundInchPerSecond,
}

/// The units of `Energy`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N> where Energy<N>: [const] yadc::Unit<Energy<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum EnergyUnit {
    #[default]
    #[unit("J", "joules", by = Energy::<N>::from(JOULE))]
    Joule,
    #[unit("W · h", "watt hours", by = Energy::<N>::from(JOULE * Exact::int(3600)))]
    WattHour,
    #[unit("eV", "electronvolts", by = Energy::<N>::from(ELECTRONVOLT))]
    Electronvolt,
    #[unit("Eₕ", "hartrees", by = Energy::<N>::from(HARTREE))]
    Hartree,
    #[unit("Btu (IT)", "British thermal units (IT)", by = Energy::<N>::from(BTU_IT))]
    BtuIt,
    #[unit("Btu", "British thermal units", by = Energy::<N>::from(BTU))]
    Btu,
    #[unit("cal (IT)", "calories (IT)", by = Energy::<N>::from(CALORIE_IT))]
    CalorieIt,
    #[unit("cal", "calories", by = Energy::<N>::from(CALORIE))]
    Calorie,
    #[unit("Cal (IT)", "Calories (IT)", by = Energy::<N>::from(CALORIE_IT_NUTRITION))]
    CalorieItNutrition,
    #[unit("Cal", "Calories", by = Energy::<N>::from(CALORIE_NUTRITION))]
    CalorieNutrition,
    #[unit("erg", "ergs", by = Energy::<N>::from(ERG))]
    Erg,
    #[unit("ft · pdl", "foot poundals", by = Energy::<N>::from(JOULE * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4))))]
    FootPoundal,
    #[unit("ft · lbf", "foot pounds-force", by = Energy::<N>::from(JOULE * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    FootPound,
    #[unit("10¹⁵ Btu (IT)", "quads", by = Energy::<N>::from(QUAD))]
    Quad,
    #[unit("thm (EC)", "therms (EC)", by = Energy::<N>::from(THERM_EC))]
    ThermEc,
    #[unit("thm", "therms", by = Energy::<N>::from(THERM_US))]
    ThermUs,
    #[unit("t of TNT", "tons of TNT", by = Energy::<N>::from(TON_TNT))]
    TonTnt,
    #[unit("W · s", "watt seconds", by = Energy::<N>::from(JOULE))]
    WattSecond,
}

/// The units of `AvailableEnergy`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AvailableEnergy<N>, number = N)]
#[unit(for<N> where AvailableEnergy<N>: [const] yadc::Unit<AvailableEnergy<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AvailableEnergyUnit {
    #[default]
    #[unit("J/kg", "joules per kilogram", by = AvailableEnergy::<N>::from(METRE * METRE / (SECOND * SECOND)))]
    JoulePerKilogram,
    #[unit("J/g", "joules per gram", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * Exact::int(1000)))]
    JoulePerGram,
    #[unit("Btu (IT)/lb", "British thermal units (IT) per pound", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * Exact::int(2326)))]
    BtuItPerPound,
    #[unit("Btu/lb", "British thermal units per pound", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * (Exact::int(4184) / Exact::dec(18, -1))))]
    BtuPerPound,
    #[unit("cal (IT)/g", "calories (IT) per gram", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * Exact::dec(41868, -1)))]
    CalorieItPerGram,
    #[unit("cal/g", "calories per gram", by = AvailableEnergy::<N>::from((METRE * METRE / (SECOND * SECOND)) * Exact::int(4184)))]
    CaloriePerGram,
}

/// The units of `Power`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<f64>, non_const)]
pub enum PowerUnit {
    #[default]
    #[unit("W", "watts", by = Power::<f64>::from(WATT))]
    Watt,
    #[unit("dBW", "decibels-watt", by = Scale::<PowerLevel<f64>, Gain<f64>>::from(DBW), via = PowerLevel<f64>)]
    DecibelWatt,
    #[unit("dBm", "decibels-milliwatt", by = Scale::<PowerLevel<f64>, Gain<f64>>::from(DBM), via = PowerLevel<f64>)]
    DecibelMilliwatt,
    #[unit("erg/s", "ergs per second", by = Power::<f64>::from(WATT * Exact::dec(1, -7)))]
    ErgPerSecond,
    #[unit("ft · lbf/h", "foot pounds-force per hour", by = Power::<f64>::from(WATT * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::int(3600))))]
    FootPoundPerHour,
    #[unit("ft · lbf/min", "foot pounds-force per minute", by = Power::<f64>::from(WATT * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::int(60))))]
    FootPoundPerMinute,
    #[unit("ft · lbf/s", "foot pounds-force per second", by = Power::<f64>::from(WATT * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    FootPoundPerSecond,
    #[unit("hp", "horsepower", by = Power::<f64>::from(HORSEPOWER))]
    Horsepower,
    #[unit("hp (S)", "horsepower (boiler)", by = Power::<f64>::from(HORSEPOWER_BOILER))]
    HorsepowerBoiler,
    #[unit("hp (E)", "horsepower (electric)", by = Power::<f64>::from(HORSEPOWER_ELECTRIC))]
    HorsepowerElectric,
    #[unit("hp (M)", "metric horsepower", by = Power::<f64>::from(HORSEPOWER_METRIC))]
    HorsepowerMetric,
    #[unit("hp (I)", "horsepower (Imperial)", by = Power::<f64>::from(HORSEPOWER_IMPERIAL))]
    HorsepowerImperial,
    #[unit("hp (hydraulic)", "hydraulic horsepower", by = Power::<f64>::from(HYDRAULIC_HORSEPOWER))]
    HydraulicHorsepower,
}

/// The units of `Gain`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Gain<N>, number = N)]
#[unit(for<N> where Gain<N>: [const] yadc::Unit<Gain<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum GainUnit {
    #[default]
    #[unit("B", "bels", by = Gain::<N>::from(BEL))]
    Bel,
    #[unit("dB", "decibels", by = Gain::<N>::from(DECIBEL))]
    Decibel,
    #[unit("Np", "nepers", by = Gain::<N>::from(NEPER))]
    Neper,
}

/// The units of `PowerLevel`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PowerLevel<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Scale<PowerLevel<N>, Gain<N>>: [const] yadc::Unit<PowerLevel<N>, N> + [const] core::marker::Destruct)]
pub enum PowerLevelUnit {
    #[default]
    #[unit("dBW", "decibels-watt", by = Scale::<PowerLevel<N>, Gain<N>>::from(DBW))]
    DecibelWatt,
    #[unit("dBm", "decibels-milliwatt", by = Scale::<PowerLevel<N>, Gain<N>>::from(DBM))]
    DecibelMilliwatt,
}

/// The units of `PowerRate`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PowerRate<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, PowerRate<N>: [const] yadc::Unit<PowerRate<N>, N> + [const] core::marker::Destruct)]
pub enum PowerRateUnit {
    #[default]
    #[unit("W/s", "watts per second", by = PowerRate::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)))]
    WattPerSecond,
    #[unit("erg/s²", "ergs per second squared", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::dec(1, -7)))]
    ErgPerSecondSquared,
    #[unit("ft · lbf/s²", "foot pounds-force per second squared", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * (Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    FootPoundPerSecondSquared,
    #[unit("hp/s", "horsepower per second", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * (Exact::int(550) * Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5))))]
    HorsepowerPerSecond,
    #[unit("hp/s (S)", "horsepower per second (boiler)", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::dec(98095, -1)))]
    HorsepowerPerSecondBoiler,
    #[unit("hp/s (E)", "horsepower per second (electric)", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::int(746)))]
    HorsepowerPerSecondElectric,
    #[unit("hp/s (M)", "metric horsepower per second", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * (Exact::int(75) * Exact::dec(980665, -5))))]
    HorsepowerPerSecondMetric,
    #[unit("hp/s (I)", "horsepower per second (Imperial)", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::dec(7457, -1)))]
    HorsepowerPerSecondImperial,
    #[unit("hp/s (hydraulic)", "hydraulic horsepower per second", by = PowerRate::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)) * Exact::dec(746043, -3)))]
    HydraulicHorsepowerPerSecond,
}

/// The units of `Action`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Action<N>, number = N)]
#[unit(for<N> where Action<N>: [const] yadc::Unit<Action<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ActionUnit {
    #[default]
    #[unit("J · s", "joule seconds", by = Action::<N>::from(METRE * METRE * KILOGRAM / SECOND))]
    JouleSecond,
    #[unit("ħ", "atomic units of action", by = Action::<N>::from(ATOMIC_UNIT_OF_ACTION))]
    AtomicUnitOfAction,
    #[unit("h", "Planck constants", by = Action::<N>::from(PLANCK_CONSTANT))]
    PlanckConstant,
    #[unit("erg · s", "erg seconds", by = Action::<N>::from((METRE * METRE * KILOGRAM / SECOND) * Exact::dec(1, -7)))]
    ErgSecond,
    #[unit("eV · s", "electronvolt seconds", by = Action::<N>::from((METRE * METRE * KILOGRAM / SECOND) * Exact::dec(1602176634, -28)))]
    ElectronvoltSecond,
}

/// The units of `Torque`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Torque<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Torque<N>: [const] yadc::Unit<Torque<N>, N> + [const] core::marker::Destruct)]
pub enum TorqueUnit {
    #[default]
    #[unit("N · m", "newton metres", by = Torque::<N>::from(METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)))]
    NewtonMetre,
    #[unit("dyn · m", "dyne metres", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * Exact::dec(1, -5)))]
    DyneMetre,
    #[unit("kgf · m", "kilogram-force metres", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * Exact::dec(980665, -5)))]
    KilogramForceMetre,
    #[unit("ozf · in", "ounces-force inches", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16)) * Exact::dec(254, -4))))]
    OunceForceInch,
    #[unit("lbf · ft", "pounds-force feet", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(3048, -4))))]
    PoundForceFoot,
    #[unit("lbf · in", "pounds-force inches", by = Torque::<N>::from((METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)) * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(254, -4))))]
    PoundForceInch,
}

/// The units of `MomentOfInertia`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MomentOfInertia<N>, number = N)]
#[unit(for<N> where MomentOfInertia<N>: [const] yadc::Unit<MomentOfInertia<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MomentOfInertiaUnit {
    #[default]
    #[unit("kg · m²", "kilogram square metres", by = MomentOfInertia::<N>::from(METRE * METRE * KILOGRAM / (RADIAN * RADIAN)))]
    KilogramSquareMetre,
    #[unit("Da · Å²", "dalton square ångströms", by = MomentOfInertia::<N>::from((METRE * METRE * KILOGRAM / (RADIAN * RADIAN)) * Exact::dec(16605390666, -57)))]
    DaltonSquareAngstrom,
}

/// The units of `Pressure`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Pressure<N>: [const] yadc::Unit<Pressure<N>, N> + [const] core::marker::Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("Pa", "pascals", by = Pressure::<N>::from(PASCAL))]
    Pascal,
    #[unit("atm", "standard atmospheres", by = Pressure::<N>::from(STANDARD_ATMOSPHERE))]
    StandardAtmosphere,
    #[unit("at", "atmospheres (technical)", by = Pressure::<N>::from(ATMOSPHERE_TECHNICAL))]
    AtmosphereTechnical,
    #[unit("bar", "bar", by = Pressure::<N>::from(BAR))]
    Bar,
    #[unit("cm Hg", "centimetres of mercury", by = Pressure::<N>::from(CENTIMETRE_OF_MERCURY))]
    CentimetreOfMercury,
    #[unit("cm H₂O", "centimetres of water", by = Pressure::<N>::from(CENTIMETRE_OF_WATER))]
    CentimetreOfWater,
    #[unit("ft Hg", "feet of mercury", by = Pressure::<N>::from(FOOT_OF_MERCURY))]
    FootOfMercury,
    #[unit("ft H₂O", "feet of water", by = Pressure::<N>::from(FOOT_OF_WATER))]
    FootOfWater,
    #[unit("in Hg", "inches of mercury", by = Pressure::<N>::from(INCH_OF_MERCURY))]
    InchOfMercury,
    #[unit("in H₂O", "inches of water", by = Pressure::<N>::from(INCH_OF_WATER))]
    InchOfWater,
    #[unit("kip/in²", "kips per square inch", by = Pressure::<N>::from(PASCAL * ((Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    KipPerSquareInch,
    #[unit("mm Hg", "millimetres of mercury", by = Pressure::<N>::from(MILLIMETRE_OF_MERCURY))]
    MillimetreOfMercury,
    #[unit("mm H₂O", "millimetres of water", by = Pressure::<N>::from(MILLIMETRE_OF_WATER))]
    MillimetreOfWater,
    #[unit("pdl/ft²", "poundals per square foot", by = Pressure::<N>::from(PASCAL * ((Exact::dec(45359237, -8) * Exact::dec(3048, -4)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PoundalPerSquareFoot,
    #[unit("lbf/ft²", "pounds-force per square foot", by = Pressure::<N>::from(PASCAL * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4)))))]
    PoundForcePerSquareFoot,
    #[unit("lbf/in²", "pounds-force per square inch", by = Pressure::<N>::from(PASCAL * ((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(254, -4) * Exact::dec(254, -4)))))]
    PoundForcePerSquareInch,
    #[unit("psi", "pounds-force per square inch", by = Pressure::<N>::from(PSI))]
    Psi,
    #[unit("Torr", "torr", by = Pressure::<N>::from(TORR))]
    Torr,
    #[unit("dyn/cm²", "dynes per square centimetre", by = Pressure::<N>::from(PASCAL * Exact::dec(1, -1)))]
    DynePerSquareCentimetre,
    #[unit("N/mm²", "newtons per square millimetre", by = Pressure::<N>::from(PASCAL * Exact::int(1000000)))]
    NewtonPerSquareMillimetre,
}

/// The units of `Frequency`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Frequency<N>, number = N)]
#[unit(for<N> where Frequency<N>: [const] yadc::Unit<Frequency<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum FrequencyUnit {
    #[default]
    #[unit("Hz", "hertz", by = Frequency::<N>::from(HERTZ))]
    Hertz,
    #[unit("1/d", "cycles per day", by = Frequency::<N>::from(HERTZ * (Exact::int(1) / Exact::int(86400))))]
    CyclePerDay,
    #[unit("1/h", "cycles per hour", by = Frequency::<N>::from(HERTZ * (Exact::int(1) / Exact::int(3600))))]
    CyclePerHour,
    #[unit("1/min", "cycles per minute", by = Frequency::<N>::from(HERTZ * (Exact::int(1) / Exact::int(60))))]
    CyclePerMinute,
    #[unit("100 MHz", "cycles per shake", by = Frequency::<N>::from(CYCLE_PER_SHAKE))]
    CyclePerShake,
    #[unit("1/a", "cycles per year", by = Frequency::<N>::from(HERTZ * (Exact::int(1) / Exact::int(31536000))))]
    CyclePerYear,
}

/// The units of `FrequencyDrift`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = FrequencyDrift<N>, number = N)]
#[unit(for<N> where FrequencyDrift<N>: [const] yadc::Unit<FrequencyDrift<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum FrequencyDriftUnit {
    #[default]
    #[unit("Hz/s", "hertz per second", by = FrequencyDrift::<N>::from(ONE / (SECOND * SECOND)))]
    HertzPerSecond,
}

/// The units of `ReciprocalLength`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReciprocalLength<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, ReciprocalLength<N>: [const] yadc::Unit<ReciprocalLength<N>, N> + [const] core::marker::Destruct)]
pub enum ReciprocalLengthUnit {
    #[default]
    #[unit("m⁻¹", "reciprocal metres", by = ReciprocalLength::<N>::from(RECIPROCAL_METRE))]
    ReciprocalMetre,
    #[unit("Å⁻¹", "reciprocal ångströms", by = ReciprocalLength::<N>::from(RECIPROCAL_ANGSTROM))]
    ReciprocalAngstrom,
    #[unit("dpt", "diopters", by = ReciprocalLength::<N>::from(DIOPTER))]
    Diopter,
    #[unit("R∞", "Rydberg constants", by = ReciprocalLength::<N>::from(RYDBERG_CONSTANT))]
    RydbergConstant,
}

/// The units of `Curvature`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Curvature<N>, number = N)]
#[unit(for<N> where Curvature<N>: [const] yadc::Unit<Curvature<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum CurvatureUnit {
    #[default]
    #[unit("rad/m", "radians per metre", by = Curvature::<N>::from(RADIAN / METRE))]
    RadianPerMetre,
    #[unit("°/m", "degrees per metre", by = Curvature::<N>::from((RADIAN / METRE) * (Exact::PI / Exact::int(180))))]
    DegreePerMetre,
}

/// The units of `SpecificArea`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificArea<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SpecificArea<N>: [const] yadc::Unit<SpecificArea<N>, N> + [const] core::marker::Destruct)]
pub enum SpecificAreaUnit {
    #[default]
    #[unit("m²/kg", "square metres per kilogram", by = SpecificArea::<N>::from(METRE * METRE / KILOGRAM))]
    SquareMetrePerKilogram,
    #[unit("m²/g", "square metres per gram", by = SpecificArea::<N>::from((METRE * METRE / KILOGRAM) * Exact::int(1000)))]
    SquareMetrePerGram,
}

/// The units of `SpecificVolume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificVolume<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SpecificVolume<N>: [const] yadc::Unit<SpecificVolume<N>, N> + [const] core::marker::Destruct)]
pub enum SpecificVolumeUnit {
    #[default]
    #[unit("m³/kg", "cubic metres per kilogram", by = SpecificVolume::<N>::from(METRE * METRE * METRE / KILOGRAM))]
    CubicMetrePerKilogram,
    #[unit("m³/g", "cubic metres per gram", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * Exact::int(1000)))]
    CubicMetrePerGram,
    #[unit("m³/ct", "cubic metres per carat", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * Exact::int(5000)))]
    CubicMetrePerCarat,
    #[unit("m³/gr", "cubic metres per grain", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(6479891, -11))))]
    CubicMetrePerGrain,
    #[unit("m³/cwt long", "cubic metres per hundredweight (long)", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(5080234544, -8))))]
    CubicMetrePerHundredweightLong,
    #[unit("m³/cwt short", "cubic metres per hundredweight (short)", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(45359237, -6))))]
    CubicMetrePerHundredweightShort,
    #[unit("m³/oz", "cubic metres per ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(28349523125, -12))))]
    CubicMetrePerOunce,
    #[unit("m³/oz t", "cubic metres per troy ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(311034768, -10))))]
    CubicMetrePerOunceTroy,
    #[unit("m³/dwt", "cubic metres per pennyweight", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(155517384, -11))))]
    CubicMetrePerPennyweight,
    #[unit("m³/lb", "cubic metres per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(45359237, -8))))]
    CubicMetrePerPound,
    #[unit("m³/lb t", "cubic metres per troy pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(3732417216, -10))))]
    CubicMetrePerPoundTroy,
    #[unit("m³/slug", "cubic metres per slug", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)))))]
    CubicMetrePerSlug,
    #[unit("m³/AT", "cubic metres per assay ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / (Exact::dec(175, -3) / Exact::int(6)))))]
    CubicMetrePerTonAssay,
    #[unit("m³/2240 lb", "cubic metres per long ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(10160469088, -7))))]
    CubicMetrePerTonLong,
    #[unit("m³/2000 lb", "cubic metres per short ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::int(1) / Exact::dec(90718474, -5))))]
    CubicMetrePerTonShort,
    #[unit("m³/t", "cubic metres per ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * Exact::dec(1, -3)))]
    CubicMetrePerTon,
    #[unit("gal/gr", "gallons per grain", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(3785411784, -12) / Exact::dec(6479891, -11))))]
    GallonPerGrain,
    #[unit("in³/oz", "cubic inches per ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4) / Exact::dec(28349523125, -12))))]
    CubicInchPerOunce,
    #[unit("gal (UK)/oz", "Imperial gallons per ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(454609, -8) / Exact::dec(28349523125, -12))))]
    GallonImperialPerOunce,
    #[unit("gal/oz", "gallons per ounce", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(3785411784, -12) / Exact::dec(28349523125, -12))))]
    GallonPerOunce,
    #[unit("ft³/lb", "cubic feet per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(45359237, -8))))]
    CubicFootPerPound,
    #[unit("in³/lb", "cubic inches per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)) / Exact::dec(45359237, -8))))]
    CubicInchPerPound,
    #[unit("yd³/lb", "cubic yards per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(45359237, -8))))]
    CubicYardPerPound,
    #[unit("gal (UK)/lb", "Imperial gallons per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(454609, -8) / Exact::dec(45359237, -8))))]
    GallonImperialPerPound,
    #[unit("gal/lb", "gallons per pound", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(3785411784, -12) / Exact::dec(45359237, -8))))]
    GallonPerPound,
    #[unit("ft³/slug", "cubic feett per slug", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4) / (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)))))]
    CubicFootPerSlug,
    #[unit("yd³/2240 lb", "cubic yards per long ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(10160469088, -7))))]
    CubicYardPerTonLong,
    #[unit("yd³/2000 lb", "cubic yards per short ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(90718474, -5))))]
    CubicYardPerTonShort,
    #[unit("ft³/2240 lb", "cubic feet per long ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(10160469088, -7))))]
    CubicFootPerTonLong,
    #[unit("ft³/2000 lb", "cubic feet per short ton", by = SpecificVolume::<N>::from((METRE * METRE * METRE / KILOGRAM) * ((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(90718474, -5))))]
    CubicFootPerTonShort,
}

/// The units of `SpecificPower`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificPower<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SpecificPower<N>: [const] yadc::Unit<SpecificPower<N>, N> + [const] core::marker::Destruct)]
pub enum SpecificPowerUnit {
    #[default]
    #[unit("W/kg", "watts per kilogram", by = SpecificPower::<N>::from(METRE * METRE / (SECOND * SECOND * SECOND)))]
    WattPerKilogram,
    #[unit("m² · s⁻³", "square metres per cubic second", by = SpecificPower::<N>::from(METRE * METRE / (SECOND * SECOND * SECOND)))]
    SquareMetrePerCubicSecond,
    #[unit("(erg/s)/kg", "ergs per second per kilogram", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * Exact::dec(1, -7)))]
    ErgPerSecondPerKilogram,
    #[unit("(ft · lbf/h)/lb", "foot pounds-force per hour per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::int(3600) * Exact::dec(45359237, -8)))))]
    FootPoundPerHourPerPound,
    #[unit("(ft · lbf/min)/lb", "foot pounds-force per minute per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::int(60) * Exact::dec(45359237, -8)))))]
    FootPoundPerMinutePerPound,
    #[unit("(ft · lbf/s)/lb", "foot pounds-force per second per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8))))]
    FootPoundPerSecondPerPound,
    #[unit("hp/lb", "horsepower per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::int(550) * Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8))))]
    HorsepowerPerPound,
    #[unit("(hp (S))/lb", "horsepower (boiler) per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * (Exact::dec(98095, -1) / Exact::dec(45359237, -8))))]
    HorsepowerBoilerPerPound,
    #[unit("(hp (E))/lb", "horsepower (electric) per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * (Exact::int(746) / Exact::dec(45359237, -8))))]
    HorsepowerElectricPerPound,
    #[unit("(hp (M))/lb", "metric horsepower per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * ((Exact::int(75) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8))))]
    HorsepowerMetricPerPound,
    #[unit("(hp (I))/lb", "horsepower (Imperial) per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * (Exact::dec(7457, -1) / Exact::dec(45359237, -8))))]
    HorsepowerImperialPerPound,
    #[unit("(hp (hydraulic))/lb", "hydraulic horsepower per pound", by = SpecificPower::<N>::from((METRE * METRE / (SECOND * SECOND * SECOND)) * (Exact::dec(746043, -3) / Exact::dec(45359237, -8))))]
    HydraulicHorsepowerPerPound,
}

/// The units of `SpecificGravitationalParameter`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificGravitationalParameter<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SpecificGravitationalParameter<N>: [const] yadc::Unit<SpecificGravitationalParameter<N>, N> + [const] core::marker::Destruct)]
pub enum SpecificGravitationalParameterUnit {
    #[default]
    #[unit("m³/(kg · s²)", "cubic metres per kilogram second squared", by = SpecificGravitationalParameter::<N>::from(METRE * METRE * METRE / (KILOGRAM * SECOND * SECOND)))]
    CubicMetrePerKilogramSecondSquared,
    #[unit("G", "Newtonian constants of gravitation", by = SpecificGravitationalParameter::<N>::from(GRAVITATIONAL_CONSTANT))]
    GravitationalConstant,
}
