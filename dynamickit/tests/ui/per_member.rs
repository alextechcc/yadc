//! A family whose facts come from its parameter, beside a plain type: each
//! member is checked when used - a currency with the gift card's code fails
//! (in a `const`, so `cargo check` sees it too; elsewhere `cargo build`
//! does).
#![feature(const_trait_impl, derive_const, const_cmp, const_destruct, const_default, const_convert)]
use core::marker::PhantomData;

pub trait Currency {
    const CODE: u16;
}
pub struct Eur;
impl Currency for Eur {
    const CODE: u16 = 978;
}
pub struct Fake;
impl Currency for Fake {
    const CODE: u16 = 0;
}

pub struct Money<C: Currency> {
    cents: i64,
    currency: PhantomData<C>,
}
pub struct GiftCard {
    balance: i64,
}

#[derive(dynamickit::Dynamic)]
#[dynamic(Money<C: Currency> { currency: PhantomData<C> } => { code: C::CODE, cents: .cents })]
#[dynamic(GiftCard => { code: 0, cents: .balance })]
pub struct AnyMoney {
    code: u16,
    cents: i64,
}

const EUROS: AnyMoney = AnyMoney::from(Money::<Eur> { cents: 1, currency: PhantomData });
const FAKES: AnyMoney = AnyMoney::from(Money::<Fake> { cents: 1, currency: PhantomData });

fn main() {
    let _ = (EUROS, FAKES);
}
