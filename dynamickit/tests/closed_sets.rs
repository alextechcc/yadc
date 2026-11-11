//! The closed-set derives: visiting shapes (one value, by reference, two at
//! once), walking variants, relating colours to shapes through an associated
//! type, and a stock of fruit converted through and computing with a dynamic
//! type.
#![feature(const_trait_impl, const_destruct, const_default, derive_const)]
#![feature(const_cmp, const_ops, const_convert, adt_const_params)]
#![allow(incomplete_features)]

use dynamickit::{Dynamic, Members, VariantVisitor, Variants, Via, Visit, Visitor};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Circle {
    r: u32,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Square {
    side: u32,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Line;

#[derive(Clone, Copy, PartialEq, Debug, Visit, Variants, Members)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

/// Every shape of contents a variant can have.
#[derive(Clone, Copy, PartialEq, Debug, Visit, Variants)]
pub enum Mark {
    Circle(Circle),
    /// Two fields: the contents are a tuple.
    Segment(Line, u32),
    /// Named fields: a tuple too, in order.
    Label { size: u32, bold: bool },
    Dot,
}

#[derive(Clone, Copy, PartialEq, Debug, Default, Variants)]
pub enum Red {
    #[default]
    Light,
    Dark,
}
#[derive(Clone, Copy, PartialEq, Debug, Default, Variants)]
pub enum Blue {
    #[default]
    Sky,
    Navy,
}

/// The colours shapes come in: circles in reds, squares in blues.
#[derive(Clone, Copy, PartialEq, Debug, Visit, Variants)]
pub enum Paint {
    Red(Red),
    Blue(Blue),
}

// ===== Visit ==================================================================

/// A visitor generic over every variant's contents: their size.
struct SizeOf;
impl<T> Visitor<T> for SizeOf {
    type Output = usize;
    fn visit(self, _: T) -> usize {
        core::mem::size_of::<T>()
    }
}

#[test]
fn visit_by_value() {
    assert_eq!(Mark::Dot.visit_with(SizeOf), 0);
    assert_eq!(Mark::Circle(Circle { r: 1 }).visit_with(SizeOf), 4);
    assert_eq!(Mark::Segment(Line, 3).visit_with(SizeOf), 4);
    assert_eq!(
        Mark::Label {
            size: 1,
            bold: true
        }
        .visit_with(SizeOf),
        core::mem::size_of::<(u32, bool)>()
    );
}

/// A visitor of references: the enum lends its contents.
struct Size;
impl Visitor<&Circle> for Size {
    type Output = u32;
    fn visit(self, c: &Circle) -> u32 {
        c.r
    }
}
impl Visitor<(&Line, &u32)> for Size {
    type Output = u32;
    fn visit(self, (_, n): (&Line, &u32)) -> u32 {
        *n
    }
}
impl Visitor<(&u32, &bool)> for Size {
    type Output = u32;
    fn visit(self, (n, _): (&u32, &bool)) -> u32 {
        *n
    }
}
impl Visitor<()> for Size {
    type Output = u32;
    fn visit(self, (): ()) -> u32 {
        0
    }
}

#[test]
fn visit_by_reference() {
    let marks = [
        Mark::Circle(Circle { r: 2 }),
        Mark::Segment(Line, 4),
        Mark::Label {
            size: 9,
            bold: false,
        },
        Mark::Dot,
    ];
    let sizes: Vec<u32> = marks.iter().map(|s| s.visit_with(Size)).collect();
    assert_eq!(sizes, [2, 4, 9, 0]);
    // The marks are still here.
    assert_eq!(marks[0], Mark::Circle(Circle { r: 2 }));
}

/// What every shape has.
trait Extent {
    fn extent(&self) -> u32;
}
impl Extent for Circle {
    fn extent(&self) -> u32 {
        2 * self.r
    }
}
impl Extent for Square {
    fn extent(&self) -> u32 {
        self.side
    }
}

/// A visitor of every pair of shapes: whether the first fits in the second.
struct FitsIn;
impl<A: Extent, B: Extent> Visitor<(A, B)> for FitsIn {
    type Output = bool;
    fn visit(self, (a, b): (A, B)) -> bool {
        a.extent() <= b.extent()
    }
}

/// A visitor of every triple.
struct Widest;
impl<A: Extent, B: Extent, C: Extent> Visitor<((A, B), C)> for Widest {
    type Output = u32;
    fn visit(self, ((a, b), c): ((A, B), C)) -> u32 {
        a.extent().max(b.extent()).max(c.extent())
    }
}

#[test]
fn visit_several_at_once() {
    let (c, s) = (
        Shape::Circle(Circle { r: 2 }),
        Shape::Square(Square { side: 3 }),
    );
    assert!(!(c, s).visit_with(FitsIn));
    assert!((s, c).visit_with(FitsIn));
    assert!((c, c).visit_with(FitsIn));
    // Pairs nest; references are visited too.
    assert_eq!(((c, s), c).visit_with(Widest), 4);
    assert_eq!(((&c, &s), &s).visit_with(WidestRef), 4);
}

struct WidestRef;
impl<A: Extent, B: Extent, C: Extent> Visitor<((&A, &B), &C)> for WidestRef {
    type Output = u32;
    fn visit(self, ((a, b), c): ((&A, &B), &C)) -> u32 {
        a.extent().max(b.extent()).max(c.extent())
    }
}

// ===== Variants ===============================================================

/// The names of the variants, whatever their contents.
struct Names(Vec<&'static str>);
impl<E, T> VariantVisitor<E, T> for Names {
    fn visit(&mut self, name: &'static str, _: fn(T) -> E) {
        self.0.push(name);
    }
}

/// Every variant built from its contents' `Default`.
struct Defaults<E>(Vec<E>);
impl<E, T: Default> VariantVisitor<E, T> for Defaults<E> {
    fn visit(&mut self, _: &'static str, make: fn(T) -> E) {
        self.0.push(make(T::default()));
    }
}

/// The variant of a given name, built from its contents' `Default`.
struct Named<E>(&'static str, Option<E>);
impl<E, T: Default> VariantVisitor<E, T> for Named<E> {
    fn visit(&mut self, name: &'static str, make: fn(T) -> E) {
        if name == self.0 {
            self.1 = Some(make(T::default()));
        }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle { r: 1 }
    }
}
impl Default for Line {
    fn default() -> Self {
        Line
    }
}

#[test]
fn variants_in_order() {
    let mut names = Names(Vec::new());
    Mark::each_variant(&mut names);
    assert_eq!(names.0, ["Circle", "Segment", "Label", "Dot"]);

    let mut all = Defaults(Vec::new());
    Mark::each_variant(&mut all);
    assert_eq!(
        all.0,
        [
            Mark::Circle(Circle { r: 1 }),
            Mark::Segment(Line, 0),
            Mark::Label {
                size: 0,
                bold: false
            },
            Mark::Dot,
        ]
    );

    // A flat enum's choices, and an enum of enums' groups.
    let mut reds = Defaults(Vec::new());
    Red::each_variant(&mut reds);
    assert_eq!(reds.0, [Red::Light, Red::Dark]);
    let mut paints = Defaults(Vec::new());
    Paint::each_variant(&mut paints);
    assert_eq!(paints.0, [Paint::Red(Red::Light), Paint::Blue(Blue::Sky)]);

    let mut blue = Named("Blue", None);
    Paint::each_variant(&mut blue);
    assert_eq!(blue.1, Some(Paint::Blue(Blue::Sky)));
}

// ===== Relating two closed sets ================================================

/// Which shape a colour family paints.
trait Paints {
    type Shape;
    fn sized(n: u32) -> Self::Shape;
}
impl Paints for Red {
    type Shape = Circle;
    fn sized(n: u32) -> Circle {
        Circle { r: n }
    }
}
impl Paints for Blue {
    type Shape = Square;
    fn sized(n: u32) -> Square {
        Square { side: n }
    }
}

/// The shape a colour paints, of size `n`: std `From` puts it in the enum.
struct Make(u32);
impl<C: Paints> Visitor<C> for Make
where
    Shape: From<C::Shape>,
{
    type Output = Shape;
    fn visit(self, _: C) -> Shape {
        Shape::from(C::sized(self.0))
    }
}

/// A colour with a shape, if it paints that shape: std `TryFrom` takes it
/// out of the enum, or hands the enum back.
struct Pair(Shape);
impl<C: Paints> Visitor<C> for Pair
where
    C::Shape: TryFrom<Shape, Error = Shape>,
{
    type Output = Result<(C, C::Shape), Shape>;
    fn visit(self, colour: C) -> Result<(C, C::Shape), Shape> {
        Ok((colour, C::Shape::try_from(self.0)?))
    }
}

/// `Pair`'s output differs per colour, so each case is turned into one type.
struct Paired(Shape);
impl<C: Paints + core::fmt::Debug> Visitor<C> for Paired
where
    C::Shape: TryFrom<Shape, Error = Shape> + core::fmt::Debug,
{
    type Output = Result<String, Shape>;
    fn visit(self, colour: C) -> Result<String, Shape> {
        Pair(self.0)
            .visit(colour)
            .map(|(c, s)| format!("{c:?} {s:?}"))
    }
}

#[test]
fn related_by_an_associated_type() {
    assert_eq!(
        Paint::Blue(Blue::Sky).visit_with(Make(5)),
        Shape::Square(Square { side: 5 })
    );
    let circle = Shape::Circle(Circle { r: 3 });
    assert_eq!(
        Paint::Red(Red::Dark).visit_with(Paired(circle)),
        Ok("Dark Circle { r: 3 }".to_string())
    );
    // Not a pair: the shape comes back.
    assert_eq!(Paint::Blue(Blue::Navy).visit_with(Paired(circle)), Err(circle));
}

// ===== Members and Via =========================================================

/// A stock family (a const parameter: the fruit) and its dynamic version,
/// which adds and subtracts stock of one fruit.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, core::marker::ConstParamTy)]
pub enum Fruit {
    Apple,
    Orange,
    Pear,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Stock<const F: Fruit>(i64);
impl<const F: Fruit> core::ops::Mul<i64> for Stock<F> {
    type Output = Stock<F>;
    fn mul(self, k: i64) -> Stock<F> {
        Stock(self.0 * k)
    }
}
impl<const F: Fruit> core::ops::Div<i64> for Stock<F> {
    type Output = Stock<F>;
    fn div(self, k: i64) -> Stock<F> {
        Stock(self.0 / k)
    }
}
type Apples = Stock<{ Fruit::Apple }>;
type Oranges = Stock<{ Fruit::Orange }>;
type Pears = Stock<{ Fruit::Pear }>;

#[derive(Copy, Clone, PartialEq, Debug, Dynamic)]
#[dynamic(Stock<const F: Fruit> => { fruit: F, count: .0 })]
#[dynamic(impl(Add -> Option<Self>, Sub -> Option<Self>))]
pub struct AnyStock {
    #[dynamic(same)]
    fruit: Fruit,
    count: i64,
}

/// A note that rides along with a stock.
#[derive_const(Default)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Note(u8);

#[derive(Copy, Clone, PartialEq, Debug, Members, Via)]
#[members(convert(AnyStock))]
#[via(AnyStock, Add, Sub, Mul<i64>, Div<i64>)]
pub enum Shelf {
    Apples(Apples),
    Oranges(
        #[members(member)] Oranges,
        #[members(default)]
        #[via(keep)]
        Note,
    ),
}

/// Pears with a note that is not `default`: no `From<Pears>`.
#[derive(Copy, Clone, PartialEq, Debug, Members)]
pub enum Crate {
    Pears(#[members(member)] Pears, Note),
    Apples(Apples),
}

#[test]
fn members() {
    let w = Shelf::from(Stock::<{ Fruit::Apple }>(250));
    assert_eq!(w, Shelf::Apples(Stock(250)));
    assert_eq!(Apples::try_from(w), Ok(Stock(250)));
    assert_eq!(Oranges::try_from(w), Err(w));
    assert_eq!(
        Shelf::from(Stock::<{ Fruit::Orange }>(5)),
        Shelf::Oranges(Stock(5), Note(0))
    );
    let any = AnyStock::from(w);
    assert_eq!(Shelf::try_from(any), Ok(w));
    assert!(Shelf::try_from(AnyStock::from(Stock::<{ Fruit::Pear }>(1))).is_err());
    // `Crate::Pears` is only taken apart.
    let p = Crate::Pears(Stock(7), Note(3));
    assert_eq!(Pears::try_from(p), Ok(Stock(7)));
    assert_eq!(Crate::from(Stock::<{ Fruit::Apple }>(1)), Crate::Apples(Stock(1)));
}

/// Generic code over a set that computes through a dynamic type names the
/// std operator.
fn total<W: core::ops::Add<Output = Option<W>>>(a: W, b: W) -> Option<W> {
    a + b
}

#[test]
fn via() {
    let (a, b) = (Shelf::Apples(Stock(250)), Shelf::Apples(Stock(100)));
    assert_eq!(a + b, Some(Shelf::Apples(Stock(350))));
    assert_eq!(total(a, b), Some(Shelf::Apples(Stock(350))));
    assert_eq!(a - b, Some(Shelf::Apples(Stock(150))));
    // `AnyStock`'s `+` refuses two fruits.
    let d = Shelf::Oranges(Stock(100), Note(9));
    assert_eq!(a + d, None);
    // A kept field rides along from the left operand.
    assert_eq!(d + d, Some(Shelf::Oranges(Stock(200), Note(9))));
    // Scaling keeps the variant: `Self`, not an `Option`.
    assert_eq!(a * 3, Shelf::Apples(Stock(750)));
    assert_eq!(d / 2, Shelf::Oranges(Stock(50), Note(9)));
}

// ===== A generic set with a default ===========================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Loose<T>(T);
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Boxed<T>(T);

/// The enum's parameter default is the enum's own: the impls take its
/// parameter without it.
#[derive(Copy, Clone, PartialEq, Debug, Visit, Variants, Members)]
pub enum Packed<T = u8> {
    Loose(Loose<T>),
    Boxed(Boxed<T>),
}

struct Inner;
impl<T> Visitor<Loose<T>> for Inner {
    type Output = T;
    fn visit(self, l: Loose<T>) -> T {
        l.0
    }
}
impl<T> Visitor<Boxed<T>> for Inner {
    type Output = T;
    fn visit(self, b: Boxed<T>) -> T {
        b.0
    }
}

#[test]
fn generic_default() {
    let p: Packed = Packed::from(Boxed(3u8));
    assert_eq!(p.visit_with(Inner), 3);
    assert_eq!(Loose::try_from(p), Err(p));
    let mut names = Names(Vec::new());
    Packed::<u8>::each_variant(&mut names);
    assert_eq!(names.0, ["Loose", "Boxed"]);
}
