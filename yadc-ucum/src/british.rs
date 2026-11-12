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

    /// One `Acre` (`[acr_br]`), as an `Area`.
    pub const ACRE: Area<Exact> = (METRE * METRE) * Exact::dec(4046850049400269056, -15);

    /// One `Inch` (`[in_br]`), as a `Length`.
    pub const INCH: Length<Exact> = METRE * Exact::dec(2539998, -8);

    /// One `Foot` (`[ft_br]`), as a `Length`.
    pub const FOOT: Length<Exact> = METRE * Exact::dec(30479976, -8);

    /// One `Rod` (`[rd_br]`), as a `Length`.
    pub const ROD: Length<Exact> = METRE * Exact::dec(502919604, -8);

    /// One `GunterSChain` (`[ch_br]`), as a `Length`.
    pub const GUNTER_S_CHAIN: Length<Exact> = METRE * Exact::dec(2011678416, -8);

    /// One `LinkForGunterSChain` (`[lk_br]`), as a `Length`.
    pub const LINK_FOR_GUNTER_S_CHAIN: Length<Exact> = METRE * Exact::dec(2011678416, -10);

    /// One `Fathom` (`[fth_br]`), as a `Length`.
    pub const FATHOM: Length<Exact> = METRE * Exact::dec(182879856, -8);

    /// One `Pace` (`[pc_br]`), as a `Length`.
    pub const PACE: Length<Exact> = METRE * Exact::dec(7619994, -7);

    /// One `Yard` (`[yd_br]`), as a `Length`.
    pub const YARD: Length<Exact> = METRE * Exact::dec(91439928, -8);

    /// One `Mile` (`[mi_br]`), as a `Length`.
    pub const MILE: Length<Exact> = METRE * Exact::dec(16093427328, -7);

    /// One `NauticalMile` (`[nmi_br]`), as a `Length`.
    pub const NAUTICAL_MILE: Length<Exact> = METRE * Exact::dec(18531825408, -7);

    /// One `Knot` (`[kn_br]`), as a `Velocity`.
    pub const KNOT: Velocity<Exact> = (METRE / SECOND) * Exact::dec(514772928, -9);

    /// One `Gallon` (`[gal_br]`), as a `Volume`.
    pub const GALLON: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(454609, -8);

    /// One `Peck` (`[pk_br]`), as a `Volume`.
    pub const PECK: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(909218, -8);

    /// One `Bushel` (`[bu_br]`), as a `Volume`.
    pub const BUSHEL: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(3636872, -8);

    /// One `Quart` (`[qt_br]`), as a `Volume`.
    pub const QUART: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(11365225, -10);

    /// One `Pint` (`[pt_br]`), as a `Volume`.
    pub const PINT: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(56826125, -11);

    /// One `Gill` (`[gil_br]`), as a `Volume`.
    pub const GILL: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(1420653125, -13);

    /// One `FluidOunce` (`[foz_br]`), as a `Volume`.
    pub const FLUID_OUNCE: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(284130625, -13);

    /// One `FluidDram` (`[fdr_br]`), as a `Volume`.
    pub const FLUID_DRAM: Volume<Exact> = (METRE * METRE * METRE) * Exact::dec(35516328125, -16);

    /// One `Minim` (`[min_br]`), as a `Volume`.
    pub const MINIM: Volume<Exact> =
        (METRE * METRE * METRE) * (Exact::int(454609) / Exact::int(7680000000000));
}

/// The units of `Area`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Area<N>, number = N)]
#[unit(for<N> where Area<N>: [const] yadc::Unit<Area<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AreaUnit {
    #[default]
    #[unit("[acr_br]", "acre", by = Area::<N>::from(ACRE))]
    Acre,
}

/// The units of `Length`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Length<N>, number = N)]
#[unit(for<N> where Length<N>: [const] yadc::Unit<Length<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
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

/// The units of `Velocity`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = Velocity<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, Velocity<N>: [const] yadc::Unit<Velocity<N>, N> + [const] core::marker::Destruct)]
pub enum VelocityUnit {
    #[default]
    #[unit("[kn_br]", "knot", by = Velocity::<N>::from(KNOT))]
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
