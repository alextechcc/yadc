use crate::ISQ;
use yadc::Dimension;

/// The dimension of `AmountOfSubstance`: `SIQuantity<AMOUNT_OF_SUBSTANCE, N>` is one.
pub const AMOUNT_OF_SUBSTANCE: ISQ = ISQ {
    mol: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MolarConcentration`: `SIQuantity<MOLAR_CONCENTRATION, N>` is one.
pub const MOLAR_CONCENTRATION: ISQ = ISQ {
    m: -3,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MolarEnergy`: `SIQuantity<MOLAR_ENERGY, N>` is one.
pub const MOLAR_ENERGY: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MolarFlux`: `SIQuantity<MOLAR_FLUX, N>` is one.
pub const MOLAR_FLUX: ISQ = ISQ {
    m: -2,
    s: -1,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MolarHeatCapacity`: `SIQuantity<MOLAR_HEAT_CAPACITY, N>` is one.
pub const MOLAR_HEAT_CAPACITY: ISQ = ISQ {
    m: 2,
    kg: 1,
    s: -2,
    K: -1,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MolarMass`: `SIQuantity<MOLAR_MASS, N>` is one.
pub const MOLAR_MASS: ISQ = ISQ {
    kg: 1,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MolarRadioactivity`: `SIQuantity<MOLAR_RADIOACTIVITY, N>` is one.
pub const MOLAR_RADIOACTIVITY: ISQ = ISQ {
    s: -1,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MolarVolume`: `SIQuantity<MOLAR_VOLUME, N>` is one.
pub const MOLAR_VOLUME: ISQ = ISQ {
    m: 3,
    mol: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Molality`: `SIQuantity<MOLALITY, N>` is one.
pub const MOLALITY: ISQ = ISQ {
    kg: -1,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `MassConcentration`: `SIQuantity<MASS_CONCENTRATION, N>` is one.
pub const MASS_CONCENTRATION: ISQ = ISQ {
    m: -3,
    kg: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `CatalyticActivity`: `SIQuantity<CATALYTIC_ACTIVITY, N>` is one.
pub const CATALYTIC_ACTIVITY: ISQ = ISQ {
    s: -1,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `CatalyticActivityConcentration`: `SIQuantity<CATALYTIC_ACTIVITY_CONCENTRATION, N>` is one.
pub const CATALYTIC_ACTIVITY_CONCENTRATION: ISQ = ISQ {
    m: -3,
    s: -1,
    mol: 1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `Radioactivity`: `SIQuantity<RADIOACTIVITY, N>` is one.
pub const RADIOACTIVITY: ISQ = ISQ {
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `SpecificRadioactivity`: `SIQuantity<SPECIFIC_RADIOACTIVITY, N>` is one.
pub const SPECIFIC_RADIOACTIVITY: ISQ = ISQ {
    kg: -1,
    s: -1,
    ..ISQ::DIMENSIONLESS
};

/// The dimension of `InverseAmountOfSubstance`: `SIQuantity<INVERSE_AMOUNT_OF_SUBSTANCE, N>` is one.
pub const INVERSE_AMOUNT_OF_SUBSTANCE: ISQ = ISQ {
    mol: -1,
    ..ISQ::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, SIUnit, SIUnitConsts};

    /// Moles (`mol`), a unit of `AmountOfSubstance`: `n` of them is `n · 1.0` coherent units.
    pub const MOLE: SIUnit<
        AMOUNT_OF_SUBSTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol",
                name: "moles",
            }
        },
    > = SIUnit;

    /// Particles (`particle`), a unit of `AmountOfSubstance`: `n` of them is `n · 1.0 / 6.02214076e23` coherent units.
    pub const PARTICLE: SIUnit<
        AMOUNT_OF_SUBSTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "particle",
                name: "particles",
            }
        },
    > = SIUnit;

    /// Moles per cubic metre (`mol/m³`), a unit of `MolarConcentration`: `n` of them is `n · 1.0` coherent units.
    pub const MOLE_PER_CUBIC_METRE: SIUnit<
        MOLAR_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol/m³",
                name: "moles per cubic metre",
            }
        },
    > = SIUnit;

    /// Moles per litre (`mol/L`), a unit of `MolarConcentration`: `n` of them is `n · 1000.0` coherent units.
    pub const MOLE_PER_LITRE: SIUnit<
        MOLAR_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "mol/L",
                name: "moles per litre",
            }
        },
    > = SIUnit;

    /// Particles per cubic metre (`particle/m³`), a unit of `MolarConcentration`: `n` of them is `n · 1.0 / 6.02214076e23` coherent units.
    pub const PARTICLE_PER_CUBIC_METRE: SIUnit<
        MOLAR_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "particle/m³",
                name: "particles per cubic metre",
            }
        },
    > = SIUnit;

    /// Kilograms square metre per second squared mole (`kg · m²/(s² · mol)`), a unit of `MolarEnergy`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_SQUARE_METRE_PER_SECOND_SQUARED_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg · m²/(s² · mol)",
                name: "kilograms square metre per second squared mole",
            }
        },
    > = SIUnit;

    /// Joules per mole (`J/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/mol",
                name: "joules per mole",
            }
        },
    > = SIUnit;

    /// Watt hours per mole (`W · h/mol`), a unit of `MolarEnergy`: `n` of them is `n · 3600.0` coherent units.
    pub const WATT_HOUR_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(3600),
                zero: Exact::ZERO,
                symbol: "W · h/mol",
                name: "watt hours per mole",
            }
        },
    > = SIUnit;

    /// British thermal units (IT) per mole (`Btu (IT)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1055.05585262` coherent units.
    pub const BTU_IT_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/mol",
                name: "British thermal units (IT) per mole",
            }
        },
    > = SIUnit;

    /// British thermal units per mole (`Btu/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4184.0 * 0.45359237 / 1.8` coherent units.
    pub const BTU_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1),
                zero: Exact::ZERO,
                symbol: "Btu/mol",
                name: "British thermal units per mole",
            }
        },
    > = SIUnit;

    /// Calories (IT) per mole (`cal (IT)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4.1868` coherent units.
    pub const CALORIE_IT_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -4),
                zero: Exact::ZERO,
                symbol: "cal (IT)/mol",
                name: "calories (IT) per mole",
            }
        },
    > = SIUnit;

    /// Calories per mole (`cal/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4.184` coherent units.
    pub const CALORIE_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal/mol",
                name: "calories per mole",
            }
        },
    > = SIUnit;

    /// Calories (IT) per mole (`Cal (IT)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4186.8` coherent units.
    pub const CALORIE_IT_NUTRITION_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Cal (IT)/mol",
                name: "Calories (IT) per mole",
            }
        },
    > = SIUnit;

    /// Calories per mole (`Cal/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4184.0` coherent units.
    pub const CALORIE_NUTRITION_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Cal/mol",
                name: "Calories per mole",
            }
        },
    > = SIUnit;

    /// Electronvolts per mole (`eV/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1.602176634e-19` coherent units.
    pub const ELECTRONVOLT_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28),
                zero: Exact::ZERO,
                symbol: "eV/mol",
                name: "electronvolts per mole",
            }
        },
    > = SIUnit;

    /// Ergs per mole (`erg/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1e-07` coherent units.
    pub const ERG_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1, -7),
                zero: Exact::ZERO,
                symbol: "erg/mol",
                name: "ergs per mole",
            }
        },
    > = SIUnit;

    /// Foot poundals per mole (`ft · pdl/mol`), a unit of `MolarEnergy`: `n` of them is `n · 0.3048 * 0.45359237 * 0.3048` coherent units.
    pub const FOOT_POUNDAL_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(3048, -4),
                zero: Exact::ZERO,
                symbol: "ft · pdl/mol",
                name: "foot poundals per mole",
            }
        },
    > = SIUnit;

    /// Foot pounds-force per mole (`ft · lbf/mol`), a unit of `MolarEnergy`: `n` of them is `n · 0.3048 * 0.45359237 * 9.80665` coherent units.
    pub const FOOT_POUND_FORCE_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(3048, -4) * Exact::dec(45359237, -8) * Exact::dec(980665, -5),
                zero: Exact::ZERO,
                symbol: "ft · lbf/mol",
                name: "foot pounds-force per mole",
            }
        },
    > = SIUnit;

    /// Quads per mole (`10¹⁵ Btu (IT)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1.05505585262e18` coherent units.
    pub const QUAD_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1055055852620000000),
                zero: Exact::ZERO,
                symbol: "10¹⁵ Btu (IT)/mol",
                name: "quads per mole",
            }
        },
    > = SIUnit;

    /// Therms (EC) per mole (`thm (EC)/mol`), a unit of `MolarEnergy`: `n` of them is `n · 105506000.0` coherent units.
    pub const THERM_EC_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(105506000),
                zero: Exact::ZERO,
                symbol: "thm (EC)/mol",
                name: "therms (EC) per mole",
            }
        },
    > = SIUnit;

    /// Therms per mole (`thm/mol`), a unit of `MolarEnergy`: `n` of them is `n · 105480400.0` coherent units.
    pub const THERM_US_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(105480400),
                zero: Exact::ZERO,
                symbol: "thm/mol",
                name: "therms per mole",
            }
        },
    > = SIUnit;

    /// Tons of TNT per mole (`t of TNT/mol`), a unit of `MolarEnergy`: `n` of them is `n · 4184000000.0` coherent units.
    pub const TON_TNT_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(4184000000),
                zero: Exact::ZERO,
                symbol: "t of TNT/mol",
                name: "tons of TNT per mole",
            }
        },
    > = SIUnit;

    /// Watt seconds per mole (`W · s/mol`), a unit of `MolarEnergy`: `n` of them is `n · 1.0` coherent units.
    pub const WATT_SECOND_PER_MOLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "W · s/mol",
                name: "watt seconds per mole",
            }
        },
    > = SIUnit;

    /// Joules per particle (`J/particle`), a unit of `MolarEnergy`: `n` of them is `n · 6.02214076e+23` coherent units.
    pub const JOULE_PER_PARTICLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "J/particle",
                name: "joules per particle",
            }
        },
    > = SIUnit;

    /// Electronvolts per particle (`eV/particle`), a unit of `MolarEnergy`: `n` of them is `n · 1.602176634e-19 * 6.02214076e23` coherent units.
    pub const ELECTRONVOLT_PER_PARTICLE: SIUnit<
        MOLAR_ENERGY,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "eV/particle",
                name: "electronvolts per particle",
            }
        },
    > = SIUnit;

    /// Moles per square metre second (`mol/(m² · s)`), a unit of `MolarFlux`: `n` of them is `n · 1.0` coherent units.
    pub const MOLE_PER_SQUARE_METRE_SECOND: SIUnit<
        MOLAR_FLUX,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol/(m² · s)",
                name: "moles per square metre second",
            }
        },
    > = SIUnit;

    /// Joules per kelvin mole (`J/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 1.0` coherent units.
    pub const JOULE_PER_KELVIN_MOLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "J/(K · mol)",
                name: "joules per kelvin mole",
            }
        },
    > = SIUnit;

    /// British thermal units (IT) per kelvin mole (`Btu (IT)/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 1055.05585262` coherent units.
    pub const BTU_IT_PER_KELVIN_MOLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(105505585262, -8),
                zero: Exact::ZERO,
                symbol: "Btu (IT)/(K · mol)",
                name: "British thermal units (IT) per kelvin mole",
            }
        },
    > = SIUnit;

    /// British thermal units per kelvin mole (`Btu/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4184.0 * 0.45359237 / 1.8` coherent units.
    pub const BTU_PER_KELVIN_MOLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(4184) * Exact::dec(45359237, -8) / Exact::dec(18, -1),
                zero: Exact::ZERO,
                symbol: "Btu/(K · mol)",
                name: "British thermal units per kelvin mole",
            }
        },
    > = SIUnit;

    /// Calories (IT) per kelvin mole (`cal (IT)/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4.1868` coherent units.
    pub const CALORIE_IT_PER_KELVIN_MOLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -4),
                zero: Exact::ZERO,
                symbol: "cal (IT)/(K · mol)",
                name: "calories (IT) per kelvin mole",
            }
        },
    > = SIUnit;

    /// Calories per kelvin mole (`cal/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4.184` coherent units.
    pub const CALORIE_PER_KELVIN_MOLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(4184, -3),
                zero: Exact::ZERO,
                symbol: "cal/(K · mol)",
                name: "calories per kelvin mole",
            }
        },
    > = SIUnit;

    /// Calories (IT) per kelvin mole (`Cal (IT)/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4186.8` coherent units.
    pub const CALORIE_IT_NUTRITION_PER_KELVIN_MOLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(41868, -1),
                zero: Exact::ZERO,
                symbol: "Cal (IT)/(K · mol)",
                name: "Calories (IT) per kelvin mole",
            }
        },
    > = SIUnit;

    /// Calories per kelvin mole (`Cal/(K · mol)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 4184.0` coherent units.
    pub const CALORIE_NUTRITION_PER_KELVIN_MOLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(4184),
                zero: Exact::ZERO,
                symbol: "Cal/(K · mol)",
                name: "Calories per kelvin mole",
            }
        },
    > = SIUnit;

    /// Joules per kelvin particle (`J/(K · particle)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 6.02214076e+23` coherent units.
    pub const JOULE_PER_KELVIN_PARTICLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "J/(K · particle)",
                name: "joules per kelvin particle",
            }
        },
    > = SIUnit;

    /// Electronvolts per kelvin particle (`eV/(K · particle)`), a unit of `MolarHeatCapacity`: `n` of them is `n · 1.602176634e-19 * 6.02214076e23` coherent units.
    pub const ELECTRONVOLT_PER_KELVIN_PARTICLE: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "eV/(K · particle)",
                name: "electronvolts per kelvin particle",
            }
        },
    > = SIUnit;

    /// Molar gas constants (`R`), a unit of `MolarHeatCapacity`: `n` of them is `n · 8.314462618` coherent units.
    pub const MOLAR_GAS_CONSTANT: SIUnit<
        MOLAR_HEAT_CAPACITY,
        {
            SIUnitConsts {
                size: Exact::dec(8314462618, -9),
                zero: Exact::ZERO,
                symbol: "R",
                name: "molar gas constants",
            }
        },
    > = SIUnit;

    /// Kilograms per mole (`kg/mol`), a unit of `MolarMass`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_MOLE: SIUnit<
        MOLAR_MASS,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/mol",
                name: "kilograms per mole",
            }
        },
    > = SIUnit;

    /// Grams per mole (`g/mol`), a unit of `MolarMass`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_PER_MOLE: SIUnit<
        MOLAR_MASS,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/mol",
                name: "grams per mole",
            }
        },
    > = SIUnit;

    /// Becquerels per mole (`Bq/mol`), a unit of `MolarRadioactivity`: `n` of them is `n · 1.0` coherent units.
    pub const BECQUEREL_PER_MOLE: SIUnit<
        MOLAR_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq/mol",
                name: "becquerels per mole",
            }
        },
    > = SIUnit;

    /// Curies per mole (`Ci/mol`), a unit of `MolarRadioactivity`: `n` of them is `n · 37000000000.0` coherent units.
    pub const CURIE_PER_MOLE: SIUnit<
        MOLAR_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci/mol",
                name: "curies per mole",
            }
        },
    > = SIUnit;

    /// Disintegrations per minute per mole (`dpm/mol`), a unit of `MolarRadioactivity`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_MOLE: SIUnit<
        MOLAR_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "dpm/mol",
                name: "disintegrations per minute per mole",
            }
        },
    > = SIUnit;

    /// Cubic metres per mole (`m³/mol`), a unit of `MolarVolume`: `n` of them is `n · 1.0` coherent units.
    pub const CUBIC_METRE_PER_MOLE: SIUnit<
        MOLAR_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "m³/mol",
                name: "cubic metres per mole",
            }
        },
    > = SIUnit;

    /// Cubic metres per particle (`m³/particle`), a unit of `MolarVolume`: `n` of them is `n · 6.02214076e+23` coherent units.
    pub const CUBIC_METRE_PER_PARTICLE: SIUnit<
        MOLAR_VOLUME,
        {
            SIUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "m³/particle",
                name: "cubic metres per particle",
            }
        },
    > = SIUnit;

    /// Moles per kilogram (`mol/kg`), a unit of `Molality`: `n` of them is `n · 1.0` coherent units.
    pub const MOLE_PER_KILOGRAM: SIUnit<
        MOLALITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol/kg",
                name: "moles per kilogram",
            }
        },
    > = SIUnit;

    /// Kilograms per cubic metre (`kg/m³`), a unit of `MassConcentration`: `n` of them is `n · 1.0` coherent units.
    pub const KILOGRAM_PER_CUBIC_METRE: SIUnit<
        MASS_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kg/m³",
                name: "kilograms per cubic metre",
            }
        },
    > = SIUnit;

    /// Grams per cubic metre (`g/m³`), a unit of `MassConcentration`: `n` of them is `n · 0.001` coherent units.
    pub const GRAM_PER_CUBIC_METRE: SIUnit<
        MASS_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::dec(1, -3),
                zero: Exact::ZERO,
                symbol: "g/m³",
                name: "grams per cubic metre",
            }
        },
    > = SIUnit;

    /// Katals (`kat`), a unit of `CatalyticActivity`: `n` of them is `n · 1.0` coherent units.
    pub const KATAL: SIUnit<
        CATALYTIC_ACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kat",
                name: "katals",
            }
        },
    > = SIUnit;

    /// Particles per second (`particle/s`), a unit of `CatalyticActivity`: `n` of them is `n · 1.0 / 6.02214076e23` coherent units.
    pub const PARTICLE_PER_SECOND: SIUnit<
        CATALYTIC_ACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "particle/s",
                name: "particles per second",
            }
        },
    > = SIUnit;

    /// Moles per second (`mol/s`), a unit of `CatalyticActivity`: `n` of them is `n · 1.0` coherent units.
    pub const MOLE_PER_SECOND: SIUnit<
        CATALYTIC_ACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol/s",
                name: "moles per second",
            }
        },
    > = SIUnit;

    /// Katals per cubic metre (`kat/m³`), a unit of `CatalyticActivityConcentration`: `n` of them is `n · 1.0` coherent units.
    pub const KATAL_PER_CUBIC_METRE: SIUnit<
        CATALYTIC_ACTIVITY_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "kat/m³",
                name: "katals per cubic metre",
            }
        },
    > = SIUnit;

    /// Katals per litre (`kat/L`), a unit of `CatalyticActivityConcentration`: `n` of them is `n · 1000.0` coherent units.
    pub const KATAL_PER_LITRE: SIUnit<
        CATALYTIC_ACTIVITY_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1000),
                zero: Exact::ZERO,
                symbol: "kat/L",
                name: "katals per litre",
            }
        },
    > = SIUnit;

    /// Particles per second cubic metre (`particle · s⁻¹ · m⁻³`), a unit of `CatalyticActivityConcentration`: `n` of them is `n · 1.0 / 6.02214076e23` coherent units.
    pub const PARTICLE_PER_SECOND_CUBIC_METRE: SIUnit<
        CATALYTIC_ACTIVITY_CONCENTRATION,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "particle · s⁻¹ · m⁻³",
                name: "particles per second cubic metre",
            }
        },
    > = SIUnit;

    /// Becquerels (`Bq`), a unit of `Radioactivity`: `n` of them is `n · 1.0` coherent units.
    pub const BECQUEREL: SIUnit<
        RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq",
                name: "becquerels",
            }
        },
    > = SIUnit;

    /// Curies (`Ci`), a unit of `Radioactivity`: `n` of them is `n · 37000000000.0` coherent units.
    pub const CURIE: SIUnit<
        RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci",
                name: "curies",
            }
        },
    > = SIUnit;

    /// Disintegrations per minute (`dpm`), a unit of `Radioactivity`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub const DISINTEGRATIONS_PER_MINUTE: SIUnit<
        RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "dpm",
                name: "disintegrations per minute",
            }
        },
    > = SIUnit;

    /// Becquerels per kilogram (`Bq/kg`), a unit of `SpecificRadioactivity`: `n` of them is `n · 1.0` coherent units.
    pub const BECQUEREL_PER_KILOGRAM: SIUnit<
        SPECIFIC_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "Bq/kg",
                name: "becquerels per kilogram",
            }
        },
    > = SIUnit;

    /// Curie per kilogram (`Ci/kg`), a unit of `SpecificRadioactivity`: `n` of them is `n · 37000000000.0` coherent units.
    pub const CURIE_PER_KILOGRAM: SIUnit<
        SPECIFIC_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci/kg",
                name: "curie per kilogram",
            }
        },
    > = SIUnit;

    /// Disintegrations per minute per kilogram (`dpm/kg`), a unit of `SpecificRadioactivity`: `n` of them is `n · 1.0 / 60.0` coherent units.
    pub const DISINTEGRATIONS_PER_MINUTE_PER_KILOGRAM: SIUnit<
        SPECIFIC_RADIOACTIVITY,
        {
            SIUnitConsts {
                size: Exact::int(1) / Exact::int(60),
                zero: Exact::ZERO,
                symbol: "dpm/kg",
                name: "disintegrations per minute per kilogram",
            }
        },
    > = SIUnit;

    /// Per mole (`mol⁻¹`), a unit of `InverseAmountOfSubstance`: `n` of them is `n · 1.0` coherent units.
    pub const PER_MOLE: SIUnit<
        INVERSE_AMOUNT_OF_SUBSTANCE,
        {
            SIUnitConsts {
                size: Exact::int(1),
                zero: Exact::ZERO,
                symbol: "mol⁻¹",
                name: "per mole",
            }
        },
    > = SIUnit;

    /// Avogadro constants (`N_A`), a unit of `InverseAmountOfSubstance`: `n` of them is `n · 6.02214076e23` coherent units.
    pub const AVOGADRO_CONSTANT: SIUnit<
        INVERSE_AMOUNT_OF_SUBSTANCE,
        {
            SIUnitConsts {
                size: Exact::int(602214076000000000000000),
                zero: Exact::ZERO,
                symbol: "N_A",
                name: "Avogadro constants",
            }
        },
    > = SIUnit;
}
