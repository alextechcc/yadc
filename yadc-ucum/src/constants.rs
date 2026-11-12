use crate::{Dim, Exact, Qty};
use consts::*;
use yadc::Dimension;

pub type Acceleration<N> = Qty<
    {
        Dim {
            m: 1,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Action<N> = Qty<
    {
        Dim {
            m: 2,
            s: -1,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricCharge<N> = Qty<
    {
        Dim {
            C: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricPermittivity<N> = Qty<
    {
        Dim {
            m: -3,
            s: 2,
            g: -1,
            C: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Force<N> = Qty<
    {
        Dim {
            m: 1,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Length<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticPermeability<N> = Qty<
    {
        Dim {
            m: 1,
            g: 1,
            C: -2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Mass<N> = Qty<
    {
        Dim {
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Pressure<N> = Qty<
    {
        Dim {
            m: -1,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type UnclassifiedM2GPerS2K<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            g: 1,
            K: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type UnclassifiedM3PerS2G<N> = Qty<
    {
        Dim {
            m: 3,
            s: -2,
            g: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Velocity<N> = Qty<
    {
        Dim {
            m: 1,
            s: -1,
            ..Dim::DIMENSIONLESS
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
    use crate::base::{KELVIN, SECOND};

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// The gram, UCUM's base unit of mass.
    pub use crate::base::GRAM;

    /// The coulomb, UCUM's base unit of electric charge.
    pub use crate::base::COULOMB;

    /// One `StandardAccelerationOfFreeFall` (`[g]`), as an `Acceleration`.
    pub const STANDARD_ACCELERATION_OF_FREE_FALL: Acceleration<Exact> =
        (METRE / (SECOND * SECOND)) * Exact::dec(980665, -5);

    /// One `PlanckConstant` (`[h]`), as an `Action`.
    pub const PLANCK_CONSTANT: Action<Exact> =
        (METRE * METRE * GRAM / SECOND) * Exact::dec(662607015, -39);

    /// One `ElementaryCharge` (`[e]`), as an `ElectricCharge`.
    pub const ELEMENTARY_CHARGE: ElectricCharge<Exact> = COULOMB * Exact::dec(1602176634, -28);

    /// One `PermittivityOfVacuum` (`[eps_0]`), as an `ElectricPermittivity`.
    pub const PERMITTIVITY_OF_VACUUM: ElectricPermittivity<Exact> =
        (SECOND * SECOND * COULOMB * COULOMB / (METRE * METRE * METRE * GRAM))
            * Exact::dec(8854187817, -24);

    /// One `GramForce` (`gf`), as a `Force`.
    pub const GRAM_FORCE: Force<Exact> =
        (METRE * GRAM / (SECOND * SECOND)) * Exact::dec(980665, -5);

    /// One `PoundForce` (`[lbf_av]`), as a `Force`.
    pub const POUND_FORCE: Force<Exact> =
        (METRE * GRAM / (SECOND * SECOND)) * Exact::dec(44482216152605, -10);

    /// One `LightYear` (`[ly]`), as a `Length`.
    pub const LIGHT_YEAR: Length<Exact> = METRE * Exact::int(9460730472580800);

    /// One `PermeabilityOfVacuum` (`[mu_0]`), as a `MagneticPermeability`.
    pub const PERMEABILITY_OF_VACUUM: MagneticPermeability<Exact> =
        (METRE * GRAM / (COULOMB * COULOMB)) * (Exact::dec(4, -4) * Exact::PI);

    /// One `ElectronMass` (`[m_e]`), as a `Mass`.
    pub const ELECTRON_MASS: Mass<Exact> = GRAM * Exact::dec(91093837139, -38);

    /// One `ProtonMass` (`[m_p]`), as a `Mass`.
    pub const PROTON_MASS: Mass<Exact> = GRAM * Exact::dec(167262192595, -35);

    /// One `StandardAtmosphere` (`atm`), as a `Pressure`.
    pub const STANDARD_ATMOSPHERE: Pressure<Exact> =
        (GRAM / (METRE * SECOND * SECOND)) * Exact::int(101325000);

    /// One `BoltzmannConstant` (`[k]`), as an `UnclassifiedM2GPerS2K`.
    pub const BOLTZMANN_CONSTANT: UnclassifiedM2GPerS2K<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND * KELVIN)) * Exact::dec(1380649, -26);

    /// One `NewtonianConstantOfGravitation` (`[G]`), as an `UnclassifiedM3PerS2G`.
    pub const NEWTONIAN_CONSTANT_OF_GRAVITATION: UnclassifiedM3PerS2G<Exact> =
        (METRE * METRE * METRE / (SECOND * SECOND * GRAM)) * Exact::dec(66743, -18);

    /// One `VelocityOfLight` (`[c]`), as a `Velocity`.
    pub const VELOCITY_OF_LIGHT: Velocity<Exact> = (METRE / SECOND) * Exact::int(299792458);
}

/// The units of `Acceleration`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Acceleration<N>, number = N)]
#[unit(for<N> where Acceleration<N>: [const] yadc::Unit<Acceleration<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AccelerationUnit {
    #[default]
    #[unit("[g]", "standard acceleration of free fall", by = Acceleration::<N>::from(STANDARD_ACCELERATION_OF_FREE_FALL))]
    StandardAccelerationOfFreeFall,
}

/// The units of `Action`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Action<N>, number = N)]
#[unit(for<N> where Action<N>: [const] yadc::Unit<Action<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ActionUnit {
    #[default]
    #[unit("[h]", "Planck constant", by = Action::<N>::from(PLANCK_CONSTANT))]
    PlanckConstant,
}

/// The units of `ElectricCharge`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N)]
#[unit(for<N> where ElectricCharge<N>: [const] yadc::Unit<ElectricCharge<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit("[e]", "elementary charge", by = ElectricCharge::<N>::from(ELEMENTARY_CHARGE))]
    ElementaryCharge,
}

/// The units of `ElectricPermittivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPermittivity<N>, number = N)]
#[unit(for<N> where ElectricPermittivity<N>: [const] yadc::Unit<ElectricPermittivity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricPermittivityUnit {
    #[default]
    #[unit("[eps_0]", "permittivity of vacuum", by = ElectricPermittivity::<N>::from(PERMITTIVITY_OF_VACUUM))]
    PermittivityOfVacuum,
}

/// The units of `Force`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N)]
#[unit(for<N> where Force<N>: [const] yadc::Unit<Force<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ForceUnit {
    #[default]
    #[unit("gf", "gram-force", by = Force::<N>::from(GRAM_FORCE))]
    GramForce,
    #[unit("[lbf_av]", "pound force", by = Force::<N>::from(POUND_FORCE))]
    PoundForce,
}

/// The units of `Length`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N> where Length<N>: [const] yadc::Unit<Length<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LengthUnit {
    #[default]
    #[unit("[ly]", "light-year", by = Length::<N>::from(LIGHT_YEAR))]
    LightYear,
}

/// The units of `MagneticPermeability`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticPermeability<N>, number = N)]
#[unit(for<N> where MagneticPermeability<N>: [const] yadc::Unit<MagneticPermeability<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticPermeabilityUnit {
    #[default]
    #[unit("[mu_0]", "permeability of vacuum", by = MagneticPermeability::<N>::from(PERMEABILITY_OF_VACUUM))]
    PermeabilityOfVacuum,
}

/// The units of `Mass`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N> where Mass<N>: [const] yadc::Unit<Mass<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassUnit {
    #[default]
    #[unit("[m_e]", "electron mass", by = Mass::<N>::from(ELECTRON_MASS))]
    ElectronMass,
    #[unit("[m_p]", "proton mass", by = Mass::<N>::from(PROTON_MASS))]
    ProtonMass,
}

/// The units of `Pressure`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Pressure<N>: [const] yadc::Unit<Pressure<N>, N> + [const] core::marker::Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("atm", "standard atmosphere", by = Pressure::<N>::from(STANDARD_ATMOSPHERE))]
    StandardAtmosphere,
}

/// The units of `UnclassifiedM2GPerS2K`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = UnclassifiedM2GPerS2K<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, UnclassifiedM2GPerS2K<N>: [const] yadc::Unit<UnclassifiedM2GPerS2K<N>, N> + [const] core::marker::Destruct)]
pub enum UnclassifiedM2GPerS2KUnit {
    #[default]
    #[unit("[k]", "Boltzmann constant", by = UnclassifiedM2GPerS2K::<N>::from(BOLTZMANN_CONSTANT))]
    BoltzmannConstant,
}

/// The units of `UnclassifiedM3PerS2G`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = UnclassifiedM3PerS2G<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, UnclassifiedM3PerS2G<N>: [const] yadc::Unit<UnclassifiedM3PerS2G<N>, N> + [const] core::marker::Destruct)]
pub enum UnclassifiedM3PerS2GUnit {
    #[default]
    #[unit("[G]", "Newtonian constant of gravitation", by = UnclassifiedM3PerS2G::<N>::from(NEWTONIAN_CONSTANT_OF_GRAVITATION))]
    NewtonianConstantOfGravitation,
}

/// The units of `Velocity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Velocity<N>: [const] yadc::Unit<Velocity<N>, N> + [const] core::marker::Destruct)]
pub enum VelocityUnit {
    #[default]
    #[unit("[c]", "velocity of light", by = Velocity::<N>::from(VELOCITY_OF_LIGHT))]
    VelocityOfLight,
}
