//! The dynamic type: one `AnyBag` for every `Bag<F>`.
//!
//! `Bag<F: Fruit>` is a family of static types: a bag of apples and a bag of
//! oranges are different types, so adding them does not compile. When the
//! fruit is only known at run time (read from a file), `AnyBag` holds any of
//! them: the fruit - a constant of the type parameter - becomes a field (a
//! *fact*), the count moves in (the *payload*).
//!
//! Its `+` does one thing to every field. The count adds, by its own `+`.
//! The fruit has no arithmetic: it is marked `same`, so two bags must hold
//! the same fruit, and there is no result otherwise - you can't add apples
//! and oranges.
#![feature(const_trait_impl, const_cmp, const_ops, const_convert, const_destruct)]
#![feature(derive_const, const_default)]

use core::marker::PhantomData;

/// The fruits, as values.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug)]
pub enum FruitKind {
    Apple,
    Orange,
}

/// A fruit, as a type: which `FruitKind` it is.
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

/// The family: one static type per fruit, untouched by const_derive_more.
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

/// Its dynamic version. The mapping says where each field comes from:
/// `fruit` from the parameter's constant, `count` from the static value's
/// field; the other field of `Bag` is declared so the way back can fill it
/// (`PhantomData` is `Default`).
#[derive(Copy, Clone, Debug, PartialEq, const_derive_more::Dynamic)]
#[dynamic(Bag<F: Fruit> { fruit: PhantomData<F> } => { fruit: F::KIND, count: .count })]
#[dynamic(impl(Add -> Option<Self>, Mul<u32> -> Self))]
pub struct AnyBag {
    #[dynamic(same)]
    pub fruit: FruitKind,
    pub count: u32,
}

// What the derive writes, by hand:
//
// impl<F: Fruit> From<Bag<F>> for AnyBag {
//     fn from(b: Bag<F>) -> Self { AnyBag { fruit: F::KIND, count: b.count } }
// }
// impl<F: Fruit> TryFrom<AnyBag> for Bag<F> {
//     type Error = AnyBag;
//     fn try_from(a: AnyBag) -> Result<Self, AnyBag> {
//         if a.fruit == F::KIND {
//             Ok(Bag { count: a.count, fruit: PhantomData::default() })
//         } else {
//             Err(a)
//         }
//     }
// }
// impl Add for AnyBag {
//     type Output = Option<AnyBag>;
//     fn add(self, other: AnyBag) -> Option<AnyBag> {
//         Some(AnyBag {
//             fruit: if self.fruit == other.fruit { self.fruit } else { return None },
//             count: self.count + other.count,
//         })
//     }
// }
// impl Mul<u32> for AnyBag {
//     type Output = AnyBag;
//     fn mul(self, k: u32) -> AnyBag { AnyBag { fruit: self.fruit, count: self.count * k } }
// }

fn main() {
    // Picked at run time: a delivery of mixed bags.
    let delivery = [
        AnyBag::from(bag::<Apple>(3)),
        AnyBag::from(bag::<Apple>(10)),
        AnyBag::from(bag::<Orange>(4)),
    ];

    let apples = (delivery[0] + delivery[1]).expect("both apples");
    println!("3 apples + 10 apples = {apples:?}");
    println!("3 apples + 4 oranges = {:?}", delivery[0] + delivery[2]);
    println!("4 oranges × 5        = {:?}", delivery[2] * 5);

    // Back to a static type: checked, the value handed back if it is not.
    let back: Bag<Apple> = Bag::try_from(apples).expect("apples");
    println!("as Bag<Apple>: {} apples", back.count);
    println!(
        "as Bag<Orange>: {:?}",
        Bag::<Orange>::try_from(apples).map(|b| b.count)
    );
}
