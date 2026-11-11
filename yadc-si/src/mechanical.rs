use crate::{Exact, ISQ, Kind, SIQuantity, Scale, counted};
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
/// (re-exported), then one per multiple, each defined exactly from them -
/// `INCH` is `0.0254 * METRE` - and counted in the number type its use asks
/// for, rounded once: `let d: Length<f64> = INCH;` is an `f64` length,
/// `Length<f32>` an `f32` one, `INCH::<Exact>` the exact value. Each is a
/// unit (`INCH(3.0)`, `INCH.value(d)`). A prefixed unit is a prefix times
/// one: `KILO * METRE`. They compose in `const` items:
/// `const LIMIT: Velocity<f64> = 50.0 * (KILO * METRE) / HOUR;`.
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

    /// One `Angstrom` (Å), as a `Length`.
    pub const ANGSTROM<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -10) * METRE::<Exact>);

    /// One `BohrRadius` (a₀), as a `Length`.
    pub const BOHR_RADIUS<N: const From<Exact>>: Length<N> = counted(Exact::dec(529177210903, -22) * METRE::<Exact>);

    /// One `AtomicUnitOfLength` (a.u. of length), as a `Length`.
    pub const ATOMIC_UNIT_OF_LENGTH<N: const From<Exact>>: Length<N> = counted(Exact::dec(529177210903, -22) * METRE::<Exact>);

    /// One `AstronomicalUnit` (ua), as a `Length`.
    pub const ASTRONOMICAL_UNIT<N: const From<Exact>>: Length<N> = counted(Exact::int(149597870700) * METRE::<Exact>);

    /// One `Chain` (ch), as a `Length`.
    pub const CHAIN<N: const From<Exact>>: Length<N> = counted((Exact::int(66) * Exact::int(1200) / Exact::int(3937)) * METRE::<Exact>);

    /// One `DataMile` (DM), as a `Length`.
    pub const DATA_MILE<N: const From<Exact>>: Length<N> = counted(Exact::dec(18288, -1) * METRE::<Exact>);

    /// One `Fathom` (fathom), as a `Length`.
    pub const FATHOM<N: const From<Exact>>: Length<N> = counted((Exact::int(6) * Exact::int(1200) / Exact::int(3937)) * METRE::<Exact>);

    /// One `Fermi` (fermi), as a `Length`.
    pub const FERMI<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -15) * METRE::<Exact>);

    /// One `Foot` (ft), as a `Length`.
    pub const FOOT<N: const From<Exact>>: Length<N> = counted(Exact::dec(3048, -4) * METRE::<Exact>);

    /// One `FootSurvey` (ft (U.S. survey)), as a `Length`.
    pub const FOOT_SURVEY<N: const From<Exact>>: Length<N> = counted((Exact::int(1200) / Exact::int(3937)) * METRE::<Exact>);

    /// One `Inch` (in), as a `Length`.
    pub const INCH<N: const From<Exact>>: Length<N> = counted(Exact::dec(254, -4) * METRE::<Exact>);

    /// One `LightYear` (l. y.), as a `Length`.
    pub const LIGHT_YEAR<N: const From<Exact>>: Length<N> = counted(Exact::int(9460730472580800) * METRE::<Exact>);

    /// One `Micron` (μ), as a `Length`.
    pub const MICRON<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -6) * METRE::<Exact>);

    /// One `Mil` (0.001 in), as a `Length`.
    pub const MIL<N: const From<Exact>>: Length<N> = counted(Exact::dec(254, -7) * METRE::<Exact>);

    /// One `Mile` (mi), as a `Length`.
    pub const MILE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1609344, -3) * METRE::<Exact>);

    /// One `MileSurvey` (mi (U.S. survey)), as a `Length`.
    pub const MILE_SURVEY<N: const From<Exact>>: Length<N> = counted((Exact::int(6336000) / Exact::int(3937)) * METRE::<Exact>);

    /// One `NauticalMile` (M), as a `Length`.
    pub const NAUTICAL_MILE<N: const From<Exact>>: Length<N> = counted(Exact::int(1852) * METRE::<Exact>);

    /// One `Parsec` (pc), as a `Length`.
    pub const PARSEC<N: const From<Exact>>: Length<N> = counted((Exact::int(149597870700) * Exact::int(648000) / Exact::PI) * METRE::<Exact>);

    /// One `PicaComputer` (1/6 in (computer)), as a `Length`.
    pub const PICA_COMPUTER<N: const From<Exact>>: Length<N> = counted((Exact::dec(254, -4) / Exact::int(6)) * METRE::<Exact>);

    /// One `PicaPrinters` (1/6 in), as a `Length`.
    pub const PICA_PRINTERS<N: const From<Exact>>: Length<N> = counted(Exact::dec(4217518, -9) * METRE::<Exact>);

    /// One `PointComputer` (1/72 in (computer)), as a `Length`.
    pub const POINT_COMPUTER<N: const From<Exact>>: Length<N> = counted((Exact::dec(254, -4) / Exact::int(72)) * METRE::<Exact>);

    /// One `PointPrinters` (1/72 in), as a `Length`.
    pub const POINT_PRINTERS<N: const From<Exact>>: Length<N> = counted(Exact::dec(3514598, -10) * METRE::<Exact>);

    /// One `Rod` (rd), as a `Length`.
    pub const ROD<N: const From<Exact>>: Length<N> = counted((Exact::dec(165, -1) * Exact::int(1200) / Exact::int(3937)) * METRE::<Exact>);

    /// One `Yard` (yd), as a `Length`.
    pub const YARD<N: const From<Exact>>: Length<N> = counted(Exact::dec(9144, -4) * METRE::<Exact>);

    /// One `Leuga` (leuga), as a `Length`.
    pub const LEUGA<N: const From<Exact>>: Length<N> = counted(Exact::int(2220) * METRE::<Exact>);

    /// One `MillePassus` (mille passus), as a `Length`.
    pub const MILLE_PASSUS<N: const From<Exact>>: Length<N> = counted(Exact::int(1480) * METRE::<Exact>);

    /// One `Stadium` (stadium), as a `Length`.
    pub const STADIUM<N: const From<Exact>>: Length<N> = counted(Exact::int(185) * METRE::<Exact>);

    /// One `Actus` (actus), as a `Length`.
    pub const ACTUS<N: const From<Exact>>: Length<N> = counted(Exact::dec(3552, -2) * METRE::<Exact>);

    /// One `Decempeda` (decempeda), as a `Length`.
    pub const DECEMPEDA<N: const From<Exact>>: Length<N> = counted(Exact::dec(296, -2) * METRE::<Exact>);

    /// One `Passus` (passus), as a `Length`.
    pub const PASSUS<N: const From<Exact>>: Length<N> = counted(Exact::dec(148, -2) * METRE::<Exact>);

    /// One `Gradus` (gradus), as a `Length`.
    pub const GRADUS<N: const From<Exact>>: Length<N> = counted(Exact::dec(74, -2) * METRE::<Exact>);

    /// One `Cubitum` (cubitum), as a `Length`.
    pub const CUBITUM<N: const From<Exact>>: Length<N> = counted(Exact::dec(444, -3) * METRE::<Exact>);

    /// One `Palmipes` (palmipes), as a `Length`.
    pub const PALMIPES<N: const From<Exact>>: Length<N> = counted(Exact::dec(37, -2) * METRE::<Exact>);

    /// One `Pes` (pes), as a `Length`.
    pub const PES<N: const From<Exact>>: Length<N> = counted(Exact::dec(296, -3) * METRE::<Exact>);

    /// One `PalmusMaior` (palmus maior), as a `Length`.
    pub const PALMUS_MAIOR<N: const From<Exact>>: Length<N> = counted(Exact::dec(222, -3) * METRE::<Exact>);

    /// One `Palmus` (palmus), as a `Length`.
    pub const PALMUS<N: const From<Exact>>: Length<N> = counted(Exact::dec(74, -3) * METRE::<Exact>);

    /// One `Uncia` (uncia), as a `Length`.
    pub const LENGTH_UNCIA<N: const From<Exact>>: Length<N> = counted((Exact::dec(296, -3) / Exact::int(12)) * METRE::<Exact>);

    /// One `Digitus` (digitus), as a `Length`.
    pub const DIGITUS<N: const From<Exact>>: Length<N> = counted(Exact::dec(185, -4) * METRE::<Exact>);

    /// One `Gram` (g), as a `Mass`.
    pub const GRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -3) * KILOGRAM::<Exact>);

    /// One `Carat` (ct), as a `Mass`.
    pub const CARAT<N: const From<Exact>>: Mass<N> = counted(Exact::dec(2, -4) * KILOGRAM::<Exact>);

    /// One `Dalton` (Da), as a `Mass`.
    pub const DALTON<N: const From<Exact>>: Mass<N> = counted(Exact::dec(16605390666, -37) * KILOGRAM::<Exact>);

    /// One `ElectronMass` (mₑ), as a `Mass`.
    pub const ELECTRON_MASS<N: const From<Exact>>: Mass<N> = counted(Exact::dec(91093837015, -41) * KILOGRAM::<Exact>);

    /// One `ProtonMass` (mₚ), as a `Mass`.
    pub const PROTON_MASS<N: const From<Exact>>: Mass<N> = counted(Exact::dec(167262192369, -38) * KILOGRAM::<Exact>);

    /// One `Grain` (gr), as a `Mass`.
    pub const GRAIN<N: const From<Exact>>: Mass<N> = counted(Exact::dec(6479891, -11) * KILOGRAM::<Exact>);

    /// One `HundredweightLong` (cwt long), as a `Mass`.
    pub const HUNDREDWEIGHT_LONG<N: const From<Exact>>: Mass<N> = counted(Exact::dec(5080234544, -8) * KILOGRAM::<Exact>);

    /// One `HundredweightShort` (cwt short), as a `Mass`.
    pub const HUNDREDWEIGHT_SHORT<N: const From<Exact>>: Mass<N> = counted(Exact::dec(45359237, -6) * KILOGRAM::<Exact>);

    /// One `Ounce` (oz), as a `Mass`.
    pub const OUNCE<N: const From<Exact>>: Mass<N> = counted(Exact::dec(28349523125, -12) * KILOGRAM::<Exact>);

    /// One `OunceTroy` (oz t), as a `Mass`.
    pub const OUNCE_TROY<N: const From<Exact>>: Mass<N> = counted(Exact::dec(311034768, -10) * KILOGRAM::<Exact>);

    /// One `Pennyweight` (dwt), as a `Mass`.
    pub const PENNYWEIGHT<N: const From<Exact>>: Mass<N> = counted(Exact::dec(155517384, -11) * KILOGRAM::<Exact>);

    /// One `Pound` (lb), as a `Mass`.
    pub const POUND<N: const From<Exact>>: Mass<N> = counted(Exact::dec(45359237, -8) * KILOGRAM::<Exact>);

    /// One `PoundTroy` (lb t), as a `Mass`.
    pub const POUND_TROY<N: const From<Exact>>: Mass<N> = counted(Exact::dec(3732417216, -10) * KILOGRAM::<Exact>);

    /// One `Slug` (slug), as a `Mass`.
    pub const SLUG<N: const From<Exact>>: Mass<N> = counted((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * KILOGRAM::<Exact>);

    /// One `TonAssay` (AT), as a `Mass`.
    pub const TON_ASSAY<N: const From<Exact>>: Mass<N> = counted((Exact::dec(175, -3) / Exact::int(6)) * KILOGRAM::<Exact>);

    /// One `TonLong` (2240 lb), as a `Mass`.
    pub const TON_LONG<N: const From<Exact>>: Mass<N> = counted(Exact::dec(10160469088, -7) * KILOGRAM::<Exact>);

    /// One `TonShort` (2000 lb), as a `Mass`.
    pub const TON_SHORT<N: const From<Exact>>: Mass<N> = counted(Exact::dec(90718474, -5) * KILOGRAM::<Exact>);

    /// One `Ton` (t), as a `Mass`.
    pub const TON<N: const From<Exact>>: Mass<N> = counted(Exact::int(1000) * KILOGRAM::<Exact>);

    /// One `Libra` (libra), as a `Mass`.
    pub const LIBRA<N: const From<Exact>>: Mass<N> = counted(Exact::dec(3289, -4) * KILOGRAM::<Exact>);

    /// One `Deunx` (deunx), as a `Mass`.
    pub const MASS_DEUNX<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) * Exact::int(11) / Exact::int(12)) * KILOGRAM::<Exact>);

    /// One `Dextans` (dextans), as a `Mass`.
    pub const MASS_DEXTANS<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) * Exact::int(10) / Exact::int(12)) * KILOGRAM::<Exact>);

    /// One `Dodrans` (dodrans), as a `Mass`.
    pub const MASS_DODRANS<N: const From<Exact>>: Mass<N> = counted(Exact::dec(246675, -6) * KILOGRAM::<Exact>);

    /// One `Bes` (bes), as a `Mass`.
    pub const MASS_BES<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) * Exact::int(8) / Exact::int(12)) * KILOGRAM::<Exact>);

    /// One `Septunx` (septunx), as a `Mass`.
    pub const MASS_SEPTUNX<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) * Exact::int(7) / Exact::int(12)) * KILOGRAM::<Exact>);

    /// One `Semis` (semis), as a `Mass`.
    pub const MASS_SEMIS<N: const From<Exact>>: Mass<N> = counted(Exact::dec(16445, -5) * KILOGRAM::<Exact>);

    /// One `Quincunx` (quincunx), as a `Mass`.
    pub const MASS_QUINCUNX<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) * Exact::int(5) / Exact::int(12)) * KILOGRAM::<Exact>);

    /// One `Triens` (triens), as a `Mass`.
    pub const MASS_TRIENS<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(3)) * KILOGRAM::<Exact>);

    /// One `QuadransTeruncius` (quadrans teruncius), as a `Mass`.
    pub const QUADRANS_TERUNCIUS<N: const From<Exact>>: Mass<N> = counted(Exact::dec(82225, -6) * KILOGRAM::<Exact>);

    /// One `Sextans` (sextans), as a `Mass`.
    pub const MASS_SEXTANS<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(6)) * KILOGRAM::<Exact>);

    /// One `Sescuncia` (sescuncia), as a `Mass`.
    pub const SESCUNCIA<N: const From<Exact>>: Mass<N> = counted(Exact::dec(411125, -7) * KILOGRAM::<Exact>);

    /// One `Uncia` (uncia), as a `Mass`.
    pub const MASS_UNCIA<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(12)) * KILOGRAM::<Exact>);

    /// One `Semuncia` (semuncia), as a `Mass`.
    pub const SEMUNCIA<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(24)) * KILOGRAM::<Exact>);

    /// One `Duella` (duella), as a `Mass`.
    pub const DUELLA<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(36)) * KILOGRAM::<Exact>);

    /// One `Sicilicus` (sicilicus), as a `Mass`.
    pub const MASS_SICILICUS<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(48)) * KILOGRAM::<Exact>);

    /// One `Sextula` (sextula), as a `Mass`.
    pub const MASS_SEXTULA<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(72)) * KILOGRAM::<Exact>);

    /// One `Semisextula` (semisextula), as a `Mass`.
    pub const SEMISEXTULA<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(144)) * KILOGRAM::<Exact>);

    /// One `Scrupulum` (scrupulum), as a `Mass`.
    pub const MASS_SCRUPULUM<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(288)) * KILOGRAM::<Exact>);

    /// One `Obolus` (obolus), as a `Mass`.
    pub const OBOLUS<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(576)) * KILOGRAM::<Exact>);

    /// One `Siliqua` (siliqua), as a `Mass`.
    pub const SILIQUA<N: const From<Exact>>: Mass<N> = counted((Exact::dec(3289, -4) / Exact::int(1728)) * KILOGRAM::<Exact>);

    /// One `Millisecond` (ms), as a `Time`.
    pub const MILLISECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -3) * SECOND::<Exact>);

    /// One `SecondSidereal` (s (sidereal)), as a `Time`.
    pub const SECOND_SIDEREAL<N: const From<Exact>>: Time<N> = counted(Exact::dec(9972696, -7) * SECOND::<Exact>);

    /// One `Day` (d), as a `Time`.
    pub const DAY<N: const From<Exact>>: Time<N> = counted(Exact::int(86400) * SECOND::<Exact>);

    /// One `DaySidereal` (d (sidereal)), as a `Time`.
    pub const DAY_SIDEREAL<N: const From<Exact>>: Time<N> = counted(Exact::dec(8616409, -2) * SECOND::<Exact>);

    /// One `Hour` (h), as a `Time`.
    pub const HOUR<N: const From<Exact>>: Time<N> = counted(Exact::int(3600) * SECOND::<Exact>);

    /// One `HourSidereal` (h (sidereal)), as a `Time`.
    pub const HOUR_SIDEREAL<N: const From<Exact>>: Time<N> = counted(Exact::dec(359017, -2) * SECOND::<Exact>);

    /// One `Minute` (min), as a `Time`.
    pub const MINUTE<N: const From<Exact>>: Time<N> = counted(Exact::int(60) * SECOND::<Exact>);

    /// One `Shake` (10.0 ns), as a `Time`.
    pub const SHAKE<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -8) * SECOND::<Exact>);

    /// One `Year` (a), as a `Time`.
    pub const YEAR<N: const From<Exact>>: Time<N> = counted(Exact::int(31536000) * SECOND::<Exact>);

    /// One `YearSidereal` (a (sidereal)), as a `Time`.
    pub const YEAR_SIDEREAL<N: const From<Exact>>: Time<N> = counted(Exact::int(31558150) * SECOND::<Exact>);

    /// One `YearTropical` (a (tropical)), as a `Time`.
    pub const YEAR_TROPICAL<N: const From<Exact>>: Time<N> = counted(Exact::int(31556930) * SECOND::<Exact>);

    /// The GPS epoch, 1980-01-06T00:00:00Z: 3657 days after the Unix epoch.
    pub const GPS_EPOCH<N: const From<Exact>>: Instant<N> = counted(UNIX_EPOCH::<Exact> + Exact::int(315964800) * SECOND::<Exact>);

    /// The zero of the Julian date: noon on 1 January 4713 BC (proleptic Julian), 2 440 587.5 days before the Unix epoch.
    pub const JULIAN_EPOCH<N: const From<Exact>>: Instant<N> = counted(UNIX_EPOCH::<Exact> - Exact::dec(24405875, -1) * DAY::<Exact>);

    /// The zero of the modified Julian date, 1858-11-17T00:00:00Z: 40 587 days before the Unix epoch.
    pub const MODIFIED_JULIAN_EPOCH<N: const From<Exact>>: Instant<N> = counted(UNIX_EPOCH::<Exact> - Exact::int(40587) * DAY::<Exact>);

    /// Unix time: seconds from the Unix epoch.
    pub const UNIX_SECONDS<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(UNIX_EPOCH, SECOND);

    /// Unix time in milliseconds.
    pub const UNIX_MILLISECONDS<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(UNIX_EPOCH, MILLISECOND);

    /// GPS time: seconds from the GPS epoch.
    pub const GPS_SECONDS<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(GPS_EPOCH, SECOND);

    /// The Julian date (JD): days from the Julian epoch.
    pub const JULIAN_DATE<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(JULIAN_EPOCH, DAY);

    /// The modified Julian date (MJD): days from the modified Julian epoch.
    pub const MODIFIED_JULIAN_DATE<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(MODIFIED_JULIAN_EPOCH, DAY);

    /// One `SquareMetre` (m²), as an `Area`.
    pub const SQUARE_METRE<N: const From<Exact>>: Area<N> = counted(METRE::<Exact> * METRE::<Exact>);

    /// One `Acre` (ac), as an `Area`.
    pub const ACRE<N: const From<Exact>>: Area<N> = counted((Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937))) * SQUARE_METRE::<Exact>);

    /// One `Are` (a), as an `Area`.
    pub const ARE<N: const From<Exact>>: Area<N> = counted(Exact::int(100) * SQUARE_METRE::<Exact>);

    /// One `Barn` (b), as an `Area`.
    pub const BARN<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -28) * SQUARE_METRE::<Exact>);

    /// One `CircularMil` (cmil), as an `Area`.
    pub const CIRCULAR_MIL<N: const From<Exact>>: Area<N> = counted((Exact::PI / Exact::int(4) * Exact::dec(254, -7) * Exact::dec(254, -7)) * SQUARE_METRE::<Exact>);

    /// One `Hectare` (ha), as an `Area`.
    pub const HECTARE<N: const From<Exact>>: Area<N> = counted(Exact::int(10000) * SQUARE_METRE::<Exact>);

    /// One `SquareFoot` (ft²), as an `Area`.
    pub const SQUARE_FOOT<N: const From<Exact>>: Area<N> = counted(Exact::dec(9290304, -8) * SQUARE_METRE::<Exact>);

    /// One `SquareInch` (in²), as an `Area`.
    pub const SQUARE_INCH<N: const From<Exact>>: Area<N> = counted(Exact::dec(64516, -8) * SQUARE_METRE::<Exact>);

    /// One `SquareMile` (mi²), as an `Area`.
    pub const SQUARE_MILE<N: const From<Exact>>: Area<N> = counted(Exact::dec(2589988110336, -6) * SQUARE_METRE::<Exact>);

    /// One `SquareYard` (yd²), as an `Area`.
    pub const SQUARE_YARD<N: const From<Exact>>: Area<N> = counted(Exact::dec(83612736, -8) * SQUARE_METRE::<Exact>);

    /// One `Saltus` (saltus), as an `Area`.
    pub const SALTUS<N: const From<Exact>>: Area<N> = counted(Exact::dec(201867264, -2) * SQUARE_METRE::<Exact>);

    /// One `Centuria` (centuria), as an `Area`.
    pub const CENTURIA<N: const From<Exact>>: Area<N> = counted(Exact::dec(50466816, -2) * SQUARE_METRE::<Exact>);

    /// One `Heredium` (heredium), as an `Area`.
    pub const HEREDIUM<N: const From<Exact>>: Area<N> = counted(Exact::dec(50466816, -4) * SQUARE_METRE::<Exact>);

    /// One `Jugerum` (jugerum), as an `Area`.
    pub const JUGERUM<N: const From<Exact>>: Area<N> = counted(Exact::dec(25233408, -4) * SQUARE_METRE::<Exact>);

    /// One `Deunx` (deunx), as an `Area`.
    pub const AREA_DEUNX<N: const From<Exact>>: Area<N> = counted(Exact::dec(23130624, -4) * SQUARE_METRE::<Exact>);

    /// One `Dextans` (dextans), as an `Area`.
    pub const AREA_DEXTANS<N: const From<Exact>>: Area<N> = counted(Exact::dec(2102784, -3) * SQUARE_METRE::<Exact>);

    /// One `Dodrans` (dodrans), as an `Area`.
    pub const AREA_DODRANS<N: const From<Exact>>: Area<N> = counted(Exact::dec(18925056, -4) * SQUARE_METRE::<Exact>);

    /// One `Bes` (bes), as an `Area`.
    pub const AREA_BES<N: const From<Exact>>: Area<N> = counted(Exact::dec(16822272, -4) * SQUARE_METRE::<Exact>);

    /// One `Septunx` (septunx), as an `Area`.
    pub const AREA_SEPTUNX<N: const From<Exact>>: Area<N> = counted(Exact::dec(14719488, -4) * SQUARE_METRE::<Exact>);

    /// One `Semis` (semis), as an `Area`.
    pub const AREA_SEMIS<N: const From<Exact>>: Area<N> = counted(Exact::dec(12616704, -4) * SQUARE_METRE::<Exact>);

    /// One `ActusQuadratus` (actus quadratus), as an `Area`.
    pub const ACTUS_QUADRATUS<N: const From<Exact>>: Area<N> = counted(Exact::dec(12616704, -4) * SQUARE_METRE::<Exact>);

    /// One `Quincunx` (quincunx), as an `Area`.
    pub const AREA_QUINCUNX<N: const From<Exact>>: Area<N> = counted(Exact::dec(1051392, -3) * SQUARE_METRE::<Exact>);

    /// One `Triens` (triens), as an `Area`.
    pub const AREA_TRIENS<N: const From<Exact>>: Area<N> = counted(Exact::dec(8411136, -4) * SQUARE_METRE::<Exact>);

    /// One `Quadrans` (quadrans), as an `Area`.
    pub const QUADRANS<N: const From<Exact>>: Area<N> = counted(Exact::dec(6308352, -4) * SQUARE_METRE::<Exact>);

    /// One `Sextans` (sextans), as an `Area`.
    pub const AREA_SEXTANS<N: const From<Exact>>: Area<N> = counted(Exact::dec(4205568, -4) * SQUARE_METRE::<Exact>);

    /// One `Clima` (clima), as an `Area`.
    pub const CLIMA<N: const From<Exact>>: Area<N> = counted(Exact::dec(3154176, -4) * SQUARE_METRE::<Exact>);

    /// One `Uncia` (uncia), as an `Area`.
    pub const AREA_UNCIA<N: const From<Exact>>: Area<N> = counted(Exact::dec(2102784, -4) * SQUARE_METRE::<Exact>);

    /// One `Semiuncia` (semiuncia), as an `Area`.
    pub const SEMIUNCIA<N: const From<Exact>>: Area<N> = counted(Exact::dec(1051392, -4) * SQUARE_METRE::<Exact>);

    /// One `Sicilicus` (sicilicus), as an `Area`.
    pub const AREA_SICILICUS<N: const From<Exact>>: Area<N> = counted(Exact::dec(525696, -4) * SQUARE_METRE::<Exact>);

    /// One `ActusSimplex` (actus simplex), as an `Area`.
    pub const ACTUS_SIMPLEX<N: const From<Exact>>: Area<N> = counted(Exact::dec(4205568, -5) * SQUARE_METRE::<Exact>);

    /// One `Sextula` (sextula), as an `Area`.
    pub const AREA_SEXTULA<N: const From<Exact>>: Area<N> = counted(Exact::dec(350464, -4) * SQUARE_METRE::<Exact>);

    /// One `DuoScrupulum` (duo scrupulum), as an `Area`.
    pub const DUO_SCRUPULUM<N: const From<Exact>>: Area<N> = counted(Exact::dec(175232, -4) * SQUARE_METRE::<Exact>);

    /// One `Scrupulum` (scrupulum), as an `Area`.
    pub const AREA_SCRUPULUM<N: const From<Exact>>: Area<N> = counted(Exact::dec(87616, -4) * SQUARE_METRE::<Exact>);

    /// One `DimidiumScrupulum` (dimidium scrupulum), as an `Area`.
    pub const DIMIDIUM_SCRUPULUM<N: const From<Exact>>: Area<N> = counted(Exact::dec(43808, -4) * SQUARE_METRE::<Exact>);

    /// One `PesQuadratus` (pes quadratus), as an `Area`.
    pub const PES_QUADRATUS<N: const From<Exact>>: Area<N> = counted(Exact::dec(87616, -6) * SQUARE_METRE::<Exact>);

    /// One `CubicMetre` (m³), as a `Volume`.
    pub const CUBIC_METRE<N: const From<Exact>>: Volume<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact>);

    /// One `AcreFoot` (ac · ft), as a `Volume`.
    pub const ACRE_FOOT<N: const From<Exact>>: Volume<N> = counted((Exact::int(43560) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937)) * (Exact::int(1200) / Exact::int(3937))) * CUBIC_METRE::<Exact>);

    /// One `Barrel` (bbl), as a `Volume`.
    pub const BARREL<N: const From<Exact>>: Volume<N> = counted(Exact::dec(158987294928, -12) * CUBIC_METRE::<Exact>);

    /// One `Bushel` (bu), as a `Volume`.
    pub const BUSHEL<N: const From<Exact>>: Volume<N> = counted(Exact::dec(3523907016688, -14) * CUBIC_METRE::<Exact>);

    /// One `Cord` (cords), as a `Volume`.
    pub const CORD<N: const From<Exact>>: Volume<N> = counted((Exact::int(128) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) * CUBIC_METRE::<Exact>);

    /// One `CubicFoot` (ft³), as a `Volume`.
    pub const CUBIC_FOOT<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) * CUBIC_METRE::<Exact>);

    /// One `CubicInch` (in³), as a `Volume`.
    pub const CUBIC_INCH<N: const From<Exact>>: Volume<N> = counted((Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)) * CUBIC_METRE::<Exact>);

    /// One `CubicMile` (mi³), as a `Volume`.
    pub const CUBIC_MILE<N: const From<Exact>>: Volume<N> = counted((Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3)) * CUBIC_METRE::<Exact>);

    /// One `CubicYard` (yd³), as a `Volume`.
    pub const CUBIC_YARD<N: const From<Exact>>: Volume<N> = counted((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) * CUBIC_METRE::<Exact>);

    /// One `Cup` (cup), as a `Volume`.
    pub const CUP<N: const From<Exact>>: Volume<N> = counted(Exact::dec(2365882365, -13) * CUBIC_METRE::<Exact>);

    /// One `FluidOunce` (fl oz), as a `Volume`.
    pub const FLUID_OUNCE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(295735295625, -16) * CUBIC_METRE::<Exact>);

    /// One `FluidOunceImperial` (fl oz (UK)), as a `Volume`.
    pub const FLUID_OUNCE_IMPERIAL<N: const From<Exact>>: Volume<N> = counted(Exact::dec(284130625, -13) * CUBIC_METRE::<Exact>);

    /// One `GallonImperial` (gal (UK)), as a `Volume`.
    pub const GALLON_IMPERIAL<N: const From<Exact>>: Volume<N> = counted(Exact::dec(454609, -8) * CUBIC_METRE::<Exact>);

    /// One `Gallon` (gal), as a `Volume`.
    pub const GALLON<N: const From<Exact>>: Volume<N> = counted(Exact::dec(3785411784, -12) * CUBIC_METRE::<Exact>);

    /// One `GillImperial` (gi (UK)), as a `Volume`.
    pub const GILL_IMPERIAL<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1420653125, -13) * CUBIC_METRE::<Exact>);

    /// One `Gill` (gi), as a `Volume`.
    pub const GILL<N: const From<Exact>>: Volume<N> = counted(Exact::dec(11829411825, -14) * CUBIC_METRE::<Exact>);

    /// One `Kilolitre` (kL), as a `Volume`.
    pub const KILOLITRE<N: const From<Exact>>: Volume<N> = counted(CUBIC_METRE::<Exact>);

    /// One `Litre` (L), as a `Volume`.
    pub const LITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -3) * CUBIC_METRE::<Exact>);

    /// One `Peck` (pk), as a `Volume`.
    pub const PECK<N: const From<Exact>>: Volume<N> = counted(Exact::dec(880976754172, -14) * CUBIC_METRE::<Exact>);

    /// One `PintDry` (dry pt), as a `Volume`.
    pub const PINT_DRY<N: const From<Exact>>: Volume<N> = counted(Exact::dec(5506104713575, -16) * CUBIC_METRE::<Exact>);

    /// One `PintLiquid` (liq pt), as a `Volume`.
    pub const PINT_LIQUID<N: const From<Exact>>: Volume<N> = counted(Exact::dec(473176473, -12) * CUBIC_METRE::<Exact>);

    /// One `QuartDry` (dry qt), as a `Volume`.
    pub const QUART_DRY<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1101220942715, -15) * CUBIC_METRE::<Exact>);

    /// One `QuartLiquid` (liq qt), as a `Volume`.
    pub const QUART_LIQUID<N: const From<Exact>>: Volume<N> = counted(Exact::dec(946352946, -12) * CUBIC_METRE::<Exact>);

    /// One `Stere` (st), as a `Volume`.
    pub const STERE<N: const From<Exact>>: Volume<N> = counted(CUBIC_METRE::<Exact>);

    /// One `Tablespoon` (tbsp), as a `Volume`.
    pub const TABLESPOON<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1478676478125, -17) * CUBIC_METRE::<Exact>);

    /// One `Teaspoon` (tsp), as a `Volume`.
    pub const TEASPOON<N: const From<Exact>>: Volume<N> = counted(Exact::dec(492892159375, -17) * CUBIC_METRE::<Exact>);

    /// One `RegisterTon` (RT), as a `Volume`.
    pub const REGISTER_TON<N: const From<Exact>>: Volume<N> = counted((Exact::int(100) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) * CUBIC_METRE::<Exact>);

    /// One `Culeus` (culeus), as a `Volume`.
    pub const CULEUS<N: const From<Exact>>: Volume<N> = counted(Exact::dec(51868672, -8) * CUBIC_METRE::<Exact>);

    /// One `AmphoraQuadrantal` (amphora quadrantal), as a `Volume`.
    pub const AMPHORA_QUADRANTAL<N: const From<Exact>>: Volume<N> = counted(Exact::dec(25934336, -9) * CUBIC_METRE::<Exact>);

    /// One `Urna` (urna), as a `Volume`.
    pub const URNA<N: const From<Exact>>: Volume<N> = counted(Exact::dec(12967168, -9) * CUBIC_METRE::<Exact>);

    /// One `ModiusCastrensis` (modius castrensis), as a `Volume`.
    pub const MODIUS_CASTRENSIS<N: const From<Exact>>: Volume<N> = counted(Exact::dec(12967168, -9) * CUBIC_METRE::<Exact>);

    /// One `Modius` (modius), as a `Volume`.
    pub const MODIUS<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3241792, -9) * Exact::int(16) / Exact::int(6)) * CUBIC_METRE::<Exact>);

    /// One `Semimodius` (semimodius), as a `Volume`.
    pub const SEMIMODIUS<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3241792, -9) * Exact::int(8) / Exact::int(6)) * CUBIC_METRE::<Exact>);

    /// One `Congius` (congius), as a `Volume`.
    pub const CONGIUS<N: const From<Exact>>: Volume<N> = counted(Exact::dec(3241792, -9) * CUBIC_METRE::<Exact>);

    /// One `Sextarius` (sextarius), as a `Volume`.
    pub const SEXTARIUS<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3241792, -9) / Exact::int(6)) * CUBIC_METRE::<Exact>);

    /// One `Hemina` (hemina), as a `Volume`.
    pub const HEMINA<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3241792, -9) / Exact::int(12)) * CUBIC_METRE::<Exact>);

    /// One `Quartarius` (quartarius), as a `Volume`.
    pub const QUARTARIUS<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3241792, -9) / Exact::int(24)) * CUBIC_METRE::<Exact>);

    /// One `Acetabulum` (acetabulum), as a `Volume`.
    pub const ACETABULUM<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3241792, -9) / Exact::int(48)) * CUBIC_METRE::<Exact>);

    /// One `Cyathus` (cyathus), as a `Volume`.
    pub const CYATHUS<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3241792, -9) / Exact::int(72)) * CUBIC_METRE::<Exact>);

    /// One `Ligula` (ligula), as a `Volume`.
    pub const LIGULA<N: const From<Exact>>: Volume<N> = counted((Exact::dec(3241792, -9) / Exact::int(288)) * CUBIC_METRE::<Exact>);

    /// One `MetrePerSecond` (m/s), as a `Velocity`.
    pub const METRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(METRE::<Exact> / SECOND::<Exact>);

    /// One `FootPerHour` (ft/h), as a `Velocity`.
    pub const FOOT_PER_HOUR<N: const From<Exact>>: Velocity<N> = counted((Exact::dec(3048, -4) / Exact::int(3600)) * METRE_PER_SECOND::<Exact>);

    /// One `FootPerMinute` (ft/min), as a `Velocity`.
    pub const FOOT_PER_MINUTE<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(508, -5) * METRE_PER_SECOND::<Exact>);

    /// One `FootPerSecond` (ft/s), as a `Velocity`.
    pub const FOOT_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(3048, -4) * METRE_PER_SECOND::<Exact>);

    /// One `InchPerSecond` (in/s), as a `Velocity`.
    pub const INCH_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(254, -4) * METRE_PER_SECOND::<Exact>);

    /// One `InchPerMinute` (in/min), as a `Velocity`.
    pub const INCH_PER_MINUTE<N: const From<Exact>>: Velocity<N> = counted((Exact::dec(254, -4) / Exact::int(60)) * METRE_PER_SECOND::<Exact>);

    /// One `KilometrePerHour` (km/h), as a `Velocity`.
    pub const KILOMETRE_PER_HOUR<N: const From<Exact>>: Velocity<N> = counted((Exact::int(1000) / Exact::int(3600)) * METRE_PER_SECOND::<Exact>);

    /// One `Knot` (kn), as a `Velocity`.
    pub const KNOT<N: const From<Exact>>: Velocity<N> = counted((Exact::int(1852) / Exact::int(3600)) * METRE_PER_SECOND::<Exact>);

    /// One `MilePerHour` (mi/h), as a `Velocity`.
    pub const MILE_PER_HOUR<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(44704, -5) * METRE_PER_SECOND::<Exact>);

    /// One `MilePerMinute` (mi/min), as a `Velocity`.
    pub const MILE_PER_MINUTE<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(268224, -4) * METRE_PER_SECOND::<Exact>);

    /// One `MilePerSecond` (mi/s), as a `Velocity`.
    pub const MILE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1609344, -3) * METRE_PER_SECOND::<Exact>);

    /// One `MillimetrePerMinute` (mm/min), as a `Velocity`.
    pub const MILLIMETRE_PER_MINUTE<N: const From<Exact>>: Velocity<N> = counted((Exact::dec(1, -3) / Exact::int(60)) * METRE_PER_SECOND::<Exact>);

    /// One `AtomicUnitOfVelocity` (a₀ · Eₕ/ħ), as a `Velocity`.
    pub const ATOMIC_UNIT_OF_VELOCITY<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(218769126364, -5) * METRE_PER_SECOND::<Exact>);

    /// One `NaturalUnitOfVelocity` (c), as a `Velocity`.
    pub const NATURAL_UNIT_OF_VELOCITY<N: const From<Exact>>: Velocity<N> = counted(Exact::int(299792458) * METRE_PER_SECOND::<Exact>);

    /// One `SpeedOfLight` (c), as a `Velocity`.
    pub const SPEED_OF_LIGHT<N: const From<Exact>>: Velocity<N> = counted(Exact::int(299792458) * METRE_PER_SECOND::<Exact>);

    /// One `MetrePerSecondSquared` (m/s²), as an `Acceleration`.
    pub const METRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `FootPerSecondSquared` (ft/s²), as an `Acceleration`.
    pub const FOOT_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(3048, -4) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `Galileo` (Gal), as an `Acceleration`.
    pub const GALILEO<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -2) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `InchPerSecondSquared` (in/s²), as an `Acceleration`.
    pub const INCH_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(254, -4) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `MillimetrePerMinuteSquared` (mm/min²), as an `Acceleration`.
    pub const MILLIMETRE_PER_MINUTE_SQUARED<N: const From<Exact>>: Acceleration<N> = counted((Exact::dec(1, -3) / (Exact::int(60) * Exact::int(60))) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `StandardGravity` (g₀), as an `Acceleration`.
    pub const STANDARD_GRAVITY<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(980665, -5) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `MetrePerSecondCubed` (m/s³), as a `Jerk`.
    pub const METRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `FootPerSecondCubed` (ft/s³), as a `Jerk`.
    pub const FOOT_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(3048, -4) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `InchPerSecondCubed` (in/s³), as a `Jerk`.
    pub const INCH_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(254, -4) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `KilometrePerMinuteCubed` (km/min³), as a `Jerk`.
    pub const KILOMETRE_PER_MINUTE_CUBED<N: const From<Exact>>: Jerk<N> = counted((Exact::int(1000) / (Exact::int(60) * Exact::int(60) * Exact::int(60))) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `MetreSecond` (m · s), as an `Absement`.
    pub const METRE_SECOND<N: const From<Exact>>: Absement<N> = counted(METRE::<Exact> * SECOND::<Exact>);

    /// One `FootSecond` (ft · s), as an `Absement`.
    pub const FOOT_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(3048, -4) * METRE_SECOND::<Exact>);

    /// One `InchSecond` (in · s), as an `Absement`.
    pub const INCH_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(254, -4) * METRE_SECOND::<Exact>);

    /// One `KilometreHour` (km · h), as an `Absement`.
    pub const KILOMETRE_HOUR<N: const From<Exact>>: Absement<N> = counted(Exact::int(3600000) * METRE_SECOND::<Exact>);

    /// One `SecondPerMetre` (s/m), as an `InverseVelocity`.
    pub const SECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(SECOND::<Exact> / METRE::<Exact>);

    /// One `HourPerFoot` (h/ft), as an `InverseVelocity`.
    pub const HOUR_PER_FOOT<N: const From<Exact>>: InverseVelocity<N> = counted((Exact::int(3600) / Exact::dec(3048, -4)) * SECOND_PER_METRE::<Exact>);

    /// One `MinutePerFoot` (min/ft), as an `InverseVelocity`.
    pub const MINUTE_PER_FOOT<N: const From<Exact>>: InverseVelocity<N> = counted((Exact::int(60) / Exact::dec(3048, -4)) * SECOND_PER_METRE::<Exact>);

    /// One `SecondPerFoot` (s/ft), as an `InverseVelocity`.
    pub const SECOND_PER_FOOT<N: const From<Exact>>: InverseVelocity<N> = counted((Exact::int(1) / Exact::dec(3048, -4)) * SECOND_PER_METRE::<Exact>);

    /// One `SecondPerInch` (s/in), as an `InverseVelocity`.
    pub const SECOND_PER_INCH<N: const From<Exact>>: InverseVelocity<N> = counted((Exact::int(1) / Exact::dec(254, -4)) * SECOND_PER_METRE::<Exact>);

    /// One `MinutePerKilometre` (min/km), as an `InverseVelocity`.
    pub const MINUTE_PER_KILOMETRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(6, -2) * SECOND_PER_METRE::<Exact>);

    /// One `HourPerKilometre` (h/km), as an `InverseVelocity`.
    pub const HOUR_PER_KILOMETRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(36, -1) * SECOND_PER_METRE::<Exact>);

    /// One `HourPerMile` (h/mi), as an `InverseVelocity`.
    pub const HOUR_PER_MILE<N: const From<Exact>>: InverseVelocity<N> = counted((Exact::int(3600) / Exact::dec(1609344, -3)) * SECOND_PER_METRE::<Exact>);

    /// One `MinutePerMile` (min/mi), as an `InverseVelocity`.
    pub const MINUTE_PER_MILE<N: const From<Exact>>: InverseVelocity<N> = counted((Exact::int(60) / Exact::dec(1609344, -3)) * SECOND_PER_METRE::<Exact>);

    /// One `SecondPerMile` (s/mi), as an `InverseVelocity`.
    pub const SECOND_PER_MILE<N: const From<Exact>>: InverseVelocity<N> = counted((Exact::int(1) / Exact::dec(1609344, -3)) * SECOND_PER_METRE::<Exact>);

    /// One `Newton` (N), as a `Force`.
    pub const NEWTON<N: const From<Exact>>: Force<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `Dyne` (dyn), as a `Force`.
    pub const DYNE<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -5) * NEWTON::<Exact>);

    /// One `KilogramForce` (kgf), as a `Force`.
    pub const KILOGRAM_FORCE<N: const From<Exact>>: Force<N> = counted(Exact::dec(980665, -5) * NEWTON::<Exact>);

    /// One `Kip` (kip), as a `Force`.
    pub const KIP<N: const From<Exact>>: Force<N> = counted((Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * NEWTON::<Exact>);

    /// One `OunceForce` (ozf), as a `Force`.
    pub const OUNCE_FORCE<N: const From<Exact>>: Force<N> = counted((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16)) * NEWTON::<Exact>);

    /// One `Poundal` (pdl), as a `Force`.
    pub const POUNDAL<N: const From<Exact>>: Force<N> = counted((Exact::dec(45359237, -8) * Exact::dec(3048, -4)) * NEWTON::<Exact>);

    /// One `PoundForce` (lbf), as a `Force`.
    pub const POUND_FORCE<N: const From<Exact>>: Force<N> = counted((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * NEWTON::<Exact>);

    /// One `TonForce` (2000 lbf), as a `Force`.
    pub const TON_FORCE<N: const From<Exact>>: Force<N> = counted((Exact::int(2000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * NEWTON::<Exact>);

    /// One `KilogramMetrePerSecond` (kg · m/s), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / SECOND::<Exact>);

    /// One `GramMetrePerSecond` (g · m/s), as a `Momentum`.
    pub const GRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `TonMetrePerSecond` (t · m/s), as a `Momentum`.
    pub const TON_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramMetrePerMinute` (kg · m/min), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_MINUTE<N: const From<Exact>>: Momentum<N> = counted((Exact::int(1) / Exact::int(60)) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramMetrePerHour` (kg · m/h), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_HOUR<N: const From<Exact>>: Momentum<N> = counted((Exact::int(1) / Exact::int(3600)) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramMetrePerDay` (kg · m/d), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_DAY<N: const From<Exact>>: Momentum<N> = counted((Exact::int(1) / Exact::int(86400)) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `SlugFootPerSecond` (slug · ft/s), as a `Momentum`.
    pub const SLUG_FOOT_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * Exact::dec(3048, -4)) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `SlugInchPerSecond` (slug · in/s), as a `Momentum`.
    pub const SLUG_INCH_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4)) * Exact::dec(254, -4)) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `PoundFootPerSecond` (lb · ft/s), as a `Momentum`.
    pub const POUND_FOOT_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(138254954376, -12) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `PoundInchPerSecond` (lb · in/s), as a `Momentum`.
    pub const POUND_INCH_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(11521246198, -12) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `Joule` (J), as an `Energy`.
    pub const JOULE<N: const From<Exact>>: Energy<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `WattHour` (W · h), as an `Energy`.
    pub const WATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(3600) * JOULE::<Exact>);

    /// One `Electronvolt` (eV), as an `Energy`.
    pub const ELECTRONVOLT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1602176634, -28) * JOULE::<Exact>);

    /// One `Hartree` (Eₕ), as an `Energy`.
    pub const HARTREE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(43597447222071, -31) * JOULE::<Exact>);

    /// One `BtuIt` (Btu (IT)), as an `Energy`.
    pub const BTU_IT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(105505585262, -8) * JOULE::<Exact>);

    /// One `Btu` (Btu), as an `Energy`.
    pub const BTU<N: const From<Exact>>: Energy<N> = counted((Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1)) * JOULE::<Exact>);

    /// One `CalorieIt` (cal (IT)), as an `Energy`.
    pub const CALORIE_IT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(41868, -4) * JOULE::<Exact>);

    /// One `Calorie` (cal), as an `Energy`.
    pub const CALORIE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(4184, -3) * JOULE::<Exact>);

    /// One `CalorieItNutrition` (Cal (IT)), as an `Energy`.
    pub const CALORIE_IT_NUTRITION<N: const From<Exact>>: Energy<N> = counted(Exact::dec(41868, -1) * JOULE::<Exact>);

    /// One `CalorieNutrition` (Cal), as an `Energy`.
    pub const CALORIE_NUTRITION<N: const From<Exact>>: Energy<N> = counted(Exact::int(4184) * JOULE::<Exact>);

    /// One `Erg` (erg), as an `Energy`.
    pub const ERG<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -7) * JOULE::<Exact>);

    /// One `FootPoundal` (ft · pdl), as an `Energy`.
    pub const FOOT_POUNDAL<N: const From<Exact>>: Energy<N> = counted((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4)) * JOULE::<Exact>);

    /// One `FootPound` (ft · lbf), as an `Energy`.
    pub const FOOT_POUND<N: const From<Exact>>: Energy<N> = counted((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * JOULE::<Exact>);

    /// One `Quad` (10¹⁵ Btu (IT)), as an `Energy`.
    pub const QUAD<N: const From<Exact>>: Energy<N> = counted(Exact::int(1055055852620000000) * JOULE::<Exact>);

    /// One `ThermEc` (thm (EC)), as an `Energy`.
    pub const THERM_EC<N: const From<Exact>>: Energy<N> = counted(Exact::int(105506000) * JOULE::<Exact>);

    /// One `ThermUs` (thm), as an `Energy`.
    pub const THERM_US<N: const From<Exact>>: Energy<N> = counted(Exact::int(105480400) * JOULE::<Exact>);

    /// One `TonTnt` (t of TNT), as an `Energy`.
    pub const TON_TNT<N: const From<Exact>>: Energy<N> = counted(Exact::int(4184000000) * JOULE::<Exact>);

    /// One `WattSecond` (W · s), as an `Energy`.
    pub const WATT_SECOND<N: const From<Exact>>: Energy<N> = counted(JOULE::<Exact>);

    /// One `JoulePerKilogram` (J/kg), as an `AvailableEnergy`.
    pub const JOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(METRE::<Exact> * METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `JoulePerGram` (J/g), as an `AvailableEnergy`.
    pub const JOULE_PER_GRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `BtuItPerPound` (Btu (IT)/lb), as an `AvailableEnergy`.
    pub const BTU_IT_PER_POUND<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(2326) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `BtuPerPound` (Btu/lb), as an `AvailableEnergy`.
    pub const BTU_PER_POUND<N: const From<Exact>>: AvailableEnergy<N> = counted((Exact::int(4184) / Exact::dec(18, -1)) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `CalorieItPerGram` (cal (IT)/g), as an `AvailableEnergy`.
    pub const CALORIE_IT_PER_GRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(41868, -1) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `CaloriePerGram` (cal/g), as an `AvailableEnergy`.
    pub const CALORIE_PER_GRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(4184) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `Watt` (W), as a `Power`.
    pub const WATT<N: const From<Exact>>: Power<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `ErgPerSecond` (erg/s), as a `Power`.
    pub const ERG_PER_SECOND<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -7) * WATT::<Exact>);

    /// One `FootPoundPerHour` (ft · lbf/h), as a `Power`.
    pub const FOOT_POUND_PER_HOUR<N: const From<Exact>>: Power<N> = counted(((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::int(3600)) * WATT::<Exact>);

    /// One `FootPoundPerMinute` (ft · lbf/min), as a `Power`.
    pub const FOOT_POUND_PER_MINUTE<N: const From<Exact>>: Power<N> = counted(((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::int(60)) * WATT::<Exact>);

    /// One `FootPoundPerSecond` (ft · lbf/s), as a `Power`.
    pub const FOOT_POUND_PER_SECOND<N: const From<Exact>>: Power<N> = counted((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * WATT::<Exact>);

    /// One `Horsepower` (hp), as a `Power`.
    pub const HORSEPOWER<N: const From<Exact>>: Power<N> = counted((Exact::int(550) * Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * WATT::<Exact>);

    /// One `HorsepowerBoiler` (hp (S)), as a `Power`.
    pub const HORSEPOWER_BOILER<N: const From<Exact>>: Power<N> = counted(Exact::dec(98095, -1) * WATT::<Exact>);

    /// One `HorsepowerElectric` (hp (E)), as a `Power`.
    pub const HORSEPOWER_ELECTRIC<N: const From<Exact>>: Power<N> = counted(Exact::int(746) * WATT::<Exact>);

    /// One `HorsepowerMetric` (hp (M)), as a `Power`.
    pub const HORSEPOWER_METRIC<N: const From<Exact>>: Power<N> = counted((Exact::int(75) * Exact::dec(980665, -5)) * WATT::<Exact>);

    /// One `HorsepowerImperial` (hp (I)), as a `Power`.
    pub const HORSEPOWER_IMPERIAL<N: const From<Exact>>: Power<N> = counted(Exact::dec(7457, -1) * WATT::<Exact>);

    /// One `HydraulicHorsepower` (hp (hydraulic)), as a `Power`.
    pub const HYDRAULIC_HORSEPOWER<N: const From<Exact>>: Power<N> = counted(Exact::dec(746043, -3) * WATT::<Exact>);

    /// One `Decibel` (dB), as a `Gain`.
    pub const DECIBEL<N: const From<Exact>>: Gain<N> = counted(Exact::dec(1, -1) * BEL::<Exact>);

    /// One `Neper` (Np), as a `Gain`.
    pub const NEPER<N: const From<Exact>>: Gain<N> = counted(Exact::dec(8685889638065036, -16) * BEL::<Exact>);

    /// The level of one milliwatt, 0 dBm: 30 dB below a watt.
    pub const MILLIWATT_LEVEL<N: const From<Exact>>: PowerLevel<N> = counted(WATT_LEVEL::<Exact> - Exact::int(30) * DECIBEL::<Exact>);

    /// Decibels-watt (dBW): decibels from one watt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBW<N: const From<Exact>>: Scale<PowerLevel<N>, Gain<N>> = Scale(WATT_LEVEL, DECIBEL);

    /// Decibels-milliwatt (dBm): decibels from one milliwatt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBM<N: const From<Exact>>: Scale<PowerLevel<N>, Gain<N>> = Scale(MILLIWATT_LEVEL, DECIBEL);

    /// One `WattPerSecond` (W/s), as a `PowerRate`.
    pub const WATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `ErgPerSecondSquared` (erg/s²), as a `PowerRate`.
    pub const ERG_PER_SECOND_SQUARED<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -7) * WATT_PER_SECOND::<Exact>);

    /// One `FootPoundPerSecondSquared` (ft · lbf/s²), as a `PowerRate`.
    pub const FOOT_POUND_PER_SECOND_SQUARED<N: const From<Exact>>: PowerRate<N> = counted((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * WATT_PER_SECOND::<Exact>);

    /// One `HorsepowerPerSecond` (hp/s), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted((Exact::int(550) * Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * WATT_PER_SECOND::<Exact>);

    /// One `HorsepowerPerSecondBoiler` (hp/s (S)), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND_BOILER<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(98095, -1) * WATT_PER_SECOND::<Exact>);

    /// One `HorsepowerPerSecondElectric` (hp/s (E)), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND_ELECTRIC<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(746) * WATT_PER_SECOND::<Exact>);

    /// One `HorsepowerPerSecondMetric` (hp/s (M)), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND_METRIC<N: const From<Exact>>: PowerRate<N> = counted((Exact::int(75) * Exact::dec(980665, -5)) * WATT_PER_SECOND::<Exact>);

    /// One `HorsepowerPerSecondImperial` (hp/s (I)), as a `PowerRate`.
    pub const HORSEPOWER_PER_SECOND_IMPERIAL<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(7457, -1) * WATT_PER_SECOND::<Exact>);

    /// One `HydraulicHorsepowerPerSecond` (hp/s (hydraulic)), as a `PowerRate`.
    pub const HYDRAULIC_HORSEPOWER_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(746043, -3) * WATT_PER_SECOND::<Exact>);

    /// One `JouleSecond` (J · s), as an `Action`.
    pub const JOULE_SECOND<N: const From<Exact>>: Action<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / SECOND::<Exact>);

    /// One `AtomicUnitOfAction` (ħ), as an `Action`.
    pub const ATOMIC_UNIT_OF_ACTION<N: const From<Exact>>: Action<N> = counted((Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI)) * JOULE_SECOND::<Exact>);

    /// One `PlanckConstant` (h), as an `Action`.
    pub const PLANCK_CONSTANT<N: const From<Exact>>: Action<N> = counted(Exact::dec(662607015, -42) * JOULE_SECOND::<Exact>);

    /// One `ErgSecond` (erg · s), as an `Action`.
    pub const ERG_SECOND<N: const From<Exact>>: Action<N> = counted(Exact::dec(1, -7) * JOULE_SECOND::<Exact>);

    /// One `ElectronvoltSecond` (eV · s), as an `Action`.
    pub const ELECTRONVOLT_SECOND<N: const From<Exact>>: Action<N> = counted(Exact::dec(1602176634, -28) * JOULE_SECOND::<Exact>);

    /// One `NewtonMetre` (N · m), as a `Torque`.
    pub const NEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * RADIAN::<Exact>));

    /// One `DyneMetre` (dyn · m), as a `Torque`.
    pub const DYNE_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -5) * NEWTON_METRE::<Exact>);

    /// One `KilogramForceMetre` (kgf · m), as a `Torque`.
    pub const KILOGRAM_FORCE_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(980665, -5) * NEWTON_METRE::<Exact>);

    /// One `OunceForceInch` (ozf · in), as a `Torque`.
    pub const OUNCE_FORCE_INCH<N: const From<Exact>>: Torque<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16)) * Exact::dec(254, -4)) * NEWTON_METRE::<Exact>);

    /// One `PoundForceFoot` (lbf · ft), as a `Torque`.
    pub const POUND_FORCE_FOOT<N: const From<Exact>>: Torque<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(3048, -4)) * NEWTON_METRE::<Exact>);

    /// One `PoundForceInch` (lbf · in), as a `Torque`.
    pub const POUND_FORCE_INCH<N: const From<Exact>>: Torque<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(254, -4)) * NEWTON_METRE::<Exact>);

    /// One `KilogramSquareMetre` (kg · m²), as a `MomentOfInertia`.
    pub const KILOGRAM_SQUARE_METRE<N: const From<Exact>>: MomentOfInertia<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (RADIAN::<Exact> * RADIAN::<Exact>));

    /// One `DaltonSquareAngstrom` (Da · Å²), as a `MomentOfInertia`.
    pub const DALTON_SQUARE_ANGSTROM<N: const From<Exact>>: MomentOfInertia<N> = counted(Exact::dec(16605390666, -57) * KILOGRAM_SQUARE_METRE::<Exact>);

    /// One `Pascal` (Pa), as a `Pressure`.
    pub const PASCAL<N: const From<Exact>>: Pressure<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `StandardAtmosphere` (atm), as a `Pressure`.
    pub const STANDARD_ATMOSPHERE<N: const From<Exact>>: Pressure<N> = counted(Exact::int(101325) * PASCAL::<Exact>);

    /// One `AtmosphereTechnical` (at), as a `Pressure`.
    pub const ATMOSPHERE_TECHNICAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(980665, -1) * PASCAL::<Exact>);

    /// One `Bar` (bar), as a `Pressure`.
    pub const BAR<N: const From<Exact>>: Pressure<N> = counted(Exact::int(100000) * PASCAL::<Exact>);

    /// One `CentimetreOfMercury` (cm Hg), as a `Pressure`.
    pub const CENTIMETRE_OF_MERCURY<N: const From<Exact>>: Pressure<N> = counted((Exact::dec(133322387415, -9) * Exact::int(10)) * PASCAL::<Exact>);

    /// One `CentimetreOfWater` (cm H₂O), as a `Pressure`.
    pub const CENTIMETRE_OF_WATER<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(980665, -4) * PASCAL::<Exact>);

    /// One `DynePerSquareCentimetre` (dyn/cm²), as a `Pressure`.
    pub const DYNE_PER_SQUARE_CENTIMETRE<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -1) * PASCAL::<Exact>);

    /// One `FootOfMercury` (ft Hg), as a `Pressure`.
    pub const FOOT_OF_MERCURY<N: const From<Exact>>: Pressure<N> = counted((Exact::dec(133322387415, -9) * Exact::dec(3048, -1)) * PASCAL::<Exact>);

    /// One `FootOfWater` (ft H₂O), as a `Pressure`.
    pub const FOOT_OF_WATER<N: const From<Exact>>: Pressure<N> = counted((Exact::dec(980665, -5) * Exact::dec(3048, -1)) * PASCAL::<Exact>);

    /// One `InchOfMercury` (in Hg), as a `Pressure`.
    pub const INCH_OF_MERCURY<N: const From<Exact>>: Pressure<N> = counted((Exact::dec(133322387415, -9) * Exact::dec(254, -1)) * PASCAL::<Exact>);

    /// One `InchOfWater` (in H₂O), as a `Pressure`.
    pub const INCH_OF_WATER<N: const From<Exact>>: Pressure<N> = counted((Exact::dec(980665, -5) * Exact::dec(254, -1)) * PASCAL::<Exact>);

    /// One `NewtonPerSquareMillimetre` (N/mm²), as a `Pressure`.
    pub const NEWTON_PER_SQUARE_MILLIMETRE<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000000) * PASCAL::<Exact>);

    /// One `KipPerSquareInch` (kip/in²), as a `Pressure`.
    pub const KIP_PER_SQUARE_INCH<N: const From<Exact>>: Pressure<N> = counted(((Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(254, -4) * Exact::dec(254, -4))) * PASCAL::<Exact>);

    /// One `MillimetreOfMercury` (mm Hg), as a `Pressure`.
    pub const MILLIMETRE_OF_MERCURY<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(133322387415, -9) * PASCAL::<Exact>);

    /// One `MillimetreOfWater` (mm H₂O), as a `Pressure`.
    pub const MILLIMETRE_OF_WATER<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(980665, -5) * PASCAL::<Exact>);

    /// One `PoundalPerSquareFoot` (pdl/ft²), as a `Pressure`.
    pub const POUNDAL_PER_SQUARE_FOOT<N: const From<Exact>>: Pressure<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(3048, -4)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * PASCAL::<Exact>);

    /// One `PoundForcePerSquareFoot` (lbf/ft²), as a `Pressure`.
    pub const POUND_FORCE_PER_SQUARE_FOOT<N: const From<Exact>>: Pressure<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(3048, -4) * Exact::dec(3048, -4))) * PASCAL::<Exact>);

    /// One `PoundForcePerSquareInch` (lbf/in²), as a `Pressure`.
    pub const POUND_FORCE_PER_SQUARE_INCH<N: const From<Exact>>: Pressure<N> = counted(((Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::dec(254, -4) * Exact::dec(254, -4))) * PASCAL::<Exact>);

    /// One `Psi` (psi), as a `Pressure`.
    pub const PSI<N: const From<Exact>>: Pressure<N> = counted((Exact::dec(45359237, -8) * Exact::dec(980665, -5) / (Exact::dec(254, -4) * Exact::dec(254, -4))) * PASCAL::<Exact>);

    /// One `Torr` (Torr), as a `Pressure`.
    pub const TORR<N: const From<Exact>>: Pressure<N> = counted((Exact::int(101325) / Exact::int(760)) * PASCAL::<Exact>);

    /// One `Hertz` (Hz), as a `Frequency`.
    pub const HERTZ<N: const From<Exact>>: Frequency<N> = counted(ONE::<Exact> / SECOND::<Exact>);

    /// One `CyclePerDay` (1/d), as a `Frequency`.
    pub const CYCLE_PER_DAY<N: const From<Exact>>: Frequency<N> = counted((Exact::int(1) / Exact::int(86400)) * HERTZ::<Exact>);

    /// One `CyclePerHour` (1/h), as a `Frequency`.
    pub const CYCLE_PER_HOUR<N: const From<Exact>>: Frequency<N> = counted((Exact::int(1) / Exact::int(3600)) * HERTZ::<Exact>);

    /// One `CyclePerMinute` (1/min), as a `Frequency`.
    pub const CYCLE_PER_MINUTE<N: const From<Exact>>: Frequency<N> = counted((Exact::int(1) / Exact::int(60)) * HERTZ::<Exact>);

    /// One `CyclePerShake` (100 MHz), as a `Frequency`.
    pub const CYCLE_PER_SHAKE<N: const From<Exact>>: Frequency<N> = counted(Exact::int(100000000) * HERTZ::<Exact>);

    /// One `CyclePerYear` (1/a), as a `Frequency`.
    pub const CYCLE_PER_YEAR<N: const From<Exact>>: Frequency<N> = counted((Exact::int(1) / Exact::int(31536000)) * HERTZ::<Exact>);

    /// One `HertzPerSecond` (Hz/s), as a `FrequencyDrift`.
    pub const HERTZ_PER_SECOND<N: const From<Exact>>: FrequencyDrift<N> = counted(ONE::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `ReciprocalMetre` (m⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_METRE<N: const From<Exact>>: ReciprocalLength<N> = counted(ONE::<Exact> / METRE::<Exact>);

    /// One `ReciprocalAngstrom` (Å⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_ANGSTROM<N: const From<Exact>>: ReciprocalLength<N> = counted(Exact::int(10000000000) * RECIPROCAL_METRE::<Exact>);

    /// One `Diopter` (dpt), as a `ReciprocalLength`.
    pub const DIOPTER<N: const From<Exact>>: ReciprocalLength<N> = counted(RECIPROCAL_METRE::<Exact>);

    /// One `RydbergConstant` (R∞), as a `ReciprocalLength`.
    pub const RYDBERG_CONSTANT<N: const From<Exact>>: ReciprocalLength<N> = counted(Exact::dec(1097373156816, -5) * RECIPROCAL_METRE::<Exact>);

    /// One `RadianPerMetre` (rad/m), as a `Curvature`.
    pub const RADIAN_PER_METRE<N: const From<Exact>>: Curvature<N> = counted(RADIAN::<Exact> / METRE::<Exact>);

    /// One `DegreePerMetre` (°/m), as a `Curvature`.
    pub const DEGREE_PER_METRE<N: const From<Exact>>: Curvature<N> = counted((Exact::PI / Exact::int(180)) * RADIAN_PER_METRE::<Exact>);

    /// One `SquareMetrePerKilogram` (m²/kg), as a `SpecificArea`.
    pub const SQUARE_METRE_PER_KILOGRAM<N: const From<Exact>>: SpecificArea<N> = counted(METRE::<Exact> * METRE::<Exact> / KILOGRAM::<Exact>);

    /// One `SquareMetrePerGram` (m²/g), as a `SpecificArea`.
    pub const SQUARE_METRE_PER_GRAM<N: const From<Exact>>: SpecificArea<N> = counted(Exact::int(1000) * SQUARE_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerKilogram` (m³/kg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_KILOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> / KILOGRAM::<Exact>);

    /// One `CubicMetrePerGram` (m³/g), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_GRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerCarat` (m³/ct), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_CARAT<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(5000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerGrain` (m³/gr), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_GRAIN<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(6479891, -11)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerHundredweightLong` (m³/cwt long), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_HUNDREDWEIGHT_LONG<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(5080234544, -8)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerHundredweightShort` (m³/cwt short), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_HUNDREDWEIGHT_SHORT<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(45359237, -6)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerOunce` (m³/oz), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_OUNCE<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(28349523125, -12)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerOunceTroy` (m³/oz t), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_OUNCE_TROY<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(311034768, -10)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerPennyweight` (m³/dwt), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_PENNYWEIGHT<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(155517384, -11)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerPound` (m³/lb), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_POUND<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(45359237, -8)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerPoundTroy` (m³/lb t), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_POUND_TROY<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(3732417216, -10)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerSlug` (m³/slug), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_SLUG<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4))) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerTonAssay` (m³/AT), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TON_ASSAY<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / (Exact::dec(175, -3) / Exact::int(6))) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerTonLong` (m³/2240 lb), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TON_LONG<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(10160469088, -7)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerTonShort` (m³/2000 lb), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TON_SHORT<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::int(1) / Exact::dec(90718474, -5)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerTon` (m³/t), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TON<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::dec(1, -3) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `GallonPerGrain` (gal/gr), as a `SpecificVolume`.
    pub const GALLON_PER_GRAIN<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::dec(3785411784, -12) / Exact::dec(6479891, -11)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicInchPerOunce` (in³/oz), as a `SpecificVolume`.
    pub const CUBIC_INCH_PER_OUNCE<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4) / Exact::dec(28349523125, -12)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `GallonImperialPerOunce` (gal (UK)/oz), as a `SpecificVolume`.
    pub const GALLON_IMPERIAL_PER_OUNCE<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::dec(454609, -8) / Exact::dec(28349523125, -12)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `GallonPerOunce` (gal/oz), as a `SpecificVolume`.
    pub const GALLON_PER_OUNCE<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::dec(3785411784, -12) / Exact::dec(28349523125, -12)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicFootPerPound` (ft³/lb), as a `SpecificVolume`.
    pub const CUBIC_FOOT_PER_POUND<N: const From<Exact>>: SpecificVolume<N> = counted(((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(45359237, -8)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicInchPerPound` (in³/lb), as a `SpecificVolume`.
    pub const CUBIC_INCH_PER_POUND<N: const From<Exact>>: SpecificVolume<N> = counted(((Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4)) / Exact::dec(45359237, -8)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicYardPerPound` (yd³/lb), as a `SpecificVolume`.
    pub const CUBIC_YARD_PER_POUND<N: const From<Exact>>: SpecificVolume<N> = counted(((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(45359237, -8)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `GallonImperialPerPound` (gal (UK)/lb), as a `SpecificVolume`.
    pub const GALLON_IMPERIAL_PER_POUND<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::dec(454609, -8) / Exact::dec(45359237, -8)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `GallonPerPound` (gal/lb), as a `SpecificVolume`.
    pub const GALLON_PER_POUND<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::dec(3785411784, -12) / Exact::dec(45359237, -8)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicFootPerSlug` (ft³/slug), as a `SpecificVolume`.
    pub const CUBIC_FOOT_PER_SLUG<N: const From<Exact>>: SpecificVolume<N> = counted((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4) / (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4))) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicYardPerTonLong` (yd³/2240 lb), as a `SpecificVolume`.
    pub const CUBIC_YARD_PER_TON_LONG<N: const From<Exact>>: SpecificVolume<N> = counted(((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(10160469088, -7)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicYardPerTonShort` (yd³/2000 lb), as a `SpecificVolume`.
    pub const CUBIC_YARD_PER_TON_SHORT<N: const From<Exact>>: SpecificVolume<N> = counted(((Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4)) / Exact::dec(90718474, -5)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicFootPerTonLong` (ft³/2240 lb), as a `SpecificVolume`.
    pub const CUBIC_FOOT_PER_TON_LONG<N: const From<Exact>>: SpecificVolume<N> = counted(((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(10160469088, -7)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicFootPerTonShort` (ft³/2000 lb), as a `SpecificVolume`.
    pub const CUBIC_FOOT_PER_TON_SHORT<N: const From<Exact>>: SpecificVolume<N> = counted(((Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4)) / Exact::dec(90718474, -5)) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `WattPerKilogram` (W/kg), as a `SpecificPower`.
    pub const WATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(METRE::<Exact> * METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `SquareMetrePerCubicSecond` (m² · s⁻³), as a `SpecificPower`.
    pub const SQUARE_METRE_PER_CUBIC_SECOND<N: const From<Exact>>: SpecificPower<N> = counted(WATT_PER_KILOGRAM::<Exact>);

    /// One `ErgPerSecondPerKilogram` ((erg/s)/kg), as a `SpecificPower`.
    pub const ERG_PER_SECOND_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -7) * WATT_PER_KILOGRAM::<Exact>);

    /// One `FootPoundPerHourPerPound` ((ft · lbf/h)/lb), as a `SpecificPower`.
    pub const FOOT_POUND_PER_HOUR_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted(((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::int(3600) * Exact::dec(45359237, -8))) * WATT_PER_KILOGRAM::<Exact>);

    /// One `FootPoundPerMinutePerPound` ((ft · lbf/min)/lb), as a `SpecificPower`.
    pub const FOOT_POUND_PER_MINUTE_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted(((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / (Exact::int(60) * Exact::dec(45359237, -8))) * WATT_PER_KILOGRAM::<Exact>);

    /// One `FootPoundPerSecondPerPound` ((ft · lbf/s)/lb), as a `SpecificPower`.
    pub const FOOT_POUND_PER_SECOND_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted(((Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8)) * WATT_PER_KILOGRAM::<Exact>);

    /// One `HorsepowerPerPound` (hp/lb), as a `SpecificPower`.
    pub const HORSEPOWER_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted(((Exact::int(550) * Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8)) * WATT_PER_KILOGRAM::<Exact>);

    /// One `HorsepowerBoilerPerPound` ((hp (S))/lb), as a `SpecificPower`.
    pub const HORSEPOWER_BOILER_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted((Exact::dec(98095, -1) / Exact::dec(45359237, -8)) * WATT_PER_KILOGRAM::<Exact>);

    /// One `HorsepowerElectricPerPound` ((hp (E))/lb), as a `SpecificPower`.
    pub const HORSEPOWER_ELECTRIC_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted((Exact::int(746) / Exact::dec(45359237, -8)) * WATT_PER_KILOGRAM::<Exact>);

    /// One `HorsepowerMetricPerPound` ((hp (M))/lb), as a `SpecificPower`.
    pub const HORSEPOWER_METRIC_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted(((Exact::int(75) * Exact::dec(980665, -5)) / Exact::dec(45359237, -8)) * WATT_PER_KILOGRAM::<Exact>);

    /// One `HorsepowerImperialPerPound` ((hp (I))/lb), as a `SpecificPower`.
    pub const HORSEPOWER_IMPERIAL_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted((Exact::dec(7457, -1) / Exact::dec(45359237, -8)) * WATT_PER_KILOGRAM::<Exact>);

    /// One `HydraulicHorsepowerPerPound` ((hp (hydraulic))/lb), as a `SpecificPower`.
    pub const HYDRAULIC_HORSEPOWER_PER_POUND<N: const From<Exact>>: SpecificPower<N> = counted((Exact::dec(746043, -3) / Exact::dec(45359237, -8)) * WATT_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerKilogramSecondSquared` (m³/(kg · s²)), as a `SpecificGravitationalParameter`.
    pub const CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED<N: const From<Exact>>: SpecificGravitationalParameter<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> / (KILOGRAM::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `GravitationalConstant` (G), as a `SpecificGravitationalParameter`.
    pub const GRAVITATIONAL_CONSTANT<N: const From<Exact>>: SpecificGravitationalParameter<N> = counted(Exact::dec(66743, -15) * CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED::<Exact>);
}

/// The units of `Length`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<f64>)]
pub enum LengthUnit {
    #[default]
    #[unit("m", "metres", by = METRE)]
    Metre,
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
#[unit(of = Mass<f64>)]
pub enum MassUnit {
    #[default]
    #[unit("kg", "kilograms", by = KILOGRAM)]
    Kilogram,
    #[unit("g", "grams", by = GRAM)]
    Gram,
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
#[unit(of = Time<f64>)]
pub enum TimeUnit {
    #[default]
    #[unit("s", "seconds", by = SECOND)]
    Second,
    #[unit("ms", "milliseconds", by = MILLISECOND)]
    Millisecond,
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
#[unit(of = Instant<f64>)]
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
#[unit(of = Area<f64>)]
pub enum AreaUnit {
    #[default]
    #[unit("m²", "square metres", by = SQUARE_METRE)]
    SquareMetre,
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
#[unit(of = Volume<f64>)]
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
    #[unit("kL", "kilolitres", by = KILOLITRE)]
    Kilolitre,
    #[unit("L", "litres", by = LITRE)]
    Litre,
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
#[unit(of = Velocity<f64>)]
pub enum VelocityUnit {
    #[default]
    #[unit("m/s", "metres per second", by = METRE_PER_SECOND)]
    MetrePerSecond,
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
#[unit(of = Acceleration<f64>)]
pub enum AccelerationUnit {
    #[default]
    #[unit("m/s²", "metres per second squared", by = METRE_PER_SECOND_SQUARED)]
    MetrePerSecondSquared,
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
#[unit(of = Jerk<f64>)]
pub enum JerkUnit {
    #[default]
    #[unit("m/s³", "metres per second cubed", by = METRE_PER_SECOND_CUBED)]
    MetrePerSecondCubed,
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
#[unit(of = Absement<f64>)]
pub enum AbsementUnit {
    #[default]
    #[unit("m · s", "metre seconds", by = METRE_SECOND)]
    MetreSecond,
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
#[unit(of = InverseVelocity<f64>)]
pub enum InverseVelocityUnit {
    #[default]
    #[unit("s/m", "seconds per metre", by = SECOND_PER_METRE)]
    SecondPerMetre,
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
#[unit(of = Force<f64>)]
pub enum ForceUnit {
    #[default]
    #[unit("N", "newtons", by = NEWTON)]
    Newton,
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
#[unit(of = Momentum<f64>)]
pub enum MomentumUnit {
    #[default]
    #[unit("kg · m/s", "kilogram metres per second", by = KILOGRAM_METRE_PER_SECOND)]
    KilogramMetrePerSecond,
    #[unit("g · m/s", "gram metres per second", by = GRAM_METRE_PER_SECOND)]
    GramMetrePerSecond,
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
#[unit(of = Energy<f64>)]
pub enum EnergyUnit {
    #[default]
    #[unit("J", "joules", by = JOULE)]
    Joule,
    #[unit("W · h", "watt hours", by = WATT_HOUR)]
    WattHour,
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
#[unit(of = AvailableEnergy<f64>)]
pub enum AvailableEnergyUnit {
    #[default]
    #[unit("J/kg", "joules per kilogram", by = JOULE_PER_KILOGRAM)]
    JoulePerKilogram,
    #[unit("J/g", "joules per gram", by = JOULE_PER_GRAM)]
    JoulePerGram,
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
#[unit(of = Power<f64>, non_const)]
pub enum PowerUnit {
    #[default]
    #[unit("W", "watts", by = WATT)]
    Watt,
    #[unit("dBW", "decibels-watt", by = DBW, via = PowerLevel<f64>)]
    DecibelWatt,
    #[unit("dBm", "decibels-milliwatt", by = DBM, via = PowerLevel<f64>)]
    DecibelMilliwatt,
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
#[unit(of = Gain<f64>)]
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
#[unit(of = PowerLevel<f64>)]
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
#[unit(of = PowerRate<f64>)]
pub enum PowerRateUnit {
    #[default]
    #[unit("W/s", "watts per second", by = WATT_PER_SECOND)]
    WattPerSecond,
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
#[unit(of = Action<f64>)]
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
#[unit(of = Torque<f64>)]
pub enum TorqueUnit {
    #[default]
    #[unit("N · m", "newton metres", by = NEWTON_METRE)]
    NewtonMetre,
    #[unit("dyn · m", "dyne metres", by = DYNE_METRE)]
    DyneMetre,
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
#[unit(of = MomentOfInertia<f64>)]
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
#[unit(of = Pressure<f64>)]
pub enum PressureUnit {
    #[default]
    #[unit("Pa", "pascals", by = PASCAL)]
    Pascal,
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
    #[unit("mm Hg", "millimetres of mercury", by = MILLIMETRE_OF_MERCURY)]
    MillimetreOfMercury,
    #[unit("mm H₂O", "millimetres of water", by = MILLIMETRE_OF_WATER)]
    MillimetreOfWater,
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
#[unit(of = Frequency<f64>)]
pub enum FrequencyUnit {
    #[default]
    #[unit("Hz", "hertz", by = HERTZ)]
    Hertz,
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
#[unit(of = FrequencyDrift<f64>)]
pub enum FrequencyDriftUnit {
    #[default]
    #[unit("Hz/s", "hertz per second", by = HERTZ_PER_SECOND)]
    HertzPerSecond,
}

/// The units of `ReciprocalLength`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ReciprocalLength<f64>)]
pub enum ReciprocalLengthUnit {
    #[default]
    #[unit("m⁻¹", "reciprocal metres", by = RECIPROCAL_METRE)]
    ReciprocalMetre,
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
#[unit(of = Curvature<f64>)]
pub enum CurvatureUnit {
    #[default]
    #[unit("rad/m", "radians per metre", by = RADIAN_PER_METRE)]
    RadianPerMetre,
    #[unit("°/m", "degrees per metre", by = DEGREE_PER_METRE)]
    DegreePerMetre,
}

/// The units of `SpecificArea`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificArea<f64>)]
pub enum SpecificAreaUnit {
    #[default]
    #[unit("m²/kg", "square metres per kilogram", by = SQUARE_METRE_PER_KILOGRAM)]
    SquareMetrePerKilogram,
    #[unit("m²/g", "square metres per gram", by = SQUARE_METRE_PER_GRAM)]
    SquareMetrePerGram,
}

/// The units of `SpecificVolume`, for a menu, a config file, a parser.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SpecificVolume<f64>)]
pub enum SpecificVolumeUnit {
    #[default]
    #[unit("m³/kg", "cubic metres per kilogram", by = CUBIC_METRE_PER_KILOGRAM)]
    CubicMetrePerKilogram,
    #[unit("m³/g", "cubic metres per gram", by = CUBIC_METRE_PER_GRAM)]
    CubicMetrePerGram,
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
#[unit(of = SpecificPower<f64>)]
pub enum SpecificPowerUnit {
    #[default]
    #[unit("W/kg", "watts per kilogram", by = WATT_PER_KILOGRAM)]
    WattPerKilogram,
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
#[unit(of = SpecificGravitationalParameter<f64>)]
pub enum SpecificGravitationalParameterUnit {
    #[default]
    #[unit("m³/(kg · s²)", "cubic metres per kilogram second squared", by = CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED)]
    CubicMetrePerKilogramSecondSquared,
    #[unit("G", "Newtonian constants of gravitation", by = GRAVITATIONAL_CONSTANT)]
    GravitationalConstant,
}
