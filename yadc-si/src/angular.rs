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

#[derive(Unit, Copy, Clone)]
#[dim(Angle)]
pub enum AngleUnit {
    #[linear("rad", "radians", 1.0)]
    Radian,
    #[linear("r", "revolutions", 6.2831853071795862)]
    Revolution,
    #[linear("°", "degrees", 0.017453292519943295)]
    Degree,
    #[linear("gon", "gons", 0.015707963267948967)]
    Gon,
    #[linear("mil", "mils", 0.00098174770000000007)]
    Mil,
    #[linear("′", "minutes", 0.00029088820866572158)]
    Minute,
    #[linear("″", "seconds", 4.8481368110953598e-06)]
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

#[derive(Unit, Copy, Clone)]
#[dim(SolidAngle)]
pub enum SolidAngleUnit {
    #[linear("sr", "steradians", 1.0)]
    Steradian,
    #[linear("sp", "spats", 12.566370614359172)]
    Spat,
    #[linear("°²", "square degrees", 0.00030461741978670862)]
    SquareDegree,
    #[linear("′²", "square minutes", 8.4615949940752387e-08)]
    SquareMinute,
    #[linear("″²", "square seconds", 2.3504430539097885e-11)]
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

#[derive(Unit, Copy, Clone)]
#[dim(AngularVelocity)]
pub enum AngularVelocityUnit {
    #[linear("rad/s", "radians per second", 1.0)]
    RadianPerSecond,
    #[linear("°/s", "degrees per second", 0.017453292519943295)]
    DegreePerSecond,
    #[linear("rps", "revolutions per second", 6.2831853071795862)]
    RevolutionPerSecond,
    #[linear("rpm", "revolutions per minute", 0.10471975511965977)]
    RevolutionPerMinute,
    #[linear("rph", "revolutions per hour", 0.0017453292519943296)]
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

#[derive(Unit, Copy, Clone)]
#[dim(AngularAcceleration)]
pub enum AngularAccelerationUnit {
    #[linear("rad/s²", "radians per second squared", 1.0)]
    RadianPerSecondSquared,
    #[linear("°/s²", "degrees per second squared", 0.017453292519943295)]
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

#[derive(Unit, Copy, Clone)]
#[dim(AngularJerk)]
pub enum AngularJerkUnit {
    #[linear("rad/s³", "radians per second cubed", 1.0)]
    RadianPerSecondCubed,
    #[linear("°/s³", "degrees per second cubed", 0.017453292519943295)]
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

#[derive(Unit, Copy, Clone)]
#[dim(AngularAbsement)]
pub enum AngularAbsementUnit {
    #[linear("rad · s", "radian seconds", 1.0)]
    RadianSecond,
    #[linear("° · s", "degree seconds", 0.017453292519943295)]
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

#[derive(Unit, Copy, Clone)]
#[dim(AngularMomentum)]
pub enum AngularMomentumUnit {
    #[linear("kg · m²/s", "kilogram square meters per second", 1.0)]
    KilogramSquareMeterPerSecond,
    #[linear("YN · m · s", "yottanewton meter seconds", 9.9999999999999998e+23)]
    YottanewtonMeterSecond,
    #[linear("ZN · m · s", "zettanewton meter seconds", 1e+21)]
    ZettanewtonMeterSecond,
    #[linear("EN · m · s", "exanewton meter seconds", 1e+18)]
    ExanewtonMeterSecond,
    #[linear("PN · m · s", "petanewton meter seconds", 1000000000000000.0)]
    PetanewtonMeterSecond,
    #[linear("TN · m · s", "teranewton meter seconds", 1000000000000.0)]
    TeranewtonMeterSecond,
    #[linear("GN · m · s", "giganewton meter seconds", 1000000000.0)]
    GiganewtonMeterSecond,
    #[linear("MN · m · s", "meganewton meter seconds", 1000000.0)]
    MeganewtonMeterSecond,
    #[linear("kN · m · s", "kilonewton meter seconds", 1000.0)]
    KilonewtonMeterSecond,
    #[linear("hN · m · s", "hectonewton meter seconds", 100.0)]
    HectonewtonMeterSecond,
    #[linear("daN · m · s", "decanewton meter seconds", 10.0)]
    DecanewtonMeterSecond,
    #[linear("N · m · s", "newton meter seconds", 1.0)]
    NewtonMeterSecond,
    #[linear("dN · m · s", "decinewton meter seconds", 0.10000000000000001)]
    DecinewtonMeterSecond,
    #[linear("cN · m · s", "centinewton meter seconds", 0.01)]
    CentinewtonMeterSecond,
    #[linear("mN · m · s", "millinewton meter seconds", 0.001)]
    MillinewtonMeterSecond,
    #[linear("µN · m · s", "micronewton meter seconds", 9.9999999999999995e-07)]
    MicronewtonMeterSecond,
    #[linear("nN · m · s", "nanonewton meter seconds", 1.0000000000000001e-09)]
    NanonewtonMeterSecond,
    #[linear("pN · m · s", "piconewton meter seconds", 9.9999999999999998e-13)]
    PiconewtonMeterSecond,
    #[linear("fN · m · s", "femtonewton meter seconds", 1.0000000000000001e-15)]
    FemtonewtonMeterSecond,
    #[linear("aN · m · s", "attonewton meter seconds", 1.0000000000000001e-18)]
    AttonewtonMeterSecond,
    #[linear("zN · m · s", "zeptonewton meter seconds", 9.9999999999999991e-22)]
    ZeptonewtonMeterSecond,
    #[linear("yN · m · s", "yoctonewton meter seconds", 9.9999999999999992e-25)]
    YoctonewtonMeterSecond,
    #[linear("N · Ym · s", "newton yottameter seconds", 9.9999999999999998e+23)]
    NewtonYottameterSecond,
    #[linear("N · Zm · s", "newton zettameter seconds", 1e+21)]
    NewtonZettameterSecond,
    #[linear("N · Em · s", "newton exameter seconds", 1e+18)]
    NewtonExameterSecond,
    #[linear("N · Pm · s", "newton petameter seconds", 1000000000000000.0)]
    NewtonPetameterSecond,
    #[linear("N · Tm · s", "newton terameter seconds", 1000000000000.0)]
    NewtonTerameterSecond,
    #[linear("N · Gm · s", "newton gigameter seconds", 1000000000.0)]
    NewtonGigameterSecond,
    #[linear("N · Mm · s", "newton megameter seconds", 1000000.0)]
    NewtonMegameterSecond,
    #[linear("N · km · s", "newton kilometer seconds", 1000.0)]
    NewtonKilometerSecond,
    #[linear("N · hm · s", "newton hectometer seconds", 100.0)]
    NewtonHectometerSecond,
    #[linear("N · dam · s", "newton decameter seconds", 10.0)]
    NewtonDecameterSecond,
    #[linear("N · dm · s", "newton decimeter seconds", 0.10000000000000001)]
    NewtonDecimeterSecond,
    #[linear("N · cm · s", "newton centimeter seconds", 0.01)]
    NewtonCentimeterSecond,
    #[linear("N · mm · s", "newton millimeter seconds", 0.001)]
    NewtonMillimeterSecond,
    #[linear("N · µm · s", "newton micrometer seconds", 9.9999999999999995e-07)]
    NewtonMicrometerSecond,
    #[linear("N · nm · s", "newton nanometer seconds", 1.0000000000000001e-09)]
    NewtonNanometerSecond,
    #[linear("N · pm · s", "newton picometer seconds", 9.9999999999999998e-13)]
    NewtonPicometerSecond,
    #[linear("N · fm · s", "newton femtometer seconds", 1.0000000000000001e-15)]
    NewtonFemtometerSecond,
    #[linear("N · am · s", "newton attometer seconds", 1.0000000000000001e-18)]
    NewtonAttometerSecond,
    #[linear("N · zm · s", "newton zeptometer seconds", 9.9999999999999991e-22)]
    NewtonZeptometerSecond,
    #[linear("N · ym · s", "newton yoctometer seconds", 9.9999999999999992e-25)]
    NewtonYoctometerSecond,
}
