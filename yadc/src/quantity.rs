//! Quantities: values that add, scale and measure themselves.

use crate::unit::Unit;
use core::ops::{Add, Div, Mul};

/// A quantity: a value you can add, scale and compare, with itself as a
/// unit.
///
/// On the whiteboard, exactly the operations listed: `a + b` is a quantity
/// of the same kind, `a * n` and `a / n` scale it by its
/// [`Number`](Self::Number)s, quantities of one kind are ordered, and any
/// non-zero quantity measures the others - `BAR.value(p)` is how many
/// bars `p` is, `BAR.quantity(2.0)` two of them. `Copy` like a
/// number.
///
/// It is not a vector space and asks for no more than that: there is no zero
/// (numbers enter and leave a system through its units, not through a zero),
/// and no `Sub<Output = Self>` - the difference of two absolute temperatures
/// is a temperature *step*, so a difference is not always the same kind of
/// quantity.
///
/// `#[derive(Quantity)]` on a static family implements it - with its
/// supertraits, the operators and [`Unit<Self, N>`] - for every member that
/// can be scaled. A point (an altitude above sea level: its kind is affine),
/// a level, or a label that cannot be added to itself is not a `Quantity`,
/// nor a unit: it is read with a unit that counts steps from a zero -
/// `SEA_LEVEL + FOOT.quantity(n)` feet above sea level.
#[diagnostic::on_unimplemented(
    message = "not a `Quantity`: this value cannot be scaled",
    label = "not a `Quantity`",
    note = "a point (its kind is affine), a level, or a label that cannot be added to itself \
            cannot be scaled, so it is not a `Quantity` nor a unit of itself"
)]
pub const trait Quantity:
    Copy
    + PartialOrd
    + [const] Add<Output = Self>
    + [const] Mul<Self::Number, Output = Self>
    + [const] Div<Self::Number, Output = Self>
    + [const] Unit<Self, Self::Number>
{
    /// The numbers it scales by and is counted in: its number type.
    type Number;
}
