//! The quantities of `mechanical` and their units.

#[allow(unused_imports)]
use crate::*;

/// A length, in `N`s of the unit `U`: by default its base unit, `METRE`.
pub type Length<N, const U: SIUnit = { METRE.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;

/// Kilometres (`km`), a unit of `Length`: `KILO * METRE`.
pub const KILOMETRE: Unit<{ (KILO * METRE).unit() }> = Unit::ONE;

/// Ångströms (`Å`), a unit of `Length`: `n` of them is `n · 1e-10` base units.
pub const ANGSTROM: Unit<{ SIUnit::scaled("Å", Exact::dec(1, -10), METRE.unit()) }> = Unit::ONE;

/// Bohr radiuses (`a₀`), a unit of `Length`: `n` of them is `n · 5.29177210903e-11` base units.
pub const BOHR_RADIUS: Unit<{ SIUnit::scaled("a₀", Exact::dec(529177210903, -22), METRE.unit()) }> =
    Unit::ONE;

/// Atomic units of length (`a.u. of length`), a unit of `Length`: `n` of them is `n · 5.29177210903e-11` base units.
pub const ATOMIC_UNIT_OF_LENGTH: Unit<
    {
        SIUnit::scaled(
            "a.u. of length",
            Exact::dec(529177210903, -22),
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Astronomical units (`ua`), a unit of `Length`: `n` of them is `n · 149597870700.0` base units.
pub const ASTRONOMICAL_UNIT: Unit<
    { SIUnit::scaled("ua", Exact::int(149597870700), METRE.unit()) },
> = Unit::ONE;

/// Chains (`ch`), a unit of `Length`: `n` of them is `n · 66.0 * 1200.0 / 3937.0` base units.
pub const CHAIN: Unit<
    {
        SIUnit::scaled(
            "ch",
            Exact::int(66) * Exact::int(1200) / Exact::int(3937),
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Data miles (`DM`), a unit of `Length`: `n` of them is `n · 1828.8` base units.
pub const DATA_MILE: Unit<{ SIUnit::scaled("DM", Exact::dec(18288, -1), METRE.unit()) }> =
    Unit::ONE;

/// Fathoms (`fathom`), a unit of `Length`: `n` of them is `n · 6.0 * 1200.0 / 3937.0` base units.
pub const FATHOM: Unit<
    {
        SIUnit::scaled(
            "fathom",
            Exact::int(6) * Exact::int(1200) / Exact::int(3937),
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Fermis (`fermi`), a unit of `Length`: `n` of them is `n · 1e-15` base units.
pub const FERMI: Unit<{ SIUnit::scaled("fermi", Exact::dec(1, -15), METRE.unit()) }> = Unit::ONE;

/// Feet (`ft`), a unit of `Length`: `n` of them is `n · 0.3048` base units.
pub const FOOT: Unit<{ SIUnit::scaled("ft", Exact::dec(3048, -4), METRE.unit()) }> = Unit::ONE;

/// Feet (U.S. survey) (`ft (U.S. survey)`), a unit of `Length`: `n` of them is `n · 1200.0 / 3937.0` base units.
pub const FOOT_SURVEY: Unit<
    {
        SIUnit::scaled(
            "ft (U.S. survey)",
            Exact::int(1200) / Exact::int(3937),
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Inches (`in`), a unit of `Length`: `n` of them is `n · 0.0254` base units.
pub const INCH: Unit<{ SIUnit::scaled("in", Exact::dec(254, -4), METRE.unit()) }> = Unit::ONE;

/// Light years (`l. y.`), a unit of `Length`: `n` of them is `n · 9460730472580800.0` base units.
pub const LIGHT_YEAR: Unit<
    { SIUnit::scaled("l. y.", Exact::int(9460730472580800), METRE.unit()) },
> = Unit::ONE;

/// Microns (`μ`), a unit of `Length`: `n` of them is `n · 1e-06` base units.
pub const MICRON: Unit<{ SIUnit::scaled("μ", Exact::dec(1, -6), METRE.unit()) }> = Unit::ONE;

/// Mils (`0.001 in`), a unit of `Length`: `n` of them is `n · 2.54e-05` base units.
pub const MIL: Unit<{ SIUnit::scaled("0.001 in", Exact::dec(254, -7), METRE.unit()) }> = Unit::ONE;

/// Miles (`mi`), a unit of `Length`: `n` of them is `n · 1609.344` base units.
pub const MILE: Unit<{ SIUnit::scaled("mi", Exact::dec(1609344, -3), METRE.unit()) }> = Unit::ONE;

/// Miles (U.S. survey) (`mi (U.S. survey)`), a unit of `Length`: `n` of them is `n · 6336000.0 / 3937.0` base units.
pub const MILE_SURVEY: Unit<
    {
        SIUnit::scaled(
            "mi (U.S. survey)",
            Exact::int(6336000) / Exact::int(3937),
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Nautical miles (`M`), a unit of `Length`: `n` of them is `n · 1852.0` base units.
pub const NAUTICAL_MILE: Unit<{ SIUnit::scaled("M", Exact::int(1852), METRE.unit()) }> = Unit::ONE;

/// Parsecs (`pc`), a unit of `Length`: `n` of them is `n · 149597870700.0 * 648000.0 / core::f64::consts::PI` base units.
pub const PARSEC: Unit<
    {
        SIUnit::scaled(
            "pc",
            Exact::int(149597870700) * Exact::int(648000) / Exact::PI,
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Picas (computer) (`1/6 in (computer)`), a unit of `Length`: `n` of them is `n · 0.0254 / 6.0` base units.
pub const PICA_COMPUTER: Unit<
    {
        SIUnit::scaled(
            "1/6 in (computer)",
            Exact::dec(254, -4) / Exact::int(6),
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Picas (printer's) (`1/6 in`), a unit of `Length`: `n` of them is `n · 0.004217518` base units.
pub const PICA_PRINTERS: Unit<{ SIUnit::scaled("1/6 in", Exact::dec(4217518, -9), METRE.unit()) }> =
    Unit::ONE;

/// Points (computer) (`1/72 in (computer)`), a unit of `Length`: `n` of them is `n · 0.0254 / 72.0` base units.
pub const POINT_COMPUTER: Unit<
    {
        SIUnit::scaled(
            "1/72 in (computer)",
            Exact::dec(254, -4) / Exact::int(72),
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Points (printer's) (`1/72 in`), a unit of `Length`: `n` of them is `n · 0.0003514598` base units.
pub const POINT_PRINTERS: Unit<
    { SIUnit::scaled("1/72 in", Exact::dec(3514598, -10), METRE.unit()) },
> = Unit::ONE;

/// Rods (`rd`), a unit of `Length`: `n` of them is `n · 16.5 * 1200.0 / 3937.0` base units.
pub const ROD: Unit<
    {
        SIUnit::scaled(
            "rd",
            Exact::dec(165, -1) * Exact::int(1200) / Exact::int(3937),
            METRE.unit(),
        )
    },
> = Unit::ONE;

/// Yards (`yd`), a unit of `Length`: `n` of them is `n · 0.9144` base units.
pub const YARD: Unit<{ SIUnit::scaled("yd", Exact::dec(9144, -4), METRE.unit()) }> = Unit::ONE;

/// Leugae (`leuga`), a unit of `Length`: `n` of them is `n · 2220.0` base units.
pub const LEUGA: Unit<{ SIUnit::scaled("leuga", Exact::int(2220), METRE.unit()) }> = Unit::ONE;

/// Mille passa (`mille passus`), a unit of `Length`: `n` of them is `n · 1480.0` base units.
pub const MILLE_PASSUS: Unit<{ SIUnit::scaled("mille passus", Exact::int(1480), METRE.unit()) }> =
    Unit::ONE;

/// Stadia (`stadium`), a unit of `Length`: `n` of them is `n · 185.0` base units.
pub const STADIUM: Unit<{ SIUnit::scaled("stadium", Exact::int(185), METRE.unit()) }> = Unit::ONE;

/// Acta (`actus`), a unit of `Length`: `n` of them is `n · 35.52` base units.
pub const ACTUS: Unit<{ SIUnit::scaled("actus", Exact::dec(3552, -2), METRE.unit()) }> = Unit::ONE;

/// Decempedae (`decempeda`), a unit of `Length`: `n` of them is `n · 2.96` base units.
pub const DECEMPEDA: Unit<{ SIUnit::scaled("decempeda", Exact::dec(296, -2), METRE.unit()) }> =
    Unit::ONE;

/// Passa (`passus`), a unit of `Length`: `n` of them is `n · 1.48` base units.
pub const PASSUS: Unit<{ SIUnit::scaled("passus", Exact::dec(148, -2), METRE.unit()) }> = Unit::ONE;

/// Gradus (`gradus`), a unit of `Length`: `n` of them is `n · 0.74` base units.
pub const GRADUS: Unit<{ SIUnit::scaled("gradus", Exact::dec(74, -2), METRE.unit()) }> = Unit::ONE;

/// Cubita (`cubitum`), a unit of `Length`: `n` of them is `n · 0.444` base units.
pub const CUBITUM: Unit<{ SIUnit::scaled("cubitum", Exact::dec(444, -3), METRE.unit()) }> =
    Unit::ONE;

/// Palmipedes (`palmipes`), a unit of `Length`: `n` of them is `n · 0.37` base units.
pub const PALMIPES: Unit<{ SIUnit::scaled("palmipes", Exact::dec(37, -2), METRE.unit()) }> =
    Unit::ONE;

/// Pedes (`pes`), a unit of `Length`: `n` of them is `n · 0.296` base units.
pub const PES: Unit<{ SIUnit::scaled("pes", Exact::dec(296, -3), METRE.unit()) }> = Unit::ONE;

/// Palmas maior (`palmus maior`), a unit of `Length`: `n` of them is `n · 0.222` base units.
pub const PALMUS_MAIOR: Unit<
    { SIUnit::scaled("palmus maior", Exact::dec(222, -3), METRE.unit()) },
> = Unit::ONE;

/// Palmas (`palmus`), a unit of `Length`: `n` of them is `n · 0.074` base units.
pub const PALMUS: Unit<{ SIUnit::scaled("palmus", Exact::dec(74, -3), METRE.unit()) }> = Unit::ONE;

/// Unciae (`uncia`), a unit of `Length`: `n` of them is `n · 0.296 / 12.0` base units.
pub const LENGTH_UNCIA: Unit<
    { SIUnit::scaled("uncia", Exact::dec(296, -3) / Exact::int(12), METRE.unit()) },
> = Unit::ONE;

/// Digiti (`digitus`), a unit of `Length`: `n` of them is `n · 0.0185` base units.
pub const DIGITUS: Unit<{ SIUnit::scaled("digitus", Exact::dec(185, -4), METRE.unit()) }> =
    Unit::ONE;

/// A mass, in `N`s of the unit `U`: by default its base unit, `KILOGRAM`.
pub type Mass<N, const U: SIUnit = { KILOGRAM.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;

/// Grams (`g`), a unit of `Mass`: `n` of them is `n · 0.001` base units.
pub const GRAM: Unit<{ SIUnit::scaled("g", Exact::dec(1, -3), KILOGRAM.unit()) }> = Unit::ONE;

/// Carats (`ct`), a unit of `Mass`: `n` of them is `n · 0.0002` base units.
pub const CARAT: Unit<{ SIUnit::scaled("ct", Exact::dec(2, -4), KILOGRAM.unit()) }> = Unit::ONE;

/// Daltons (`Da`), a unit of `Mass`: `n` of them is `n · 1.6605390666e-27` base units.
pub const DALTON: Unit<{ SIUnit::scaled("Da", Exact::dec(16605390666, -37), KILOGRAM.unit()) }> =
    Unit::ONE;

/// Electron masses (`mₑ`), a unit of `Mass`: `n` of them is `n · 9.1093837015e-31` base units.
pub const ELECTRON_MASS: Unit<
    { SIUnit::scaled("mₑ", Exact::dec(91093837015, -41), KILOGRAM.unit()) },
> = Unit::ONE;

/// Proton masses (`mₚ`), a unit of `Mass`: `n` of them is `n · 1.67262192369e-27` base units.
pub const PROTON_MASS: Unit<
    { SIUnit::scaled("mₚ", Exact::dec(167262192369, -38), KILOGRAM.unit()) },
> = Unit::ONE;

/// Grains (`gr`), a unit of `Mass`: `n` of them is `n · 6.479891e-05` base units.
pub const GRAIN: Unit<{ SIUnit::scaled("gr", Exact::dec(6479891, -11), KILOGRAM.unit()) }> =
    Unit::ONE;

/// Hundredweight (long) (`cwt long`), a unit of `Mass`: `n` of them is `n · 50.80234544` base units.
pub const HUNDREDWEIGHT_LONG: Unit<
    { SIUnit::scaled("cwt long", Exact::dec(5080234544, -8), KILOGRAM.unit()) },
> = Unit::ONE;

/// Hundredweight (short) (`cwt short`), a unit of `Mass`: `n` of them is `n · 45.359237` base units.
pub const HUNDREDWEIGHT_SHORT: Unit<
    { SIUnit::scaled("cwt short", Exact::dec(45359237, -6), KILOGRAM.unit()) },
> = Unit::ONE;

/// Ounces (`oz`), a unit of `Mass`: `n` of them is `n · 0.028349523125` base units.
pub const OUNCE: Unit<{ SIUnit::scaled("oz", Exact::dec(28349523125, -12), KILOGRAM.unit()) }> =
    Unit::ONE;

/// Troy ounces (`oz t`), a unit of `Mass`: `n` of them is `n · 0.0311034768` base units.
pub const OUNCE_TROY: Unit<
    { SIUnit::scaled("oz t", Exact::dec(311034768, -10), KILOGRAM.unit()) },
> = Unit::ONE;

/// Pennyweight (`dwt`), a unit of `Mass`: `n` of them is `n · 0.00155517384` base units.
pub const PENNYWEIGHT: Unit<
    { SIUnit::scaled("dwt", Exact::dec(155517384, -11), KILOGRAM.unit()) },
> = Unit::ONE;

/// Pounds (`lb`), a unit of `Mass`: `n` of them is `n · 0.45359237` base units.
pub const POUND: Unit<{ SIUnit::scaled("lb", Exact::dec(45359237, -8), KILOGRAM.unit()) }> =
    Unit::ONE;

/// Troy pounds (`lb t`), a unit of `Mass`: `n` of them is `n · 0.3732417216` base units.
pub const POUND_TROY: Unit<
    { SIUnit::scaled("lb t", Exact::dec(3732417216, -10), KILOGRAM.unit()) },
> = Unit::ONE;

/// Slugs (`slug`), a unit of `Mass`: `n` of them is `n · 0.45359237 * 9.80665 / 0.3048` base units.
pub const SLUG: Unit<
    {
        SIUnit::scaled(
            "slug",
            Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Assay tons (`AT`), a unit of `Mass`: `n` of them is `n · 0.175 / 6.0` base units.
pub const TON_ASSAY: Unit<
    { SIUnit::scaled("AT", Exact::dec(175, -3) / Exact::int(6), KILOGRAM.unit()) },
> = Unit::ONE;

/// Long tons (`2240 lb`), a unit of `Mass`: `n` of them is `n · 1016.0469088` base units.
pub const TON_LONG: Unit<
    { SIUnit::scaled("2240 lb", Exact::dec(10160469088, -7), KILOGRAM.unit()) },
> = Unit::ONE;

/// Short tons (`2000 lb`), a unit of `Mass`: `n` of them is `n · 907.18474` base units.
pub const TON_SHORT: Unit<
    { SIUnit::scaled("2000 lb", Exact::dec(90718474, -5), KILOGRAM.unit()) },
> = Unit::ONE;

/// Tons (`t`), a unit of `Mass`: `n` of them is `n · 1000.0` base units.
pub const TON: Unit<{ SIUnit::scaled("t", Exact::int(1000), KILOGRAM.unit()) }> = Unit::ONE;

/// Librae (`libra`), a unit of `Mass`: `n` of them is `n · 0.3289` base units.
pub const LIBRA: Unit<{ SIUnit::scaled("libra", Exact::dec(3289, -4), KILOGRAM.unit()) }> =
    Unit::ONE;

/// Deunces (`deunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 11.0 / 12.0` base units.
pub const MASS_DEUNX: Unit<
    {
        SIUnit::scaled(
            "deunx",
            Exact::dec(3289, -4) * Exact::int(11) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Dextantes (`dextans`), a unit of `Mass`: `n` of them is `n · 0.3289 * 10.0 / 12.0` base units.
pub const MASS_DEXTANS: Unit<
    {
        SIUnit::scaled(
            "dextans",
            Exact::dec(3289, -4) * Exact::int(10) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Dodrantes (`dodrans`), a unit of `Mass`: `n` of them is `n · 0.246675` base units.
pub const MASS_DODRANS: Unit<
    { SIUnit::scaled("dodrans", Exact::dec(246675, -6), KILOGRAM.unit()) },
> = Unit::ONE;

/// Bessis (`bes`), a unit of `Mass`: `n` of them is `n · 0.3289 * 8.0 / 12.0` base units.
pub const MASS_BES: Unit<
    {
        SIUnit::scaled(
            "bes",
            Exact::dec(3289, -4) * Exact::int(8) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Septunces (`septunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 7.0 / 12.0` base units.
pub const MASS_SEPTUNX: Unit<
    {
        SIUnit::scaled(
            "septunx",
            Exact::dec(3289, -4) * Exact::int(7) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Semisses (`semis`), a unit of `Mass`: `n` of them is `n · 0.16445` base units.
pub const MASS_SEMIS: Unit<{ SIUnit::scaled("semis", Exact::dec(16445, -5), KILOGRAM.unit()) }> =
    Unit::ONE;

/// Quincunx (`quincunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 5.0 / 12.0` base units.
pub const MASS_QUINCUNX: Unit<
    {
        SIUnit::scaled(
            "quincunx",
            Exact::dec(3289, -4) * Exact::int(5) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Trientes (`triens`), a unit of `Mass`: `n` of them is `n · 0.3289 / 3.0` base units.
pub const MASS_TRIENS: Unit<
    {
        SIUnit::scaled(
            "triens",
            Exact::dec(3289, -4) / Exact::int(3),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Quadrantes teruncius (`quadrans teruncius`), a unit of `Mass`: `n` of them is `n · 0.082225` base units.
pub const QUADRANS_TERUNCIUS: Unit<
    { SIUnit::scaled("quadrans teruncius", Exact::dec(82225, -6), KILOGRAM.unit()) },
> = Unit::ONE;

/// Sextantes (`sextans`), a unit of `Mass`: `n` of them is `n · 0.3289 / 6.0` base units.
pub const MASS_SEXTANS: Unit<
    {
        SIUnit::scaled(
            "sextans",
            Exact::dec(3289, -4) / Exact::int(6),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Sescunciae (`sescuncia`), a unit of `Mass`: `n` of them is `n · 0.0411125` base units.
pub const SESCUNCIA: Unit<
    { SIUnit::scaled("sescuncia", Exact::dec(411125, -7), KILOGRAM.unit()) },
> = Unit::ONE;

/// Unciae (`uncia`), a unit of `Mass`: `n` of them is `n · 0.3289 / 12.0` base units.
pub const MASS_UNCIA: Unit<
    {
        SIUnit::scaled(
            "uncia",
            Exact::dec(3289, -4) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Semunciae (`semuncia`), a unit of `Mass`: `n` of them is `n · 0.3289 / 24.0` base units.
pub const SEMUNCIA: Unit<
    {
        SIUnit::scaled(
            "semuncia",
            Exact::dec(3289, -4) / Exact::int(24),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Duella (`duella`), a unit of `Mass`: `n` of them is `n · 0.3289 / 36.0` base units.
pub const DUELLA: Unit<
    {
        SIUnit::scaled(
            "duella",
            Exact::dec(3289, -4) / Exact::int(36),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Scilici (`sicilicus`), a unit of `Mass`: `n` of them is `n · 0.3289 / 48.0` base units.
pub const MASS_SICILICUS: Unit<
    {
        SIUnit::scaled(
            "sicilicus",
            Exact::dec(3289, -4) / Exact::int(48),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Sextulae (`sextula`), a unit of `Mass`: `n` of them is `n · 0.3289 / 72.0` base units.
pub const MASS_SEXTULA: Unit<
    {
        SIUnit::scaled(
            "sextula",
            Exact::dec(3289, -4) / Exact::int(72),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Semisextulae (`semisextula`), a unit of `Mass`: `n` of them is `n · 0.3289 / 144.0` base units.
pub const SEMISEXTULA: Unit<
    {
        SIUnit::scaled(
            "semisextula",
            Exact::dec(3289, -4) / Exact::int(144),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Scrupula (`scrupulum`), a unit of `Mass`: `n` of them is `n · 0.3289 / 288.0` base units.
pub const MASS_SCRUPULUM: Unit<
    {
        SIUnit::scaled(
            "scrupulum",
            Exact::dec(3289, -4) / Exact::int(288),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Oboli (`obolus`), a unit of `Mass`: `n` of them is `n · 0.3289 / 576.0` base units.
pub const OBOLUS: Unit<
    {
        SIUnit::scaled(
            "obolus",
            Exact::dec(3289, -4) / Exact::int(576),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Siliquae (`siliqua`), a unit of `Mass`: `n` of them is `n · 0.3289 / 1728.0` base units.
pub const SILIQUA: Unit<
    {
        SIUnit::scaled(
            "siliqua",
            Exact::dec(3289, -4) / Exact::int(1728),
            KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// A time, in `N`s of the unit `U`: by default its base unit, `SECOND`.
pub type Time<N, const U: SIUnit = { SECOND.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;

/// Seconds (sidereal) (`s (sidereal)`), a unit of `Time`: `n` of them is `n · 0.9972696` base units.
pub const SECOND_SIDEREAL: Unit<
    { SIUnit::scaled("s (sidereal)", Exact::dec(9972696, -7), SECOND.unit()) },
> = Unit::ONE;

/// Days (`d`), a unit of `Time`: `n` of them is `n · 86400.0` base units.
pub const DAY: Unit<{ SIUnit::scaled("d", Exact::int(86400), SECOND.unit()) }> = Unit::ONE;

/// Days (sidereal) (`d (sidereal)`), a unit of `Time`: `n` of them is `n · 86164.09` base units.
pub const DAY_SIDEREAL: Unit<
    { SIUnit::scaled("d (sidereal)", Exact::dec(8616409, -2), SECOND.unit()) },
> = Unit::ONE;

/// Hours (`h`), a unit of `Time`: `n` of them is `n · 3600.0` base units.
pub const HOUR: Unit<{ SIUnit::scaled("h", Exact::int(3600), SECOND.unit()) }> = Unit::ONE;

/// Hours (sidereal) (`h (sidereal)`), a unit of `Time`: `n` of them is `n · 3590.17` base units.
pub const HOUR_SIDEREAL: Unit<
    { SIUnit::scaled("h (sidereal)", Exact::dec(359017, -2), SECOND.unit()) },
> = Unit::ONE;

/// Minutes (`min`), a unit of `Time`: `n` of them is `n · 60.0` base units.
pub const MINUTE: Unit<{ SIUnit::scaled("min", Exact::int(60), SECOND.unit()) }> = Unit::ONE;

/// Shakes (`10.0 ns`), a unit of `Time`: `n` of them is `n · 1e-08` base units.
pub const SHAKE: Unit<{ SIUnit::scaled("10.0 ns", Exact::dec(1, -8), SECOND.unit()) }> = Unit::ONE;

/// Years (`a`), a unit of `Time`: `n` of them is `n · 31536000.0` base units.
pub const YEAR: Unit<{ SIUnit::scaled("a", Exact::int(31536000), SECOND.unit()) }> = Unit::ONE;

/// Years (sidereal) (`a (sidereal)`), a unit of `Time`: `n` of them is `n · 31558150.0` base units.
pub const YEAR_SIDEREAL: Unit<
    { SIUnit::scaled("a (sidereal)", Exact::int(31558150), SECOND.unit()) },
> = Unit::ONE;

/// Years (tropical) (`a (tropical)`), a unit of `Time`: `n` of them is `n · 31556930.0` base units.
pub const YEAR_TROPICAL: Unit<
    { SIUnit::scaled("a (tropical)", Exact::int(31556930), SECOND.unit()) },
> = Unit::ONE;

/// An instant, in `N`s of the unit `U`: by default its base unit, `UNIX_SECOND`.
pub type Instant<N, const U: SIUnit = { UNIX_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Seconds of Unix time (`s Unix`), a unit of `Instant`: the coherent unit, a product of base units.
pub const UNIX_SECOND: Unit<{ SECOND.unit() }, BASE> = Unit::ONE;

/// Milliseconds of Unix time (`ms Unix`), a unit of `Instant`: `MILLI * UNIX_SECOND`.
pub const UNIX_MILLISECOND: Unit<{ (MILLI * UNIX_SECOND).unit() }> = Unit::ONE;

/// Seconds of GPS time (`s GPS`), a unit of `Instant`: `n` of them is `n · 1.0 + 315964800.0` base units.
pub const GPS_SECOND: Unit<
    {
        SIUnit::offset(
            "s GPS",
            Exact::int(1),
            Exact::int(315964800),
            UNIX_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// Julian date (`JD`), a unit of `Instant`: `n` of them is `n · 86400.0 + -2440587.5 * 86400.0` base units.
pub const JULIAN_DATE: Unit<
    {
        SIUnit::offset(
            "JD",
            Exact::int(86400),
            -Exact::dec(24405875, -1) * Exact::int(86400),
            UNIX_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// Modified Julian date (`MJD`), a unit of `Instant`: `n` of them is `n · 86400.0 + -40587.0 * 86400.0` base units.
pub const MODIFIED_JULIAN_DATE: Unit<
    {
        SIUnit::offset(
            "MJD",
            Exact::int(86400),
            -Exact::int(40587) * Exact::int(86400),
            UNIX_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// An area, in `N`s of the unit `U`: by default its base unit, `SQUARE_METRE`.
pub type Area<N, const U: SIUnit = { SQUARE_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Square metres (`m²`), a unit of `Area`: the coherent unit, a product of base units.
pub const SQUARE_METRE: Unit<{ (METRE * METRE).unit() }, BASE> = Unit::ONE;

/// Acres (`ac`), a unit of `Area`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0)` base units.
pub const ACRE: Unit<
    {
        SIUnit::scaled(
            "ac",
            Exact::int(43560)
                * (Exact::int(1200) / Exact::int(3937))
                * (Exact::int(1200) / Exact::int(3937)),
            SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Ares (`a`), a unit of `Area`: `n` of them is `n · 100.0` base units.
pub const ARE: Unit<{ SIUnit::scaled("a", Exact::int(100), SQUARE_METRE.unit()) }> = Unit::ONE;

/// Barns (`b`), a unit of `Area`: `n` of them is `n · 1e-28` base units.
pub const BARN: Unit<{ SIUnit::scaled("b", Exact::dec(1, -28), SQUARE_METRE.unit()) }> = Unit::ONE;

/// Circular mils (`cmil`), a unit of `Area`: `n` of them is `n · core::f64::consts::PI / 4.0 * 2.54e-05 * 2.54e-05` base units.
pub const CIRCULAR_MIL: Unit<
    {
        SIUnit::scaled(
            "cmil",
            Exact::PI / Exact::int(4) * Exact::dec(254, -7) * Exact::dec(254, -7),
            SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Hectares (`ha`), a unit of `Area`: `HECTO * ARE`.
pub const HECTARE: Unit<{ (HECTO * ARE).unit() }> = Unit::ONE;

/// Square feet (`ft²`), a unit of `Area`: `n` of them is `n · 0.09290304` base units.
pub const SQUARE_FOOT: Unit<
    { SIUnit::scaled("ft²", Exact::dec(9290304, -8), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Square inches (`in²`), a unit of `Area`: `n` of them is `n · 0.00064516` base units.
pub const SQUARE_INCH: Unit<{ SIUnit::scaled("in²", Exact::dec(64516, -8), SQUARE_METRE.unit()) }> =
    Unit::ONE;

/// Square miles (`mi²`), a unit of `Area`: `n` of them is `n · 2589988.110336` base units.
pub const SQUARE_MILE: Unit<
    { SIUnit::scaled("mi²", Exact::dec(2589988110336, -6), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Square yards (`yd²`), a unit of `Area`: `n` of them is `n · 0.83612736` base units.
pub const SQUARE_YARD: Unit<
    { SIUnit::scaled("yd²", Exact::dec(83612736, -8), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Saltus (`saltus`), a unit of `Area`: `n` of them is `n · 2018672.64` base units.
pub const SALTUS: Unit<
    { SIUnit::scaled("saltus", Exact::dec(201867264, -2), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Centuriae (`centuria`), a unit of `Area`: `n` of them is `n · 504668.16` base units.
pub const CENTURIA: Unit<
    { SIUnit::scaled("centuria", Exact::dec(50466816, -2), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Heredia (`heredium`), a unit of `Area`: `n` of them is `n · 5046.6816` base units.
pub const HEREDIUM: Unit<
    { SIUnit::scaled("heredium", Exact::dec(50466816, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Jugera (`jugerum`), a unit of `Area`: `n` of them is `n · 2523.3408` base units.
pub const JUGERUM: Unit<
    { SIUnit::scaled("jugerum", Exact::dec(25233408, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Deunces (`deunx`), a unit of `Area`: `n` of them is `n · 2313.0624` base units.
pub const AREA_DEUNX: Unit<
    { SIUnit::scaled("deunx", Exact::dec(23130624, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Dextantes (`dextans`), a unit of `Area`: `n` of them is `n · 2102.784` base units.
pub const AREA_DEXTANS: Unit<
    { SIUnit::scaled("dextans", Exact::dec(2102784, -3), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Dodrantes (`dodrans`), a unit of `Area`: `n` of them is `n · 1892.5056` base units.
pub const AREA_DODRANS: Unit<
    { SIUnit::scaled("dodrans", Exact::dec(18925056, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Besses (`bes`), a unit of `Area`: `n` of them is `n · 1682.2272` base units.
pub const AREA_BES: Unit<{ SIUnit::scaled("bes", Exact::dec(16822272, -4), SQUARE_METRE.unit()) }> =
    Unit::ONE;

/// Septunces (`septunx`), a unit of `Area`: `n` of them is `n · 1471.9488` base units.
pub const AREA_SEPTUNX: Unit<
    { SIUnit::scaled("septunx", Exact::dec(14719488, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Semisses (`semis`), a unit of `Area`: `n` of them is `n · 1261.6704` base units.
pub const AREA_SEMIS: Unit<
    { SIUnit::scaled("semis", Exact::dec(12616704, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Acta quadratus (`actus quadratus`), a unit of `Area`: `n` of them is `n · 1261.6704` base units.
pub const ACTUS_QUADRATUS: Unit<
    {
        SIUnit::scaled(
            "actus quadratus",
            Exact::dec(12616704, -4),
            SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Quincunces (`quincunx`), a unit of `Area`: `n` of them is `n · 1051.392` base units.
pub const AREA_QUINCUNX: Unit<
    { SIUnit::scaled("quincunx", Exact::dec(1051392, -3), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Trientes (`triens`), a unit of `Area`: `n` of them is `n · 841.1136` base units.
pub const AREA_TRIENS: Unit<
    { SIUnit::scaled("triens", Exact::dec(8411136, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Quadrantes (`quadrans`), a unit of `Area`: `n` of them is `n · 630.8352` base units.
pub const QUADRANS: Unit<
    { SIUnit::scaled("quadrans", Exact::dec(6308352, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Sextantes (`sextans`), a unit of `Area`: `n` of them is `n · 420.5568` base units.
pub const AREA_SEXTANS: Unit<
    { SIUnit::scaled("sextans", Exact::dec(4205568, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Climata (`clima`), a unit of `Area`: `n` of them is `n · 315.4176` base units.
pub const CLIMA: Unit<{ SIUnit::scaled("clima", Exact::dec(3154176, -4), SQUARE_METRE.unit()) }> =
    Unit::ONE;

/// Unciae (`uncia`), a unit of `Area`: `n` of them is `n · 210.2784` base units.
pub const AREA_UNCIA: Unit<
    { SIUnit::scaled("uncia", Exact::dec(2102784, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Semiunciae (`semiuncia`), a unit of `Area`: `n` of them is `n · 105.1392` base units.
pub const SEMIUNCIA: Unit<
    { SIUnit::scaled("semiuncia", Exact::dec(1051392, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Siculi (`sicilicus`), a unit of `Area`: `n` of them is `n · 52.5696` base units.
pub const AREA_SICILICUS: Unit<
    { SIUnit::scaled("sicilicus", Exact::dec(525696, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Acta simplex (`actus simplex`), a unit of `Area`: `n` of them is `n · 42.05568` base units.
pub const ACTUS_SIMPLEX: Unit<
    {
        SIUnit::scaled(
            "actus simplex",
            Exact::dec(4205568, -5),
            SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Sextulae (`sextula`), a unit of `Area`: `n` of them is `n · 35.0464` base units.
pub const AREA_SEXTULA: Unit<
    { SIUnit::scaled("sextula", Exact::dec(350464, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Duo scrupula (`duo scrupulum`), a unit of `Area`: `n` of them is `n · 17.5232` base units.
pub const DUO_SCRUPULUM: Unit<
    { SIUnit::scaled("duo scrupulum", Exact::dec(175232, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Scrupula (`scrupulum`), a unit of `Area`: `n` of them is `n · 8.7616` base units.
pub const AREA_SCRUPULUM: Unit<
    { SIUnit::scaled("scrupulum", Exact::dec(87616, -4), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// Dimidium scupula (`dimidium scrupulum`), a unit of `Area`: `n` of them is `n · 4.3808` base units.
pub const DIMIDIUM_SCRUPULUM: Unit<
    {
        SIUnit::scaled(
            "dimidium scrupulum",
            Exact::dec(43808, -4),
            SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Pedes quadratus (`pes quadratus`), a unit of `Area`: `n` of them is `n · 0.087616` base units.
pub const PES_QUADRATUS: Unit<
    { SIUnit::scaled("pes quadratus", Exact::dec(87616, -6), SQUARE_METRE.unit()) },
> = Unit::ONE;

/// A volume, in `N`s of the unit `U`: by default its base unit, `CUBIC_METRE`.
pub type Volume<N, const U: SIUnit = { CUBIC_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Cubic metres (`m³`), a unit of `Volume`: the coherent unit, a product of base units.
pub const CUBIC_METRE: Unit<{ (METRE * METRE * METRE).unit() }, BASE> = Unit::ONE;

/// Acre-feet (`ac · ft`), a unit of `Volume`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0) * (1200.0 / 3937.0)` base units.
pub const ACRE_FOOT: Unit<
    {
        SIUnit::scaled(
            "ac · ft",
            Exact::int(43560)
                * (Exact::int(1200) / Exact::int(3937))
                * (Exact::int(1200) / Exact::int(3937))
                * (Exact::int(1200) / Exact::int(3937)),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Barrels (`bbl`), a unit of `Volume`: `n` of them is `n · 0.158987294928` base units.
pub const BARREL: Unit<
    { SIUnit::scaled("bbl", Exact::dec(158987294928, -12), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Bushels (`bu`), a unit of `Volume`: `n` of them is `n · 0.03523907016688` base units.
pub const BUSHEL: Unit<
    { SIUnit::scaled("bu", Exact::dec(3523907016688, -14), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Cords (`cords`), a unit of `Volume`: `n` of them is `n · 128.0 * 0.3048 * 0.3048 * 0.3048` base units.
pub const CORD: Unit<
    {
        SIUnit::scaled(
            "cords",
            Exact::int(128) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Cubic feet (`ft³`), a unit of `Volume`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048` base units.
pub const CUBIC_FOOT: Unit<
    {
        SIUnit::scaled(
            "ft³",
            Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Cubic inches (`in³`), a unit of `Volume`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254` base units.
pub const CUBIC_INCH: Unit<
    {
        SIUnit::scaled(
            "in³",
            Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Cubic miles (`mi³`), a unit of `Volume`: `n` of them is `n · 1609.344 * 1609.344 * 1609.344` base units.
pub const CUBIC_MILE: Unit<
    {
        SIUnit::scaled(
            "mi³",
            Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Cubic yards (`yd³`), a unit of `Volume`: `n` of them is `n · 0.9144 * 0.9144 * 0.9144` base units.
pub const CUBIC_YARD: Unit<
    {
        SIUnit::scaled(
            "yd³",
            Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Cups (`cup`), a unit of `Volume`: `n` of them is `n · 0.0002365882365` base units.
pub const CUP: Unit<{ SIUnit::scaled("cup", Exact::dec(2365882365, -13), CUBIC_METRE.unit()) }> =
    Unit::ONE;

/// Fluid ounces (`fl oz`), a unit of `Volume`: `n` of them is `n · 2.95735295625e-05` base units.
pub const FLUID_OUNCE: Unit<
    { SIUnit::scaled("fl oz", Exact::dec(295735295625, -16), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Imperial fluid ounces (`fl oz (UK)`), a unit of `Volume`: `n` of them is `n · 2.84130625e-05` base units.
pub const FLUID_OUNCE_IMPERIAL: Unit<
    { SIUnit::scaled("fl oz (UK)", Exact::dec(284130625, -13), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Imperial gallons (`gal (UK)`), a unit of `Volume`: `n` of them is `n · 0.00454609` base units.
pub const GALLON_IMPERIAL: Unit<
    { SIUnit::scaled("gal (UK)", Exact::dec(454609, -8), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Gallons (`gal`), a unit of `Volume`: `n` of them is `n · 0.003785411784` base units.
pub const GALLON: Unit<{ SIUnit::scaled("gal", Exact::dec(3785411784, -12), CUBIC_METRE.unit()) }> =
    Unit::ONE;

/// Imperial gills (`gi (UK)`), a unit of `Volume`: `n` of them is `n · 0.0001420653125` base units.
pub const GILL_IMPERIAL: Unit<
    { SIUnit::scaled("gi (UK)", Exact::dec(1420653125, -13), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Gills (`gi`), a unit of `Volume`: `n` of them is `n · 0.00011829411825` base units.
pub const GILL: Unit<{ SIUnit::scaled("gi", Exact::dec(11829411825, -14), CUBIC_METRE.unit()) }> =
    Unit::ONE;

/// Litres (`L`), a unit of `Volume`: `n` of them is `n · 0.001` base units.
pub const LITRE: Unit<{ SIUnit::scaled("L", Exact::dec(1, -3), CUBIC_METRE.unit()) }> = Unit::ONE;

/// Pecks (`pk`), a unit of `Volume`: `n` of them is `n · 0.00880976754172` base units.
pub const PECK: Unit<{ SIUnit::scaled("pk", Exact::dec(880976754172, -14), CUBIC_METRE.unit()) }> =
    Unit::ONE;

/// Dry pints (`dry pt`), a unit of `Volume`: `n` of them is `n · 0.0005506104713575` base units.
pub const PINT_DRY: Unit<
    { SIUnit::scaled("dry pt", Exact::dec(5506104713575, -16), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Liquid pints (`liq pt`), a unit of `Volume`: `n` of them is `n · 0.000473176473` base units.
pub const PINT_LIQUID: Unit<
    { SIUnit::scaled("liq pt", Exact::dec(473176473, -12), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Dry quarts (`dry qt`), a unit of `Volume`: `n` of them is `n · 0.001101220942715` base units.
pub const QUART_DRY: Unit<
    { SIUnit::scaled("dry qt", Exact::dec(1101220942715, -15), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Liquid quarts (`liq qt`), a unit of `Volume`: `n` of them is `n · 0.000946352946` base units.
pub const QUART_LIQUID: Unit<
    { SIUnit::scaled("liq qt", Exact::dec(946352946, -12), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Steres (`st`), a unit of `Volume`: `n` of them is `n · 1.0` base units.
pub const STERE: Unit<{ SIUnit::scaled("st", Exact::int(1), CUBIC_METRE.unit()) }> = Unit::ONE;

/// Tablespoons (`tbsp`), a unit of `Volume`: `n` of them is `n · 1.478676478125e-05` base units.
pub const TABLESPOON: Unit<
    { SIUnit::scaled("tbsp", Exact::dec(1478676478125, -17), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Teaspoons (`tsp`), a unit of `Volume`: `n` of them is `n · 4.92892159375e-06` base units.
pub const TEASPOON: Unit<
    { SIUnit::scaled("tsp", Exact::dec(492892159375, -17), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Register tons (`RT`), a unit of `Volume`: `n` of them is `n · 100.0 * 0.3048 * 0.3048 * 0.3048` base units.
pub const REGISTER_TON: Unit<
    {
        SIUnit::scaled(
            "RT",
            Exact::int(100) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Culei (`culeus`), a unit of `Volume`: `n` of them is `n · 0.51868672` base units.
pub const CULEUS: Unit<{ SIUnit::scaled("culeus", Exact::dec(51868672, -8), CUBIC_METRE.unit()) }> =
    Unit::ONE;

/// Amphora quadrantal (`amphora quadrantal`), a unit of `Volume`: `n` of them is `n · 0.025934336` base units.
pub const AMPHORA_QUADRANTAL: Unit<
    {
        SIUnit::scaled(
            "amphora quadrantal",
            Exact::dec(25934336, -9),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Urnae (`urna`), a unit of `Volume`: `n` of them is `n · 0.012967168` base units.
pub const URNA: Unit<{ SIUnit::scaled("urna", Exact::dec(12967168, -9), CUBIC_METRE.unit()) }> =
    Unit::ONE;

/// Modii castrensis (`modius castrensis`), a unit of `Volume`: `n` of them is `n · 0.012967168` base units.
pub const MODIUS_CASTRENSIS: Unit<
    {
        SIUnit::scaled(
            "modius castrensis",
            Exact::dec(12967168, -9),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Modii (`modius`), a unit of `Volume`: `n` of them is `n · 0.003241792 * 16.0 / 6.0` base units.
pub const MODIUS: Unit<
    {
        SIUnit::scaled(
            "modius",
            Exact::dec(3241792, -9) * Exact::int(16) / Exact::int(6),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Semimodii (`semimodius`), a unit of `Volume`: `n` of them is `n · 0.003241792 * 8.0 / 6.0` base units.
pub const SEMIMODIUS: Unit<
    {
        SIUnit::scaled(
            "semimodius",
            Exact::dec(3241792, -9) * Exact::int(8) / Exact::int(6),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Congii (`congius`), a unit of `Volume`: `n` of them is `n · 0.003241792` base units.
pub const CONGIUS: Unit<
    { SIUnit::scaled("congius", Exact::dec(3241792, -9), CUBIC_METRE.unit()) },
> = Unit::ONE;

/// Sextarii (`sextarius`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 6.0` base units.
pub const SEXTARIUS: Unit<
    {
        SIUnit::scaled(
            "sextarius",
            Exact::dec(3241792, -9) / Exact::int(6),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Heminae (`hemina`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 12.0` base units.
pub const HEMINA: Unit<
    {
        SIUnit::scaled(
            "hemina",
            Exact::dec(3241792, -9) / Exact::int(12),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Quartarii (`quartarius`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 24.0` base units.
pub const QUARTARIUS: Unit<
    {
        SIUnit::scaled(
            "quartarius",
            Exact::dec(3241792, -9) / Exact::int(24),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Acetabula (`acetabulum`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 48.0` base units.
pub const ACETABULUM: Unit<
    {
        SIUnit::scaled(
            "acetabulum",
            Exact::dec(3241792, -9) / Exact::int(48),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Cyathi (`cyathus`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 72.0` base units.
pub const CYATHUS: Unit<
    {
        SIUnit::scaled(
            "cyathus",
            Exact::dec(3241792, -9) / Exact::int(72),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Ligula (`ligula`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 288.0` base units.
pub const LIGULA: Unit<
    {
        SIUnit::scaled(
            "ligula",
            Exact::dec(3241792, -9) / Exact::int(288),
            CUBIC_METRE.unit(),
        )
    },
> = Unit::ONE;

/// A velocity, in `N`s of the unit `U`: by default its base unit, `METRE_PER_SECOND`.
pub type Velocity<N, const U: SIUnit = { METRE_PER_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `Velocity`: a product of base units.
pub const METRE_PER_SECOND: Unit<{ (METRE / SECOND).unit() }, BASE> = Unit::ONE;

/// Knots (`kn`), a unit of `Velocity`: `n` of them is `n · 1852.0 / 3600.0` base units.
pub const KNOT: Unit<
    {
        SIUnit::scaled(
            "kn",
            Exact::int(1852) / Exact::int(3600),
            METRE_PER_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// Atomic units of velocity (`a₀ · Eₕ/ħ`), a unit of `Velocity`: `n` of them is `n · 2187691.26364` base units.
pub const ATOMIC_UNIT_OF_VELOCITY: Unit<
    {
        SIUnit::scaled(
            "a₀ · Eₕ/ħ",
            Exact::dec(218769126364, -5),
            METRE_PER_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// Natural units of velocity (`c`), a unit of `Velocity`: `n` of them is `n · 299792458.0` base units.
pub const NATURAL_UNIT_OF_VELOCITY: Unit<
    { SIUnit::scaled("c", Exact::int(299792458), METRE_PER_SECOND.unit()) },
> = Unit::ONE;

/// Speeds of light in vacuum (`c`), a unit of `Velocity`: `n` of them is `n · 299792458.0` base units.
pub const SPEED_OF_LIGHT: Unit<
    { SIUnit::scaled("c", Exact::int(299792458), METRE_PER_SECOND.unit()) },
> = Unit::ONE;

/// An acceleration, in `N`s of the unit `U`: by default its base unit, `METRE_PER_SECOND_SQUARED`.
pub type Acceleration<
    N,
    const U: SIUnit = { METRE_PER_SECOND_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `Acceleration`: a product of base units.
pub const METRE_PER_SECOND_SQUARED: Unit<{ (METRE / (SECOND * SECOND)).unit() }, BASE> = Unit::ONE;

/// Galileos (`Gal`), a unit of `Acceleration`: `n` of them is `n · 0.01` base units.
pub const GALILEO: Unit<
    { SIUnit::scaled("Gal", Exact::dec(1, -2), METRE_PER_SECOND_SQUARED.unit()) },
> = Unit::ONE;

/// Standard accelerations of gravity (`g₀`), a unit of `Acceleration`: `n` of them is `n · 9.80665` base units.
pub const STANDARD_GRAVITY: Unit<
    {
        SIUnit::scaled(
            "g₀",
            Exact::dec(980665, -5),
            METRE_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// A jerk, in `N`s of the unit `U`: by default its base unit, `METRE_PER_SECOND_CUBED`.
pub type Jerk<N, const U: SIUnit = { METRE_PER_SECOND_CUBED.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `Jerk`: a product of base units.
pub const METRE_PER_SECOND_CUBED: Unit<{ (METRE / (SECOND * SECOND * SECOND)).unit() }, BASE> =
    Unit::ONE;

/// An absement, in `N`s of the unit `U`: by default its base unit, `METRE_SECOND`.
pub type Absement<N, const U: SIUnit = { METRE_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Metre seconds (`m · s`), a unit of `Absement`: the coherent unit, a product of base units.
pub const METRE_SECOND: Unit<{ (METRE * SECOND).unit() }, BASE> = Unit::ONE;

/// Foot seconds (`ft · s`), a unit of `Absement`: `n` of them is `n · 0.3048` base units.
pub const FOOT_SECOND: Unit<
    { SIUnit::scaled("ft · s", Exact::dec(3048, -4), METRE_SECOND.unit()) },
> = Unit::ONE;

/// Inch seconds (`in · s`), a unit of `Absement`: `n` of them is `n · 0.0254` base units.
pub const INCH_SECOND: Unit<
    { SIUnit::scaled("in · s", Exact::dec(254, -4), METRE_SECOND.unit()) },
> = Unit::ONE;

/// Kilometre hours (`km · h`), a unit of `Absement`: `n` of them is `n · 3600000.0` base units.
pub const KILOMETRE_HOUR: Unit<
    { SIUnit::scaled("km · h", Exact::int(3600000), METRE_SECOND.unit()) },
> = Unit::ONE;

/// An inverse velocity, in `N`s of the unit `U`: by default its base unit, `SECOND_PER_METRE`.
pub type InverseVelocity<N, const U: SIUnit = { SECOND_PER_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `InverseVelocity`: a product of base units.
pub const SECOND_PER_METRE: Unit<{ (SECOND / METRE).unit() }, BASE> = Unit::ONE;

/// A force, in `N`s of the unit `U`: by default its base unit, `NEWTON`.
pub type Force<N, const U: SIUnit = { NEWTON.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;

/// Newtons (`N`), a unit of `Force`: the coherent unit, a product of base units.
pub const NEWTON: Unit<{ (METRE * KILOGRAM / (SECOND * SECOND)).unit() }, BASE> = Unit::ONE;

/// Dynes (`dyn`), a unit of `Force`: `n` of them is `n · 1e-05` base units.
pub const DYNE: Unit<{ SIUnit::scaled("dyn", Exact::dec(1, -5), NEWTON.unit()) }> = Unit::ONE;

/// Kilograms-force (`kgf`), a unit of `Force`: `n` of them is `n · 9.80665` base units.
pub const KILOGRAM_FORCE: Unit<{ SIUnit::scaled("kgf", Exact::dec(980665, -5), NEWTON.unit()) }> =
    Unit::ONE;

/// Kips (`kip`), a unit of `Force`: `n` of them is `n · 1000.0 * 0.45359237 * 9.80665` base units.
pub const KIP: Unit<
    {
        SIUnit::scaled(
            "kip",
            Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
            NEWTON.unit(),
        )
    },
> = Unit::ONE;

/// Ounces-force (`ozf`), a unit of `Force`: `n` of them is `n · 0.45359237 * 9.80665 / 16.0` base units.
pub const OUNCE_FORCE: Unit<
    {
        SIUnit::scaled(
            "ozf",
            Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16),
            NEWTON.unit(),
        )
    },
> = Unit::ONE;

/// Poundals (`pdl`), a unit of `Force`: `n` of them is `n · 0.45359237 * 0.3048` base units.
pub const POUNDAL: Unit<
    {
        SIUnit::scaled(
            "pdl",
            Exact::dec(45359237, -8) * Exact::dec(3048, -4),
            NEWTON.unit(),
        )
    },
> = Unit::ONE;

/// Pounds-force (`lbf`), a unit of `Force`: `n` of them is `n · 0.45359237 * 9.80665` base units.
pub const POUND_FORCE: Unit<
    {
        SIUnit::scaled(
            "lbf",
            Exact::dec(45359237, -8) * Exact::dec(980665, -5),
            NEWTON.unit(),
        )
    },
> = Unit::ONE;

/// Tons-force (`2000 lbf`), a unit of `Force`: `n` of them is `n · 2000.0 * 0.45359237 * 9.80665` base units.
pub const TON_FORCE: Unit<
    {
        SIUnit::scaled(
            "2000 lbf",
            Exact::int(2000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
            NEWTON.unit(),
        )
    },
> = Unit::ONE;

/// A momentum, in `N`s of the unit `U`: by default its base unit, `METRE_KILOGRAM_PER_SECOND`.
pub type Momentum<N, const U: SIUnit = { METRE_KILOGRAM_PER_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `Momentum`: a product of base units.
pub const METRE_KILOGRAM_PER_SECOND: Unit<{ (METRE * KILOGRAM / SECOND).unit() }, BASE> = Unit::ONE;

/// An energy, in `N`s of the unit `U`: by default its base unit, `JOULE`.
pub type Energy<N, const U: SIUnit = { JOULE.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;

/// Joules (`J`), a unit of `Energy`: the coherent unit, a product of base units.
pub const JOULE: Unit<{ (METRE * METRE * KILOGRAM / (SECOND * SECOND)).unit() }, BASE> = Unit::ONE;

/// Watt hours (`W · h`), a unit of `Energy`: `n` of them is `n · 3600.0` base units.
pub const WATT_HOUR: Unit<{ SIUnit::scaled("W · h", Exact::int(3600), JOULE.unit()) }> = Unit::ONE;

/// Electronvolts (`eV`), a unit of `Energy`: `n` of them is `n · 1.602176634e-19` base units.
pub const ELECTRONVOLT: Unit<{ SIUnit::scaled("eV", Exact::dec(1602176634, -28), JOULE.unit()) }> =
    Unit::ONE;

/// Hartrees (`Eₕ`), a unit of `Energy`: `n` of them is `n · 4.3597447222071e-18` base units.
pub const HARTREE: Unit<{ SIUnit::scaled("Eₕ", Exact::dec(43597447222071, -31), JOULE.unit()) }> =
    Unit::ONE;

/// British thermal units (IT) (`Btu (IT)`), a unit of `Energy`: `n` of them is `n · 1055.05585262` base units.
pub const BTU_IT: Unit<{ SIUnit::scaled("Btu (IT)", Exact::dec(105505585262, -8), JOULE.unit()) }> =
    Unit::ONE;

/// British thermal units (`Btu`), a unit of `Energy`: `n` of them is `n · 4184.0 * 0.45359237 / 1.8` base units.
pub const BTU: Unit<
    {
        SIUnit::scaled(
            "Btu",
            Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1),
            JOULE.unit(),
        )
    },
> = Unit::ONE;

/// Calories (IT) (`cal (IT)`), a unit of `Energy`: `n` of them is `n · 4.1868` base units.
pub const CALORIE_IT: Unit<{ SIUnit::scaled("cal (IT)", Exact::dec(41868, -4), JOULE.unit()) }> =
    Unit::ONE;

/// Calories (`cal`), a unit of `Energy`: `n` of them is `n · 4.184` base units.
pub const CALORIE: Unit<{ SIUnit::scaled("cal", Exact::dec(4184, -3), JOULE.unit()) }> = Unit::ONE;

/// Calories (IT) (`Cal (IT)`), a unit of `Energy`: `n` of them is `n · 4186.8` base units.
pub const CALORIE_IT_NUTRITION: Unit<
    { SIUnit::scaled("Cal (IT)", Exact::dec(41868, -1), JOULE.unit()) },
> = Unit::ONE;

/// Calories (`Cal`), a unit of `Energy`: `n` of them is `n · 4184.0` base units.
pub const CALORIE_NUTRITION: Unit<{ SIUnit::scaled("Cal", Exact::int(4184), JOULE.unit()) }> =
    Unit::ONE;

/// Ergs (`erg`), a unit of `Energy`: `n` of them is `n · 1e-07` base units.
pub const ERG: Unit<{ SIUnit::scaled("erg", Exact::dec(1, -7), JOULE.unit()) }> = Unit::ONE;

/// Foot poundals (`ft · pdl`), a unit of `Energy`: `n` of them is `n · 0.3048 * 0.45359237 * 0.3048` base units.
pub const FOOT_POUNDAL: Unit<
    {
        SIUnit::scaled(
            "ft · pdl",
            Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4),
            JOULE.unit(),
        )
    },
> = Unit::ONE;

/// Foot pounds-force (`ft · lbf`), a unit of `Energy`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` base units.
pub const FOOT_POUND: Unit<
    {
        SIUnit::scaled(
            "ft · lbf",
            Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
            JOULE.unit(),
        )
    },
> = Unit::ONE;

/// Quads (`10¹⁵ Btu (IT)`), a unit of `Energy`: `n` of them is `n · 1.05505585262e18` base units.
pub const QUAD: Unit<
    {
        SIUnit::scaled(
            "10¹⁵ Btu (IT)",
            Exact::int(1055055852620000000),
            JOULE.unit(),
        )
    },
> = Unit::ONE;

/// Therms (EC) (`thm (EC)`), a unit of `Energy`: `n` of them is `n · 105506000.0` base units.
pub const THERM_EC: Unit<{ SIUnit::scaled("thm (EC)", Exact::int(105506000), JOULE.unit()) }> =
    Unit::ONE;

/// Therms (`thm`), a unit of `Energy`: `n` of them is `n · 105480400.0` base units.
pub const THERM_US: Unit<{ SIUnit::scaled("thm", Exact::int(105480400), JOULE.unit()) }> =
    Unit::ONE;

/// Tons of TNT (`t of TNT`), a unit of `Energy`: `n` of them is `n · 4184000000.0` base units.
pub const TON_TNT: Unit<{ SIUnit::scaled("t of TNT", Exact::int(4184000000), JOULE.unit()) }> =
    Unit::ONE;

/// Watt seconds (`W · s`), a unit of `Energy`: `n` of them is `n · 1.0` base units.
pub const WATT_SECOND: Unit<{ SIUnit::scaled("W · s", Exact::int(1), JOULE.unit()) }> = Unit::ONE;

/// An available energy, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_PER_SECOND_SQUARED`.
pub type AvailableEnergy<
    N,
    const U: SIUnit = { METRE_SQUARED_PER_SECOND_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `AvailableEnergy`: a product of base units.
pub const METRE_SQUARED_PER_SECOND_SQUARED: Unit<
    { (METRE * METRE / (SECOND * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// A power, in `N`s of the unit `U`: by default its base unit, `WATT`.
pub type Power<N, const U: SIUnit = { WATT.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;

/// Watts (`W`), a unit of `Power`: the coherent unit, a product of base units.
pub const WATT: Unit<{ (METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND)).unit() }, BASE> =
    Unit::ONE;

/// Horsepower (`hp`), a unit of `Power`: `n` of them is `n · 550.0 * 0.3048 * 0.45359237 * 9.80665` base units.
pub const HORSEPOWER: Unit<
    {
        SIUnit::scaled(
            "hp",
            Exact::int(550)
                * Exact::dec(3048, -4)
                * Exact::dec(45359237, -8)
                * Exact::dec(980665, -5),
            WATT.unit(),
        )
    },
> = Unit::ONE;

/// Horsepower (boiler) (`hp (S)`), a unit of `Power`: `n` of them is `n · 9809.5` base units.
pub const HORSEPOWER_BOILER: Unit<
    { SIUnit::scaled("hp (S)", Exact::dec(98095, -1), WATT.unit()) },
> = Unit::ONE;

/// Horsepower (electric) (`hp (E)`), a unit of `Power`: `n` of them is `n · 746.0` base units.
pub const HORSEPOWER_ELECTRIC: Unit<{ SIUnit::scaled("hp (E)", Exact::int(746), WATT.unit()) }> =
    Unit::ONE;

/// Metric horsepower (`hp (M)`), a unit of `Power`: `n` of them is `n · 75.0 * 9.80665` base units.
pub const HORSEPOWER_METRIC: Unit<
    {
        SIUnit::scaled(
            "hp (M)",
            Exact::int(75) * Exact::dec(980665, -5),
            WATT.unit(),
        )
    },
> = Unit::ONE;

/// Horsepower (Imperial) (`hp (I)`), a unit of `Power`: `n` of them is `n · 745.7` base units.
pub const HORSEPOWER_IMPERIAL: Unit<
    { SIUnit::scaled("hp (I)", Exact::dec(7457, -1), WATT.unit()) },
> = Unit::ONE;

/// Hydraulic horsepower (`hp (hydraulic)`), a unit of `Power`: `n` of them is `n · 746.043` base units.
pub const HYDRAULIC_HORSEPOWER: Unit<
    { SIUnit::scaled("hp (hydraulic)", Exact::dec(746043, -3), WATT.unit()) },
> = Unit::ONE;

/// Decibels-watt (`dBW`), a unit of `Power`: `n` of them is `1.0 · 10.0^(n / 10.0)` base units.
pub const DECIBEL_WATT: Unit<
    {
        SIUnit::from_log(
            "dBW",
            Exact::int(10),
            Exact::int(10),
            Exact::int(1),
            WATT.unit(),
        )
    },
> = Unit::ONE;

/// Decibels-milliwatt (`dBm`), a unit of `Power`: `n` of them is `0.001 · 10.0^(n / 10.0)` base units.
pub const DECIBEL_MILLIWATT: Unit<
    {
        SIUnit::from_log(
            "dBm",
            Exact::int(10),
            Exact::int(10),
            Exact::dec(1, -3),
            WATT.unit(),
        )
    },
> = Unit::ONE;

/// A power rate, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_KILOGRAM_PER_SECOND_POW4`.
pub type PowerRate<
    N,
    const U: SIUnit = { METRE_SQUARED_KILOGRAM_PER_SECOND_POW4.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `PowerRate`: a product of base units.
pub const METRE_SQUARED_KILOGRAM_PER_SECOND_POW4: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// An action, in `N`s of the unit `U`: by default its base unit, `JOULE_SECOND`.
pub type Action<N, const U: SIUnit = { JOULE_SECOND.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Joule seconds (`J · s`), a unit of `Action`: the coherent unit, a product of base units.
pub const JOULE_SECOND: Unit<{ (METRE * METRE * KILOGRAM / SECOND).unit() }, BASE> = Unit::ONE;

/// Atomic units of action (`ħ`), a unit of `Action`: `n` of them is `n · 6.62607015e-34 / (2.0 * core::f64::consts::PI)` base units.
pub const ATOMIC_UNIT_OF_ACTION: Unit<
    {
        SIUnit::scaled(
            "ħ",
            Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI),
            JOULE_SECOND.unit(),
        )
    },
> = Unit::ONE;

/// Planck constants (`h`), a unit of `Action`: `n` of them is `n · 6.62607015e-34` base units.
pub const PLANCK_CONSTANT: Unit<
    { SIUnit::scaled("h", Exact::dec(662607015, -42), JOULE_SECOND.unit()) },
> = Unit::ONE;

/// Erg seconds (`erg · s`), a unit of `Action`: `n` of them is `n · 1e-07` base units.
pub const ERG_SECOND: Unit<{ SIUnit::scaled("erg · s", Exact::dec(1, -7), JOULE_SECOND.unit()) }> =
    Unit::ONE;

/// Electronvolt seconds (`eV · s`), a unit of `Action`: `n` of them is `n · 1.602176634e-19` base units.
pub const ELECTRONVOLT_SECOND: Unit<
    { SIUnit::scaled("eV · s", Exact::dec(1602176634, -28), JOULE_SECOND.unit()) },
> = Unit::ONE;

/// A torque, in `N`s of the unit `U`: by default its base unit, `NEWTON_METRE`.
pub type Torque<N, const U: SIUnit = { NEWTON_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Newton metres (`N · m`), a unit of `Torque`: the coherent unit, a product of base units.
pub const NEWTON_METRE: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * RADIAN)).unit() },
    BASE,
> = Unit::ONE;

/// Dyne metres (`dyn · m`), a unit of `Torque`: `n` of them is `n · 1e-05` base units.
pub const DYNE_METRE: Unit<{ SIUnit::scaled("dyn · m", Exact::dec(1, -5), NEWTON_METRE.unit()) }> =
    Unit::ONE;

/// Kilogram-force metres (`kgf · m`), a unit of `Torque`: `n` of them is `n · 9.80665` base units.
pub const KILOGRAM_FORCE_METRE: Unit<
    { SIUnit::scaled("kgf · m", Exact::dec(980665, -5), NEWTON_METRE.unit()) },
> = Unit::ONE;

/// Ounces-force inches (`ozf · in`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665 / 16.0) * 0.0254` base units.
pub const OUNCE_FORCE_INCH: Unit<
    {
        SIUnit::scaled(
            "ozf · in",
            (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16))
                * Exact::dec(254, -4),
            NEWTON_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Pounds-force feet (`lbf · ft`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665) * 0.3048` base units.
pub const POUND_FORCE_FOOT: Unit<
    {
        SIUnit::scaled(
            "lbf · ft",
            (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(3048, -4),
            NEWTON_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Pounds-force inches (`lbf · in`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665) * 0.0254` base units.
pub const POUND_FORCE_INCH: Unit<
    {
        SIUnit::scaled(
            "lbf · in",
            (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(254, -4),
            NEWTON_METRE.unit(),
        )
    },
> = Unit::ONE;

/// A moment of inertia, in `N`s of the unit `U`: by default its base unit, `KILOGRAM_SQUARE_METRE`.
pub type MomentOfInertia<
    N,
    const U: SIUnit = { KILOGRAM_SQUARE_METRE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Kilogram square metres (`kg · m²`), a unit of `MomentOfInertia`: the coherent unit, a product of base units.
pub const KILOGRAM_SQUARE_METRE: Unit<
    { (METRE * METRE * KILOGRAM / (RADIAN * RADIAN)).unit() },
    BASE,
> = Unit::ONE;

/// Dalton square ångströms (`Da · Å²`), a unit of `MomentOfInertia`: `n` of them is `n · 1.6605390666e-47` base units.
pub const DALTON_SQUARE_ANGSTROM: Unit<
    {
        SIUnit::scaled(
            "Da · Å²",
            Exact::dec(16605390666, -57),
            KILOGRAM_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// A pressure, in `N`s of the unit `U`: by default its base unit, `PASCAL`.
pub type Pressure<N, const U: SIUnit = { PASCAL.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Pascals (`Pa`), a unit of `Pressure`: the coherent unit, a product of base units.
pub const PASCAL: Unit<{ (KILOGRAM / (METRE * SECOND * SECOND)).unit() }, BASE> = Unit::ONE;

/// Standard atmospheres (`atm`), a unit of `Pressure`: `n` of them is `n · 101325.0` base units.
pub const STANDARD_ATMOSPHERE: Unit<{ SIUnit::scaled("atm", Exact::int(101325), PASCAL.unit()) }> =
    Unit::ONE;

/// Atmospheres (technical) (`at`), a unit of `Pressure`: `n` of them is `n · 98066.5` base units.
pub const ATMOSPHERE_TECHNICAL: Unit<
    { SIUnit::scaled("at", Exact::dec(980665, -1), PASCAL.unit()) },
> = Unit::ONE;

/// Bar (`bar`), a unit of `Pressure`: `n` of them is `n · 100000.0` base units.
pub const BAR: Unit<{ SIUnit::scaled("bar", Exact::int(100000), PASCAL.unit()) }> = Unit::ONE;

/// Centimetres of mercury (`cm Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 10.0` base units.
pub const CENTIMETRE_OF_MERCURY: Unit<
    {
        SIUnit::scaled(
            "cm Hg",
            Exact::dec(133322387415, -9) * Exact::int(10),
            PASCAL.unit(),
        )
    },
> = Unit::ONE;

/// Centimetres of water (`cm H₂O`), a unit of `Pressure`: `n` of them is `n · 98.0665` base units.
pub const CENTIMETRE_OF_WATER: Unit<
    { SIUnit::scaled("cm H₂O", Exact::dec(980665, -4), PASCAL.unit()) },
> = Unit::ONE;

/// Feet of mercury (`ft Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 304.8` base units.
pub const FOOT_OF_MERCURY: Unit<
    {
        SIUnit::scaled(
            "ft Hg",
            Exact::dec(133322387415, -9) * Exact::dec(3048, -1),
            PASCAL.unit(),
        )
    },
> = Unit::ONE;

/// Feet of water (`ft H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665 * 304.8` base units.
pub const FOOT_OF_WATER: Unit<
    {
        SIUnit::scaled(
            "ft H₂O",
            Exact::dec(980665, -5) * Exact::dec(3048, -1),
            PASCAL.unit(),
        )
    },
> = Unit::ONE;

/// Inches of mercury (`in Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 25.4` base units.
pub const INCH_OF_MERCURY: Unit<
    {
        SIUnit::scaled(
            "in Hg",
            Exact::dec(133322387415, -9) * Exact::dec(254, -1),
            PASCAL.unit(),
        )
    },
> = Unit::ONE;

/// Inches of water (`in H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665 * 25.4` base units.
pub const INCH_OF_WATER: Unit<
    {
        SIUnit::scaled(
            "in H₂O",
            Exact::dec(980665, -5) * Exact::dec(254, -1),
            PASCAL.unit(),
        )
    },
> = Unit::ONE;

/// Millimetres of mercury (`mm Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415` base units.
pub const MILLIMETRE_OF_MERCURY: Unit<
    { SIUnit::scaled("mm Hg", Exact::dec(133322387415, -9), PASCAL.unit()) },
> = Unit::ONE;

/// Millimetres of water (`mm H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665` base units.
pub const MILLIMETRE_OF_WATER: Unit<
    { SIUnit::scaled("mm H₂O", Exact::dec(980665, -5), PASCAL.unit()) },
> = Unit::ONE;

/// Pounds-force per square inch (`psi`), a unit of `Pressure`: `n` of them is `n · 0.45359237 * 9.80665 / (0.0254 * 0.0254)` base units.
pub const PSI: Unit<
    {
        SIUnit::scaled(
            "psi",
            Exact::dec(45359237, -8) * Exact::dec(980665, -5)
                / (Exact::dec(254, -4) * Exact::dec(254, -4)),
            PASCAL.unit(),
        )
    },
> = Unit::ONE;

/// Torr (`Torr`), a unit of `Pressure`: `n` of them is `n · 101325.0 / 760.0` base units.
pub const TORR: Unit<
    { SIUnit::scaled("Torr", Exact::int(101325) / Exact::int(760), PASCAL.unit()) },
> = Unit::ONE;

/// A frequency, in `N`s of the unit `U`: by default its base unit, `HERTZ`.
pub type Frequency<N, const U: SIUnit = { HERTZ.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Hertz (`Hz`), a unit of `Frequency`: the coherent unit, a product of base units.
pub const HERTZ: Unit<{ SIUnit::ONE.times(SECOND.unit(), -1) }, BASE> = Unit::ONE;

/// A frequency drift, in `N`s of the unit `U`: by default its base unit, `PER_SECOND_SQUARED`.
pub type FrequencyDrift<N, const U: SIUnit = { PER_SECOND_SQUARED.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `FrequencyDrift`: a product of base units.
pub const PER_SECOND_SQUARED: Unit<
    {
        SIUnit::ONE
            .times(SECOND.unit(), -1)
            .times(SECOND.unit(), -1)
    },
    BASE,
> = Unit::ONE;

/// A reciprocal length, in `N`s of the unit `U`: by default its base unit, `RECIPROCAL_METRE`.
pub type ReciprocalLength<N, const U: SIUnit = { RECIPROCAL_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Reciprocal metres (`m⁻¹`), a unit of `ReciprocalLength`: the coherent unit, a product of base units.
pub const RECIPROCAL_METRE: Unit<{ SIUnit::ONE.times(METRE.unit(), -1) }, BASE> = Unit::ONE;

/// Reciprocal ångströms (`Å⁻¹`), a unit of `ReciprocalLength`: `n` of them is `n · 10000000000.0` base units.
pub const RECIPROCAL_ANGSTROM: Unit<
    { SIUnit::scaled("Å⁻¹", Exact::int(10000000000), RECIPROCAL_METRE.unit()) },
> = Unit::ONE;

/// Diopters (`dpt`), a unit of `ReciprocalLength`: `n` of them is `n · 1.0` base units.
pub const DIOPTER: Unit<{ SIUnit::scaled("dpt", Exact::int(1), RECIPROCAL_METRE.unit()) }> =
    Unit::ONE;

/// Rydberg constants (`R∞`), a unit of `ReciprocalLength`: `n` of them is `n · 10973731.56816` base units.
pub const RYDBERG_CONSTANT: Unit<
    { SIUnit::scaled("R∞", Exact::dec(1097373156816, -5), RECIPROCAL_METRE.unit()) },
> = Unit::ONE;

/// A curvature, in `N`s of the unit `U`: by default its base unit, `RADIAN_PER_METRE`.
pub type Curvature<N, const U: SIUnit = { RADIAN_PER_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// The base unit of `Curvature`: a product of base units.
pub const RADIAN_PER_METRE: Unit<{ (RADIAN / METRE).unit() }, BASE> = Unit::ONE;

/// A specific area, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_PER_KILOGRAM`.
pub type SpecificArea<
    N,
    const U: SIUnit = { METRE_SQUARED_PER_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `SpecificArea`: a product of base units.
pub const METRE_SQUARED_PER_KILOGRAM: Unit<{ (METRE * METRE / KILOGRAM).unit() }, BASE> = Unit::ONE;

/// A specific volume, in `N`s of the unit `U`: by default its base unit, `METRE_CUBED_PER_KILOGRAM`.
pub type SpecificVolume<
    N,
    const U: SIUnit = { METRE_CUBED_PER_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `SpecificVolume`: a product of base units.
pub const METRE_CUBED_PER_KILOGRAM: Unit<{ (METRE * METRE * METRE / KILOGRAM).unit() }, BASE> =
    Unit::ONE;

/// A specific power, in `N`s of the unit `U`: by default its base unit, `METRE_SQUARED_PER_SECOND_CUBED`.
pub type SpecificPower<
    N,
    const U: SIUnit = { METRE_SQUARED_PER_SECOND_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `SpecificPower`: a product of base units.
pub const METRE_SQUARED_PER_SECOND_CUBED: Unit<
    { (METRE * METRE / (SECOND * SECOND * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// A specific gravitational parameter, in `N`s of the unit `U`: by default its base unit, `METRE_CUBED_PER_KILOGRAM_SECOND_SQUARED`.
pub type SpecificGravitationalParameter<
    N,
    const U: SIUnit = { METRE_CUBED_PER_KILOGRAM_SECOND_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `SpecificGravitationalParameter`: a product of base units.
pub const METRE_CUBED_PER_KILOGRAM_SECOND_SQUARED: Unit<
    { (METRE * METRE * METRE / (KILOGRAM * SECOND * SECOND)).unit() },
    BASE,
> = Unit::ONE;

/// Newtonian constants of gravitation (`G`), a unit of `SpecificGravitationalParameter`: `n` of them is `n · 6.6743e-11` base units.
pub const GRAVITATIONAL_CONSTANT: Unit<
    {
        SIUnit::scaled(
            "G",
            Exact::dec(66743, -15),
            METRE_CUBED_PER_KILOGRAM_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;
