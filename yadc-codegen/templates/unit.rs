//! A unit: a product of named units, each a base unit or a size of others.

use crate::Exact;
use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

/// At most this many named units in a product.
const F: usize = __F__;
/// At most this many base units in what a named unit is.
const B: usize = __B__;

/// A unit: an exact scale times a product of named units to powers -
/// merged, without zero powers, in a fixed order. A named unit is a base unit
/// (a dimension of its own), or a size times other units. A prefix is a
/// scale: every prefix of a product multiplies into the one scale, so a unit
/// has one form however it is built - `KILO * KILO` is `MEGA`, a kilometre
/// per metre is `KILO`, a kilometre times a millimetre is a square metre. A
/// unit of a single named unit may also count from a zero, or on a
/// logarithmic scale: such a unit has no products, and its quantities reach
/// the coherent unit through `From`.
///
/// It is the const parameter of a quantity: `__QTY__<U, N>` is a number of
/// `U`. Quantities of one unit add; their product is counted in the product
/// of the units.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct __UNIT__ {
    scale: Exact,
    factors: [Factor; F],
    zero: Exact,
    log_base: Exact,
    log_per: Exact,
}

/// One named unit of a product, to a power: its symbol, its size in base
/// units, and what it is in base units. Power 0 is an empty slot,
/// `Factor::NONE`.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Factor {
    symbol: &'static str,
    size: Exact,
    power: i8,
    base: [(&'static str, i8); B],
}

impl Factor {
    /// An empty slot.
    const NONE: Factor = Factor {
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

/// `x` to the power `n`.
const fn pow(x: Exact, n: i8) -> Exact {
    let mut r = Exact::ONE;
    let mut p = n;
    while p > 0 {
        r = r * x;
        p -= 1;
    }
    while p < 0 {
        r = r / x;
        p += 1;
    }
    r
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
                into.swap(a, b);
            }
            b += 1;
        }
        a += 1;
    }
    Some(into)
}

impl __UNIT__ {
    /// The unit of a plain number: no named units, scale one.
    pub const ONE: __UNIT__ = __UNIT__ {
        scale: Exact::ONE,
        factors: [Factor::NONE; F],
        zero: Exact::ZERO,
        log_base: Exact::ZERO,
        log_per: Exact::ZERO,
    };

    /// A base unit: a dimension of its own, `METRE`'s `"m"`.
    pub const fn base(symbol: &'static str) -> __UNIT__ {
        let mut u = __UNIT__::ONE;
        u.factors[0].symbol = symbol;
        u.factors[0].power = 1;
        u.factors[0].base[0] = (symbol, 1);
        u
    }

    /// A prefix: a plain number of `size`, the scale of the units it
    /// multiplies - `KILO * METRE` is a kilometre.
    pub const fn prefix(size: Exact) -> __UNIT__ {
        let mut u = __UNIT__::ONE;
        u.scale = size;
        u
    }

    /// A named unit of `size` times `of`, which may be any unit without a
    /// zero or a log scale: the hour is 3600 seconds.
    pub const fn scaled(symbol: &'static str, size: Exact, of: __UNIT__) -> __UNIT__ {
        let mut u = __UNIT__::ONE;
        u.factors[0].symbol = symbol;
        u.factors[0].power = 1;
        u.factors[0].size = size * of.size();
        u.factors[0].base = of.dimension();
        u
    }

    /// A named unit counted from a zero: `n` of it is `n · size + zero` of
    /// `of` - the degree Celsius is 1 kelvin counted from 273.15 K.
    pub const fn offset(symbol: &'static str, size: Exact, zero: Exact, of: __UNIT__) -> __UNIT__ {
        let mut u = __UNIT__::scaled(symbol, size, of);
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
        of: __UNIT__,
    ) -> __UNIT__ {
        let mut u = __UNIT__::scaled(symbol, reference, of);
        u.log_base = base;
        u.log_per = per;
        u
    }

    const fn is_linear(self) -> bool {
        self.zero == Exact::ZERO && !self.is_log()
    }

    /// This unit times `other` to the power `n`: `None` for a unit with a
    /// zero or a log scale, and where the product has too many named units.
    const fn product(self, other: __UNIT__, n: i8) -> Option<__UNIT__> {
        if !self.is_linear() || !other.is_linear() {
            return None;
        }
        let mut out = self;
        out.scale = self.scale * pow(other.scale, n);
        let mut i = 0;
        while i < F {
            let o = other.factors[i];
            if o.power != 0 {
                let mut j = 0;
                let mut placed = false;
                while j < F && !placed {
                    let f = out.factors[j];
                    if f.power != 0 && str_cmp(f.symbol, o.symbol) == 0 {
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

    /// Empty slots cleared and last; the rest by symbol.
    const fn normal(mut self) -> __UNIT__ {
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
                if (x.power == 0 && y.power != 0)
                    || (x.power != 0 && y.power != 0 && str_cmp(y.symbol, x.symbol) < 0)
                {
                    self.factors.swap(a, b);
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
    pub const fn times(self, other: __UNIT__, n: i8) -> __UNIT__ {
        match self.product(other, n) {
            Some(u) => u,
            None => panic!("these units have no product"),
        }
    }

    /// Its size in base units.
    pub const fn size(self) -> Exact {
        let mut s = self.scale;
        let mut i = 0;
        while i < F {
            s = s * pow(self.factors[i].size, self.factors[i].power);
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
    pub const fn same_dimension(self, other: __UNIT__) -> bool {
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

    /// Writes the unit. A coherent unit with a name: the prefix its scale is
    /// and its name (`N`, `kPa`). A plain number: its prefix (`k`). Any
    /// other: its named units with their powers, the positive powers first,
    /// then `/` and the negative ones, its scale a prefix on one of them to
    /// that one's power (`km/h`, `m/s^2`, `kg·m^2/s^2`, `1/h`, `W/(m^2·K)`,
    /// `km^2`), or a number before them where no prefix is (`1000·m^2`).
    fn write(self, w: &mut impl core::fmt::Write) -> core::fmt::Result {
        if let Some((prefix, name)) = self.named() {
            w.write_str(prefix)?;
            return w.write_str(name);
        }
        if self.factors[0].power == 0 {
            return match prefix_of(self.scale, 1) {
                Some(prefix) => w.write_str(prefix),
                None => write!(w, "{}", self.scale),
            };
        }
        match self.spelling() {
            Some(spelling) => self.write_factors(w, spelling),
            None => {
                write!(w, "{}·", self.scale)?;
                self.write_factors(w, Spelling::PLAIN)
            }
        }
    }

    /// The coherent unit with a name this unit is a prefix times: the prefix
    /// and the name.
    fn named(self) -> Option<(&'static str, &'static str)> {
        if !self.is_linear() {
            return None;
        }
        crate::NAMED.iter().find_map(|(unit, name)| {
            if unit.factors != self.factors {
                return None;
            }
            Some((prefix_of(self.scale / unit.scale, 1)?, *name))
        })
    }

    /// Its scale as a prefix on one named unit: the positive powers first, a
    /// base unit with a prefix in its symbol (`kg`) last, as its unprefixed
    /// unit (`mg`).
    fn spelling(self) -> Option<Spelling> {
        if self.scale == Exact::ONE {
            return Some(Spelling::PLAIN);
        }
        for prefixed_base in [false, true] {
            for positive in [true, false] {
                for (at, f) in self.factors.iter().enumerate() {
                    if f.power == 0 || (f.power > 0) != positive {
                        continue;
                    }
                    let (symbol, size) = match (unprefixed(f.symbol), prefixed_base) {
                        (None, false) => (f.symbol, Exact::ONE),
                        (Some(unprefixed), true) => unprefixed,
                        _ => continue,
                    };
                    if let Some(prefix) = prefix_of(self.scale / pow(size, f.power), f.power) {
                        return Some(Spelling { at, prefix, symbol });
                    }
                }
            }
        }
        None
    }

    /// Its named units, `spelling`'s one with its prefix and symbol.
    fn write_factors(self, w: &mut impl core::fmt::Write, spelling: Spelling) -> core::fmt::Result {
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
        self.write_powers(w, true, spelling)?;
        if den > 0 {
            w.write_str("/")?;
            if den > 1 {
                w.write_str("(")?;
            }
            self.write_powers(w, false, spelling)?;
            if den > 1 {
                w.write_str(")")?;
            }
        }
        Ok(())
    }

    /// The named units of positive powers, or of negative ones, with the
    /// powers' sizes: `kg·m^2`.
    fn write_powers(
        self,
        w: &mut impl core::fmt::Write,
        positive: bool,
        spelling: Spelling,
    ) -> core::fmt::Result {
        let mut first = true;
        for (at, f) in self.factors.iter().enumerate() {
            let p = f.power;
            if p == 0 || (p > 0) != positive {
                continue;
            }
            if !first {
                w.write_str("·")?;
            }
            first = false;
            if at == spelling.at {
                write!(w, "{}{}", spelling.prefix, spelling.symbol)?;
            } else {
                w.write_str(f.symbol)?;
            }
            if p.abs() != 1 {
                write!(w, "^{}", p.abs())?;
            }
        }
        Ok(())
    }

    /// How `{:e}` writes a quantity of this unit: where it is a product of
    /// base units, linear, one of its named units (the first of a positive
    /// power) or its name takes a prefix. A plain number is written as a
    /// number.
    pub(crate) fn engineering(self) -> Option<Engineering> {
        if !self.is_linear() || self.factors.iter().any(|f| f.power != 0 && f.size != Exact::ONE) {
            return None;
        }
        if self.factors[0].power == 0 {
            return Some(Engineering {
                unit: self,
                target: Target::Number,
                power: 1,
                one: Exact::ONE / self.scale,
            });
        }
        if let Some((unit, name)) = crate::NAMED.iter().find(|(unit, _)| unit.factors == self.factors) {
            return Some(Engineering {
                unit: self,
                target: Target::Name(name),
                power: 1,
                one: unit.scale / self.scale,
            });
        }
        let at = self.factors.iter().position(|f| f.power > 0).unwrap_or(0);
        let f = self.factors[at];
        let (symbol, size) = unprefixed(f.symbol).unwrap_or((f.symbol, Exact::ONE));
        Some(Engineering {
            unit: self,
            target: Target::Factor(at, symbol),
            power: f.power,
            one: pow(size, f.power) / self.scale,
        })
    }
}

/// The prefix of `size` to the power `power` that is `x`: `""` for one.
fn prefix_of(x: Exact, power: i8) -> Option<&'static str> {
    if x == Exact::ONE {
        return Some("");
    }
    crate::PREFIXES
        .iter()
        .find(|(_, exp)| pow(Exact::dec(1, *exp), power) == x)
        .map(|(symbol, _)| *symbol)
}

/// A base unit with a prefix in its symbol as its unprefixed unit: `kg` as
/// `g`, of 0.001 kg.
fn unprefixed(symbol: &str) -> Option<(&'static str, Exact)> {
    crate::PREFIXED_BASES
        .iter()
        .find(|(base, _, _)| *base == symbol)
        .map(|(_, unprefixed, size)| (*unprefixed, *size))
}

/// One named unit of a product written with a prefix and a symbol of its
/// own: `at` its slot.
#[derive(Copy, Clone)]
struct Spelling {
    at: usize,
    prefix: &'static str,
    symbol: &'static str,
}

impl Spelling {
    /// Every named unit as it is.
    const PLAIN: Spelling = Spelling {
        at: F,
        prefix: "",
        symbol: "",
    };
}

/// What takes the prefix in `{:e}`.
#[derive(Copy, Clone)]
enum Target {
    /// Nothing: a plain number.
    Number,
    /// A coherent unit's name.
    Name(&'static str),
    /// A named unit, its slot and its unprefixed symbol.
    Factor(usize, &'static str),
}

/// A unit as `{:e}` writes it: `target` prefixed. `one` is the size, in the
/// unit, of the unprefixed target to its `power`.
pub(crate) struct Engineering {
    unit: __UNIT__,
    target: Target,
    power: i8,
    one: Exact,
}

impl Engineering {
    /// The size, in the unit, of one of what a quantity of a plain number is
    /// written as a number of.
    pub(crate) fn number(&self) -> Option<Exact> {
        match self.target {
            Target::Number => Some(self.one),
            _ => None,
        }
    }

    /// The prefixes it may take, none and those of powers of 1000, each with
    /// the size, in the unit, of one of it written so.
    pub(crate) fn prefixes(&self) -> impl Iterator<Item = (&'static str, Exact)> + '_ {
        core::iter::once(("", 0))
            .chain(crate::PREFIXES.iter().copied().filter(|(_, exp)| exp % 3 == 0))
            .map(|(prefix, exp)| (prefix, pow(Exact::dec(1, exp), self.power) * self.one))
    }

    /// The unit, its target with `prefix`.
    pub(crate) fn write(&self, w: &mut impl core::fmt::Write, prefix: &'static str) -> core::fmt::Result {
        match self.target {
            Target::Number => Ok(()),
            Target::Name(name) => {
                w.write_str(prefix)?;
                w.write_str(name)
            }
            Target::Factor(at, symbol) => self.unit.write_factors(w, Spelling { at, prefix, symbol }),
        }
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
impl core::fmt::Display for __UNIT__ {
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
        let text = core::str::from_utf8(&symbol.bytes[..symbol.len]).map_err(|_| core::fmt::Error)?;
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

const impl Mul for __UNIT__ {
    type Output = Option<__UNIT__>;
    fn mul(self, o: __UNIT__) -> Option<__UNIT__> {
        self.product(o, 1)
    }
}
const impl Div for __UNIT__ {
    type Output = Option<__UNIT__>;
    fn div(self, o: __UNIT__) -> Option<__UNIT__> {
        self.product(o, -1)
    }
}
/// The unit of a quantity scaled by a number: its own.
const impl Mul<f64> for __UNIT__ {
    type Output = __UNIT__;
    fn mul(self, _: f64) -> __UNIT__ {
        self
    }
}
/// The unit of a quantity divided by a number: its own.
const impl Div<f64> for __UNIT__ {
    type Output = __UNIT__;
    fn div(self, _: f64) -> __UNIT__ {
        self
    }
}
/// One unit adds to itself; nothing else adds.
const impl Add for __UNIT__ {
    type Output = Option<__UNIT__>;
    fn add(self, o: __UNIT__) -> Option<__UNIT__> {
        if self == o { Some(self) } else { None }
    }
}
const impl Sub for __UNIT__ {
    type Output = Option<__UNIT__>;
    fn sub(self, o: __UNIT__) -> Option<__UNIT__> {
        if self == o { Some(self) } else { None }
    }
}
