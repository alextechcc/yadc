//! A closed set over a dynamic type: a `Cart` of the fruit a shop sells,
//! computing through `AnyBag`.
//!
//! `#[derive(Members)]` converts between the enum and its variants' members
//! (`Bag<Apple>` <-> `Cart::Apples`) and, with `convert(AnyBag)`, to and
//! from the dynamic type. `#[derive(Via)]` borrows the operators it lists:
//! `+` converts both carts into `AnyBag`, adds them there, and converts
//! back - `None` for apples and oranges. Scaling keeps the variant, so it
//! returns a `Cart`.
#![feature(const_trait_impl, const_cmp, const_ops, const_convert)]
#![feature(const_destruct, const_default, derive_const)]

use core::marker::PhantomData;
use core::ops::Mul;

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug)]
pub enum FruitKind {
    Apple,
    Orange,
}

pub trait Fruit {
    const KIND: FruitKind;
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Apple;
impl Fruit for Apple {
    const KIND: FruitKind = FruitKind::Apple;
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Orange;
impl Fruit for Orange {
    const KIND: FruitKind = FruitKind::Orange;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bag<F: Fruit> {
    pub count: u32,
    pub fruit: PhantomData<F>,
}

pub fn bag<F: Fruit>(count: u32) -> Bag<F> {
    Bag {
        count,
        fruit: PhantomData,
    }
}

/// The static family scales by its own operator.
impl<F: Fruit> Mul<u32> for Bag<F> {
    type Output = Bag<F>;
    fn mul(self, k: u32) -> Bag<F> {
        bag(self.count * k)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, dynamickit::Dynamic)]
#[dynamic(Bag<F: Fruit> { fruit: PhantomData<F> } => { fruit: F::KIND, count: .count })]
#[dynamic(impl(Add -> Option<Self>))]
pub struct AnyBag {
    #[dynamic(same)]
    pub fruit: FruitKind,
    pub count: u32,
}

/// The fruit this shop sells.
#[derive(Copy, Clone, Debug, PartialEq, dynamickit::Members, dynamickit::Via)]
#[members(convert(AnyBag))]
#[via(AnyBag, Add, Mul<u32>)]
pub enum Cart {
    Apples(Bag<Apple>),
    Oranges(Bag<Orange>),
}

// What the derives write, by hand (per variant, and for each operator):
//
// impl From<Bag<Apple>> for Cart { fn from(b: Bag<Apple>) -> Self { Cart::Apples(b) } }
// impl TryFrom<Cart> for Bag<Apple> {
//     type Error = Cart;
//     fn try_from(c: Cart) -> Result<Self, Cart> { match c { Cart::Apples(b) => Ok(b), c => Err(c) } }
// }
// impl From<Cart> for AnyBag { fn from(c: Cart) -> Self { match c { Cart::Apples(b) => b.into(), .. } } }
// impl TryFrom<AnyBag> for Cart { /* each member's `TryFrom<AnyBag>` in turn */ }
// impl Add for Cart {
//     type Output = Option<Cart>;
//     fn add(self, other: Cart) -> Option<Cart> {
//         Cart::try_from((AnyBag::from(self) + AnyBag::from(other))?).ok()
//     }
// }
// impl Mul<u32> for Cart {
//     type Output = Cart;
//     fn mul(self, k: u32) -> Cart {
//         match self { Cart::Apples(b) => Cart::Apples(b * k), Cart::Oranges(b) => Cart::Oranges(b * k) }
//     }
// }

fn main() {
    let apples = Cart::from(bag::<Apple>(6));
    let more = Cart::from(bag::<Apple>(3));
    let oranges = Cart::from(bag::<Orange>(4));
    println!("6 apples + 3 apples  = {:?}", apples + more);
    println!("6 apples + 4 oranges = {:?}", apples + oranges);
    println!("4 oranges × 3        = {:?}", oranges * 3);
}
