use crate::{Dim, Exact, Qty};
use consts::*;
use yadc::Dimension;

pub type Area<N> = Qty<
    {
        Dim {
            m: 2,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type DepthOfWater<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type HeightOfHorses<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Length<N> = Qty<
    {
        Dim {
            m: 1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Velocity<N> = Qty<
    {
        Dim {
            m: 1,
            s: -1,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

pub type Volume<N> = Qty<
    {
        Dim {
            m: 3,
            ..Dim::DIMENSIONLESS
        }
    },
    N,
>;

/// The constants of this module: the base constants of its quantities
/// (re-exported), then one per named unit, each defined exactly from them
/// (`INCH` is `0.0254 * METRE`). A unit written as others is not one:
/// `SLUG / CUBIC_FOOT`, `KILO * METRE` are exact too. Converting rounds,
/// once, where it is written - in a `const`, by the compiler:
/// `const MPH: Velocity<f64> = (MILE / HOUR).into();`, then `MPH(60.0)`.
pub mod consts {
    use super::*;
    use crate::base::SECOND;

    /// The metre, UCUM's base unit of length.
    pub use crate::base::METRE;

    /// One `SquareInch` (`[sin_i]`), as an `Area`.
    pub const SQUARE_INCH: Area<Exact> = (METRE * METRE) * Exact::dec(64516, -8);

    /// One `SquareFoot` (`[sft_i]`), as an `Area`.
    pub const SQUARE_FOOT: Area<Exact> = (METRE * METRE) * Exact::dec(9290304, -8);

    /// One `SquareYard` (`[syd_i]`), as an `Area`.
    pub const SQUARE_YARD: Area<Exact> = (METRE * METRE) * Exact::dec(83612736, -8);

    /// One `CircularMil` (`[cml_i]`), as an `Area`.
    pub const CIRCULAR_MIL: Area<Exact> = (METRE * METRE) * (Exact::dec(16129, -14) * Exact::PI);

    /// One `Fathom` (`[fth_i]`), as a `DepthOfWater`.
    pub const FATHOM: DepthOfWater<Exact> = METRE * Exact::dec(18288, -4);

    /// One `Hand` (`[hd_i]`), as a `HeightOfHorses`.
    pub const HAND: HeightOfHorses<Exact> = METRE * Exact::dec(1016, -4);

    /// One `Inch` (`[in_i]`), as a `Length`.
    pub const INCH: Length<Exact> = METRE * Exact::dec(254, -4);

    /// One `Foot` (`[ft_i]`), as a `Length`.
    pub const FOOT: Length<Exact> = METRE * Exact::dec(3048, -4);

    /// One `Yard` (`[yd_i]`), as a `Length`.
    pub const YARD: Length<Exact> = METRE * Exact::dec(9144, -4);

    /// One `Mile` (`[mi_i]`), as a `Length`.
    pub const MILE: Length<Exact> = METRE * Exact::dec(1609344, -3);

    /// One `NauticalMile` (`[nmi_i]`), as a `Length`.
    pub const NAUTICAL_MILE: Length<Exact> = METRE * Exact::int(1852);

    /// One `Mil` (`[mil_i]`), as a `Length`.
    pub const MIL: Length<Exact> = METRE * Exact::dec(254, -7);

    /// One `Knot` (`[kn_i]`), as a `Velocity`.
    pub const KNOT: Velocity<Exact> = (METRE / SECOND) * (Exact::int(463) / Exact::int(900));

    /// One `CubicInch` (`[cin_i]`), as a `Volume`.
    pub const CUBIC_INCH: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(16387064, -12);

    /// One `CubicFoot` (`[cft_i]`), as a `Volume`.
    pub const CUBIC_FOOT: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(28316846592, -12);

    /// One `CubicYard` (`[cyd_i]`), as a `Volume`.
    pub const CUBIC_YARD: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(764554857984, -12);

    /// One `BoardFoot` (`[bf_i]`), as a `Volume`.
    pub const BOARD_FOOT: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(2359737216, -12);

    /// One `Cord` (`[cr_i]`), as a `Volume`.
    pub const CORD: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(3624556363776, -12);
}

/// The units of `Area`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N> where Area<N>: [const] yadc::Unit<Area<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `DepthOfWater`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = DepthOfWater<N>, number = N)]
#[unit(for<N> where DepthOfWater<N>: [const] yadc::Unit<DepthOfWater<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum DepthOfWaterUnit {
    #[default]
    #[unit("[fth_i]", "fathom", by = DepthOfWater::<N>::from(FATHOM))]
    Fathom,
}

/// The units of `HeightOfHorses`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = HeightOfHorses<N>, number = N)]
#[unit(for<N> where HeightOfHorses<N>: [const] yadc::Unit<HeightOfHorses<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum HeightOfHorsesUnit {
    #[default]
    #[unit("[hd_i]", "hand", by = HeightOfHorses::<N>::from(HAND))]
    Hand,
}

/// The units of `Length`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N> where Length<N>: [const] yadc::Unit<Length<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `Velocity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Velocity<N>: [const] yadc::Unit<Velocity<N>, N> + [const] core::marker::Destruct)]
pub enum VelocityUnit {
    #[default]
    #[unit("[kn_i]", "knot", by = Velocity::<N>::from(KNOT))]
    Knot,
}

/// The units of `Volume`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Volume<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Volume<N>: [const] yadc::Unit<Volume<N>, N> + [const] core::marker::Destruct)]
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
