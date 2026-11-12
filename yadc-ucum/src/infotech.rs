use crate::{Dim, Exact, Qty};
use consts::*;
use yadc::Dimension;

pub type AmountOfInformation<N> = Qty<{ Dim::DIMENSIONLESS }, N>;

pub type SignalTransmissionRate<N> = Qty<
    {
        Dim {
            s: -1,
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

    /// The number one: the unit of dimension one.
    pub use crate::base::ONE;

    /// One `Bit` (`bit`), as an `AmountOfInformation`.
    pub const BIT: AmountOfInformation<Exact> = ONE;

    /// One `Byte` (`By`), as an `AmountOfInformation`.
    pub const BYTE: AmountOfInformation<Exact> = BIT * Exact::int(8);

    /// One `Baud` (`Bd`), as a `SignalTransmissionRate`.
    pub const BAUD: SignalTransmissionRate<Exact> = ONE / SECOND;
}

/// The units of `AmountOfInformation`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = AmountOfInformation<N>, number = N)]
#[unit(for<N> where AmountOfInformation<N>: [const] yadc::Unit<AmountOfInformation<N>, N> + [const] core::marker::Destruct, N: yadc::num_traits::Num + const From<Exact>)]
pub enum AmountOfInformationUnit {
    #[default]
    #[unit("bit", "bit", by = AmountOfInformation::<N>::from(BIT))]
    Bit,
    #[unit("By", "byte", by = AmountOfInformation::<N>::from(BYTE))]
    Byte,
}

/// The units of `SignalTransmissionRate`, for a menu, a config file, a parser: in any number
/// type the constants convert to (`LengthUnit::Foot.quantity(3.0)`).
#[derive_const(Default)]
#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[unit(of = SignalTransmissionRate<N>, number = N)]
#[unit(for<N> where N: yadc::num_traits::Num + const From<Exact>, SignalTransmissionRate<N>: [const] yadc::Unit<SignalTransmissionRate<N>, N> + [const] core::marker::Destruct)]
pub enum SignalTransmissionRateUnit {
    #[default]
    #[unit("Bd", "baud", by = SignalTransmissionRate::<N>::from(BAUD))]
    Baud,
}
