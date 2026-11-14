use crate::{ISQ, SIQuantity};
use yadc::Dimension;

/// The dimension of `AmountOfSubstance`.
pub const AMOUNT_OF_SUBSTANCE: ISQ = ISQ {
    mol: 1,
    ..ISQ::DIMENSIONLESS
};
pub type AmountOfSubstance<N> = SIQuantity<AMOUNT_OF_SUBSTANCE, N>;

/// The dimension of `MolarConcentration`.
pub const MOLAR_CONCENTRATION: ISQ = ISQ {
    m: -3,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};
pub type MolarConcentration<N> = SIQuantity<MOLAR_CONCENTRATION, N>;

/// The dimension of `MolarEnergy`.
pub const MOLAR_ENERGY: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MolarEnergy<N> = SIQuantity<MOLAR_ENERGY, N>;

/// The dimension of `MolarFlux`.
pub const MOLAR_FLUX: ISQ = ISQ {
    m: -2,
    s: -1,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};
pub type MolarFlux<N> = SIQuantity<MOLAR_FLUX, N>;

/// The dimension of `MolarHeatCapacity`.
pub const MOLAR_HEAT_CAPACITY: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    K: -1,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MolarHeatCapacity<N> = SIQuantity<MOLAR_HEAT_CAPACITY, N>;

/// The dimension of `MolarMass`.
pub const MOLAR_MASS: ISQ = ISQ {
    kg: 1,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MolarMass<N> = SIQuantity<MOLAR_MASS, N>;

/// The dimension of `MolarRadioactivity`.
pub const MOLAR_RADIOACTIVITY: ISQ = ISQ {
    s: -1,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MolarRadioactivity<N> = SIQuantity<MOLAR_RADIOACTIVITY, N>;

/// The dimension of `MolarVolume`.
pub const MOLAR_VOLUME: ISQ = ISQ {
    m: 3,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};
pub type MolarVolume<N> = SIQuantity<MOLAR_VOLUME, N>;

/// The dimension of `Molality`.
pub const MOLALITY: ISQ = ISQ {
    kg: -1,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};
pub type Molality<N> = SIQuantity<MOLALITY, N>;

/// The dimension of `MassConcentration`.
pub const MASS_CONCENTRATION: ISQ = ISQ {
    m: -3,
    kg: 1,
    ..ISQ::DIMENSIONLESS
};
pub type MassConcentration<N> = SIQuantity<MASS_CONCENTRATION, N>;

/// The dimension of `CatalyticActivity`.
pub const CATALYTIC_ACTIVITY: ISQ = ISQ {
    s: -1,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};
pub type CatalyticActivity<N> = SIQuantity<CATALYTIC_ACTIVITY, N>;

/// The dimension of `CatalyticActivityConcentration`.
pub const CATALYTIC_ACTIVITY_CONCENTRATION: ISQ = ISQ {
    m: -3,
    s: -1,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};
pub type CatalyticActivityConcentration<N> = SIQuantity<CATALYTIC_ACTIVITY_CONCENTRATION, N>;

/// The dimension of `Radioactivity`.
pub const RADIOACTIVITY: ISQ = ISQ {
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type Radioactivity<N> = SIQuantity<RADIOACTIVITY, N>;

/// The dimension of `SpecificRadioactivity`.
pub const SPECIFIC_RADIOACTIVITY: ISQ = ISQ {
    kg: -1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};
pub type SpecificRadioactivity<N> = SIQuantity<SPECIFIC_RADIOACTIVITY, N>;

/// The dimension of `InverseAmountOfSubstance`.
pub const INVERSE_AMOUNT_OF_SUBSTANCE: ISQ = ISQ {
    mol: -1,
    ..ISQ::DIMENSIONLESS
};
pub type InverseAmountOfSubstance<N> = SIQuantity<INVERSE_AMOUNT_OF_SUBSTANCE, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Moles (`mol`), a unit of `AmountOfSubstance`: `n` of them is `n · 1.0` coherent units.
    pub type Mole = SIUnit<
        AMOUNT_OF_SUBSTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol",
                name: "moles",
            }
        },
    >;

    /// Particles (`particle`), a unit of `AmountOfSubstance`: `n` of them is `n · 1.0 / 6.02214076e23` coherent units.
    pub type Particle = SIUnit<
        AMOUNT_OF_SUBSTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "particle",
                name: "particles",
            }
        },
    >;

    /// Moles per cubic metre (`mol/m³`), a unit of `MolarConcentration`: `n` of them is `n · 1.0` coherent units.
    pub type MolePerCubicMetre = SIUnit<
        MOLAR_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol/m³",
                name: "moles per cubic metre",
            }
        },
    >;

    /// Moles per litre (`mol/L`), a unit of `MolarConcentration`: `n` of them is `n · 1000.0` coherent units.
    pub type MolePerLitre = SIUnit<
        MOLAR_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "mol/L",
                name: "moles per litre",
            }
        },
    >;

    /// Particles per cubic metre (`particle/m³`), a unit of `MolarConcentration`: `n` of them is `n · 1.0 / 6.02214076e23` coherent units.
    pub type ParticlePerCubicMetre = SIUnit<
        MOLAR_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "particle/m³",
                name: "particles per cubic metre",
            }
        },
    >;

    /// Kilograms square metre per second squared mole (`kg · m²/(s² · mol)`), a unit of `MolarEnergy`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramSquareMetrePerSecondSquaredMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/(s² · mol)",
                name: "kilograms square metre per second squared mole",
            }
        },
    >;

    /// Joules per mole (`J/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/mol",
                name: "joules per mole",
            }
        },
    >;

    /// Watt hours per mole (`W · h/mol`), a unit of `MolarEnergy`: `n` of them is `n · 3600.0` coherent units.
    pub type WattHourPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "W · h/mol",
                name: "watt hours per mole",
            }
        },
    >;

    /// British thermal units (IT) per mole (`Btu (IT)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1055.05585262` coherent units.
    pub type BtuItPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/mol",
                name: "British thermal units (IT) per mole",
            }
        },
    >;

    /// British thermal units per mole (`Btu/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4184.0 * 0.45359237 / 1.8` coherent units.
    pub type BtuPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1),
                zero: Exact::ZERO,
                symbol: "Btu/mol",
                name: "British thermal units per mole",
            }
        },
    >;

    /// Calories (IT) per mole (`cal (IT)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4.1868` coherent units.
    pub type CalorieItPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -4),
                zero: Exact::ZERO,
                symbol: "cal (IT)/mol",
                name: "calories (IT) per mole",
            }
        },
    >;

    /// Calories per mole (`cal/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4.184` coherent units.
    pub type CaloriePerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal/mol",
                name: "calories per mole",
            }
        },
    >;

    /// Calories (IT) per mole (`Cal (IT)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4186.8` coherent units.
    pub type CalorieItNutritionPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Cal (IT)/mol",
                name: "Calories (IT) per mole",
            }
        },
    >;

    /// Calories per mole (`Cal/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4184.0` coherent units.
    pub type CalorieNutritionPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Cal/mol",
                name: "Calories per mole",
            }
        },
    >;

    /// Electronvolts per mole (`eV/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub type ElectronvoltPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "eV/mol",
                name: "electronvolts per mole",
            }
        },
    >;

    /// Ergs per mole (`erg/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1e-07` coherent units.
    pub type ErgPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg/mol",
                name: "ergs per mole",
            }
        },
    >;

    /// Foot poundals per mole (`ft · pdl/mol`), a unit of `MolarEnergy`: `n` of them is `n · 0.3048 * 0.45359237 * 0.3048` coherent units.
    pub type FootPoundalPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft · pdl/mol",
                name: "foot poundals per mole",
            }
        },
    >;

    /// Foot pounds-force per mole (`ft · lbf/mol`), a unit of `MolarEnergy`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub type FootPoundForcePerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "ft · lbf/mol",
                name: "foot pounds-force per mole",
            }
        },
    >;

    /// Quads per mole (`10¹⁵ Btu (IT)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1.05505585262e18` coherent units.
    pub type QuadPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1055055852620000000),
                zero: Exact::ZERO,
                symbol: "10¹⁵ Btu (IT)/mol",
                name: "quads per mole",
            }
        },
    >;

    /// Therms (EC) per mole (`thm (EC)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 105506000.0` coherent units.
    pub type ThermEcPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(105506000),
                zero: Exact::ZERO,
                symbol: "thm (EC)/mol",
                name: "therms (EC) per mole",
            }
        },
    >;

    /// Therms per mole (`thm/mol`), a unit of `MolarEnergy`: `n` of them is `n · 105480400.0` coherent units.
    pub type ThermUsPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(105480400),
                zero: Exact::ZERO,
                symbol: "thm/mol",
                name: "therms per mole",
            }
        },
    >;

    /// Tons of TNT per mole (`t of TNT/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4184000000.0` coherent units.
    pub type TonTntPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184000000),
                zero: Exact::ZERO,
                symbol: "t of TNT/mol",
                name: "tons of TNT per mole",
            }
        },
    >;

    /// Watt seconds per mole (`W · s/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1.0` coherent units.
    pub type WattSecondPerMole = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W · s/mol",
                name: "watt seconds per mole",
            }
        },
    >;

    /// Joules per particle (`J/particle`), a unit of `MolarEnergy`: `n` of them is `n · 6.02214076e+23` coherent units.
    pub type JoulePerParticle = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "J/particle",
                name: "joules per particle",
            }
        },
    >;

    /// Electronvolts per particle (`eV/particle`), a unit of `MolarEnergy`: `n` of them is `n · 1.602176634e-19 * 6.02214076e23` coherent units.
    pub type ElectronvoltPerParticle = SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "eV/particle",
                name: "electronvolts per particle",
            }
        },
    >;

    /// Moles per square metre second (`mol/(m² · s)`), a unit of `MolarFlux`: `n` of them is `n · 1.0` coherent units.
    pub type MolePerSquareMetreSecond = SIUnit<
        MOLAR_FLUX,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol/(m² · s)",
                name: "moles per square metre second",
            }
        },
    >;

    /// Joules per kelvin mole (`J/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub type JoulePerKelvinMole = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(K · mol)",
                name: "joules per kelvin mole",
            }
        },
    >;

    /// British thermal units (IT) per kelvin mole (`Btu (IT)/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 1055.05585262` coherent units.
    pub type BtuItPerKelvinMole = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(K · mol)",
                name: "British thermal units (IT) per kelvin mole",
            }
        },
    >;

    /// British thermal units per kelvin mole (`Btu/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4184.0 * 0.45359237 / 1.8` coherent units.
    pub type BtuPerKelvinMole = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1),
                zero: Exact::ZERO,
                symbol: "Btu/(K · mol)",
                name: "British thermal units per kelvin mole",
            }
        },
    >;

    /// Calories (IT) per kelvin mole (`cal (IT)/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4.1868` coherent units.
    pub type CalorieItPerKelvinMole = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -4),
                zero: Exact::ZERO,
                symbol: "cal (IT)/(K · mol)",
                name: "calories (IT) per kelvin mole",
            }
        },
    >;

    /// Calories per kelvin mole (`cal/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4.184` coherent units.
    pub type CaloriePerKelvinMole = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal/(K · mol)",
                name: "calories per kelvin mole",
            }
        },
    >;

    /// Calories (IT) per kelvin mole (`Cal (IT)/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4186.8` coherent units.
    pub type CalorieItNutritionPerKelvinMole = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Cal (IT)/(K · mol)",
                name: "Calories (IT) per kelvin mole",
            }
        },
    >;

    /// Calories per kelvin mole (`Cal/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4184.0` coherent units.
    pub type CalorieNutritionPerKelvinMole = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Cal/(K · mol)",
                name: "Calories per kelvin mole",
            }
        },
    >;

    /// Joules per kelvin particle (`J/(K · particle)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 6.02214076e+23` coherent units.
    pub type JoulePerKelvinParticle = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "J/(K · particle)",
                name: "joules per kelvin particle",
            }
        },
    >;

    /// Electronvolts per kelvin particle (`eV/(K · particle)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 1.602176634e-19 * 6.02214076e23` coherent units.
    pub type ElectronvoltPerKelvinParticle = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "eV/(K · particle)",
                name: "electronvolts per kelvin particle",
            }
        },
    >;

    /// Molar gas constants (`R`), a unit of `MolarHeatCapacity`: `n` of them is `n · 8.314462618` coherent units.
    pub type MolarGasConstant = SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(8314462618, -9),
                zero: Exact::ZERO,
                symbol: "R",
                name: "molar gas constants",
            }
        },
    >;

    /// Kilograms per mole (`kg/mol`), a unit of `MolarMass`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerMole = SIUnit<
        MOLAR_MASS,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/mol",
                name: "kilograms per mole",
            }
        },
    >;

    /// Grams per mole (`g/mol`), a unit of `MolarMass`: `n` of them is `n · 0.001` coherent units.
    pub type GramPerMole = SIUnit<
        MOLAR_MASS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/mol",
                name: "grams per mole",
            }
        },
    >;

    /// Becquerels per mole (`Bq/mol`), a unit of `MolarRadioactivity`: `n` of them is `n · 1.0` coherent units.
    pub type BecquerelPerMole = SIUnit<
        MOLAR_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq/mol",
                name: "becquerels per mole",
            }
        },
    >;

    /// Curies per mole (`Ci/mol`), a unit of `MolarRadioactivity`: `n` of them is `n · 37000000000.0` coherent units.
    pub type CuriePerMole = SIUnit<
        MOLAR_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci/mol",
                name: "curies per mole",
            }
        },
    >;

    /// Disintegrations per minute per mole (`dpm/mol`), a unit of `MolarRadioactivity`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub type DisintegrationsPerMinutePerMole = SIUnit<
        MOLAR_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "dpm/mol",
                name: "disintegrations per minute per mole",
            }
        },
    >;

    /// Cubic metres per mole (`m³/mol`), a unit of `MolarVolume`: `n` of them is `n · 1.0` coherent units.
    pub type CubicMetrePerMole = SIUnit<
        MOLAR_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³/mol",
                name: "cubic metres per mole",
            }
        },
    >;

    /// Cubic metres per particle (`m³/particle`), a unit of `MolarVolume`: `n` of them is `n · 6.02214076e+23` coherent units.
    pub type CubicMetrePerParticle = SIUnit<
        MOLAR_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "m³/particle",
                name: "cubic metres per particle",
            }
        },
    >;

    /// Moles per kilogram (`mol/kg`), a unit of `Molality`: `n` of them is `n · 1.0` coherent units.
    pub type MolePerKilogram = SIUnit<
        MOLALITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol/kg",
                name: "moles per kilogram",
            }
        },
    >;

    /// Kilograms per cubic metre (`kg/m³`), a unit of `MassConcentration`: `n` of them is `n · 1.0` coherent units.
    pub type KilogramPerCubicMetre = SIUnit<
        MASS_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/m³",
                name: "kilograms per cubic metre",
            }
        },
    >;

    /// Grams per cubic metre (`g/m³`), a unit of `MassConcentration`: `n` of them is `n · 0.001` coherent units.
    pub type GramPerCubicMetre = SIUnit<
        MASS_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/m³",
                name: "grams per cubic metre",
            }
        },
    >;

    /// Katals (`kat`), a unit of `CatalyticActivity`: `n` of them is `n · 1.0` coherent units.
    pub type Katal = SIUnit<
        CATALYTIC_ACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kat",
                name: "katals",
            }
        },
    >;

    /// Particles per second (`particle/s`), a unit of `CatalyticActivity`: `n` of them is `n · 1.0 / 6.02214076e23` coherent units.
    pub type ParticlePerSecond = SIUnit<
        CATALYTIC_ACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "particle/s",
                name: "particles per second",
            }
        },
    >;

    /// Moles per second (`mol/s`), a unit of `CatalyticActivity`: `n` of them is `n · 1.0` coherent units.
    pub type MolePerSecond = SIUnit<
        CATALYTIC_ACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol/s",
                name: "moles per second",
            }
        },
    >;

    /// Katals per cubic metre (`kat/m³`), a unit of `CatalyticActivityConcentration`: `n` of them is `n · 1.0` coherent units.
    pub type KatalPerCubicMetre = SIUnit<
        CATALYTIC_ACTIVITY_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kat/m³",
                name: "katals per cubic metre",
            }
        },
    >;

    /// Katals per litre (`kat/L`), a unit of `CatalyticActivityConcentration`: `n` of them is `n · 1000.0` coherent units.
    pub type KatalPerLitre = SIUnit<
        CATALYTIC_ACTIVITY_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "kat/L",
                name: "katals per litre",
            }
        },
    >;

    /// Particles per second cubic metre (`particle · s⁻¹ · m⁻³`), a unit of `CatalyticActivityConcentration`: `n` of them is `n · 1.0 / 6.02214076e23` coherent units.
    pub type ParticlePerSecondCubicMetre = SIUnit<
        CATALYTIC_ACTIVITY_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "particle · s⁻¹ · m⁻³",
                name: "particles per second cubic metre",
            }
        },
    >;

    /// Becquerels (`Bq`), a unit of `Radioactivity`: `n` of them is `n · 1.0` coherent units.
    pub type Becquerel = SIUnit<
        RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq",
                name: "becquerels",
            }
        },
    >;

    /// Curies (`Ci`), a unit of `Radioactivity`: `n` of them is `n · 37000000000.0` coherent units.
    pub type Curie = SIUnit<
        RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci",
                name: "curies",
            }
        },
    >;

    /// Disintegrations per minute (`dpm`), a unit of `Radioactivity`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub type DisintegrationsPerMinute = SIUnit<
        RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "dpm",
                name: "disintegrations per minute",
            }
        },
    >;

    /// Becquerels per kilogram (`Bq/kg`), a unit of `SpecificRadioactivity`: `n` of them is `n · 1.0` coherent units.
    pub type BecquerelPerKilogram = SIUnit<
        SPECIFIC_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq/kg",
                name: "becquerels per kilogram",
            }
        },
    >;

    /// Curie per kilogram (`Ci/kg`), a unit of `SpecificRadioactivity`: `n` of them is `n · 37000000000.0` coherent units.
    pub type CuriePerKilogram = SIUnit<
        SPECIFIC_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci/kg",
                name: "curie per kilogram",
            }
        },
    >;

    /// Disintegrations per minute per kilogram (`dpm/kg`), a unit of `SpecificRadioactivity`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub type DisintegrationsPerMinutePerKilogram = SIUnit<
        SPECIFIC_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "dpm/kg",
                name: "disintegrations per minute per kilogram",
            }
        },
    >;

    /// Per mole (`mol⁻¹`), a unit of `InverseAmountOfSubstance`: `n` of them is `n · 1.0` coherent units.
    pub type PerMole = SIUnit<
        INVERSE_AMOUNT_OF_SUBSTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol⁻¹",
                name: "per mole",
            }
        },
    >;

    /// Avogadro constants (`N_A`), a unit of `InverseAmountOfSubstance`: `n` of them is `n · 6.02214076e23` coherent units.
    pub type AvogadroConstant = SIUnit<
        INVERSE_AMOUNT_OF_SUBSTANCE,
        {
            SIUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "N_A",
                name: "Avogadro constants",
            }
        },
    >;
}
