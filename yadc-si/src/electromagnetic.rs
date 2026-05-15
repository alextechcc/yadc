use crate::{ISQ, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type ElectricCharge = StaticSIQuantity<
    {
        ISQ {
            s: 1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricCharge)]
pub enum ElectricChargeUnit {
    #[unit(9.9999999999999998e+23, "YC", "yottacoulombs")]
    Yottacoulomb,
    #[unit(1e+21, "ZC", "zettacoulombs")]
    Zettacoulomb,
    #[unit(1e+18, "EC", "exacoulombs")]
    Exacoulomb,
    #[unit(1000000000000000.0, "PC", "petacoulombs")]
    Petacoulomb,
    #[unit(1000000000000.0, "TC", "teracoulombs")]
    Teracoulomb,
    #[unit(1000000000.0, "GC", "gigacoulombs")]
    Gigacoulomb,
    #[unit(1000000.0, "MC", "megacoulombs")]
    Megacoulomb,
    #[unit(1000.0, "kC", "kilocoulombs")]
    Kilocoulomb,
    #[unit(100.0, "hC", "hectocoulombs")]
    Hectocoulomb,
    #[unit(10.0, "daC", "decacoulombs")]
    Decacoulomb,
    #[unit(1.0, "C", "coulombs")]
    Coulomb,
    #[unit(0.10000000000000001, "dC", "decicoulombs")]
    Decicoulomb,
    #[unit(0.01, "cC", "centicoulombs")]
    Centicoulomb,
    #[unit(0.001, "mC", "millcoulombs")]
    Millicoulomb,
    #[unit(9.9999999999999995e-07, "µC", "microcoulombs")]
    Microcoulomb,
    #[unit(1.0000000000000001e-09, "nC", "nanocoulombs")]
    Nanocoulomb,
    #[unit(9.9999999999999998e-13, "pC", "picocoulombs")]
    Picocoulomb,
    #[unit(1.0000000000000001e-15, "fC", "femtocoulombs")]
    Femtocoulomb,
    #[unit(1.0000000000000001e-18, "aC", "attocoulombs")]
    Attocoulomb,
    #[unit(9.9999999999999991e-22, "zC", "zeptocoulombs")]
    Zeptocoulomb,
    #[unit(9.9999999999999992e-25, "yC", "yoctocoulombs")]
    Yoctocoulomb,
    #[unit(3.6e+18, "PA · h", "petaampere hours")]
    PetaampereHour,
    #[unit(3600000000000000.0, "TA · h", "teraampere hours")]
    TeraampereHour,
    #[unit(3600000000000.0, "GA · h", "gigaampere hours")]
    GigaampereHour,
    #[unit(3600000000.0, "MA · h", "megaampere hours")]
    MegaampereHour,
    #[unit(3600000.0, "kA · h", "kiloampere hours")]
    KiloampereHour,
    #[unit(360000.0, "hA · h", "hectoampere hours")]
    HectoampereHour,
    #[unit(36000.0, "daA · h", "decaampere hours")]
    DecaampereHour,
    #[unit(3600.0, "A · h", "ampere hours")]
    AmpereHour,
    #[unit(3.6000000000000001, "mA · h", "milliampere hours")]
    MilliampereHour,
    #[unit(0.0035999999999999999, "µA · h", "microampere hours")]
    MicroampereHour,
    #[unit(1.6021766339999999e-19, "e", "elementary charges")]
    ElementaryCharge,
    #[unit(1.6021766339999999e-19, "a.u. of charge", "atomic units of charge")]
    AtomicUnitOfCharge,
    #[unit(10.0, "abC", "abcoulombs")]
    Abcoulomb,
    #[unit(96485.309999999998, "F", "faradays")]
    Faraday,
    #[unit(3.335641e-10, "Fr", "franklins")]
    Franklin,
    #[unit(3.335641e-10, "statC", "statcoulombs")]
    Statcoulomb,
}

pub type ElectricChargeArealDensity = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: 1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricChargeArealDensity)]
pub enum ElectricChargeArealDensityUnit {
    #[unit(1.0, "C/m²", "coulombs per square meter")]
    CoulombPerSquareMeter,
}

pub type ElectricChargeLinearDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            s: 1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricChargeLinearDensity)]
pub enum ElectricChargeLinearDensityUnit {
    #[unit(1.0, "C/m", "coulombs per meter")]
    CoulombPerMeter,
    #[unit(100.0, "C/cm", "coulombs per centimeter")]
    CoulombPerCentimeter,
}

pub type ElectricChargeVolumetricDensity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            s: 1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricChargeVolumetricDensity)]
pub enum ElectricChargeVolumetricDensityUnit {
    #[unit(1.0, "C/m³", "coulombs per cubic meter")]
    CoulombPerCubicMeter,
}

pub type ElectricCurrent = StaticSIQuantity<{ ISQ { A: 1, ..ISQ::ZERO } }>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricCurrent)]
pub enum ElectricCurrentUnit {
    #[unit(9.9999999999999998e+23, "YA", "yottaamperes")]
    Yottaampere,
    #[unit(1e+21, "ZA", "zettaamperes")]
    Zettaampere,
    #[unit(1e+18, "EA", "exaamperes")]
    Exaampere,
    #[unit(1000000000000000.0, "PA", "petaamperes")]
    Petaampere,
    #[unit(1000000000000.0, "TA", "teraamperes")]
    Teraampere,
    #[unit(1000000000.0, "GA", "gigaamperes")]
    Gigaampere,
    #[unit(1000000.0, "MA", "megaamperes")]
    Megaampere,
    #[unit(1000.0, "kA", "kiloamperes")]
    Kiloampere,
    #[unit(100.0, "hA", "hectoamperes")]
    Hectoampere,
    #[unit(10.0, "daA", "decaamperes")]
    Decaampere,
    #[unit(1.0, "A", "amperes")]
    Ampere,
    #[unit(0.10000000000000001, "dA", "deciamperes")]
    Deciampere,
    #[unit(0.01, "cA", "centiamperes")]
    Centiampere,
    #[unit(0.001, "mA", "millamperes")]
    Milliampere,
    #[unit(9.9999999999999995e-07, "µA", "microamperes")]
    Microampere,
    #[unit(1.0000000000000001e-09, "nA", "nanoamperes")]
    Nanoampere,
    #[unit(9.9999999999999998e-13, "pA", "picoamperes")]
    Picoampere,
    #[unit(1.0000000000000001e-15, "fA", "femtoamperes")]
    Femtoampere,
    #[unit(1.0000000000000001e-18, "aA", "attoamperes")]
    Attoampere,
    #[unit(9.9999999999999991e-22, "zA", "zeptoamperes")]
    Zeptoampere,
    #[unit(9.9999999999999992e-25, "yA", "yoctoamperes")]
    Yoctoampere,
    #[unit(1.6021766339999999e-19, "e/s", "elementary charges per second")]
    ElementaryChargePerSecond,
    #[unit(
        1.6021766339999999e-19,
        "a.u. of charge/s",
        "atomic units of charge per second"
    )]
    AtomicUnitOfChargePerSecond,
    #[unit(10.0, "abA", "abamperes")]
    Abampere,
    #[unit(0.79577469999999995, "Gi", "gilberts")]
    Gilbert,
    #[unit(3.335641e-10, "statA", "statamperes")]
    Statampere,
}

pub type ElectricCurrentDensity = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricCurrentDensity)]
pub enum ElectricCurrentDensityUnit {
    #[unit(1.0, "A/m²", "amperes per square meter")]
    AmperePerSquareMeter,
}

pub type SurfaceElectricCurrentDensity = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(SurfaceElectricCurrentDensity)]
pub enum SurfaceElectricCurrentDensityUnit {
    #[unit(1.0, "A/m", "amperes per meter")]
    AmperePerMeter,
    #[unit(100.0, "A/cm", "amperes per centimeter")]
    AmperePerCentimeter,
    #[unit(1000.0, "A/mm", "amperes per millimeter")]
    AmperePerMillimeter,
    #[unit(1000000.0, "A/μm", "amperes per micrometer")]
    AmperePerMicrometer,
}

pub type ElectricPotential = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricPotential)]
pub enum ElectricPotentialUnit {
    #[unit(9.9999999999999998e+23, "YV", "yottavolts")]
    Yottavolt,
    #[unit(1e+21, "ZV", "zettavolts")]
    Zettavolt,
    #[unit(1e+18, "EV", "exavolts")]
    Exavolt,
    #[unit(1000000000000000.0, "PV", "petavolts")]
    Petavolt,
    #[unit(1000000000000.0, "TV", "teravolts")]
    Teravolt,
    #[unit(1000000000.0, "GV", "gigavolts")]
    Gigavolt,
    #[unit(1000000.0, "MV", "megavolts")]
    Megavolt,
    #[unit(1000.0, "kV", "kilovolts")]
    Kilovolt,
    #[unit(100.0, "hV", "hectovolts")]
    Hectovolt,
    #[unit(10.0, "daV", "decavolts")]
    Decavolt,
    #[unit(1.0, "V", "volts")]
    Volt,
    #[unit(0.10000000000000001, "dV", "decivolts")]
    Decivolt,
    #[unit(0.01, "cV", "centivolts")]
    Centivolt,
    #[unit(0.001, "mV", "millivolts")]
    Millivolt,
    #[unit(9.9999999999999995e-07, "µV", "microvolts")]
    Microvolt,
    #[unit(1.0000000000000001e-09, "nV", "nanovolts")]
    Nanovolt,
    #[unit(9.9999999999999998e-13, "pV", "picovolts")]
    Picovolt,
    #[unit(1.0000000000000001e-15, "fV", "femtovolts")]
    Femtovolt,
    #[unit(1.0000000000000001e-18, "aV", "attovolts")]
    Attovolt,
    #[unit(9.9999999999999991e-22, "zV", "zeptovolts")]
    Zeptovolt,
    #[unit(9.9999999999999992e-25, "yV", "yoctovolts")]
    Yoctovolt,
    #[unit(1e-08, "abV", "abvolts")]
    Abvolt,
    #[unit(299.79250000000002, "statV", "statvolts")]
    Statvolt,
}

pub type ElectricField = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricField)]
pub enum ElectricFieldUnit {
    #[unit(1.0, "V/m", "volts per meter")]
    VoltPerMeter,
    #[unit(100.0, "V/cm", "volts per centimeter")]
    VoltPerCentimeter,
    #[unit(1000.0, "V/mm", "volts per millimeter")]
    VoltPerMillimeter,
    #[unit(1000000.0, "V/μm", "volts per micrometer")]
    VoltPerMicrometer,
    #[unit(1000000.0, "kV/mm", "kilovolts per millimeter")]
    KilovoltPerMillimeter,
    #[unit(1000000.0, "MV/m", "megavolts per meter")]
    MegavoltPerMeter,
    #[unit(100000000.0, "MV/cm", "megavolts per centimeter")]
    MegavoltPerCentimeter,
    #[unit(39370.078740157478, "V/mil", "volts per mil")]
    VoltPerMil,
    #[unit(
        514220674763.25952,
        "a.u. of electric field",
        "atomic units of electric field"
    )]
    AtomicUnitOfElectricField,
}

pub type ElectricDisplacementField = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            s: 1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricDisplacementField)]
pub enum ElectricDisplacementFieldUnit {
    #[unit(1.0, "C/m²", "coulombs per square meter")]
    CoulombPerSquareMeter,
}

pub type ElectricFlux = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            kg: 1,
            s: -3,
            A: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricFlux)]
pub enum ElectricFluxUnit {
    #[unit(1.0, "V ⋅ m", "volt meters")]
    VoltMeter,
    #[unit(0.01, "V ⋅ cm", "volt centimeters")]
    VoltCentimeter,
}

pub type ElectricDipoleMoment = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            s: 1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricDipoleMoment)]
pub enum ElectricDipoleMomentUnit {
    #[unit(1.0, "C · m", "coulomb meters")]
    CoulombMeter,
    #[unit(
        1.6021766339999998e-21,
        "a.u. of charge · cm",
        "atomic unit of charge centimeters"
    )]
    AtomicUnitOfChargeCentimeter,
    #[unit(1.6021766339999998e-21, "e · cm", "elementary charge centimeters")]
    ElementaryChargeCentimeter,
    #[unit(
        8.478353625540766e-30,
        "e · a₀",
        "atomic units of electric dipole moment"
    )]
    AtomicUnitOfElectricDipoleMoment,
}

pub type ElectricQuadrupoleMoment = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            s: 1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricQuadrupoleMoment)]
pub enum ElectricQuadrupoleMomentUnit {
    #[unit(1.0, "C · m²", "coulomb square meters")]
    CoulombSquareMeter,
    #[unit(1.6021766339999998e-47, "e · b", "elementary charge barns")]
    ElementaryChargeBarn,
    #[unit(
        4.4865515246129999e-40,
        "e · a₀²",
        "atomic units of electric quadrupole moment"
    )]
    AtomicUnitOfElectricQuadrupoleMoment,
}

pub type ElectricPermittivity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 4,
            A: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricPermittivity)]
pub enum ElectricPermittivityUnit {
    #[unit(1.0, "F/m", "farads per meter")]
    FaradPerMeter,
    #[unit(8.8541878128000006e-12, "ε₀", "vacuum electric permittivity")]
    VacuumElectricPermittivity,
}

pub type ElectricalConductance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 3,
            A: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricalConductance)]
pub enum ElectricalConductanceUnit {
    #[unit(9.9999999999999998e+23, "YS", "yottasiemens")]
    Yottasiemens,
    #[unit(1e+21, "ZS", "zettasiemens")]
    Zettasiemens,
    #[unit(1e+18, "ES", "exasiemens")]
    Exasiemens,
    #[unit(1000000000000000.0, "PS", "petasiemens")]
    Petasiemens,
    #[unit(1000000000000.0, "TS", "terasiemens")]
    Terasiemens,
    #[unit(1000000000.0, "GS", "gigasiemens")]
    Gigasiemens,
    #[unit(1000000.0, "MS", "megasiemens")]
    Megasiemens,
    #[unit(1000.0, "kS", "kilosiemens")]
    Kilosiemens,
    #[unit(100.0, "hS", "hectosiemens")]
    Hectosiemens,
    #[unit(10.0, "daS", "decasiemens")]
    Decasiemens,
    #[unit(1.0, "S", "siemens")]
    Siemens,
    #[unit(1.0, "℧", "mhos")]
    Mho,
    #[unit(0.10000000000000001, "dS", "decisiemens")]
    Decisiemens,
    #[unit(0.01, "cS", "centisiemens")]
    Centisiemens,
    #[unit(0.001, "mS", "millisiemens")]
    Millisiemens,
    #[unit(9.9999999999999995e-07, "µS", "microsiemens")]
    Microsiemens,
    #[unit(1.0000000000000001e-09, "nS", "nanosiemens")]
    Nanosiemens,
    #[unit(9.9999999999999998e-13, "pS", "picosiemens")]
    Picosiemens,
    #[unit(1.0000000000000001e-15, "fS", "femtosiemens")]
    Femtosiemens,
    #[unit(1.0000000000000001e-18, "aS", "attosiemens")]
    Attosiemens,
    #[unit(9.9999999999999991e-22, "zS", "zeptosiemens")]
    Zeptosiemens,
    #[unit(9.9999999999999992e-25, "yS", "yoctosiemens")]
    Yoctosiemens,
    #[unit(1000000000.0, "abmho", "abmhos")]
    Abmho,
    #[unit(1000000000.0, "abS", "abmsiemens")]
    Absiemens,
    #[unit(1.1126499999999999e-12, "statS", "statsiemens")]
    Statsiemens,
    #[unit(1.1126499999999999e-12, "statmho", "statmhos")]
    Statmho,
}

pub type ElectricalConductivity = StaticSIQuantity<
    {
        ISQ {
            m: -3,
            kg: -1,
            s: 3,
            A: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricalConductivity)]
pub enum ElectricalConductivityUnit {
    #[unit(1.0, "S/m", "siemens per meter")]
    SiemensPerMeter,
    #[unit(100.0, "S/cm", "siemens per centimeter")]
    SiemensPerCentimeter,
}

pub type ElectricalMobility = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            s: 2,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricalMobility)]
pub enum ElectricalMobilityUnit {
    #[unit(1.0, "m²/(V · s)", "square meters per volt second")]
    SquareMeterPerVoltSecond,
    #[unit(0.0001, "cm²/(V · s)", "square centimeters per volt second")]
    SquareCentimeterPerVoltSecond,
}

pub type ElectricalResistance = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -3,
            A: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricalResistance)]
pub enum ElectricalResistanceUnit {
    #[unit(9.9999999999999998e+23, "YΩ", "yottaohms")]
    Yottaohm,
    #[unit(1e+21, "ZΩ", "zettaohms")]
    Zettaohm,
    #[unit(1e+18, "EΩ", "exaohms")]
    Exaohm,
    #[unit(1000000000000000.0, "PΩ", "petaohms")]
    Petaohm,
    #[unit(1000000000000.0, "TΩ", "teraohms")]
    Teraohm,
    #[unit(1000000000.0, "GΩ", "gigaohms")]
    Gigaohm,
    #[unit(1000000.0, "MΩ", "megaohms")]
    Megaohm,
    #[unit(1000.0, "kΩ", "kiloohms")]
    Kiloohm,
    #[unit(100.0, "hΩ", "hectoohms")]
    Hectoohm,
    #[unit(10.0, "daΩ", "decaohms")]
    Decaohm,
    #[unit(1.0, "Ω", "ohms")]
    Ohm,
    #[unit(0.10000000000000001, "dΩ", "deciohms")]
    Deciohm,
    #[unit(0.01, "cΩ", "centiohms")]
    Centiohm,
    #[unit(0.001, "mΩ", "milliohms")]
    Milliohm,
    #[unit(9.9999999999999995e-07, "µΩ", "microohms")]
    Microohm,
    #[unit(1.0000000000000001e-09, "nΩ", "nanoohms")]
    Nanoohm,
    #[unit(9.9999999999999998e-13, "pΩ", "picoohms")]
    Picoohm,
    #[unit(1.0000000000000001e-15, "fΩ", "femtoohms")]
    Femtoohm,
    #[unit(1.0000000000000001e-18, "aΩ", "attoohms")]
    Attoohm,
    #[unit(9.9999999999999991e-22, "zΩ", "zeptoohms")]
    Zeptoohm,
    #[unit(9.9999999999999992e-25, "yΩ", "yoctoohms")]
    Yoctoohm,
    #[unit(1.0000000000000001e-09, "abΩ", "abohms")]
    Abohm,
    #[unit(898755291711.5481, "statΩ", "statohms")]
    Statohm,
}

pub type ElectricalResistivity = StaticSIQuantity<
    {
        ISQ {
            m: 3,
            kg: 1,
            s: -3,
            A: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(ElectricalResistivity)]
pub enum ElectricalResistivityUnit {
    #[unit(9.9999999999999998e+23, "YΩ · m", "yottaohm meters")]
    YottaohmMeter,
    #[unit(1e+21, "ZΩ · m", "zettaohm meters")]
    ZettaohmMeter,
    #[unit(1e+18, "EΩ · m", "exaohm meters")]
    ExaohmMeter,
    #[unit(1000000000000000.0, "PΩ · m", "petaohm meters")]
    PetaohmMeter,
    #[unit(1000000000000.0, "TΩ · m", "teraohm meters")]
    TeraohmMeter,
    #[unit(1000000000.0, "GΩ · m", "gigaohm meters")]
    GigaohmMeter,
    #[unit(1000000.0, "MΩ · m", "megaohm meters")]
    MegaohmMeter,
    #[unit(1000.0, "kΩ · m", "kiloohm meters")]
    KiloohmMeter,
    #[unit(100.0, "hΩ · m", "hectoohm meters")]
    HectoohmMeter,
    #[unit(10.0, "daΩ · m", "decaohm meters")]
    DecaohmMeter,
    #[unit(1.0, "Ω · m", "ohm meters")]
    OhmMeter,
    #[unit(0.10000000000000001, "dΩ · m", "deciohm meters")]
    DeciohmMeter,
    #[unit(0.01, "cΩ · m", "centiohm meters")]
    CentiohmMeter,
    #[unit(0.001, "mΩ · m", "milliohm meters")]
    MilliohmMeter,
    #[unit(9.9999999999999995e-07, "µΩ · m", "microohm meters")]
    MicroohmMeter,
    #[unit(1.0000000000000001e-09, "nΩ · m", "nanoohm meters")]
    NanoohmMeter,
    #[unit(9.9999999999999998e-13, "pΩ · m", "picoohm meters")]
    PicoohmMeter,
    #[unit(1.0000000000000001e-15, "fΩ · m", "femtoohm meters")]
    FemtoohmMeter,
    #[unit(1.0000000000000001e-18, "aΩ · m", "attoohm meters")]
    AttoohmMeter,
    #[unit(9.9999999999999991e-22, "zΩ · m", "zeptoohm meters")]
    ZeptoohmMeter,
    #[unit(9.9999999999999992e-25, "yΩ · m", "yoctoohm meters")]
    YoctoohmMeter,
    #[unit(1.0000000000000001e-09, "abΩ · m", "abohm meters")]
    AbohmMeter,
    #[unit(898755291711.5481, "statΩ · m", "statohm meters")]
    StatohmMeter,
    #[unit(0.01, "Ω · cm", "ohm centimeters")]
    OhmCentimeter,
    #[unit(1.0000000000000001e-11, "abΩ · cm", "abohm centimeters")]
    AbohmCentimeter,
    #[unit(8987552917.1154804, "statΩ · cm", "statohm centimeters")]
    StatohmCentimeter,
    #[unit(0.025399999999999999, "Ω · in", "ohm inches")]
    OhmInch,
    #[unit(0.30480000000000002, "Ω · ft", "ohm feet")]
    OhmFoot,
    #[unit(0.91439999999999999, "Ω · yd", "ohm yards")]
    OhmYard,
    #[unit(
        9.9999999999999995e-07,
        "Ω · mm²/m",
        "ohm square millimeters per meter"
    )]
    OhmSquareMillimeterPerMeter,
}

pub type Capacitance = StaticSIQuantity<
    {
        ISQ {
            m: -2,
            kg: -1,
            s: 4,
            A: 2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Capacitance)]
pub enum CapacitanceUnit {
    #[unit(9.9999999999999998e+23, "YF", "yottafarads")]
    Yottafarad,
    #[unit(1e+21, "ZF", "zettafarads")]
    Zettafarad,
    #[unit(1e+18, "EF", "exafarads")]
    Exafarad,
    #[unit(1000000000000000.0, "PF", "petafarads")]
    Petafarad,
    #[unit(1000000000000.0, "TF", "terafarads")]
    Terafarad,
    #[unit(1000000000.0, "GF", "gigafarads")]
    Gigafarad,
    #[unit(1000000.0, "MF", "megafarads")]
    Megafarad,
    #[unit(1000.0, "kF", "kilofarads")]
    Kilofarad,
    #[unit(100.0, "hF", "hectofarads")]
    Hectofarad,
    #[unit(10.0, "daF", "decafarads")]
    Decafarad,
    #[unit(1.0, "F", "farads")]
    Farad,
    #[unit(0.10000000000000001, "dF", "decifarads")]
    Decifarad,
    #[unit(0.01, "cF", "centifarads")]
    Centifarad,
    #[unit(0.001, "mF", "millifarads")]
    Millifarad,
    #[unit(9.9999999999999995e-07, "µF", "microfarads")]
    Microfarad,
    #[unit(1.0000000000000001e-09, "nF", "nanofarads")]
    Nanofarad,
    #[unit(9.9999999999999998e-13, "pF", "picofarads")]
    Picofarad,
    #[unit(1.0000000000000001e-15, "fF", "femtofarads")]
    Femtofarad,
    #[unit(1.0000000000000001e-18, "aF", "attofarads")]
    Attofarad,
    #[unit(9.9999999999999991e-22, "zF", "zeptofarads")]
    Zeptofarad,
    #[unit(9.9999999999999992e-25, "yF", "yoctofarads")]
    Yoctofarad,
    #[unit(1000000000.0, "abF", "abfarads")]
    Abfarad,
    #[unit(1.1126499999999999e-12, "statF", "statfarads")]
    Statfarad,
}

pub type Inductance = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            A: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Inductance)]
pub enum InductanceUnit {
    #[unit(9.9999999999999998e+23, "YH", "yottahenries")]
    Yottahenry,
    #[unit(1e+21, "ZH", "zettahenries")]
    Zettahenry,
    #[unit(1e+18, "EH", "exahenries")]
    Exahenry,
    #[unit(1000000000000000.0, "PH", "petahenries")]
    Petahenry,
    #[unit(1000000000000.0, "TH", "terahenries")]
    Terahenry,
    #[unit(1000000000.0, "GH", "gigahenries")]
    Gigahenry,
    #[unit(1000000.0, "MH", "megahenries")]
    Megahenry,
    #[unit(1000.0, "kH", "kilohenries")]
    Kilohenry,
    #[unit(100.0, "hH", "hectohenries")]
    Hectohenry,
    #[unit(10.0, "daH", "decahenries")]
    Decahenry,
    #[unit(1.0, "H", "henries")]
    Henry,
    #[unit(0.10000000000000001, "dH", "decihenries")]
    Decihenry,
    #[unit(0.01, "cH", "centihenries")]
    Centihenry,
    #[unit(0.001, "mH", "millihenries")]
    Millihenry,
    #[unit(9.9999999999999995e-07, "µH", "microhenries")]
    Microhenry,
    #[unit(1.0000000000000001e-09, "nH", "nanohenries")]
    Nanohenry,
    #[unit(9.9999999999999998e-13, "pH", "picohenries")]
    Picohenry,
    #[unit(1.0000000000000001e-15, "fH", "femtohenries")]
    Femtohenry,
    #[unit(1.0000000000000001e-18, "aH", "attohenries")]
    Attohenry,
    #[unit(9.9999999999999991e-22, "zH", "zeptohenries")]
    Zeptohenry,
    #[unit(9.9999999999999992e-25, "yH", "yoctohenries")]
    Yoctohenry,
    #[unit(1.0000000000000001e-09, "abH", "abhenries")]
    Abhenry,
    #[unit(898755291711.5481, "statH", "stathenries")]
    Stathenry,
}

pub type MagneticFieldStrength = StaticSIQuantity<
    {
        ISQ {
            m: -1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(MagneticFieldStrength)]
pub enum MagneticFieldStrengthUnit {
    #[unit(1.0, "A/m", "amperes per meter")]
    AmperePerMeter,
    #[unit(100.0, "A/cm", "amperes per centimeter")]
    AmperePerCentimeter,
    #[unit(1000.0, "A/mm", "amperes per millimeter")]
    AmperePerMillimeter,
    #[unit(1000000.0, "A/μm", "amperes per micrometer")]
    AmperePerMicrometer,
    #[unit(79.577471545947674, "Oe", "oersteds")]
    Oersted,
}

pub type MagneticFlux = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -2,
            A: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(MagneticFlux)]
pub enum MagneticFluxUnit {
    #[unit(9.9999999999999998e+23, "YWb", "yottawebers")]
    Yottaweber,
    #[unit(1e+21, "ZWb", "zettawebers")]
    Zettaweber,
    #[unit(1e+18, "EWb", "exawebers")]
    Exaweber,
    #[unit(1000000000000000.0, "PWb", "petawebers")]
    Petaweber,
    #[unit(1000000000000.0, "TWb", "terawebers")]
    Teraweber,
    #[unit(1000000000.0, "GWb", "gigawebers")]
    Gigaweber,
    #[unit(1000000.0, "MWb", "megawebers")]
    Megaweber,
    #[unit(1000.0, "kWb", "kilowebers")]
    Kiloweber,
    #[unit(100.0, "hWb", "hectowebers")]
    Hectoweber,
    #[unit(10.0, "daWb", "decawebers")]
    Decaweber,
    #[unit(1.0, "Wb", "webers")]
    Weber,
    #[unit(0.10000000000000001, "dWb", "deciwebers")]
    Deciweber,
    #[unit(0.01, "cWb", "centiwebers")]
    Centiweber,
    #[unit(0.001, "mWb", "milliwebers")]
    Milliweber,
    #[unit(9.9999999999999995e-07, "µWb", "microwebers")]
    Microweber,
    #[unit(1.0000000000000001e-09, "nWb", "nanowebers")]
    Nanoweber,
    #[unit(9.9999999999999998e-13, "pWb", "picowebers")]
    Picoweber,
    #[unit(1.0000000000000001e-15, "fWb", "femtowebers")]
    Femtoweber,
    #[unit(1.0000000000000001e-18, "aWb", "attowebers")]
    Attoweber,
    #[unit(9.9999999999999991e-22, "zWb", "zeptowebers")]
    Zeptoweber,
    #[unit(9.9999999999999992e-25, "yWb", "yoctowebers")]
    Yoctoweber,
    #[unit(1e-08, "Mx", "maxwells")]
    Maxwell,
}

pub type MagneticFluxDensity = StaticSIQuantity<
    {
        ISQ {
            kg: 1,
            s: -2,
            A: -1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(MagneticFluxDensity)]
pub enum MagneticFluxDensityUnit {
    #[unit(9.9999999999999998e+23, "YT", "yottateslas")]
    Yottatesla,
    #[unit(1e+21, "ZT", "zettateslas")]
    Zettatesla,
    #[unit(1e+18, "ET", "exateslas")]
    Exatesla,
    #[unit(1000000000000000.0, "PT", "petateslas")]
    Petatesla,
    #[unit(1000000000000.0, "TT", "terateslas")]
    Teratesla,
    #[unit(1000000000.0, "GT", "gigateslas")]
    Gigatesla,
    #[unit(1000000.0, "MT", "megateslas")]
    Megatesla,
    #[unit(1000.0, "kT", "kiloteslas")]
    Kilotesla,
    #[unit(100.0, "hT", "hectoteslas")]
    Hectotesla,
    #[unit(10.0, "daT", "decateslas")]
    Decatesla,
    #[unit(1.0, "T", "teslas")]
    Tesla,
    #[unit(0.10000000000000001, "dT", "deciteslas")]
    Decitesla,
    #[unit(0.01, "cT", "centiteslas")]
    Centitesla,
    #[unit(0.001, "mT", "milliteslas")]
    Millitesla,
    #[unit(9.9999999999999995e-07, "µT", "microteslas")]
    Microtesla,
    #[unit(1.0000000000000001e-09, "nT", "nanoteslas")]
    Nanotesla,
    #[unit(9.9999999999999998e-13, "pT", "picoteslas")]
    Picotesla,
    #[unit(1.0000000000000001e-15, "fT", "femtoteslas")]
    Femtotesla,
    #[unit(1.0000000000000001e-18, "aT", "attoteslas")]
    Attotesla,
    #[unit(9.9999999999999991e-22, "zT", "zeptoteslas")]
    Zeptotesla,
    #[unit(9.9999999999999992e-25, "yT", "yoctoteslas")]
    Yoctotesla,
    #[unit(1.0000000000000001e-09, "γ", "gammas")]
    Gamma,
    #[unit(0.0001, "G", "gauss")]
    Gauss,
}

pub type MagneticMoment = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(MagneticMoment)]
pub enum MagneticMomentUnit {
    #[unit(1.0, "A · m²", "ampere square meters")]
    AmpereSquareMeter,
    #[unit(1.0, "J/T", "joules per tesla")]
    JoulePerTesla,
    #[unit(1.0, "N · m/T", "newton meters per tesla")]
    NewtonMeterPerTesla,
    #[unit(0.0009999999999999998, "erg/G", "ergs per gauss")]
    ErgPerGauss,
    #[unit(9.2740100783000004e-24, "µ(Bohr)", "Bohr magnetons")]
    BohrMagneton,
    #[unit(5.0507837461000003e-27, "μ(Nuclear)", "nuclear magnetons")]
    NuclearMagneton,
    #[unit(
        1.8548020156600001e-23,
        "ħ · e/mₑ",
        " atomic units of magnetic dipole moment"
    )]
    AtomicUnitOfMagneticDipoleMoment,
    #[unit(4.3307350939999997e-27, "μ(deuteron)", "deuteron magnetic moments")]
    DeuteronMagneticMoment,
    #[unit(1.4106067973599999e-26, "μ(proton)", "proton magnetic moments")]
    ProtonMagneticMoment,
    #[unit(1.41057056e-26, "μ’(proton)", "shielded proton magnetic moments")]
    ShieldedProtonMagneticMoment,
    #[unit(1.5046095202000001e-26, "μ(triton)", "triton magnetic moments")]
    TritonMagneticMoment,
}

pub type MagneticPermeability = StaticSIQuantity<
    {
        ISQ {
            m: 1,
            kg: 1,
            s: -2,
            A: -2,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(MagneticPermeability)]
pub enum MagneticPermeabilityUnit {
    #[unit(1.0, "H/m", "henrys per meter")]
    HenryPerMeter,
    #[unit(1.2566370621199999e-06, "µ₀", "vacuum magnetic permeability")]
    VacuumMagneticPermeability,
}

pub type GyromagneticRatio = StaticSIQuantity<
    {
        ISQ {
            kg: -1,
            s: 1,
            A: 1,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(GyromagneticRatio)]
pub enum GyromagneticRatioUnit {
    #[unit(9.9999999999999998e+23, "YHz/T", "yottahertz per tesla")]
    YottahertzPerTesla,
    #[unit(1e+21, "ZHz/T", "zettahertz per tesla")]
    ZettahertzPerTesla,
    #[unit(1e+18, "EHz/T", "exahertz per tesla")]
    ExahertzPerTesla,
    #[unit(1000000000000000.0, "PHz/T", "petahertz per tesla")]
    PetahertzPerTesla,
    #[unit(1000000000000.0, "THz/T", "terahertz per tesla")]
    TerahertzPerTesla,
    #[unit(1000000000.0, "GHz/T", "gigahertz per tesla")]
    GigahertzPerTesla,
    #[unit(1000000.0, "MHz/T", "megahertz per tesla")]
    MegahertzPerTesla,
    #[unit(1000.0, "kHz/T", "kilohertz per tesla")]
    KilohertzPerTesla,
    #[unit(100.0, "hHz/T", "hectohertz per tesla")]
    HectohertzPerTesla,
    #[unit(10.0, "daHz/T", "decahertz per tesla")]
    DecahertzPerTesla,
    #[unit(1.0, "Hz/T", "hertz per tesla")]
    HertzPerTesla,
    #[unit(0.10000000000000001, "dHz/T", "decihertz per tesla")]
    DecihertzPerTesla,
    #[unit(0.01, "cHz/T", "centihertz per tesla")]
    CentihertzPerTesla,
    #[unit(0.001, "mHz/T", "millihertz per tesla")]
    MillihertzPerTesla,
    #[unit(9.9999999999999995e-07, "µHz/T", "microhertz per tesla")]
    MicrohertzPerTesla,
    #[unit(1.0000000000000001e-09, "nHz/T", "nanohertz per tesla")]
    NanohertzPerTesla,
    #[unit(9.9999999999999998e-13, "pHz/T", "picohertz per tesla")]
    PicohertzPerTesla,
    #[unit(1.0000000000000001e-15, "fHz/T", "femtohertz per tesla")]
    FemtohertzPerTesla,
    #[unit(1.0000000000000001e-18, "aHz/T", "attohertz per tesla")]
    AttohertzPerTesla,
    #[unit(9.9999999999999991e-22, "zHz/T", "zeptohertz per tesla")]
    ZeptohertzPerTesla,
    #[unit(9.9999999999999992e-25, "yHz/T", "yoctohertz per tesla")]
    YoctohertzPerTesla,
    #[unit(1.0, "rad/(s · T)", "radians per second tesla")]
    RadianPerSecondTesla,
    #[unit(267522187.08000001, "γₚ", "proton gyromagnetic ratio")]
    ProtonGyromagneticRatio,
    #[unit(183247174.0, "γₙ", "neutron gyromagnetic ratio")]
    NeutronGyromagneticRatio,
    #[unit(176085962784.0, "γₑ", "electron gyromagnetic ratio")]
    ElectronGyromagneticRatio,
    #[unit(267515319.40000001, "γ'ₚ", "shielded proton gyromagnetic ratio")]
    ShieldedProtonGyromagneticRatio,
    #[unit(203789460.78, "γ'ₕ", "shielded helion gyromagnetic ratio")]
    ShieldedHelionGyromagneticRatio,
    #[unit(41066288.799999997, "γ₂", "deuteron gyromagnetic ratio")]
    DeuteronGyromagneticRatio,
    #[unit(285349844.0, "γ₃", "triton gyromagnetic ratio")]
    TritonGyromagneticRatio,
    #[unit(103975264.0, "γ₇", "lithium7 gyromagnetic ratio")]
    Lithium7GyromagneticRatio,
    #[unit(85838414.299999997, "γ₁₁", "boron11 gyromagnetic ratio")]
    Boron11GyromagneticRatio,
    #[unit(67278754.599999994, "γ₁₃", "carbon13 gyromagnetic ratio")]
    Carbon13GyromagneticRatio,
    #[unit(19328792.199999999, "γ₁₄", "nitrogen14 gyromagnetic ratio")]
    Nitrogen14GyromagneticRatio,
    #[unit(251762483.0, "γ₁₉", "flourine19 gyromagnetic ratio")]
    Flourine19GyromagneticRatio,
    #[unit(70803541.700000003, "γ₂₃", "sodium23 gyromagnetic ratio")]
    Sodium23GyromagneticRatio,
    #[unit(108329419.0, "γ₃₁", "phosphorus31 gyromagnetic ratio")]
    Phosphorus31GyromagneticRatio,
    #[unit(42577478.461000003, "̅γₚ", "proton reduced gyromagnetic ratio")]
    ProtonReducedGyromagneticRatio,
    #[unit(29164693.5, "̅γₙ", "neutron reduced gyromagnetic ratio")]
    NeutronReducedGyromagneticRatio,
    #[unit(28024951386.099998, "̅γₑ", "electron reduced gyromagnetic ratio")]
    ElectronReducedGyromagneticRatio,
    #[unit(42576385.43, "γ'ₚ", "shielded proton reduced gyromagnetic ratio")]
    ShieldedProtonReducedGyromagneticRatio,
    #[unit(32434100.033, "γ'ₕ", "shielded helion reduced gyromagnetic ratio")]
    ShieldedHelionReducedGyromagneticRatio,
    #[unit(6535902.8499999996, "̅γ₂", "deuteron reduced gyromagnetic ratio")]
    DeuteronReducedGyromagneticRatio,
    #[unit(45414838.100000001, "̅γ₃", "triton reduced gyromagnetic ratio")]
    TritonReducedGyromagneticRatio,
    #[unit(16548177.300000001, "̅γ₇", "lithium7 reduced gyromagnetic ratio")]
    Lithium7ReducedGyromagneticRatio,
    #[unit(13661608.0, "̅γ₁₁", "boron11 reduced gyromagnetic ratio")]
    Boron11ReducedGyromagneticRatio,
    #[unit(10707746.4, "̅γ₁₃", "carbon13 reduced gyromagnetic ratio")]
    Carbon13ReducedGyromagneticRatio,
    #[unit(3076272.8100000001, "̅γ₁₄", "nitrogen14 reduced gyromagnetic ratio")]
    Nitrogen14ReducedGyromagneticRatio,
    #[unit(40069243.700000003, "̅γ₁₉", "flourine19 reduced gyromagnetic ratio")]
    Flourine19ReducedGyromagneticRatio,
    #[unit(11268733.6, "̅γ₂₃", "sodium23 reduced gyromagnetic ratio")]
    Sodium23ReducedGyromagneticRatio,
    #[unit(17241162.5, "̅γ₃₁", "phosphorus31 reduced gyromagnetic ratio")]
    Phosphorus31ReducedGyromagneticRatio,
}
