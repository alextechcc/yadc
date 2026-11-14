//! The quantities of `cgs` and their units.

#[allow(unused_imports)]
use crate::*;

// `Acceleration` is `crate::Acceleration`.

/// Gal (`Gal`), a unit of `Acceleration`: `n` of them is `n · 1e-2` base units.
pub const GAL: Unit<
    { UcumUnit::scaled("Gal", Exact::dec(1, -2), METER_PER_SECOND_SQUARED.unit()) },
> = Unit::ONE;

/// A brightness, in `N`s of the unit `U`: by default its base unit, `CANDELA_PER_METER_SQUARED`.
pub type Brightness<
    N,
    const U: UcumUnit = { CANDELA_PER_METER_SQUARED.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `Brightness`: a product of base units.
pub const CANDELA_PER_METER_SQUARED: Unit<{ (CANDELA / (METER * METER)).unit() }, BASE> = Unit::ONE;

/// Lambert (`Lmb`), a unit of `Brightness`: `n` of them is `n · 1e4 / core::f64::consts::PI` base units.
pub const LAMBERT: Unit<
    {
        UcumUnit::scaled(
            "Lmb",
            Exact::int(10000) / Exact::PI,
            CANDELA_PER_METER_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `DoseEquivalent` is `crate::DoseEquivalent`.

/// Radiation equivalent man (`REM`), a unit of `DoseEquivalent`: `n` of them is `n · 1e-2` base units.
pub const RADIATION_EQUIVALENT_MAN: Unit<
    {
        UcumUnit::scaled(
            "REM",
            Exact::dec(1, -2),
            METER_SQUARED_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// A dynamic viscosity, in `N`s of the unit `U`: by default its base unit, `GRAM_PER_METER_SECOND`.
pub type DynamicViscosity<
    N,
    const U: UcumUnit = { GRAM_PER_METER_SECOND.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `DynamicViscosity`: a product of base units.
pub const GRAM_PER_METER_SECOND: Unit<{ (GRAM / (METER * SECOND)).unit() }, BASE> = Unit::ONE;

/// Poise (`P`), a unit of `DynamicViscosity`: `n` of them is `n · 1e2` base units.
pub const POISE: Unit<{ UcumUnit::scaled("P", Exact::int(100), GRAM_PER_METER_SECOND.unit()) }> =
    Unit::ONE;

// `ElectricCurrent` is `crate::ElectricCurrent`.

/// Biot (`Bi`), a unit of `ElectricCurrent`: `n` of them is `n · 1e1` base units.
pub const BIOT: Unit<{ UcumUnit::scaled("Bi", Exact::int(10), COULOMB_PER_SECOND.unit()) }> =
    Unit::ONE;

// `Energy` is `crate::Energy`.

/// Erg (`erg`), a unit of `Energy`: `n` of them is `n · 1e-4` base units.
pub const ERG: Unit<
    {
        UcumUnit::scaled(
            "erg",
            Exact::dec(1, -4),
            METER_SQUARED_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `EnergyDose` is `crate::EnergyDose`.

/// Radiation absorbed dose (`RAD`), a unit of `EnergyDose`: `n` of them is `n · 1e-2` base units.
pub const RADIATION_ABSORBED_DOSE: Unit<
    {
        UcumUnit::scaled(
            "RAD",
            Exact::dec(1, -2),
            METER_SQUARED_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// A flux of magnetic induction, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED_GRAM_PER_SECOND_COULOMB`.
pub type FluxOfMagneticInduction<
    N,
    const U: UcumUnit = { METER_SQUARED_GRAM_PER_SECOND_COULOMB.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `FluxOfMagneticInduction`: a product of base units.
pub const METER_SQUARED_GRAM_PER_SECOND_COULOMB: Unit<
    { (METER * METER * GRAM / (SECOND * COULOMB)).unit() },
    BASE,
> = Unit::ONE;

/// Maxwell (`Mx`), a unit of `FluxOfMagneticInduction`: `n` of them is `n · 1e-5` base units.
pub const MAXWELL: Unit<
    {
        UcumUnit::scaled(
            "Mx",
            Exact::dec(1, -5),
            METER_SQUARED_GRAM_PER_SECOND_COULOMB.unit(),
        )
    },
> = Unit::ONE;

// `Force` is `crate::Force`.

/// Dyne (`dyn`), a unit of `Force`: `n` of them is `n · 1e-2` base units.
pub const DYNE: Unit<
    {
        UcumUnit::scaled(
            "dyn",
            Exact::dec(1, -2),
            METER_GRAM_PER_SECOND_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `Illuminance` is `crate::Illuminance`.

/// Phot (`ph`), a unit of `Illuminance`: `n` of them is `n · 1e-4` base units.
pub const PHOT: Unit<
    {
        UcumUnit::scaled(
            "ph",
            Exact::dec(1, -4),
            RADIAN_SQUARED_CANDELA_PER_METER_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// An ion dose, in `N`s of the unit `U`: by default its base unit, `COULOMB_PER_GRAM`.
pub type IonDose<N, const U: UcumUnit = { COULOMB_PER_GRAM.unit() }, const B: Base = BASE> =
    Qty<U, N, B>;

/// The base unit of `IonDose`: a product of base units.
pub const COULOMB_PER_GRAM: Unit<{ (COULOMB / GRAM).unit() }, BASE> = Unit::ONE;

/// Roentgen (`R`), a unit of `IonDose`: `n` of them is `n · 258e-9` base units.
pub const ROENTGEN: Unit<{ UcumUnit::scaled("R", Exact::dec(258, -9), COULOMB_PER_GRAM.unit()) }> =
    Unit::ONE;

/// A kinematic viscosity, in `N`s of the unit `U`: by default its base unit, `METER_SQUARED_PER_SECOND`.
pub type KinematicViscosity<
    N,
    const U: UcumUnit = { METER_SQUARED_PER_SECOND.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `KinematicViscosity`: a product of base units.
pub const METER_SQUARED_PER_SECOND: Unit<{ (METER * METER / SECOND).unit() }, BASE> = Unit::ONE;

/// Stokes (`St`), a unit of `KinematicViscosity`: `n` of them is `n · 1e-4` base units.
pub const STOKES: Unit<
    { UcumUnit::scaled("St", Exact::dec(1, -4), METER_SQUARED_PER_SECOND.unit()) },
> = Unit::ONE;

// `LineicNumber` is `crate::LineicNumber`.

/// Kayser (`Ky`), a unit of `LineicNumber`: `n` of them is `n · 1e2` base units.
pub const KAYSER: Unit<{ UcumUnit::scaled("Ky", Exact::int(100), PER_METER.unit()) }> = Unit::ONE;

/// A lum intensity density, in `N`s of the unit `U`: by default its base unit, `CANDELA_PER_METER_SQUARED`.
pub type LumIntensityDensity<
    N,
    const U: UcumUnit = { CANDELA_PER_METER_SQUARED.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Stilb (`sb`), a unit of `LumIntensityDensity`: `n` of them is `n · 1e4` base units.
pub const STILB: Unit<
    { UcumUnit::scaled("sb", Exact::int(10000), CANDELA_PER_METER_SQUARED.unit()) },
> = Unit::ONE;

/// A magnetic field intensity, in `N`s of the unit `U`: by default its base unit, `COULOMB_PER_METER_SECOND`.
pub type MagneticFieldIntensity<
    N,
    const U: UcumUnit = { COULOMB_PER_METER_SECOND.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// The base unit of `MagneticFieldIntensity`: a product of base units.
pub const COULOMB_PER_METER_SECOND: Unit<{ (COULOMB / (METER * SECOND)).unit() }, BASE> = Unit::ONE;

/// Oersted (`Oe`), a unit of `MagneticFieldIntensity`: `n` of them is `n · 25e1 / core::f64::consts::PI` base units.
pub const OERSTED: Unit<
    {
        UcumUnit::scaled(
            "Oe",
            Exact::int(250) / Exact::PI,
            COULOMB_PER_METER_SECOND.unit(),
        )
    },
> = Unit::ONE;

// `MagneticFluxDensity` is `crate::MagneticFluxDensity`.

/// Gauss (`G`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e-1` base units.
pub const GAUSS: Unit<
    { UcumUnit::scaled("G", Exact::dec(1, -1), GRAM_PER_SECOND_COULOMB.unit()) },
> = Unit::ONE;

/// A magnetic tension, in `N`s of the unit `U`: by default its base unit, `COULOMB_PER_SECOND`.
pub type MagneticTension<
    N,
    const U: UcumUnit = { COULOMB_PER_SECOND.unit() },
    const B: Base = BASE,
> = Qty<U, N, B>;

/// Gilbert (`Gb`), a unit of `MagneticTension`: `n` of them is `n · 25e-1 / core::f64::consts::PI` base units.
pub const GILBERT: Unit<
    {
        UcumUnit::scaled(
            "Gb",
            Exact::dec(25, -1) / Exact::PI,
            COULOMB_PER_SECOND.unit(),
        )
    },
> = Unit::ONE;

// `Radioactivity` is `crate::Radioactivity`.

/// Curie (`Ci`), a unit of `Radioactivity`: `n` of them is `n · 37e9` base units.
pub const CURIE: Unit<{ UcumUnit::scaled("Ci", Exact::int(37000000000), PER_SECOND.unit()) }> =
    Unit::ONE;
