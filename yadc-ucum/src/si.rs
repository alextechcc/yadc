use crate::{Dim, Exact, Qty};
use consts::*;
use yadc::Dimension;

pub type AmountOfSubstance<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type DoseEquivalent<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricCapacitance<N> = Qty<
    {
        Dim {
            m: -2,
            s: 2,
            g: -1,
            C: 2,
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

pub type ElectricConductance<N> = Qty<
    {
        Dim {
            m: -2,
            s: 1,
            g: -1,
            C: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricCurrent<N> = Qty<
    {
        Dim {
            s: -1,
            C: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricPotential<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            g: 1,
            C: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type ElectricResistance<N> = Qty<
    {
        Dim {
            m: 2,
            s: -1,
            g: 1,
            C: -2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Energy<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
            g: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type EnergyDose<N> = Qty<
    {
        Dim {
            m: 2,
            s: -2,
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

pub type Frequency<N> = Qty<
    {
        Dim {
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Illuminance<N> = Qty<
    {
        Dim {
            m: -2,
            rad: 2,
            cd: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Inductance<N> = Qty<
    {
        Dim {
            m: 2,
            g: 1,
            C: -2,
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

pub type LuminousFlux<N> = Qty<
    {
        Dim {
            rad: 2,
            cd: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type LuminousIntensity<N> = Qty<
    {
        Dim {
            cd: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFlux<N> = Qty<
    {
        Dim {
            m: 2,
            s: -1,
            g: 1,
            C: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type MagneticFluxDensity<N> = Qty<
    {
        Dim {
            s: -1,
            g: 1,
            C: -1,
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

pub type PlaneAngle<N> = Qty<
    {
        Dim {
            rad: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Power<N> = Qty<
    {
        Dim {
            m: 2,
            s: -3,
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

pub type Radioactivity<N> = Qty<
    {
        Dim {
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type SolidAngle<N> = Qty<
    {
        Dim {
            rad: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Time<N> = Qty<
    {
        Dim {
            s: 1,
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

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// The second, UCUM's base unit of time.
    pub use crate::base::SECOND;

    /// The gram, UCUM's base unit of mass.
    pub use crate::base::GRAM;

    /// The radian, UCUM's base unit of plane angle.
    pub use crate::base::RADIAN;

    /// The coulomb, UCUM's base unit of electric charge.
    pub use crate::base::COULOMB;

    /// The candela, UCUM's base unit of luminous intensity.
    pub use crate::base::CANDELA;

    /// One `Mole` (`mol`), as an `AmountOfSubstance`.
    pub const MOLE: AmountOfSubstance<Exact> = ONE * Exact::int(602214076000000000000000);

    /// One `Sievert` (`Sv`), as a `DoseEquivalent`.
    pub const SIEVERT: DoseEquivalent<Exact> = METRE * METRE / (SECOND * SECOND);

    /// One `Farad` (`F`), as an `ElectricCapacitance`.
    pub const FARAD: ElectricCapacitance<Exact> =
        (SECOND * SECOND * COULOMB * COULOMB / (METRE * METRE * GRAM)) * Exact::dec(1, -3);

    /// One `Siemens` (`S`), as an `ElectricConductance`.
    pub const SIEMENS: ElectricConductance<Exact> =
        (SECOND * COULOMB * COULOMB / (METRE * METRE * GRAM)) * Exact::dec(1, -3);

    /// One `Ampere` (`A`), as an `ElectricCurrent`.
    pub const AMPERE: ElectricCurrent<Exact> = COULOMB / SECOND;

    /// One `Volt` (`V`), as an `ElectricPotential`.
    pub const VOLT: ElectricPotential<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND * COULOMB)) * Exact::int(1000);

    /// One `Ohm` (`Ohm`), as an `ElectricResistance`.
    pub const OHM: ElectricResistance<Exact> =
        (METRE * METRE * GRAM / (SECOND * COULOMB * COULOMB)) * Exact::int(1000);

    /// One `Joule` (`J`), as an `Energy`.
    pub const JOULE: Energy<Exact> = (METRE * METRE * GRAM / (SECOND * SECOND)) * Exact::int(1000);

    /// One `Gray` (`Gy`), as an `EnergyDose`.
    pub const GRAY: EnergyDose<Exact> = METRE * METRE / (SECOND * SECOND);

    /// One `Newton` (`N`), as a `Force`.
    pub const NEWTON: Force<Exact> = (METRE * GRAM / (SECOND * SECOND)) * Exact::int(1000);

    /// One `Hertz` (`Hz`), as a `Frequency`.
    pub const HERTZ: Frequency<Exact> = ONE / SECOND;

    /// One `Lux` (`lx`), as an `Illuminance`.
    pub const LUX: Illuminance<Exact> = RADIAN * RADIAN * CANDELA / (METRE * METRE);

    /// One `Henry` (`H`), as an `Inductance`.
    pub const HENRY: Inductance<Exact> =
        (METRE * METRE * GRAM / (COULOMB * COULOMB)) * Exact::int(1000);

    /// One `Meter` (`m`), as a `Length`.
    pub const METER: Length<Exact> = METRE;

    /// One `Lumen` (`lm`), as a `LuminousFlux`.
    pub const LUMEN: LuminousFlux<Exact> = RADIAN * RADIAN * CANDELA;

    /// One `Weber` (`Wb`), as a `MagneticFlux`.
    pub const WEBER: MagneticFlux<Exact> =
        (METRE * METRE * GRAM / (SECOND * COULOMB)) * Exact::int(1000);

    /// One `Tesla` (`T`), as a `MagneticFluxDensity`.
    pub const TESLA: MagneticFluxDensity<Exact> = (GRAM / (SECOND * COULOMB)) * Exact::int(1000);

    /// One `Watt` (`W`), as a `Power`.
    pub const WATT: Power<Exact> =
        (METRE * METRE * GRAM / (SECOND * SECOND * SECOND)) * Exact::int(1000);

    /// One `Pascal` (`Pa`), as a `Pressure`.
    pub const PASCAL: Pressure<Exact> = (GRAM / (METRE * SECOND * SECOND)) * Exact::int(1000);

    /// One `Becquerel` (`Bq`), as a `Radioactivity`.
    pub const BECQUEREL: Radioactivity<Exact> = ONE / SECOND;

    /// One `Steradian` (`sr`), as a `SolidAngle`.
    pub const STERADIAN: SolidAngle<Exact> = RADIAN * RADIAN;
}

/// The units of `AmountOfSubstance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfSubstance<N>, number = N)]
#[unit(for<N> where AmountOfSubstance<N>: [const] yadc::Unit<AmountOfSubstance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AmountOfSubstanceUnit {
    #[default]
    #[unit("mol", "mole", by = AmountOfSubstance::<N>::from(MOLE))]
    Mole,
}

/// The units of `DoseEquivalent`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DoseEquivalent<N>, number = N)]
#[unit(for<N> where DoseEquivalent<N>: [const] yadc::Unit<DoseEquivalent<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum DoseEquivalentUnit {
    #[default]
    #[unit("Sv", "sievert", by = DoseEquivalent::<N>::from(SIEVERT))]
    Sievert,
}

/// The units of `ElectricCapacitance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCapacitance<N>, number = N)]
#[unit(for<N> where ElectricCapacitance<N>: [const] yadc::Unit<ElectricCapacitance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricCapacitanceUnit {
    #[default]
    #[unit("F", "farad", by = ElectricCapacitance::<N>::from(FARAD))]
    Farad,
}

/// The units of `ElectricCharge`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCharge<N>, number = N)]
#[unit(for<N> where ElectricCharge<N>: [const] yadc::Unit<ElectricCharge<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricChargeUnit {
    #[default]
    #[unit("C", "coulomb", by = ElectricCharge::<N>::from(COULOMB))]
    Coulomb,
}

/// The units of `ElectricConductance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricConductance<N>, number = N)]
#[unit(for<N> where ElectricConductance<N>: [const] yadc::Unit<ElectricConductance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricConductanceUnit {
    #[default]
    #[unit("S", "siemens", by = ElectricConductance::<N>::from(SIEMENS))]
    Siemens,
}

/// The units of `ElectricCurrent`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricCurrent<N>, number = N)]
#[unit(for<N> where ElectricCurrent<N>: [const] yadc::Unit<ElectricCurrent<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricCurrentUnit {
    #[default]
    #[unit("A", "ampère", by = ElectricCurrent::<N>::from(AMPERE))]
    Ampere,
}

/// The units of `ElectricPotential`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricPotential<N>, number = N)]
#[unit(for<N> where ElectricPotential<N>: [const] yadc::Unit<ElectricPotential<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricPotentialUnit {
    #[default]
    #[unit("V", "volt", by = ElectricPotential::<N>::from(VOLT))]
    Volt,
}

/// The units of `ElectricResistance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = ElectricResistance<N>, number = N)]
#[unit(for<N> where ElectricResistance<N>: [const] yadc::Unit<ElectricResistance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ElectricResistanceUnit {
    #[default]
    #[unit("Ohm", "ohm", by = ElectricResistance::<N>::from(OHM))]
    Ohm,
}

/// The units of `Energy`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Energy<N>, number = N)]
#[unit(for<N> where Energy<N>: [const] yadc::Unit<Energy<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum EnergyUnit {
    #[default]
    #[unit("J", "joule", by = Energy::<N>::from(JOULE))]
    Joule,
}

/// The units of `EnergyDose`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = EnergyDose<N>, number = N)]
#[unit(for<N> where EnergyDose<N>: [const] yadc::Unit<EnergyDose<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum EnergyDoseUnit {
    #[default]
    #[unit("Gy", "gray", by = EnergyDose::<N>::from(GRAY))]
    Gray,
}

/// The units of `Force`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Force<N>, number = N)]
#[unit(for<N> where Force<N>: [const] yadc::Unit<Force<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum ForceUnit {
    #[default]
    #[unit("N", "newton", by = Force::<N>::from(NEWTON))]
    Newton,
}

/// The units of `Frequency`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Frequency<N>, number = N)]
#[unit(for<N> where Frequency<N>: [const] yadc::Unit<Frequency<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum FrequencyUnit {
    #[default]
    #[unit("Hz", "hertz", by = Frequency::<N>::from(HERTZ))]
    Hertz,
}

/// The units of `Illuminance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Illuminance<N>, number = N)]
#[unit(for<N> where Illuminance<N>: [const] yadc::Unit<Illuminance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum IlluminanceUnit {
    #[default]
    #[unit("lx", "lux", by = Illuminance::<N>::from(LUX))]
    Lux,
}

/// The units of `Inductance`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Inductance<N>, number = N)]
#[unit(for<N> where Inductance<N>: [const] yadc::Unit<Inductance<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum InductanceUnit {
    #[default]
    #[unit("H", "henry", by = Inductance::<N>::from(HENRY))]
    Henry,
}

/// The units of `Length`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N> where Length<N>: [const] yadc::Unit<Length<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LengthUnit {
    #[default]
    #[unit("m", "meter", by = Length::<N>::from(METER))]
    Meter,
}

/// The units of `LuminousFlux`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousFlux<N>, number = N)]
#[unit(for<N> where LuminousFlux<N>: [const] yadc::Unit<LuminousFlux<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LuminousFluxUnit {
    #[default]
    #[unit("lm", "lumen", by = LuminousFlux::<N>::from(LUMEN))]
    Lumen,
}

/// The units of `LuminousIntensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = LuminousIntensity<N>, number = N)]
#[unit(for<N> where LuminousIntensity<N>: [const] yadc::Unit<LuminousIntensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum LuminousIntensityUnit {
    #[default]
    #[unit("cd", "candela", by = LuminousIntensity::<N>::from(CANDELA))]
    Candela,
}

/// The units of `MagneticFlux`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFlux<N>, number = N)]
#[unit(for<N> where MagneticFlux<N>: [const] yadc::Unit<MagneticFlux<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticFluxUnit {
    #[default]
    #[unit("Wb", "weber", by = MagneticFlux::<N>::from(WEBER))]
    Weber,
}

/// The units of `MagneticFluxDensity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = MagneticFluxDensity<N>, number = N)]
#[unit(for<N> where MagneticFluxDensity<N>: [const] yadc::Unit<MagneticFluxDensity<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MagneticFluxDensityUnit {
    #[default]
    #[unit("T", "tesla", by = MagneticFluxDensity::<N>::from(TESLA))]
    Tesla,
}

/// The units of `Mass`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Mass<N>, number = N)]
#[unit(for<N> where Mass<N>: [const] yadc::Unit<Mass<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum MassUnit {
    #[default]
    #[unit("g", "gram", by = Mass::<N>::from(GRAM))]
    Gram,
}

/// The units of `PlaneAngle`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = PlaneAngle<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, PlaneAngle<N>: [const] yadc::Unit<PlaneAngle<N>, N> + [const] core::marker::Destruct)]
pub enum PlaneAngleUnit {
    #[default]
    #[unit("rad", "radian", by = PlaneAngle::<N>::from(RADIAN))]
    Radian,
}

/// The units of `Power`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Power<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Power<N>: [const] yadc::Unit<Power<N>, N> + [const] core::marker::Destruct)]
pub enum PowerUnit {
    #[default]
    #[unit("W", "watt", by = Power::<N>::from(WATT))]
    Watt,
}

/// The units of `Pressure`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Pressure<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Pressure<N>: [const] yadc::Unit<Pressure<N>, N> + [const] core::marker::Destruct)]
pub enum PressureUnit {
    #[default]
    #[unit("Pa", "pascal", by = Pressure::<N>::from(PASCAL))]
    Pascal,
}

/// The units of `Radioactivity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Radioactivity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Radioactivity<N>: [const] yadc::Unit<Radioactivity<N>, N> + [const] core::marker::Destruct)]
pub enum RadioactivityUnit {
    #[default]
    #[unit("Bq", "becquerel", by = Radioactivity::<N>::from(BECQUEREL))]
    Becquerel,
}

/// The units of `SolidAngle`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SolidAngle<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SolidAngle<N>: [const] yadc::Unit<SolidAngle<N>, N> + [const] core::marker::Destruct)]
pub enum SolidAngleUnit {
    #[default]
    #[unit("sr", "steradian", by = SolidAngle::<N>::from(STERADIAN))]
    Steradian,
}

/// The units of `Time`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Time<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Time<N>: [const] yadc::Unit<Time<N>, N> + [const] core::marker::Destruct)]
pub enum TimeUnit {
    #[default]
    #[unit("s", "second", by = Time::<N>::from(SECOND))]
    Second,
}
