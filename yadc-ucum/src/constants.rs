use crate::Dim;
use yadc::Dimension;

/// The dimension of `Acceleration`: `Qty<ACCELERATION, N>` is one.
pub const ACCELERATION: Dim = Dim {
    m: 1,
    s: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Action`: `Qty<ACTION, N>` is one.
pub const ACTION: Dim = Dim {
    m: 2,
    s: -1,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricCharge`: `Qty<ELECTRIC_CHARGE, N>` is one.
pub const ELECTRIC_CHARGE: Dim = Dim {
    C: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricPermittivity`: `Qty<ELECTRIC_PERMITTIVITY, N>` is one.
pub const ELECTRIC_PERMITTIVITY: Dim = Dim {
    m: -3,
    s: 2,
    g: -1,
    C: 2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Force`: `Qty<FORCE, N>` is one.
pub const FORCE: Dim = Dim {
    m: 1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Length`: `Qty<LENGTH, N>` is one.
pub const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `MagneticPermeability`: `Qty<MAGNETIC_PERMEABILITY, N>` is one.
pub const MAGNETIC_PERMEABILITY: Dim = Dim {
    m: 1,
    g: 1,
    C: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Mass`: `Qty<MASS, N>` is one.
pub const MASS: Dim = Dim {
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Pressure`: `Qty<PRESSURE, N>` is one.
pub const PRESSURE: Dim = Dim {
    m: -1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `UnclassifiedM2GPerS2K`: `Qty<UNCLASSIFIED_M2_G_PER_S2_K, N>` is one.
pub const UNCLASSIFIED_M2_G_PER_S2_K: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    K: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `UnclassifiedM3PerS2G`: `Qty<UNCLASSIFIED_M3_PER_S2_G, N>` is one.
pub const UNCLASSIFIED_M3_PER_S2_G: Dim = Dim {
    m: 3,
    s: -2,
    g: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Velocity`: `Qty<VELOCITY, N>` is one.
pub const VELOCITY: Dim = Dim {
    m: 1,
    s: -1,
    ..Dim::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Standard acceleration of free fall (`[g]`), a unit of `Acceleration`: `n` of them is `n · 980665e-5` coherent units.
    pub const STANDARD_ACCELERATION_OF_FREE_FALL: UcumUnit<
        ACCELERATION,
        {
            UcumUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "[g]",
                name: "standard acceleration of free fall",
            }
        },
    > = UcumUnit;

    /// Planck constant (`[h]`), a unit of `Action`: `n` of them is `n · 662607015e-39` coherent units.
    pub const PLANCK_CONSTANT: UcumUnit<
        ACTION,
        {
            UcumUnitConsts {
                size: Exact::dec(662607015, -39),
                zero: Exact::ZERO,
                symbol: "[h]",
                name: "Planck constant",
            }
        },
    > = UcumUnit;

    /// Elementary charge (`[e]`), a unit of `ElectricCharge`: `n` of them is `n · 1602176634e-28` coherent units.
    pub const ELEMENTARY_CHARGE: UcumUnit<
        ELECTRIC_CHARGE,
        {
            UcumUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "[e]",
                name: "elementary charge",
            }
        },
    > = UcumUnit;

    /// Permittivity of vacuum (`[eps_0]`), a unit of `ElectricPermittivity`: `n` of them is `n · 8854187817e-24` coherent units.
    pub const PERMITTIVITY_OF_VACUUM: UcumUnit<
        ELECTRIC_PERMITTIVITY,
        {
            UcumUnitConsts {
                size: Exact::dec(8854187817, -24),
                zero: Exact::ZERO,
                symbol: "[eps_0]",
                name: "permittivity of vacuum",
            }
        },
    > = UcumUnit;

    /// Gram-force (`gf`), a unit of `Force`: `n` of them is `n · 980665e-5` coherent units.
    pub const GRAM_FORCE: UcumUnit<
        FORCE,
        {
            UcumUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "gf",
                name: "gram-force",
            }
        },
    > = UcumUnit;

    /// Pound force (`[lbf_av]`), a unit of `Force`: `n` of them is `n · 44482216152605e-10` coherent units.
    pub const POUND_FORCE: UcumUnit<
        FORCE,
        {
            UcumUnitConsts {
                size: Exact::dec(44482216152605, -10),
                zero: Exact::ZERO,
                symbol: "[lbf_av]",
                name: "pound force",
            }
        },
    > = UcumUnit;

    /// Light-year (`[ly]`), a unit of `Length`: `n` of them is `n · 94607304725808e2` coherent units.
    pub const LIGHT_YEAR: UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(9460730472580800),
                zero: Exact::ZERO,
                symbol: "[ly]",
                name: "light-year",
            }
        },
    > = UcumUnit;

    /// Permeability of vacuum (`[mu_0]`), a unit of `MagneticPermeability`: `n` of them is `n · 4e-4 * core::f64::consts::PI` coherent units.
    pub const PERMEABILITY_OF_VACUUM: UcumUnit<
        MAGNETIC_PERMEABILITY,
        {
            UcumUnitConsts {
                size: Exact::dec(4, -4) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "[mu_0]",
                name: "permeability of vacuum",
            }
        },
    > = UcumUnit;

    /// Electron mass (`[m_e]`), a unit of `Mass`: `n` of them is `n · 91093837139e-38` coherent units.
    pub const ELECTRON_MASS: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(91093837139, -38),
                zero: Exact::ZERO,
                symbol: "[m_e]",
                name: "electron mass",
            }
        },
    > = UcumUnit;

    /// Proton mass (`[m_p]`), a unit of `Mass`: `n` of them is `n · 167262192595e-35` coherent units.
    pub const PROTON_MASS: UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(167262192595, -35),
                zero: Exact::ZERO,
                symbol: "[m_p]",
                name: "proton mass",
            }
        },
    > = UcumUnit;

    /// Standard atmosphere (`atm`), a unit of `Pressure`: `n` of them is `n · 101325e3` coherent units.
    pub const STANDARD_ATMOSPHERE: UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(101325000),
                zero: Exact::ZERO,
                symbol: "atm",
                name: "standard atmosphere",
            }
        },
    > = UcumUnit;

    /// Boltzmann constant (`[k]`), a unit of `UnclassifiedM2GPerS2K`: `n` of them is `n · 1380649e-26` coherent units.
    pub const BOLTZMANN_CONSTANT: UcumUnit<
        UNCLASSIFIED_M2_G_PER_S2_K,
        {
            UcumUnitConsts {
                size: Exact::dec(1380649, -26),
                zero: Exact::ZERO,
                symbol: "[k]",
                name: "Boltzmann constant",
            }
        },
    > = UcumUnit;

    /// Newtonian constant of gravitation (`[G]`), a unit of `UnclassifiedM3PerS2G`: `n` of them is `n · 66743e-18` coherent units.
    pub const NEWTONIAN_CONSTANT_OF_GRAVITATION: UcumUnit<
        UNCLASSIFIED_M3_PER_S2_G,
        {
            UcumUnitConsts {
                size: Exact::dec(66743, -18),
                zero: Exact::ZERO,
                symbol: "[G]",
                name: "Newtonian constant of gravitation",
            }
        },
    > = UcumUnit;

    /// Velocity of light (`[c]`), a unit of `Velocity`: `n` of them is `n · 299792458` coherent units.
    pub const VELOCITY_OF_LIGHT: UcumUnit<
        VELOCITY,
        {
            UcumUnitConsts {
                size: Exact::int(299792458),
                zero: Exact::ZERO,
                symbol: "[c]",
                name: "velocity of light",
            }
        },
    > = UcumUnit;
}
