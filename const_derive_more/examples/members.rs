//! A choice of a few unrelated types is an enum: an RGB colour or an HSV
//! colour.
//!
//! `Rgb` and `Hsv` have nothing in common but being colours - different
//! channels, no arithmetic between them - so the type holding either is a
//! plain Rust enum, one variant per type. `#[derive(Members)]` writes the
//! std conversions between the enum and its variants' members, so a colour
//! goes in with `From` and comes back out, checked, with `TryFrom`.
//!
//! (A struct with a fact field - `#[derive(Dynamic)]` - is for a *family*
//! whose members differ by a value: a fruit, a dimension. See `dynamic` and `dynamic_const`.)
#![feature(const_trait_impl, const_convert, const_destruct)]

/// Red, green, blue: each 0 to 1.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// Hue in degrees, saturation and value 0 to 1.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

/// A colour in either space.
#[derive(Copy, Clone, Debug, PartialEq, const_derive_more::Members)]
pub enum AnyColor {
    Rgb(Rgb),
    Hsv(Hsv),
}

// What the derive writes, by hand (and the same for `Hsv`):
//
// impl From<Rgb> for AnyColor {
//     fn from(c: Rgb) -> Self { AnyColor::Rgb(c) }
// }
// impl TryFrom<AnyColor> for Rgb {
//     type Error = AnyColor;
//     fn try_from(c: AnyColor) -> Result<Self, AnyColor> {
//         match c { AnyColor::Rgb(c) => Ok(c), c => Err(c) }
//     }
// }

fn main() {
    let palette = [
        AnyColor::from(Rgb {
            r: 1.0,
            g: 0.5,
            b: 0.0,
        }),
        AnyColor::from(Hsv {
            h: 210.0,
            s: 0.8,
            v: 0.9,
        }),
    ];
    for color in palette {
        match color {
            AnyColor::Rgb(c) => println!("rgb({}, {}, {})", c.r, c.g, c.b),
            AnyColor::Hsv(c) => println!("hsv({}°, {}, {})", c.h, c.s, c.v),
        }
    }

    // Back to one of the static types: checked.
    println!("as Rgb: {:?}", Rgb::try_from(palette[0]));
    println!("as Hsv: {:?}", Hsv::try_from(palette[0]).is_ok());
}
