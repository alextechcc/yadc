use crate::{Exact, ISQ, Kind, MILLI, SIQuantity, Scale};
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

    /// One `Angstrom` (`Å`), as a `Length`.
    pub const ANGSTROM: Length<Exact> = METRE * Exact::dec(1, -10);

    /// One `BohrRadius` (`a₀`), as a `Length`.
    pub const BOHR_RADIUS: Length<Exact> = METRE * Exact::dec(529177210903, -22);

    /// One `AtomicUnitOfLength` (`a.u. of length`), as a `Length`.
    pub const ATOMIC_UNIT_OF_LENGTH: Length<Exact> = METRE * Exact::dec(529177210903, -22);

    /// One `AstronomicalUnit` (`ua`), as a `Length`.
    pub const ASTRONOMICAL_UNIT: Length<Exact> = METRE * Exact::int(149597870700);

    /// One `Chain` (`ch`), as a `Length`.
    pub const CHAIN: Length<Exact> = METRE * (Exact::int(66) * Exact::int(1200) / Exact::int(3937));

    /// One `DataMile` (`DM`), as a `Length`.
    pub const DATA_MILE: Length<Exact> = METRE * Exact::dec(18288, -1);

    /// One `Fathom` (`fathom`), as a `Length`.
    pub const FATHOM: Length<Exact> = METRE * (Exact::int(6) * Exact::int(1200) / Exact::int(3937));

    /// One `Fermi` (`fermi`), as a `Length`.
    pub const FERMI: Length<Exact> = METRE * Exact::dec(1, -15);

    /// One `Foot` (`ft`), as a `Length`.
    pub const FOOT: Length<Exact> = METRE * Exact::dec(3048, -4);

    /// One `FootSurvey` (`ft (U.S. survey)`), as a `Length`.
    pub const FOOT_SURVEY: Length<Exact> = METRE * (Exact::int(1200) / Exact::int(3937));

    /// One `Inch` (`in`), as a `Length`.
    pub const INCH: Length<Exact> = METRE * Exact::dec(254, -4);

    /// One `LightYear` (`l. y.`), as a `Length`.
    pub const LIGHT_YEAR: Length<Exact> = METRE * Exact::int(9460730472580800);

    /// One `Micron` (`μ`), as a `Length`.
    pub const MICRON: Length<Exact> = METRE * Exact::dec(1, -6);

    /// One `Mil` (`0.001 in`), as a `Length`.
    pub const MIL: Length<Exact> = METRE * Exact::dec(254, -7);

    /// One `Mile` (`mi`), as a `Length`.
    pub const MILE: Length<Exact> = METRE * Exact::dec(1609344, -3);

    /// One `MileSurvey` (`mi (U.S. survey)`), as a `Length`.
    pub const MILE_SURVEY: Length<Exact> = METRE * (Exact::int(6336000) / Exact::int(3937));

    /// One `NauticalMile` (`M`), as a `Length`.
    pub const NAUTICAL_MILE: Length<Exact> = METRE * Exact::int(1852);

    /// One `Parsec` (`pc`), as a `Length`.
    pub const PARSEC: Length<Exact> =
        METRE * (Exact::int(149597870700) * Exact::int(648000) / Exact::PI);

    /// One `PicaComputer` (`1/6 in (computer)`), as a `Length`.
    pub const PICA_COMPUTER: Length<Exact> = METRE * (Exact::dec(254, -4) / Exact::int(6));

    /// One `PicaPrinters` (`1/6 in`), as a `Length`.
    pub const PICA_PRINTERS: Length<Exact> = METRE * Exact::dec(4217518, -9);

    /// One `PointComputer` (`1/72 in (computer)`), as a `Length`.
    pub const POINT_COMPUTER: Length<Exact> = METRE * (Exact::dec(254, -4) / Exact::int(72));

    /// One `PointPrinters` (`1/72 in`), as a `Length`.
    pub const POINT_PRINTERS: Length<Exact> = METRE * Exact::dec(3514598, -10);

    /// One `Rod` (`rd`), as a `Length`.
    pub const ROD: Length<Exact> =
        METRE * (Exact::dec(165, -1) * Exact::int(1200) / Exact::int(3937));

    /// One `Yard` (`yd`), as a `Length`.
    pub const YARD: Length<Exact> = METRE * Exact::dec(9144, -4);

    /// One `Leuga` (`leuga`), as a `Length`.
    pub const LEUGA: Length<Exact> = METRE * Exact::int(2220);

    /// One `MillePassus` (`mille passus`), as a `Length`.
    pub const MILLE_PASSUS: Length<Exact> = METRE * Exact::int(1480);

    /// One `Stadium` (`stadium`), as a `Length`.
    pub const STADIUM: Length<Exact> = METRE * Exact::int(185);

    /// One `Actus` (`actus`), as a `Length`.
    pub const ACTUS: Length<Exact> = METRE * Exact::dec(3552, -2);

    /// One `Decempeda` (`decempeda`), as a `Length`.
    pub const DECEMPEDA: Length<Exact> = METRE * Exact::dec(296, -2);

    /// One `Passus` (`passus`), as a `Length`.
    pub const PASSUS: Length<Exact> = METRE * Exact::dec(148, -2);

    /// One `Gradus` (`gradus`), as a `Length`.
    pub const GRADUS: Length<Exact> = METRE * Exact::dec(74, -2);

    /// One `Cubitum` (`cubitum`), as a `Length`.
    pub const CUBITUM: Length<Exact> = METRE * Exact::dec(444, -3);

    /// One `Palmipes` (`palmipes`), as a `Length`.
    pub const PALMIPES: Length<Exact> = METRE * Exact::dec(37, -2);

    /// One `Pes` (`pes`), as a `Length`.
    pub const PES: Length<Exact> = METRE * Exact::dec(296, -3);

    /// One `PalmusMaior` (`palmus maior`), as a `Length`.
    pub const PALMUS_MAIOR: Length<Exact> = METRE * Exact::dec(222, -3);

    /// One `Palmus` (`palmus`), as a `Length`.
    pub const PALMUS: Length<Exact> = METRE * Exact::dec(74, -3);

    /// One `Uncia` (`uncia`), as a `Length`.
    pub const LENGTH_UNCIA: Length<Exact> = METRE * (Exact::dec(296, -3) / Exact::int(12));

    /// One `Digitus` (`digitus`), as a `Length`.
    pub const DIGITUS: Length<Exact> = METRE * Exact::dec(185, -4);

    /// One `Gram` (`g`), as a `Mass`.
    pub const GRAM: Mass<Exact> = KILOGRAM * Exact::dec(1, -3);

    /// One `Carat` (`ct`), as a `Mass`.
    pub const CARAT: Mass<Exact> = KILOGRAM * Exact::dec(2, -4);

    /// One `Dalton` (`Da`), as a `Mass`.
    pub const DALTON: Mass<Exact> = KILOGRAM * Exact::dec(16605390666, -37);

    /// One `ElectronMass` (`mₑ`), as a `Mass`.
    pub const ELECTRON_MASS: Mass<Exact> = KILOGRAM * Exact::dec(91093837015, -41);

    /// One `ProtonMass` (`mₚ`), as a `Mass`.
    pub const PROTON_MASS: Mass<Exact> = KILOGRAM * Exact::dec(167262192369, -38);

    /// One `Grain` (`gr`), as a `Mass`.
    pub const GRAIN: Mass<Exact> = KILOGRAM * Exact::dec(6479891, -11);

    /// One `HundredweightLong` (`cwt long`), as a `Mass`.
    pub const HUNDREDWEIGHT_LONG: Mass<Exact> = KILOGRAM * Exact::dec(5080234544, -8);

    /// One `HundredweightShort` (`cwt short`), as a `Mass`.
    pub const HUNDREDWEIGHT_SHORT: Mass<Exact> = KILOGRAM * Exact::dec(45359237, -6);

    /// One `Ounce` (`oz`), as a `Mass`.
    pub const OUNCE: Mass<Exact> = KILOGRAM * Exact::dec(28349523125, -12);

    /// One `OunceTroy` (`oz t`), as a `Mass`.
    pub const OUNCE_TROY: Mass<Exact> = KILOGRAM * Exact::dec(311034768, -10);

    /// One `Pennyweight` (`dwt`), as a `Mass`.
    pub const PENNYWEIGHT: Mass<Exact> = KILOGRAM * Exact::dec(155517384, -11);

    /// One `Pound` (`lb`), as a `Mass`.
    pub const POUND: Mass<Exact> = KILOGRAM * Exact::dec(45359237, -8);

    /// One `PoundTroy` (`lb t`), as a `Mass`.
    pub const POUND_TROY: Mass<Exact> = KILOGRAM * Exact::dec(3732417216, -10);

    /// One `Slug` (`slug`), as a `Mass`.
    pub const SLUG: Mass<Exact> =
        KILOGRAM * (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4));

    /// One `TonAssay` (`AT`), as a `Mass`.
    pub const TON_ASSAY: Mass<Exact> = KILOGRAM * (Exact::dec(175, -3) / Exact::int(6));

    /// One `TonLong` (`2240 lb`), as a `Mass`.
    pub const TON_LONG: Mass<Exact> = KILOGRAM * Exact::dec(10160469088, -7);

    /// One `TonShort` (`2000 lb`), as a `Mass`.
    pub const TON_SHORT: Mass<Exact> = KILOGRAM * Exact::dec(90718474, -5);

    /// One `Ton` (`t`), as a `Mass`.
    pub const TON: Mass<Exact> = KILOGRAM * Exact::int(1000);

    /// One `Libra` (`libra`), as a `Mass`.
    pub const LIBRA: Mass<Exact> = KILOGRAM * Exact::dec(3289, -4);

    /// One `Deunx` (`deunx`), as a `Mass`.
    pub const MASS_DEUNX: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(11) / Exact::int(12));

    /// One `Dextans` (`dextans`), as a `Mass`.
    pub const MASS_DEXTANS: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(10) / Exact::int(12));

    /// One `Dodrans` (`dodrans`), as a `Mass`.
    pub const MASS_DODRANS: Mass<Exact> = KILOGRAM * Exact::dec(246675, -6);

    /// One `Bes` (`bes`), as a `Mass`.
    pub const MASS_BES: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(8) / Exact::int(12));

    /// One `Septunx` (`septunx`), as a `Mass`.
    pub const MASS_SEPTUNX: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(7) / Exact::int(12));

    /// One `Semis` (`semis`), as a `Mass`.
    pub const MASS_SEMIS: Mass<Exact> = KILOGRAM * Exact::dec(16445, -5);

    /// One `Quincunx` (`quincunx`), as a `Mass`.
    pub const MASS_QUINCUNX: Mass<Exact> =
        KILOGRAM * (Exact::dec(3289, -4) * Exact::int(5) / Exact::int(12));

    /// One `Triens` (`triens`), as a `Mass`.
    pub const MASS_TRIENS: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(3));

    /// One `QuadransTeruncius` (`quadrans teruncius`), as a `Mass`.
    pub const QUADRANS_TERUNCIUS: Mass<Exact> = KILOGRAM * Exact::dec(82225, -6);

    /// One `Sextans` (`sextans`), as a `Mass`.
    pub const MASS_SEXTANS: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(6));

    /// One `Sescuncia` (`sescuncia`), as a `Mass`.
    pub const SESCUNCIA: Mass<Exact> = KILOGRAM * Exact::dec(411125, -7);

    /// One `Uncia` (`uncia`), as a `Mass`.
    pub const MASS_UNCIA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(12));

    /// One `Semuncia` (`semuncia`), as a `Mass`.
    pub const SEMUNCIA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(24));

    /// One `Duella` (`duella`), as a `Mass`.
    pub const DUELLA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(36));

    /// One `Sicilicus` (`sicilicus`), as a `Mass`.
    pub const MASS_SICILICUS: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(48));

    /// One `Sextula` (`sextula`), as a `Mass`.
    pub const MASS_SEXTULA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(72));

    /// One `Semisextula` (`semisextula`), as a `Mass`.
    pub const SEMISEXTULA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(144));

    /// One `Scrupulum` (`scrupulum`), as a `Mass`.
    pub const MASS_SCRUPULUM: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(288));

    /// One `Obolus` (`obolus`), as a `Mass`.
    pub const OBOLUS: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(576));

    /// One `Siliqua` (`siliqua`), as a `Mass`.
    pub const SILIQUA: Mass<Exact> = KILOGRAM * (Exact::dec(3289, -4) / Exact::int(1728));

    /// One `SecondSidereal` (`s (sidereal)`), as a `Time`.
    pub const SECOND_SIDEREAL: Time<Exact> = SECOND * Exact::dec(9972696, -7);

    /// One `Day` (`d`), as a `Time`.
    pub const DAY: Time<Exact> = SECOND * Exact::int(86400);

    /// One `DaySidereal` (`d (sidereal)`), as a `Time`.
    pub const DAY_SIDEREAL: Time<Exact> = SECOND * Exact::dec(8616409, -2);

    /// One `Hour` (`h`), as a `Time`.
    pub const HOUR: Time<Exact> = SECOND * Exact::int(3600);

    /// One `HourSidereal` (`h (sidereal)`), as a `Time`.
    pub const HOUR_SIDEREAL: Time<Exact> = SECOND * Exact::dec(359017, -2);

    /// One `Minute` (`min`), as a `Time`.
    pub const MINUTE: Time<Exact> = SECOND * Exact::int(60);

    /// One `Shake` (`10.0 ns`), as a `Time`.
    pub const SHAKE: Time<Exact> = SECOND * Exact::dec(1, -8);

    /// One `Year` (`a`), as a `Time`.
    pub const YEAR: Time<Exact> = SECOND * Exact::int(31536000);

    /// One `YearSidereal` (`a (sidereal)`), as a `Time`.
    pub const YEAR_SIDEREAL: Time<Exact> = SECOND * Exact::int(31558150);

    /// One `YearTropical` (`a (tropical)`), as a `Time`.
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

    /// One `SquareMetre` (`m²`), as an `Area`.
    pub const SQUARE_METRE: Area<Exact> = METRE * METRE;

    /// One `Acre` (`ac`), as an `Area`.
    pub const ACRE: Area<Exact> = SQUARE_METRE
        * (Exact::int(43560)
            * (Exact::int(1200) / Exact::int(3937))
            * (Exact::int(1200) / Exact::int(3937)));

    /// One `Are` (`a`), as an `Area`.
    pub const ARE: Area<Exact> = SQUARE_METRE * Exact::int(100);

    /// One `Barn` (`b`), as an `Area`.
    pub const BARN: Area<Exact> = SQUARE_METRE * Exact::dec(1, -28);

    /// One `CircularMil` (`cmil`), as an `Area`.
    pub const CIRCULAR_MIL: Area<Exact> =
        SQUARE_METRE * (Exact::PI / Exact::int(4) * Exact::dec(254, -7) * Exact::dec(254, -7));

    /// One `Hectare` (`ha`), as an `Area`.
    pub const HECTARE: Area<Exact> = SQUARE_METRE * Exact::int(10000);

    /// One `SquareFoot` (`ft²`), as an `Area`.
    pub const SQUARE_FOOT: Area<Exact> = SQUARE_METRE * Exact::dec(9290304, -8);

    /// One `SquareInch` (`in²`), as an `Area`.
    pub const SQUARE_INCH: Area<Exact> = SQUARE_METRE * Exact::dec(64516, -8);

    /// One `SquareMile` (`mi²`), as an `Area`.
    pub const SQUARE_MILE: Area<Exact> = SQUARE_METRE * Exact::dec(2589988110336, -6);

    /// One `SquareYard` (`yd²`), as an `Area`.
    pub const SQUARE_YARD: Area<Exact> = SQUARE_METRE * Exact::dec(83612736, -8);

    /// One `Saltus` (`saltus`), as an `Area`.
    pub const SALTUS: Area<Exact> = SQUARE_METRE * Exact::dec(201867264, -2);

    /// One `Centuria` (`centuria`), as an `Area`.
    pub const CENTURIA: Area<Exact> = SQUARE_METRE * Exact::dec(50466816, -2);

    /// One `Heredium` (`heredium`), as an `Area`.
    pub const HEREDIUM: Area<Exact> = SQUARE_METRE * Exact::dec(50466816, -4);

    /// One `Jugerum` (`jugerum`), as an `Area`.
    pub const JUGERUM: Area<Exact> = SQUARE_METRE * Exact::dec(25233408, -4);

    /// One `Deunx` (`deunx`), as an `Area`.
    pub const AREA_DEUNX: Area<Exact> = SQUARE_METRE * Exact::dec(23130624, -4);

    /// One `Dextans` (`dextans`), as an `Area`.
    pub const AREA_DEXTANS: Area<Exact> = SQUARE_METRE * Exact::dec(2102784, -3);

    /// One `Dodrans` (`dodrans`), as an `Area`.
    pub const AREA_DODRANS: Area<Exact> = SQUARE_METRE * Exact::dec(18925056, -4);

    /// One `Bes` (`bes`), as an `Area`.
    pub const AREA_BES: Area<Exact> = SQUARE_METRE * Exact::dec(16822272, -4);

    /// One `Septunx` (`septunx`), as an `Area`.
    pub const AREA_SEPTUNX: Area<Exact> = SQUARE_METRE * Exact::dec(14719488, -4);

    /// One `Semis` (`semis`), as an `Area`.
    pub const AREA_SEMIS: Area<Exact> = SQUARE_METRE * Exact::dec(12616704, -4);

    /// One `ActusQuadratus` (`actus quadratus`), as an `Area`.
    pub const ACTUS_QUADRATUS: Area<Exact> = SQUARE_METRE * Exact::dec(12616704, -4);

    /// One `Quincunx` (`quincunx`), as an `Area`.
    pub const AREA_QUINCUNX: Area<Exact> = SQUARE_METRE * Exact::dec(1051392, -3);

    /// One `Triens` (`triens`), as an `Area`.
    pub const AREA_TRIENS: Area<Exact> = SQUARE_METRE * Exact::dec(8411136, -4);

    /// One `Quadrans` (`quadrans`), as an `Area`.
    pub const QUADRANS: Area<Exact> = SQUARE_METRE * Exact::dec(6308352, -4);

    /// One `Sextans` (`sextans`), as an `Area`.
    pub const AREA_SEXTANS: Area<Exact> = SQUARE_METRE * Exact::dec(4205568, -4);

    /// One `Clima` (`clima`), as an `Area`.
    pub const CLIMA: Area<Exact> = SQUARE_METRE * Exact::dec(3154176, -4);

    /// One `Uncia` (`uncia`), as an `Area`.
    pub const AREA_UNCIA: Area<Exact> = SQUARE_METRE * Exact::dec(2102784, -4);

    /// One `Semiuncia` (`semiuncia`), as an `Area`.
    pub const SEMIUNCIA: Area<Exact> = SQUARE_METRE * Exact::dec(1051392, -4);

    /// One `Sicilicus` (`sicilicus`), as an `Area`.
    pub const AREA_SICILICUS: Area<Exact> = SQUARE_METRE * Exact::dec(525696, -4);

    /// One `ActusSimplex` (`actus simplex`), as an `Area`.
    pub const ACTUS_SIMPLEX: Area<Exact> = SQUARE_METRE * Exact::dec(4205568, -5);

    /// One `Sextula` (`sextula`), as an `Area`.
    pub const AREA_SEXTULA: Area<Exact> = SQUARE_METRE * Exact::dec(350464, -4);

    /// One `DuoScrupulum` (`duo scrupulum`), as an `Area`.
    pub const DUO_SCRUPULUM: Area<Exact> = SQUARE_METRE * Exact::dec(175232, -4);

    /// One `Scrupulum` (`scrupulum`), as an `Area`.
    pub const AREA_SCRUPULUM: Area<Exact> = SQUARE_METRE * Exact::dec(87616, -4);

    /// One `DimidiumScrupulum` (`dimidium scrupulum`), as an `Area`.
    pub const DIMIDIUM_SCRUPULUM: Area<Exact> = SQUARE_METRE * Exact::dec(43808, -4);

    /// One `PesQuadratus` (`pes quadratus`), as an `Area`.
    pub const PES_QUADRATUS: Area<Exact> = SQUARE_METRE * Exact::dec(87616, -6);

    /// One `CubicMetre` (`m³`), as a `Volume`.
    pub const CUBIC_METRE: Volume<Exact> = METRE * METRE * METRE;

    /// One `Barrel` (`bbl`), as a `Volume`.
    pub const BARREL: Volume<Exact> = CUBIC_METRE * Exact::dec(158987294928, -12);

    /// One `Bushel` (`bu`), as a `Volume`.
    pub const BUSHEL: Volume<Exact> = CUBIC_METRE * Exact::dec(3523907016688, -14);

    /// One `Cord` (`cords`), as a `Volume`.
    pub const CORD: Volume<Exact> = CUBIC_METRE
        * (Exact::int(128) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4));

    /// One `CubicFoot` (`ft³`), as a `Volume`.
    pub const CUBIC_FOOT: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4));

    /// One `CubicInch` (`in³`), as a `Volume`.
    pub const CUBIC_INCH: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4));

    /// One `CubicMile` (`mi³`), as a `Volume`.
    pub const CUBIC_MILE: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3));

    /// One `CubicYard` (`yd³`), as a `Volume`.
    pub const CUBIC_YARD: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4));

    /// One `Cup` (`cup`), as a `Volume`.
    pub const CUP: Volume<Exact> = CUBIC_METRE * Exact::dec(2365882365, -13);

    /// One `FluidOunce` (`fl oz`), as a `Volume`.
    pub const FLUID_OUNCE: Volume<Exact> = CUBIC_METRE * Exact::dec(295735295625, -16);

    /// One `FluidOunceImperial` (`fl oz (UK)`), as a `Volume`.
    pub const FLUID_OUNCE_IMPERIAL: Volume<Exact> = CUBIC_METRE * Exact::dec(284130625, -13);

    /// One `GallonImperial` (`gal (UK)`), as a `Volume`.
    pub const GALLON_IMPERIAL: Volume<Exact> = CUBIC_METRE * Exact::dec(454609, -8);

    /// One `Gallon` (`gal`), as a `Volume`.
    pub const GALLON: Volume<Exact> = CUBIC_METRE * Exact::dec(3785411784, -12);

    /// One `GillImperial` (`gi (UK)`), as a `Volume`.
    pub const GILL_IMPERIAL: Volume<Exact> = CUBIC_METRE * Exact::dec(1420653125, -13);

    /// One `Gill` (`gi`), as a `Volume`.
    pub const GILL: Volume<Exact> = CUBIC_METRE * Exact::dec(11829411825, -14);

    /// One `Litre` (`L`), as a `Volume`.
    pub const LITRE: Volume<Exact> = CUBIC_METRE * Exact::dec(1, -3);

    /// One `Peck` (`pk`), as a `Volume`.
    pub const PECK: Volume<Exact> = CUBIC_METRE * Exact::dec(880976754172, -14);

    /// One `PintDry` (`dry pt`), as a `Volume`.
    pub const PINT_DRY: Volume<Exact> = CUBIC_METRE * Exact::dec(5506104713575, -16);

    /// One `PintLiquid` (`liq pt`), as a `Volume`.
    pub const PINT_LIQUID: Volume<Exact> = CUBIC_METRE * Exact::dec(473176473, -12);

    /// One `QuartDry` (`dry qt`), as a `Volume`.
    pub const QUART_DRY: Volume<Exact> = CUBIC_METRE * Exact::dec(1101220942715, -15);

    /// One `QuartLiquid` (`liq qt`), as a `Volume`.
    pub const QUART_LIQUID: Volume<Exact> = CUBIC_METRE * Exact::dec(946352946, -12);

    /// One `Stere` (`st`), as a `Volume`.
    pub const STERE: Volume<Exact> = CUBIC_METRE;

    /// One `Tablespoon` (`tbsp`), as a `Volume`.
    pub const TABLESPOON: Volume<Exact> = CUBIC_METRE * Exact::dec(1478676478125, -17);

    /// One `Teaspoon` (`tsp`), as a `Volume`.
    pub const TEASPOON: Volume<Exact> = CUBIC_METRE * Exact::dec(492892159375, -17);

    /// One `RegisterTon` (`RT`), as a `Volume`.
    pub const REGISTER_TON: Volume<Exact> = CUBIC_METRE
        * (Exact::int(100) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4));

    /// One `Culeus` (`culeus`), as a `Volume`.
    pub const CULEUS: Volume<Exact> = CUBIC_METRE * Exact::dec(51868672, -8);

    /// One `AmphoraQuadrantal` (`amphora quadrantal`), as a `Volume`.
    pub const AMPHORA_QUADRANTAL: Volume<Exact> = CUBIC_METRE * Exact::dec(25934336, -9);

    /// One `Urna` (`urna`), as a `Volume`.
    pub const URNA: Volume<Exact> = CUBIC_METRE * Exact::dec(12967168, -9);

    /// One `ModiusCastrensis` (`modius castrensis`), as a `Volume`.
    pub const MODIUS_CASTRENSIS: Volume<Exact> = CUBIC_METRE * Exact::dec(12967168, -9);

    /// One `Modius` (`modius`), as a `Volume`.
    pub const MODIUS: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(3241792, -9) * Exact::int(16) / Exact::int(6));

    /// One `Semimodius` (`semimodius`), as a `Volume`.
    pub const SEMIMODIUS: Volume<Exact> =
        CUBIC_METRE * (Exact::dec(3241792, -9) * Exact::int(8) / Exact::int(6));

    /// One `Congius` (`congius`), as a `Volume`.
    pub const CONGIUS: Volume<Exact> = CUBIC_METRE * Exact::dec(3241792, -9);

    /// One `Sextarius` (`sextarius`), as a `Volume`.
    pub const SEXTARIUS: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(6));

    /// One `Hemina` (`hemina`), as a `Volume`.
    pub const HEMINA: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(12));

    /// One `Quartarius` (`quartarius`), as a `Volume`.
    pub const QUARTARIUS: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(24));

    /// One `Acetabulum` (`acetabulum`), as a `Volume`.
    pub const ACETABULUM: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(48));

    /// One `Cyathus` (`cyathus`), as a `Volume`.
    pub const CYATHUS: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(72));

    /// One `Ligula` (`ligula`), as a `Volume`.
    pub const LIGULA: Volume<Exact> = CUBIC_METRE * (Exact::dec(3241792, -9) / Exact::int(288));

    /// One `MetrePerSecond` (`m/s`), as a `Velocity`.
    pub const METRE_PER_SECOND: Velocity<Exact> = METRE / SECOND;

    /// One `Knot` (`kn`), as a `Velocity`.
    pub const KNOT: Velocity<Exact> = METRE_PER_SECOND * (Exact::int(1852) / Exact::int(3600));

    /// One `MilePerHour` (`mi/h`), as a `Velocity`.
    pub const MILE_PER_HOUR: Velocity<Exact> = METRE_PER_SECOND * Exact::dec(44704, -5);

    /// One `NaturalUnitOfVelocity` (`c`), as a `Velocity`.
    pub const NATURAL_UNIT_OF_VELOCITY: Velocity<Exact> = METRE_PER_SECOND * Exact::int(299792458);

    /// One `SpeedOfLight` (`c`), as a `Velocity`.
    pub const SPEED_OF_LIGHT: Velocity<Exact> = METRE_PER_SECOND * Exact::int(299792458);

    /// One `MetrePerSecondSquared` (`m/s²`), as an `Acceleration`.
    pub const METRE_PER_SECOND_SQUARED: Acceleration<Exact> = METRE / (SECOND * SECOND);

    /// One `Galileo` (`Gal`), as an `Acceleration`.
    pub const GALILEO: Acceleration<Exact> = METRE_PER_SECOND_SQUARED * Exact::dec(1, -2);

    /// One `StandardGravity` (`g₀`), as an `Acceleration`.
    pub const STANDARD_GRAVITY: Acceleration<Exact> =
        METRE_PER_SECOND_SQUARED * Exact::dec(980665, -5);

    /// One `Newton` (`N`), as a `Force`.
    pub const NEWTON: Force<Exact> = METRE * KILOGRAM / (SECOND * SECOND);

    /// One `Dyne` (`dyn`), as a `Force`.
    pub const DYNE: Force<Exact> = NEWTON * Exact::dec(1, -5);

    /// One `KilogramForce` (`kgf`), as a `Force`.
    pub const KILOGRAM_FORCE: Force<Exact> = NEWTON * Exact::dec(980665, -5);

    /// One `Kip` (`kip`), as a `Force`.
    pub const KIP: Force<Exact> =
        NEWTON * (Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5));

    /// One `OunceForce` (`ozf`), as a `Force`.
    pub const OUNCE_FORCE: Force<Exact> =
        NEWTON * (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16));

    /// One `Poundal` (`pdl`), as a `Force`.
    pub const POUNDAL: Force<Exact> = NEWTON * (Exact::dec(45359237, -8) * Exact::dec(3048, -4));

    /// One `PoundForce` (`lbf`), as a `Force`.
    pub const POUND_FORCE: Force<Exact> =
        NEWTON * (Exact::dec(45359237, -8) * Exact::dec(980665, -5));

    /// One `TonForce` (`2000 lbf`), as a `Force`.
    pub const TON_FORCE: Force<Exact> =
        NEWTON * (Exact::int(2000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5));

    /// One `Joule` (`J`), as an `Energy`.
    pub const JOULE: Energy<Exact> = METRE * METRE * KILOGRAM / (SECOND * SECOND);

    /// One `Electronvolt` (`eV`), as an `Energy`.
    pub const ELECTRONVOLT: Energy<Exact> = JOULE * Exact::dec(1602176634, -28);

    /// One `Hartree` (`Eₕ`), as an `Energy`.
    pub const HARTREE: Energy<Exact> = JOULE * Exact::dec(43597447222071, -31);

    /// One `BtuIt` (`Btu (IT)`), as an `Energy`.
    pub const BTU_IT: Energy<Exact> = JOULE * Exact::dec(105505585262, -8);

    /// One `Btu` (`Btu`), as an `Energy`.
    pub const BTU: Energy<Exact> =
        JOULE * (Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1));

    /// One `CalorieIt` (`cal (IT)`), as an `Energy`.
    pub const CALORIE_IT: Energy<Exact> = JOULE * Exact::dec(41868, -4);

    /// One `Calorie` (`cal`), as an `Energy`.
    pub const CALORIE: Energy<Exact> = JOULE * Exact::dec(4184, -3);

    /// One `CalorieItNutrition` (`Cal (IT)`), as an `Energy`.
    pub const CALORIE_IT_NUTRITION: Energy<Exact> = JOULE * Exact::dec(41868, -1);

    /// One `CalorieNutrition` (`Cal`), as an `Energy`.
    pub const CALORIE_NUTRITION: Energy<Exact> = JOULE * Exact::int(4184);

    /// One `Erg` (`erg`), as an `Energy`.
    pub const ERG: Energy<Exact> = JOULE * Exact::dec(1, -7);

    /// One `Quad` (`10¹⁵ Btu (IT)`), as an `Energy`.
    pub const QUAD: Energy<Exact> = JOULE * Exact::int(1055055852620000000);

    /// One `ThermEc` (`thm (EC)`), as an `Energy`.
    pub const THERM_EC: Energy<Exact> = JOULE * Exact::int(105506000);

    /// One `ThermUs` (`thm`), as an `Energy`.
    pub const THERM_US: Energy<Exact> = JOULE * Exact::int(105480400);

    /// One `TonTnt` (`t of TNT`), as an `Energy`.
    pub const TON_TNT: Energy<Exact> = JOULE * Exact::int(4184000000);

    /// One `Watt` (`W`), as a `Power`.
    pub const WATT: Power<Exact> = METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND);

    /// One `Horsepower` (`hp`), as a `Power`.
    pub const HORSEPOWER: Power<Exact> = WATT
        * (Exact::int(550)
            * Exact::dec(3048, -4)
            * Exact::dec(45359237, -8)
            * Exact::dec(980665, -5));

    /// One `HorsepowerBoiler` (`hp (S)`), as a `Power`.
    pub const HORSEPOWER_BOILER: Power<Exact> = WATT * Exact::dec(98095, -1);

    /// One `HorsepowerElectric` (`hp (E)`), as a `Power`.
    pub const HORSEPOWER_ELECTRIC: Power<Exact> = WATT * Exact::int(746);

    /// One `HorsepowerMetric` (`hp (M)`), as a `Power`.
    pub const HORSEPOWER_METRIC: Power<Exact> = WATT * (Exact::int(75) * Exact::dec(980665, -5));

    /// One `HorsepowerImperial` (`hp (I)`), as a `Power`.
    pub const HORSEPOWER_IMPERIAL: Power<Exact> = WATT * Exact::dec(7457, -1);

    /// One `HydraulicHorsepower` (`hp (hydraulic)`), as a `Power`.
    pub const HYDRAULIC_HORSEPOWER: Power<Exact> = WATT * Exact::dec(746043, -3);

    /// One `Decibel` (`dB`), as a `Gain`.
    pub const DECIBEL: Gain<Exact> = BEL * Exact::dec(1, -1);

    /// One `Neper` (`Np`), as a `Gain`.
    pub const NEPER: Gain<Exact> = BEL * Exact::dec(8685889638065036, -16);

    /// The level of one milliwatt, 0 dBm: 30 dB below a watt.
    pub const MILLIWATT_LEVEL: PowerLevel<Exact> = WATT_LEVEL - DECIBEL * Exact::int(30);

    /// Decibels-watt (dBW): decibels from one watt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBW: Scale<PowerLevel<Exact>, Gain<Exact>> = Scale(WATT_LEVEL, DECIBEL);

    /// Decibels-milliwatt (dBm): decibels from one milliwatt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBM: Scale<PowerLevel<Exact>, Gain<Exact>> = Scale(MILLIWATT_LEVEL, DECIBEL);

    /// One `AtomicUnitOfAction` (`ħ`), as an `Action`.
    pub const ATOMIC_UNIT_OF_ACTION: Action<Exact> = (METRE * METRE * KILOGRAM / SECOND)
        * (Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI));

    /// One `PlanckConstant` (`h`), as an `Action`.
    pub const PLANCK_CONSTANT: Action<Exact> =
        (METRE * METRE * KILOGRAM / SECOND) * Exact::dec(662607015, -42);

    /// One `Pascal` (`Pa`), as a `Pressure`.
    pub const PASCAL: Pressure<Exact> = KILOGRAM / (METRE * SECOND * SECOND);

    /// One `StandardAtmosphere` (`atm`), as a `Pressure`.
    pub const STANDARD_ATMOSPHERE: Pressure<Exact> = PASCAL * Exact::int(101325);

    /// One `AtmosphereTechnical` (`at`), as a `Pressure`.
    pub const ATMOSPHERE_TECHNICAL: Pressure<Exact> = PASCAL * Exact::dec(980665, -1);

    /// One `Bar` (`bar`), as a `Pressure`.
    pub const BAR: Pressure<Exact> = PASCAL * Exact::int(100000);

    /// One `CentimetreOfMercury` (`cm Hg`), as a `Pressure`.
    pub const CENTIMETRE_OF_MERCURY: Pressure<Exact> =
        PASCAL * (Exact::dec(133322387415, -9) * Exact::int(10));

    /// One `CentimetreOfWater` (`cm H₂O`), as a `Pressure`.
    pub const CENTIMETRE_OF_WATER: Pressure<Exact> = PASCAL * Exact::dec(980665, -4);

    /// One `FootOfMercury` (`ft Hg`), as a `Pressure`.
    pub const FOOT_OF_MERCURY: Pressure<Exact> =
        PASCAL * (Exact::dec(133322387415, -9) * Exact::dec(3048, -1));

    /// One `FootOfWater` (`ft H₂O`), as a `Pressure`.
    pub const FOOT_OF_WATER: Pressure<Exact> =
        PASCAL * (Exact::dec(980665, -5) * Exact::dec(3048, -1));

    /// One `InchOfMercury` (`in Hg`), as a `Pressure`.
    pub const INCH_OF_MERCURY: Pressure<Exact> =
        PASCAL * (Exact::dec(133322387415, -9) * Exact::dec(254, -1));

    /// One `InchOfWater` (`in H₂O`), as a `Pressure`.
    pub const INCH_OF_WATER: Pressure<Exact> =
        PASCAL * (Exact::dec(980665, -5) * Exact::dec(254, -1));

    /// One `MillimetreOfMercury` (`mm Hg`), as a `Pressure`.
    pub const MILLIMETRE_OF_MERCURY: Pressure<Exact> = PASCAL * Exact::dec(133322387415, -9);

    /// One `MillimetreOfWater` (`mm H₂O`), as a `Pressure`.
    pub const MILLIMETRE_OF_WATER: Pressure<Exact> = PASCAL * Exact::dec(980665, -5);

    /// One `Psi` (`psi`), as a `Pressure`.
    pub const PSI: Pressure<Exact> = PASCAL
        * (Exact::dec(45359237, -8) * Exact::dec(980665, -5)
            / (Exact::dec(254, -4) * Exact::dec(254, -4)));

    /// One `Torr` (`Torr`), as a `Pressure`.
    pub const TORR: Pressure<Exact> = PASCAL * (Exact::int(101325) / Exact::int(760));

    /// One `Hertz` (`Hz`), as a `Frequency`.
    pub const HERTZ: Frequency<Exact> = ONE / SECOND;

    /// One `CyclePerShake` (`100 MHz`), as a `Frequency`.
    pub const CYCLE_PER_SHAKE: Frequency<Exact> = HERTZ * Exact::int(100000000);

    /// One `ReciprocalMetre` (`m⁻¹`), as a `ReciprocalLength`.
    pub const RECIPROCAL_METRE: ReciprocalLength<Exact> = ONE / METRE;

    /// One `ReciprocalAngstrom` (`Å⁻¹`), as a `ReciprocalLength`.
    pub const RECIPROCAL_ANGSTROM: ReciprocalLength<Exact> =
        RECIPROCAL_METRE * Exact::int(10000000000);

    /// One `Diopter` (`dpt`), as a `ReciprocalLength`.
    pub const DIOPTER: ReciprocalLength<Exact> = RECIPROCAL_METRE;

    /// One `RydbergConstant` (`R∞`), as a `ReciprocalLength`.
    pub const RYDBERG_CONSTANT: ReciprocalLength<Exact> =
        RECIPROCAL_METRE * Exact::dec(1097373156816, -5);

    /// One `GravitationalConstant` (`G`), as a `SpecificGravitationalParameter`.
    pub const GRAVITATIONAL_CONSTANT: SpecificGravitationalParameter<Exact> =
        (METRE * METRE * METRE / (KILOGRAM * SECOND * SECOND)) * Exact::dec(66743, -15);
}
