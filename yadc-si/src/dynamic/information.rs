//! The units of `information`'s quantities, chosen at run time: for a menu, a
//! config file, a parser.

#[allow(unused_imports)]
use crate::information::*;
#[allow(unused_imports)]
use crate::*;

/// The units of `Information`: a choice among them, printed and parsed as its
/// unit, and converted to it.
#[derive_const(Default)]
#[derive(
    const_derive_more::Values, const_derive_more::Variants, Copy, Clone, PartialEq, Eq, Hash, Debug,
)]
#[values(into(SIUnit))]
pub enum InformationUnit {
    #[value(BIT.unit())]
    Bit,
    #[default]
    #[value(BYTE.unit())]
    Byte,
    #[value(OCTET.unit())]
    Octet,
    #[value(NIBBLE.unit())]
    Nibble,
    #[value(CRUMB.unit())]
    Crumb,
    #[value(SHANNON.unit())]
    Shannon,
}
