//! The quantities of `mechanical` and their units.

#[allow(unused_imports)]
use crate::*;

/// A length, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Length<N, const U: SIUnit = { METRE.unit() }, const IS_BASE: bool = true> =
    SIQuantity<U, N, IS_BASE>;

/// Kilometres (`km`), a unit of `Length`: `n` of them is `n · 1000.0` base units.
pub const KILOMETRE: SIQuantity<
    { SIUnit::scaled("km", Exact::int(1000), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Ångströms (`Å`), a unit of `Length`: `n` of them is `n · 1e-10` base units.
pub const ANGSTROM: SIQuantity<
    { SIUnit::scaled("Å", Exact::dec(1, -10), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Bohr radiuses (`a₀`), a unit of `Length`: `n` of them is `n · 5.29177210903e-11` base units.
pub const BOHR_RADIUS: SIQuantity<
    { SIUnit::scaled("a₀", Exact::dec(529177210903, -22), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Atomic units of length (`a.u. of length`), a unit of `Length`: `n` of them is `n · 5.29177210903e-11` base units.
pub const ATOMIC_UNIT_OF_LENGTH: SIQuantity<
    {
        SIUnit::scaled(
            "a.u. of length",
            Exact::dec(529177210903, -22),
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Astronomical units (`ua`), a unit of `Length`: `n` of them is `n · 149597870700.0` base units.
pub const ASTRONOMICAL_UNIT: SIQuantity<
    { SIUnit::scaled("ua", Exact::int(149597870700), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Chains (`ch`), a unit of `Length`: `n` of them is `n · 66.0 * 1200.0 / 3937.0` base units.
pub const CHAIN: SIQuantity<
    {
        SIUnit::scaled(
            "ch",
            Exact::int(66) * Exact::int(1200) / Exact::int(3937),
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Data miles (`DM`), a unit of `Length`: `n` of them is `n · 1828.8` base units.
pub const DATA_MILE: SIQuantity<
    { SIUnit::scaled("DM", Exact::dec(18288, -1), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Fathoms (`fathom`), a unit of `Length`: `n` of them is `n · 6.0 * 1200.0 / 3937.0` base units.
pub const FATHOM: SIQuantity<
    {
        SIUnit::scaled(
            "fathom",
            Exact::int(6) * Exact::int(1200) / Exact::int(3937),
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Fermis (`fermi`), a unit of `Length`: `n` of them is `n · 1e-15` base units.
pub const FERMI: SIQuantity<
    { SIUnit::scaled("fermi", Exact::dec(1, -15), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Feet (`ft`), a unit of `Length`: `n` of them is `n · 0.3048` base units.
pub const FOOT: SIQuantity<
    { SIUnit::scaled("ft", Exact::dec(3048, -4), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Feet (U.S. survey) (`ft (U.S. survey)`), a unit of `Length`: `n` of them is `n · 1200.0 / 3937.0` base units.
pub const FOOT_SURVEY: SIQuantity<
    {
        SIUnit::scaled(
            "ft (U.S. survey)",
            Exact::int(1200) / Exact::int(3937),
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Inches (`in`), a unit of `Length`: `n` of them is `n · 0.0254` base units.
pub const INCH: SIQuantity<
    { SIUnit::scaled("in", Exact::dec(254, -4), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Light years (`l. y.`), a unit of `Length`: `n` of them is `n · 9460730472580800.0` base units.
pub const LIGHT_YEAR: SIQuantity<
    { SIUnit::scaled("l. y.", Exact::int(9460730472580800), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Microns (`μ`), a unit of `Length`: `n` of them is `n · 1e-06` base units.
pub const MICRON: SIQuantity<
    { SIUnit::scaled("μ", Exact::dec(1, -6), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Mils (`0.001 in`), a unit of `Length`: `n` of them is `n · 2.54e-05` base units.
pub const MIL: SIQuantity<
    { SIUnit::scaled("0.001 in", Exact::dec(254, -7), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Miles (`mi`), a unit of `Length`: `n` of them is `n · 1609.344` base units.
pub const MILE: SIQuantity<
    { SIUnit::scaled("mi", Exact::dec(1609344, -3), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Miles (U.S. survey) (`mi (U.S. survey)`), a unit of `Length`: `n` of them is `n · 6336000.0 / 3937.0` base units.
pub const MILE_SURVEY: SIQuantity<
    {
        SIUnit::scaled(
            "mi (U.S. survey)",
            Exact::int(6336000) / Exact::int(3937),
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Nautical miles (`M`), a unit of `Length`: `n` of them is `n · 1852.0` base units.
pub const NAUTICAL_MILE: SIQuantity<
    { SIUnit::scaled("M", Exact::int(1852), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Parsecs (`pc`), a unit of `Length`: `n` of them is `n · 149597870700.0 * 648000.0 / core::f64::consts::PI` base units.
pub const PARSEC: SIQuantity<
    {
        SIUnit::scaled(
            "pc",
            Exact::int(149597870700) * Exact::int(648000) / Exact::PI,
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Picas (computer) (`1/6 in (computer)`), a unit of `Length`: `n` of them is `n · 0.0254 / 6.0` base units.
pub const PICA_COMPUTER: SIQuantity<
    {
        SIUnit::scaled(
            "1/6 in (computer)",
            Exact::dec(254, -4) / Exact::int(6),
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Picas (printer's) (`1/6 in`), a unit of `Length`: `n` of them is `n · 0.004217518` base units.
pub const PICA_PRINTERS: SIQuantity<
    { SIUnit::scaled("1/6 in", Exact::dec(4217518, -9), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Points (computer) (`1/72 in (computer)`), a unit of `Length`: `n` of them is `n · 0.0254 / 72.0` base units.
pub const POINT_COMPUTER: SIQuantity<
    {
        SIUnit::scaled(
            "1/72 in (computer)",
            Exact::dec(254, -4) / Exact::int(72),
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Points (printer's) (`1/72 in`), a unit of `Length`: `n` of them is `n · 0.0003514598` base units.
pub const POINT_PRINTERS: SIQuantity<
    { SIUnit::scaled("1/72 in", Exact::dec(3514598, -10), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Rods (`rd`), a unit of `Length`: `n` of them is `n · 16.5 * 1200.0 / 3937.0` base units.
pub const ROD: SIQuantity<
    {
        SIUnit::scaled(
            "rd",
            Exact::dec(165, -1) * Exact::int(1200) / Exact::int(3937),
            METRE.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Yards (`yd`), a unit of `Length`: `n` of them is `n · 0.9144` base units.
pub const YARD: SIQuantity<
    { SIUnit::scaled("yd", Exact::dec(9144, -4), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Leugae (`leuga`), a unit of `Length`: `n` of them is `n · 2220.0` base units.
pub const LEUGA: SIQuantity<
    { SIUnit::scaled("leuga", Exact::int(2220), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Mille passa (`mille passus`), a unit of `Length`: `n` of them is `n · 1480.0` base units.
pub const MILLE_PASSUS: SIQuantity<
    { SIUnit::scaled("mille passus", Exact::int(1480), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Stadia (`stadium`), a unit of `Length`: `n` of them is `n · 185.0` base units.
pub const STADIUM: SIQuantity<
    { SIUnit::scaled("stadium", Exact::int(185), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Acta (`actus`), a unit of `Length`: `n` of them is `n · 35.52` base units.
pub const ACTUS: SIQuantity<
    { SIUnit::scaled("actus", Exact::dec(3552, -2), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Decempedae (`decempeda`), a unit of `Length`: `n` of them is `n · 2.96` base units.
pub const DECEMPEDA: SIQuantity<
    { SIUnit::scaled("decempeda", Exact::dec(296, -2), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Passa (`passus`), a unit of `Length`: `n` of them is `n · 1.48` base units.
pub const PASSUS: SIQuantity<
    { SIUnit::scaled("passus", Exact::dec(148, -2), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Gradus (`gradus`), a unit of `Length`: `n` of them is `n · 0.74` base units.
pub const GRADUS: SIQuantity<
    { SIUnit::scaled("gradus", Exact::dec(74, -2), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Cubita (`cubitum`), a unit of `Length`: `n` of them is `n · 0.444` base units.
pub const CUBITUM: SIQuantity<
    { SIUnit::scaled("cubitum", Exact::dec(444, -3), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Palmipedes (`palmipes`), a unit of `Length`: `n` of them is `n · 0.37` base units.
pub const PALMIPES: SIQuantity<
    { SIUnit::scaled("palmipes", Exact::dec(37, -2), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pedes (`pes`), a unit of `Length`: `n` of them is `n · 0.296` base units.
pub const PES: SIQuantity<
    { SIUnit::scaled("pes", Exact::dec(296, -3), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Palmas maior (`palmus maior`), a unit of `Length`: `n` of them is `n · 0.222` base units.
pub const PALMUS_MAIOR: SIQuantity<
    { SIUnit::scaled("palmus maior", Exact::dec(222, -3), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Palmas (`palmus`), a unit of `Length`: `n` of them is `n · 0.074` base units.
pub const PALMUS: SIQuantity<
    { SIUnit::scaled("palmus", Exact::dec(74, -3), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Unciae (`uncia`), a unit of `Length`: `n` of them is `n · 0.296 / 12.0` base units.
pub const LENGTH_UNCIA: SIQuantity<
    { SIUnit::scaled("uncia", Exact::dec(296, -3) / Exact::int(12), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Digiti (`digitus`), a unit of `Length`: `n` of them is `n · 0.0185` base units.
pub const DIGITUS: SIQuantity<
    { SIUnit::scaled("digitus", Exact::dec(185, -4), METRE.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// A mass, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Mass<N, const U: SIUnit = { KILOGRAM.unit() }, const IS_BASE: bool = true> =
    SIQuantity<U, N, IS_BASE>;

/// Grams (`g`), a unit of `Mass`: `n` of them is `n · 0.001` base units.
pub const GRAM: SIQuantity<
    { SIUnit::scaled("g", Exact::dec(1, -3), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Carats (`ct`), a unit of `Mass`: `n` of them is `n · 0.0002` base units.
pub const CARAT: SIQuantity<
    { SIUnit::scaled("ct", Exact::dec(2, -4), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Daltons (`Da`), a unit of `Mass`: `n` of them is `n · 1.6605390666e-27` base units.
pub const DALTON: SIQuantity<
    { SIUnit::scaled("Da", Exact::dec(16605390666, -37), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Electron masses (`mₑ`), a unit of `Mass`: `n` of them is `n · 9.1093837015e-31` base units.
pub const ELECTRON_MASS: SIQuantity<
    { SIUnit::scaled("mₑ", Exact::dec(91093837015, -41), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Proton masses (`mₚ`), a unit of `Mass`: `n` of them is `n · 1.67262192369e-27` base units.
pub const PROTON_MASS: SIQuantity<
    { SIUnit::scaled("mₚ", Exact::dec(167262192369, -38), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Grains (`gr`), a unit of `Mass`: `n` of them is `n · 6.479891e-05` base units.
pub const GRAIN: SIQuantity<
    { SIUnit::scaled("gr", Exact::dec(6479891, -11), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Hundredweight (long) (`cwt long`), a unit of `Mass`: `n` of them is `n · 50.80234544` base units.
pub const HUNDREDWEIGHT_LONG: SIQuantity<
    { SIUnit::scaled("cwt long", Exact::dec(5080234544, -8), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Hundredweight (short) (`cwt short`), a unit of `Mass`: `n` of them is `n · 45.359237` base units.
pub const HUNDREDWEIGHT_SHORT: SIQuantity<
    { SIUnit::scaled("cwt short", Exact::dec(45359237, -6), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Ounces (`oz`), a unit of `Mass`: `n` of them is `n · 0.028349523125` base units.
pub const OUNCE: SIQuantity<
    { SIUnit::scaled("oz", Exact::dec(28349523125, -12), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Troy ounces (`oz t`), a unit of `Mass`: `n` of them is `n · 0.0311034768` base units.
pub const OUNCE_TROY: SIQuantity<
    { SIUnit::scaled("oz t", Exact::dec(311034768, -10), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pennyweight (`dwt`), a unit of `Mass`: `n` of them is `n · 0.00155517384` base units.
pub const PENNYWEIGHT: SIQuantity<
    { SIUnit::scaled("dwt", Exact::dec(155517384, -11), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pounds (`lb`), a unit of `Mass`: `n` of them is `n · 0.45359237` base units.
pub const POUND: SIQuantity<
    { SIUnit::scaled("lb", Exact::dec(45359237, -8), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Troy pounds (`lb t`), a unit of `Mass`: `n` of them is `n · 0.3732417216` base units.
pub const POUND_TROY: SIQuantity<
    { SIUnit::scaled("lb t", Exact::dec(3732417216, -10), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Slugs (`slug`), a unit of `Mass`: `n` of them is `n · 0.45359237 * 9.80665 / 0.3048` base units.
pub const SLUG: SIQuantity<
    {
        SIUnit::scaled(
            "slug",
            Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::dec(3048, -4),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Assay tons (`AT`), a unit of `Mass`: `n` of them is `n · 0.175 / 6.0` base units.
pub const TON_ASSAY: SIQuantity<
    { SIUnit::scaled("AT", Exact::dec(175, -3) / Exact::int(6), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Long tons (`2240 lb`), a unit of `Mass`: `n` of them is `n · 1016.0469088` base units.
pub const TON_LONG: SIQuantity<
    { SIUnit::scaled("2240 lb", Exact::dec(10160469088, -7), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Short tons (`2000 lb`), a unit of `Mass`: `n` of them is `n · 907.18474` base units.
pub const TON_SHORT: SIQuantity<
    { SIUnit::scaled("2000 lb", Exact::dec(90718474, -5), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Tons (`t`), a unit of `Mass`: `n` of them is `n · 1000.0` base units.
pub const TON: SIQuantity<
    { SIUnit::scaled("t", Exact::int(1000), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Librae (`libra`), a unit of `Mass`: `n` of them is `n · 0.3289` base units.
pub const LIBRA: SIQuantity<
    { SIUnit::scaled("libra", Exact::dec(3289, -4), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Deunces (`deunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 11.0 / 12.0` base units.
pub const MASS_DEUNX: SIQuantity<
    {
        SIUnit::scaled(
            "deunx",
            Exact::dec(3289, -4) * Exact::int(11) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Dextantes (`dextans`), a unit of `Mass`: `n` of them is `n · 0.3289 * 10.0 / 12.0` base units.
pub const MASS_DEXTANS: SIQuantity<
    {
        SIUnit::scaled(
            "dextans",
            Exact::dec(3289, -4) * Exact::int(10) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Dodrantes (`dodrans`), a unit of `Mass`: `n` of them is `n · 0.246675` base units.
pub const MASS_DODRANS: SIQuantity<
    { SIUnit::scaled("dodrans", Exact::dec(246675, -6), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Bessis (`bes`), a unit of `Mass`: `n` of them is `n · 0.3289 * 8.0 / 12.0` base units.
pub const MASS_BES: SIQuantity<
    {
        SIUnit::scaled(
            "bes",
            Exact::dec(3289, -4) * Exact::int(8) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Septunces (`septunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 7.0 / 12.0` base units.
pub const MASS_SEPTUNX: SIQuantity<
    {
        SIUnit::scaled(
            "septunx",
            Exact::dec(3289, -4) * Exact::int(7) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Semisses (`semis`), a unit of `Mass`: `n` of them is `n · 0.16445` base units.
pub const MASS_SEMIS: SIQuantity<
    { SIUnit::scaled("semis", Exact::dec(16445, -5), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Quincunx (`quincunx`), a unit of `Mass`: `n` of them is `n · 0.3289 * 5.0 / 12.0` base units.
pub const MASS_QUINCUNX: SIQuantity<
    {
        SIUnit::scaled(
            "quincunx",
            Exact::dec(3289, -4) * Exact::int(5) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Trientes (`triens`), a unit of `Mass`: `n` of them is `n · 0.3289 / 3.0` base units.
pub const MASS_TRIENS: SIQuantity<
    {
        SIUnit::scaled(
            "triens",
            Exact::dec(3289, -4) / Exact::int(3),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Quadrantes teruncius (`quadrans teruncius`), a unit of `Mass`: `n` of them is `n · 0.082225` base units.
pub const QUADRANS_TERUNCIUS: SIQuantity<
    { SIUnit::scaled("quadrans teruncius", Exact::dec(82225, -6), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Sextantes (`sextans`), a unit of `Mass`: `n` of them is `n · 0.3289 / 6.0` base units.
pub const MASS_SEXTANS: SIQuantity<
    {
        SIUnit::scaled(
            "sextans",
            Exact::dec(3289, -4) / Exact::int(6),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Sescunciae (`sescuncia`), a unit of `Mass`: `n` of them is `n · 0.0411125` base units.
pub const SESCUNCIA: SIQuantity<
    { SIUnit::scaled("sescuncia", Exact::dec(411125, -7), KILOGRAM.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Unciae (`uncia`), a unit of `Mass`: `n` of them is `n · 0.3289 / 12.0` base units.
pub const MASS_UNCIA: SIQuantity<
    {
        SIUnit::scaled(
            "uncia",
            Exact::dec(3289, -4) / Exact::int(12),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Semunciae (`semuncia`), a unit of `Mass`: `n` of them is `n · 0.3289 / 24.0` base units.
pub const SEMUNCIA: SIQuantity<
    {
        SIUnit::scaled(
            "semuncia",
            Exact::dec(3289, -4) / Exact::int(24),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Duella (`duella`), a unit of `Mass`: `n` of them is `n · 0.3289 / 36.0` base units.
pub const DUELLA: SIQuantity<
    {
        SIUnit::scaled(
            "duella",
            Exact::dec(3289, -4) / Exact::int(36),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Scilici (`sicilicus`), a unit of `Mass`: `n` of them is `n · 0.3289 / 48.0` base units.
pub const MASS_SICILICUS: SIQuantity<
    {
        SIUnit::scaled(
            "sicilicus",
            Exact::dec(3289, -4) / Exact::int(48),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Sextulae (`sextula`), a unit of `Mass`: `n` of them is `n · 0.3289 / 72.0` base units.
pub const MASS_SEXTULA: SIQuantity<
    {
        SIUnit::scaled(
            "sextula",
            Exact::dec(3289, -4) / Exact::int(72),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Semisextulae (`semisextula`), a unit of `Mass`: `n` of them is `n · 0.3289 / 144.0` base units.
pub const SEMISEXTULA: SIQuantity<
    {
        SIUnit::scaled(
            "semisextula",
            Exact::dec(3289, -4) / Exact::int(144),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Scrupula (`scrupulum`), a unit of `Mass`: `n` of them is `n · 0.3289 / 288.0` base units.
pub const MASS_SCRUPULUM: SIQuantity<
    {
        SIUnit::scaled(
            "scrupulum",
            Exact::dec(3289, -4) / Exact::int(288),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Oboli (`obolus`), a unit of `Mass`: `n` of them is `n · 0.3289 / 576.0` base units.
pub const OBOLUS: SIQuantity<
    {
        SIUnit::scaled(
            "obolus",
            Exact::dec(3289, -4) / Exact::int(576),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Siliquae (`siliqua`), a unit of `Mass`: `n` of them is `n · 0.3289 / 1728.0` base units.
pub const SILIQUA: SIQuantity<
    {
        SIUnit::scaled(
            "siliqua",
            Exact::dec(3289, -4) / Exact::int(1728),
            KILOGRAM.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A time, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Time<N, const U: SIUnit = { SECOND.unit() }, const IS_BASE: bool = true> =
    SIQuantity<U, N, IS_BASE>;

/// Seconds (sidereal) (`s (sidereal)`), a unit of `Time`: `n` of them is `n · 0.9972696` base units.
pub const SECOND_SIDEREAL: SIQuantity<
    { SIUnit::scaled("s (sidereal)", Exact::dec(9972696, -7), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Days (`d`), a unit of `Time`: `n` of them is `n · 86400.0` base units.
pub const DAY: SIQuantity<{ SIUnit::scaled("d", Exact::int(86400), SECOND.unit()) }, Exact, false> =
    SIQuantity::ONE;

/// Days (sidereal) (`d (sidereal)`), a unit of `Time`: `n` of them is `n · 86164.09` base units.
pub const DAY_SIDEREAL: SIQuantity<
    { SIUnit::scaled("d (sidereal)", Exact::dec(8616409, -2), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Hours (`h`), a unit of `Time`: `n` of them is `n · 3600.0` base units.
pub const HOUR: SIQuantity<{ SIUnit::scaled("h", Exact::int(3600), SECOND.unit()) }, Exact, false> =
    SIQuantity::ONE;

/// Hours (sidereal) (`h (sidereal)`), a unit of `Time`: `n` of them is `n · 3590.17` base units.
pub const HOUR_SIDEREAL: SIQuantity<
    { SIUnit::scaled("h (sidereal)", Exact::dec(359017, -2), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Minutes (`min`), a unit of `Time`: `n` of them is `n · 60.0` base units.
pub const MINUTE: SIQuantity<
    { SIUnit::scaled("min", Exact::int(60), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Shakes (`10.0 ns`), a unit of `Time`: `n` of them is `n · 1e-08` base units.
pub const SHAKE: SIQuantity<
    { SIUnit::scaled("10.0 ns", Exact::dec(1, -8), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Years (`a`), a unit of `Time`: `n` of them is `n · 31536000.0` base units.
pub const YEAR: SIQuantity<
    { SIUnit::scaled("a", Exact::int(31536000), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Years (sidereal) (`a (sidereal)`), a unit of `Time`: `n` of them is `n · 31558150.0` base units.
pub const YEAR_SIDEREAL: SIQuantity<
    { SIUnit::scaled("a (sidereal)", Exact::int(31558150), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Years (tropical) (`a (tropical)`), a unit of `Time`: `n` of them is `n · 31556930.0` base units.
pub const YEAR_TROPICAL: SIQuantity<
    { SIUnit::scaled("a (tropical)", Exact::int(31556930), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// An instant, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Instant<N, const U: SIUnit = { SECOND.unit() }, const IS_BASE: bool = true> =
    SIQuantity<U, N, IS_BASE>;

/// Seconds of Unix time (`s Unix`), a unit of `Instant`: the coherent unit, a product of base units.
pub const UNIX_SECOND: SIQuantity<{ SECOND.unit() }, Exact, true> = SIQuantity::ONE;

/// Milliseconds of Unix time (`ms Unix`), a unit of `Instant`: `n` of them is `n · 0.001` base units.
pub const UNIX_MILLISECOND: SIQuantity<
    { SIUnit::scaled("ms Unix", Exact::dec(1, -3), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Seconds of GPS time (`s GPS`), a unit of `Instant`: `n` of them is `n · 1.0 + 315964800.0` base units.
pub const GPS_SECOND: SIQuantity<
    { SIUnit::offset("s GPS", Exact::int(1), Exact::int(315964800), SECOND.unit()) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Julian date (`JD`), a unit of `Instant`: `n` of them is `n · 86400.0 + -2440587.5 * 86400.0` base units.
pub const JULIAN_DATE: SIQuantity<
    {
        SIUnit::offset(
            "JD",
            Exact::int(86400),
            -Exact::dec(24405875, -1) * Exact::int(86400),
            SECOND.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Modified Julian date (`MJD`), a unit of `Instant`: `n` of them is `n · 86400.0 + -40587.0 * 86400.0` base units.
pub const MODIFIED_JULIAN_DATE: SIQuantity<
    {
        SIUnit::offset(
            "MJD",
            Exact::int(86400),
            -Exact::int(40587) * Exact::int(86400),
            SECOND.unit(),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// An area, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Area<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 2) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Square metres (`m²`), a unit of `Area`: the coherent unit, a product of base units.
pub const SQUARE_METRE: SIQuantity<{ SIUnit::ONE.times(METRE.unit(), 2) }, Exact, true> =
    SIQuantity::ONE;

/// Acres (`ac`), a unit of `Area`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0)` base units.
pub const ACRE: SIQuantity<
    {
        SIUnit::scaled(
            "ac",
            Exact::int(43560)
                * (Exact::int(1200) / Exact::int(3937))
                * (Exact::int(1200) / Exact::int(3937)),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Ares (`a`), a unit of `Area`: `n` of them is `n · 100.0` base units.
pub const ARE: SIQuantity<
    { SIUnit::scaled("a", Exact::int(100), SIUnit::ONE.times(METRE.unit(), 2)) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Barns (`b`), a unit of `Area`: `n` of them is `n · 1e-28` base units.
pub const BARN: SIQuantity<
    { SIUnit::scaled("b", Exact::dec(1, -28), SIUnit::ONE.times(METRE.unit(), 2)) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Circular mils (`cmil`), a unit of `Area`: `n` of them is `n · core::f64::consts::PI / 4.0 * 2.54e-05 * 2.54e-05` base units.
pub const CIRCULAR_MIL: SIQuantity<
    {
        SIUnit::scaled(
            "cmil",
            Exact::PI / Exact::int(4) * Exact::dec(254, -7) * Exact::dec(254, -7),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Hectares (`ha`), a unit of `Area`: `n` of them is `n · 10000.0` base units.
pub const HECTARE: SIQuantity<
    { SIUnit::scaled("ha", Exact::int(10000), SIUnit::ONE.times(METRE.unit(), 2)) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Square feet (`ft²`), a unit of `Area`: `n` of them is `n · 0.09290304` base units.
pub const SQUARE_FOOT: SIQuantity<
    {
        SIUnit::scaled(
            "ft²",
            Exact::dec(9290304, -8),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Square inches (`in²`), a unit of `Area`: `n` of them is `n · 0.00064516` base units.
pub const SQUARE_INCH: SIQuantity<
    {
        SIUnit::scaled(
            "in²",
            Exact::dec(64516, -8),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Square miles (`mi²`), a unit of `Area`: `n` of them is `n · 2589988.110336` base units.
pub const SQUARE_MILE: SIQuantity<
    {
        SIUnit::scaled(
            "mi²",
            Exact::dec(2589988110336, -6),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Square yards (`yd²`), a unit of `Area`: `n` of them is `n · 0.83612736` base units.
pub const SQUARE_YARD: SIQuantity<
    {
        SIUnit::scaled(
            "yd²",
            Exact::dec(83612736, -8),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Saltus (`saltus`), a unit of `Area`: `n` of them is `n · 2018672.64` base units.
pub const SALTUS: SIQuantity<
    {
        SIUnit::scaled(
            "saltus",
            Exact::dec(201867264, -2),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Centuriae (`centuria`), a unit of `Area`: `n` of them is `n · 504668.16` base units.
pub const CENTURIA: SIQuantity<
    {
        SIUnit::scaled(
            "centuria",
            Exact::dec(50466816, -2),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Heredia (`heredium`), a unit of `Area`: `n` of them is `n · 5046.6816` base units.
pub const HEREDIUM: SIQuantity<
    {
        SIUnit::scaled(
            "heredium",
            Exact::dec(50466816, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Jugera (`jugerum`), a unit of `Area`: `n` of them is `n · 2523.3408` base units.
pub const JUGERUM: SIQuantity<
    {
        SIUnit::scaled(
            "jugerum",
            Exact::dec(25233408, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Deunces (`deunx`), a unit of `Area`: `n` of them is `n · 2313.0624` base units.
pub const AREA_DEUNX: SIQuantity<
    {
        SIUnit::scaled(
            "deunx",
            Exact::dec(23130624, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Dextantes (`dextans`), a unit of `Area`: `n` of them is `n · 2102.784` base units.
pub const AREA_DEXTANS: SIQuantity<
    {
        SIUnit::scaled(
            "dextans",
            Exact::dec(2102784, -3),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Dodrantes (`dodrans`), a unit of `Area`: `n` of them is `n · 1892.5056` base units.
pub const AREA_DODRANS: SIQuantity<
    {
        SIUnit::scaled(
            "dodrans",
            Exact::dec(18925056, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Besses (`bes`), a unit of `Area`: `n` of them is `n · 1682.2272` base units.
pub const AREA_BES: SIQuantity<
    {
        SIUnit::scaled(
            "bes",
            Exact::dec(16822272, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Septunces (`septunx`), a unit of `Area`: `n` of them is `n · 1471.9488` base units.
pub const AREA_SEPTUNX: SIQuantity<
    {
        SIUnit::scaled(
            "septunx",
            Exact::dec(14719488, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Semisses (`semis`), a unit of `Area`: `n` of them is `n · 1261.6704` base units.
pub const AREA_SEMIS: SIQuantity<
    {
        SIUnit::scaled(
            "semis",
            Exact::dec(12616704, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Acta quadratus (`actus quadratus`), a unit of `Area`: `n` of them is `n · 1261.6704` base units.
pub const ACTUS_QUADRATUS: SIQuantity<
    {
        SIUnit::scaled(
            "actus quadratus",
            Exact::dec(12616704, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Quincunces (`quincunx`), a unit of `Area`: `n` of them is `n · 1051.392` base units.
pub const AREA_QUINCUNX: SIQuantity<
    {
        SIUnit::scaled(
            "quincunx",
            Exact::dec(1051392, -3),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Trientes (`triens`), a unit of `Area`: `n` of them is `n · 841.1136` base units.
pub const AREA_TRIENS: SIQuantity<
    {
        SIUnit::scaled(
            "triens",
            Exact::dec(8411136, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Quadrantes (`quadrans`), a unit of `Area`: `n` of them is `n · 630.8352` base units.
pub const QUADRANS: SIQuantity<
    {
        SIUnit::scaled(
            "quadrans",
            Exact::dec(6308352, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Sextantes (`sextans`), a unit of `Area`: `n` of them is `n · 420.5568` base units.
pub const AREA_SEXTANS: SIQuantity<
    {
        SIUnit::scaled(
            "sextans",
            Exact::dec(4205568, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Climata (`clima`), a unit of `Area`: `n` of them is `n · 315.4176` base units.
pub const CLIMA: SIQuantity<
    {
        SIUnit::scaled(
            "clima",
            Exact::dec(3154176, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Unciae (`uncia`), a unit of `Area`: `n` of them is `n · 210.2784` base units.
pub const AREA_UNCIA: SIQuantity<
    {
        SIUnit::scaled(
            "uncia",
            Exact::dec(2102784, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Semiunciae (`semiuncia`), a unit of `Area`: `n` of them is `n · 105.1392` base units.
pub const SEMIUNCIA: SIQuantity<
    {
        SIUnit::scaled(
            "semiuncia",
            Exact::dec(1051392, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Siculi (`sicilicus`), a unit of `Area`: `n` of them is `n · 52.5696` base units.
pub const AREA_SICILICUS: SIQuantity<
    {
        SIUnit::scaled(
            "sicilicus",
            Exact::dec(525696, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Acta simplex (`actus simplex`), a unit of `Area`: `n` of them is `n · 42.05568` base units.
pub const ACTUS_SIMPLEX: SIQuantity<
    {
        SIUnit::scaled(
            "actus simplex",
            Exact::dec(4205568, -5),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Sextulae (`sextula`), a unit of `Area`: `n` of them is `n · 35.0464` base units.
pub const AREA_SEXTULA: SIQuantity<
    {
        SIUnit::scaled(
            "sextula",
            Exact::dec(350464, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Duo scrupula (`duo scrupulum`), a unit of `Area`: `n` of them is `n · 17.5232` base units.
pub const DUO_SCRUPULUM: SIQuantity<
    {
        SIUnit::scaled(
            "duo scrupulum",
            Exact::dec(175232, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Scrupula (`scrupulum`), a unit of `Area`: `n` of them is `n · 8.7616` base units.
pub const AREA_SCRUPULUM: SIQuantity<
    {
        SIUnit::scaled(
            "scrupulum",
            Exact::dec(87616, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Dimidium scupula (`dimidium scrupulum`), a unit of `Area`: `n` of them is `n · 4.3808` base units.
pub const DIMIDIUM_SCRUPULUM: SIQuantity<
    {
        SIUnit::scaled(
            "dimidium scrupulum",
            Exact::dec(43808, -4),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pedes quadratus (`pes quadratus`), a unit of `Area`: `n` of them is `n · 0.087616` base units.
pub const PES_QUADRATUS: SIQuantity<
    {
        SIUnit::scaled(
            "pes quadratus",
            Exact::dec(87616, -6),
            SIUnit::ONE.times(METRE.unit(), 2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A volume, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Volume<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 3) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Cubic metres (`m³`), a unit of `Volume`: the coherent unit, a product of base units.
pub const CUBIC_METRE: SIQuantity<{ SIUnit::ONE.times(METRE.unit(), 3) }, Exact, true> =
    SIQuantity::ONE;

/// Acre-feet (`ac · ft`), a unit of `Volume`: `n` of them is `n · 43560.0 * (1200.0 / 3937.0) * (1200.0 / 3937.0) * (1200.0 / 3937.0)` base units.
pub const ACRE_FOOT: SIQuantity<
    {
        SIUnit::scaled(
            "ac · ft",
            Exact::int(43560)
                * (Exact::int(1200) / Exact::int(3937))
                * (Exact::int(1200) / Exact::int(3937))
                * (Exact::int(1200) / Exact::int(3937)),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Barrels (`bbl`), a unit of `Volume`: `n` of them is `n · 0.158987294928` base units.
pub const BARREL: SIQuantity<
    {
        SIUnit::scaled(
            "bbl",
            Exact::dec(158987294928, -12),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Bushels (`bu`), a unit of `Volume`: `n` of them is `n · 0.03523907016688` base units.
pub const BUSHEL: SIQuantity<
    {
        SIUnit::scaled(
            "bu",
            Exact::dec(3523907016688, -14),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Cords (`cords`), a unit of `Volume`: `n` of them is `n · 128.0 * 0.3048 * 0.3048 * 0.3048` base units.
pub const CORD: SIQuantity<
    {
        SIUnit::scaled(
            "cords",
            Exact::int(128) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Cubic feet (`ft³`), a unit of `Volume`: `n` of them is `n · 0.3048 * 0.3048 * 0.3048` base units.
pub const CUBIC_FOOT: SIQuantity<
    {
        SIUnit::scaled(
            "ft³",
            Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Cubic inches (`in³`), a unit of `Volume`: `n` of them is `n · 0.0254 * 0.0254 * 0.0254` base units.
pub const CUBIC_INCH: SIQuantity<
    {
        SIUnit::scaled(
            "in³",
            Exact::dec(254, -4) * Exact::dec(254, -4) * Exact::dec(254, -4),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Cubic miles (`mi³`), a unit of `Volume`: `n` of them is `n · 1609.344 * 1609.344 * 1609.344` base units.
pub const CUBIC_MILE: SIQuantity<
    {
        SIUnit::scaled(
            "mi³",
            Exact::dec(1609344, -3) * Exact::dec(1609344, -3) * Exact::dec(1609344, -3),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Cubic yards (`yd³`), a unit of `Volume`: `n` of them is `n · 0.9144 * 0.9144 * 0.9144` base units.
pub const CUBIC_YARD: SIQuantity<
    {
        SIUnit::scaled(
            "yd³",
            Exact::dec(9144, -4) * Exact::dec(9144, -4) * Exact::dec(9144, -4),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Cups (`cup`), a unit of `Volume`: `n` of them is `n · 0.0002365882365` base units.
pub const CUP: SIQuantity<
    {
        SIUnit::scaled(
            "cup",
            Exact::dec(2365882365, -13),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Fluid ounces (`fl oz`), a unit of `Volume`: `n` of them is `n · 2.95735295625e-05` base units.
pub const FLUID_OUNCE: SIQuantity<
    {
        SIUnit::scaled(
            "fl oz",
            Exact::dec(295735295625, -16),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Imperial fluid ounces (`fl oz (UK)`), a unit of `Volume`: `n` of them is `n · 2.84130625e-05` base units.
pub const FLUID_OUNCE_IMPERIAL: SIQuantity<
    {
        SIUnit::scaled(
            "fl oz (UK)",
            Exact::dec(284130625, -13),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Imperial gallons (`gal (UK)`), a unit of `Volume`: `n` of them is `n · 0.00454609` base units.
pub const GALLON_IMPERIAL: SIQuantity<
    {
        SIUnit::scaled(
            "gal (UK)",
            Exact::dec(454609, -8),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Gallons (`gal`), a unit of `Volume`: `n` of them is `n · 0.003785411784` base units.
pub const GALLON: SIQuantity<
    {
        SIUnit::scaled(
            "gal",
            Exact::dec(3785411784, -12),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Imperial gills (`gi (UK)`), a unit of `Volume`: `n` of them is `n · 0.0001420653125` base units.
pub const GILL_IMPERIAL: SIQuantity<
    {
        SIUnit::scaled(
            "gi (UK)",
            Exact::dec(1420653125, -13),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Gills (`gi`), a unit of `Volume`: `n` of them is `n · 0.00011829411825` base units.
pub const GILL: SIQuantity<
    {
        SIUnit::scaled(
            "gi",
            Exact::dec(11829411825, -14),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Litres (`L`), a unit of `Volume`: `n` of them is `n · 0.001` base units.
pub const LITRE: SIQuantity<
    { SIUnit::scaled("L", Exact::dec(1, -3), SIUnit::ONE.times(METRE.unit(), 3)) },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pecks (`pk`), a unit of `Volume`: `n` of them is `n · 0.00880976754172` base units.
pub const PECK: SIQuantity<
    {
        SIUnit::scaled(
            "pk",
            Exact::dec(880976754172, -14),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Dry pints (`dry pt`), a unit of `Volume`: `n` of them is `n · 0.0005506104713575` base units.
pub const PINT_DRY: SIQuantity<
    {
        SIUnit::scaled(
            "dry pt",
            Exact::dec(5506104713575, -16),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Liquid pints (`liq pt`), a unit of `Volume`: `n` of them is `n · 0.000473176473` base units.
pub const PINT_LIQUID: SIQuantity<
    {
        SIUnit::scaled(
            "liq pt",
            Exact::dec(473176473, -12),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Dry quarts (`dry qt`), a unit of `Volume`: `n` of them is `n · 0.001101220942715` base units.
pub const QUART_DRY: SIQuantity<
    {
        SIUnit::scaled(
            "dry qt",
            Exact::dec(1101220942715, -15),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Liquid quarts (`liq qt`), a unit of `Volume`: `n` of them is `n · 0.000946352946` base units.
pub const QUART_LIQUID: SIQuantity<
    {
        SIUnit::scaled(
            "liq qt",
            Exact::dec(946352946, -12),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Steres (`st`), a unit of `Volume`: the coherent unit, a product of base units.
pub const STERE: SIQuantity<{ SIUnit::ONE.times(METRE.unit(), 3) }, Exact, true> = SIQuantity::ONE;

/// Tablespoons (`tbsp`), a unit of `Volume`: `n` of them is `n · 1.478676478125e-05` base units.
pub const TABLESPOON: SIQuantity<
    {
        SIUnit::scaled(
            "tbsp",
            Exact::dec(1478676478125, -17),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Teaspoons (`tsp`), a unit of `Volume`: `n` of them is `n · 4.92892159375e-06` base units.
pub const TEASPOON: SIQuantity<
    {
        SIUnit::scaled(
            "tsp",
            Exact::dec(492892159375, -17),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Register tons (`RT`), a unit of `Volume`: `n` of them is `n · 100.0 * 0.3048 * 0.3048 * 0.3048` base units.
pub const REGISTER_TON: SIQuantity<
    {
        SIUnit::scaled(
            "RT",
            Exact::int(100) * Exact::dec(3048, -4) * Exact::dec(3048, -4) * Exact::dec(3048, -4),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Culei (`culeus`), a unit of `Volume`: `n` of them is `n · 0.51868672` base units.
pub const CULEUS: SIQuantity<
    {
        SIUnit::scaled(
            "culeus",
            Exact::dec(51868672, -8),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Amphora quadrantal (`amphora quadrantal`), a unit of `Volume`: `n` of them is `n · 0.025934336` base units.
pub const AMPHORA_QUADRANTAL: SIQuantity<
    {
        SIUnit::scaled(
            "amphora quadrantal",
            Exact::dec(25934336, -9),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Urnae (`urna`), a unit of `Volume`: `n` of them is `n · 0.012967168` base units.
pub const URNA: SIQuantity<
    {
        SIUnit::scaled(
            "urna",
            Exact::dec(12967168, -9),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Modii castrensis (`modius castrensis`), a unit of `Volume`: `n` of them is `n · 0.012967168` base units.
pub const MODIUS_CASTRENSIS: SIQuantity<
    {
        SIUnit::scaled(
            "modius castrensis",
            Exact::dec(12967168, -9),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Modii (`modius`), a unit of `Volume`: `n` of them is `n · 0.003241792 * 16.0 / 6.0` base units.
pub const MODIUS: SIQuantity<
    {
        SIUnit::scaled(
            "modius",
            Exact::dec(3241792, -9) * Exact::int(16) / Exact::int(6),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Semimodii (`semimodius`), a unit of `Volume`: `n` of them is `n · 0.003241792 * 8.0 / 6.0` base units.
pub const SEMIMODIUS: SIQuantity<
    {
        SIUnit::scaled(
            "semimodius",
            Exact::dec(3241792, -9) * Exact::int(8) / Exact::int(6),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Congii (`congius`), a unit of `Volume`: `n` of them is `n · 0.003241792` base units.
pub const CONGIUS: SIQuantity<
    {
        SIUnit::scaled(
            "congius",
            Exact::dec(3241792, -9),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Sextarii (`sextarius`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 6.0` base units.
pub const SEXTARIUS: SIQuantity<
    {
        SIUnit::scaled(
            "sextarius",
            Exact::dec(3241792, -9) / Exact::int(6),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Heminae (`hemina`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 12.0` base units.
pub const HEMINA: SIQuantity<
    {
        SIUnit::scaled(
            "hemina",
            Exact::dec(3241792, -9) / Exact::int(12),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Quartarii (`quartarius`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 24.0` base units.
pub const QUARTARIUS: SIQuantity<
    {
        SIUnit::scaled(
            "quartarius",
            Exact::dec(3241792, -9) / Exact::int(24),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Acetabula (`acetabulum`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 48.0` base units.
pub const ACETABULUM: SIQuantity<
    {
        SIUnit::scaled(
            "acetabulum",
            Exact::dec(3241792, -9) / Exact::int(48),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Cyathi (`cyathus`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 72.0` base units.
pub const CYATHUS: SIQuantity<
    {
        SIUnit::scaled(
            "cyathus",
            Exact::dec(3241792, -9) / Exact::int(72),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Ligula (`ligula`), a unit of `Volume`: `n` of them is `n · 0.003241792 / 288.0` base units.
pub const LIGULA: SIQuantity<
    {
        SIUnit::scaled(
            "ligula",
            Exact::dec(3241792, -9) / Exact::int(288),
            SIUnit::ONE.times(METRE.unit(), 3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A velocity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Velocity<
    N,
    const U: SIUnit = { METRE.unit().times(SECOND.unit(), -1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Knots (`kn`), a unit of `Velocity`: `n` of them is `n · 1852.0 / 3600.0` base units.
pub const KNOT: SIQuantity<
    {
        SIUnit::scaled(
            "kn",
            Exact::int(1852) / Exact::int(3600),
            METRE.unit().times(SECOND.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Atomic units of velocity (`a₀ · Eₕ/ħ`), a unit of `Velocity`: `n` of them is `n · 2187691.26364` base units.
pub const ATOMIC_UNIT_OF_VELOCITY: SIQuantity<
    {
        SIUnit::scaled(
            "a₀ · Eₕ/ħ",
            Exact::dec(218769126364, -5),
            METRE.unit().times(SECOND.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Natural units of velocity (`c`), a unit of `Velocity`: `n` of them is `n · 299792458.0` base units.
pub const NATURAL_UNIT_OF_VELOCITY: SIQuantity<
    {
        SIUnit::scaled(
            "c",
            Exact::int(299792458),
            METRE.unit().times(SECOND.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Speeds of light in vacuum (`c`), a unit of `Velocity`: `n` of them is `n · 299792458.0` base units.
pub const SPEED_OF_LIGHT: SIQuantity<
    {
        SIUnit::scaled(
            "c",
            Exact::int(299792458),
            METRE.unit().times(SECOND.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// An acceleration, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Acceleration<
    N,
    const U: SIUnit = { METRE.unit().times(SECOND.unit(), -2) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Galileos (`Gal`), a unit of `Acceleration`: `n` of them is `n · 0.01` base units.
pub const GALILEO: SIQuantity<
    {
        SIUnit::scaled(
            "Gal",
            Exact::dec(1, -2),
            METRE.unit().times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Standard accelerations of gravity (`g₀`), a unit of `Acceleration`: `n` of them is `n · 9.80665` base units.
pub const STANDARD_GRAVITY: SIQuantity<
    {
        SIUnit::scaled(
            "g₀",
            Exact::dec(980665, -5),
            METRE.unit().times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A jerk, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Jerk<
    N,
    const U: SIUnit = { METRE.unit().times(SECOND.unit(), -3) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// An absement, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Absement<
    N,
    const U: SIUnit = { METRE.unit().times(SECOND.unit(), 1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Metre seconds (`m · s`), a unit of `Absement`: the coherent unit, a product of base units.
pub const METRE_SECOND: SIQuantity<{ METRE.unit().times(SECOND.unit(), 1) }, Exact, true> =
    SIQuantity::ONE;

/// Foot seconds (`ft · s`), a unit of `Absement`: `n` of them is `n · 0.3048` base units.
pub const FOOT_SECOND: SIQuantity<
    {
        SIUnit::scaled(
            "ft · s",
            Exact::dec(3048, -4),
            METRE.unit().times(SECOND.unit(), 1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Inch seconds (`in · s`), a unit of `Absement`: `n` of them is `n · 0.0254` base units.
pub const INCH_SECOND: SIQuantity<
    {
        SIUnit::scaled(
            "in · s",
            Exact::dec(254, -4),
            METRE.unit().times(SECOND.unit(), 1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Kilometre hours (`km · h`), a unit of `Absement`: `n` of them is `n · 3600000.0` base units.
pub const KILOMETRE_HOUR: SIQuantity<
    {
        SIUnit::scaled(
            "km · h",
            Exact::int(3600000),
            METRE.unit().times(SECOND.unit(), 1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// An inverse velocity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type InverseVelocity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -1).times(SECOND.unit(), 1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A force, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Force<
    N,
    const U: SIUnit = {
        METRE
            .unit()
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Newtons (`N`), a unit of `Force`: the coherent unit, a product of base units.
pub const NEWTON: SIQuantity<
    {
        METRE
            .unit()
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
    },
    Exact,
    true,
> = SIQuantity::ONE;

/// Dynes (`dyn`), a unit of `Force`: `n` of them is `n · 1e-05` base units.
pub const DYNE: SIQuantity<
    {
        SIUnit::scaled(
            "dyn",
            Exact::dec(1, -5),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Kilograms-force (`kgf`), a unit of `Force`: `n` of them is `n · 9.80665` base units.
pub const KILOGRAM_FORCE: SIQuantity<
    {
        SIUnit::scaled(
            "kgf",
            Exact::dec(980665, -5),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Kips (`kip`), a unit of `Force`: `n` of them is `n · 1000.0 * 0.45359237 * 9.80665` base units.
pub const KIP: SIQuantity<
    {
        SIUnit::scaled(
            "kip",
            Exact::int(1000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Ounces-force (`ozf`), a unit of `Force`: `n` of them is `n · 0.45359237 * 9.80665 / 16.0` base units.
pub const OUNCE_FORCE: SIQuantity<
    {
        SIUnit::scaled(
            "ozf",
            Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Poundals (`pdl`), a unit of `Force`: `n` of them is `n · 0.45359237 * 0.3048` base units.
pub const POUNDAL: SIQuantity<
    {
        SIUnit::scaled(
            "pdl",
            Exact::dec(45359237, -8) * Exact::dec(3048, -4),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pounds-force (`lbf`), a unit of `Force`: `n` of them is `n · 0.45359237 * 9.80665` base units.
pub const POUND_FORCE: SIQuantity<
    {
        SIUnit::scaled(
            "lbf",
            Exact::dec(45359237, -8) * Exact::dec(980665, -5),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Tons-force (`2000 lbf`), a unit of `Force`: `n` of them is `n · 2000.0 * 0.45359237 * 9.80665` base units.
pub const TON_FORCE: SIQuantity<
    {
        SIUnit::scaled(
            "2000 lbf",
            Exact::int(2000) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A momentum, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Momentum<
    N,
    const U: SIUnit = {
        METRE
            .unit()
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// An energy, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Energy<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Joules (`J`), a unit of `Energy`: the coherent unit, a product of base units.
pub const JOULE: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
    },
    Exact,
    true,
> = SIQuantity::ONE;

/// Watt hours (`W · h`), a unit of `Energy`: `n` of them is `n · 3600.0` base units.
pub const WATT_HOUR: SIQuantity<
    {
        SIUnit::scaled(
            "W · h",
            Exact::int(3600),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Electronvolts (`eV`), a unit of `Energy`: `n` of them is `n · 1.602176634e-19` base units.
pub const ELECTRONVOLT: SIQuantity<
    {
        SIUnit::scaled(
            "eV",
            Exact::dec(1602176634, -28),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Hartrees (`Eₕ`), a unit of `Energy`: `n` of them is `n · 4.3597447222071e-18` base units.
pub const HARTREE: SIQuantity<
    {
        SIUnit::scaled(
            "Eₕ",
            Exact::dec(43597447222071, -31),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// British thermal units (IT) (`Btu (IT)`), a unit of `Energy`: `n` of them is `n · 1055.05585262` base units.
pub const BTU_IT: SIQuantity<
    {
        SIUnit::scaled(
            "Btu (IT)",
            Exact::dec(105505585262, -8),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// British thermal units (`Btu`), a unit of `Energy`: `n` of them is `n · 4184.0 * 0.45359237 / 1.8` base units.
pub const BTU: SIQuantity<
    {
        SIUnit::scaled(
            "Btu",
            Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Calories (IT) (`cal (IT)`), a unit of `Energy`: `n` of them is `n · 4.1868` base units.
pub const CALORIE_IT: SIQuantity<
    {
        SIUnit::scaled(
            "cal (IT)",
            Exact::dec(41868, -4),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Calories (`cal`), a unit of `Energy`: `n` of them is `n · 4.184` base units.
pub const CALORIE: SIQuantity<
    {
        SIUnit::scaled(
            "cal",
            Exact::dec(4184, -3),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Calories (IT) (`Cal (IT)`), a unit of `Energy`: `n` of them is `n · 4186.8` base units.
pub const CALORIE_IT_NUTRITION: SIQuantity<
    {
        SIUnit::scaled(
            "Cal (IT)",
            Exact::dec(41868, -1),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Calories (`Cal`), a unit of `Energy`: `n` of them is `n · 4184.0` base units.
pub const CALORIE_NUTRITION: SIQuantity<
    {
        SIUnit::scaled(
            "Cal",
            Exact::int(4184),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Ergs (`erg`), a unit of `Energy`: `n` of them is `n · 1e-07` base units.
pub const ERG: SIQuantity<
    {
        SIUnit::scaled(
            "erg",
            Exact::dec(1, -7),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Foot poundals (`ft · pdl`), a unit of `Energy`: `n` of them is `n · 0.3048 * 0.45359237 * 0.3048` base units.
pub const FOOT_POUNDAL: SIQuantity<
    {
        SIUnit::scaled(
            "ft · pdl",
            Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Foot pounds-force (`ft · lbf`), a unit of `Energy`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` base units.
pub const FOOT_POUND: SIQuantity<
    {
        SIUnit::scaled(
            "ft · lbf",
            Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Quads (`10¹⁵ Btu (IT)`), a unit of `Energy`: `n` of them is `n · 1.05505585262e18` base units.
pub const QUAD: SIQuantity<
    {
        SIUnit::scaled(
            "10¹⁵ Btu (IT)",
            Exact::int(1055055852620000000),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Therms (EC) (`thm (EC)`), a unit of `Energy`: `n` of them is `n · 105506000.0` base units.
pub const THERM_EC: SIQuantity<
    {
        SIUnit::scaled(
            "thm (EC)",
            Exact::int(105506000),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Therms (`thm`), a unit of `Energy`: `n` of them is `n · 105480400.0` base units.
pub const THERM_US: SIQuantity<
    {
        SIUnit::scaled(
            "thm",
            Exact::int(105480400),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Tons of TNT (`t of TNT`), a unit of `Energy`: `n` of them is `n · 4184000000.0` base units.
pub const TON_TNT: SIQuantity<
    {
        SIUnit::scaled(
            "t of TNT",
            Exact::int(4184000000),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Watt seconds (`W · s`), a unit of `Energy`: the coherent unit, a product of base units.
pub const WATT_SECOND: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
    },
    Exact,
    true,
> = SIQuantity::ONE;

/// An available energy, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type AvailableEnergy<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 2).times(SECOND.unit(), -2) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A power, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Power<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Watts (`W`), a unit of `Power`: the coherent unit, a product of base units.
pub const WATT: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
    },
    Exact,
    true,
> = SIQuantity::ONE;

/// Horsepower (`hp`), a unit of `Power`: `n` of them is `n · 550.0 * 0.3048 * 0.45359237 * 9.80665` base units.
pub const HORSEPOWER: SIQuantity<
    {
        SIUnit::scaled(
            "hp",
            Exact::int(550)
                * Exact::dec(3048, -4)
                * Exact::dec(45359237, -8)
                * Exact::dec(980665, -5),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Horsepower (boiler) (`hp (S)`), a unit of `Power`: `n` of them is `n · 9809.5` base units.
pub const HORSEPOWER_BOILER: SIQuantity<
    {
        SIUnit::scaled(
            "hp (S)",
            Exact::dec(98095, -1),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Horsepower (electric) (`hp (E)`), a unit of `Power`: `n` of them is `n · 746.0` base units.
pub const HORSEPOWER_ELECTRIC: SIQuantity<
    {
        SIUnit::scaled(
            "hp (E)",
            Exact::int(746),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Metric horsepower (`hp (M)`), a unit of `Power`: `n` of them is `n · 75.0 * 9.80665` base units.
pub const HORSEPOWER_METRIC: SIQuantity<
    {
        SIUnit::scaled(
            "hp (M)",
            Exact::int(75) * Exact::dec(980665, -5),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Horsepower (Imperial) (`hp (I)`), a unit of `Power`: `n` of them is `n · 745.7` base units.
pub const HORSEPOWER_IMPERIAL: SIQuantity<
    {
        SIUnit::scaled(
            "hp (I)",
            Exact::dec(7457, -1),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Hydraulic horsepower (`hp (hydraulic)`), a unit of `Power`: `n` of them is `n · 746.043` base units.
pub const HYDRAULIC_HORSEPOWER: SIQuantity<
    {
        SIUnit::scaled(
            "hp (hydraulic)",
            Exact::dec(746043, -3),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Decibels-watt (`dBW`), a unit of `Power`: `n` of them is `1.0 · 10.0^(n / 10.0)` base units.
pub const DECIBEL_WATT: SIQuantity<
    {
        SIUnit::from_log(
            "dBW",
            Exact::int(10),
            Exact::int(10),
            Exact::int(1),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Decibels-milliwatt (`dBm`), a unit of `Power`: `n` of them is `0.001 · 10.0^(n / 10.0)` base units.
pub const DECIBEL_MILLIWATT: SIQuantity<
    {
        SIUnit::from_log(
            "dBm",
            Exact::int(10),
            Exact::int(10),
            Exact::dec(1, -3),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A power rate, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type PowerRate<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -4)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// An action, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Action<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Joule seconds (`J · s`), a unit of `Action`: the coherent unit, a product of base units.
pub const JOULE_SECOND: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -1)
    },
    Exact,
    true,
> = SIQuantity::ONE;

/// Atomic units of action (`ħ`), a unit of `Action`: `n` of them is `n · 6.62607015e-34 / (2.0 * core::f64::consts::PI)` base units.
pub const ATOMIC_UNIT_OF_ACTION: SIQuantity<
    {
        SIUnit::scaled(
            "ħ",
            Exact::dec(662607015, -42) / (Exact::int(2) * Exact::PI),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Planck constants (`h`), a unit of `Action`: `n` of them is `n · 6.62607015e-34` base units.
pub const PLANCK_CONSTANT: SIQuantity<
    {
        SIUnit::scaled(
            "h",
            Exact::dec(662607015, -42),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Erg seconds (`erg · s`), a unit of `Action`: `n` of them is `n · 1e-07` base units.
pub const ERG_SECOND: SIQuantity<
    {
        SIUnit::scaled(
            "erg · s",
            Exact::dec(1, -7),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Electronvolt seconds (`eV · s`), a unit of `Action`: `n` of them is `n · 1.602176634e-19` base units.
pub const ELECTRONVOLT_SECOND: SIQuantity<
    {
        SIUnit::scaled(
            "eV · s",
            Exact::dec(1602176634, -28),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A torque, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Torque<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(RADIAN.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Newton metres (`N · m`), a unit of `Torque`: the coherent unit, a product of base units.
pub const NEWTON_METRE: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(RADIAN.unit(), -1)
    },
    Exact,
    true,
> = SIQuantity::ONE;

/// Dyne metres (`dyn · m`), a unit of `Torque`: `n` of them is `n · 1e-05` base units.
pub const DYNE_METRE: SIQuantity<
    {
        SIUnit::scaled(
            "dyn · m",
            Exact::dec(1, -5),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(RADIAN.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Kilogram-force metres (`kgf · m`), a unit of `Torque`: `n` of them is `n · 9.80665` base units.
pub const KILOGRAM_FORCE_METRE: SIQuantity<
    {
        SIUnit::scaled(
            "kgf · m",
            Exact::dec(980665, -5),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(RADIAN.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Ounces-force inches (`ozf · in`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665 / 16.0) * 0.0254` base units.
pub const OUNCE_FORCE_INCH: SIQuantity<
    {
        SIUnit::scaled(
            "ozf · in",
            (Exact::dec(45359237, -8) * Exact::dec(980665, -5) / Exact::int(16))
                * Exact::dec(254, -4),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(RADIAN.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pounds-force feet (`lbf · ft`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665) * 0.3048` base units.
pub const POUND_FORCE_FOOT: SIQuantity<
    {
        SIUnit::scaled(
            "lbf · ft",
            (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(3048, -4),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(RADIAN.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pounds-force inches (`lbf · in`), a unit of `Torque`: `n` of them is `n · (0.45359237 * 9.80665) * 0.0254` base units.
pub const POUND_FORCE_INCH: SIQuantity<
    {
        SIUnit::scaled(
            "lbf · in",
            (Exact::dec(45359237, -8) * Exact::dec(980665, -5)) * Exact::dec(254, -4),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(RADIAN.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A moment of inertia, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MomentOfInertia<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(RADIAN.unit(), -2)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Kilogram square metres (`kg · m²`), a unit of `MomentOfInertia`: the coherent unit, a product of base units.
pub const KILOGRAM_SQUARE_METRE: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(RADIAN.unit(), -2)
    },
    Exact,
    true,
> = SIQuantity::ONE;

/// Dalton square ångströms (`Da · Å²`), a unit of `MomentOfInertia`: `n` of them is `n · 1.6605390666e-47` base units.
pub const DALTON_SQUARE_ANGSTROM: SIQuantity<
    {
        SIUnit::scaled(
            "Da · Å²",
            Exact::dec(16605390666, -57),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(RADIAN.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A pressure, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Pressure<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Pascals (`Pa`), a unit of `Pressure`: the coherent unit, a product of base units.
pub const PASCAL: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
    },
    Exact,
    true,
> = SIQuantity::ONE;

/// Standard atmospheres (`atm`), a unit of `Pressure`: `n` of them is `n · 101325.0` base units.
pub const STANDARD_ATMOSPHERE: SIQuantity<
    {
        SIUnit::scaled(
            "atm",
            Exact::int(101325),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Atmospheres (technical) (`at`), a unit of `Pressure`: `n` of them is `n · 98066.5` base units.
pub const ATMOSPHERE_TECHNICAL: SIQuantity<
    {
        SIUnit::scaled(
            "at",
            Exact::dec(980665, -1),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Bar (`bar`), a unit of `Pressure`: `n` of them is `n · 100000.0` base units.
pub const BAR: SIQuantity<
    {
        SIUnit::scaled(
            "bar",
            Exact::int(100000),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Centimetres of mercury (`cm Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 10.0` base units.
pub const CENTIMETRE_OF_MERCURY: SIQuantity<
    {
        SIUnit::scaled(
            "cm Hg",
            Exact::dec(133322387415, -9) * Exact::int(10),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Centimetres of water (`cm H₂O`), a unit of `Pressure`: `n` of them is `n · 98.0665` base units.
pub const CENTIMETRE_OF_WATER: SIQuantity<
    {
        SIUnit::scaled(
            "cm H₂O",
            Exact::dec(980665, -4),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Feet of mercury (`ft Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 304.8` base units.
pub const FOOT_OF_MERCURY: SIQuantity<
    {
        SIUnit::scaled(
            "ft Hg",
            Exact::dec(133322387415, -9) * Exact::dec(3048, -1),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Feet of water (`ft H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665 * 304.8` base units.
pub const FOOT_OF_WATER: SIQuantity<
    {
        SIUnit::scaled(
            "ft H₂O",
            Exact::dec(980665, -5) * Exact::dec(3048, -1),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Inches of mercury (`in Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415 * 25.4` base units.
pub const INCH_OF_MERCURY: SIQuantity<
    {
        SIUnit::scaled(
            "in Hg",
            Exact::dec(133322387415, -9) * Exact::dec(254, -1),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Inches of water (`in H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665 * 25.4` base units.
pub const INCH_OF_WATER: SIQuantity<
    {
        SIUnit::scaled(
            "in H₂O",
            Exact::dec(980665, -5) * Exact::dec(254, -1),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Millimetres of mercury (`mm Hg`), a unit of `Pressure`: `n` of them is `n · 133.322387415` base units.
pub const MILLIMETRE_OF_MERCURY: SIQuantity<
    {
        SIUnit::scaled(
            "mm Hg",
            Exact::dec(133322387415, -9),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Millimetres of water (`mm H₂O`), a unit of `Pressure`: `n` of them is `n · 9.80665` base units.
pub const MILLIMETRE_OF_WATER: SIQuantity<
    {
        SIUnit::scaled(
            "mm H₂O",
            Exact::dec(980665, -5),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Pounds-force per square inch (`psi`), a unit of `Pressure`: `n` of them is `n · 0.45359237 * 9.80665 / (0.0254 * 0.0254)` base units.
pub const PSI: SIQuantity<
    {
        SIUnit::scaled(
            "psi",
            Exact::dec(45359237, -8) * Exact::dec(980665, -5)
                / (Exact::dec(254, -4) * Exact::dec(254, -4)),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Torr (`Torr`), a unit of `Pressure`: `n` of them is `n · 101325.0 / 760.0` base units.
pub const TORR: SIQuantity<
    {
        SIUnit::scaled(
            "Torr",
            Exact::int(101325) / Exact::int(760),
            SIUnit::ONE
                .times(METRE.unit(), -1)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A frequency, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Frequency<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Hertz (`Hz`), a unit of `Frequency`: the coherent unit, a product of base units.
pub const HERTZ: SIQuantity<{ SIUnit::ONE.times(SECOND.unit(), -1) }, Exact, true> =
    SIQuantity::ONE;

/// A frequency drift, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type FrequencyDrift<
    N,
    const U: SIUnit = { SIUnit::ONE.times(SECOND.unit(), -2) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A reciprocal length, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ReciprocalLength<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Reciprocal metres (`m⁻¹`), a unit of `ReciprocalLength`: the coherent unit, a product of base units.
pub const RECIPROCAL_METRE: SIQuantity<{ SIUnit::ONE.times(METRE.unit(), -1) }, Exact, true> =
    SIQuantity::ONE;

/// Reciprocal ångströms (`Å⁻¹`), a unit of `ReciprocalLength`: `n` of them is `n · 10000000000.0` base units.
pub const RECIPROCAL_ANGSTROM: SIQuantity<
    {
        SIUnit::scaled(
            "Å⁻¹",
            Exact::int(10000000000),
            SIUnit::ONE.times(METRE.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// Diopters (`dpt`), a unit of `ReciprocalLength`: the coherent unit, a product of base units.
pub const DIOPTER: SIQuantity<{ SIUnit::ONE.times(METRE.unit(), -1) }, Exact, true> =
    SIQuantity::ONE;

/// Rydberg constants (`R∞`), a unit of `ReciprocalLength`: `n` of them is `n · 10973731.56816` base units.
pub const RYDBERG_CONSTANT: SIQuantity<
    {
        SIUnit::scaled(
            "R∞",
            Exact::dec(1097373156816, -5),
            SIUnit::ONE.times(METRE.unit(), -1),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;

/// A curvature, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Curvature<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -1).times(RADIAN.unit(), 1) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A specific area, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SpecificArea<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A specific volume, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SpecificVolume<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 3)
            .times(KILOGRAM.unit(), -1)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A specific power, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SpecificPower<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 2).times(SECOND.unit(), -3) },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// A specific gravitational parameter, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SpecificGravitationalParameter<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 3)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), -2)
    },
    const IS_BASE: bool = true,
> = SIQuantity<U, N, IS_BASE>;

/// Newtonian constants of gravitation (`G`), a unit of `SpecificGravitationalParameter`: `n` of them is `n · 6.6743e-11` base units.
pub const GRAVITATIONAL_CONSTANT: SIQuantity<
    {
        SIUnit::scaled(
            "G",
            Exact::dec(66743, -15),
            SIUnit::ONE
                .times(METRE.unit(), 3)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    false,
> = SIQuantity::ONE;
