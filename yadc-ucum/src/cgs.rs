//! The quantities of `cgs` and their units.

#[allow(unused_imports)]
use crate::*;

// `Acceleration` is `crate::Acceleration`.

/// Gal (`Gal`), a unit of `Acceleration`: `n` of them is `n · 1e-2` base units.
pub const GAL: Qty<
    {
        UcumUnit::scaled(
            "Gal",
            Exact::dec(1, -2),
            METER.unit().times(SECOND.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A brightness, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Brightness<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -2)
            .times(CANDELA.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Lambert (`Lmb`), a unit of `Brightness`: `n` of them is `n · 1e4 / core::f64::consts::PI` base units.
pub const LAMBERT: Qty<
    {
        UcumUnit::scaled(
            "Lmb",
            Exact::int(10000) / Exact::PI,
            UcumUnit::ONE
                .times(METER.unit(), -2)
                .times(CANDELA.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `DoseEquivalent` is `crate::DoseEquivalent`.

/// Radiation equivalent man (`REM`), a unit of `DoseEquivalent`: `n` of them is `n · 1e-2` base units.
pub const RADIATION_EQUIVALENT_MAN: Qty<
    {
        UcumUnit::scaled(
            "REM",
            Exact::dec(1, -2),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A dynamic viscosity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type DynamicViscosity<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -1)
            .times(SECOND.unit(), -1)
            .times(GRAM.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Poise (`P`), a unit of `DynamicViscosity`: `n` of them is `n · 1e2` base units.
pub const POISE: Qty<
    {
        UcumUnit::scaled(
            "P",
            Exact::int(100),
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `ElectricCurrent` is `crate::ElectricCurrent`.

/// Biot (`Bi`), a unit of `ElectricCurrent`: `n` of them is `n · 1e1` base units.
pub const BIOT: Qty<
    {
        UcumUnit::scaled(
            "Bi",
            Exact::int(10),
            UcumUnit::ONE
                .times(SECOND.unit(), -1)
                .times(COULOMB.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Energy` is `crate::Energy`.

/// Erg (`erg`), a unit of `Energy`: `n` of them is `n · 1e-4` base units.
pub const ERG: Qty<
    {
        UcumUnit::scaled(
            "erg",
            Exact::dec(1, -4),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2)
                .times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `EnergyDose` is `crate::EnergyDose`.

/// Radiation absorbed dose (`RAD`), a unit of `EnergyDose`: `n` of them is `n · 1e-2` base units.
pub const RADIATION_ABSORBED_DOSE: Qty<
    {
        UcumUnit::scaled(
            "RAD",
            Exact::dec(1, -2),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A flux of magnetic induction, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type FluxOfMagneticInduction<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -1)
            .times(GRAM.unit(), 1)
            .times(COULOMB.unit(), -1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Maxwell (`Mx`), a unit of `FluxOfMagneticInduction`: `n` of them is `n · 1e-5` base units.
pub const MAXWELL: Qty<
    {
        UcumUnit::scaled(
            "Mx",
            Exact::dec(1, -5),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1)
                .times(COULOMB.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Force` is `crate::Force`.

/// Dyne (`dyn`), a unit of `Force`: `n` of them is `n · 1e-2` base units.
pub const DYNE: Qty<
    {
        UcumUnit::scaled(
            "dyn",
            Exact::dec(1, -2),
            METER.unit().times(SECOND.unit(), -2).times(GRAM.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Illuminance` is `crate::Illuminance`.

/// Phot (`ph`), a unit of `Illuminance`: `n` of them is `n · 1e-4` base units.
pub const PHOT: Qty<
    {
        UcumUnit::scaled(
            "ph",
            Exact::dec(1, -4),
            UcumUnit::ONE
                .times(METER.unit(), -2)
                .times(RADIAN.unit(), 2)
                .times(CANDELA.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// An ion dose, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type IonDose<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(GRAM.unit(), -1)
            .times(COULOMB.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Roentgen (`R`), a unit of `IonDose`: `n` of them is `n · 258e-9` base units.
pub const ROENTGEN: Qty<
    {
        UcumUnit::scaled(
            "R",
            Exact::dec(258, -9),
            UcumUnit::ONE
                .times(GRAM.unit(), -1)
                .times(COULOMB.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A kinematic viscosity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type KinematicViscosity<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), 2)
            .times(SECOND.unit(), -1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Stokes (`St`), a unit of `KinematicViscosity`: `n` of them is `n · 1e-4` base units.
pub const STOKES: Qty<
    {
        UcumUnit::scaled(
            "St",
            Exact::dec(1, -4),
            UcumUnit::ONE
                .times(METER.unit(), 2)
                .times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `LineicNumber` is `crate::LineicNumber`.

/// Kayser (`Ky`), a unit of `LineicNumber`: `n` of them is `n · 1e2` base units.
pub const KAYSER: Qty<
    { UcumUnit::scaled("Ky", Exact::int(100), UcumUnit::ONE.times(METER.unit(), -1)) },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A lum intensity density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type LumIntensityDensity<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -2)
            .times(CANDELA.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Stilb (`sb`), a unit of `LumIntensityDensity`: `n` of them is `n · 1e4` base units.
pub const STILB: Qty<
    {
        UcumUnit::scaled(
            "sb",
            Exact::int(10000),
            UcumUnit::ONE
                .times(METER.unit(), -2)
                .times(CANDELA.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A magnetic field intensity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticFieldIntensity<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(METER.unit(), -1)
            .times(SECOND.unit(), -1)
            .times(COULOMB.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Oersted (`Oe`), a unit of `MagneticFieldIntensity`: `n` of them is `n · 25e1 / core::f64::consts::PI` base units.
pub const OERSTED: Qty<
    {
        UcumUnit::scaled(
            "Oe",
            Exact::int(250) / Exact::PI,
            UcumUnit::ONE
                .times(METER.unit(), -1)
                .times(SECOND.unit(), -1)
                .times(COULOMB.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `MagneticFluxDensity` is `crate::MagneticFluxDensity`.

/// Gauss (`G`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e-1` base units.
pub const GAUSS: Qty<
    {
        UcumUnit::scaled(
            "G",
            Exact::dec(1, -1),
            UcumUnit::ONE
                .times(SECOND.unit(), -1)
                .times(GRAM.unit(), 1)
                .times(COULOMB.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

/// A magnetic tension, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticTension<
    N,
    const U: UcumUnit = {
        UcumUnit::ONE
            .times(SECOND.unit(), -1)
            .times(COULOMB.unit(), 1)
    },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Gilbert (`Gb`), a unit of `MagneticTension`: `n` of them is `n · 25e-1 / core::f64::consts::PI` base units.
pub const GILBERT: Qty<
    {
        UcumUnit::scaled(
            "Gb",
            Exact::dec(25, -1) / Exact::PI,
            UcumUnit::ONE
                .times(SECOND.unit(), -1)
                .times(COULOMB.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;

// `Radioactivity` is `crate::Radioactivity`.

/// Curie (`Ci`), a unit of `Radioactivity`: `n` of them is `n · 37e9` base units.
pub const CURIE: Qty<
    {
        UcumUnit::scaled(
            "Ci",
            Exact::int(37000000000),
            UcumUnit::ONE.times(SECOND.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = Qty::ONE;
