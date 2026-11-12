//! The units of `intcust`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

use crate::Exact;
use crate::intcust::consts::*;
use crate::intcust::*;
use core::marker::Destruct;
use yadc::{Unit, num_traits::Num};

/// The units of `Area`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Area<N>: [const] Unit<Area<N>, N> + [const] Destruct)]
pub enum AreaUnit {
    #[default]
    #[unit("[sin_i]", "square inch", by = Area::<N>::from(SQUARE_INCH))]
    SquareInch,
    #[unit("[sft_i]", "square foot", by = Area::<N>::from(SQUARE_FOOT))]
    SquareFoot,
    #[unit("[syd_i]", "square yard", by = Area::<N>::from(SQUARE_YARD))]
    SquareYard,
    #[unit("[cml_i]", "circular mil", by = Area::<N>::from(CIRCULAR_MIL))]
    CircularMil,
}

/// The units of `DepthOfWater`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DepthOfWater<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where DepthOfWater<N>: [const] Unit<DepthOfWater<N>, N> + [const] Destruct)]
pub enum DepthOfWaterUnit {
    #[default]
    #[unit("[fth_i]", "fathom", by = DepthOfWater::<N>::from(FATHOM))]
    Fathom,
}

/// The units of `HeightOfHorses`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeightOfHorses<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where HeightOfHorses<N>: [const] Unit<HeightOfHorses<N>, N> + [const] Destruct)]
pub enum HeightOfHorsesUnit {
    #[default]
    #[unit("[hd_i]", "hand", by = HeightOfHorses::<N>::from(HAND))]
    Hand,
}

/// The units of `Length`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Length<N>: [const] Unit<Length<N>, N> + [const] Destruct)]
pub enum LengthUnit {
    #[default]
    #[unit("[in_i]", "inch", by = Length::<N>::from(INCH))]
    Inch,
    #[unit("[ft_i]", "foot", by = Length::<N>::from(FOOT))]
    Foot,
    #[unit("[yd_i]", "yard", by = Length::<N>::from(YARD))]
    Yard,
    #[unit("[mi_i]", "mile", by = Length::<N>::from(MILE))]
    Mile,
    #[unit("[nmi_i]", "nautical mile", by = Length::<N>::from(NAUTICAL_MILE))]
    NauticalMile,
    #[unit("[mil_i]", "mil", by = Length::<N>::from(MIL))]
    Mil,
}

/// The units of `Velocity`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Velocity<N>: [const] Unit<Velocity<N>, N> + [const] Destruct)]
pub enum VelocityUnit {
    #[default]
    #[unit("[kn_i]", "knot", by = Velocity::<N>::from(KNOT))]
    Knot,
}

/// The units of `Volume`, in any number type the constants convert to.
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N: Num + const From<Exact>> where Volume<N>: [const] Unit<Volume<N>, N> + [const] Destruct)]
pub enum VolumeUnit {
    #[default]
    #[unit("[cin_i]", "cubic inch", by = Volume::<N>::from(CUBIC_INCH))]
    CubicInch,
    #[unit("[cft_i]", "cubic foot", by = Volume::<N>::from(CUBIC_FOOT))]
    CubicFoot,
    #[unit("[cyd_i]", "cubic yard", by = Volume::<N>::from(CUBIC_YARD))]
    CubicYard,
    #[unit("[bf_i]", "board foot", by = Volume::<N>::from(BOARD_FOOT))]
    BoardFoot,
    #[unit("[cr_i]", "cord", by = Volume::<N>::from(CORD))]
    Cord,
}
