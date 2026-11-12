//! The units of `british`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::british::consts::*;
use crate::british::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Area`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Area<N>: [const] Unit<Area<N>, N> + [const] Destruct)]
pub enum AreaUnit {
    #[default]
    #[unit("[acr_br]", "acre", by = Area::<N>::from(ACRE))]
    Acre,
}

/// The units of `Length`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Length<N>: [const] Unit<Length<N>, N> + [const] Destruct)]
pub enum LengthUnit {
    #[default]
    #[unit("[in_br]", "inch", by = Length::<N>::from(INCH))]
    Inch,
    #[unit("[ft_br]", "foot", by = Length::<N>::from(FOOT))]
    Foot,
    #[unit("[rd_br]", "rod", by = Length::<N>::from(ROD))]
    Rod,
    #[unit("[ch_br]", "Gunter's chain", by = Length::<N>::from(GUNTER_S_CHAIN))]
    GunterSChain,
    #[unit("[lk_br]", "link for Gunter's chain", by = Length::<N>::from(LINK_FOR_GUNTER_S_CHAIN))]
    LinkForGunterSChain,
    #[unit("[fth_br]", "fathom", by = Length::<N>::from(FATHOM))]
    Fathom,
    #[unit("[pc_br]", "pace", by = Length::<N>::from(PACE))]
    Pace,
    #[unit("[yd_br]", "yard", by = Length::<N>::from(YARD))]
    Yard,
    #[unit("[mi_br]", "mile", by = Length::<N>::from(MILE))]
    Mile,
    #[unit("[nmi_br]", "nautical mile", by = Length::<N>::from(NAUTICAL_MILE))]
    NauticalMile,
}

/// The units of `Velocity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Velocity<N>: [const] Unit<Velocity<N>, N> + [const] Destruct)]
pub enum VelocityUnit {
    #[default]
    #[unit("[kn_br]", "knot", by = Velocity::<N>::from(KNOT))]
    Knot,
}

/// The units of `Volume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Volume<N>: [const] Unit<Volume<N>, N> + [const] Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("[gal_br]", "gallon", by = Volume::<N>::from(GALLON))]
    Gallon,
    #[unit("[pk_br]", "peck", by = Volume::<N>::from(PECK))]
    Peck,
    #[unit("[bu_br]", "bushel", by = Volume::<N>::from(BUSHEL))]
    Bushel,
    #[unit("[qt_br]", "quart", by = Volume::<N>::from(QUART))]
    Quart,
    #[unit("[pt_br]", "pint", by = Volume::<N>::from(PINT))]
    Pint,
    #[unit("[gil_br]", "gill", by = Volume::<N>::from(GILL))]
    Gill,
    #[unit("[foz_br]", "fluid ounce", by = Volume::<N>::from(FLUID_OUNCE))]
    FluidOunce,
    #[unit("[fdr_br]", "fluid dram", by = Volume::<N>::from(FLUID_DRAM))]
    FluidDram,
    #[unit("[min_br]", "minim", by = Volume::<N>::from(MINIM))]
    Minim,
}
