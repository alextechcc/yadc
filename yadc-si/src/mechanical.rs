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
/// `KILOMETRE` is `1000 * METRE` - and counted in the number type its use
/// asks for, rounded once: `let d: Length = KILOMETRE;` is an `f64` length,
/// `Length<f32>` an `f32` one, `KILOMETRE::<Exact>` the exact value. Each is
/// a unit (`KILOMETRE(3.0)`, `KILOMETRE.value(d)`), and they compose in
/// `const` items: `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.
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
    pub const YOTTAMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(1000000000000000000000000) * METRE::<Exact>);

    /// One `Zettametre` (Zm), as a `Length`.
    pub const ZETTAMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(1000000000000000000000) * METRE::<Exact>);

    /// One `Exametre` (Em), as a `Length`.
    pub const EXAMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(1000000000000000000) * METRE::<Exact>);

    /// One `Petametre` (Pm), as a `Length`.
    pub const PETAMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(1000000000000000) * METRE::<Exact>);

    /// One `Terametre` (Tm), as a `Length`.
    pub const TERAMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(1000000000000) * METRE::<Exact>);

    /// One `Gigametre` (Gm), as a `Length`.
    pub const GIGAMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(1000000000) * METRE::<Exact>);

    /// One `Megametre` (Mm), as a `Length`.
    pub const MEGAMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(1000000) * METRE::<Exact>);

    /// One `Kilometre` (km), as a `Length`.
    pub const KILOMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(1000) * METRE::<Exact>);

    /// One `Hectometre` (hm), as a `Length`.
    pub const HECTOMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(100) * METRE::<Exact>);

    /// One `Decametre` (dam), as a `Length`.
    pub const DECAMETRE<N: const From<Exact>>: Length<N> = counted(Exact::int(10) * METRE::<Exact>);

    /// One `Decimetre` (dm), as a `Length`.
    pub const DECIMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -1) * METRE::<Exact>);

    /// One `Centimetre` (cm), as a `Length`.
    pub const CENTIMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -2) * METRE::<Exact>);

    /// One `Millimetre` (mm), as a `Length`.
    pub const MILLIMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -3) * METRE::<Exact>);

    /// One `Micrometre` (µm), as a `Length`.
    pub const MICROMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -6) * METRE::<Exact>);

    /// One `Nanometre` (nm), as a `Length`.
    pub const NANOMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -9) * METRE::<Exact>);

    /// One `Picometre` (pm), as a `Length`.
    pub const PICOMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -12) * METRE::<Exact>);

    /// One `Femtometre` (fm), as a `Length`.
    pub const FEMTOMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -15) * METRE::<Exact>);

    /// One `Attometre` (am), as a `Length`.
    pub const ATTOMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -18) * METRE::<Exact>);

    /// One `Zeptometre` (zm), as a `Length`.
    pub const ZEPTOMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -21) * METRE::<Exact>);

    /// One `Yoctometre` (ym), as a `Length`.
    pub const YOCTOMETRE<N: const From<Exact>>: Length<N> = counted(Exact::dec(1, -24) * METRE::<Exact>);

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

    /// One `Microinch` (μin), as a `Length`.
    pub const MICROINCH<N: const From<Exact>>: Length<N> = counted(Exact::dec(254, -10) * METRE::<Exact>);

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

    /// One `Yottagram` (Yg), as a `Mass`.
    pub const YOTTAGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM::<Exact>);

    /// One `Zettagram` (Zg), as a `Mass`.
    pub const ZETTAGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::int(1000000000000000000) * KILOGRAM::<Exact>);

    /// One `Exagram` (Eg), as a `Mass`.
    pub const EXAGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::int(1000000000000000) * KILOGRAM::<Exact>);

    /// One `Petagram` (Pg), as a `Mass`.
    pub const PETAGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::int(1000000000000) * KILOGRAM::<Exact>);

    /// One `Teragram` (Tg), as a `Mass`.
    pub const TERAGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::int(1000000000) * KILOGRAM::<Exact>);

    /// One `Gigagram` (Gg), as a `Mass`.
    pub const GIGAGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::int(1000000) * KILOGRAM::<Exact>);

    /// One `Megagram` (Mg), as a `Mass`.
    pub const MEGAGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::int(1000) * KILOGRAM::<Exact>);

    /// One `Hectogram` (hg), as a `Mass`.
    pub const HECTOGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -1) * KILOGRAM::<Exact>);

    /// One `Decagram` (dag), as a `Mass`.
    pub const DECAGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -2) * KILOGRAM::<Exact>);

    /// One `Gram` (g), as a `Mass`.
    pub const GRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -3) * KILOGRAM::<Exact>);

    /// One `Decigram` (dg), as a `Mass`.
    pub const DECIGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -4) * KILOGRAM::<Exact>);

    /// One `Centigram` (cg), as a `Mass`.
    pub const CENTIGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -5) * KILOGRAM::<Exact>);

    /// One `Milligram` (mg), as a `Mass`.
    pub const MILLIGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -6) * KILOGRAM::<Exact>);

    /// One `Microgram` (µg), as a `Mass`.
    pub const MICROGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -9) * KILOGRAM::<Exact>);

    /// One `Nanogram` (ng), as a `Mass`.
    pub const NANOGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -12) * KILOGRAM::<Exact>);

    /// One `Picogram` (pg), as a `Mass`.
    pub const PICOGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -15) * KILOGRAM::<Exact>);

    /// One `Femtogram` (fg), as a `Mass`.
    pub const FEMTOGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -18) * KILOGRAM::<Exact>);

    /// One `Attogram` (ag), as a `Mass`.
    pub const ATTOGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -21) * KILOGRAM::<Exact>);

    /// One `Zeptogram` (zg), as a `Mass`.
    pub const ZEPTOGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -24) * KILOGRAM::<Exact>);

    /// One `Yoctogram` (yg), as a `Mass`.
    pub const YOCTOGRAM<N: const From<Exact>>: Mass<N> = counted(Exact::dec(1, -27) * KILOGRAM::<Exact>);

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

    /// One `Yottasecond` (Ys), as a `Time`.
    pub const YOTTASECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(1000000000000000000000000) * SECOND::<Exact>);

    /// One `Zettasecond` (Zs), as a `Time`.
    pub const ZETTASECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(1000000000000000000000) * SECOND::<Exact>);

    /// One `Exasecond` (Es), as a `Time`.
    pub const EXASECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(1000000000000000000) * SECOND::<Exact>);

    /// One `Petasecond` (Ps), as a `Time`.
    pub const PETASECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(1000000000000000) * SECOND::<Exact>);

    /// One `Terasecond` (Ts), as a `Time`.
    pub const TERASECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(1000000000000) * SECOND::<Exact>);

    /// One `Gigasecond` (Gs), as a `Time`.
    pub const GIGASECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(1000000000) * SECOND::<Exact>);

    /// One `Megasecond` (Ms), as a `Time`.
    pub const MEGASECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(1000000) * SECOND::<Exact>);

    /// One `Kilosecond` (ks), as a `Time`.
    pub const KILOSECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(1000) * SECOND::<Exact>);

    /// One `Hectosecond` (hs), as a `Time`.
    pub const HECTOSECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(100) * SECOND::<Exact>);

    /// One `Decasecond` (das), as a `Time`.
    pub const DECASECOND<N: const From<Exact>>: Time<N> = counted(Exact::int(10) * SECOND::<Exact>);

    /// One `Decisecond` (ds), as a `Time`.
    pub const DECISECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -1) * SECOND::<Exact>);

    /// One `Centisecond` (cs), as a `Time`.
    pub const CENTISECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -2) * SECOND::<Exact>);

    /// One `Millisecond` (ms), as a `Time`.
    pub const MILLISECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -3) * SECOND::<Exact>);

    /// One `Microsecond` (µs), as a `Time`.
    pub const MICROSECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -6) * SECOND::<Exact>);

    /// One `Nanosecond` (ns), as a `Time`.
    pub const NANOSECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -9) * SECOND::<Exact>);

    /// One `Picosecond` (ps), as a `Time`.
    pub const PICOSECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -12) * SECOND::<Exact>);

    /// One `Femtosecond` (fs), as a `Time`.
    pub const FEMTOSECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -15) * SECOND::<Exact>);

    /// One `Attosecond` (as), as a `Time`.
    pub const ATTOSECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -18) * SECOND::<Exact>);

    /// One `Zeptosecond` (zs), as a `Time`.
    pub const ZEPTOSECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -21) * SECOND::<Exact>);

    /// One `Yoctosecond` (ys), as a `Time`.
    pub const YOCTOSECOND<N: const From<Exact>>: Time<N> = counted(Exact::dec(1, -24) * SECOND::<Exact>);

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
    pub const UNIX_SECONDS<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(counted(UNIX_EPOCH::<Exact>), counted(SECOND::<Exact>));

    /// Unix time in milliseconds.
    pub const UNIX_MILLISECONDS<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(counted(UNIX_EPOCH::<Exact>), counted(MILLISECOND::<Exact>));

    /// GPS time: seconds from the GPS epoch.
    pub const GPS_SECONDS<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(counted(GPS_EPOCH::<Exact>), counted(SECOND::<Exact>));

    /// The Julian date (JD): days from the Julian epoch.
    pub const JULIAN_DATE<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(counted(JULIAN_EPOCH::<Exact>), counted(DAY::<Exact>));

    /// The modified Julian date (MJD): days from the modified Julian epoch.
    pub const MODIFIED_JULIAN_DATE<N: const From<Exact>>: Scale<Instant<N>, Time<N>> = Scale(counted(MODIFIED_JULIAN_EPOCH::<Exact>), counted(DAY::<Exact>));

    /// One `SquareYottametre` (Ym²), as an `Area`.
    pub const SQUARE_YOTTAMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, 48) * SQUARE_METRE::<Exact>);

    /// One `SquareZettametre` (Zm²), as an `Area`.
    pub const SQUARE_ZETTAMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, 42) * SQUARE_METRE::<Exact>);

    /// One `SquareExametre` (Em²), as an `Area`.
    pub const SQUARE_EXAMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, 36) * SQUARE_METRE::<Exact>);

    /// One `SquarePetametre` (Pm²), as an `Area`.
    pub const SQUARE_PETAMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, 30) * SQUARE_METRE::<Exact>);

    /// One `SquareTerametre` (Tm²), as an `Area`.
    pub const SQUARE_TERAMETRE<N: const From<Exact>>: Area<N> = counted(Exact::int(1000000000000000000000000) * SQUARE_METRE::<Exact>);

    /// One `SquareGigametre` (Gm²), as an `Area`.
    pub const SQUARE_GIGAMETRE<N: const From<Exact>>: Area<N> = counted(Exact::int(1000000000000000000) * SQUARE_METRE::<Exact>);

    /// One `SquareMegametre` (Mm²), as an `Area`.
    pub const SQUARE_MEGAMETRE<N: const From<Exact>>: Area<N> = counted(Exact::int(1000000000000) * SQUARE_METRE::<Exact>);

    /// One `SquareKilometre` (km²), as an `Area`.
    pub const SQUARE_KILOMETRE<N: const From<Exact>>: Area<N> = counted(Exact::int(1000000) * SQUARE_METRE::<Exact>);

    /// One `SquareHectometre` (hm²), as an `Area`.
    pub const SQUARE_HECTOMETRE<N: const From<Exact>>: Area<N> = counted(Exact::int(10000) * SQUARE_METRE::<Exact>);

    /// One `SquareDecametre` (dam²), as an `Area`.
    pub const SQUARE_DECAMETRE<N: const From<Exact>>: Area<N> = counted(Exact::int(100) * SQUARE_METRE::<Exact>);

    /// One `SquareMetre` (m²), as an `Area`.
    pub const SQUARE_METRE<N: const From<Exact>>: Area<N> = counted(METRE::<Exact> * METRE::<Exact>);

    /// One `SquareDecimetre` (dm²), as an `Area`.
    pub const SQUARE_DECIMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -2) * SQUARE_METRE::<Exact>);

    /// One `SquareCentimetre` (cm²), as an `Area`.
    pub const SQUARE_CENTIMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -4) * SQUARE_METRE::<Exact>);

    /// One `SquareMillimetre` (mm²), as an `Area`.
    pub const SQUARE_MILLIMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -6) * SQUARE_METRE::<Exact>);

    /// One `SquareMicrometre` (µm²), as an `Area`.
    pub const SQUARE_MICROMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -12) * SQUARE_METRE::<Exact>);

    /// One `SquareNanometre` (nm²), as an `Area`.
    pub const SQUARE_NANOMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -18) * SQUARE_METRE::<Exact>);

    /// One `SquarePicometre` (pm²), as an `Area`.
    pub const SQUARE_PICOMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -24) * SQUARE_METRE::<Exact>);

    /// One `SquareFemtometre` (fm²), as an `Area`.
    pub const SQUARE_FEMTOMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -30) * SQUARE_METRE::<Exact>);

    /// One `SquareAttometre` (am²), as an `Area`.
    pub const SQUARE_ATTOMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -36) * SQUARE_METRE::<Exact>);

    /// One `SquareZeptometre` (zm²), as an `Area`.
    pub const SQUARE_ZEPTOMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -42) * SQUARE_METRE::<Exact>);

    /// One `SquareYoctometre` (ym²), as an `Area`.
    pub const SQUARE_YOCTOMETRE<N: const From<Exact>>: Area<N> = counted(Exact::dec(1, -48) * SQUARE_METRE::<Exact>);

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

    /// One `Yottalitre` (YL), as a `Volume`.
    pub const YOTTALITRE<N: const From<Exact>>: Volume<N> = counted(Exact::int(1000000000000000000000) * CUBIC_METRE::<Exact>);

    /// One `Zettalitre` (ZL), as a `Volume`.
    pub const ZETTALITRE<N: const From<Exact>>: Volume<N> = counted(Exact::int(1000000000000000000) * CUBIC_METRE::<Exact>);

    /// One `Exalitre` (EL), as a `Volume`.
    pub const EXALITRE<N: const From<Exact>>: Volume<N> = counted(Exact::int(1000000000000000) * CUBIC_METRE::<Exact>);

    /// One `Petalitre` (PL), as a `Volume`.
    pub const PETALITRE<N: const From<Exact>>: Volume<N> = counted(Exact::int(1000000000000) * CUBIC_METRE::<Exact>);

    /// One `Teralitre` (TL), as a `Volume`.
    pub const TERALITRE<N: const From<Exact>>: Volume<N> = counted(Exact::int(1000000000) * CUBIC_METRE::<Exact>);

    /// One `Gigalitre` (GL), as a `Volume`.
    pub const GIGALITRE<N: const From<Exact>>: Volume<N> = counted(Exact::int(1000000) * CUBIC_METRE::<Exact>);

    /// One `Megalitre` (ML), as a `Volume`.
    pub const MEGALITRE<N: const From<Exact>>: Volume<N> = counted(Exact::int(1000) * CUBIC_METRE::<Exact>);

    /// One `Kilolitre` (kL), as a `Volume`.
    pub const KILOLITRE<N: const From<Exact>>: Volume<N> = counted(CUBIC_METRE::<Exact>);

    /// One `Hectolitre` (hL), as a `Volume`.
    pub const HECTOLITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -1) * CUBIC_METRE::<Exact>);

    /// One `Decalitre` (daL), as a `Volume`.
    pub const DECALITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -2) * CUBIC_METRE::<Exact>);

    /// One `Litre` (L), as a `Volume`.
    pub const LITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -3) * CUBIC_METRE::<Exact>);

    /// One `Decilitre` (dL), as a `Volume`.
    pub const DECILITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -4) * CUBIC_METRE::<Exact>);

    /// One `Centilitre` (cL), as a `Volume`.
    pub const CENTILITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -5) * CUBIC_METRE::<Exact>);

    /// One `Millilitre` (mL), as a `Volume`.
    pub const MILLILITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -6) * CUBIC_METRE::<Exact>);

    /// One `Microlitre` (µL), as a `Volume`.
    pub const MICROLITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -9) * CUBIC_METRE::<Exact>);

    /// One `Nanolitre` (nL), as a `Volume`.
    pub const NANOLITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -12) * CUBIC_METRE::<Exact>);

    /// One `Picolitre` (pL), as a `Volume`.
    pub const PICOLITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -15) * CUBIC_METRE::<Exact>);

    /// One `Femtolitre` (fL), as a `Volume`.
    pub const FEMTOLITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -18) * CUBIC_METRE::<Exact>);

    /// One `Attolitre` (aL), as a `Volume`.
    pub const ATTOLITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -21) * CUBIC_METRE::<Exact>);

    /// One `Zeptolitre` (zL), as a `Volume`.
    pub const ZEPTOLITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -24) * CUBIC_METRE::<Exact>);

    /// One `Yoctolitre` (yL), as a `Volume`.
    pub const YOCTOLITRE<N: const From<Exact>>: Volume<N> = counted(Exact::dec(1, -27) * CUBIC_METRE::<Exact>);

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

    /// One `YottametrePerSecond` (Ym/s), as a `Velocity`.
    pub const YOTTAMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(1000000000000000000000000) * METRE_PER_SECOND::<Exact>);

    /// One `ZettametrePerSecond` (Zm/s), as a `Velocity`.
    pub const ZETTAMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(1000000000000000000000) * METRE_PER_SECOND::<Exact>);

    /// One `ExametrePerSecond` (Em/s), as a `Velocity`.
    pub const EXAMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(1000000000000000000) * METRE_PER_SECOND::<Exact>);

    /// One `PetametrePerSecond` (Pm/s), as a `Velocity`.
    pub const PETAMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(1000000000000000) * METRE_PER_SECOND::<Exact>);

    /// One `TerametrePerSecond` (Tm/s), as a `Velocity`.
    pub const TERAMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(1000000000000) * METRE_PER_SECOND::<Exact>);

    /// One `GigametrePerSecond` (Gm/s), as a `Velocity`.
    pub const GIGAMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(1000000000) * METRE_PER_SECOND::<Exact>);

    /// One `MegametrePerSecond` (Mm/s), as a `Velocity`.
    pub const MEGAMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(1000000) * METRE_PER_SECOND::<Exact>);

    /// One `KilometrePerSecond` (km/s), as a `Velocity`.
    pub const KILOMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(1000) * METRE_PER_SECOND::<Exact>);

    /// One `HectometrePerSecond` (hm/s), as a `Velocity`.
    pub const HECTOMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(100) * METRE_PER_SECOND::<Exact>);

    /// One `DecametrePerSecond` (dam/s), as a `Velocity`.
    pub const DECAMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::int(10) * METRE_PER_SECOND::<Exact>);

    /// One `MetrePerSecond` (m/s), as a `Velocity`.
    pub const METRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(METRE::<Exact> / SECOND::<Exact>);

    /// One `DecimetrePerSecond` (dm/s), as a `Velocity`.
    pub const DECIMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -1) * METRE_PER_SECOND::<Exact>);

    /// One `CentimetrePerSecond` (cm/s), as a `Velocity`.
    pub const CENTIMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -2) * METRE_PER_SECOND::<Exact>);

    /// One `MillimetrePerSecond` (mm/s), as a `Velocity`.
    pub const MILLIMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -3) * METRE_PER_SECOND::<Exact>);

    /// One `MicrometrePerSecond` (µm/s), as a `Velocity`.
    pub const MICROMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -6) * METRE_PER_SECOND::<Exact>);

    /// One `NanometrePerSecond` (nm/s), as a `Velocity`.
    pub const NANOMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -9) * METRE_PER_SECOND::<Exact>);

    /// One `PicometrePerSecond` (pm/s), as a `Velocity`.
    pub const PICOMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -12) * METRE_PER_SECOND::<Exact>);

    /// One `FemtometrePerSecond` (fm/s), as a `Velocity`.
    pub const FEMTOMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -15) * METRE_PER_SECOND::<Exact>);

    /// One `AttometrePerSecond` (am/s), as a `Velocity`.
    pub const ATTOMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -18) * METRE_PER_SECOND::<Exact>);

    /// One `ZeptometrePerSecond` (zm/s), as a `Velocity`.
    pub const ZEPTOMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -21) * METRE_PER_SECOND::<Exact>);

    /// One `YoctometrePerSecond` (ym/s), as a `Velocity`.
    pub const YOCTOMETRE_PER_SECOND<N: const From<Exact>>: Velocity<N> = counted(Exact::dec(1, -24) * METRE_PER_SECOND::<Exact>);

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

    /// One `YottametrePerSecondSquared` (Ym/s²), as an `Acceleration`.
    pub const YOTTAMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(1000000000000000000000000) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `ZettametrePerSecondSquared` (Zm/s²), as an `Acceleration`.
    pub const ZETTAMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(1000000000000000000000) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `ExametrePerSecondSquared` (Em/s²), as an `Acceleration`.
    pub const EXAMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(1000000000000000000) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `PetametrePerSecondSquared` (Pm/s²), as an `Acceleration`.
    pub const PETAMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(1000000000000000) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `TerametrePerSecondSquared` (Tm/s²), as an `Acceleration`.
    pub const TERAMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(1000000000000) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `GigametrePerSecondSquared` (Gm/s²), as an `Acceleration`.
    pub const GIGAMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(1000000000) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `MegametrePerSecondSquared` (Mm/s²), as an `Acceleration`.
    pub const MEGAMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(1000000) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `KilometrePerSecondSquared` (km/s²), as an `Acceleration`.
    pub const KILOMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(1000) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `HectometrePerSecondSquared` (hm/s²), as an `Acceleration`.
    pub const HECTOMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(100) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `DecametrePerSecondSquared` (dam/s²), as an `Acceleration`.
    pub const DECAMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::int(10) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `MetrePerSecondSquared` (m/s²), as an `Acceleration`.
    pub const METRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `DecimetrePerSecondSquared` (dm/s²), as an `Acceleration`.
    pub const DECIMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -1) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `CentimetrePerSecondSquared` (cm/s²), as an `Acceleration`.
    pub const CENTIMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -2) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `MillimetrePerSecondSquared` (mm/s²), as an `Acceleration`.
    pub const MILLIMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -3) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `MicrometrePerSecondSquared` (µm/s²), as an `Acceleration`.
    pub const MICROMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -6) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `NanometrePerSecondSquared` (nm/s²), as an `Acceleration`.
    pub const NANOMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -9) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `PicometrePerSecondSquared` (pm/s²), as an `Acceleration`.
    pub const PICOMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -12) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `FemtometrePerSecondSquared` (fm/s²), as an `Acceleration`.
    pub const FEMTOMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -15) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `AttometrePerSecondSquared` (am/s²), as an `Acceleration`.
    pub const ATTOMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -18) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `ZeptometrePerSecondSquared` (zm/s²), as an `Acceleration`.
    pub const ZEPTOMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -21) * METRE_PER_SECOND_SQUARED::<Exact>);

    /// One `YoctometrePerSecondSquared` (ym/s²), as an `Acceleration`.
    pub const YOCTOMETRE_PER_SECOND_SQUARED<N: const From<Exact>>: Acceleration<N> = counted(Exact::dec(1, -24) * METRE_PER_SECOND_SQUARED::<Exact>);

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

    /// One `YottametrePerSecondCubed` (Ym/s³), as a `Jerk`.
    pub const YOTTAMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(1000000000000000000000000) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `ZettametrePerSecondCubed` (Zm/s³), as a `Jerk`.
    pub const ZETTAMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(1000000000000000000000) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `ExametrePerSecondCubed` (Em/s³), as a `Jerk`.
    pub const EXAMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(1000000000000000000) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `PetametrePerSecondCubed` (Pm/s³), as a `Jerk`.
    pub const PETAMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(1000000000000000) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `TerametrePerSecondCubed` (Tm/s³), as a `Jerk`.
    pub const TERAMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(1000000000000) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `GigametrePerSecondCubed` (Gm/s³), as a `Jerk`.
    pub const GIGAMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(1000000000) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `MegametrePerSecondCubed` (Mm/s³), as a `Jerk`.
    pub const MEGAMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(1000000) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `KilometrePerSecondCubed` (km/s³), as a `Jerk`.
    pub const KILOMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(1000) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `HectometrePerSecondCubed` (hm/s³), as a `Jerk`.
    pub const HECTOMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(100) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `DecametrePerSecondCubed` (dam/s³), as a `Jerk`.
    pub const DECAMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::int(10) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `MetrePerSecondCubed` (m/s³), as a `Jerk`.
    pub const METRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `DecimetrePerSecondCubed` (dm/s³), as a `Jerk`.
    pub const DECIMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -1) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `CentimetrePerSecondCubed` (cm/s³), as a `Jerk`.
    pub const CENTIMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -2) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `MillimetrePerSecondCubed` (mm/s³), as a `Jerk`.
    pub const MILLIMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -3) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `MicrometrePerSecondCubed` (µm/s³), as a `Jerk`.
    pub const MICROMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -6) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `NanometrePerSecondCubed` (nm/s³), as a `Jerk`.
    pub const NANOMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -9) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `PicometrePerSecondCubed` (pm/s³), as a `Jerk`.
    pub const PICOMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -12) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `FemtometrePerSecondCubed` (fm/s³), as a `Jerk`.
    pub const FEMTOMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -15) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `AttometrePerSecondCubed` (am/s³), as a `Jerk`.
    pub const ATTOMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -18) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `ZeptometrePerSecondCubed` (zm/s³), as a `Jerk`.
    pub const ZEPTOMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -21) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `YoctometrePerSecondCubed` (ym/s³), as a `Jerk`.
    pub const YOCTOMETRE_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(1, -24) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `FootPerSecondCubed` (ft/s³), as a `Jerk`.
    pub const FOOT_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(3048, -4) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `InchPerSecondCubed` (in/s³), as a `Jerk`.
    pub const INCH_PER_SECOND_CUBED<N: const From<Exact>>: Jerk<N> = counted(Exact::dec(254, -4) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `KilometrePerMinuteCubed` (km/min³), as a `Jerk`.
    pub const KILOMETRE_PER_MINUTE_CUBED<N: const From<Exact>>: Jerk<N> = counted((Exact::int(1000) / (Exact::int(60) * Exact::int(60) * Exact::int(60))) * METRE_PER_SECOND_CUBED::<Exact>);

    /// One `YottametreSecond` (Ym · s), as an `Absement`.
    pub const YOTTAMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(1000000000000000000000000) * METRE_SECOND::<Exact>);

    /// One `ZettametreSecond` (Zm · s), as an `Absement`.
    pub const ZETTAMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(1000000000000000000000) * METRE_SECOND::<Exact>);

    /// One `ExametreSecond` (Em · s), as an `Absement`.
    pub const EXAMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(1000000000000000000) * METRE_SECOND::<Exact>);

    /// One `PetametreSecond` (Pm · s), as an `Absement`.
    pub const PETAMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(1000000000000000) * METRE_SECOND::<Exact>);

    /// One `TerametreSecond` (Tm · s), as an `Absement`.
    pub const TERAMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(1000000000000) * METRE_SECOND::<Exact>);

    /// One `GigametreSecond` (Gm · s), as an `Absement`.
    pub const GIGAMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(1000000000) * METRE_SECOND::<Exact>);

    /// One `MegametreSecond` (Mm · s), as an `Absement`.
    pub const MEGAMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(1000000) * METRE_SECOND::<Exact>);

    /// One `KilometreSecond` (km · s), as an `Absement`.
    pub const KILOMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(1000) * METRE_SECOND::<Exact>);

    /// One `HectometreSecond` (hm · s), as an `Absement`.
    pub const HECTOMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(100) * METRE_SECOND::<Exact>);

    /// One `DecametreSecond` (dam · s), as an `Absement`.
    pub const DECAMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::int(10) * METRE_SECOND::<Exact>);

    /// One `MetreSecond` (m · s), as an `Absement`.
    pub const METRE_SECOND<N: const From<Exact>>: Absement<N> = counted(METRE::<Exact> * SECOND::<Exact>);

    /// One `DecimetreSecond` (dm · s), as an `Absement`.
    pub const DECIMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -1) * METRE_SECOND::<Exact>);

    /// One `CentimetreSecond` (cm · s), as an `Absement`.
    pub const CENTIMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -2) * METRE_SECOND::<Exact>);

    /// One `MillimetreSecond` (mm · s), as an `Absement`.
    pub const MILLIMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -3) * METRE_SECOND::<Exact>);

    /// One `MicrometreSecond` (µm · s), as an `Absement`.
    pub const MICROMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -6) * METRE_SECOND::<Exact>);

    /// One `NanometreSecond` (nm · s), as an `Absement`.
    pub const NANOMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -9) * METRE_SECOND::<Exact>);

    /// One `PicometreSecond` (pm · s), as an `Absement`.
    pub const PICOMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -12) * METRE_SECOND::<Exact>);

    /// One `FemtometreSecond` (fm · s), as an `Absement`.
    pub const FEMTOMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -15) * METRE_SECOND::<Exact>);

    /// One `AttometreSecond` (am · s), as an `Absement`.
    pub const ATTOMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -18) * METRE_SECOND::<Exact>);

    /// One `ZeptometreSecond` (zm · s), as an `Absement`.
    pub const ZEPTOMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -21) * METRE_SECOND::<Exact>);

    /// One `YoctometreSecond` (ym · s), as an `Absement`.
    pub const YOCTOMETRE_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(1, -24) * METRE_SECOND::<Exact>);

    /// One `FootSecond` (ft · s), as an `Absement`.
    pub const FOOT_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(3048, -4) * METRE_SECOND::<Exact>);

    /// One `InchSecond` (in · s), as an `Absement`.
    pub const INCH_SECOND<N: const From<Exact>>: Absement<N> = counted(Exact::dec(254, -4) * METRE_SECOND::<Exact>);

    /// One `KilometreHour` (km · h), as an `Absement`.
    pub const KILOMETRE_HOUR<N: const From<Exact>>: Absement<N> = counted(Exact::int(3600000) * METRE_SECOND::<Exact>);

    /// One `YottasecondPerMetre` (Ys/m), as an `InverseVelocity`.
    pub const YOTTASECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(1000000000000000000000000) * SECOND_PER_METRE::<Exact>);

    /// One `ZettasecondPerMetre` (Zs/m), as an `InverseVelocity`.
    pub const ZETTASECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(1000000000000000000000) * SECOND_PER_METRE::<Exact>);

    /// One `ExasecondPerMetre` (Es/m), as an `InverseVelocity`.
    pub const EXASECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(1000000000000000000) * SECOND_PER_METRE::<Exact>);

    /// One `PetasecondPerMetre` (Ps/m), as an `InverseVelocity`.
    pub const PETASECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(1000000000000000) * SECOND_PER_METRE::<Exact>);

    /// One `TerasecondPerMetre` (Ts/m), as an `InverseVelocity`.
    pub const TERASECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(1000000000000) * SECOND_PER_METRE::<Exact>);

    /// One `GigasecondPerMetre` (Gs/m), as an `InverseVelocity`.
    pub const GIGASECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(1000000000) * SECOND_PER_METRE::<Exact>);

    /// One `MegasecondPerMetre` (Ms/m), as an `InverseVelocity`.
    pub const MEGASECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(1000000) * SECOND_PER_METRE::<Exact>);

    /// One `KilosecondPerMetre` (ks/m), as an `InverseVelocity`.
    pub const KILOSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(1000) * SECOND_PER_METRE::<Exact>);

    /// One `HectosecondPerMetre` (hs/m), as an `InverseVelocity`.
    pub const HECTOSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(100) * SECOND_PER_METRE::<Exact>);

    /// One `DecasecondPerMetre` (das/m), as an `InverseVelocity`.
    pub const DECASECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::int(10) * SECOND_PER_METRE::<Exact>);

    /// One `SecondPerMetre` (s/m), as an `InverseVelocity`.
    pub const SECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(SECOND::<Exact> / METRE::<Exact>);

    /// One `DecisecondPerMetre` (ds/m), as an `InverseVelocity`.
    pub const DECISECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -1) * SECOND_PER_METRE::<Exact>);

    /// One `CentisecondPerMetre` (cs/m), as an `InverseVelocity`.
    pub const CENTISECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -2) * SECOND_PER_METRE::<Exact>);

    /// One `MillisecondPerMetre` (ms/m), as an `InverseVelocity`.
    pub const MILLISECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -3) * SECOND_PER_METRE::<Exact>);

    /// One `MicrosecondPerMetre` (µs/m), as an `InverseVelocity`.
    pub const MICROSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -6) * SECOND_PER_METRE::<Exact>);

    /// One `NanosecondPerMetre` (ns/m), as an `InverseVelocity`.
    pub const NANOSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -9) * SECOND_PER_METRE::<Exact>);

    /// One `PicosecondPerMetre` (ps/m), as an `InverseVelocity`.
    pub const PICOSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -12) * SECOND_PER_METRE::<Exact>);

    /// One `FemtosecondPerMetre` (fs/m), as an `InverseVelocity`.
    pub const FEMTOSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -15) * SECOND_PER_METRE::<Exact>);

    /// One `AttosecondPerMetre` (as/m), as an `InverseVelocity`.
    pub const ATTOSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -18) * SECOND_PER_METRE::<Exact>);

    /// One `ZeptosecondPerMetre` (zs/m), as an `InverseVelocity`.
    pub const ZEPTOSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -21) * SECOND_PER_METRE::<Exact>);

    /// One `YoctosecondPerMetre` (ys/m), as an `InverseVelocity`.
    pub const YOCTOSECOND_PER_METRE<N: const From<Exact>>: InverseVelocity<N> = counted(Exact::dec(1, -24) * SECOND_PER_METRE::<Exact>);

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

    /// One `Yottanewton` (YN), as a `Force`.
    pub const YOTTANEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(1000000000000000000000000) * NEWTON::<Exact>);

    /// One `Zettanewton` (ZN), as a `Force`.
    pub const ZETTANEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(1000000000000000000000) * NEWTON::<Exact>);

    /// One `Exanewton` (EN), as a `Force`.
    pub const EXANEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(1000000000000000000) * NEWTON::<Exact>);

    /// One `Petanewton` (PN), as a `Force`.
    pub const PETANEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(1000000000000000) * NEWTON::<Exact>);

    /// One `Teranewton` (TN), as a `Force`.
    pub const TERANEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(1000000000000) * NEWTON::<Exact>);

    /// One `Giganewton` (GN), as a `Force`.
    pub const GIGANEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(1000000000) * NEWTON::<Exact>);

    /// One `Meganewton` (MN), as a `Force`.
    pub const MEGANEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(1000000) * NEWTON::<Exact>);

    /// One `Kilonewton` (kN), as a `Force`.
    pub const KILONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(1000) * NEWTON::<Exact>);

    /// One `Hectonewton` (hN), as a `Force`.
    pub const HECTONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(100) * NEWTON::<Exact>);

    /// One `Decanewton` (daN), as a `Force`.
    pub const DECANEWTON<N: const From<Exact>>: Force<N> = counted(Exact::int(10) * NEWTON::<Exact>);

    /// One `Newton` (N), as a `Force`.
    pub const NEWTON<N: const From<Exact>>: Force<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `Decinewton` (dN), as a `Force`.
    pub const DECINEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -1) * NEWTON::<Exact>);

    /// One `Centinewton` (cN), as a `Force`.
    pub const CENTINEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -2) * NEWTON::<Exact>);

    /// One `Millinewton` (mN), as a `Force`.
    pub const MILLINEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -3) * NEWTON::<Exact>);

    /// One `Micronewton` (µN), as a `Force`.
    pub const MICRONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -6) * NEWTON::<Exact>);

    /// One `Nanonewton` (nN), as a `Force`.
    pub const NANONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -9) * NEWTON::<Exact>);

    /// One `Piconewton` (pN), as a `Force`.
    pub const PICONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -12) * NEWTON::<Exact>);

    /// One `Femtonewton` (fN), as a `Force`.
    pub const FEMTONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -15) * NEWTON::<Exact>);

    /// One `Attonewton` (aN), as a `Force`.
    pub const ATTONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -18) * NEWTON::<Exact>);

    /// One `Zeptonewton` (zN), as a `Force`.
    pub const ZEPTONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -21) * NEWTON::<Exact>);

    /// One `Yoctonewton` (yN), as a `Force`.
    pub const YOCTONEWTON<N: const From<Exact>>: Force<N> = counted(Exact::dec(1, -24) * NEWTON::<Exact>);

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

    /// One `YottagramMetrePerSecond` (Yg · m/s), as a `Momentum`.
    pub const YOTTAGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `ZettagramMetrePerSecond` (Zg · m/s), as a `Momentum`.
    pub const ZETTAGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `ExagramMetrePerSecond` (Eg · m/s), as a `Momentum`.
    pub const EXAGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `PetagramMetrePerSecond` (Pg · m/s), as a `Momentum`.
    pub const PETAGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `TeragramMetrePerSecond` (Tg · m/s), as a `Momentum`.
    pub const TERAGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `GigagramMetrePerSecond` (Gg · m/s), as a `Momentum`.
    pub const GIGAGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `MegagramMetrePerSecond` (Mg · m/s), as a `Momentum`.
    pub const MEGAGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramMetrePerSecond` (kg · m/s), as a `Momentum`.
    pub const KILOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(METRE::<Exact> * KILOGRAM::<Exact> / SECOND::<Exact>);

    /// One `HectogramMetrePerSecond` (hg · m/s), as a `Momentum`.
    pub const HECTOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -1) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `DecagramMetrePerSecond` (dag · m/s), as a `Momentum`.
    pub const DECAGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -2) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `GramMetrePerSecond` (g · m/s), as a `Momentum`.
    pub const GRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `DecigramMetrePerSecond` (dg · m/s), as a `Momentum`.
    pub const DECIGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -4) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `CentigramMetrePerSecond` (cg · m/s), as a `Momentum`.
    pub const CENTIGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -5) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `MilligramMetrePerSecond` (mg · m/s), as a `Momentum`.
    pub const MILLIGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -6) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `MicrogramMetrePerSecond` (µg · m/s), as a `Momentum`.
    pub const MICROGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -9) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `NanogramMetrePerSecond` (ng · m/s), as a `Momentum`.
    pub const NANOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -12) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `PicogramMetrePerSecond` (pg · m/s), as a `Momentum`.
    pub const PICOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -15) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `FemtogramMetrePerSecond` (fg · m/s), as a `Momentum`.
    pub const FEMTOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -18) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `AttogramMetrePerSecond` (ag · m/s), as a `Momentum`.
    pub const ATTOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -21) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `ZeptogramMetrePerSecond` (zg · m/s), as a `Momentum`.
    pub const ZEPTOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -24) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `YoctogramMetrePerSecond` (yg · m/s), as a `Momentum`.
    pub const YOCTOGRAM_METRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -27) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramYottametrePerSecond` (kg · Ym/s), as a `Momentum`.
    pub const KILOGRAM_YOTTAMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramZettametrePerSecond` (kg · Zm/s), as a `Momentum`.
    pub const KILOGRAM_ZETTAMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramExametrePerSecond` (kg · Em/s), as a `Momentum`.
    pub const KILOGRAM_EXAMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramPetametrePerSecond` (kg · Pm/s), as a `Momentum`.
    pub const KILOGRAM_PETAMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramTerametrePerSecond` (kg · Tm/s), as a `Momentum`.
    pub const KILOGRAM_TERAMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramGigametrePerSecond` (kg · Gm/s), as a `Momentum`.
    pub const KILOGRAM_GIGAMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramMegametrePerSecond` (kg · Mm/s), as a `Momentum`.
    pub const KILOGRAM_MEGAMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramKilometrePerSecond` (kg · km/s), as a `Momentum`.
    pub const KILOGRAM_KILOMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(1000) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramHectometrePerSecond` (kg · hm/s), as a `Momentum`.
    pub const KILOGRAM_HECTOMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(100) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramDecametrePerSecond` (kg · dam/s), as a `Momentum`.
    pub const KILOGRAM_DECAMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::int(10) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramDecimetrePerSecond` (kg · dm/s), as a `Momentum`.
    pub const KILOGRAM_DECIMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -1) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramCentimetrePerSecond` (kg · cm/s), as a `Momentum`.
    pub const KILOGRAM_CENTIMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -2) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramMillimetrePerSecond` (kg · mm/s), as a `Momentum`.
    pub const KILOGRAM_MILLIMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -3) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramMicrometrePerSecond` (kg · µm/s), as a `Momentum`.
    pub const KILOGRAM_MICROMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -6) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramNanometrePerSecond` (kg · nm/s), as a `Momentum`.
    pub const KILOGRAM_NANOMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -9) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramPicometrePerSecond` (kg · pm/s), as a `Momentum`.
    pub const KILOGRAM_PICOMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -12) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramFemtometrePerSecond` (kg · fm/s), as a `Momentum`.
    pub const KILOGRAM_FEMTOMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -15) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramAttometrePerSecond` (kg · am/s), as a `Momentum`.
    pub const KILOGRAM_ATTOMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -18) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramZeptometrePerSecond` (kg · zm/s), as a `Momentum`.
    pub const KILOGRAM_ZEPTOMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -21) * KILOGRAM_METRE_PER_SECOND::<Exact>);

    /// One `KilogramYoctometrePerSecond` (kg · ym/s), as a `Momentum`.
    pub const KILOGRAM_YOCTOMETRE_PER_SECOND<N: const From<Exact>>: Momentum<N> = counted(Exact::dec(1, -24) * KILOGRAM_METRE_PER_SECOND::<Exact>);

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

    /// One `Yottajoule` (YJ), as an `Energy`.
    pub const YOTTAJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(1000000000000000000000000) * JOULE::<Exact>);

    /// One `Zettajoule` (ZJ), as an `Energy`.
    pub const ZETTAJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(1000000000000000000000) * JOULE::<Exact>);

    /// One `Exajoule` (EJ), as an `Energy`.
    pub const EXAJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(1000000000000000000) * JOULE::<Exact>);

    /// One `Petajoule` (PJ), as an `Energy`.
    pub const PETAJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(1000000000000000) * JOULE::<Exact>);

    /// One `Terajoule` (TJ), as an `Energy`.
    pub const TERAJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(1000000000000) * JOULE::<Exact>);

    /// One `Gigajoule` (GJ), as an `Energy`.
    pub const GIGAJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(1000000000) * JOULE::<Exact>);

    /// One `Megajoule` (MJ), as an `Energy`.
    pub const MEGAJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(1000000) * JOULE::<Exact>);

    /// One `Kilojoule` (kJ), as an `Energy`.
    pub const KILOJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(1000) * JOULE::<Exact>);

    /// One `Hectojoule` (hJ), as an `Energy`.
    pub const HECTOJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(100) * JOULE::<Exact>);

    /// One `Decajoule` (daJ), as an `Energy`.
    pub const DECAJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::int(10) * JOULE::<Exact>);

    /// One `Joule` (J), as an `Energy`.
    pub const JOULE<N: const From<Exact>>: Energy<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `Decijoule` (dJ), as an `Energy`.
    pub const DECIJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -1) * JOULE::<Exact>);

    /// One `Centijoule` (cJ), as an `Energy`.
    pub const CENTIJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -2) * JOULE::<Exact>);

    /// One `Millijoule` (mJ), as an `Energy`.
    pub const MILLIJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -3) * JOULE::<Exact>);

    /// One `Microjoule` (µJ), as an `Energy`.
    pub const MICROJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -6) * JOULE::<Exact>);

    /// One `Nanojoule` (nJ), as an `Energy`.
    pub const NANOJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -9) * JOULE::<Exact>);

    /// One `Picojoule` (pJ), as an `Energy`.
    pub const PICOJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -12) * JOULE::<Exact>);

    /// One `Femtojoule` (fJ), as an `Energy`.
    pub const FEMTOJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -15) * JOULE::<Exact>);

    /// One `Attojoule` (aJ), as an `Energy`.
    pub const ATTOJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -18) * JOULE::<Exact>);

    /// One `Zeptojoule` (zJ), as an `Energy`.
    pub const ZEPTOJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -21) * JOULE::<Exact>);

    /// One `Yoctojoule` (yJ), as an `Energy`.
    pub const YOCTOJOULE<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1, -24) * JOULE::<Exact>);

    /// One `PetawattHour` (PW · h), as an `Energy`.
    pub const PETAWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(3600000000000000000) * JOULE::<Exact>);

    /// One `TerawattHour` (TW · h), as an `Energy`.
    pub const TERAWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(3600000000000000) * JOULE::<Exact>);

    /// One `GigawattHour` (GW · h), as an `Energy`.
    pub const GIGAWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(3600000000000) * JOULE::<Exact>);

    /// One `MegawattHour` (MW · h), as an `Energy`.
    pub const MEGAWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(3600000000) * JOULE::<Exact>);

    /// One `KilowattHour` (kW · h), as an `Energy`.
    pub const KILOWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(3600000) * JOULE::<Exact>);

    /// One `HectowattHour` (hW · h), as an `Energy`.
    pub const HECTOWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(360000) * JOULE::<Exact>);

    /// One `DecawattHour` (daW · h), as an `Energy`.
    pub const DECAWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(36000) * JOULE::<Exact>);

    /// One `WattHour` (W · h), as an `Energy`.
    pub const WATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::int(3600) * JOULE::<Exact>);

    /// One `MilliwattHour` (mW · h), as an `Energy`.
    pub const MILLIWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::dec(36, -1) * JOULE::<Exact>);

    /// One `MicrowattHour` (µW · h), as an `Energy`.
    pub const MICROWATT_HOUR<N: const From<Exact>>: Energy<N> = counted(Exact::dec(36, -4) * JOULE::<Exact>);

    /// One `Petaelectronvolt` (PeV), as an `Energy`.
    pub const PETAELECTRONVOLT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1602176634, -13) * JOULE::<Exact>);

    /// One `Teraelectronvolt` (TeV), as an `Energy`.
    pub const TERAELECTRONVOLT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1602176634, -16) * JOULE::<Exact>);

    /// One `Gigaelectronvolt` (GeV), as an `Energy`.
    pub const GIGAELECTRONVOLT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1602176634, -19) * JOULE::<Exact>);

    /// One `Megaelectronvolt` (MeV), as an `Energy`.
    pub const MEGAELECTRONVOLT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1602176634, -22) * JOULE::<Exact>);

    /// One `Kiloelectronvolt` (keV), as an `Energy`.
    pub const KILOELECTRONVOLT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1602176634, -25) * JOULE::<Exact>);

    /// One `Hectoelectronvolt` (heV), as an `Energy`.
    pub const HECTOELECTRONVOLT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1602176634, -26) * JOULE::<Exact>);

    /// One `Decaelectronvolt` (daeV), as an `Energy`.
    pub const DECAELECTRONVOLT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(1602176634, -27) * JOULE::<Exact>);

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

    /// One `KilocalorieIt` (kcal (IT)), as an `Energy`.
    pub const KILOCALORIE_IT<N: const From<Exact>>: Energy<N> = counted(Exact::dec(41868, -1) * JOULE::<Exact>);

    /// One `Kilocalorie` (kcal), as an `Energy`.
    pub const KILOCALORIE<N: const From<Exact>>: Energy<N> = counted(Exact::int(4184) * JOULE::<Exact>);

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

    /// One `YottajoulePerKilogram` (YJ/kg), as an `AvailableEnergy`.
    pub const YOTTAJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `ZettajoulePerKilogram` (ZJ/kg), as an `AvailableEnergy`.
    pub const ZETTAJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `ExajoulePerKilogram` (EJ/kg), as an `AvailableEnergy`.
    pub const EXAJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `PetajoulePerKilogram` (PJ/kg), as an `AvailableEnergy`.
    pub const PETAJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `TerajoulePerKilogram` (TJ/kg), as an `AvailableEnergy`.
    pub const TERAJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `GigajoulePerKilogram` (GJ/kg), as an `AvailableEnergy`.
    pub const GIGAJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `MegajoulePerKilogram` (MJ/kg), as an `AvailableEnergy`.
    pub const MEGAJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `KilojoulePerKilogram` (kJ/kg), as an `AvailableEnergy`.
    pub const KILOJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `HectojoulePerKilogram` (hJ/kg), as an `AvailableEnergy`.
    pub const HECTOJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(100) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `DecajoulePerKilogram` (daJ/kg), as an `AvailableEnergy`.
    pub const DECAJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(10) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerKilogram` (J/kg), as an `AvailableEnergy`.
    pub const JOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(METRE::<Exact> * METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `DecijoulePerKilogram` (dJ/kg), as an `AvailableEnergy`.
    pub const DECIJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -1) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `CentijoulePerKilogram` (cJ/kg), as an `AvailableEnergy`.
    pub const CENTIJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -2) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `MillijoulePerKilogram` (mJ/kg), as an `AvailableEnergy`.
    pub const MILLIJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -3) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `MicrojoulePerKilogram` (µJ/kg), as an `AvailableEnergy`.
    pub const MICROJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -6) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `NanojoulePerKilogram` (nJ/kg), as an `AvailableEnergy`.
    pub const NANOJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -9) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `PicojoulePerKilogram` (pJ/kg), as an `AvailableEnergy`.
    pub const PICOJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -12) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `FemtojoulePerKilogram` (fJ/kg), as an `AvailableEnergy`.
    pub const FEMTOJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -15) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `AttojoulePerKilogram` (aJ/kg), as an `AvailableEnergy`.
    pub const ATTOJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -18) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `ZeptojoulePerKilogram` (zJ/kg), as an `AvailableEnergy`.
    pub const ZEPTOJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -21) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `YoctojoulePerKilogram` (yJ/kg), as an `AvailableEnergy`.
    pub const YOCTOJOULE_PER_KILOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -24) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerZeptogram` (J/zg), as an `AvailableEnergy`.
    pub const JOULE_PER_ZEPTOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerAttogram` (J/ag), as an `AvailableEnergy`.
    pub const JOULE_PER_ATTOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerFemtogram` (J/fg), as an `AvailableEnergy`.
    pub const JOULE_PER_FEMTOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerPicogram` (J/pg), as an `AvailableEnergy`.
    pub const JOULE_PER_PICOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerNanogram` (J/ng), as an `AvailableEnergy`.
    pub const JOULE_PER_NANOGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerMicrogram` (J/µg), as an `AvailableEnergy`.
    pub const JOULE_PER_MICROGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerMilligram` (J/mg), as an `AvailableEnergy`.
    pub const JOULE_PER_MILLIGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerGram` (J/g), as an `AvailableEnergy`.
    pub const JOULE_PER_GRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(1000) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerMegagram` (J/Mg), as an `AvailableEnergy`.
    pub const JOULE_PER_MEGAGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -3) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerGigagram` (J/Gg), as an `AvailableEnergy`.
    pub const JOULE_PER_GIGAGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -6) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerTeragram` (J/Tg), as an `AvailableEnergy`.
    pub const JOULE_PER_TERAGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -9) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerPetagram` (J/Pg), as an `AvailableEnergy`.
    pub const JOULE_PER_PETAGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -12) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerExagram` (J/Eg), as an `AvailableEnergy`.
    pub const JOULE_PER_EXAGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -15) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerZettagram` (J/Zg), as an `AvailableEnergy`.
    pub const JOULE_PER_ZETTAGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -18) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `JoulePerYottagram` (J/Yg), as an `AvailableEnergy`.
    pub const JOULE_PER_YOTTAGRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(1, -21) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `BtuItPerPound` (Btu (IT)/lb), as an `AvailableEnergy`.
    pub const BTU_IT_PER_POUND<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(2326) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `BtuPerPound` (Btu/lb), as an `AvailableEnergy`.
    pub const BTU_PER_POUND<N: const From<Exact>>: AvailableEnergy<N> = counted((Exact::int(4184) / Exact::dec(18, -1)) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `CalorieItPerGram` (cal (IT)/g), as an `AvailableEnergy`.
    pub const CALORIE_IT_PER_GRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::dec(41868, -1) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `CaloriePerGram` (cal/g), as an `AvailableEnergy`.
    pub const CALORIE_PER_GRAM<N: const From<Exact>>: AvailableEnergy<N> = counted(Exact::int(4184) * JOULE_PER_KILOGRAM::<Exact>);

    /// One `Yottawatt` (YW), as a `Power`.
    pub const YOTTAWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(1000000000000000000000000) * WATT::<Exact>);

    /// One `Zettawatt` (ZW), as a `Power`.
    pub const ZETTAWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(1000000000000000000000) * WATT::<Exact>);

    /// One `Exawatt` (EW), as a `Power`.
    pub const EXAWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(1000000000000000000) * WATT::<Exact>);

    /// One `Petawatt` (PW), as a `Power`.
    pub const PETAWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(1000000000000000) * WATT::<Exact>);

    /// One `Terawatt` (TW), as a `Power`.
    pub const TERAWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(1000000000000) * WATT::<Exact>);

    /// One `Gigawatt` (GW), as a `Power`.
    pub const GIGAWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(1000000000) * WATT::<Exact>);

    /// One `Megawatt` (MW), as a `Power`.
    pub const MEGAWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(1000000) * WATT::<Exact>);

    /// One `Kilowatt` (kW), as a `Power`.
    pub const KILOWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(1000) * WATT::<Exact>);

    /// One `Hectowatt` (hW), as a `Power`.
    pub const HECTOWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(100) * WATT::<Exact>);

    /// One `Decawatt` (daW), as a `Power`.
    pub const DECAWATT<N: const From<Exact>>: Power<N> = counted(Exact::int(10) * WATT::<Exact>);

    /// One `Watt` (W), as a `Power`.
    pub const WATT<N: const From<Exact>>: Power<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `Deciwatt` (dW), as a `Power`.
    pub const DECIWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -1) * WATT::<Exact>);

    /// One `Centiwatt` (cW), as a `Power`.
    pub const CENTIWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -2) * WATT::<Exact>);

    /// One `Milliwatt` (mW), as a `Power`.
    pub const MILLIWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -3) * WATT::<Exact>);

    /// One `Microwatt` (µW), as a `Power`.
    pub const MICROWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -6) * WATT::<Exact>);

    /// One `Nanowatt` (nW), as a `Power`.
    pub const NANOWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -9) * WATT::<Exact>);

    /// One `Picowatt` (pW), as a `Power`.
    pub const PICOWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -12) * WATT::<Exact>);

    /// One `Femtowatt` (fW), as a `Power`.
    pub const FEMTOWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -15) * WATT::<Exact>);

    /// One `Attowatt` (aW), as a `Power`.
    pub const ATTOWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -18) * WATT::<Exact>);

    /// One `Zeptowatt` (zW), as a `Power`.
    pub const ZEPTOWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -21) * WATT::<Exact>);

    /// One `Yoctowatt` (yW), as a `Power`.
    pub const YOCTOWATT<N: const From<Exact>>: Power<N> = counted(Exact::dec(1, -24) * WATT::<Exact>);

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
    pub const DBW<N: const From<Exact>>: Scale<PowerLevel<N>, Gain<N>> = Scale(counted(WATT_LEVEL::<Exact>), counted(DECIBEL::<Exact>));

    /// Decibels-milliwatt (dBm): decibels from one milliwatt. A unit of `PowerLevel`; the watts are `Power::from` away.
    pub const DBM<N: const From<Exact>>: Scale<PowerLevel<N>, Gain<N>> = Scale(counted(MILLIWATT_LEVEL::<Exact>), counted(DECIBEL::<Exact>));

    /// One `YottawattPerSecond` (YW/s), as a `PowerRate`.
    pub const YOTTAWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(1000000000000000000000000) * WATT_PER_SECOND::<Exact>);

    /// One `ZettawattPerSecond` (ZW/s), as a `PowerRate`.
    pub const ZETTAWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(1000000000000000000000) * WATT_PER_SECOND::<Exact>);

    /// One `ExawattPerSecond` (EW/s), as a `PowerRate`.
    pub const EXAWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(1000000000000000000) * WATT_PER_SECOND::<Exact>);

    /// One `PetawattPerSecond` (PW/s), as a `PowerRate`.
    pub const PETAWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(1000000000000000) * WATT_PER_SECOND::<Exact>);

    /// One `TerawattPerSecond` (TW/s), as a `PowerRate`.
    pub const TERAWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(1000000000000) * WATT_PER_SECOND::<Exact>);

    /// One `GigawattPerSecond` (GW/s), as a `PowerRate`.
    pub const GIGAWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(1000000000) * WATT_PER_SECOND::<Exact>);

    /// One `MegawattPerSecond` (MW/s), as a `PowerRate`.
    pub const MEGAWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(1000000) * WATT_PER_SECOND::<Exact>);

    /// One `KilowattPerSecond` (kW/s), as a `PowerRate`.
    pub const KILOWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(1000) * WATT_PER_SECOND::<Exact>);

    /// One `HectowattPerSecond` (hW/s), as a `PowerRate`.
    pub const HECTOWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(100) * WATT_PER_SECOND::<Exact>);

    /// One `DecawattPerSecond` (daW/s), as a `PowerRate`.
    pub const DECAWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::int(10) * WATT_PER_SECOND::<Exact>);

    /// One `WattPerSecond` (W/s), as a `PowerRate`.
    pub const WATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `DeciwattPerSecond` (dW/s), as a `PowerRate`.
    pub const DECIWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -1) * WATT_PER_SECOND::<Exact>);

    /// One `CentiwattPerSecond` (cW/s), as a `PowerRate`.
    pub const CENTIWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -2) * WATT_PER_SECOND::<Exact>);

    /// One `MilliwattPerSecond` (mW/s), as a `PowerRate`.
    pub const MILLIWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -3) * WATT_PER_SECOND::<Exact>);

    /// One `MicrowattPerSecond` (µW/s), as a `PowerRate`.
    pub const MICROWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -6) * WATT_PER_SECOND::<Exact>);

    /// One `NanowattPerSecond` (nW/s), as a `PowerRate`.
    pub const NANOWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -9) * WATT_PER_SECOND::<Exact>);

    /// One `PicowattPerSecond` (pW/s), as a `PowerRate`.
    pub const PICOWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -12) * WATT_PER_SECOND::<Exact>);

    /// One `FemtowattPerSecond` (fW/s), as a `PowerRate`.
    pub const FEMTOWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -15) * WATT_PER_SECOND::<Exact>);

    /// One `AttowattPerSecond` (aW/s), as a `PowerRate`.
    pub const ATTOWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -18) * WATT_PER_SECOND::<Exact>);

    /// One `ZeptowattPerSecond` (zW/s), as a `PowerRate`.
    pub const ZEPTOWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -21) * WATT_PER_SECOND::<Exact>);

    /// One `YoctowattPerSecond` (yW/s), as a `PowerRate`.
    pub const YOCTOWATT_PER_SECOND<N: const From<Exact>>: PowerRate<N> = counted(Exact::dec(1, -24) * WATT_PER_SECOND::<Exact>);

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

    /// One `YottanewtonMetre` (YN · m), as a `Torque`.
    pub const YOTTANEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000000000000000) * NEWTON_METRE::<Exact>);

    /// One `ZettanewtonMetre` (ZN · m), as a `Torque`.
    pub const ZETTANEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000000000000) * NEWTON_METRE::<Exact>);

    /// One `ExanewtonMetre` (EN · m), as a `Torque`.
    pub const EXANEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000000000) * NEWTON_METRE::<Exact>);

    /// One `PetanewtonMetre` (PN · m), as a `Torque`.
    pub const PETANEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000000) * NEWTON_METRE::<Exact>);

    /// One `TeranewtonMetre` (TN · m), as a `Torque`.
    pub const TERANEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000) * NEWTON_METRE::<Exact>);

    /// One `GiganewtonMetre` (GN · m), as a `Torque`.
    pub const GIGANEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000) * NEWTON_METRE::<Exact>);

    /// One `MeganewtonMetre` (MN · m), as a `Torque`.
    pub const MEGANEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000) * NEWTON_METRE::<Exact>);

    /// One `KilonewtonMetre` (kN · m), as a `Torque`.
    pub const KILONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000) * NEWTON_METRE::<Exact>);

    /// One `HectonewtonMetre` (hN · m), as a `Torque`.
    pub const HECTONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(100) * NEWTON_METRE::<Exact>);

    /// One `DecanewtonMetre` (daN · m), as a `Torque`.
    pub const DECANEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(10) * NEWTON_METRE::<Exact>);

    /// One `NewtonMetre` (N · m), as a `Torque`.
    pub const NEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(METRE::<Exact> * METRE::<Exact> * KILOGRAM::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * RADIAN::<Exact>));

    /// One `DecinewtonMetre` (dN · m), as a `Torque`.
    pub const DECINEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -1) * NEWTON_METRE::<Exact>);

    /// One `CentinewtonMetre` (cN · m), as a `Torque`.
    pub const CENTINEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -2) * NEWTON_METRE::<Exact>);

    /// One `MillinewtonMetre` (mN · m), as a `Torque`.
    pub const MILLINEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -3) * NEWTON_METRE::<Exact>);

    /// One `MicronewtonMetre` (µN · m), as a `Torque`.
    pub const MICRONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -6) * NEWTON_METRE::<Exact>);

    /// One `NanonewtonMetre` (nN · m), as a `Torque`.
    pub const NANONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -9) * NEWTON_METRE::<Exact>);

    /// One `PiconewtonMetre` (pN · m), as a `Torque`.
    pub const PICONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -12) * NEWTON_METRE::<Exact>);

    /// One `FemtonewtonMetre` (fN · m), as a `Torque`.
    pub const FEMTONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -15) * NEWTON_METRE::<Exact>);

    /// One `AttonewtonMetre` (aN · m), as a `Torque`.
    pub const ATTONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -18) * NEWTON_METRE::<Exact>);

    /// One `ZeptonewtonMetre` (zN · m), as a `Torque`.
    pub const ZEPTONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -21) * NEWTON_METRE::<Exact>);

    /// One `YoctonewtonMetre` (yN · m), as a `Torque`.
    pub const YOCTONEWTON_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -24) * NEWTON_METRE::<Exact>);

    /// One `NewtonYottametre` (N · Ym), as a `Torque`.
    pub const NEWTON_YOTTAMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000000000000000) * NEWTON_METRE::<Exact>);

    /// One `NewtonZettametre` (N · Zm), as a `Torque`.
    pub const NEWTON_ZETTAMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000000000000) * NEWTON_METRE::<Exact>);

    /// One `NewtonExametre` (N · Em), as a `Torque`.
    pub const NEWTON_EXAMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000000000) * NEWTON_METRE::<Exact>);

    /// One `NewtonPetametre` (N · Pm), as a `Torque`.
    pub const NEWTON_PETAMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000000) * NEWTON_METRE::<Exact>);

    /// One `NewtonTerametre` (N · Tm), as a `Torque`.
    pub const NEWTON_TERAMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000000) * NEWTON_METRE::<Exact>);

    /// One `NewtonGigametre` (N · Gm), as a `Torque`.
    pub const NEWTON_GIGAMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000000) * NEWTON_METRE::<Exact>);

    /// One `NewtonMegametre` (N · Mm), as a `Torque`.
    pub const NEWTON_MEGAMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000000) * NEWTON_METRE::<Exact>);

    /// One `NewtonKilometre` (N · km), as a `Torque`.
    pub const NEWTON_KILOMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(1000) * NEWTON_METRE::<Exact>);

    /// One `NewtonHectometre` (N · hm), as a `Torque`.
    pub const NEWTON_HECTOMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(100) * NEWTON_METRE::<Exact>);

    /// One `NewtonDecametre` (N · dam), as a `Torque`.
    pub const NEWTON_DECAMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::int(10) * NEWTON_METRE::<Exact>);

    /// One `NewtonDecimetre` (N · dm), as a `Torque`.
    pub const NEWTON_DECIMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -1) * NEWTON_METRE::<Exact>);

    /// One `NewtonCentimetre` (N · cm), as a `Torque`.
    pub const NEWTON_CENTIMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -2) * NEWTON_METRE::<Exact>);

    /// One `NewtonMillimetre` (N · mm), as a `Torque`.
    pub const NEWTON_MILLIMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -3) * NEWTON_METRE::<Exact>);

    /// One `NewtonMicrometre` (N · µm), as a `Torque`.
    pub const NEWTON_MICROMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -6) * NEWTON_METRE::<Exact>);

    /// One `NewtonNanometre` (N · nm), as a `Torque`.
    pub const NEWTON_NANOMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -9) * NEWTON_METRE::<Exact>);

    /// One `NewtonPicometre` (N · pm), as a `Torque`.
    pub const NEWTON_PICOMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -12) * NEWTON_METRE::<Exact>);

    /// One `NewtonFemtometre` (N · fm), as a `Torque`.
    pub const NEWTON_FEMTOMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -15) * NEWTON_METRE::<Exact>);

    /// One `NewtonAttometre` (N · am), as a `Torque`.
    pub const NEWTON_ATTOMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -18) * NEWTON_METRE::<Exact>);

    /// One `NewtonZeptometre` (N · zm), as a `Torque`.
    pub const NEWTON_ZEPTOMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -21) * NEWTON_METRE::<Exact>);

    /// One `NewtonYoctometre` (N · ym), as a `Torque`.
    pub const NEWTON_YOCTOMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -24) * NEWTON_METRE::<Exact>);

    /// One `DyneMetre` (dyn · m), as a `Torque`.
    pub const DYNE_METRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -5) * NEWTON_METRE::<Exact>);

    /// One `DyneCentimetre` (dyn · cm), as a `Torque`.
    pub const DYNE_CENTIMETRE<N: const From<Exact>>: Torque<N> = counted(Exact::dec(1, -7) * NEWTON_METRE::<Exact>);

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

    /// One `Yottapascal` (YPa), as a `Pressure`.
    pub const YOTTAPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000000000000000000000000) * PASCAL::<Exact>);

    /// One `Zettapascal` (ZPa), as a `Pressure`.
    pub const ZETTAPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000000000000000000000) * PASCAL::<Exact>);

    /// One `Exapascal` (EPa), as a `Pressure`.
    pub const EXAPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000000000000000000) * PASCAL::<Exact>);

    /// One `Petapascal` (PPa), as a `Pressure`.
    pub const PETAPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000000000000000) * PASCAL::<Exact>);

    /// One `Terapascal` (TPa), as a `Pressure`.
    pub const TERAPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000000000000) * PASCAL::<Exact>);

    /// One `Gigapascal` (GPa), as a `Pressure`.
    pub const GIGAPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000000000) * PASCAL::<Exact>);

    /// One `Megapascal` (MPa), as a `Pressure`.
    pub const MEGAPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000000) * PASCAL::<Exact>);

    /// One `Kilopascal` (kPa), as a `Pressure`.
    pub const KILOPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(1000) * PASCAL::<Exact>);

    /// One `Hectopascal` (hPa), as a `Pressure`.
    pub const HECTOPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(100) * PASCAL::<Exact>);

    /// One `Decapascal` (daPa), as a `Pressure`.
    pub const DECAPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::int(10) * PASCAL::<Exact>);

    /// One `Pascal` (Pa), as a `Pressure`.
    pub const PASCAL<N: const From<Exact>>: Pressure<N> = counted(KILOGRAM::<Exact> / (METRE::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `Decipascal` (dPa), as a `Pressure`.
    pub const DECIPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -1) * PASCAL::<Exact>);

    /// One `Centipascal` (cPa), as a `Pressure`.
    pub const CENTIPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -2) * PASCAL::<Exact>);

    /// One `Millipascal` (mPa), as a `Pressure`.
    pub const MILLIPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -3) * PASCAL::<Exact>);

    /// One `Micropascal` (µPa), as a `Pressure`.
    pub const MICROPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -6) * PASCAL::<Exact>);

    /// One `Nanopascal` (nPa), as a `Pressure`.
    pub const NANOPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -9) * PASCAL::<Exact>);

    /// One `Picopascal` (pPa), as a `Pressure`.
    pub const PICOPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -12) * PASCAL::<Exact>);

    /// One `Femtopascal` (fPa), as a `Pressure`.
    pub const FEMTOPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -15) * PASCAL::<Exact>);

    /// One `Attopascal` (aPa), as a `Pressure`.
    pub const ATTOPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -18) * PASCAL::<Exact>);

    /// One `Zeptopascal` (zPa), as a `Pressure`.
    pub const ZEPTOPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -21) * PASCAL::<Exact>);

    /// One `Yoctopascal` (yPa), as a `Pressure`.
    pub const YOCTOPASCAL<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(1, -24) * PASCAL::<Exact>);

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

    /// One `Millibar` (mbar), as a `Pressure`.
    pub const MILLIBAR<N: const From<Exact>>: Pressure<N> = counted(Exact::int(100) * PASCAL::<Exact>);

    /// One `MillimetreOfMercury` (mm Hg), as a `Pressure`.
    pub const MILLIMETRE_OF_MERCURY<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(133322387415, -9) * PASCAL::<Exact>);

    /// One `MillimetreOfWater` (mm H₂O), as a `Pressure`.
    pub const MILLIMETRE_OF_WATER<N: const From<Exact>>: Pressure<N> = counted(Exact::dec(980665, -5) * PASCAL::<Exact>);

    /// One `Millitorr` (mTorr), as a `Pressure`.
    pub const MILLITORR<N: const From<Exact>>: Pressure<N> = counted((Exact::int(101325) / Exact::int(760000)) * PASCAL::<Exact>);

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

    /// One `Yottahertz` (YHz), as a `Frequency`.
    pub const YOTTAHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(1000000000000000000000000) * HERTZ::<Exact>);

    /// One `Zettahertz` (ZHz), as a `Frequency`.
    pub const ZETTAHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(1000000000000000000000) * HERTZ::<Exact>);

    /// One `Exahertz` (EHz), as a `Frequency`.
    pub const EXAHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(1000000000000000000) * HERTZ::<Exact>);

    /// One `Petahertz` (PHz), as a `Frequency`.
    pub const PETAHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(1000000000000000) * HERTZ::<Exact>);

    /// One `Terahertz` (THz), as a `Frequency`.
    pub const TERAHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(1000000000000) * HERTZ::<Exact>);

    /// One `Gigahertz` (GHz), as a `Frequency`.
    pub const GIGAHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(1000000000) * HERTZ::<Exact>);

    /// One `Megahertz` (MHz), as a `Frequency`.
    pub const MEGAHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(1000000) * HERTZ::<Exact>);

    /// One `Kilohertz` (kHz), as a `Frequency`.
    pub const KILOHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(1000) * HERTZ::<Exact>);

    /// One `Hectohertz` (hHz), as a `Frequency`.
    pub const HECTOHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(100) * HERTZ::<Exact>);

    /// One `Decahertz` (daHz), as a `Frequency`.
    pub const DECAHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::int(10) * HERTZ::<Exact>);

    /// One `Hertz` (Hz), as a `Frequency`.
    pub const HERTZ<N: const From<Exact>>: Frequency<N> = counted(ONE::<Exact> / SECOND::<Exact>);

    /// One `Decihertz` (dHz), as a `Frequency`.
    pub const DECIHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -1) * HERTZ::<Exact>);

    /// One `Centihertz` (cHz), as a `Frequency`.
    pub const CENTIHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -2) * HERTZ::<Exact>);

    /// One `Millihertz` (mHz), as a `Frequency`.
    pub const MILLIHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -3) * HERTZ::<Exact>);

    /// One `Microhertz` (µHz), as a `Frequency`.
    pub const MICROHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -6) * HERTZ::<Exact>);

    /// One `Nanohertz` (nHz), as a `Frequency`.
    pub const NANOHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -9) * HERTZ::<Exact>);

    /// One `Picohertz` (pHz), as a `Frequency`.
    pub const PICOHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -12) * HERTZ::<Exact>);

    /// One `Femtohertz` (fHz), as a `Frequency`.
    pub const FEMTOHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -15) * HERTZ::<Exact>);

    /// One `Attohertz` (aHz), as a `Frequency`.
    pub const ATTOHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -18) * HERTZ::<Exact>);

    /// One `Zeptohertz` (zHz), as a `Frequency`.
    pub const ZEPTOHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -21) * HERTZ::<Exact>);

    /// One `Yoctohertz` (yHz), as a `Frequency`.
    pub const YOCTOHERTZ<N: const From<Exact>>: Frequency<N> = counted(Exact::dec(1, -24) * HERTZ::<Exact>);

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

    /// One `TerahertzPerSecond` (THz/s), as a `FrequencyDrift`.
    pub const TERAHERTZ_PER_SECOND<N: const From<Exact>>: FrequencyDrift<N> = counted(Exact::int(1000000000000) * HERTZ_PER_SECOND::<Exact>);

    /// One `GigahertzPerSecond` (GHz/s), as a `FrequencyDrift`.
    pub const GIGAHERTZ_PER_SECOND<N: const From<Exact>>: FrequencyDrift<N> = counted(Exact::int(1000000000) * HERTZ_PER_SECOND::<Exact>);

    /// One `MegahertzPerSecond` (MHz/s), as a `FrequencyDrift`.
    pub const MEGAHERTZ_PER_SECOND<N: const From<Exact>>: FrequencyDrift<N> = counted(Exact::int(1000000) * HERTZ_PER_SECOND::<Exact>);

    /// One `KilohertzPerSecond` (kHz/s), as a `FrequencyDrift`.
    pub const KILOHERTZ_PER_SECOND<N: const From<Exact>>: FrequencyDrift<N> = counted(Exact::int(1000) * HERTZ_PER_SECOND::<Exact>);

    /// One `HertzPerSecond` (Hz/s), as a `FrequencyDrift`.
    pub const HERTZ_PER_SECOND<N: const From<Exact>>: FrequencyDrift<N> = counted(ONE::<Exact> / (SECOND::<Exact> * SECOND::<Exact>));

    /// One `ReciprocalKilometre` (km⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_KILOMETRE<N: const From<Exact>>: ReciprocalLength<N> = counted(Exact::dec(1, -3) * RECIPROCAL_METRE::<Exact>);

    /// One `ReciprocalMetre` (m⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_METRE<N: const From<Exact>>: ReciprocalLength<N> = counted(ONE::<Exact> / METRE::<Exact>);

    /// One `ReciprocalDecimetre` (dm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_DECIMETRE<N: const From<Exact>>: ReciprocalLength<N> = counted(Exact::int(10) * RECIPROCAL_METRE::<Exact>);

    /// One `ReciprocalCentimetre` (cm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_CENTIMETRE<N: const From<Exact>>: ReciprocalLength<N> = counted(Exact::int(100) * RECIPROCAL_METRE::<Exact>);

    /// One `ReciprocalMillimetre` (mm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_MILLIMETRE<N: const From<Exact>>: ReciprocalLength<N> = counted(Exact::int(1000) * RECIPROCAL_METRE::<Exact>);

    /// One `ReciprocalMicrometre` (µm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_MICROMETRE<N: const From<Exact>>: ReciprocalLength<N> = counted(Exact::int(1000000) * RECIPROCAL_METRE::<Exact>);

    /// One `ReciprocalNanometre` (nm⁻¹), as a `ReciprocalLength`.
    pub const RECIPROCAL_NANOMETRE<N: const From<Exact>>: ReciprocalLength<N> = counted(Exact::int(1000000000) * RECIPROCAL_METRE::<Exact>);

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

    /// One `RadianPerMillimetre` (rad/mm), as a `Curvature`.
    pub const RADIAN_PER_MILLIMETRE<N: const From<Exact>>: Curvature<N> = counted(Exact::int(1000) * RADIAN_PER_METRE::<Exact>);

    /// One `DegreePerMillimetre` (°/mm), as a `Curvature`.
    pub const DEGREE_PER_MILLIMETRE<N: const From<Exact>>: Curvature<N> = counted((Exact::int(1000) * Exact::PI / Exact::int(180)) * RADIAN_PER_METRE::<Exact>);

    /// One `SquareMetrePerKilogram` (m²/kg), as a `SpecificArea`.
    pub const SQUARE_METRE_PER_KILOGRAM<N: const From<Exact>>: SpecificArea<N> = counted(METRE::<Exact> * METRE::<Exact> / KILOGRAM::<Exact>);

    /// One `SquareCentimetrePerKilogram` (cm²/kg), as a `SpecificArea`.
    pub const SQUARE_CENTIMETRE_PER_KILOGRAM<N: const From<Exact>>: SpecificArea<N> = counted(Exact::dec(1, -4) * SQUARE_METRE_PER_KILOGRAM::<Exact>);

    /// One `SquareMetrePerGram` (m²/g), as a `SpecificArea`.
    pub const SQUARE_METRE_PER_GRAM<N: const From<Exact>>: SpecificArea<N> = counted(Exact::int(1000) * SQUARE_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerYottagram` (m³/Yg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_YOTTAGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::dec(1, -21) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerZettagram` (m³/Zg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_ZETTAGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::dec(1, -18) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerExagram` (m³/Eg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_EXAGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::dec(1, -15) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerPetagram` (m³/Pg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_PETAGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::dec(1, -12) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerTeragram` (m³/Tg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_TERAGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::dec(1, -9) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerGigagram` (m³/Gg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_GIGAGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::dec(1, -6) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerMegagram` (m³/Mg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_MEGAGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::dec(1, -3) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerKilogram` (m³/kg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_KILOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(METRE::<Exact> * METRE::<Exact> * METRE::<Exact> / KILOGRAM::<Exact>);

    /// One `CubicMetrePerHectogram` (m³/hg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_HECTOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(10) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerDecagram` (m³/dag), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_DECAGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(100) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerGram` (m³/g), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_GRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerDecigram` (m³/dg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_DECIGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(10000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerCentigram` (m³/cg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_CENTIGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(100000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerMilligram` (m³/mg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_MILLIGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerMicrogram` (m³/µg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_MICROGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000000000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerNanogram` (m³/ng), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_NANOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000000000000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerPicogram` (m³/pg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_PICOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000000000000000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerFemtogram` (m³/fg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_FEMTOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000000000000000000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerAttogram` (m³/ag), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_ATTOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000000000000000000000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerZeptogram` (m³/zg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_ZEPTOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000000000000000000000000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

    /// One `CubicMetrePerYoctogram` (m³/yg), as a `SpecificVolume`.
    pub const CUBIC_METRE_PER_YOCTOGRAM<N: const From<Exact>>: SpecificVolume<N> = counted(Exact::int(1000000000000000000000000000) * CUBIC_METRE_PER_KILOGRAM::<Exact>);

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

    /// One `YottawattPerKilogram` (YW/kg), as a `SpecificPower`.
    pub const YOTTAWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(1000000000000000000000000) * WATT_PER_KILOGRAM::<Exact>);

    /// One `ZettawattPerKilogram` (ZW/kg), as a `SpecificPower`.
    pub const ZETTAWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(1000000000000000000000) * WATT_PER_KILOGRAM::<Exact>);

    /// One `ExawattPerKilogram` (EW/kg), as a `SpecificPower`.
    pub const EXAWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(1000000000000000000) * WATT_PER_KILOGRAM::<Exact>);

    /// One `PetawattPerKilogram` (PW/kg), as a `SpecificPower`.
    pub const PETAWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(1000000000000000) * WATT_PER_KILOGRAM::<Exact>);

    /// One `TerawattPerKilogram` (TW/kg), as a `SpecificPower`.
    pub const TERAWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(1000000000000) * WATT_PER_KILOGRAM::<Exact>);

    /// One `GigawattPerKilogram` (GW/kg), as a `SpecificPower`.
    pub const GIGAWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(1000000000) * WATT_PER_KILOGRAM::<Exact>);

    /// One `MegawattPerKilogram` (MW/kg), as a `SpecificPower`.
    pub const MEGAWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(1000000) * WATT_PER_KILOGRAM::<Exact>);

    /// One `KilowattPerKilogram` (kW/kg), as a `SpecificPower`.
    pub const KILOWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(1000) * WATT_PER_KILOGRAM::<Exact>);

    /// One `HectowattPerKilogram` (hW/kg), as a `SpecificPower`.
    pub const HECTOWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(100) * WATT_PER_KILOGRAM::<Exact>);

    /// One `DecawattPerKilogram` (daW/kg), as a `SpecificPower`.
    pub const DECAWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::int(10) * WATT_PER_KILOGRAM::<Exact>);

    /// One `WattPerKilogram` (W/kg), as a `SpecificPower`.
    pub const WATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(METRE::<Exact> * METRE::<Exact> / (SECOND::<Exact> * SECOND::<Exact> * SECOND::<Exact>));

    /// One `DeciwattPerKilogram` (dW/kg), as a `SpecificPower`.
    pub const DECIWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -1) * WATT_PER_KILOGRAM::<Exact>);

    /// One `CentiwattPerKilogram` (cW/kg), as a `SpecificPower`.
    pub const CENTIWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -2) * WATT_PER_KILOGRAM::<Exact>);

    /// One `MilliwattPerKilogram` (mW/kg), as a `SpecificPower`.
    pub const MILLIWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -3) * WATT_PER_KILOGRAM::<Exact>);

    /// One `MicrowattPerKilogram` (µW/kg), as a `SpecificPower`.
    pub const MICROWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -6) * WATT_PER_KILOGRAM::<Exact>);

    /// One `NanowattPerKilogram` (nW/kg), as a `SpecificPower`.
    pub const NANOWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -9) * WATT_PER_KILOGRAM::<Exact>);

    /// One `PicowattPerKilogram` (pW/kg), as a `SpecificPower`.
    pub const PICOWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -12) * WATT_PER_KILOGRAM::<Exact>);

    /// One `FemtowattPerKilogram` (fW/kg), as a `SpecificPower`.
    pub const FEMTOWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -15) * WATT_PER_KILOGRAM::<Exact>);

    /// One `AttowattPerKilogram` (aW/kg), as a `SpecificPower`.
    pub const ATTOWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -18) * WATT_PER_KILOGRAM::<Exact>);

    /// One `ZeptowattPerKilogram` (zW/kg), as a `SpecificPower`.
    pub const ZEPTOWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -21) * WATT_PER_KILOGRAM::<Exact>);

    /// One `YoctowattPerKilogram` (yW/kg), as a `SpecificPower`.
    pub const YOCTOWATT_PER_KILOGRAM<N: const From<Exact>>: SpecificPower<N> = counted(Exact::dec(1, -24) * WATT_PER_KILOGRAM::<Exact>);

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
#[unit(of = Mass<f64>)]
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
#[unit(of = Time<f64>)]
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
#[unit(of = Velocity<f64>)]
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
#[unit(of = Acceleration<f64>)]
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
#[unit(of = Jerk<f64>)]
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
#[unit(of = Absement<f64>)]
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
#[unit(of = InverseVelocity<f64>)]
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
#[unit(of = Force<f64>)]
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
#[unit(of = Momentum<f64>)]
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
#[unit(of = Energy<f64>)]
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
#[unit(of = AvailableEnergy<f64>)]
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
#[unit(of = Power<f64>, non_const)]
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
    #[unit("dBW", "decibels-watt", by = DBW, via = PowerLevel<f64>)]
    DecibelWatt,
    #[unit("dBm", "decibels-milliwatt", by = DBM, via = PowerLevel<f64>)]
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
#[unit(of = Frequency<f64>)]
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
#[unit(of = FrequencyDrift<f64>)]
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
#[unit(of = ReciprocalLength<f64>)]
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
#[unit(of = Curvature<f64>)]
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
#[unit(of = SpecificArea<f64>)]
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
#[unit(of = SpecificVolume<f64>)]
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
#[unit(of = SpecificPower<f64>)]
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
#[unit(of = SpecificGravitationalParameter<f64>)]
pub enum SpecificGravitationalParameterUnit {
    #[default]
    #[unit("m³/(kg · s²)", "cubic metres per kilogram second squared", by = CUBIC_METRE_PER_KILOGRAM_SECOND_SQUARED)]
    CubicMetrePerKilogramSecondSquared,
    #[unit("G", "Newtonian constants of gravitation", by = GRAVITATIONAL_CONSTANT)]
    GravitationalConstant,
}
