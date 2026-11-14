//! A unit: a product of named units, each a base unit or a size of others.

use crate::Exact;
use core::marker::ConstParamTy;
use core::ops::{Add, Div, Mul, Sub};

/// At most this many named units in a product.
const F: usize = __F__;
/// At most this many base units in what a named unit is.
const B: usize = __B__;

/// A unit: a product of named units to powers - merged, without zero
/// powers, in a fixed order. A named unit is a base unit (a dimension of its
/// own), or a size times other units; a prefix fuses with the one named unit
/// it multiplies. A unit of a single named unit may also count from a zero,
/// or on a logarithmic scale: such a unit has no products, and its
/// quantities reach the coherent unit through `From`.
///
/// It is the const parameter of a quantity: `__QTY__<U, N>` is a number of
/// `U`. Quantities of one unit add; their product is counted in the product
/// of the units.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct __UNIT__ {
    prefixes: [&'static str; F],
    symbols: [&'static str; F],
    sizes: [Exact; F],
    powers: [i8; F],
    bases: [[&'static str; B]; F],
    base_powers: [[i8; B]; F],
    zero: Exact,
    log_base: Exact,
    log_per: Exact,
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

/// Adds `sign` times each named thing of `from` into `into`: merged, without
/// zeros, sorted, empties last. `None` when there is no room.
const fn merge<const K: usize>(
    mut syms: [&'static str; K],
    mut pows: [i8; K],
    from: [&'static str; K],
    from_pows: [i8; K],
    sign: i8,
) -> Option<([&'static str; K], [i8; K])> {
    let mut i = 0;
    while i < K {
        if from_pows[i] != 0 {
            let mut j = 0;
            let mut placed = false;
            while j < K && !placed {
                if pows[j] != 0 && str_cmp(syms[j], from[i]) == 0 {
                    pows[j] += sign * from_pows[i];
                    placed = true;
                }
                j += 1;
            }
            j = 0;
            while j < K && !placed {
                if pows[j] == 0 {
                    syms[j] = from[i];
                    pows[j] = sign * from_pows[i];
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
        if pows[k] == 0 {
            syms[k] = "";
        }
        k += 1;
    }
    let mut a = 0;
    while a < K {
        let mut b = a + 1;
        while b < K {
            if (pows[a] == 0 && pows[b] != 0)
                || (pows[a] != 0 && pows[b] != 0 && str_cmp(syms[b], syms[a]) < 0)
            {
                let (s, p) = (syms[a], pows[a]);
                syms[a] = syms[b];
                pows[a] = pows[b];
                syms[b] = s;
                pows[b] = p;
            }
            b += 1;
        }
        a += 1;
    }
    Some((syms, pows))
}

impl __UNIT__ {
    /// The unit of a plain number: no named units.
    pub const ONE: __UNIT__ = __UNIT__ {
        prefixes: [""; F],
        symbols: [""; F],
        sizes: [Exact::ONE; F],
        powers: [0; F],
        bases: [[""; B]; F],
        base_powers: [[0; B]; F],
        zero: Exact::ZERO,
        log_base: Exact::ZERO,
        log_per: Exact::ZERO,
    };

    /// A base unit: a dimension of its own, `METRE`'s `"m"`.
    pub const fn base(symbol: &'static str) -> __UNIT__ {
        let mut u = __UNIT__::ONE;
        u.symbols[0] = symbol;
        u.powers[0] = 1;
        u.bases[0][0] = symbol;
        u.base_powers[0][0] = 1;
        u
    }

    /// A prefix: a plain number that fuses with the one unprefixed named unit
    /// it multiplies - `KILO * GRAM` is `kg`.
    pub const fn prefix(symbol: &'static str, size: Exact) -> __UNIT__ {
        let mut u = __UNIT__::ONE;
        u.prefixes[0] = symbol;
        u.sizes[0] = size;
        u.powers[0] = 1;
        u
    }

    /// A named unit of `size` times `of`, which may be any unit without a
    /// zero or a log scale: the hour is 3600 seconds.
    pub const fn scaled(symbol: &'static str, size: Exact, of: __UNIT__) -> __UNIT__ {
        let mut u = __UNIT__::ONE;
        u.symbols[0] = symbol;
        u.powers[0] = 1;
        u.sizes[0] = size * of.size();
        let (b, p) = of.in_base();
        u.bases[0] = b;
        u.base_powers[0] = p;
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

    const fn is_prefix(self) -> bool {
        self.powers[0] == 1 && self.symbols[0].is_empty() && !self.prefixes[0].is_empty()
    }
    const fn is_linear(self) -> bool {
        self.zero == Exact::ZERO && !self.is_log()
    }
    const fn is_single(self) -> bool {
        self.powers[0] == 1 && self.powers[1] == 0
    }

    /// This unit times `other` to the power `n`: `None` for a unit with a
    /// zero or a log scale, and where the product has too many named units.
    const fn product(self, other: __UNIT__, n: i8) -> Option<__UNIT__> {
        if !self.is_linear() || !other.is_linear() {
            return None;
        }
        if n == 1
            && self.is_prefix()
            && other.is_single()
            && other.prefixes[0].is_empty()
            && !other.is_prefix()
        {
            let mut u = other;
            u.prefixes[0] = self.prefixes[0];
            u.sizes[0] = self.sizes[0] * other.sizes[0];
            return Some(u);
        }
        let mut out = self;
        let mut i = 0;
        while i < F {
            if other.powers[i] != 0 {
                let mut j = 0;
                let mut placed = false;
                while j < F && !placed {
                    if out.powers[j] != 0
                        && str_cmp(out.symbols[j], other.symbols[i]) == 0
                        && str_cmp(out.prefixes[j], other.prefixes[i]) == 0
                    {
                        out.powers[j] += n * other.powers[i];
                        placed = true;
                    }
                    j += 1;
                }
                j = 0;
                while j < F && !placed {
                    if out.powers[j] == 0 {
                        out.prefixes[j] = other.prefixes[i];
                        out.symbols[j] = other.symbols[i];
                        out.sizes[j] = other.sizes[i];
                        out.bases[j] = other.bases[i];
                        out.base_powers[j] = other.base_powers[i];
                        out.powers[j] = n * other.powers[i];
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
    const fn normal(mut self) -> __UNIT__ {
        let mut k = 0;
        while k < F {
            if self.powers[k] == 0 {
                self.prefixes[k] = "";
                self.symbols[k] = "";
                self.sizes[k] = Exact::ONE;
                self.bases[k] = [""; B];
                self.base_powers[k] = [0; B];
            }
            k += 1;
        }
        let mut a = 0;
        while a < F {
            let mut b = a + 1;
            while b < F {
                let later = match str_cmp(self.symbols[b], self.symbols[a]) {
                    0 => str_cmp(self.prefixes[b], self.prefixes[a]) < 0,
                    c => c < 0,
                };
                if (self.powers[a] == 0 && self.powers[b] != 0)
                    || (self.powers[a] != 0 && self.powers[b] != 0 && later)
                {
                    let t = (
                        self.prefixes[a],
                        self.symbols[a],
                        self.sizes[a],
                        self.powers[a],
                        self.bases[a],
                        self.base_powers[a],
                    );
                    self.prefixes[a] = self.prefixes[b];
                    self.symbols[a] = self.symbols[b];
                    self.sizes[a] = self.sizes[b];
                    self.powers[a] = self.powers[b];
                    self.bases[a] = self.bases[b];
                    self.base_powers[a] = self.base_powers[b];
                    self.prefixes[b] = t.0;
                    self.symbols[b] = t.1;
                    self.sizes[b] = t.2;
                    self.powers[b] = t.3;
                    self.bases[b] = t.4;
                    self.base_powers[b] = t.5;
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
        let mut s = Exact::ONE;
        let mut i = 0;
        while i < F {
            let mut p = self.powers[i];
            while p > 0 {
                s = s * self.sizes[i];
                p -= 1;
            }
            while p < 0 {
                s = s / self.sizes[i];
                p += 1;
            }
            i += 1;
        }
        s
    }

    /// What it is in base units - its dimension: base symbols and powers.
    pub const fn in_base(self) -> ([&'static str; B], [i8; B]) {
        let (mut s, mut p) = ([""; B], [0i8; B]);
        let mut i = 0;
        while i < F {
            if self.powers[i] != 0 {
                let mut scaled = self.base_powers[i];
                let mut k = 0;
                while k < B {
                    scaled[k] *= self.powers[i];
                    k += 1;
                }
                (s, p) = match merge(s, p, self.bases[i], scaled, 1) {
                    Some(x) => x,
                    None => panic!("too many base units"),
                };
            }
            i += 1;
        }
        (s, p)
    }

    /// Whether two units measure the same dimension.
    pub const fn same_dimension(self, other: __UNIT__) -> bool {
        let (a, x) = self.in_base();
        let (b, y) = other.in_base();
        let mut i = 0;
        while i < B {
            if x[i] != y[i] || (x[i] != 0 && str_cmp(a[i], b[i]) != 0) {
                return false;
            }
            i += 1;
        }
        true
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

    /// Writes the unit as its named units, each with its prefix and power.
    fn write(self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;
        for k in 0..F {
            if self.powers[k] == 0 {
                continue;
            }
            if !first {
                f.write_str("·")?;
            }
            first = false;
            write!(f, "{}{}", self.prefixes[k], self.symbols[k])?;
            if self.powers[k] != 1 {
                write!(f, "^{}", self.powers[k])?;
            }
        }
        Ok(())
    }
}

/// The unit's symbol: a coherent unit with a name its name (`N`), any other
/// its named units with their prefixes and powers (`km`, `m·s^-1`). The
/// unit of a plain number prints nothing.
impl core::fmt::Display for __UNIT__ {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (unit, name) in crate::NAMED {
            if unit == *self {
                return f.write_str(name);
            }
        }
        self.write(f)
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
