//! The quantities of `british` and their units.

#[allow(unused_imports)]
use crate::*;

// `Area` is `crate::Area`.

/// Acre (`[acr_br]`), a unit of `Area`: `n` of them is `n · 4046850049400269056e-15` base units.
pub const ACRE: Unit<
    {
        UcumUnit::scaled(
            "[acr_br]",
            Exact::dec(4046850049400269056, -15),
            METER_SQUARED.unit(),
        )
    },
> = Unit::ONE;

// `Length` is `crate::Length`.

/// Inch (`[in_br]`), a unit of `Length`: `n` of them is `n · 2539998e-8` base units.
pub const INCH: Unit<{ UcumUnit::scaled("[in_br]", Exact::dec(2539998, -8), METER.unit()) }> =
    Unit::ONE;

/// Foot (`[ft_br]`), a unit of `Length`: `n` of them is `n · 30479976e-8` base units.
pub const FOOT: Unit<{ UcumUnit::scaled("[ft_br]", Exact::dec(30479976, -8), METER.unit()) }> =
    Unit::ONE;

/// Rod (`[rd_br]`), a unit of `Length`: `n` of them is `n · 502919604e-8` base units.
pub const ROD: Unit<{ UcumUnit::scaled("[rd_br]", Exact::dec(502919604, -8), METER.unit()) }> =
    Unit::ONE;

/// Gunter's chain (`[ch_br]`), a unit of `Length`: `n` of them is `n · 2011678416e-8` base units.
pub const GUNTER_S_CHAIN: Unit<
    { UcumUnit::scaled("[ch_br]", Exact::dec(2011678416, -8), METER.unit()) },
> = Unit::ONE;

/// Link for Gunter's chain (`[lk_br]`), a unit of `Length`: `n` of them is `n · 2011678416e-10` base units.
pub const LINK_FOR_GUNTER_S_CHAIN: Unit<
    { UcumUnit::scaled("[lk_br]", Exact::dec(2011678416, -10), METER.unit()) },
> = Unit::ONE;

/// Fathom (`[fth_br]`), a unit of `Length`: `n` of them is `n · 182879856e-8` base units.
pub const FATHOM: Unit<{ UcumUnit::scaled("[fth_br]", Exact::dec(182879856, -8), METER.unit()) }> =
    Unit::ONE;

/// Pace (`[pc_br]`), a unit of `Length`: `n` of them is `n · 7619994e-7` base units.
pub const PACE: Unit<{ UcumUnit::scaled("[pc_br]", Exact::dec(7619994, -7), METER.unit()) }> =
    Unit::ONE;

/// Yard (`[yd_br]`), a unit of `Length`: `n` of them is `n · 91439928e-8` base units.
pub const YARD: Unit<{ UcumUnit::scaled("[yd_br]", Exact::dec(91439928, -8), METER.unit()) }> =
    Unit::ONE;

/// Mile (`[mi_br]`), a unit of `Length`: `n` of them is `n · 16093427328e-7` base units.
pub const MILE: Unit<{ UcumUnit::scaled("[mi_br]", Exact::dec(16093427328, -7), METER.unit()) }> =
    Unit::ONE;

/// Nautical mile (`[nmi_br]`), a unit of `Length`: `n` of them is `n · 18531825408e-7` base units.
pub const NAUTICAL_MILE: Unit<
    { UcumUnit::scaled("[nmi_br]", Exact::dec(18531825408, -7), METER.unit()) },
> = Unit::ONE;

// `Velocity` is `crate::Velocity`.

/// Knot (`[kn_br]`), a unit of `Velocity`: `n` of them is `n · 514772928e-9` base units.
pub const KNOT: Unit<
    {
        UcumUnit::scaled(
            "[kn_br]",
            Exact::dec(514772928, -9),
            METER_PER_SECOND.unit(),
        )
    },
> = Unit::ONE;

// `Volume` is `crate::Volume`.

/// Gallon (`[gal_br]`), a unit of `Volume`: `n` of them is `n · 454609e-8` base units.
pub const GALLON: Unit<
    { UcumUnit::scaled("[gal_br]", Exact::dec(454609, -8), METER_CUBED.unit()) },
> = Unit::ONE;

/// Peck (`[pk_br]`), a unit of `Volume`: `n` of them is `n · 909218e-8` base units.
pub const PECK: Unit<{ UcumUnit::scaled("[pk_br]", Exact::dec(909218, -8), METER_CUBED.unit()) }> =
    Unit::ONE;

/// Bushel (`[bu_br]`), a unit of `Volume`: `n` of them is `n · 3636872e-8` base units.
pub const BUSHEL: Unit<
    { UcumUnit::scaled("[bu_br]", Exact::dec(3636872, -8), METER_CUBED.unit()) },
> = Unit::ONE;

/// Quart (`[qt_br]`), a unit of `Volume`: `n` of them is `n · 11365225e-10` base units.
pub const QUART: Unit<
    { UcumUnit::scaled("[qt_br]", Exact::dec(11365225, -10), METER_CUBED.unit()) },
> = Unit::ONE;

/// Pint (`[pt_br]`), a unit of `Volume`: `n` of them is `n · 56826125e-11` base units.
pub const PINT: Unit<
    { UcumUnit::scaled("[pt_br]", Exact::dec(56826125, -11), METER_CUBED.unit()) },
> = Unit::ONE;

/// Gill (`[gil_br]`), a unit of `Volume`: `n` of them is `n · 1420653125e-13` base units.
pub const GILL: Unit<
    { UcumUnit::scaled("[gil_br]", Exact::dec(1420653125, -13), METER_CUBED.unit()) },
> = Unit::ONE;

/// Fluid ounce (`[foz_br]`), a unit of `Volume`: `n` of them is `n · 284130625e-13` base units.
pub const FLUID_OUNCE: Unit<
    { UcumUnit::scaled("[foz_br]", Exact::dec(284130625, -13), METER_CUBED.unit()) },
> = Unit::ONE;

/// Fluid dram (`[fdr_br]`), a unit of `Volume`: `n` of them is `n · 35516328125e-16` base units.
pub const FLUID_DRAM: Unit<
    { UcumUnit::scaled("[fdr_br]", Exact::dec(35516328125, -16), METER_CUBED.unit()) },
> = Unit::ONE;

/// Minim (`[min_br]`), a unit of `Volume`: `n` of them is `n · 454609 / 7680000000000` base units.
pub const MINIM: Unit<
    {
        UcumUnit::scaled(
            "[min_br]",
            Exact::int(454609) / Exact::int(7680000000000),
            METER_CUBED.unit(),
        )
    },
> = Unit::ONE;
