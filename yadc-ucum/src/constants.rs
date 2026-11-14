use crate::{Dim, Qty};
use yadc::Dimension;

/// The dimension of `Acceleration`.
pub const ACCELERATION: Dim = Dim {
    m: 1,
    s: -2,
    ..Dim::DIMENSIONLESS
};
pub type Acceleration<N> = Qty<ACCELERATION, N>;

/// The dimension of `Action`.
pub const ACTION: Dim = Dim {
    m: 2,
    s: -1,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Action<N> = Qty<ACTION, N>;

/// The dimension of `ElectricCharge`.
pub const ELECTRIC_CHARGE: Dim = Dim {
    C: 1,
    ..Dim::DIMENSIONLESS
};
pub type ElectricCharge<N> = Qty<ELECTRIC_CHARGE, N>;

/// The dimension of `ElectricPermittivity`.
pub const ELECTRIC_PERMITTIVITY: Dim = Dim {
    m: -3,
    s: 2,
    g: -1,
    C: 2,
    ..Dim::DIMENSIONLESS
};
pub type ElectricPermittivity<N> = Qty<ELECTRIC_PERMITTIVITY, N>;

/// The dimension of `Force`.
pub const FORCE: Dim = Dim {
    m: 1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Force<N> = Qty<FORCE, N>;

/// The dimension of `Length`.
pub const LENGTH: Dim = Dim {
    m: 1,
    ..Dim::DIMENSIONLESS
};
pub type Length<N> = Qty<LENGTH, N>;

/// The dimension of `MagneticPermeability`.
pub const MAGNETIC_PERMEABILITY: Dim = Dim {
    m: 1,
    g: 1,
    C: -2,
    ..Dim::DIMENSIONLESS
};
pub type MagneticPermeability<N> = Qty<MAGNETIC_PERMEABILITY, N>;

/// The dimension of `Mass`.
pub const MASS: Dim = Dim {
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Mass<N> = Qty<MASS, N>;

/// The dimension of `Pressure`.
pub const PRESSURE: Dim = Dim {
    m: -1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Pressure<N> = Qty<PRESSURE, N>;

/// The dimension of `UnclassifiedM2GPerS2K`.
pub const UNCLASSIFIED_M2_G_PER_S2_K: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    K: -1,
    ..Dim::DIMENSIONLESS
};
pub type UnclassifiedM2GPerS2K<N> = Qty<UNCLASSIFIED_M2_G_PER_S2_K, N>;

/// The dimension of `UnclassifiedM3PerS2G`.
pub const UNCLASSIFIED_M3_PER_S2_G: Dim = Dim {
    m: 3,
    s: -2,
    g: -1,
    ..Dim::DIMENSIONLESS
};
pub type UnclassifiedM3PerS2G<N> = Qty<UNCLASSIFIED_M3_PER_S2_G, N>;

/// The dimension of `Velocity`.
pub const VELOCITY: Dim = Dim {
    m: 1,
    s: -1,
    ..Dim::DIMENSIONLESS
};
pub type Velocity<N> = Qty<VELOCITY, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Standard acceleration of free fall (`[g]`), a unit of `Acceleration`: `n` of them is `n · 980665e-5` coherent units.
    pub type StandardAccelerationOfFreeFall = UcumUnit<
        ACCELERATION,
        {
            UcumUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "[g]",
                name: "standard acceleration of free fall",
            }
        },
    >;

    /// Planck constant (`[h]`), a unit of `Action`: `n` of them is `n · 662607015e-39` coherent units.
    pub type PlanckConstant = UcumUnit<
        ACTION,
        {
            UcumUnitConsts {
                size: Exact::dec(662607015, -39),
                zero: Exact::ZERO,
                symbol: "[h]",
                name: "Planck constant",
            }
        },
    >;

    /// Elementary charge (`[e]`), a unit of `ElectricCharge`: `n` of them is `n · 1602176634e-28` coherent units.
    pub type ElementaryCharge = UcumUnit<
        ELECTRIC_CHARGE,
        {
            UcumUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "[e]",
                name: "elementary charge",
            }
        },
    >;

    /// Permittivity of vacuum (`[eps_0]`), a unit of `ElectricPermittivity`: `n` of them is `n · 8854187817e-24` coherent units.
    pub type PermittivityOfVacuum = UcumUnit<
        ELECTRIC_PERMITTIVITY,
        {
            UcumUnitConsts {
                size: Exact::dec(8854187817, -24),
                zero: Exact::ZERO,
                symbol: "[eps_0]",
                name: "permittivity of vacuum",
            }
        },
    >;

    /// Gram-force (`gf`), a unit of `Force`: `n` of them is `n · 980665e-5` coherent units.
    pub type GramForce = UcumUnit<
        FORCE,
        {
            UcumUnitConsts {
                size: Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "gf",
                name: "gram-force",
            }
        },
    >;

    /// Pound force (`[lbf_av]`), a unit of `Force`: `n` of them is `n · 44482216152605e-10` coherent units.
    pub type PoundForce = UcumUnit<
        FORCE,
        {
            UcumUnitConsts {
                size: Exact::dec(44482216152605, -10),
                zero: Exact::ZERO,
                symbol: "[lbf_av]",
                name: "pound force",
            }
        },
    >;

    /// Light-year (`[ly]`), a unit of `Length`: `n` of them is `n · 94607304725808e2` coherent units.
    pub type LightYear = UcumUnit<
        LENGTH,
        {
            UcumUnitConsts {
                size: Exact::int(9460730472580800),
                zero: Exact::ZERO,
                symbol: "[ly]",
                name: "light-year",
            }
        },
    >;

    /// Permeability of vacuum (`[mu_0]`), a unit of `MagneticPermeability`: `n` of them is `n · 4e-4 * core::f64::consts::PI` coherent units.
    pub type PermeabilityOfVacuum = UcumUnit<
        MAGNETIC_PERMEABILITY,
        {
            UcumUnitConsts {
                size: Exact::dec(4, -4) * Exact::PI,
                zero: Exact::ZERO,
                symbol: "[mu_0]",
                name: "permeability of vacuum",
            }
        },
    >;

    /// Electron mass (`[m_e]`), a unit of `Mass`: `n` of them is `n · 91093837139e-38` coherent units.
    pub type ElectronMass = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(91093837139, -38),
                zero: Exact::ZERO,
                symbol: "[m_e]",
                name: "electron mass",
            }
        },
    >;

    /// Proton mass (`[m_p]`), a unit of `Mass`: `n` of them is `n · 167262192595e-35` coherent units.
    pub type ProtonMass = UcumUnit<
        MASS,
        {
            UcumUnitConsts {
                size: Exact::dec(167262192595, -35),
                zero: Exact::ZERO,
                symbol: "[m_p]",
                name: "proton mass",
            }
        },
    >;

    /// Standard atmosphere (`atm`), a unit of `Pressure`: `n` of them is `n · 101325e3` coherent units.
    pub type StandardAtmosphere = UcumUnit<
        PRESSURE,
        {
            UcumUnitConsts {
                size: Exact::int(101325000),
                zero: Exact::ZERO,
                symbol: "atm",
                name: "standard atmosphere",
            }
        },
    >;

    /// Boltzmann constant (`[k]`), a unit of `UnclassifiedM2GPerS2K`: `n` of them is `n · 1380649e-26` coherent units.
    pub type BoltzmannConstant = UcumUnit<
        UNCLASSIFIED_M2_G_PER_S2_K,
        {
            UcumUnitConsts {
                size: Exact::dec(1380649, -26),
                zero: Exact::ZERO,
                symbol: "[k]",
                name: "Boltzmann constant",
            }
        },
    >;

    /// Newtonian constant of gravitation (`[G]`), a unit of `UnclassifiedM3PerS2G`: `n` of them is `n · 66743e-18` coherent units.
    pub type NewtonianConstantOfGravitation = UcumUnit<
        UNCLASSIFIED_M3_PER_S2_G,
        {
            UcumUnitConsts {
                size: Exact::dec(66743, -18),
                zero: Exact::ZERO,
                symbol: "[G]",
                name: "Newtonian constant of gravitation",
            }
        },
    >;

    /// Velocity of light (`[c]`), a unit of `Velocity`: `n` of them is `n · 299792458` coherent units.
    pub type VelocityOfLight = UcumUnit<
        VELOCITY,
        {
            UcumUnitConsts {
                size: Exact::int(299792458),
                zero: Exact::ZERO,
                symbol: "[c]",
                name: "velocity of light",
            }
        },
    >;
}
