//! Visiting a closed set: one generic visitor instead of a `match` per job.
//!
//! `Shape` is an enum of a few shapes; `#[derive(Visit)]` hands a variant's
//! contents to any `Visitor` of all of them. `Area` is one such visitor; a
//! visit by reference (`(&shape).visit_with(..)`) lends the contents.
//!
//! Two shapes are visited at once as a pair: `(a, b).visit_with(..)` hands
//! the visitor both contents, whichever shapes they are - checked for every
//! pair when compiling.
#![feature(const_trait_impl)]

use const_derive_more::{Visit, Visitor};

#[derive(Copy, Clone, Debug)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Square {
    pub side: f64,
}

#[derive(Copy, Clone, Debug, Visit)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

// What the derive writes, by hand (and the same for `&Shape`, with
// `Visitor<&Circle>` and `Visitor<&Square>`):
//
// impl<X> Visit<X> for Shape
// where X: Visitor<Circle> + Visitor<Square, Output = <X as Visitor<Circle>>::Output>
// {
//     type Output = <X as Visitor<Circle>>::Output;
//     fn visit_with(self, x: X) -> Self::Output {
//         match self { Shape::Circle(c) => x.visit(c), Shape::Square(s) => x.visit(s) }
//     }
// }

/// What every shape can say about itself.
pub trait HasArea {
    fn area(&self) -> f64;
}
impl HasArea for Circle {
    fn area(&self) -> f64 {
        core::f64::consts::PI * self.radius * self.radius
    }
}
impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// A shape's area: one visitor for every shape.
struct Area;
impl<S: HasArea> Visitor<&S> for Area {
    type Output = f64;
    fn visit(self, shape: &S) -> f64 {
        shape.area()
    }
}

/// How many of the second shape cover the first: one visitor for every
/// pair of shapes.
struct Covers;
impl<A: HasArea, B: HasArea> Visitor<(A, B)> for Covers {
    type Output = f64;
    fn visit(self, (a, b): (A, B)) -> f64 {
        (a.area() / b.area()).ceil()
    }
}

fn main() {
    let shapes = [
        Shape::Circle(Circle { radius: 1.0 }),
        Shape::Square(Square { side: 2.0 }),
    ];
    for shape in &shapes {
        println!("{shape:?}: area {:.3}", shape.visit_with(Area));
    }
    let total: f64 = shapes.iter().map(|s| s.visit_with(Area)).sum();
    println!("total area {total:.3}");

    // Two at once.
    let tile = Shape::Square(Square { side: 0.5 });
    for shape in shapes {
        println!("{} tiles cover {shape:?}", (shape, tile).visit_with(Covers));
    }
}
