use crate::{ISQ, Kind, StaticSIQuantity};
use yadc::{Dimension, Unit};

pub type Angle = StaticSIQuantity<
    {
        ISQ {
            kind: Kind::Angle,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(Angle)]
pub enum AngleUnit {
    #[unit(1.0, "rad", "radians")]
    Radian,
    #[unit(6.2831853071795862, "r", "revolutions")]
    Revolution,
    #[unit(0.017453292519943295, "°", "degrees")]
    Degree,
    #[unit(0.015707963267948967, "gon", "gons")]
    Gon,
    #[unit(0.00098174770000000007, "mil", "mils")]
    Mil,
    #[unit(0.00029088820866572158, "′", "minutes")]
    Minute,
    #[unit(4.8481368110953598e-06, "″", "seconds")]
    Second,
}

pub type SolidAngle = StaticSIQuantity<
    {
        ISQ {
            kind: Kind::SolidAngle,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(SolidAngle)]
pub enum SolidAngleUnit {
    #[unit(1.0, "sr", "steradians")]
    Steradian,
    #[unit(12.566370614359172, "sp", "spats")]
    Spat,
    #[unit(0.00030461741978670862, "°²", "square degrees")]
    SquareDegree,
    #[unit(8.4615949940752387e-08, "′²", "square minutes")]
    SquareMinute,
    #[unit(2.3504430539097885e-11, "″²", "square seconds")]
    SquareSecond,
}

pub type AngularVelocity = StaticSIQuantity<
    {
        ISQ {
            s: -1,
            kind: Kind::Angle,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(AngularVelocity)]
pub enum AngularVelocityUnit {
    #[unit(1.0, "rad/s", "radians per second")]
    RadianPerSecond,
    #[unit(0.017453292519943295, "°/s", "degrees per second")]
    DegreePerSecond,
    #[unit(6.2831853071795862, "rps", "revolutions per second")]
    RevolutionPerSecond,
    #[unit(0.10471975511965977, "rpm", "revolutions per minute")]
    RevolutionPerMinute,
    #[unit(0.0017453292519943296, "rph", "revolutions per hour")]
    RevolutionPerHour,
}

pub type AngularAcceleration = StaticSIQuantity<
    {
        ISQ {
            s: -2,
            kind: Kind::Angle,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(AngularAcceleration)]
pub enum AngularAccelerationUnit {
    #[unit(1.0, "rad/s²", "radians per second squared")]
    RadianPerSecondSquared,
    #[unit(0.017453292519943295, "°/s²", "degrees per second squared")]
    DegreePerSecondSquared,
}

pub type AngularJerk = StaticSIQuantity<
    {
        ISQ {
            s: -3,
            kind: Kind::Angle,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(AngularJerk)]
pub enum AngularJerkUnit {
    #[unit(1.0, "rad/s³", "radians per second cubed")]
    RadianPerSecondCubed,
    #[unit(0.017453292519943295, "°/s³", "degrees per second cubed")]
    DegreePerSecondCubed,
}

pub type AngularAbsement = StaticSIQuantity<
    {
        ISQ {
            s: 1,
            kind: Kind::Angle,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(AngularAbsement)]
pub enum AngularAbsementUnit {
    #[unit(1.0, "rad · s", "radian seconds")]
    RadianSecond,
    #[unit(0.017453292519943295, "° · s", "degree seconds")]
    DegreeSecond,
}

pub type AngularMomentum = StaticSIQuantity<
    {
        ISQ {
            m: 2,
            kg: 1,
            s: -1,
            kind: Kind::Angle,
            ..ISQ::ZERO
        }
    },
>;

#[derive(Copy, Clone, Unit)]
#[dim(AngularMomentum)]
pub enum AngularMomentumUnit {
    #[unit(1.0, "kg · m²/s", "kilogram square meters per second")]
    KilogramSquareMeterPerSecond,
    #[unit(9.9999999999999998e+23, "YN · m · s", "yottanewton meter seconds")]
    YottanewtonMeterSecond,
    #[unit(1e+21, "ZN · m · s", "zettanewton meter seconds")]
    ZettanewtonMeterSecond,
    #[unit(1e+18, "EN · m · s", "exanewton meter seconds")]
    ExanewtonMeterSecond,
    #[unit(1000000000000000.0, "PN · m · s", "petanewton meter seconds")]
    PetanewtonMeterSecond,
    #[unit(1000000000000.0, "TN · m · s", "teranewton meter seconds")]
    TeranewtonMeterSecond,
    #[unit(1000000000.0, "GN · m · s", "giganewton meter seconds")]
    GiganewtonMeterSecond,
    #[unit(1000000.0, "MN · m · s", "meganewton meter seconds")]
    MeganewtonMeterSecond,
    #[unit(1000.0, "kN · m · s", "kilonewton meter seconds")]
    KilonewtonMeterSecond,
    #[unit(100.0, "hN · m · s", "hectonewton meter seconds")]
    HectonewtonMeterSecond,
    #[unit(10.0, "daN · m · s", "decanewton meter seconds")]
    DecanewtonMeterSecond,
    #[unit(1.0, "N · m · s", "newton meter seconds")]
    NewtonMeterSecond,
    #[unit(0.10000000000000001, "dN · m · s", "decinewton meter seconds")]
    DecinewtonMeterSecond,
    #[unit(0.01, "cN · m · s", "centinewton meter seconds")]
    CentinewtonMeterSecond,
    #[unit(0.001, "mN · m · s", "millinewton meter seconds")]
    MillinewtonMeterSecond,
    #[unit(9.9999999999999995e-07, "µN · m · s", "micronewton meter seconds")]
    MicronewtonMeterSecond,
    #[unit(1.0000000000000001e-09, "nN · m · s", "nanonewton meter seconds")]
    NanonewtonMeterSecond,
    #[unit(9.9999999999999998e-13, "pN · m · s", "piconewton meter seconds")]
    PiconewtonMeterSecond,
    #[unit(1.0000000000000001e-15, "fN · m · s", "femtonewton meter seconds")]
    FemtonewtonMeterSecond,
    #[unit(1.0000000000000001e-18, "aN · m · s", "attonewton meter seconds")]
    AttonewtonMeterSecond,
    #[unit(9.9999999999999991e-22, "zN · m · s", "zeptonewton meter seconds")]
    ZeptonewtonMeterSecond,
    #[unit(9.9999999999999992e-25, "yN · m · s", "yoctonewton meter seconds")]
    YoctonewtonMeterSecond,
    #[unit(9.9999999999999998e+23, "N · Ym · s", "newton yottameter seconds")]
    NewtonYottameterSecond,
    #[unit(1e+21, "N · Zm · s", "newton zettameter seconds")]
    NewtonZettameterSecond,
    #[unit(1e+18, "N · Em · s", "newton exameter seconds")]
    NewtonExameterSecond,
    #[unit(1000000000000000.0, "N · Pm · s", "newton petameter seconds")]
    NewtonPetameterSecond,
    #[unit(1000000000000.0, "N · Tm · s", "newton terameter seconds")]
    NewtonTerameterSecond,
    #[unit(1000000000.0, "N · Gm · s", "newton gigameter seconds")]
    NewtonGigameterSecond,
    #[unit(1000000.0, "N · Mm · s", "newton megameter seconds")]
    NewtonMegameterSecond,
    #[unit(1000.0, "N · km · s", "newton kilometer seconds")]
    NewtonKilometerSecond,
    #[unit(100.0, "N · hm · s", "newton hectometer seconds")]
    NewtonHectometerSecond,
    #[unit(10.0, "N · dam · s", "newton decameter seconds")]
    NewtonDecameterSecond,
    #[unit(0.10000000000000001, "N · dm · s", "newton decimeter seconds")]
    NewtonDecimeterSecond,
    #[unit(0.01, "N · cm · s", "newton centimeter seconds")]
    NewtonCentimeterSecond,
    #[unit(0.001, "N · mm · s", "newton millimeter seconds")]
    NewtonMillimeterSecond,
    #[unit(9.9999999999999995e-07, "N · µm · s", "newton micrometer seconds")]
    NewtonMicrometerSecond,
    #[unit(1.0000000000000001e-09, "N · nm · s", "newton nanometer seconds")]
    NewtonNanometerSecond,
    #[unit(9.9999999999999998e-13, "N · pm · s", "newton picometer seconds")]
    NewtonPicometerSecond,
    #[unit(1.0000000000000001e-15, "N · fm · s", "newton femtometer seconds")]
    NewtonFemtometerSecond,
    #[unit(1.0000000000000001e-18, "N · am · s", "newton attometer seconds")]
    NewtonAttometerSecond,
    #[unit(9.9999999999999991e-22, "N · zm · s", "newton zeptometer seconds")]
    NewtonZeptometerSecond,
    #[unit(9.9999999999999992e-25, "N · ym · s", "newton yoctometer seconds")]
    NewtonYoctometerSecond,
}
