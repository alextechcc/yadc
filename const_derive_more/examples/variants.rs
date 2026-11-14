//! Walking an enum's variants: a settings menu built from `Variants`.
//!
//! `#[derive(Variants)]` hands every variant's name and constructor, in
//! order, to a `VariantVisitor` - without a value of the enum. What a menu
//! needs is two such visitors, written here: one that lists the choices of
//! a flat enum, one that lists the groups of an enum of enums.
#![feature(const_trait_impl)]

use const_derive_more::{VariantVisitor, Variants};

#[derive(Copy, Clone, Debug, PartialEq, Variants)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Copy, Clone, Debug, PartialEq, Variants)]
pub enum FontSize {
    Small,
    Medium,
    Large,
}

/// Every setting, in groups.
#[derive(Copy, Clone, Debug, PartialEq, Variants)]
pub enum Setting {
    Theme(Theme),
    FontSize(FontSize),
}

// What the derive writes, by hand:
//
// impl<X: VariantVisitor<Theme, ()>> Variants<X> for Theme {
//     fn each_variant(x: &mut X) {
//         x.visit("Light", |()| Theme::Light);
//         x.visit("Dark", |()| Theme::Dark);
//     }
// }
// impl<X> Variants<X> for Setting
// where X: VariantVisitor<Setting, Theme> + VariantVisitor<Setting, FontSize>
// {
//     fn each_variant(x: &mut X) {
//         x.visit("Theme", Setting::Theme);
//         x.visit("FontSize", Setting::FontSize);
//     }
// }

/// The choices of a flat enum: every variant without contents, built.
struct Choices<E>(Vec<E>);
impl<E> VariantVisitor<E, ()> for Choices<E> {
    fn visit(&mut self, _name: &'static str, make: fn(()) -> E) {
        self.0.push(make(()));
    }
}

fn choices<E: Variants<Choices<E>>>() -> Vec<E> {
    let mut all = Choices(Vec::new());
    E::each_variant(&mut all);
    all.0
}

/// The groups of an enum of enums: each variant's name, and the choices of
/// the enum it wraps, wrapped.
struct Groups<E>(Vec<(&'static str, Vec<E>)>);
impl<E, T: Variants<Choices<T>>> VariantVisitor<E, T> for Groups<E> {
    fn visit(&mut self, name: &'static str, make: fn(T) -> E) {
        self.0
            .push((name, choices::<T>().into_iter().map(make).collect()));
    }
}

fn main() {
    println!("themes: {:?}", choices::<Theme>());

    let mut menu = Groups(Vec::new());
    Setting::each_variant(&mut menu);
    for (group, entries) in menu.0 {
        println!("{group}: {entries:?}");
    }
}
