use crate::Dim;
use yadc::Dimension;

/// The dimension of `Acceleration`: `Qty<ACCELERATION, N>` is one.
pub const ACCELERATION: Dim = Dim {
    m: 1,
    s: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Brightness`: `Qty<BRIGHTNESS, N>` is one.
pub const BRIGHTNESS: Dim = Dim {
    m: -2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `DoseEquivalent`: `Qty<DOSE_EQUIVALENT, N>` is one.
pub const DOSE_EQUIVALENT: Dim = Dim {
    m: 2,
    s: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `DynamicViscosity`: `Qty<DYNAMIC_VISCOSITY, N>` is one.
pub const DYNAMIC_VISCOSITY: Dim = Dim {
    m: -1,
    s: -1,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `ElectricCurrent`: `Qty<ELECTRIC_CURRENT, N>` is one.
pub const ELECTRIC_CURRENT: Dim = Dim {
    s: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Energy`: `Qty<ENERGY, N>` is one.
pub const ENERGY: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `EnergyDose`: `Qty<ENERGY_DOSE, N>` is one.
pub const ENERGY_DOSE: Dim = Dim {
    m: 2,
    s: -2,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `FluxOfMagneticInduction`: `Qty<FLUX_OF_MAGNETIC_INDUCTION, N>` is one.
pub const FLUX_OF_MAGNETIC_INDUCTION: Dim = Dim {
    m: 2,
    s: -1,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Force`: `Qty<FORCE, N>` is one.
pub const FORCE: Dim = Dim {
    m: 1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Illuminance`: `Qty<ILLUMINANCE, N>` is one.
pub const ILLUMINANCE: Dim = Dim {
    m: -2,
    rad: 2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `IonDose`: `Qty<ION_DOSE, N>` is one.
pub const ION_DOSE: Dim = Dim {
    g: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `KinematicViscosity`: `Qty<KINEMATIC_VISCOSITY, N>` is one.
pub const KINEMATIC_VISCOSITY: Dim = Dim {
    m: 2,
    s: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `LineicNumber`: `Qty<LINEIC_NUMBER, N>` is one.
pub const LINEIC_NUMBER: Dim = Dim {
    m: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `LumIntensityDensity`: `Qty<LUM_INTENSITY_DENSITY, N>` is one.
pub const LUM_INTENSITY_DENSITY: Dim = Dim {
    m: -2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `MagneticFieldIntensity`: `Qty<MAGNETIC_FIELD_INTENSITY, N>` is one.
pub const MAGNETIC_FIELD_INTENSITY: Dim = Dim {
    m: -1,
    s: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `MagneticFluxDensity`: `Qty<MAGNETIC_FLUX_DENSITY, N>` is one.
pub const MAGNETIC_FLUX_DENSITY: Dim = Dim {
    s: -1,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `MagneticTension`: `Qty<MAGNETIC_TENSION, N>` is one.
pub const MAGNETIC_TENSION: Dim = Dim {
    s: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};

/// The dimension of `Radioactivity`: `Qty<RADIOACTIVITY, N>` is one.
pub const RADIOACTIVITY: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};

/// The units of this module's quantities: each a constant of its own unit
/// type, its size and zero in its quantity's coherent unit, exact
/// (`MILE.quantity(3.0)`, `MILE.value(d)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Gal (`Gal`), a unit of `Acceleration`: `n` of them is `n · 1e-2` coherent units.
    pub const GAL: UcumUnit<
        ACCELERATION,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "Gal",
                name: "Gal",
            }
        },
    > = UcumUnit;

    /// Lambert (`Lmb`), a unit of `Brightness`: `n` of them is `n · 1e4 / core::f64::consts::PI` coherent units.
    pub const LAMBERT: UcumUnit<
        BRIGHTNESS,
        {
            UcumUnitConsts {
                size: Exact::int(10000) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "Lmb",
                name: "Lambert",
            }
        },
    > = UcumUnit;

    /// Radiation equivalent man (`REM`), a unit of `DoseEquivalent`: `n` of them is `n · 1e-2` coherent units.
    pub const RADIATION_EQUIVALENT_MAN: UcumUnit<
        DOSE_EQUIVALENT,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "REM",
                name: "radiation equivalent man",
            }
        },
    > = UcumUnit;

    /// Poise (`P`), a unit of `DynamicViscosity`: `n` of them is `n · 1e2` coherent units.
    pub const POISE: UcumUnit<
        DYNAMIC_VISCOSITY,
        {
            UcumUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "P",
                name: "Poise",
            }
        },
    > = UcumUnit;

    /// Biot (`Bi`), a unit of `ElectricCurrent`: `n` of them is `n · 1e1` coherent units.
    pub const BIOT: UcumUnit<
        ELECTRIC_CURRENT,
        {
            UcumUnitConsts {
                size: Exact::int(10),
                zero: Exact::ZERO,
                symbol: "Bi",
                name: "Biot",
            }
        },
    > = UcumUnit;

    /// Erg (`erg`), a unit of `Energy`: `n` of them is `n · 1e-4` coherent units.
    pub const ERG: UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "erg",
                name: "erg",
            }
        },
    > = UcumUnit;

    /// Radiation absorbed dose (`RAD`), a unit of `EnergyDose`: `n` of them is `n · 1e-2` coherent units.
    pub const RADIATION_ABSORBED_DOSE: UcumUnit<
        ENERGY_DOSE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "RAD",
                name: "radiation absorbed dose",
            }
        },
    > = UcumUnit;

    /// Maxwell (`Mx`), a unit of `FluxOfMagneticInduction`: `n` of them is `n · 1e-5` coherent units.
    pub const MAXWELL: UcumUnit<
        FLUX_OF_MAGNETIC_INDUCTION,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -5),
                zero: Exact::ZERO,
                symbol: "Mx",
                name: "Maxwell",
            }
        },
    > = UcumUnit;

    /// Dyne (`dyn`), a unit of `Force`: `n` of them is `n · 1e-2` coherent units.
    pub const DYNE: UcumUnit<
        FORCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "dyn",
                name: "dyne",
            }
        },
    > = UcumUnit;

    /// Phot (`ph`), a unit of `Illuminance`: `n` of them is `n · 1e-4` coherent units.
    pub const PHOT: UcumUnit<
        ILLUMINANCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "ph",
                name: "phot",
            }
        },
    > = UcumUnit;

    /// Roentgen (`R`), a unit of `IonDose`: `n` of them is `n · 258e-9` coherent units.
    pub const ROENTGEN: UcumUnit<
        ION_DOSE,
        {
            UcumUnitConsts {
                size: Exact::dec(258, -9),
                zero: Exact::ZERO,
                symbol: "R",
                name: "Roentgen",
            }
        },
    > = UcumUnit;

    /// Stokes (`St`), a unit of `KinematicViscosity`: `n` of them is `n · 1e-4` coherent units.
    pub const STOKES: UcumUnit<
        KINEMATIC_VISCOSITY,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "St",
                name: "Stokes",
            }
        },
    > = UcumUnit;

    /// Kayser (`Ky`), a unit of `LineicNumber`: `n` of them is `n · 1e2` coherent units.
    pub const KAYSER: UcumUnit<
        LINEIC_NUMBER,
        {
            UcumUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "Ky",
                name: "Kayser",
            }
        },
    > = UcumUnit;

    /// Stilb (`sb`), a unit of `LumIntensityDensity`: `n` of them is `n · 1e4` coherent units.
    pub const STILB: UcumUnit<
        LUM_INTENSITY_DENSITY,
        {
            UcumUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "sb",
                name: "stilb",
            }
        },
    > = UcumUnit;

    /// Oersted (`Oe`), a unit of `MagneticFieldIntensity`: `n` of them is `n · 25e1 / core::f64::consts::PI` coherent units.
    pub const OERSTED: UcumUnit<
        MAGNETIC_FIELD_INTENSITY,
        {
            UcumUnitConsts {
                size: Exact::int(250) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "Oe",
                name: "Oersted",
            }
        },
    > = UcumUnit;

    /// Gauss (`G`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e-1` coherent units.
    pub const GAUSS: UcumUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "G",
                name: "Gauss",
            }
        },
    > = UcumUnit;

    /// Gilbert (`Gb`), a unit of `MagneticTension`: `n` of them is `n · 25e-1 / core::f64::consts::PI` coherent units.
    pub const GILBERT: UcumUnit<
        MAGNETIC_TENSION,
        {
            UcumUnitConsts {
                size: Exact::dec(25, -1) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "Gb",
                name: "Gilbert",
            }
        },
    > = UcumUnit;

    /// Curie (`Ci`), a unit of `Radioactivity`: `n` of them is `n · 37e9` coherent units.
    pub const CURIE: UcumUnit<
        RADIOACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci",
                name: "Curie",
            }
        },
    > = UcumUnit;
}
