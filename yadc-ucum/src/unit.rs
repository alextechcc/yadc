//! A unit: a product of named units, each a base unit or a size of others.

use crate::Exact;
use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

/// At most this many named units in a product.
const F: usize = 7;
/// At most this many base units in what a named unit is.
const B: usize = 7;

/// A unit: a product of named units to powers - merged, without zero
/// powers, in a fixed order. A named unit is a base unit (a dimension of its
/// own), or a size times other units; a prefix fuses with the one named unit
/// it multiplies. A unit of a single named unit may also count from a zero,
/// or on a logarithmic scale: such a unit has no products, and its
/// quantities reach the coherent unit through `From`.
///
/// It is the const parameter of a quantity: `Qty<U, N>` is a number of
/// `U`. Quantities of one unit add; their product is counted in the product
/// of the units.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct UcumUnit {
    factors: [Factor; F],
    zero: Exact,
    log_base: Exact,
    log_per: Exact,
}

/// One named unit of a product, to a power: its prefix and symbol, its size
/// in base units (the prefix's included), and what it is in base units.
/// Power 0 is an empty slot, `Factor::NONE`.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Factor {
    prefix: &'static str,
    symbol: &'static str,
    size: Exact,
    power: i8,
    base: [(&'static str, i8); B],
}

impl Factor {
    /// An empty slot.
    const NONE: Factor = Factor {
        prefix: "",
        symbol: "",
        size: Exact::ONE,
        power: 0,
        base: [("", 0); B],
    };
}

const fn str_cmp(a: &str, b: &str) -> i8 {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut i = 0;
    while i < a.len() && i < b.len() {
        if a[i] != b[i] {
            return if a[i] < b[i] { -1 } else { 1 };
        }
        i += 1;
    }
    if a.len() < b.len() {
        -1
    } else if a.len() > b.len() {
        1
    } else {
        0
    }
}

/// Adds `sign` times each base unit of `from` into `into`: merged, without
/// zeros, sorted, empties last. `None` when there is no room.
const fn merge<const K: usize>(
    mut into: [(&'static str, i8); K],
    from: [(&'static str, i8); K],
    sign: i8,
) -> Option<[(&'static str, i8); K]> {
    let mut i = 0;
    while i < K {
        if from[i].1 != 0 {
            let mut j = 0;
            let mut placed = false;
            while j < K && !placed {
                if into[j].1 != 0 && str_cmp(into[j].0, from[i].0) == 0 {
                    into[j].1 += sign * from[i].1;
                    placed = true;
                }
                j += 1;
            }
            j = 0;
            while j < K && !placed {
                if into[j].1 == 0 {
                    into[j] = (from[i].0, sign * from[i].1);
                    placed = true;
                }
                j += 1;
            }
            if !placed {
                return None;
            }
        }
        i += 1;
    }
    let mut k = 0;
    while k < K {
        if into[k].1 == 0 {
            into[k].0 = "";
        }
        k += 1;
    }
    let mut a = 0;
    while a < K {
        let mut b = a + 1;
        while b < K {
            if (into[a].1 == 0 && into[b].1 != 0)
                || (into[a].1 != 0 && into[b].1 != 0 && str_cmp(into[b].0, into[a].0) < 0)
            {
                let t = into[a];
                into[a] = into[b];
                into[b] = t;
            }
            b += 1;
        }
        a += 1;
    }
    Some(into)
}

impl UcumUnit {
    /// The unit of a plain number: no named units.
    pub const ONE: UcumUnit = UcumUnit {
        factors: [Factor::NONE; F],
        zero: Exact::ZERO,
        log_base: Exact::ZERO,
        log_per: Exact::ZERO,
    };

    /// A base unit: a dimension of its own, `METRE`'s `"m"`.
    pub const fn base(symbol: &'static str) -> UcumUnit {
        let mut u = UcumUnit::ONE;
        u.factors[0].symbol = symbol;
        u.factors[0].power = 1;
        u.factors[0].base[0] = (symbol, 1);
        u
    }

    /// A prefix: a plain number that fuses with the one unprefixed named unit
    /// it multiplies - `KILO * METRE` is `km`.
    pub const fn prefix(symbol: &'static str, size: Exact) -> UcumUnit {
        let mut u = UcumUnit::ONE;
        u.factors[0].prefix = symbol;
        u.factors[0].size = size;
        u.factors[0].power = 1;
        u
    }

    /// A named unit of `size` times `of`, which may be any unit without a
    /// zero or a log scale: the hour is 3600 seconds.
    pub const fn scaled(symbol: &'static str, size: Exact, of: UcumUnit) -> UcumUnit {
        let mut u = UcumUnit::ONE;
        u.factors[0].symbol = symbol;
        u.factors[0].power = 1;
        u.factors[0].size = size * of.size();
        u.factors[0].base = of.dimension();
        u
    }

    /// A named unit counted from a zero: `n` of it is `n · size + zero` of
    /// `of` - the degree Celsius is 1 kelvin counted from 273.15 K.
    pub const fn offset(symbol: &'static str, size: Exact, zero: Exact, of: UcumUnit) -> UcumUnit {
        let mut u = UcumUnit::scaled(symbol, size, of);
        u.zero = zero * of.size();
        u
    }

    /// A named unit on a logarithmic scale: `n` of it is `reference ·
    /// base^(n / per)` of `of` - dBm is base 10, per 10, from a milliwatt.
    pub const fn from_log(
        symbol: &'static str,
        base: Exact,
        per: Exact,
        reference: Exact,
        of: UcumUnit,
    ) -> UcumUnit {
        let mut u = UcumUnit::scaled(symbol, reference, of);
        u.log_base = base;
        u.log_per = per;
        u
    }

    const fn is_prefix(self) -> bool {
        let f = self.factors[0];
        f.power == 1 && f.symbol.is_empty() && !f.prefix.is_empty()
    }
    const fn is_linear(self) -> bool {
        self.zero == Exact::ZERO && !self.is_log()
    }
    const fn is_single(self) -> bool {
        self.factors[0].power == 1 && self.factors[1].power == 0
    }

    /// This unit times `other` to the power `n`: `None` for a unit with a
    /// zero or a log scale, and where the product has too many named units.
    const fn product(self, other: UcumUnit, n: i8) -> Option<UcumUnit> {
        if !self.is_linear() || !other.is_linear() {
            return None;
        }
        if n == 1
            && self.is_prefix()
            && other.is_single()
            && other.factors[0].prefix.is_empty()
            && !other.is_prefix()
        {
            let mut u = other;
            u.factors[0].prefix = self.factors[0].prefix;
            u.factors[0].size = self.factors[0].size * other.factors[0].size;
            return Some(u);
        }
        let mut out = self;
        let mut i = 0;
        while i < F {
            let o = other.factors[i];
            if o.power != 0 {
                let mut j = 0;
                let mut placed = false;
                while j < F && !placed {
                    let f = out.factors[j];
                    if f.power != 0
                        && str_cmp(f.symbol, o.symbol) == 0
                        && str_cmp(f.prefix, o.prefix) == 0
                    {
                        out.factors[j].power += n * o.power;
                        placed = true;
                    }
                    j += 1;
                }
                j = 0;
                while j < F && !placed {
                    if out.factors[j].power == 0 {
                        out.factors[j] = Factor {
                            power: n * o.power,
                            ..o
                        };
                        placed = true;
                    }
                    j += 1;
                }
                if !placed {
                    return None;
                }
            }
            i += 1;
        }
        Some(out.normal())
    }

    /// Empty slots cleared and last; the rest by symbol, then prefix.
    const fn normal(mut self) -> UcumUnit {
        let mut k = 0;
        while k < F {
            if self.factors[k].power == 0 {
                self.factors[k] = Factor::NONE;
            }
            k += 1;
        }
        let mut a = 0;
        while a < F {
            let mut b = a + 1;
            while b < F {
                let (x, y) = (self.factors[a], self.factors[b]);
                let later = match str_cmp(y.symbol, x.symbol) {
                    0 => str_cmp(y.prefix, x.prefix) < 0,
                    c => c < 0,
                };
                if (x.power == 0 && y.power != 0) || (x.power != 0 && y.power != 0 && later) {
                    self.factors[a] = y;
                    self.factors[b] = x;
                }
                b += 1;
            }
            a += 1;
        }
        self
    }

    /// This unit times `other` to the power `n`: `METRE.unit().times(SECOND.unit(), -1)`
    /// is m/s. A panic (in a constant, a compile error) where there is no
    /// product.
    pub const fn times(self, other: UcumUnit, n: i8) -> UcumUnit {
        match self.product(other, n) {
            Some(u) => u,
            None => panic!("these units have no product"),
        }
    }

    /// Its size in base units.
    pub const fn size(self) -> Exact {
        let mut s = Exact::ONE;
        let mut i = 0;
        while i < F {
            let f = self.factors[i];
            let mut p = f.power;
            while p > 0 {
                s = s * f.size;
                p -= 1;
            }
            while p < 0 {
                s = s / f.size;
                p += 1;
            }
            i += 1;
        }
        s
    }

    /// What it is in base units - its dimension: base symbols and powers.
    pub const fn in_base(self) -> ([&'static str; B], [i8; B]) {
        let base = self.dimension();
        let (mut symbols, mut powers) = ([""; B], [0; B]);
        let mut i = 0;
        while i < B {
            (symbols[i], powers[i]) = base[i];
            i += 1;
        }
        (symbols, powers)
    }

    /// Its dimension as base symbols with their powers, sorted, empties last.
    const fn dimension(self) -> [(&'static str, i8); B] {
        let mut out = [("", 0); B];
        let mut i = 0;
        while i < F {
            let f = self.factors[i];
            if f.power != 0 {
                let mut scaled = f.base;
                let mut k = 0;
                while k < B {
                    scaled[k].1 *= f.power;
                    k += 1;
                }
                out = match merge(out, scaled, 1) {
                    Some(x) => x,
                    None => panic!("too many base units"),
                };
            }
            i += 1;
        }
        out
    }

    /// Whether two units measure the same dimension.
    pub const fn same_dimension(self, other: UcumUnit) -> bool {
        let (a, b) = (self.dimension(), other.dimension());
        let mut i = 0;
        while i < B {
            if a[i].1 != b[i].1 || (a[i].1 != 0 && str_cmp(a[i].0, b[i].0) != 0) {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Whether it is the unit of a plain number: of dimension one, without a
    /// zero or a log scale - `m/km`, `%`, `k`.
    pub const fn is_plain_number(self) -> bool {
        let base = self.dimension();
        let mut i = 0;
        while i < B {
            if base[i].1 != 0 {
                return false;
            }
            i += 1;
        }
        self.is_linear()
    }

    /// Whether it is the unit of a plain number of a whole size: `km/m`, not
    /// `m/km`.
    pub const fn is_whole_plain_number(self) -> bool {
        self.is_plain_number() && self.size().is_integer()
    }

    /// Whether it is on a logarithmic scale.
    pub const fn is_log(self) -> bool {
        self.log_per != Exact::ZERO
    }

    /// The number of base units `n` of this linear unit is: `n · size +
    /// zero`.
    pub const fn to_coherent<N>(self, n: N) -> N
    where
        N: [const] From<Exact> + [const] Mul<Output = N> + [const] Add<Output = N>,
    {
        n * N::from(self.size()) + N::from(self.zero)
    }

    /// The number of this linear unit `x` base units is.
    pub const fn from_coherent<N>(self, x: N) -> N
    where
        N: [const] From<Exact> + [const] Sub<Output = N> + [const] Div<Output = N>,
    {
        (x - N::from(self.zero)) / N::from(self.size())
    }

    /// The number of base units `n` of this log unit is: `reference ·
    /// base^(n / per)`.
    pub fn log_to_coherent<N: num_traits::real::Real + From<Exact>>(self, n: N) -> N {
        let exact = <N as From<Exact>>::from;
        exact(self.size()) * exact(self.log_base).powf(n / exact(self.log_per))
    }

    /// The number of this log unit `x` base units is.
    pub fn log_from_coherent<N: num_traits::real::Real + From<Exact>>(self, x: N) -> N {
        let exact = <N as From<Exact>>::from;
        exact(self.log_per) * (x / exact(self.size())).log(exact(self.log_base))
    }

    /// The number of base units `n` of this unit is, either scale.
    pub fn to_base(self, n: f64) -> f64 {
        if self.is_log() {
            self.log_to_coherent(n)
        } else {
            self.to_coherent(n)
        }
    }

    /// The number of this unit `x` base units is, either scale.
    pub fn from_base(self, x: f64) -> f64 {
        if self.is_log() {
            self.log_from_coherent(x)
        } else {
            self.from_coherent(x)
        }
    }

    /// Writes the unit: a coherent unit with a name its name (`N`), a prefix
    /// on one its prefix and name (`kPa`), any other its named units with
    /// their prefixes and powers, the positive powers first, then `/` and the
    /// negative ones (`km/h`, `m/s^2`, `kg·m^2/s^2`, `1/h`, `W/(m^2·K)`).
    fn write(self, w: &mut impl core::fmt::Write) -> core::fmt::Result {
        if let Some(name) = self.name() {
            return w.write_str(name);
        }
        let mut k = 0;
        while k < F {
            let f = self.factors[k];
            if f.power == 1 && f.symbol.is_empty() && !f.prefix.is_empty() {
                let mut rest = self;
                rest.factors[k].power = 0;
                if let Some(name) = rest.normal().name() {
                    w.write_str(f.prefix)?;
                    return w.write_str(name);
                }
            }
            k += 1;
        }
        let count = |positive: bool| {
            self.factors
                .iter()
                .filter(|f| f.power != 0 && (f.power > 0) == positive)
                .count()
        };
        let (num, den) = (count(true), count(false));
        if num == 0 && den > 0 {
            w.write_str("1")?;
        }
        self.write_powers(w, true)?;
        if den > 0 {
            w.write_str("/")?;
            if den > 1 {
                w.write_str("(")?;
            }
            self.write_powers(w, false)?;
            if den > 1 {
                w.write_str(")")?;
            }
        }
        Ok(())
    }

    /// The name of a coherent unit that has one.
    fn name(self) -> Option<&'static str> {
        crate::NAMED
            .iter()
            .find(|(unit, _)| *unit == self)
            .map(|(_, name)| *name)
    }

    /// The named units of positive powers, or of negative ones, with their
    /// prefixes and the powers' sizes: `kg·m^2`.
    fn write_powers(self, w: &mut impl core::fmt::Write, positive: bool) -> core::fmt::Result {
        let mut first = true;
        for k in 0..F {
            let f = self.factors[k];
            let p = f.power;
            if p == 0 || (p > 0) != positive {
                continue;
            }
            if !first {
                w.write_str("·")?;
            }
            first = false;
            write!(w, "{}{}", f.prefix, f.symbol)?;
            if p.abs() != 1 {
                write!(w, "^{}", p.abs())?;
            }
        }
        Ok(())
    }
}

/// The longest unit symbol `Display` writes.
const LONGEST: usize = 256;

/// A unit's symbol, written into a fixed buffer to be padded as a whole.
struct Symbol {
    bytes: [u8; LONGEST],
    len: usize,
}

impl core::fmt::Write for Symbol {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let end = self.len + s.len();
        if end > LONGEST {
            return Err(core::fmt::Error);
        }
        self.bytes[self.len..end].copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

/// The unit's symbol (see `write`), padded to the width, fill and
/// alignment asked for: `{:>6}`. The unit of a plain number prints nothing.
impl core::fmt::Display for UcumUnit {
    /// The symbol; a width pads it with the fill, aligned as asked (left by
    /// default). A precision does not cut it.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Some(width) = f.width() else {
            return self.write(f);
        };
        let mut symbol = Symbol {
            bytes: [0; LONGEST],
            len: 0,
        };
        if self.write(&mut symbol).is_err() {
            // Longer than any width a table asks for: written as it is.
            return self.write(f);
        }
        // Whole `str`s were written, so the bytes are UTF-8.
        let text =
            core::str::from_utf8(&symbol.bytes[..symbol.len]).map_err(|_| core::fmt::Error)?;
        let pad = width.saturating_sub(text.chars().count());
        let (before, after) = match f.align() {
            Some(core::fmt::Alignment::Right) => (pad, 0),
            Some(core::fmt::Alignment::Center) => (pad / 2, pad - pad / 2),
            _ => (0, pad),
        };
        let fill = f.fill();
        for _ in 0..before {
            core::fmt::Write::write_char(f, fill)?;
        }
        f.write_str(text)?;
        for _ in 0..after {
            core::fmt::Write::write_char(f, fill)?;
        }
        Ok(())
    }
}

impl const Mul for UcumUnit {
    type Output = Option<UcumUnit>;
    fn mul(self, o: UcumUnit) -> Option<UcumUnit> {
        self.product(o, 1)
    }
}
impl const Div for UcumUnit {
    type Output = Option<UcumUnit>;
    fn div(self, o: UcumUnit) -> Option<UcumUnit> {
        self.product(o, -1)
    }
}
/// The unit of a quantity scaled by a number: its own.
impl const Mul<f64> for UcumUnit {
    type Output = UcumUnit;
    fn mul(self, _: f64) -> UcumUnit {
        self
    }
}
/// The unit of a quantity divided by a number: its own.
impl const Div<f64> for UcumUnit {
    type Output = UcumUnit;
    fn div(self, _: f64) -> UcumUnit {
        self
    }
}
/// One unit adds to itself; nothing else adds.
impl const Add for UcumUnit {
    type Output = Option<UcumUnit>;
    fn add(self, o: UcumUnit) -> Option<UcumUnit> {
        if self == o { Some(self) } else { None }
    }
}
impl const Sub for UcumUnit {
    type Output = Option<UcumUnit>;
    fn sub(self, o: UcumUnit) -> Option<UcumUnit> {
        if self == o { Some(self) } else { None }
    }
}
