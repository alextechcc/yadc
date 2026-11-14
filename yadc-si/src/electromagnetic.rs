//! The quantities of `electromagnetic` and their units.

#[allow(unused_imports)]
use crate::*;

/// An electric charge, in `N`s of the unit `U`: by default its base unit, `COULOMB`.
pub type ElectricCharge<N, const U: SIUnit = { COULOMB.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Coulombs (`C`), a unit of `ElectricCharge`: the coherent unit, a product of base units.
pub const COULOMB: Unit<{ (SECOND * AMPERE).unit() }, BASE> = Unit::ONE;

/// Ampere hours (`A · h`), a unit of `ElectricCharge`: `n` of them is `n · 3600.0` base units.
pub const AMPERE_HOUR: Unit<{ SIUnit::scaled("A · h", Exact::int(3600), COULOMB.unit()) }> =
    Unit::ONE;

/// Elementary charges (`e`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19` base units.
pub const ELEMENTARY_CHARGE: Unit<
    { SIUnit::scaled("e", Exact::dec(1602176634, -28), COULOMB.unit()) },
> = Unit::ONE;

/// Atomic units of charge (`a.u. of charge`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19` base units.
pub const ATOMIC_UNIT_OF_CHARGE: Unit<
    {
        SIUnit::scaled(
            "a.u. of charge",
            Exact::dec(1602176634, -28),
            COULOMB.unit(),
        )
    },
> = Unit::ONE;

/// Abcoulombs (`abC`), a unit of `ElectricCharge`: `n` of them is `n · 10.0` base units.
pub const ABCOULOMB: Unit<{ SIUnit::scaled("abC", Exact::int(10), COULOMB.unit()) }> = Unit::ONE;

/// Faradays (`F`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19 * 6.02214076e23` base units.
pub const FARADAY: Unit<
    {
        SIUnit::scaled(
            "F",
            Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000),
            COULOMB.unit(),
        )
    },
> = Unit::ONE;

/// Franklins (`Fr`), a unit of `ElectricCharge`: `n` of them is `n · 1.0 / 2997924580.0` base units.
pub const FRANKLIN: Unit<
    { SIUnit::scaled("Fr", Exact::int(1) / Exact::int(2997924580), COULOMB.unit()) },
> = Unit::ONE;

/// Statcoulombs (`statC`), a unit of `ElectricCharge`: `n` of them is `n · 1.0 / 2997924580.0` base units.
pub const STATCOULOMB: Unit<
    {
        SIUnit::scaled(
            "statC",
            Exact::int(1) / Exact::int(2997924580),
            COULOMB.unit(),
        )
    },
> = Unit::ONE;

/// An electric charge areal density, in `N`s of the unit `U`: by default its base unit, `SECOND_AMPERE_PER_METRE_SQUARED`.
pub type ElectricChargeArealDensity<
    N,
    const U: SIUnit = { SECOND_AMPERE_PER_METRE_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ElectricChargeArealDensity`: a product of base units.
pub const SECOND_AMPERE_PER_METRE_SQUARED: Unit<
    { (SECOND * AMPERE / (METRE * METRE)).unit() },
    BASE,
> = Unit::ONE;

/// An electric charge linear density, in `N`s of the unit `U`: by default its base unit, `SECOND_AMPERE_PER_METRE`.
pub type ElectricChargeLinearDensity<
    N,
    const U: SIUnit = { SECOND_AMPERE_PER_METRE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ElectricChargeLinearDensity`: a product of base units.
pub const SECOND_AMPERE_PER_METRE: Unit<{ (SECOND * AMPERE / METRE).unit() }, BASE> = Unit::ONE;

/// An electric charge volumetric density, in `N`s of the unit `U`: by default its base unit, `SECOND_AMPERE_PER_METRE_CUBED`.
pub type ElectricChargeVolumetricDensity<
    N,
    const U: SIUnit = { SECOND_AMPERE_PER_METRE_CUBED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ElectricChargeVolumetricDensity`: a product of base units.
pub const SECOND_AMPERE_PER_METRE_CUBED: Unit<
    { (SECOND * AMPERE / (METRE * METRE * METRE)).unit() },
    BASE,
> = Unit::ONE;

/// An electric current, in `N`s of the unit `U`: by default its base unit, `AMPERE`.
pub type ElectricCurrent<N, const U: SIUnit = { AMPERE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Abamperes (`abA`), a unit of `ElectricCurrent`: `n` of them is `n · 10.0` base units.
pub const ABAMPERE: Unit<{ SIUnit::scaled("abA", Exact::int(10), AMPERE.unit()) }> = Unit::ONE;

/// Gilberts (`Gi`), a unit of `ElectricCurrent`: `n` of them is `n · 10.0 / (4.0 * core::f64::consts::PI)` base units.
pub const GILBERT: Unit<
    {
        SIUnit::scaled(
            "Gi",
            Exact::int(10) / (Exact::int(4) * Exact::PI),
            AMPERE.unit(),
        )
    },
> = Unit::ONE;

/// Statamperes (`statA`), a unit of `ElectricCurrent`: `n` of them is `n · 1.0 / 2997924580.0` base units.
pub const STATAMPERE: Unit<
    {
        SIUnit::scaled(
            "statA",
            Exact::int(1) / Exact::int(2997924580),
            AMPERE.unit(),
        )
    },
> = Unit::ONE;

/// An electric current density, in `N`s of the unit `U`: by default its base unit, `AMPERE_PER_METRE_SQUARED`.
pub type ElectricCurrentDensity<
    N,
    const U: SIUnit = { AMPERE_PER_METRE_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ElectricCurrentDensity`: a product of base units.
pub const AMPERE_PER_METRE_SQUARED: Unit<{ (AMPERE / (METRE * METRE)).unit() }, BASE> = Unit::ONE;

/// A surface electric current density, in `N`s of the unit `U`: by default its base unit, `AMPERE_PER_METRE`.
pub type SurfaceElectricCurrentDensity<
    N,
    const U: SIUnit = { AMPERE_PER_METRE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `SurfaceElectricCurrentDensity`: a product of base units.
pub const AMPERE_PER_METRE: Unit<{ (AMPERE / METRE).unit() }, BASE> = Unit::ONE;

/// An electric potential, in `N`s of the unit `U`: by default its base unit, `VOLT`.
pub type ElectricPotential<N, const U: SIUnit = { VOLT.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Volts (`V`), a unit of `ElectricPotential`: the coherent unit, a product of base units.
pub const VOLT: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)).unit() },
    BASE,
> = Unit::ONE;

/// Abvolts (`abV`), a unit of `ElectricPotential`: `n` of them is `n · 1e-08` base units.
pub const ABVOLT: Unit<{ SIUnit::scaled("abV", Exact::dec(1, -8), VOLT.unit()) }> = Unit::ONE;

/// Statvolts (`statV`), a unit of `ElectricPotential`: `n` of them is `n · 299.792458` base units.
pub const STATVOLT: Unit<{ SIUnit::scaled("statV", Exact::dec(299792458, -6), VOLT.unit()) }> =
    Unit::ONE;

/// An electric field, in `N`s of the unit `U`: by default its base unit, `METRE_KILOGRAM_PER_SECOND_CUBED_AMPERE`.
pub type ElectricField<
    N,
    const U: SIUnit = { METRE_KILOGRAM_PER_SECOND_CUBED_AMPERE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ElectricField`: a product of base units.
pub const METRE_KILOGRAM_PER_SECOND_CUBED_AMPERE: Unit<
    { (METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)).unit() },
    BASE,
> = Unit::ONE;

/// Atomic units of electric field (`a.u. of electric field`), a unit of `ElectricField`: `n` of them is `n · 4.3597447222071e-18 / (1.602176634e-19 * 5.29177210903e-11)` base units.
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD: Unit<
    {
        SIUnit::scaled(
            "a.u. of electric field",
            Exact::dec(43597447222071, -31)
                / (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22)),
            METRE_KILOGRAM_PER_SECOND_CUBED_AMPERE.unit(),
        )
    },
> = Unit::ONE;

/// An electric displacement field, in `N`s of the unit `U`: by default its base unit, `SECOND_AMPERE_PER_METRE_SQUARED`.
pub type ElectricDisplacementField<
    N,
    const U: SIUnit = { SECOND_AMPERE_PER_METRE_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An electric flux, in `N`s of the unit `U`: by default its base unit, `VOLT_METRE`.
pub type ElectricFlux<N, const U: SIUnit = { VOLT_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Volt metres (`V ⋅ m`), a unit of `ElectricFlux`: the coherent unit, a product of base units.
pub const VOLT_METRE: Unit<
    { (METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE)).unit() },
    BASE,
> = Unit::ONE;

/// An electric dipole moment, in `N`s of the unit `U`: by default its base unit, `COULOMB_METRE`.
pub type ElectricDipoleMoment<N, const U: SIUnit = { COULOMB_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Coulomb metres (`C · m`), a unit of `ElectricDipoleMoment`: the coherent unit, a product of base units.
pub const COULOMB_METRE: Unit<{ (METRE * SECOND * AMPERE).unit() }, BASE> = Unit::ONE;

/// Atomic units of electric dipole moment (`e · a₀`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 5.29177210903e-11` base units.
pub const ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT: Unit<
    {
        SIUnit::scaled(
            "e · a₀",
            Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22),
            COULOMB_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Atomic unit of charge centimetres (`a.u. of charge · cm`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 0.01` base units.
pub const ATOMIC_UNIT_OF_CHARGE_CENTIMETRE: Unit<
    {
        SIUnit::scaled(
            "a.u. of charge · cm",
            Exact::dec(1602176634, -28) * Exact::dec(1, -2),
            COULOMB_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Elementary charge centimetres (`e · cm`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 0.01` base units.
pub const ELEMENTARY_CHARGE_CENTIMETRE: Unit<
    {
        SIUnit::scaled(
            "e · cm",
            Exact::dec(1602176634, -28) * Exact::dec(1, -2),
            COULOMB_METRE.unit(),
        )
    },
> = Unit::ONE;

/// An electric quadrupole moment, in `N`s of the unit `U`: by default its base unit, `COULOMB_SQUARE_METRE`.
pub type ElectricQuadrupoleMoment<
    N,
    const U: SIUnit = { COULOMB_SQUARE_METRE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Coulomb square metres (`C · m²`), a unit of `ElectricQuadrupoleMoment`: the coherent unit, a product of base units.
pub const COULOMB_SQUARE_METRE: Unit<{ (METRE * METRE * SECOND * AMPERE).unit() }, BASE> =
    Unit::ONE;

/// Elementary charge barns (`e · b`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.602176634e-19 * 1e-28` base units.
pub const ELEMENTARY_CHARGE_BARN: Unit<
    {
        SIUnit::scaled(
            "e · b",
            Exact::dec(1602176634, -28) * Exact::dec(1, -28),
            COULOMB_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Atomic units of electric quadrupole moment (`e · a₀²`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.602176634e-19 * 5.29177210903e-11 * 5.29177210903e-11` base units.
pub const ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT: Unit<
    {
        SIUnit::scaled(
            "e · a₀²",
            Exact::dec(1602176634, -28)
                * Exact::dec(529177210903, -22)
                * Exact::dec(529177210903, -22),
            COULOMB_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// An electric permittivity, in `N`s of the unit `U`: by default its base unit, `SECOND_POW4_AMPERE_SQUARED_PER_METRE_CUBED_KILOGRAM`.
pub type ElectricPermittivity<
    N,
    const U: SIUnit = { SECOND_POW4_AMPERE_SQUARED_PER_METRE_CUBED_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ElectricPermittivity`: a product of base units.
pub const SECOND_POW4_AMPERE_SQUARED_PER_METRE_CUBED_KILOGRAM: Unit<
    {
        (SECOND * SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM))
            .unit()
    },
    BASE,
> = Unit::ONE;

/// Vacuum electric permittivity (`ε₀`), a unit of `ElectricPermittivity`: `n` of them is `n · 8.8541878128e-12` base units.
pub const VACUUM_PERMITTIVITY: Unit<
    {
        SIUnit::scaled(
            "ε₀",
            Exact::dec(88541878128, -22),
            SECOND_POW4_AMPERE_SQUARED_PER_METRE_CUBED_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// An electrical conductance, in `N`s of the unit `U`: by default its base unit, `SIEMENS`.
pub type ElectricalConductance<N, const U: SIUnit = { SIEMENS.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Siemens (`S`), a unit of `ElectricalConductance`: the coherent unit, a product of base units.
pub const SIEMENS: Unit<
    { (SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * KILOGRAM)).unit() },
    BASE,
> = Unit::ONE;

/// Mhos (`℧`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0` base units.
pub const MHO: Unit<{ SIUnit::scaled("℧", Exact::int(1), SIEMENS.unit()) }> = Unit::ONE;

/// Abmhos (`abmho`), a unit of `ElectricalConductance`: `n` of them is `n · 1000000000.0` base units.
pub const ABMHO: Unit<{ SIUnit::scaled("abmho", Exact::int(1000000000), SIEMENS.unit()) }> =
    Unit::ONE;

/// Abmsiemens (`abS`), a unit of `ElectricalConductance`: `n` of them is `n · 1000000000.0` base units.
pub const ABSIEMENS: Unit<{ SIUnit::scaled("abS", Exact::int(1000000000), SIEMENS.unit()) }> =
    Unit::ONE;

/// Statsiemens (`statS`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` base units.
pub const STATSIEMENS: Unit<
    {
        SIUnit::scaled(
            "statS",
            Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
            SIEMENS.unit(),
        )
    },
> = Unit::ONE;

/// Statmhos (`statmho`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` base units.
pub const STATMHO: Unit<
    {
        SIUnit::scaled(
            "statmho",
            Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
            SIEMENS.unit(),
        )
    },
> = Unit::ONE;

/// An electrical conductivity, in `N`s of the unit `U`: by default its base unit, `SECOND_CUBED_AMPERE_SQUARED_PER_METRE_CUBED_KILOGRAM`.
pub type ElectricalConductivity<
    N,
    const U: SIUnit = { SECOND_CUBED_AMPERE_SQUARED_PER_METRE_CUBED_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ElectricalConductivity`: a product of base units.
pub const SECOND_CUBED_AMPERE_SQUARED_PER_METRE_CUBED_KILOGRAM: Unit<
    { (SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * METRE * KILOGRAM)).unit() },
    BASE,
> = Unit::ONE;

/// An electrical mobility, in `N`s of the unit `U`: by default its base unit, `SECOND_SQUARED_AMPERE_PER_KILOGRAM`.
pub type ElectricalMobility<
    N,
    const U: SIUnit = { SECOND_SQUARED_AMPERE_PER_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ElectricalMobility`: a product of base units.
pub const SECOND_SQUARED_AMPERE_PER_KILOGRAM: Unit<
    { (SECOND * SECOND * AMPERE / KILOGRAM).unit() },
    BASE,
> = Unit::ONE;

/// An electrical resistance, in `N`s of the unit `U`: by default its base unit, `OHM`.
pub type ElectricalResistance<N, const U: SIUnit = { OHM.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Ohms (`Ω`), a unit of `ElectricalResistance`: the coherent unit, a product of base units.
pub const OHM: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)).unit() },
    BASE,
> = Unit::ONE;

/// Abohms (`abΩ`), a unit of `ElectricalResistance`: `n` of them is `n · 1e-09` base units.
pub const ABOHM: Unit<{ SIUnit::scaled("abΩ", Exact::dec(1, -9), OHM.unit()) }> = Unit::ONE;

/// Statohms (`statΩ`), a unit of `ElectricalResistance`: `n` of them is `n · 299.792458 * 2997924580.0` base units.
pub const STATOHM: Unit<
    {
        SIUnit::scaled(
            "statΩ",
            Exact::dec(299792458, -6) * Exact::int(2997924580),
            OHM.unit(),
        )
    },
> = Unit::ONE;

/// An electrical resistivity, in `N`s of the unit `U`: by default its base unit, `OHM_METRE`.
pub type ElectricalResistivity<N, const U: SIUnit = { OHM_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Ohm metres (`Ω · m`), a unit of `ElectricalResistivity`: the coherent unit, a product of base units.
pub const OHM_METRE: Unit<
    { (METRE * METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND * AMPERE * AMPERE)).unit() },
    BASE,
> = Unit::ONE;

/// Abohm metres (`abΩ · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 1e-09` base units.
pub const ABOHM_METRE: Unit<{ SIUnit::scaled("abΩ · m", Exact::dec(1, -9), OHM_METRE.unit()) }> =
    Unit::ONE;

/// Statohm metres (`statΩ · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 299.792458 * 2997924580.0` base units.
pub const STATOHM_METRE: Unit<
    {
        SIUnit::scaled(
            "statΩ · m",
            Exact::dec(299792458, -6) * Exact::int(2997924580),
            OHM_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Ohm inches (`Ω · in`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.0254` base units.
pub const OHM_INCH: Unit<{ SIUnit::scaled("Ω · in", Exact::dec(254, -4), OHM_METRE.unit()) }> =
    Unit::ONE;

/// Ohm feet (`Ω · ft`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.3048` base units.
pub const OHM_FOOT: Unit<{ SIUnit::scaled("Ω · ft", Exact::dec(3048, -4), OHM_METRE.unit()) }> =
    Unit::ONE;

/// Ohm yards (`Ω · yd`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.9144` base units.
pub const OHM_YARD: Unit<{ SIUnit::scaled("Ω · yd", Exact::dec(9144, -4), OHM_METRE.unit()) }> =
    Unit::ONE;

/// A capacitance, in `N`s of the unit `U`: by default its base unit, `FARAD`.
pub type Capacitance<N, const U: SIUnit = { FARAD.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Farads (`F`), a unit of `Capacitance`: the coherent unit, a product of base units.
pub const FARAD: Unit<
    { (SECOND * SECOND * SECOND * SECOND * AMPERE * AMPERE / (METRE * METRE * KILOGRAM)).unit() },
    BASE,
> = Unit::ONE;

/// Abfarads (`abF`), a unit of `Capacitance`: `n` of them is `n · 1000000000.0` base units.
pub const ABFARAD: Unit<{ SIUnit::scaled("abF", Exact::int(1000000000), FARAD.unit()) }> =
    Unit::ONE;

/// Statfarads (`statF`), a unit of `Capacitance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` base units.
pub const STATFARAD: Unit<
    {
        SIUnit::scaled(
            "statF",
            Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
            FARAD.unit(),
        )
    },
> = Unit::ONE;

/// An inductance, in `N`s of the unit `U`: by default its base unit, `HENRY`.
pub type Inductance<N, const U: SIUnit = { HENRY.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Henries (`H`), a unit of `Inductance`: the coherent unit, a product of base units.
pub const HENRY: Unit<
    { (METRE * METRE * KILOGRAM / (SECOND * SECOND * AMPERE * AMPERE)).unit() },
    BASE,
> = Unit::ONE;

/// Abhenries (`abH`), a unit of `Inductance`: `n` of them is `n · 1e-09` base units.
pub const ABHENRY: Unit<{ SIUnit::scaled("abH", Exact::dec(1, -9), HENRY.unit()) }> = Unit::ONE;

/// Stathenries (`statH`), a unit of `Inductance`: `n` of them is `n · 299.792458 * 2997924580.0` base units.
pub const STATHENRY: Unit<
    {
        SIUnit::scaled(
            "statH",
            Exact::dec(299792458, -6) * Exact::int(2997924580),
            HENRY.unit(),
        )
    },
> = Unit::ONE;

/// A magnetic field strength, in `N`s of the unit `U`: by default its base unit, `AMPERE_PER_METRE`.
pub type MagneticFieldStrength<
    N,
    const U: SIUnit = { AMPERE_PER_METRE.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Oersteds (`Oe`), a unit of `MagneticFieldStrength`: `n` of them is `n · 1000.0 / (4.0 * core::f64::consts::PI)` base units.
pub const OERSTED: Unit<
    {
        SIUnit::scaled(
            "Oe",
            Exact::int(1000) / (Exact::int(4) * Exact::PI),
            AMPERE_PER_METRE.unit(),
        )
    },
> = Unit::ONE;

/// A magnetic flux, in `N`s of the unit `U`: by default its base unit, `WEBER`.
pub type MagneticFlux<N, const U: SIUnit = { WEBER.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Webers (`Wb`), a unit of `MagneticFlux`: the coherent unit, a product of base units.
pub const WEBER: Unit<{ (METRE * METRE * KILOGRAM / (SECOND * SECOND * AMPERE)).unit() }, BASE> =
    Unit::ONE;

/// Maxwells (`Mx`), a unit of `MagneticFlux`: `n` of them is `n · 1e-08` base units.
pub const MAXWELL: Unit<{ SIUnit::scaled("Mx", Exact::dec(1, -8), WEBER.unit()) }> = Unit::ONE;

/// A magnetic flux density, in `N`s of the unit `U`: by default its base unit, `TESLA`.
pub type MagneticFluxDensity<N, const U: SIUnit = { TESLA.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Teslas (`T`), a unit of `MagneticFluxDensity`: the coherent unit, a product of base units.
pub const TESLA: Unit<{ (KILOGRAM / (SECOND * SECOND * AMPERE)).unit() }, BASE> = Unit::ONE;

/// Gammas (`γ`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e-09` base units.
pub const GAMMA: Unit<{ SIUnit::scaled("γ", Exact::dec(1, -9), TESLA.unit()) }> = Unit::ONE;

/// Gauss (`G`), a unit of `MagneticFluxDensity`: `n` of them is `n · 0.0001` base units.
pub const GAUSS: Unit<{ SIUnit::scaled("G", Exact::dec(1, -4), TESLA.unit()) }> = Unit::ONE;

/// A magnetic moment, in `N`s of the unit `U`: by default its base unit, `AMPERE_SQUARE_METRE`.
pub type MagneticMoment<N, const U: SIUnit = { AMPERE_SQUARE_METRE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Ampere square metres (`A · m²`), a unit of `MagneticMoment`: the coherent unit, a product of base units.
pub const AMPERE_SQUARE_METRE: Unit<{ (METRE * METRE * AMPERE).unit() }, BASE> = Unit::ONE;

/// Bohr magnetons (`µ(Bohr)`), a unit of `MagneticMoment`: `n` of them is `n · 9.2740100783e-24` base units.
pub const BOHR_MAGNETON: Unit<
    {
        SIUnit::scaled(
            "µ(Bohr)",
            Exact::dec(92740100783, -34),
            AMPERE_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Nuclear magnetons (`μ(Nuclear)`), a unit of `MagneticMoment`: `n` of them is `n · 5.0507837461e-27` base units.
pub const NUCLEAR_MAGNETON: Unit<
    {
        SIUnit::scaled(
            "μ(Nuclear)",
            Exact::dec(50507837461, -37),
            AMPERE_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

///  atomic units of magnetic dipole moment (`ħ · e/mₑ`), a unit of `MagneticMoment`: `n` of them is `n · 1.85480201566e-23` base units.
pub const ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT: Unit<
    {
        SIUnit::scaled(
            "ħ · e/mₑ",
            Exact::dec(185480201566, -34),
            AMPERE_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Deuteron magnetic moments (`μ(deuteron)`), a unit of `MagneticMoment`: `n` of them is `n · 4.330735094e-27` base units.
pub const DEUTERON_MAGNETIC_MOMENT: Unit<
    {
        SIUnit::scaled(
            "μ(deuteron)",
            Exact::dec(4330735094, -36),
            AMPERE_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Proton magnetic moments (`μ(proton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.41060679736e-26` base units.
pub const PROTON_MAGNETIC_MOMENT: Unit<
    {
        SIUnit::scaled(
            "μ(proton)",
            Exact::dec(141060679736, -37),
            AMPERE_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Shielded proton magnetic moments (`μ’(proton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.41057056e-26` base units.
pub const SHIELDED_PROTON_MAGNETIC_MOMENT: Unit<
    {
        SIUnit::scaled(
            "μ’(proton)",
            Exact::dec(141057056, -34),
            AMPERE_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// Triton magnetic moments (`μ(triton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.5046095202e-26` base units.
pub const TRITON_MAGNETIC_MOMENT: Unit<
    {
        SIUnit::scaled(
            "μ(triton)",
            Exact::dec(15046095202, -36),
            AMPERE_SQUARE_METRE.unit(),
        )
    },
> = Unit::ONE;

/// A magnetic permeability, in `N`s of the unit `U`: by default its base unit, `METRE_KILOGRAM_PER_SECOND_SQUARED_AMPERE_SQUARED`.
pub type MagneticPermeability<
    N,
    const U: SIUnit = { METRE_KILOGRAM_PER_SECOND_SQUARED_AMPERE_SQUARED.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `MagneticPermeability`: a product of base units.
pub const METRE_KILOGRAM_PER_SECOND_SQUARED_AMPERE_SQUARED: Unit<
    { (METRE * KILOGRAM / (SECOND * SECOND * AMPERE * AMPERE)).unit() },
    BASE,
> = Unit::ONE;

/// Vacuum magnetic permeability (`µ₀`), a unit of `MagneticPermeability`: `n` of them is `n · 1.25663706212e-6` base units.
pub const VACUUM_PERMEABILITY: Unit<
    {
        SIUnit::scaled(
            "µ₀",
            Exact::dec(125663706212, -17),
            METRE_KILOGRAM_PER_SECOND_SQUARED_AMPERE_SQUARED.unit(),
        )
    },
> = Unit::ONE;

/// A gyromagnetic ratio, in `N`s of the unit `U`: by default its base unit, `SECOND_AMPERE_RADIAN_PER_KILOGRAM`.
pub type GyromagneticRatio<
    N,
    const U: SIUnit = { SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `GyromagneticRatio`: a product of base units.
pub const SECOND_AMPERE_RADIAN_PER_KILOGRAM: Unit<
    { (SECOND * AMPERE * RADIAN / KILOGRAM).unit() },
    BASE,
> = Unit::ONE;

/// Proton gyromagnetic ratio (`γₚ`), a unit of `GyromagneticRatio`: `n` of them is `n · 267522187.08` base units.
pub const PROTON_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γₚ",
            Exact::dec(26752218708, -2),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Neutron gyromagnetic ratio (`γₙ`), a unit of `GyromagneticRatio`: `n` of them is `n · 183247174.0` base units.
pub const NEUTRON_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γₙ",
            Exact::int(183247174),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Electron gyromagnetic ratio (`γₑ`), a unit of `GyromagneticRatio`: `n` of them is `n · 176085962784.0` base units.
pub const ELECTRON_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γₑ",
            Exact::int(176085962784),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Shielded proton gyromagnetic ratio (`γ'ₚ`), a unit of `GyromagneticRatio`: `n` of them is `n · 267515319.4` base units.
pub const SHIELDED_PROTON_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ'ₚ",
            Exact::dec(2675153194, -1),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Shielded helion gyromagnetic ratio (`γ'ₕ`), a unit of `GyromagneticRatio`: `n` of them is `n · 203789460.78` base units.
pub const SHIELDED_HELION_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ'ₕ",
            Exact::dec(20378946078, -2),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Deuteron gyromagnetic ratio (`γ₂`), a unit of `GyromagneticRatio`: `n` of them is `n · 41066288.8` base units.
pub const DEUTERON_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₂",
            Exact::dec(410662888, -1),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Triton gyromagnetic ratio (`γ₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 285349844.0` base units.
pub const TRITON_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₃",
            Exact::int(285349844),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Lithium7 gyromagnetic ratio (`γ₇`), a unit of `GyromagneticRatio`: `n` of them is `n · 103975264.0` base units.
pub const LITHIUM7_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₇",
            Exact::int(103975264),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Boron11 gyromagnetic ratio (`γ₁₁`), a unit of `GyromagneticRatio`: `n` of them is `n · 85838414.3` base units.
pub const BORON11_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₁₁",
            Exact::dec(858384143, -1),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Carbon13 gyromagnetic ratio (`γ₁₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 67278754.6` base units.
pub const CARBON13_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₁₃",
            Exact::dec(672787546, -1),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Nitrogen14 gyromagnetic ratio (`γ₁₄`), a unit of `GyromagneticRatio`: `n` of them is `n · 19328792.2` base units.
pub const NITROGEN14_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₁₄",
            Exact::dec(193287922, -1),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Flourine19 gyromagnetic ratio (`γ₁₉`), a unit of `GyromagneticRatio`: `n` of them is `n · 251762483.0` base units.
pub const FLOURINE19_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₁₉",
            Exact::int(251762483),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Sodium23 gyromagnetic ratio (`γ₂₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 70803541.7` base units.
pub const SODIUM23_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₂₃",
            Exact::dec(708035417, -1),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Phosphorus31 gyromagnetic ratio (`γ₃₁`), a unit of `GyromagneticRatio`: `n` of them is `n · 108329419.0` base units.
pub const PHOSPHORUS31_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ₃₁",
            Exact::int(108329419),
            SECOND_AMPERE_RADIAN_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// A reduced gyromagnetic ratio, in `N`s of the unit `U`: by default its base unit, `SECOND_AMPERE_PER_KILOGRAM`.
pub type ReducedGyromagneticRatio<
    N,
    const U: SIUnit = { SECOND_AMPERE_PER_KILOGRAM.unit() },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// The base unit of `ReducedGyromagneticRatio`: a product of base units.
pub const SECOND_AMPERE_PER_KILOGRAM: Unit<{ (SECOND * AMPERE / KILOGRAM).unit() }, BASE> =
    Unit::ONE;

/// Proton reduced gyromagnetic ratio (`̅γₚ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 42577478.461` base units.
pub const PROTON_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γₚ",
            Exact::dec(42577478461, -3),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Neutron reduced gyromagnetic ratio (`̅γₙ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 29164693.5` base units.
pub const NEUTRON_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γₙ",
            Exact::dec(291646935, -1),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Electron reduced gyromagnetic ratio (`̅γₑ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 28024951386.1` base units.
pub const ELECTRON_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γₑ",
            Exact::dec(280249513861, -1),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Shielded proton reduced gyromagnetic ratio (`γ'ₚ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 42576385.43` base units.
pub const SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ'ₚ",
            Exact::dec(4257638543, -2),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Shielded helion reduced gyromagnetic ratio (`γ'ₕ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 32434100.033` base units.
pub const SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "γ'ₕ",
            Exact::dec(32434100033, -3),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Deuteron reduced gyromagnetic ratio (`̅γ₂`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 6535902.85` base units.
pub const DEUTERON_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₂",
            Exact::dec(653590285, -2),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Triton reduced gyromagnetic ratio (`̅γ₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 45414838.1` base units.
pub const TRITON_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₃",
            Exact::dec(454148381, -1),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Lithium7 reduced gyromagnetic ratio (`̅γ₇`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 16548177.3` base units.
pub const LITHIUM7_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₇",
            Exact::dec(165481773, -1),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Boron11 reduced gyromagnetic ratio (`̅γ₁₁`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 13661608.0` base units.
pub const BORON11_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₁₁",
            Exact::int(13661608),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Carbon13 reduced gyromagnetic ratio (`̅γ₁₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 10707746.4` base units.
pub const CARBON13_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₁₃",
            Exact::dec(107077464, -1),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Nitrogen14 reduced gyromagnetic ratio (`̅γ₁₄`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 3076272.81` base units.
pub const NITROGEN14_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₁₄",
            Exact::dec(307627281, -2),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Flourine19 reduced gyromagnetic ratio (`̅γ₁₉`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 40069243.7` base units.
pub const FLOURINE19_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₁₉",
            Exact::dec(400692437, -1),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Sodium23 reduced gyromagnetic ratio (`̅γ₂₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 11268733.6` base units.
pub const SODIUM23_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₂₃",
            Exact::dec(112687336, -1),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;

/// Phosphorus31 reduced gyromagnetic ratio (`̅γ₃₁`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 17241162.5` base units.
pub const PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO: Unit<
    {
        SIUnit::scaled(
            "̅γ₃₁",
            Exact::dec(172411625, -1),
            SECOND_AMPERE_PER_KILOGRAM.unit(),
        )
    },
> = Unit::ONE;
