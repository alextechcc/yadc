//! The SI prefixes, once: exact powers of ten that scale a unit
//! (`Prefixed { prefix: Some(Prefix::Kilo), unit: Metre {} }`, and
//! `"km".parse::<Prefixed<LengthUnit>>()`).

use crate::Exact;
use core::fmt;
use core::str::FromStr;
use yadc::{Unit, UnknownUnit};

/// An SI prefix: a power of ten a unit is scaled by.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq)]
pub enum Prefix {
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deca,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
}

impl Prefix {
    /// Every prefix, largest first.
    pub const ALL: [Prefix; 20] = [
        Prefix::Yotta,
        Prefix::Zetta,
        Prefix::Exa,
        Prefix::Peta,
        Prefix::Tera,
        Prefix::Giga,
        Prefix::Mega,
        Prefix::Kilo,
        Prefix::Hecto,
        Prefix::Deca,
        Prefix::Deci,
        Prefix::Centi,
        Prefix::Milli,
        Prefix::Micro,
        Prefix::Nano,
        Prefix::Pico,
        Prefix::Femto,
        Prefix::Atto,
        Prefix::Zepto,
        Prefix::Yocto,
    ];

    /// The power of ten it scales by.
    pub const fn exponent(self) -> i16 {
        match self {
            Prefix::Yotta => 24,
            Prefix::Zetta => 21,
            Prefix::Exa => 18,
            Prefix::Peta => 15,
            Prefix::Tera => 12,
            Prefix::Giga => 9,
            Prefix::Mega => 6,
            Prefix::Kilo => 3,
            Prefix::Hecto => 2,
            Prefix::Deca => 1,
            Prefix::Deci => -1,
            Prefix::Centi => -2,
            Prefix::Milli => -3,
            Prefix::Micro => -6,
            Prefix::Nano => -9,
            Prefix::Pico => -12,
            Prefix::Femto => -15,
            Prefix::Atto => -18,
            Prefix::Zepto => -21,
            Prefix::Yocto => -24,
        }
    }

    /// The exact number it scales by: `10^exponent`.
    pub const fn factor(self) -> Exact {
        Exact::dec(1, self.exponent())
    }

    /// Its symbol: `k`, `µ`, `da`.
    pub const fn symbol(self) -> &'static str {
        match self {
            Prefix::Yotta => "Y",
            Prefix::Zetta => "Z",
            Prefix::Exa => "E",
            Prefix::Peta => "P",
            Prefix::Tera => "T",
            Prefix::Giga => "G",
            Prefix::Mega => "M",
            Prefix::Kilo => "k",
            Prefix::Hecto => "h",
            Prefix::Deca => "da",
            Prefix::Deci => "d",
            Prefix::Centi => "c",
            Prefix::Milli => "m",
            Prefix::Micro => "µ",
            Prefix::Nano => "n",
            Prefix::Pico => "p",
            Prefix::Femto => "f",
            Prefix::Atto => "a",
            Prefix::Zepto => "z",
            Prefix::Yocto => "y",
        }
    }

    /// Its name: `kilo`, `micro`, `deca`.
    pub const fn name(self) -> &'static str {
        match self {
            Prefix::Yotta => "yotta",
            Prefix::Zetta => "zetta",
            Prefix::Exa => "exa",
            Prefix::Peta => "peta",
            Prefix::Tera => "tera",
            Prefix::Giga => "giga",
            Prefix::Mega => "mega",
            Prefix::Kilo => "kilo",
            Prefix::Hecto => "hecto",
            Prefix::Deca => "deca",
            Prefix::Deci => "deci",
            Prefix::Centi => "centi",
            Prefix::Milli => "milli",
            Prefix::Micro => "micro",
            Prefix::Nano => "nano",
            Prefix::Pico => "pico",
            Prefix::Femto => "femto",
            Prefix::Atto => "atto",
            Prefix::Zepto => "zepto",
            Prefix::Yocto => "yocto",
        }
    }

    /// The prefix a symbol starts with, and the rest: `"km"` is `(Kilo, "m")`.
    /// `µ` is also written `μ` (Greek mu) or `u`.
    fn split(s: &str) -> impl Iterator<Item = (Prefix, &str)> {
        Prefix::ALL.into_iter().flat_map(move |p| {
            let spellings: &[&str] = match p {
                Prefix::Micro => &["µ", "μ", "u"],
                _ => core::slice::from_ref(&SYMBOLS[p as usize]),
            };
            spellings
                .iter()
                .filter_map(move |sym| s.strip_prefix(sym).map(|rest| (p, rest)))
        })
    }
}

const SYMBOLS: [&str; 20] = {
    let mut out = [""; 20];
    let mut i = 0;
    while i < 20 {
        out[i] = Prefix::ALL[i].symbol();
        i += 1;
    }
    out
};

/// A unit enum's value with an optional prefix: `km` is `Prefixed { prefix:
/// Some(Prefix::Kilo), unit: LengthUnit::Metre }`. The prefix scales the
/// unit's first symbol with its power - `km²` is (10³ m)², a million square
/// metres; `km/s²` a thousand metres per second squared. It is a unit of what
/// the enum's values are, and parses and prints with the prefix's symbol and
/// name.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Prefixed<U> {
    /// The prefix, if any.
    pub prefix: Option<Prefix>,
    /// The unit it scales.
    pub unit: U,
}

impl<U> From<U> for Prefixed<U> {
    fn from(unit: U) -> Self {
        Prefixed { prefix: None, unit }
    }
}

impl<U: fmt::Display> Prefixed<U> {
    /// The number the prefix scales the unit by: its factor raised to the
    /// power of the unit's first symbol (`m²` in `km²` is squared).
    pub fn factor(&self) -> Exact {
        let Some(p) = self.prefix else {
            return Exact::ONE;
        };
        let mut symbol = Symbol::default();
        let _ = fmt::write(&mut symbol, format_args!("{}", self.unit));
        let mut f = Exact::ONE;
        for _ in 0..symbol.first_power() {
            f = f * p.factor();
        }
        f
    }
}

impl<Q, N, U> Unit<Q, N> for Prefixed<U>
where
    U: Unit<Q, N> + fmt::Display,
    N: From<Exact> + core::ops::Mul<Output = N> + core::ops::Div<Output = N>,
{
    fn quantity(&self, value: N) -> Q {
        match self.prefix {
            None => self.unit.quantity(value),
            Some(_) => self.unit.quantity(value * N::from(self.factor())),
        }
    }
    fn value(&self, quantity: Q) -> N {
        match self.prefix {
            None => self.unit.value(quantity),
            Some(_) => self.unit.value(quantity) / N::from(self.factor()),
        }
    }
}

/// A unit's symbol, written into a fixed buffer to read its first factor's
/// power: `m²` is 2, `m/s²` 1, `m³/kg` 3.
struct Symbol {
    buf: [u8; 64],
    len: usize,
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol {
            buf: [0; 64],
            len: 0,
        }
    }
}

impl fmt::Write for Symbol {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let n = s.len().min(self.buf.len() - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&s.as_bytes()[..n]);
        self.len += n;
        Ok(())
    }
}

impl Symbol {
    fn first_power(&self) -> u32 {
        let s = core::str::from_utf8(&self.buf[..self.len]).unwrap_or("");
        let after = s.trim_start_matches(|c: char| c.is_alphabetic() || c == 'µ' || c == '°');
        match after.chars().next() {
            Some('²') => 2,
            Some('³') => 3,
            Some(c) if c.is_ascii_digit() => c.to_digit(10).unwrap_or(1),
            _ => 1,
        }
    }
}

impl<U: fmt::Display> fmt::Display for Prefixed<U> {
    /// `{}` the symbols (`km`, `km²`), `{:#}` the names (`kilometres`,
    /// `square kilometres`); width, fill and alignment apply to the whole.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = Symbol::default();
        let _ = match (self.prefix, f.alternate()) {
            (None, false) => fmt::write(&mut text, format_args!("{}", self.unit)),
            (None, true) => fmt::write(&mut text, format_args!("{:#}", self.unit)),
            (Some(p), false) => fmt::write(&mut text, format_args!("{}{}", p.symbol(), self.unit)),
            (Some(p), true) => {
                let mut name = Symbol::default();
                let _ = fmt::write(&mut name, format_args!("{:#}", self.unit));
                let name = core::str::from_utf8(&name.buf[..name.len]).unwrap_or("");
                // The prefix joins the unit's name after a power's word:
                // square kilometres, cubic millimetres.
                match ["square ", "cubic "]
                    .iter()
                    .find_map(|w| name.strip_prefix(w).map(|r| (w, r)))
                {
                    Some((w, rest)) => fmt::write(&mut text, format_args!("{w}{}{rest}", p.name())),
                    None => fmt::write(&mut text, format_args!("{}{name}", p.name())),
                }
            }
        };
        f.pad(core::str::from_utf8(&text.buf[..text.len]).unwrap_or(""))
    }
}

impl<U> FromStr for Prefixed<U>
where
    U: FromStr<Err = UnknownUnit>,
{
    type Err = UnknownUnit;

    /// A unit's own symbol first (`M` is the nautical mile, `Pa` the pascal,
    /// `min` the minute), then a prefix and a unit's symbol (`km`, `mPa`).
    fn from_str(s: &str) -> Result<Self, UnknownUnit> {
        if let Ok(unit) = s.parse::<U>() {
            return Ok(Prefixed { prefix: None, unit });
        }
        Prefix::split(s)
            .find_map(|(p, rest)| {
                let unit = rest.parse::<U>().ok()?;
                Some(Prefixed {
                    prefix: Some(p),
                    unit,
                })
            })
            .ok_or(UnknownUnit)
    }
}
