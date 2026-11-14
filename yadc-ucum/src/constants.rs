//! The quantities of `constants` and their units.

#[allow(unused_imports)]
use crate::*;

// `Acceleration` is `crate::Acceleration`.

/// Standard acceleration of free fall (`[g]`), a unit of `Acceleration`: `n` of them is `n · 980665e-5` base units.
pub const STANDARD_ACCELERATION_OF_FREE_FALL: Unit<
    {
        UcumUnit::scaled(
            "[g]",
            Exact::dec(980665, -5),
            METER_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// An action, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED_GRAM_PER_SECOND`.
pub type Action<
    N,
    const U: UcumUnit = { METER_SQUARED_GRAM_PER_SECOND.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `Action`: a product of base units.
pub const METER_SQUARED_GRAM_PER_SECOND: Unit<{ (METER * METER * GRAM / SECOND).unit() }, BASE> =
    Unit::ONE;

/// Planck constant (`[h]`), a unit of `Action`: `n` of them is `n · 662607015e-39` base units.
pub const PLANCK_CONSTANT: Unit<
    {
        UcumUnit::scaled(
            "[h]",
            Exact::dec(662607015, -39),
            METER_SQUARED_GRAM_PER_SECOND.unit(),
        )
    },
> = Unit::ONE;

// `ElectricCharge` is `crate::ElectricCharge`.

/// Elementary charge (`[e]`), a unit of `ElectricCharge`: `n` of them is `n · 1602176634e-28` base units.
pub const ELEMENTARY_CHARGE: Unit<
    { UcumUnit::scaled("[e]", Exact::dec(1602176634, -28), COULOMB.unit()) },
> = Unit::ONE;

/// An electric permittivity, in `N`s of the unit `U`: by default its base unit, `SECOND_SQUARED_COULOMB_SQUARED_PER_METER_CUBED_GRAM`.
pub type ElectricPermittivity<
    N,
    const U: UcumUnit = { SECOND_SQUARED_COULOMB_SQUARED_PER_METER_CUBED_GRAM.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `ElectricPermittivity`: a product of base units.
pub const SECOND_SQUARED_COULOMB_SQUARED_PER_METER_CUBED_GRAM: Unit<
    { (SECOND * SECOND * COULOMB * COULOMB / (METER * METER * METER * GRAM)).unit() },
    BASE,
> = Unit::ONE;

/// Permittivity of vacuum (`[eps_0]`), a unit of `ElectricPermittivity`: `n` of them is `n · 8854187817e-24` base units.
pub const PERMITTIVITY_OF_VACUUM: Unit<
    {
        UcumUnit::scaled(
            "[eps_0]",
            Exact::dec(8854187817, -24),
            SECOND_SQUARED_COULOMB_SQUARED_PER_METER_CUBED_GRAM.unit(),
        )
    },
> = Unit::ONE;

// `Force` is `crate::Force`.

/// Gram-force (`gf`), a unit of `Force`: `n` of them is `n · 980665e-5` base units.
pub const GRAM_FORCE: Unit<
    {
        UcumUnit::scaled(
            "gf",
            Exact::dec(980665, -5),
            METER_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// Pound force (`[lbf_av]`), a unit of `Force`: `n` of them is `n · 44482216152605e-10` base units.
pub const POUND_FORCE: Unit<
    {
        UcumUnit::scaled(
            "[lbf_av]",
            Exact::dec(44482216152605, -10),
            METER_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `Length` is `crate::Length`.

/// Light-year (`[ly]`), a unit of `Length`: `n` of them is `n · 94607304725808e2` base units.
pub const LIGHT_YEAR: Unit<
    { UcumUnit::scaled("[ly]", Exact::int(9460730472580800), METER.unit()) },
> = Unit::ONE;

/// A magnetic permeability, in `N`s of the unit `U`: by default its base unit, `METER_GRAM_PER_COULOMB_SQUARED`.
pub type MagneticPermeability<
    N,
    const U: UcumUnit = { METER_GRAM_PER_COULOMB_SQUARED.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `MagneticPermeability`: a product of base units.
pub const METER_GRAM_PER_COULOMB_SQUARED: Unit<
    { (METER * GRAM / (COULOMB * COULOMB)).unit() },
    BASE,
> = Unit::ONE;

/// Permeability of vacuum (`[mu_0]`), a unit of `MagneticPermeability`: `n` of them is `n · 4e-4 * core::f64::consts::PI` base units.
pub const PERMEABILITY_OF_VACUUM: Unit<
    {
        UcumUnit::scaled(
            "[mu_0]",
            Exact::dec(4, -4) * Exact::PI,
            METER_GRAM_PER_COULOMB_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `Mass` is `crate::Mass`.

/// Electron mass (`[m_e]`), a unit of `Mass`: `n` of them is `n · 91093837139e-38` base units.
pub const ELECTRON_MASS: Unit<
    { UcumUnit::scaled("[m_e]", Exact::dec(91093837139, -38), GRAM.unit()) },
> = Unit::ONE;

/// Proton mass (`[m_p]`), a unit of `Mass`: `n` of them is `n · 167262192595e-35` base units.
pub const PROTON_MASS: Unit<
    { UcumUnit::scaled("[m_p]", Exact::dec(167262192595, -35), GRAM.unit()) },
> = Unit::ONE;

// `Pressure` is `crate::Pressure`.

/// Standard atmosphere (`atm`), a unit of `Pressure`: `n` of them is `n · 101325e3` base units.
pub const STANDARD_ATMOSPHERE: Unit<
    {
        UcumUnit::scaled(
            "atm",
            Exact::int(101325000),
            GRAM_PER_METER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// An unclassified m2 g per s2 k, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED_GRAM_PER_SECOND_SQUARED_KELVIN`.
pub type UnclassifiedM2GPerS2K<
    N,
    const U: UcumUnit = { METER_SQUARED_GRAM_PER_SECOND_SQUARED_KELVIN.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `UnclassifiedM2GPerS2K`: a product of base units.
pub const METER_SQUARED_GRAM_PER_SECOND_SQUARED_KELVIN: Unit<
    { (METER * METER * GRAM / (SECOND * SECOND * KELVIN)).unit() },
    BASE,
> = Unit::ONE;

/// Boltzmann constant (`[k]`), a unit of `UnclassifiedM2GPerS2K`: `n` of them is `n · 1380649e-26` base units.
pub const BOLTZMANN_CONSTANT: Unit<
    {
        UcumUnit::scaled(
            "[k]",
            Exact::dec(1380649, -26),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED_KELVIN.unit(),
        )
    },
> = Unit::ONE;

/// An unclassified m3 per s2 g, in `N`s of the unit `U`: by default its base unit, `METER_CUBED_PER_SECOND_SQUARED_GRAM`.
pub type UnclassifiedM3PerS2G<
    N,
    const U: UcumUnit = { METER_CUBED_PER_SECOND_SQUARED_GRAM.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `UnclassifiedM3PerS2G`: a product of base units.
pub const METER_CUBED_PER_SECOND_SQUARED_GRAM: Unit<
    { (METER * METER * METER / (SECOND * SECOND * GRAM)).unit() },
    BASE,
> = Unit::ONE;

/// Newtonian constant of gravitation (`[G]`), a unit of `UnclassifiedM3PerS2G`: `n` of them is `n · 66743e-18` base units.
pub const NEWTONIAN_CONSTANT_OF_GRAVITATION: Unit<
    {
        UcumUnit::scaled(
            "[G]",
            Exact::dec(66743, -18),
            METER_CUBED_PER_SECOND_SQUARED_GRAM.unit(),
        )
    },
> = Unit::ONE;

// `Velocity` is `crate::Velocity`.

/// Velocity of light (`[c]`), a unit of `Velocity`: `n` of them is `n · 299792458` base units.
pub const VELOCITY_OF_LIGHT: Unit<
    { UcumUnit::scaled("[c]", Exact::int(299792458), METER_PER_SECOND.unit()) },
> = Unit::ONE;
