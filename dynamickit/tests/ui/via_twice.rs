//! One `#[via(..)]` lists every operator: a second is refused, not ignored.
#![feature(const_trait_impl, const_convert, const_destruct)]

#[derive(Copy, Clone)]
pub struct Apples(i64);
impl core::ops::Mul<i64> for Apples {
    type Output = Apples;
    fn mul(self, k: i64) -> Apples {
        Apples(self.0 * k)
    }
}

#[derive(Copy, Clone, dynamickit::Via)]
#[via(Apples, Mul<i64>)]
#[via(Apples, Div<i64>)]
pub enum Shelf {
    Apples(Apples),
}

fn main() {}
