use crate::{Dim, Qty};
use yadc::Dimension;

/// The dimension of `Acceleration`.
pub const ACCELERATION: Dim = Dim {
    m: 1,
    s: -2,
    ..Dim::DIMENSIONLESS
};
pub type Acceleration<N> = Qty<ACCELERATION, N>;

/// The dimension of `Brightness`.
pub const BRIGHTNESS: Dim = Dim {
    m: -2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};
pub type Brightness<N> = Qty<BRIGHTNESS, N>;

/// The dimension of `DoseEquivalent`.
pub const DOSE_EQUIVALENT: Dim = Dim {
    m: 2,
    s: -2,
    ..Dim::DIMENSIONLESS
};
pub type DoseEquivalent<N> = Qty<DOSE_EQUIVALENT, N>;

/// The dimension of `DynamicViscosity`.
pub const DYNAMIC_VISCOSITY: Dim = Dim {
    m: -1,
    s: -1,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type DynamicViscosity<N> = Qty<DYNAMIC_VISCOSITY, N>;

/// The dimension of `ElectricCurrent`.
pub const ELECTRIC_CURRENT: Dim = Dim {
    s: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};
pub type ElectricCurrent<N> = Qty<ELECTRIC_CURRENT, N>;

/// The dimension of `Energy`.
pub const ENERGY: Dim = Dim {
    m: 2,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Energy<N> = Qty<ENERGY, N>;

/// The dimension of `EnergyDose`.
pub const ENERGY_DOSE: Dim = Dim {
    m: 2,
    s: -2,
    ..Dim::DIMENSIONLESS
};
pub type EnergyDose<N> = Qty<ENERGY_DOSE, N>;

/// The dimension of `FluxOfMagneticInduction`.
pub const FLUX_OF_MAGNETIC_INDUCTION: Dim = Dim {
    m: 2,
    s: -1,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};
pub type FluxOfMagneticInduction<N> = Qty<FLUX_OF_MAGNETIC_INDUCTION, N>;

/// The dimension of `Force`.
pub const FORCE: Dim = Dim {
    m: 1,
    s: -2,
    g: 1,
    ..Dim::DIMENSIONLESS
};
pub type Force<N> = Qty<FORCE, N>;

/// The dimension of `Illuminance`.
pub const ILLUMINANCE: Dim = Dim {
    m: -2,
    rad: 2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};
pub type Illuminance<N> = Qty<ILLUMINANCE, N>;

/// The dimension of `IonDose`.
pub const ION_DOSE: Dim = Dim {
    g: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};
pub type IonDose<N> = Qty<ION_DOSE, N>;

/// The dimension of `KinematicViscosity`.
pub const KINEMATIC_VISCOSITY: Dim = Dim {
    m: 2,
    s: -1,
    ..Dim::DIMENSIONLESS
};
pub type KinematicViscosity<N> = Qty<KINEMATIC_VISCOSITY, N>;

/// The dimension of `LineicNumber`.
pub const LINEIC_NUMBER: Dim = Dim {
    m: -1,
    ..Dim::DIMENSIONLESS
};
pub type LineicNumber<N> = Qty<LINEIC_NUMBER, N>;

/// The dimension of `LumIntensityDensity`.
pub const LUM_INTENSITY_DENSITY: Dim = Dim {
    m: -2,
    cd: 1,
    ..Dim::DIMENSIONLESS
};
pub type LumIntensityDensity<N> = Qty<LUM_INTENSITY_DENSITY, N>;

/// The dimension of `MagneticFieldIntensity`.
pub const MAGNETIC_FIELD_INTENSITY: Dim = Dim {
    m: -1,
    s: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};
pub type MagneticFieldIntensity<N> = Qty<MAGNETIC_FIELD_INTENSITY, N>;

/// The dimension of `MagneticFluxDensity`.
pub const MAGNETIC_FLUX_DENSITY: Dim = Dim {
    s: -1,
    g: 1,
    C: -1,
    ..Dim::DIMENSIONLESS
};
pub type MagneticFluxDensity<N> = Qty<MAGNETIC_FLUX_DENSITY, N>;

/// The dimension of `MagneticTension`.
pub const MAGNETIC_TENSION: Dim = Dim {
    s: -1,
    C: 1,
    ..Dim::DIMENSIONLESS
};
pub type MagneticTension<N> = Qty<MAGNETIC_TENSION, N>;

/// The dimension of `Radioactivity`.
pub const RADIOACTIVITY: Dim = Dim {
    s: -1,
    ..Dim::DIMENSIONLESS
};
pub type Radioactivity<N> = Qty<RADIOACTIVITY, N>;

/// The units of this module's quantities: a type each, its size and zero
/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).
pub mod units {
    use super::*;
    use crate::{Exact, UcumUnit, UcumUnitConsts};

    /// Gal (`Gal`), a unit of `Acceleration`: `n` of them is `n · 1e-2` coherent units.
    pub type Gal = UcumUnit<
        ACCELERATION,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "Gal",
                name: "Gal",
            }
        },
    >;

    /// Lambert (`Lmb`), a unit of `Brightness`: `n` of them is `n · 1e4 / core::f64::consts::PI` coherent units.
    pub type Lambert = UcumUnit<
        BRIGHTNESS,
        {
            UcumUnitConsts {
                size: Exact::int(10000) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "Lmb",
                name: "Lambert",
            }
        },
    >;

    /// Radiation equivalent man (`REM`), a unit of `DoseEquivalent`: `n` of them is `n · 1e-2` coherent units.
    pub type RadiationEquivalentMan = UcumUnit<
        DOSE_EQUIVALENT,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "REM",
                name: "radiation equivalent man",
            }
        },
    >;

    /// Poise (`P`), a unit of `DynamicViscosity`: `n` of them is `n · 1e2` coherent units.
    pub type Poise = UcumUnit<
        DYNAMIC_VISCOSITY,
        {
            UcumUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "P",
                name: "Poise",
            }
        },
    >;

    /// Biot (`Bi`), a unit of `ElectricCurrent`: `n` of them is `n · 1e1` coherent units.
    pub type Biot = UcumUnit<
        ELECTRIC_CURRENT,
        {
            UcumUnitConsts {
                size: Exact::int(10),
                zero: Exact::ZERO,
                symbol: "Bi",
                name: "Biot",
            }
        },
    >;

    /// Erg (`erg`), a unit of `Energy`: `n` of them is `n · 1e-4` coherent units.
    pub type Erg = UcumUnit<
        ENERGY,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "erg",
                name: "erg",
            }
        },
    >;

    /// Radiation absorbed dose (`RAD`), a unit of `EnergyDose`: `n` of them is `n · 1e-2` coherent units.
    pub type RadiationAbsorbedDose = UcumUnit<
        ENERGY_DOSE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "RAD",
                name: "radiation absorbed dose",
            }
        },
    >;

    /// Maxwell (`Mx`), a unit of `FluxOfMagneticInduction`: `n` of them is `n · 1e-5` coherent units.
    pub type Maxwell = UcumUnit<
        FLUX_OF_MAGNETIC_INDUCTION,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -5),
                zero: Exact::ZERO,
                symbol: "Mx",
                name: "Maxwell",
            }
        },
    >;

    /// Dyne (`dyn`), a unit of `Force`: `n` of them is `n · 1e-2` coherent units.
    pub type Dyne = UcumUnit<
        FORCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -2),
                zero: Exact::ZERO,
                symbol: "dyn",
                name: "dyne",
            }
        },
    >;

    /// Phot (`ph`), a unit of `Illuminance`: `n` of them is `n · 1e-4` coherent units.
    pub type Phot = UcumUnit<
        ILLUMINANCE,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "ph",
                name: "phot",
            }
        },
    >;

    /// Roentgen (`R`), a unit of `IonDose`: `n` of them is `n · 258e-9` coherent units.
    pub type Roentgen = UcumUnit<
        ION_DOSE,
        {
            UcumUnitConsts {
                size: Exact::dec(258, -9),
                zero: Exact::ZERO,
                symbol: "R",
                name: "Roentgen",
            }
        },
    >;

    /// Stokes (`St`), a unit of `KinematicViscosity`: `n` of them is `n · 1e-4` coherent units.
    pub type Stokes = UcumUnit<
        KINEMATIC_VISCOSITY,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -4),
                zero: Exact::ZERO,
                symbol: "St",
                name: "Stokes",
            }
        },
    >;

    /// Kayser (`Ky`), a unit of `LineicNumber`: `n` of them is `n · 1e2` coherent units.
    pub type Kayser = UcumUnit<
        LINEIC_NUMBER,
        {
            UcumUnitConsts {
                size: Exact::int(100),
                zero: Exact::ZERO,
                symbol: "Ky",
                name: "Kayser",
            }
        },
    >;

    /// Stilb (`sb`), a unit of `LumIntensityDensity`: `n` of them is `n · 1e4` coherent units.
    pub type Stilb = UcumUnit<
        LUM_INTENSITY_DENSITY,
        {
            UcumUnitConsts {
                size: Exact::int(10000),
                zero: Exact::ZERO,
                symbol: "sb",
                name: "stilb",
            }
        },
    >;

    /// Oersted (`Oe`), a unit of `MagneticFieldIntensity`: `n` of them is `n · 25e1 / core::f64::consts::PI` coherent units.
    pub type Oersted = UcumUnit<
        MAGNETIC_FIELD_INTENSITY,
        {
            UcumUnitConsts {
                size: Exact::int(250) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "Oe",
                name: "Oersted",
            }
        },
    >;

    /// Gauss (`G`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e-1` coherent units.
    pub type Gauss = UcumUnit<
        MAGNETIC_FLUX_DENSITY,
        {
            UcumUnitConsts {
                size: Exact::dec(1, -1),
                zero: Exact::ZERO,
                symbol: "G",
                name: "Gauss",
            }
        },
    >;

    /// Gilbert (`Gb`), a unit of `MagneticTension`: `n` of them is `n · 25e-1 / core::f64::consts::PI` coherent units.
    pub type Gilbert = UcumUnit<
        MAGNETIC_TENSION,
        {
            UcumUnitConsts {
                size: Exact::dec(25, -1) / Exact::PI,
                zero: Exact::ZERO,
                symbol: "Gb",
                name: "Gilbert",
            }
        },
    >;

    /// Curie (`Ci`), a unit of `Radioactivity`: `n` of them is `n · 37e9` coherent units.
    pub type Curie = UcumUnit<
        RADIOACTIVITY,
        {
            UcumUnitConsts {
                size: Exact::int(37000000000),
                zero: Exact::ZERO,
                symbol: "Ci",
                name: "Curie",
            }
        },
    >;
}
