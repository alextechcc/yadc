//! A choice among named constants: a paper size picked at run time.
//!
//! The sizes are constants of a type of yours (`A4`, `LETTER`); code that
//! knows its size uses them directly. When the size is a setting - read from
//! a file, picked by a user - it is an enum, one variant per constant.
//! `#[derive(Values)]` makes the enum act as the constant its variant names:
//! it implements the traits listed (`delegate`), prints and parses as the
//! constant (`Display`, `FromStr`), and converts through it (`into`). Its
//! impls are ordinary ones - a choice is made at run time - so it needs no
//! crate features.

use dynamickit::{NoValue, Values};
use std::fmt;
use std::ops::Mul;

/// A sheet of paper: a name and its sides in millimetres.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Paper {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
}

pub const A4: Paper = Paper {
    name: "A4",
    width: 210,
    height: 297,
};
pub const A5: Paper = Paper {
    name: "A5",
    width: 148,
    height: 210,
};
pub const LETTER: Paper = Paper {
    name: "Letter",
    width: 216,
    height: 279,
};

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

/// Something with sides, measured in square millimetres counted in `N`.
pub trait Sheet<N> {
    fn area(&self) -> N;
    /// Whether a sheet of `area` fits on this one by area alone.
    fn holds(&self, area: N) -> bool;
}

impl<N: From<u32> + Mul<Output = N> + PartialOrd> Sheet<N> for Paper {
    fn area(&self) -> N {
        N::from(self.width) * N::from(self.height)
    }
    fn holds(&self, area: N) -> bool {
        area <= self.area()
    }
}

/// The sides alone, as a layout engine wants them.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size(pub u32, pub u32);

impl From<Paper> for Size {
    fn from(p: Paper) -> Size {
        Size(p.width, p.height)
    }
}

/// The paper sizes the printer takes.
#[derive(Copy, Clone, Debug, PartialEq, Values)]
#[values(
    delegate(Sheet<N> {
        fn area(&self) -> N;
        fn holds(&self, area: N) -> bool;
    }),
    into(Size),
    for<N: From<u32> + Mul<Output = N> + PartialOrd>
)]
pub enum Tray {
    #[value(A4)]
    A4,
    #[value(A5)]
    A5,
    #[value(LETTER)]
    Letter,
}

// What the derive writes, by hand:
//
// impl<N: From<u32> + Mul<Output = N> + PartialOrd> Sheet<N> for Tray {
//     fn area(&self) -> N {
//         match self {
//             Tray::A4 => <_ as Sheet<N>>::area(&A4),
//             Tray::A5 => <_ as Sheet<N>>::area(&A5),
//             Tray::Letter => <_ as Sheet<N>>::area(&LETTER),
//         }
//     }
//     fn holds(&self, area: N) -> bool { /* alike */ }
// }
// impl From<Tray> for Size {
//     fn from(e: Tray) -> Size {
//         match e { Tray::A4 => Size::from(A4), /* .. */ }
//     }
// }
// impl Display for Tray { /* match: A4's Display, .. */ }
// impl FromStr for Tray {
//     type Err = NoValue;
//     // the first variant whose constant prints as `s`
// }

fn main() {
    // The setting, parsed by the constant's name.
    let tray: Tray = "Letter".parse().unwrap();
    assert_eq!(tray, Tray::Letter);
    assert_eq!("B5".parse::<Tray>(), Err(NoValue));

    // It is the constant it names: its traits, its name, its conversions.
    let area: u64 = tray.area();
    assert_eq!(area, Sheet::<u64>::area(&LETTER));
    assert!(tray.holds(Sheet::<u64>::area(&A5)));
    assert!(!Tray::A5.holds(area));
    assert_eq!(Size::from(tray), Size(216, 279));
    assert_eq!(tray.to_string(), "Letter");

    for t in [Tray::A4, Tray::A5, Tray::Letter] {
        let area: f64 = t.area();
        println!("{t}: {:?}, {:.0} cm²", Size::from(t), area / 100.0);
    }
}
