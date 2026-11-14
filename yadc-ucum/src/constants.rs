//! The quantities of `constants` and their units.

#[allow(unused_imports)]
use crate::*;

// `Acceleration` is `crate::Acceleration`.

/// Standard acceleration of free fall (`[g]`), a unit of `Acceleration`: `n` of them is `n · 980665e-5` base units.
pub const STANDARD_ACCELERATION_OF_FREE_FALL: Qty<
    {
        UcumUnit::scaled(
            "[g]",
            Exact::dec(980665, -5),
            METER.unit().times(SECOND.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// An action, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Action<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -1)
            .times(GRAM.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Planck constant (`[h]`), a unit of `Action`: `n` of them is `n · 662607015e-39` base units.
pub const PLANCK_CONSTANT: Qty<
    {
        UcumUnit::scaled(
            "[h]",
            Exact::dec(662607015, -39),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `ElectricCharge` is `crate::ElectricCharge`.

/// Elementary charge (`[e]`), a unit of `ElectricCharge`: `n` of them is `n · 1602176634e-28` base units.
pub const ELEMENTARY_CHARGE: Qty<
    { UcumUnit::scaled("[e]", Exact::dec(1602176634, -28), COULOMB.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// An electric permittivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricPermittivity<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -3)
            .times(SECOND.unit(), 2)
            .times(GRAM.unit(), -1)
            .times(COULOMB.unit(), 2)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Permittivity of vacuum (`[eps_0]`), a unit of `ElectricPermittivity`: `n` of them is `n · 8854187817e-24` base units.
pub const PERMITTIVITY_OF_VACUUM: Qty<
    {
        UcumUnit::scaled(
            "[eps_0]",
            Exact::dec(8854187817, -24),
            UcumUnit::ONE
                .times(METER.unit(), -3)
                .times(SECOND.unit(), 2)
                .times(GRAM.unit(), -1)
                .times(COULOMB.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Force` is `crate::Force`.

/// Gram-force (`gf`), a unit of `Force`: `n` of them is `n · 980665e-5` base units.
pub const GRAM_FORCE: Qty<
    {
        UcumUnit::scaled(
            "gf",
            Exact::dec(980665, -5),
            METER.unit().times(SECOND.unit(), -2).times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Pound force (`[lbf_av]`), a unit of `Force`: `n` of them is `n · 44482216152605e-10` base units.
pub const POUND_FORCE: Qty<
    {
        UcumUnit::scaled(
            "[lbf_av]",
            Exact::dec(44482216152605, -10),
            METER.unit().times(SECOND.unit(), -2).times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Length` is `crate::Length`.

/// Light-year (`[ly]`), a unit of `Length`: `n` of them is `n · 94607304725808e2` base units.
pub const LIGHT_YEAR: Qty<
    { UcumUnit::scaled("[ly]", Exact::int(9460730472580800), METER.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A magnetic permeability, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticPermeability<
    N,
    const U: UcumUnit = { METER.unit().times(GRAM.unit(), 1).times(COULOMB.unit(), -2) },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Permeability of vacuum (`[mu_0]`), a unit of `MagneticPermeability`: `n` of them is `n · 4e-4 * core::f64::consts::PI` base units.
pub const PERMEABILITY_OF_VACUUM: Qty<
    {
        UcumUnit::scaled(
            "[mu_0]",
            Exact::dec(4, -4) * Exact::PI,
            METER.unit().times(GRAM.unit(), 1).times(COULOMB.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Mass` is `crate::Mass`.

/// Electron mass (`[m_e]`), a unit of `Mass`: `n` of them is `n · 91093837139e-38` base units.
pub const ELECTRON_MASS: Qty<
    { UcumUnit::scaled("[m_e]", Exact::dec(91093837139, -38), GRAM.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// Proton mass (`[m_p]`), a unit of `Mass`: `n` of them is `n · 167262192595e-35` base units.
pub const PROTON_MASS: Qty<
    { UcumUnit::scaled("[m_p]", Exact::dec(167262192595, -35), GRAM.unit()) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Pressure` is `crate::Pressure`.

/// Standard atmosphere (`atm`), a unit of `Pressure`: `n` of them is `n · 101325e3` base units.
pub const STANDARD_ATMOSPHERE: Qty<
    {
        UcumUnit::scaled(
            "atm",
            Exact::int(101325000),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// An unclassified m2 g per s2 k, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type UnclassifiedM2GPerS2K<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -2)
            .times(GRAM.unit(), 1)
            .times(KELVIN.unit(), -1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Boltzmann constant (`[k]`), a unit of `UnclassifiedM2GPerS2K`: `n` of them is `n · 1380649e-26` base units.
pub const BOLTZMANN_CONSTANT: Qty<
    {
        UcumUnit::scaled(
            "[k]",
            Exact::dec(1380649, -26),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1)
                .times(KELVIN.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// An unclassified m3 per s2 g, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type UnclassifiedM3PerS2G<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 3)
            .times(SECOND.unit(), -2)
            .times(GRAM.unit(), -1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Newtonian constant of gravitation (`[G]`), a unit of `UnclassifiedM3PerS2G`: `n` of them is `n · 66743e-18` base units.
pub const NEWTONIAN_CONSTANT_OF_GRAVITATION: Qty<
    {
        UcumUnit::scaled(
            "[G]",
            Exact::dec(66743, -18),
            UcumUnit::ONE
                .times(METER.unit(), 3)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Velocity` is `crate::Velocity`.

/// Velocity of light (`[c]`), a unit of `Velocity`: `n` of them is `n · 299792458` base units.
pub const VELOCITY_OF_LIGHT: Qty<
    {
        UcumUnit::scaled(
            "[c]",
            Exact::int(299792458),
            METER.unit().times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;
