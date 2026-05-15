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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricCharge)]
pub enum ElectricChargeUnit {
    #[linear("YC", "yottacoulombs", 9.9999999999999998e+23)]
    Yottacoulomb,
    #[linear("ZC", "zettacoulombs", 1e+21)]
    Zettacoulomb,
    #[linear("EC", "exacoulombs", 1e+18)]
    Exacoulomb,
    #[linear("PC", "petacoulombs", 1000000000000000.0)]
    Petacoulomb,
    #[linear("TC", "teracoulombs", 1000000000000.0)]
    Teracoulomb,
    #[linear("GC", "gigacoulombs", 1000000000.0)]
    Gigacoulomb,
    #[linear("MC", "megacoulombs", 1000000.0)]
    Megacoulomb,
    #[linear("kC", "kilocoulombs", 1000.0)]
    Kilocoulomb,
    #[linear("hC", "hectocoulombs", 100.0)]
    Hectocoulomb,
    #[linear("daC", "decacoulombs", 10.0)]
    Decacoulomb,
    #[linear("C", "coulombs", 1.0)]
    Coulomb,
    #[linear("dC", "decicoulombs", 0.10000000000000001)]
    Decicoulomb,
    #[linear("cC", "centicoulombs", 0.01)]
    Centicoulomb,
    #[linear("mC", "millcoulombs", 0.001)]
    Millicoulomb,
    #[linear("µC", "microcoulombs", 9.9999999999999995e-07)]
    Microcoulomb,
    #[linear("nC", "nanocoulombs", 1.0000000000000001e-09)]
    Nanocoulomb,
    #[linear("pC", "picocoulombs", 9.9999999999999998e-13)]
    Picocoulomb,
    #[linear("fC", "femtocoulombs", 1.0000000000000001e-15)]
    Femtocoulomb,
    #[linear("aC", "attocoulombs", 1.0000000000000001e-18)]
    Attocoulomb,
    #[linear("zC", "zeptocoulombs", 9.9999999999999991e-22)]
    Zeptocoulomb,
    #[linear("yC", "yoctocoulombs", 9.9999999999999992e-25)]
    Yoctocoulomb,
    #[linear("PA · h", "petaampere hours", 3.6e+18)]
    PetaampereHour,
    #[linear("TA · h", "teraampere hours", 3600000000000000.0)]
    TeraampereHour,
    #[linear("GA · h", "gigaampere hours", 3600000000000.0)]
    GigaampereHour,
    #[linear("MA · h", "megaampere hours", 3600000000.0)]
    MegaampereHour,
    #[linear("kA · h", "kiloampere hours", 3600000.0)]
    KiloampereHour,
    #[linear("hA · h", "hectoampere hours", 360000.0)]
    HectoampereHour,
    #[linear("daA · h", "decaampere hours", 36000.0)]
    DecaampereHour,
    #[linear("A · h", "ampere hours", 3600.0)]
    AmpereHour,
    #[linear("mA · h", "milliampere hours", 3.6000000000000001)]
    MilliampereHour,
    #[linear("µA · h", "microampere hours", 0.0035999999999999999)]
    MicroampereHour,
    #[linear("e", "elementary charges", 1.6021766339999999e-19)]
    ElementaryCharge,
    #[linear("a.u. of charge", "atomic units of charge", 1.6021766339999999e-19)]
    AtomicUnitOfCharge,
    #[linear("abC", "abcoulombs", 10.0)]
    Abcoulomb,
    #[linear("F", "faradays", 96485.309999999998)]
    Faraday,
    #[linear("Fr", "franklins", 3.335641e-10)]
    Franklin,
    #[linear("statC", "statcoulombs", 3.335641e-10)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricChargeArealDensity)]
pub enum ElectricChargeArealDensityUnit {
    #[linear("C/m²", "coulombs per square meter", 1.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricChargeLinearDensity)]
pub enum ElectricChargeLinearDensityUnit {
    #[linear("C/m", "coulombs per meter", 1.0)]
    CoulombPerMeter,
    #[linear("C/cm", "coulombs per centimeter", 100.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricChargeVolumetricDensity)]
pub enum ElectricChargeVolumetricDensityUnit {
    #[linear("C/m³", "coulombs per cubic meter", 1.0)]
    CoulombPerCubicMeter,
}

pub type ElectricCurrent = StaticSIQuantity<{ ISQ { A: 1, ..ISQ::ZERO } }>;

#[derive(Unit, Copy, Clone)]
#[dim(ElectricCurrent)]
pub enum ElectricCurrentUnit {
    #[linear("YA", "yottaamperes", 9.9999999999999998e+23)]
    Yottaampere,
    #[linear("ZA", "zettaamperes", 1e+21)]
    Zettaampere,
    #[linear("EA", "exaamperes", 1e+18)]
    Exaampere,
    #[linear("PA", "petaamperes", 1000000000000000.0)]
    Petaampere,
    #[linear("TA", "teraamperes", 1000000000000.0)]
    Teraampere,
    #[linear("GA", "gigaamperes", 1000000000.0)]
    Gigaampere,
    #[linear("MA", "megaamperes", 1000000.0)]
    Megaampere,
    #[linear("kA", "kiloamperes", 1000.0)]
    Kiloampere,
    #[linear("hA", "hectoamperes", 100.0)]
    Hectoampere,
    #[linear("daA", "decaamperes", 10.0)]
    Decaampere,
    #[linear("A", "amperes", 1.0)]
    Ampere,
    #[linear("dA", "deciamperes", 0.10000000000000001)]
    Deciampere,
    #[linear("cA", "centiamperes", 0.01)]
    Centiampere,
    #[linear("mA", "millamperes", 0.001)]
    Milliampere,
    #[linear("µA", "microamperes", 9.9999999999999995e-07)]
    Microampere,
    #[linear("nA", "nanoamperes", 1.0000000000000001e-09)]
    Nanoampere,
    #[linear("pA", "picoamperes", 9.9999999999999998e-13)]
    Picoampere,
    #[linear("fA", "femtoamperes", 1.0000000000000001e-15)]
    Femtoampere,
    #[linear("aA", "attoamperes", 1.0000000000000001e-18)]
    Attoampere,
    #[linear("zA", "zeptoamperes", 9.9999999999999991e-22)]
    Zeptoampere,
    #[linear("yA", "yoctoamperes", 9.9999999999999992e-25)]
    Yoctoampere,
    #[linear("e/s", "elementary charges per second", 1.6021766339999999e-19)]
    ElementaryChargePerSecond,
    #[linear(
        "a.u. of charge/s",
        "atomic units of charge per second",
        1.6021766339999999e-19
    )]
    AtomicUnitOfChargePerSecond,
    #[linear("abA", "abamperes", 10.0)]
    Abampere,
    #[linear("Gi", "gilberts", 0.79577469999999995)]
    Gilbert,
    #[linear("statA", "statamperes", 3.335641e-10)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricCurrentDensity)]
pub enum ElectricCurrentDensityUnit {
    #[linear("A/m²", "amperes per square meter", 1.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(SurfaceElectricCurrentDensity)]
pub enum SurfaceElectricCurrentDensityUnit {
    #[linear("A/m", "amperes per meter", 1.0)]
    AmperePerMeter,
    #[linear("A/cm", "amperes per centimeter", 100.0)]
    AmperePerCentimeter,
    #[linear("A/mm", "amperes per millimeter", 1000.0)]
    AmperePerMillimeter,
    #[linear("A/μm", "amperes per micrometer", 1000000.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricPotential)]
pub enum ElectricPotentialUnit {
    #[linear("YV", "yottavolts", 9.9999999999999998e+23)]
    Yottavolt,
    #[linear("ZV", "zettavolts", 1e+21)]
    Zettavolt,
    #[linear("EV", "exavolts", 1e+18)]
    Exavolt,
    #[linear("PV", "petavolts", 1000000000000000.0)]
    Petavolt,
    #[linear("TV", "teravolts", 1000000000000.0)]
    Teravolt,
    #[linear("GV", "gigavolts", 1000000000.0)]
    Gigavolt,
    #[linear("MV", "megavolts", 1000000.0)]
    Megavolt,
    #[linear("kV", "kilovolts", 1000.0)]
    Kilovolt,
    #[linear("hV", "hectovolts", 100.0)]
    Hectovolt,
    #[linear("daV", "decavolts", 10.0)]
    Decavolt,
    #[linear("V", "volts", 1.0)]
    Volt,
    #[linear("dV", "decivolts", 0.10000000000000001)]
    Decivolt,
    #[linear("cV", "centivolts", 0.01)]
    Centivolt,
    #[linear("mV", "millivolts", 0.001)]
    Millivolt,
    #[linear("µV", "microvolts", 9.9999999999999995e-07)]
    Microvolt,
    #[linear("nV", "nanovolts", 1.0000000000000001e-09)]
    Nanovolt,
    #[linear("pV", "picovolts", 9.9999999999999998e-13)]
    Picovolt,
    #[linear("fV", "femtovolts", 1.0000000000000001e-15)]
    Femtovolt,
    #[linear("aV", "attovolts", 1.0000000000000001e-18)]
    Attovolt,
    #[linear("zV", "zeptovolts", 9.9999999999999991e-22)]
    Zeptovolt,
    #[linear("yV", "yoctovolts", 9.9999999999999992e-25)]
    Yoctovolt,
    #[linear("abV", "abvolts", 1e-08)]
    Abvolt,
    #[linear("statV", "statvolts", 299.79250000000002)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricField)]
pub enum ElectricFieldUnit {
    #[linear("V/m", "volts per meter", 1.0)]
    VoltPerMeter,
    #[linear("V/cm", "volts per centimeter", 100.0)]
    VoltPerCentimeter,
    #[linear("V/mm", "volts per millimeter", 1000.0)]
    VoltPerMillimeter,
    #[linear("V/μm", "volts per micrometer", 1000000.0)]
    VoltPerMicrometer,
    #[linear("kV/mm", "kilovolts per millimeter", 1000000.0)]
    KilovoltPerMillimeter,
    #[linear("MV/m", "megavolts per meter", 1000000.0)]
    MegavoltPerMeter,
    #[linear("MV/cm", "megavolts per centimeter", 100000000.0)]
    MegavoltPerCentimeter,
    #[linear("V/mil", "volts per mil", 39370.078740157478)]
    VoltPerMil,
    #[linear(
        "a.u. of electric field",
        "atomic units of electric field",
        514220674763.25952
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricDisplacementField)]
pub enum ElectricDisplacementFieldUnit {
    #[linear("C/m²", "coulombs per square meter", 1.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricFlux)]
pub enum ElectricFluxUnit {
    #[linear("V ⋅ m", "volt meters", 1.0)]
    VoltMeter,
    #[linear("V ⋅ cm", "volt centimeters", 0.01)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricDipoleMoment)]
pub enum ElectricDipoleMomentUnit {
    #[linear("C · m", "coulomb meters", 1.0)]
    CoulombMeter,
    #[linear(
        "a.u. of charge · cm",
        "atomic unit of charge centimeters",
        1.6021766339999998e-21
    )]
    AtomicUnitOfChargeCentimeter,
    #[linear("e · cm", "elementary charge centimeters", 1.6021766339999998e-21)]
    ElementaryChargeCentimeter,
    #[linear(
        "e · a₀",
        "atomic units of electric dipole moment",
        8.478353625540766e-30
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricQuadrupoleMoment)]
pub enum ElectricQuadrupoleMomentUnit {
    #[linear("C · m²", "coulomb square meters", 1.0)]
    CoulombSquareMeter,
    #[linear("e · b", "elementary charge barns", 1.6021766339999998e-47)]
    ElementaryChargeBarn,
    #[linear(
        "e · a₀²",
        "atomic units of electric quadrupole moment",
        4.4865515246129999e-40
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricPermittivity)]
pub enum ElectricPermittivityUnit {
    #[linear("F/m", "farads per meter", 1.0)]
    FaradPerMeter,
    #[linear("ε₀", "vacuum electric permittivity", 8.8541878128000006e-12)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricalConductance)]
pub enum ElectricalConductanceUnit {
    #[linear("YS", "yottasiemens", 9.9999999999999998e+23)]
    Yottasiemens,
    #[linear("ZS", "zettasiemens", 1e+21)]
    Zettasiemens,
    #[linear("ES", "exasiemens", 1e+18)]
    Exasiemens,
    #[linear("PS", "petasiemens", 1000000000000000.0)]
    Petasiemens,
    #[linear("TS", "terasiemens", 1000000000000.0)]
    Terasiemens,
    #[linear("GS", "gigasiemens", 1000000000.0)]
    Gigasiemens,
    #[linear("MS", "megasiemens", 1000000.0)]
    Megasiemens,
    #[linear("kS", "kilosiemens", 1000.0)]
    Kilosiemens,
    #[linear("hS", "hectosiemens", 100.0)]
    Hectosiemens,
    #[linear("daS", "decasiemens", 10.0)]
    Decasiemens,
    #[linear("S", "siemens", 1.0)]
    Siemens,
    #[linear("℧", "mhos", 1.0)]
    Mho,
    #[linear("dS", "decisiemens", 0.10000000000000001)]
    Decisiemens,
    #[linear("cS", "centisiemens", 0.01)]
    Centisiemens,
    #[linear("mS", "millisiemens", 0.001)]
    Millisiemens,
    #[linear("µS", "microsiemens", 9.9999999999999995e-07)]
    Microsiemens,
    #[linear("nS", "nanosiemens", 1.0000000000000001e-09)]
    Nanosiemens,
    #[linear("pS", "picosiemens", 9.9999999999999998e-13)]
    Picosiemens,
    #[linear("fS", "femtosiemens", 1.0000000000000001e-15)]
    Femtosiemens,
    #[linear("aS", "attosiemens", 1.0000000000000001e-18)]
    Attosiemens,
    #[linear("zS", "zeptosiemens", 9.9999999999999991e-22)]
    Zeptosiemens,
    #[linear("yS", "yoctosiemens", 9.9999999999999992e-25)]
    Yoctosiemens,
    #[linear("abmho", "abmhos", 1000000000.0)]
    Abmho,
    #[linear("abS", "abmsiemens", 1000000000.0)]
    Absiemens,
    #[linear("statS", "statsiemens", 1.1126499999999999e-12)]
    Statsiemens,
    #[linear("statmho", "statmhos", 1.1126499999999999e-12)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricalConductivity)]
pub enum ElectricalConductivityUnit {
    #[linear("S/m", "siemens per meter", 1.0)]
    SiemensPerMeter,
    #[linear("S/cm", "siemens per centimeter", 100.0)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricalMobility)]
pub enum ElectricalMobilityUnit {
    #[linear("m²/(V · s)", "square meters per volt second", 1.0)]
    SquareMeterPerVoltSecond,
    #[linear("cm²/(V · s)", "square centimeters per volt second", 0.0001)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricalResistance)]
pub enum ElectricalResistanceUnit {
    #[linear("YΩ", "yottaohms", 9.9999999999999998e+23)]
    Yottaohm,
    #[linear("ZΩ", "zettaohms", 1e+21)]
    Zettaohm,
    #[linear("EΩ", "exaohms", 1e+18)]
    Exaohm,
    #[linear("PΩ", "petaohms", 1000000000000000.0)]
    Petaohm,
    #[linear("TΩ", "teraohms", 1000000000000.0)]
    Teraohm,
    #[linear("GΩ", "gigaohms", 1000000000.0)]
    Gigaohm,
    #[linear("MΩ", "megaohms", 1000000.0)]
    Megaohm,
    #[linear("kΩ", "kiloohms", 1000.0)]
    Kiloohm,
    #[linear("hΩ", "hectoohms", 100.0)]
    Hectoohm,
    #[linear("daΩ", "decaohms", 10.0)]
    Decaohm,
    #[linear("Ω", "ohms", 1.0)]
    Ohm,
    #[linear("dΩ", "deciohms", 0.10000000000000001)]
    Deciohm,
    #[linear("cΩ", "centiohms", 0.01)]
    Centiohm,
    #[linear("mΩ", "milliohms", 0.001)]
    Milliohm,
    #[linear("µΩ", "microohms", 9.9999999999999995e-07)]
    Microohm,
    #[linear("nΩ", "nanoohms", 1.0000000000000001e-09)]
    Nanoohm,
    #[linear("pΩ", "picoohms", 9.9999999999999998e-13)]
    Picoohm,
    #[linear("fΩ", "femtoohms", 1.0000000000000001e-15)]
    Femtoohm,
    #[linear("aΩ", "attoohms", 1.0000000000000001e-18)]
    Attoohm,
    #[linear("zΩ", "zeptoohms", 9.9999999999999991e-22)]
    Zeptoohm,
    #[linear("yΩ", "yoctoohms", 9.9999999999999992e-25)]
    Yoctoohm,
    #[linear("abΩ", "abohms", 1.0000000000000001e-09)]
    Abohm,
    #[linear("statΩ", "statohms", 898755291711.5481)]
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

#[derive(Unit, Copy, Clone)]
#[dim(ElectricalResistivity)]
pub enum ElectricalResistivityUnit {
    #[linear("YΩ · m", "yottaohm meters", 9.9999999999999998e+23)]
    YottaohmMeter,
    #[linear("ZΩ · m", "zettaohm meters", 1e+21)]
    ZettaohmMeter,
    #[linear("EΩ · m", "exaohm meters", 1e+18)]
    ExaohmMeter,
    #[linear("PΩ · m", "petaohm meters", 1000000000000000.0)]
    PetaohmMeter,
    #[linear("TΩ · m", "teraohm meters", 1000000000000.0)]
    TeraohmMeter,
    #[linear("GΩ · m", "gigaohm meters", 1000000000.0)]
    GigaohmMeter,
    #[linear("MΩ · m", "megaohm meters", 1000000.0)]
    MegaohmMeter,
    #[linear("kΩ · m", "kiloohm meters", 1000.0)]
    KiloohmMeter,
    #[linear("hΩ · m", "hectoohm meters", 100.0)]
    HectoohmMeter,
    #[linear("daΩ · m", "decaohm meters", 10.0)]
    DecaohmMeter,
    #[linear("Ω · m", "ohm meters", 1.0)]
    OhmMeter,
    #[linear("dΩ · m", "deciohm meters", 0.10000000000000001)]
    DeciohmMeter,
    #[linear("cΩ · m", "centiohm meters", 0.01)]
    CentiohmMeter,
    #[linear("mΩ · m", "milliohm meters", 0.001)]
    MilliohmMeter,
    #[linear("µΩ · m", "microohm meters", 9.9999999999999995e-07)]
    MicroohmMeter,
    #[linear("nΩ · m", "nanoohm meters", 1.0000000000000001e-09)]
    NanoohmMeter,
    #[linear("pΩ · m", "picoohm meters", 9.9999999999999998e-13)]
    PicoohmMeter,
    #[linear("fΩ · m", "femtoohm meters", 1.0000000000000001e-15)]
    FemtoohmMeter,
    #[linear("aΩ · m", "attoohm meters", 1.0000000000000001e-18)]
    AttoohmMeter,
    #[linear("zΩ · m", "zeptoohm meters", 9.9999999999999991e-22)]
    ZeptoohmMeter,
    #[linear("yΩ · m", "yoctoohm meters", 9.9999999999999992e-25)]
    YoctoohmMeter,
    #[linear("abΩ · m", "abohm meters", 1.0000000000000001e-09)]
    AbohmMeter,
    #[linear("statΩ · m", "statohm meters", 898755291711.5481)]
    StatohmMeter,
    #[linear("Ω · cm", "ohm centimeters", 0.01)]
    OhmCentimeter,
    #[linear("abΩ · cm", "abohm centimeters", 1.0000000000000001e-11)]
    AbohmCentimeter,
    #[linear("statΩ · cm", "statohm centimeters", 8987552917.1154804)]
    StatohmCentimeter,
    #[linear("Ω · in", "ohm inches", 0.025399999999999999)]
    OhmInch,
    #[linear("Ω · ft", "ohm feet", 0.30480000000000002)]
    OhmFoot,
    #[linear("Ω · yd", "ohm yards", 0.91439999999999999)]
    OhmYard,
    #[linear(
        "Ω · mm²/m",
        "ohm square millimeters per meter",
        9.9999999999999995e-07
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

#[derive(Unit, Copy, Clone)]
#[dim(Capacitance)]
pub enum CapacitanceUnit {
    #[linear("YF", "yottafarads", 9.9999999999999998e+23)]
    Yottafarad,
    #[linear("ZF", "zettafarads", 1e+21)]
    Zettafarad,
    #[linear("EF", "exafarads", 1e+18)]
    Exafarad,
    #[linear("PF", "petafarads", 1000000000000000.0)]
    Petafarad,
    #[linear("TF", "terafarads", 1000000000000.0)]
    Terafarad,
    #[linear("GF", "gigafarads", 1000000000.0)]
    Gigafarad,
    #[linear("MF", "megafarads", 1000000.0)]
    Megafarad,
    #[linear("kF", "kilofarads", 1000.0)]
    Kilofarad,
    #[linear("hF", "hectofarads", 100.0)]
    Hectofarad,
    #[linear("daF", "decafarads", 10.0)]
    Decafarad,
    #[linear("F", "farads", 1.0)]
    Farad,
    #[linear("dF", "decifarads", 0.10000000000000001)]
    Decifarad,
    #[linear("cF", "centifarads", 0.01)]
    Centifarad,
    #[linear("mF", "millifarads", 0.001)]
    Millifarad,
    #[linear("µF", "microfarads", 9.9999999999999995e-07)]
    Microfarad,
    #[linear("nF", "nanofarads", 1.0000000000000001e-09)]
    Nanofarad,
    #[linear("pF", "picofarads", 9.9999999999999998e-13)]
    Picofarad,
    #[linear("fF", "femtofarads", 1.0000000000000001e-15)]
    Femtofarad,
    #[linear("aF", "attofarads", 1.0000000000000001e-18)]
    Attofarad,
    #[linear("zF", "zeptofarads", 9.9999999999999991e-22)]
    Zeptofarad,
    #[linear("yF", "yoctofarads", 9.9999999999999992e-25)]
    Yoctofarad,
    #[linear("abF", "abfarads", 1000000000.0)]
    Abfarad,
    #[linear("statF", "statfarads", 1.1126499999999999e-12)]
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

#[derive(Unit, Copy, Clone)]
#[dim(Inductance)]
pub enum InductanceUnit {
    #[linear("YH", "yottahenries", 9.9999999999999998e+23)]
    Yottahenry,
    #[linear("ZH", "zettahenries", 1e+21)]
    Zettahenry,
    #[linear("EH", "exahenries", 1e+18)]
    Exahenry,
    #[linear("PH", "petahenries", 1000000000000000.0)]
    Petahenry,
    #[linear("TH", "terahenries", 1000000000000.0)]
    Terahenry,
    #[linear("GH", "gigahenries", 1000000000.0)]
    Gigahenry,
    #[linear("MH", "megahenries", 1000000.0)]
    Megahenry,
    #[linear("kH", "kilohenries", 1000.0)]
    Kilohenry,
    #[linear("hH", "hectohenries", 100.0)]
    Hectohenry,
    #[linear("daH", "decahenries", 10.0)]
    Decahenry,
    #[linear("H", "henries", 1.0)]
    Henry,
    #[linear("dH", "decihenries", 0.10000000000000001)]
    Decihenry,
    #[linear("cH", "centihenries", 0.01)]
    Centihenry,
    #[linear("mH", "millihenries", 0.001)]
    Millihenry,
    #[linear("µH", "microhenries", 9.9999999999999995e-07)]
    Microhenry,
    #[linear("nH", "nanohenries", 1.0000000000000001e-09)]
    Nanohenry,
    #[linear("pH", "picohenries", 9.9999999999999998e-13)]
    Picohenry,
    #[linear("fH", "femtohenries", 1.0000000000000001e-15)]
    Femtohenry,
    #[linear("aH", "attohenries", 1.0000000000000001e-18)]
    Attohenry,
    #[linear("zH", "zeptohenries", 9.9999999999999991e-22)]
    Zeptohenry,
    #[linear("yH", "yoctohenries", 9.9999999999999992e-25)]
    Yoctohenry,
    #[linear("abH", "abhenries", 1.0000000000000001e-09)]
    Abhenry,
    #[linear("statH", "stathenries", 898755291711.5481)]
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

#[derive(Unit, Copy, Clone)]
#[dim(MagneticFieldStrength)]
pub enum MagneticFieldStrengthUnit {
    #[linear("A/m", "amperes per meter", 1.0)]
    AmperePerMeter,
    #[linear("A/cm", "amperes per centimeter", 100.0)]
    AmperePerCentimeter,
    #[linear("A/mm", "amperes per millimeter", 1000.0)]
    AmperePerMillimeter,
    #[linear("A/μm", "amperes per micrometer", 1000000.0)]
    AmperePerMicrometer,
    #[linear("Oe", "oersteds", 79.577471545947674)]
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

#[derive(Unit, Copy, Clone)]
#[dim(MagneticFlux)]
pub enum MagneticFluxUnit {
    #[linear("YWb", "yottawebers", 9.9999999999999998e+23)]
    Yottaweber,
    #[linear("ZWb", "zettawebers", 1e+21)]
    Zettaweber,
    #[linear("EWb", "exawebers", 1e+18)]
    Exaweber,
    #[linear("PWb", "petawebers", 1000000000000000.0)]
    Petaweber,
    #[linear("TWb", "terawebers", 1000000000000.0)]
    Teraweber,
    #[linear("GWb", "gigawebers", 1000000000.0)]
    Gigaweber,
    #[linear("MWb", "megawebers", 1000000.0)]
    Megaweber,
    #[linear("kWb", "kilowebers", 1000.0)]
    Kiloweber,
    #[linear("hWb", "hectowebers", 100.0)]
    Hectoweber,
    #[linear("daWb", "decawebers", 10.0)]
    Decaweber,
    #[linear("Wb", "webers", 1.0)]
    Weber,
    #[linear("dWb", "deciwebers", 0.10000000000000001)]
    Deciweber,
    #[linear("cWb", "centiwebers", 0.01)]
    Centiweber,
    #[linear("mWb", "milliwebers", 0.001)]
    Milliweber,
    #[linear("µWb", "microwebers", 9.9999999999999995e-07)]
    Microweber,
    #[linear("nWb", "nanowebers", 1.0000000000000001e-09)]
    Nanoweber,
    #[linear("pWb", "picowebers", 9.9999999999999998e-13)]
    Picoweber,
    #[linear("fWb", "femtowebers", 1.0000000000000001e-15)]
    Femtoweber,
    #[linear("aWb", "attowebers", 1.0000000000000001e-18)]
    Attoweber,
    #[linear("zWb", "zeptowebers", 9.9999999999999991e-22)]
    Zeptoweber,
    #[linear("yWb", "yoctowebers", 9.9999999999999992e-25)]
    Yoctoweber,
    #[linear("Mx", "maxwells", 1e-08)]
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

#[derive(Unit, Copy, Clone)]
#[dim(MagneticFluxDensity)]
pub enum MagneticFluxDensityUnit {
    #[linear("YT", "yottateslas", 9.9999999999999998e+23)]
    Yottatesla,
    #[linear("ZT", "zettateslas", 1e+21)]
    Zettatesla,
    #[linear("ET", "exateslas", 1e+18)]
    Exatesla,
    #[linear("PT", "petateslas", 1000000000000000.0)]
    Petatesla,
    #[linear("TT", "terateslas", 1000000000000.0)]
    Teratesla,
    #[linear("GT", "gigateslas", 1000000000.0)]
    Gigatesla,
    #[linear("MT", "megateslas", 1000000.0)]
    Megatesla,
    #[linear("kT", "kiloteslas", 1000.0)]
    Kilotesla,
    #[linear("hT", "hectoteslas", 100.0)]
    Hectotesla,
    #[linear("daT", "decateslas", 10.0)]
    Decatesla,
    #[linear("T", "teslas", 1.0)]
    Tesla,
    #[linear("dT", "deciteslas", 0.10000000000000001)]
    Decitesla,
    #[linear("cT", "centiteslas", 0.01)]
    Centitesla,
    #[linear("mT", "milliteslas", 0.001)]
    Millitesla,
    #[linear("µT", "microteslas", 9.9999999999999995e-07)]
    Microtesla,
    #[linear("nT", "nanoteslas", 1.0000000000000001e-09)]
    Nanotesla,
    #[linear("pT", "picoteslas", 9.9999999999999998e-13)]
    Picotesla,
    #[linear("fT", "femtoteslas", 1.0000000000000001e-15)]
    Femtotesla,
    #[linear("aT", "attoteslas", 1.0000000000000001e-18)]
    Attotesla,
    #[linear("zT", "zeptoteslas", 9.9999999999999991e-22)]
    Zeptotesla,
    #[linear("yT", "yoctoteslas", 9.9999999999999992e-25)]
    Yoctotesla,
    #[linear("γ", "gammas", 1.0000000000000001e-09)]
    Gamma,
    #[linear("G", "gauss", 0.0001)]
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

#[derive(Unit, Copy, Clone)]
#[dim(MagneticMoment)]
pub enum MagneticMomentUnit {
    #[linear("A · m²", "ampere square meters", 1.0)]
    AmpereSquareMeter,
    #[linear("J/T", "joules per tesla", 1.0)]
    JoulePerTesla,
    #[linear("N · m/T", "newton meters per tesla", 1.0)]
    NewtonMeterPerTesla,
    #[linear("erg/G", "ergs per gauss", 0.0009999999999999998)]
    ErgPerGauss,
    #[linear("µ(Bohr)", "Bohr magnetons", 9.2740100783000004e-24)]
    BohrMagneton,
    #[linear("μ(Nuclear)", "nuclear magnetons", 5.0507837461000003e-27)]
    NuclearMagneton,
    #[linear(
        "ħ · e/mₑ",
        " atomic units of magnetic dipole moment",
        1.8548020156600001e-23
    )]
    AtomicUnitOfMagneticDipoleMoment,
    #[linear("μ(deuteron)", "deuteron magnetic moments", 4.3307350939999997e-27)]
    DeuteronMagneticMoment,
    #[linear("μ(proton)", "proton magnetic moments", 1.4106067973599999e-26)]
    ProtonMagneticMoment,
    #[linear("μ’(proton)", "shielded proton magnetic moments", 1.41057056e-26)]
    ShieldedProtonMagneticMoment,
    #[linear("μ(triton)", "triton magnetic moments", 1.5046095202000001e-26)]
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

#[derive(Unit, Copy, Clone)]
#[dim(MagneticPermeability)]
pub enum MagneticPermeabilityUnit {
    #[linear("H/m", "henrys per meter", 1.0)]
    HenryPerMeter,
    #[linear("µ₀", "vacuum magnetic permeability", 1.2566370621199999e-06)]
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

#[derive(Unit, Copy, Clone)]
#[dim(GyromagneticRatio)]
pub enum GyromagneticRatioUnit {
    #[linear("YHz/T", "yottahertz per tesla", 9.9999999999999998e+23)]
    YottahertzPerTesla,
    #[linear("ZHz/T", "zettahertz per tesla", 1e+21)]
    ZettahertzPerTesla,
    #[linear("EHz/T", "exahertz per tesla", 1e+18)]
    ExahertzPerTesla,
    #[linear("PHz/T", "petahertz per tesla", 1000000000000000.0)]
    PetahertzPerTesla,
    #[linear("THz/T", "terahertz per tesla", 1000000000000.0)]
    TerahertzPerTesla,
    #[linear("GHz/T", "gigahertz per tesla", 1000000000.0)]
    GigahertzPerTesla,
    #[linear("MHz/T", "megahertz per tesla", 1000000.0)]
    MegahertzPerTesla,
    #[linear("kHz/T", "kilohertz per tesla", 1000.0)]
    KilohertzPerTesla,
    #[linear("hHz/T", "hectohertz per tesla", 100.0)]
    HectohertzPerTesla,
    #[linear("daHz/T", "decahertz per tesla", 10.0)]
    DecahertzPerTesla,
    #[linear("Hz/T", "hertz per tesla", 1.0)]
    HertzPerTesla,
    #[linear("dHz/T", "decihertz per tesla", 0.10000000000000001)]
    DecihertzPerTesla,
    #[linear("cHz/T", "centihertz per tesla", 0.01)]
    CentihertzPerTesla,
    #[linear("mHz/T", "millihertz per tesla", 0.001)]
    MillihertzPerTesla,
    #[linear("µHz/T", "microhertz per tesla", 9.9999999999999995e-07)]
    MicrohertzPerTesla,
    #[linear("nHz/T", "nanohertz per tesla", 1.0000000000000001e-09)]
    NanohertzPerTesla,
    #[linear("pHz/T", "picohertz per tesla", 9.9999999999999998e-13)]
    PicohertzPerTesla,
    #[linear("fHz/T", "femtohertz per tesla", 1.0000000000000001e-15)]
    FemtohertzPerTesla,
    #[linear("aHz/T", "attohertz per tesla", 1.0000000000000001e-18)]
    AttohertzPerTesla,
    #[linear("zHz/T", "zeptohertz per tesla", 9.9999999999999991e-22)]
    ZeptohertzPerTesla,
    #[linear("yHz/T", "yoctohertz per tesla", 9.9999999999999992e-25)]
    YoctohertzPerTesla,
    #[linear("rad/(s · T)", "radians per second tesla", 1.0)]
    RadianPerSecondTesla,
    #[linear("γₚ", "proton gyromagnetic ratio", 267522187.08000001)]
    ProtonGyromagneticRatio,
    #[linear("γₙ", "neutron gyromagnetic ratio", 183247174.0)]
    NeutronGyromagneticRatio,
    #[linear("γₑ", "electron gyromagnetic ratio", 176085962784.0)]
    ElectronGyromagneticRatio,
    #[linear("γ'ₚ", "shielded proton gyromagnetic ratio", 267515319.40000001)]
    ShieldedProtonGyromagneticRatio,
    #[linear("γ'ₕ", "shielded helion gyromagnetic ratio", 203789460.78)]
    ShieldedHelionGyromagneticRatio,
    #[linear("γ₂", "deuteron gyromagnetic ratio", 41066288.799999997)]
    DeuteronGyromagneticRatio,
    #[linear("γ₃", "triton gyromagnetic ratio", 285349844.0)]
    TritonGyromagneticRatio,
    #[linear("γ₇", "lithium7 gyromagnetic ratio", 103975264.0)]
    Lithium7GyromagneticRatio,
    #[linear("γ₁₁", "boron11 gyromagnetic ratio", 85838414.299999997)]
    Boron11GyromagneticRatio,
    #[linear("γ₁₃", "carbon13 gyromagnetic ratio", 67278754.599999994)]
    Carbon13GyromagneticRatio,
    #[linear("γ₁₄", "nitrogen14 gyromagnetic ratio", 19328792.199999999)]
    Nitrogen14GyromagneticRatio,
    #[linear("γ₁₉", "flourine19 gyromagnetic ratio", 251762483.0)]
    Flourine19GyromagneticRatio,
    #[linear("γ₂₃", "sodium23 gyromagnetic ratio", 70803541.700000003)]
    Sodium23GyromagneticRatio,
    #[linear("γ₃₁", "phosphorus31 gyromagnetic ratio", 108329419.0)]
    Phosphorus31GyromagneticRatio,
    #[linear("̅γₚ", "proton reduced gyromagnetic ratio", 42577478.461000003)]
    ProtonReducedGyromagneticRatio,
    #[linear("̅γₙ", "neutron reduced gyromagnetic ratio", 29164693.5)]
    NeutronReducedGyromagneticRatio,
    #[linear("̅γₑ", "electron reduced gyromagnetic ratio", 28024951386.099998)]
    ElectronReducedGyromagneticRatio,
    #[linear("γ'ₚ", "shielded proton reduced gyromagnetic ratio", 42576385.43)]
    ShieldedProtonReducedGyromagneticRatio,
    #[linear("γ'ₕ", "shielded helion reduced gyromagnetic ratio", 32434100.033)]
    ShieldedHelionReducedGyromagneticRatio,
    #[linear("̅γ₂", "deuteron reduced gyromagnetic ratio", 6535902.8499999996)]
    DeuteronReducedGyromagneticRatio,
    #[linear("̅γ₃", "triton reduced gyromagnetic ratio", 45414838.100000001)]
    TritonReducedGyromagneticRatio,
    #[linear("̅γ₇", "lithium7 reduced gyromagnetic ratio", 16548177.300000001)]
    Lithium7ReducedGyromagneticRatio,
    #[linear("̅γ₁₁", "boron11 reduced gyromagnetic ratio", 13661608.0)]
    Boron11ReducedGyromagneticRatio,
    #[linear("̅γ₁₃", "carbon13 reduced gyromagnetic ratio", 10707746.4)]
    Carbon13ReducedGyromagneticRatio,
    #[linear("̅γ₁₄", "nitrogen14 reduced gyromagnetic ratio", 3076272.8100000001)]
    Nitrogen14ReducedGyromagneticRatio,
    #[linear("̅γ₁₉", "flourine19 reduced gyromagnetic ratio", 40069243.700000003)]
    Flourine19ReducedGyromagneticRatio,
    #[linear("̅γ₂₃", "sodium23 reduced gyromagnetic ratio", 11268733.6)]
    Sodium23ReducedGyromagneticRatio,
    #[linear("̅γ₃₁", "phosphorus31 reduced gyromagnetic ratio", 17241162.5)]
    Phosphorus31ReducedGyromagneticRatio,
}
