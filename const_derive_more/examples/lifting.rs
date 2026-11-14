//! Lifting: a trait's method and a getter, written once for the dynamic
//! type from the fields' own impls.
//!
//! `Share::share(self, n) -> Option<Self>` divides a value into `n` equal
//! parts, if it can be: a count only when it divides evenly. Lifted,
//! `AnyBag`'s `share` calls it on the count; the fruit is `same`, so every
//! part keeps it. The getter `Kinded::kind(&self) -> FruitKind = self.fruit`
//! reads a field.
#![feature(
    const_trait_impl,
    const_cmp,
    const_convert,
    const_destruct,
    const_default,
    derive_const
)]

use core::marker::PhantomData;

/// Dividing into `n` equal parts.
pub trait Share: Sized {
    fn share(self, n: u32) -> Option<Self>;
}

impl Share for u32 {
    /// Whole fruit only.
    fn share(self, n: u32) -> Option<u32> {
        if n != 0 && self.is_multiple_of(n) {
            Some(self / n)
        } else {
            None
        }
    }
}

/// Anything that is of one fruit.
pub const trait Kinded {
    fn kind(&self) -> FruitKind;
}

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
pub struct Bag<F: Fruit> {
    pub count: u32,
    pub fruit: PhantomData<F>,
}

#[derive(Copy, Clone, Debug, PartialEq, const_derive_more::Dynamic)]
#[dynamic(Bag<F: Fruit> { fruit: PhantomData<F> } => { fruit: F::KIND, count: .count })]
#[dynamic(impl(
    Share::share(self, n: u32) -> Option<Self>,
    Kinded::kind(&self) -> FruitKind = self.fruit,
))]
pub struct AnyBag {
    #[dynamic(same)]
    pub fruit: FruitKind,
    pub count: u32,
}

// What the derive writes for the two items, by hand:
//
// impl Share for AnyBag {
//     fn share(self, n: u32) -> Option<AnyBag> {
//         Some(AnyBag { fruit: self.fruit, count: self.count.share(n)? })
//     }
// }
// const impl Kinded for AnyBag {
//     fn kind(&self) -> FruitKind { self.fruit }
// }

fn main() {
    let apples = AnyBag::from(Bag::<Apple> {
        count: 12,
        fruit: PhantomData,
    });
    println!("12 apples between 3: {:?}", apples.share(3));
    println!("12 apples between 5: {:?}", apples.share(5));
    println!("fruit: {:?}", apples.kind());
}
