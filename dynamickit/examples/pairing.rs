//! Relating two closed sets: each colour family paints one shape.
//!
//! Circles come in reds, squares in blues. That relation is a fact about
//! the types, so it is written as one: `impl Paints for Red { type Shape =
//! Circle; }`. With it, one visitor of `Colour` does the work of a `match`
//! over both enums:
//!
//! - *making* the shape a colour paints: the visitor builds a
//!   `C::Shape` and std `From` puts it in the `Shape` enum;
//! - *pairing* a colour with a shape: std `TryFrom` takes the `C::Shape`
//!   out of the `Shape` enum, or hands the shape back.
//!
//! `Visit` and `Members` (std conversions) are all it takes.
#![feature(const_trait_impl, const_convert, const_destruct)]

use dynamickit::{Members, Visit, Visitor};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Circle {
    pub radius: f64,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    pub side: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Members)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Red {
    Light,
    Dark,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Blue {
    Sky,
    Navy,
}

#[derive(Copy, Clone, Debug, PartialEq, Visit)]
pub enum Colour {
    Red(Red),
    Blue(Blue),
}

/// The relation: which shape a colour family paints, and how.
pub trait Paints: core::fmt::Debug {
    type Shape;
    fn sized(size: f64) -> Self::Shape;
    fn describe(&self, shape: &Self::Shape) -> String;
}
impl Paints for Red {
    type Shape = Circle;
    fn sized(size: f64) -> Circle {
        Circle { radius: size }
    }
    fn describe(&self, c: &Circle) -> String {
        format!("a {self:?} red circle of radius {}", c.radius)
    }
}
impl Paints for Blue {
    type Shape = Square;
    fn sized(size: f64) -> Square {
        Square { side: size }
    }
    fn describe(&self, s: &Square) -> String {
        format!("a {self:?} blue square of side {}", s.side)
    }
}

/// The shape a colour paints, of a given size.
struct Make(f64);
impl<C: Paints> Visitor<C> for Make
where
    Shape: From<C::Shape>,
{
    type Output = Shape;
    fn visit(self, _: C) -> Shape {
        Shape::from(C::sized(self.0))
    }
}

/// A colour and a shape, described - if the colour paints that shape.
struct Describe(Shape);
impl<C: Paints> Visitor<C> for Describe
where
    C::Shape: TryFrom<Shape, Error = Shape>,
{
    type Output = Result<String, Shape>;
    fn visit(self, colour: C) -> Result<String, Shape> {
        let shape = C::Shape::try_from(self.0)?;
        Ok(colour.describe(&shape))
    }
}

fn main() {
    let circle = Shape::Circle(Circle { radius: 1.5 });
    println!("{:?}", Colour::Red(Red::Dark).visit_with(Describe(circle)));
    // Not a pair: the shape comes back.
    println!("{:?}", Colour::Blue(Blue::Sky).visit_with(Describe(circle)));
    let made = Colour::Blue(Blue::Navy).visit_with(Make(2.0));
    println!("made {made:?}");
}
