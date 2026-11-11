//! Closed sets: `From<M>` needs the other fields marked `default` for
//! `convert`; `Via` lists its operators; a `same` field can refuse, so an
//! operator on two values states `Option<Self>`.
#![feature(const_trait_impl, const_destruct, const_cmp, const_ops, const_convert)]

pub struct Any;
impl From<Purse> for Any {
    fn from(_: Purse) -> Any {
        Any
    }
}

#[derive(dynamickit::Members)]
#[members(convert(Any))]
pub enum Purse {
    Pounds(#[members(member)] u32, u8),
}

#[derive(dynamickit::Via)]
#[via(Any)]
pub enum Unlisted {
    Pounds(u32),
}

pub struct Bag(u32);

#[derive(dynamickit::Dynamic)]
#[dynamic(Bag => { kind: 0, count: .0 })]
#[dynamic(impl(Add -> Self))]
pub struct AnyBag {
    #[dynamic(same)]
    kind: u8,
    count: u32,
}

fn main() {}
