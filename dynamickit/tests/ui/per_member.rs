//! A family whose facts come from its parameter, beside a plain type: each
//! member is checked when used - a fruit with the bowl's kind fails (in a
//! `const`, so `cargo check` sees it too; elsewhere `cargo build` does).
#![feature(const_trait_impl, derive_const, const_cmp, const_destruct, const_default, const_convert)]
use core::marker::PhantomData;

pub trait Fruit {
    const KIND: u8;
}
pub struct Apple;
impl Fruit for Apple {
    const KIND: u8 = 1;
}
pub struct Fake;
impl Fruit for Fake {
    const KIND: u8 = 0;
}

pub struct Bag<F: Fruit> {
    count: i64,
    fruit: PhantomData<F>,
}
pub struct Bowl {
    pieces: i64,
}

#[derive(dynamickit::Dynamic)]
#[dynamic(Bag<F: Fruit> { fruit: PhantomData<F> } => { fruit: F::KIND, count: .count })]
#[dynamic(Bowl => { fruit: 0, count: .pieces })]
pub struct AnyBag {
    fruit: u8,
    count: i64,
}

const APPLES: AnyBag = AnyBag::from(Bag::<Apple> { count: 1, fruit: PhantomData });
const FAKES: AnyBag = AnyBag::from(Bag::<Fake> { count: 1, fruit: PhantomData });

fn main() {
    let _ = (APPLES, FAKES);
}
