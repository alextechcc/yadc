//! The quantities of `electromagnetic` and their units.

#[allow(unused_imports)]
use crate::*;

/// An electric charge, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricCharge<
    N,
    const U: SIUnit = { SECOND.unit().times(AMPERE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Coulombs (`C`), a unit of `ElectricCharge`: the coherent unit, a product of base units.
pub const COULOMB: SIQuantity<{ SECOND.unit().times(AMPERE.unit(), 1) }, Exact, BASE> =
    SIQuantity::ONE;

/// Ampere hours (`A · h`), a unit of `ElectricCharge`: `n` of them is `n · 3600.0` base units.
pub const AMPERE_HOUR: SIQuantity<
    {
        SIUnit::scaled(
            "A · h",
            Exact::int(3600),
            SECOND.unit().times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Elementary charges (`e`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19` base units.
pub const ELEMENTARY_CHARGE: SIQuantity<
    {
        SIUnit::scaled(
            "e",
            Exact::dec(1602176634, -28),
            SECOND.unit().times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Atomic units of charge (`a.u. of charge`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19` base units.
pub const ATOMIC_UNIT_OF_CHARGE: SIQuantity<
    {
        SIUnit::scaled(
            "a.u. of charge",
            Exact::dec(1602176634, -28),
            SECOND.unit().times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Abcoulombs (`abC`), a unit of `ElectricCharge`: `n` of them is `n · 10.0` base units.
pub const ABCOULOMB: SIQuantity<
    { SIUnit::scaled("abC", Exact::int(10), SECOND.unit().times(AMPERE.unit(), 1)) },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Faradays (`F`), a unit of `ElectricCharge`: `n` of them is `n · 1.602176634e-19 * 6.02214076e23` base units.
pub const FARADAY: SIQuantity<
    {
        SIUnit::scaled(
            "F",
            Exact::dec(1602176634, -28) * Exact::int(602214076000000000000000),
            SECOND.unit().times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Franklins (`Fr`), a unit of `ElectricCharge`: `n` of them is `n · 1.0 / 2997924580.0` base units.
pub const FRANKLIN: SIQuantity<
    {
        SIUnit::scaled(
            "Fr",
            Exact::int(1) / Exact::int(2997924580),
            SECOND.unit().times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Statcoulombs (`statC`), a unit of `ElectricCharge`: `n` of them is `n · 1.0 / 2997924580.0` base units.
pub const STATCOULOMB: SIQuantity<
    {
        SIUnit::scaled(
            "statC",
            Exact::int(1) / Exact::int(2997924580),
            SECOND.unit().times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electric charge areal density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricChargeArealDensity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(SECOND.unit(), 1)
            .times(AMPERE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An electric charge linear density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricChargeLinearDensity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -1)
            .times(SECOND.unit(), 1)
            .times(AMPERE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An electric charge volumetric density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricChargeVolumetricDensity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -3)
            .times(SECOND.unit(), 1)
            .times(AMPERE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An electric current, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricCurrent<N, const U: SIUnit = { AMPERE.unit() }, const B: Base = BASE> =
    SIQuantity<U, N, B>;

/// Abamperes (`abA`), a unit of `ElectricCurrent`: `n` of them is `n · 10.0` base units.
pub const ABAMPERE: SIQuantity<
    { SIUnit::scaled("abA", Exact::int(10), AMPERE.unit()) },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Gilberts (`Gi`), a unit of `ElectricCurrent`: `n` of them is `n · 10.0 / (4.0 * core::f64::consts::PI)` base units.
pub const GILBERT: SIQuantity<
    {
        SIUnit::scaled(
            "Gi",
            Exact::int(10) / (Exact::int(4) * Exact::PI),
            AMPERE.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Statamperes (`statA`), a unit of `ElectricCurrent`: `n` of them is `n · 1.0 / 2997924580.0` base units.
pub const STATAMPERE: SIQuantity<
    {
        SIUnit::scaled(
            "statA",
            Exact::int(1) / Exact::int(2997924580),
            AMPERE.unit(),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electric current density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricCurrentDensity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -2).times(AMPERE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// A surface electric current density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type SurfaceElectricCurrentDensity<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -1).times(AMPERE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An electric potential, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricPotential<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Volts (`V`), a unit of `ElectricPotential`: the coherent unit, a product of base units.
pub const VOLT: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -1)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Abvolts (`abV`), a unit of `ElectricPotential`: `n` of them is `n · 1e-08` base units.
pub const ABVOLT: SIQuantity<
    {
        SIUnit::scaled(
            "abV",
            Exact::dec(1, -8),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Statvolts (`statV`), a unit of `ElectricPotential`: `n` of them is `n · 299.792458` base units.
pub const STATVOLT: SIQuantity<
    {
        SIUnit::scaled(
            "statV",
            Exact::dec(299792458, -6),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electric field, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricField<
    N,
    const U: SIUnit = {
        METRE
            .unit()
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Atomic units of electric field (`a.u. of electric field`), a unit of `ElectricField`: `n` of them is `n · 4.3597447222071e-18 / (1.602176634e-19 * 5.29177210903e-11)` base units.
pub const ATOMIC_UNIT_OF_ELECTRIC_FIELD: SIQuantity<
    {
        SIUnit::scaled(
            "a.u. of electric field",
            Exact::dec(43597447222071, -31)
                / (Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22)),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electric displacement field, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricDisplacementField<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(SECOND.unit(), 1)
            .times(AMPERE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An electric flux, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricFlux<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 3)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Volt metres (`V ⋅ m`), a unit of `ElectricFlux`: the coherent unit, a product of base units.
pub const VOLT_METRE: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 3)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -1)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// An electric dipole moment, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricDipoleMoment<
    N,
    const U: SIUnit = { METRE.unit().times(SECOND.unit(), 1).times(AMPERE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Coulomb metres (`C · m`), a unit of `ElectricDipoleMoment`: the coherent unit, a product of base units.
pub const COULOMB_METRE: SIQuantity<
    { METRE.unit().times(SECOND.unit(), 1).times(AMPERE.unit(), 1) },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Atomic units of electric dipole moment (`e · a₀`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 5.29177210903e-11` base units.
pub const ATOMIC_UNIT_OF_ELECTRIC_DIPOLE_MOMENT: SIQuantity<
    {
        SIUnit::scaled(
            "e · a₀",
            Exact::dec(1602176634, -28) * Exact::dec(529177210903, -22),
            METRE.unit().times(SECOND.unit(), 1).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Atomic unit of charge centimetres (`a.u. of charge · cm`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 0.01` base units.
pub const ATOMIC_UNIT_OF_CHARGE_CENTIMETRE: SIQuantity<
    {
        SIUnit::scaled(
            "a.u. of charge · cm",
            Exact::dec(1602176634, -28) * Exact::dec(1, -2),
            METRE.unit().times(SECOND.unit(), 1).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Elementary charge centimetres (`e · cm`), a unit of `ElectricDipoleMoment`: `n` of them is `n · 1.602176634e-19 * 0.01` base units.
pub const ELEMENTARY_CHARGE_CENTIMETRE: SIQuantity<
    {
        SIUnit::scaled(
            "e · cm",
            Exact::dec(1602176634, -28) * Exact::dec(1, -2),
            METRE.unit().times(SECOND.unit(), 1).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electric quadrupole moment, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricQuadrupoleMoment<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(SECOND.unit(), 1)
            .times(AMPERE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Coulomb square metres (`C · m²`), a unit of `ElectricQuadrupoleMoment`: the coherent unit, a product of base units.
pub const COULOMB_SQUARE_METRE: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(SECOND.unit(), 1)
            .times(AMPERE.unit(), 1)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Elementary charge barns (`e · b`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.602176634e-19 * 1e-28` base units.
pub const ELEMENTARY_CHARGE_BARN: SIQuantity<
    {
        SIUnit::scaled(
            "e · b",
            Exact::dec(1602176634, -28) * Exact::dec(1, -28),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Atomic units of electric quadrupole moment (`e · a₀²`), a unit of `ElectricQuadrupoleMoment`: `n` of them is `n · 1.602176634e-19 * 5.29177210903e-11 * 5.29177210903e-11` base units.
pub const ATOMIC_UNIT_OF_ELECTRIC_QUADRUPOLE_MOMENT: SIQuantity<
    {
        SIUnit::scaled(
            "e · a₀²",
            Exact::dec(1602176634, -28)
                * Exact::dec(529177210903, -22)
                * Exact::dec(529177210903, -22),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electric permittivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricPermittivity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -3)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 4)
            .times(AMPERE.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Vacuum electric permittivity (`ε₀`), a unit of `ElectricPermittivity`: `n` of them is `n · 8.8541878128e-12` base units.
pub const VACUUM_PERMITTIVITY: SIQuantity<
    {
        SIUnit::scaled(
            "ε₀",
            Exact::dec(88541878128, -22),
            SIUnit::ONE
                .times(METRE.unit(), -3)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 4)
                .times(AMPERE.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electrical conductance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricalConductance<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 3)
            .times(AMPERE.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Siemens (`S`), a unit of `ElectricalConductance`: the coherent unit, a product of base units.
pub const SIEMENS: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 3)
            .times(AMPERE.unit(), 2)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Mhos (`℧`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0` base units.
pub const MHO: SIQuantity<
    {
        SIUnit::scaled(
            "℧",
            Exact::int(1),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 3)
                .times(AMPERE.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Abmhos (`abmho`), a unit of `ElectricalConductance`: `n` of them is `n · 1000000000.0` base units.
pub const ABMHO: SIQuantity<
    {
        SIUnit::scaled(
            "abmho",
            Exact::int(1000000000),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 3)
                .times(AMPERE.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Abmsiemens (`abS`), a unit of `ElectricalConductance`: `n` of them is `n · 1000000000.0` base units.
pub const ABSIEMENS: SIQuantity<
    {
        SIUnit::scaled(
            "abS",
            Exact::int(1000000000),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 3)
                .times(AMPERE.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Statsiemens (`statS`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` base units.
pub const STATSIEMENS: SIQuantity<
    {
        SIUnit::scaled(
            "statS",
            Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 3)
                .times(AMPERE.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Statmhos (`statmho`), a unit of `ElectricalConductance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` base units.
pub const STATMHO: SIQuantity<
    {
        SIUnit::scaled(
            "statmho",
            Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 3)
                .times(AMPERE.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electrical conductivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricalConductivity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -3)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 3)
            .times(AMPERE.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An electrical mobility, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricalMobility<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 2)
            .times(AMPERE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// An electrical resistance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricalResistance<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Ohms (`Ω`), a unit of `ElectricalResistance`: the coherent unit, a product of base units.
pub const OHM: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -2)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Abohms (`abΩ`), a unit of `ElectricalResistance`: `n` of them is `n · 1e-09` base units.
pub const ABOHM: SIQuantity<
    {
        SIUnit::scaled(
            "abΩ",
            Exact::dec(1, -9),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Statohms (`statΩ`), a unit of `ElectricalResistance`: `n` of them is `n · 299.792458 * 2997924580.0` base units.
pub const STATOHM: SIQuantity<
    {
        SIUnit::scaled(
            "statΩ",
            Exact::dec(299792458, -6) * Exact::int(2997924580),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An electrical resistivity, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ElectricalResistivity<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 3)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Ohm metres (`Ω · m`), a unit of `ElectricalResistivity`: the coherent unit, a product of base units.
pub const OHM_METRE: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 3)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -3)
            .times(AMPERE.unit(), -2)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Abohm metres (`abΩ · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 1e-09` base units.
pub const ABOHM_METRE: SIQuantity<
    {
        SIUnit::scaled(
            "abΩ · m",
            Exact::dec(1, -9),
            SIUnit::ONE
                .times(METRE.unit(), 3)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Statohm metres (`statΩ · m`), a unit of `ElectricalResistivity`: `n` of them is `n · 299.792458 * 2997924580.0` base units.
pub const STATOHM_METRE: SIQuantity<
    {
        SIUnit::scaled(
            "statΩ · m",
            Exact::dec(299792458, -6) * Exact::int(2997924580),
            SIUnit::ONE
                .times(METRE.unit(), 3)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Ohm inches (`Ω · in`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.0254` base units.
pub const OHM_INCH: SIQuantity<
    {
        SIUnit::scaled(
            "Ω · in",
            Exact::dec(254, -4),
            SIUnit::ONE
                .times(METRE.unit(), 3)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Ohm feet (`Ω · ft`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.3048` base units.
pub const OHM_FOOT: SIQuantity<
    {
        SIUnit::scaled(
            "Ω · ft",
            Exact::dec(3048, -4),
            SIUnit::ONE
                .times(METRE.unit(), 3)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Ohm yards (`Ω · yd`), a unit of `ElectricalResistivity`: `n` of them is `n · 0.9144` base units.
pub const OHM_YARD: SIQuantity<
    {
        SIUnit::scaled(
            "Ω · yd",
            Exact::dec(9144, -4),
            SIUnit::ONE
                .times(METRE.unit(), 3)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -3)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A capacitance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Capacitance<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 4)
            .times(AMPERE.unit(), 2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Farads (`F`), a unit of `Capacitance`: the coherent unit, a product of base units.
pub const FARAD: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), -2)
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 4)
            .times(AMPERE.unit(), 2)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Abfarads (`abF`), a unit of `Capacitance`: `n` of them is `n · 1000000000.0` base units.
pub const ABFARAD: SIQuantity<
    {
        SIUnit::scaled(
            "abF",
            Exact::int(1000000000),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 4)
                .times(AMPERE.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Statfarads (`statF`), a unit of `Capacitance`: `n` of them is `n · 1.0 / (299.792458 * 2997924580.0)` base units.
pub const STATFARAD: SIQuantity<
    {
        SIUnit::scaled(
            "statF",
            Exact::int(1) / (Exact::dec(299792458, -6) * Exact::int(2997924580)),
            SIUnit::ONE
                .times(METRE.unit(), -2)
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 4)
                .times(AMPERE.unit(), 2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// An inductance, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type Inductance<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(AMPERE.unit(), -2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Henries (`H`), a unit of `Inductance`: the coherent unit, a product of base units.
pub const HENRY: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(AMPERE.unit(), -2)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Abhenries (`abH`), a unit of `Inductance`: `n` of them is `n · 1e-09` base units.
pub const ABHENRY: SIQuantity<
    {
        SIUnit::scaled(
            "abH",
            Exact::dec(1, -9),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Stathenries (`statH`), a unit of `Inductance`: `n` of them is `n · 299.792458 * 2997924580.0` base units.
pub const STATHENRY: SIQuantity<
    {
        SIUnit::scaled(
            "statH",
            Exact::dec(299792458, -6) * Exact::int(2997924580),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A magnetic field strength, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticFieldStrength<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), -1).times(AMPERE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Oersteds (`Oe`), a unit of `MagneticFieldStrength`: `n` of them is `n · 1000.0 / (4.0 * core::f64::consts::PI)` base units.
pub const OERSTED: SIQuantity<
    {
        SIUnit::scaled(
            "Oe",
            Exact::int(1000) / (Exact::int(4) * Exact::PI),
            SIUnit::ONE.times(METRE.unit(), -1).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A magnetic flux, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticFlux<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(AMPERE.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Webers (`Wb`), a unit of `MagneticFlux`: the coherent unit, a product of base units.
pub const WEBER: SIQuantity<
    {
        SIUnit::ONE
            .times(METRE.unit(), 2)
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(AMPERE.unit(), -1)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Maxwells (`Mx`), a unit of `MagneticFlux`: `n` of them is `n · 1e-08` base units.
pub const MAXWELL: SIQuantity<
    {
        SIUnit::scaled(
            "Mx",
            Exact::dec(1, -8),
            SIUnit::ONE
                .times(METRE.unit(), 2)
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(AMPERE.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A magnetic flux density, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticFluxDensity<
    N,
    const U: SIUnit = {
        KILOGRAM
            .unit()
            .times(SECOND.unit(), -2)
            .times(AMPERE.unit(), -1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Teslas (`T`), a unit of `MagneticFluxDensity`: the coherent unit, a product of base units.
pub const TESLA: SIQuantity<
    {
        KILOGRAM
            .unit()
            .times(SECOND.unit(), -2)
            .times(AMPERE.unit(), -1)
    },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Gammas (`γ`), a unit of `MagneticFluxDensity`: `n` of them is `n · 1e-09` base units.
pub const GAMMA: SIQuantity<
    {
        SIUnit::scaled(
            "γ",
            Exact::dec(1, -9),
            KILOGRAM
                .unit()
                .times(SECOND.unit(), -2)
                .times(AMPERE.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Gauss (`G`), a unit of `MagneticFluxDensity`: `n` of them is `n · 0.0001` base units.
pub const GAUSS: SIQuantity<
    {
        SIUnit::scaled(
            "G",
            Exact::dec(1, -4),
            KILOGRAM
                .unit()
                .times(SECOND.unit(), -2)
                .times(AMPERE.unit(), -1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A magnetic moment, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticMoment<
    N,
    const U: SIUnit = { SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1) },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Ampere square metres (`A · m²`), a unit of `MagneticMoment`: the coherent unit, a product of base units.
pub const AMPERE_SQUARE_METRE: SIQuantity<
    { SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1) },
    Exact,
    BASE,
> = SIQuantity::ONE;

/// Bohr magnetons (`µ(Bohr)`), a unit of `MagneticMoment`: `n` of them is `n · 9.2740100783e-24` base units.
pub const BOHR_MAGNETON: SIQuantity<
    {
        SIUnit::scaled(
            "µ(Bohr)",
            Exact::dec(92740100783, -34),
            SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Nuclear magnetons (`μ(Nuclear)`), a unit of `MagneticMoment`: `n` of them is `n · 5.0507837461e-27` base units.
pub const NUCLEAR_MAGNETON: SIQuantity<
    {
        SIUnit::scaled(
            "μ(Nuclear)",
            Exact::dec(50507837461, -37),
            SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

///  atomic units of magnetic dipole moment (`ħ · e/mₑ`), a unit of `MagneticMoment`: `n` of them is `n · 1.85480201566e-23` base units.
pub const ATOMIC_UNIT_OF_MAGNETIC_DIPOLE_MOMENT: SIQuantity<
    {
        SIUnit::scaled(
            "ħ · e/mₑ",
            Exact::dec(185480201566, -34),
            SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Deuteron magnetic moments (`μ(deuteron)`), a unit of `MagneticMoment`: `n` of them is `n · 4.330735094e-27` base units.
pub const DEUTERON_MAGNETIC_MOMENT: SIQuantity<
    {
        SIUnit::scaled(
            "μ(deuteron)",
            Exact::dec(4330735094, -36),
            SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Proton magnetic moments (`μ(proton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.41060679736e-26` base units.
pub const PROTON_MAGNETIC_MOMENT: SIQuantity<
    {
        SIUnit::scaled(
            "μ(proton)",
            Exact::dec(141060679736, -37),
            SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Shielded proton magnetic moments (`μ’(proton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.41057056e-26` base units.
pub const SHIELDED_PROTON_MAGNETIC_MOMENT: SIQuantity<
    {
        SIUnit::scaled(
            "μ’(proton)",
            Exact::dec(141057056, -34),
            SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Triton magnetic moments (`μ(triton)`), a unit of `MagneticMoment`: `n` of them is `n · 1.5046095202e-26` base units.
pub const TRITON_MAGNETIC_MOMENT: SIQuantity<
    {
        SIUnit::scaled(
            "μ(triton)",
            Exact::dec(15046095202, -36),
            SIUnit::ONE.times(METRE.unit(), 2).times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A magnetic permeability, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type MagneticPermeability<
    N,
    const U: SIUnit = {
        METRE
            .unit()
            .times(KILOGRAM.unit(), 1)
            .times(SECOND.unit(), -2)
            .times(AMPERE.unit(), -2)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Vacuum magnetic permeability (`µ₀`), a unit of `MagneticPermeability`: `n` of them is `n · 1.25663706212e-6` base units.
pub const VACUUM_PERMEABILITY: SIQuantity<
    {
        SIUnit::scaled(
            "µ₀",
            Exact::dec(125663706212, -17),
            METRE
                .unit()
                .times(KILOGRAM.unit(), 1)
                .times(SECOND.unit(), -2)
                .times(AMPERE.unit(), -2),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A gyromagnetic ratio, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type GyromagneticRatio<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 1)
            .times(AMPERE.unit(), 1)
            .times(RADIAN.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Proton gyromagnetic ratio (`γₚ`), a unit of `GyromagneticRatio`: `n` of them is `n · 267522187.08` base units.
pub const PROTON_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γₚ",
            Exact::dec(26752218708, -2),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Neutron gyromagnetic ratio (`γₙ`), a unit of `GyromagneticRatio`: `n` of them is `n · 183247174.0` base units.
pub const NEUTRON_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γₙ",
            Exact::int(183247174),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Electron gyromagnetic ratio (`γₑ`), a unit of `GyromagneticRatio`: `n` of them is `n · 176085962784.0` base units.
pub const ELECTRON_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γₑ",
            Exact::int(176085962784),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Shielded proton gyromagnetic ratio (`γ'ₚ`), a unit of `GyromagneticRatio`: `n` of them is `n · 267515319.4` base units.
pub const SHIELDED_PROTON_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ'ₚ",
            Exact::dec(2675153194, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Shielded helion gyromagnetic ratio (`γ'ₕ`), a unit of `GyromagneticRatio`: `n` of them is `n · 203789460.78` base units.
pub const SHIELDED_HELION_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ'ₕ",
            Exact::dec(20378946078, -2),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Deuteron gyromagnetic ratio (`γ₂`), a unit of `GyromagneticRatio`: `n` of them is `n · 41066288.8` base units.
pub const DEUTERON_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₂",
            Exact::dec(410662888, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Triton gyromagnetic ratio (`γ₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 285349844.0` base units.
pub const TRITON_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₃",
            Exact::int(285349844),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Lithium7 gyromagnetic ratio (`γ₇`), a unit of `GyromagneticRatio`: `n` of them is `n · 103975264.0` base units.
pub const LITHIUM7_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₇",
            Exact::int(103975264),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Boron11 gyromagnetic ratio (`γ₁₁`), a unit of `GyromagneticRatio`: `n` of them is `n · 85838414.3` base units.
pub const BORON11_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₁₁",
            Exact::dec(858384143, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Carbon13 gyromagnetic ratio (`γ₁₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 67278754.6` base units.
pub const CARBON13_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₁₃",
            Exact::dec(672787546, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Nitrogen14 gyromagnetic ratio (`γ₁₄`), a unit of `GyromagneticRatio`: `n` of them is `n · 19328792.2` base units.
pub const NITROGEN14_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₁₄",
            Exact::dec(193287922, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Flourine19 gyromagnetic ratio (`γ₁₉`), a unit of `GyromagneticRatio`: `n` of them is `n · 251762483.0` base units.
pub const FLOURINE19_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₁₉",
            Exact::int(251762483),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Sodium23 gyromagnetic ratio (`γ₂₃`), a unit of `GyromagneticRatio`: `n` of them is `n · 70803541.7` base units.
pub const SODIUM23_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₂₃",
            Exact::dec(708035417, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Phosphorus31 gyromagnetic ratio (`γ₃₁`), a unit of `GyromagneticRatio`: `n` of them is `n · 108329419.0` base units.
pub const PHOSPHORUS31_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ₃₁",
            Exact::int(108329419),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1)
                .times(RADIAN.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// A reduced gyromagnetic ratio, in `N`s of the unit `U`: by default its coherent unit, a
/// product of base units.
pub type ReducedGyromagneticRatio<
    N,
    const U: SIUnit = {
        SIUnit::ONE
            .times(KILOGRAM.unit(), -1)
            .times(SECOND.unit(), 1)
            .times(AMPERE.unit(), 1)
    },
    const B: Base = BASE,
> = SIQuantity<U, N, B>;

/// Proton reduced gyromagnetic ratio (`̅γₚ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 42577478.461` base units.
pub const PROTON_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γₚ",
            Exact::dec(42577478461, -3),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Neutron reduced gyromagnetic ratio (`̅γₙ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 29164693.5` base units.
pub const NEUTRON_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γₙ",
            Exact::dec(291646935, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Electron reduced gyromagnetic ratio (`̅γₑ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 28024951386.1` base units.
pub const ELECTRON_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γₑ",
            Exact::dec(280249513861, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Shielded proton reduced gyromagnetic ratio (`γ'ₚ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 42576385.43` base units.
pub const SHIELDED_PROTON_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ'ₚ",
            Exact::dec(4257638543, -2),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Shielded helion reduced gyromagnetic ratio (`γ'ₕ`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 32434100.033` base units.
pub const SHIELDED_HELION_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "γ'ₕ",
            Exact::dec(32434100033, -3),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Deuteron reduced gyromagnetic ratio (`̅γ₂`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 6535902.85` base units.
pub const DEUTERON_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₂",
            Exact::dec(653590285, -2),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Triton reduced gyromagnetic ratio (`̅γ₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 45414838.1` base units.
pub const TRITON_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₃",
            Exact::dec(454148381, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Lithium7 reduced gyromagnetic ratio (`̅γ₇`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 16548177.3` base units.
pub const LITHIUM7_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₇",
            Exact::dec(165481773, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Boron11 reduced gyromagnetic ratio (`̅γ₁₁`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 13661608.0` base units.
pub const BORON11_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₁₁",
            Exact::int(13661608),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Carbon13 reduced gyromagnetic ratio (`̅γ₁₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 10707746.4` base units.
pub const CARBON13_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₁₃",
            Exact::dec(107077464, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Nitrogen14 reduced gyromagnetic ratio (`̅γ₁₄`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 3076272.81` base units.
pub const NITROGEN14_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₁₄",
            Exact::dec(307627281, -2),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Flourine19 reduced gyromagnetic ratio (`̅γ₁₉`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 40069243.7` base units.
pub const FLOURINE19_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₁₉",
            Exact::dec(400692437, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Sodium23 reduced gyromagnetic ratio (`̅γ₂₃`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 11268733.6` base units.
pub const SODIUM23_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₂₃",
            Exact::dec(112687336, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;

/// Phosphorus31 reduced gyromagnetic ratio (`̅γ₃₁`), a unit of `ReducedGyromagneticRatio`: `n` of them is `n · 17241162.5` base units.
pub const PHOSPHORUS31_REDUCED_GYROMAGNETIC_RATIO: SIQuantity<
    {
        SIUnit::scaled(
            "̅γ₃₁",
            Exact::dec(172411625, -1),
            SIUnit::ONE
                .times(KILOGRAM.unit(), -1)
                .times(SECOND.unit(), 1)
                .times(AMPERE.unit(), 1),
        )
    },
    Exact,
    NOT_BASE,
> = SIQuantity::ONE;
