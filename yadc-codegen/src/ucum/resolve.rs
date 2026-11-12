//! Evaluate UCUM unit expressions to dimension vectors + multipliers.

use super::parse::UcumEssence;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::collections::{BTreeMap, HashMap};

/// An exact factor: a rational times a power of π. UCUM writes every value
/// as a decimal, and π (`[pi]`) as a long one, which is read as π itself.
#[derive(Debug, Clone, PartialEq)]
pub struct Factor {
    pub ratio: BigRational,
    pub pi: i32,
}

impl Factor {
    pub fn one() -> Self {
        Factor { ratio: BigRational::one(), pi: 0 }
    }

    /// π.
    pub fn pi() -> Self {
        Factor { ratio: BigRational::one(), pi: 1 }
    }

    /// `10^n`.
    fn ten_to(n: i32) -> Self {
        let p = BigInt::from(10).pow(n.unsigned_abs());
        let ratio = if n >= 0 {
            BigRational::from_integer(p)
        } else {
            BigRational::new(BigInt::one(), p)
        };
        Factor { ratio, pi: 0 }
    }

    /// A decimal as written: `2.54`, `1e24`, `6.62607015e-34`.
    pub fn decimal(s: &str) -> Option<Self> {
        let s = s.trim();
        let (mantissa, exp) = match s.find(['e', 'E']) {
            Some(k) => (&s[..k], s[k + 1..].parse::<i32>().ok()?),
            None => (s, 0),
        };
        let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let digits: BigInt = format!("{int}{frac}").parse().ok()?;
        let mut f = Factor { ratio: BigRational::from_integer(digits), pi: 0 };
        f = f.mul(&Factor::ten_to(exp - frac.len() as i32));
        Some(f)
    }

    pub fn mul(&self, o: &Factor) -> Factor {
        Factor { ratio: &self.ratio * &o.ratio, pi: self.pi + o.pi }
    }

    pub fn div(&self, o: &Factor) -> Factor {
        Factor { ratio: &self.ratio / &o.ratio, pi: self.pi - o.pi }
    }

    pub fn pow(&self, n: i32) -> Factor {
        let r = num_traits::pow(self.ratio.clone(), n.unsigned_abs() as usize);
        Factor { ratio: if n >= 0 { r } else { r.recip() }, pi: self.pi * n }
    }

    /// The factor as an expression of number literals, exactly: `254e-4`,
    /// `100 / 3937`, `1 / 180 * core::f64::consts::PI`. The generator
    /// writes each literal as an `Exact`.
    pub fn expr(&self) -> String {
        // `n · 10^k`, with `n` free of trailing zeros.
        fn split(mut n: BigInt) -> (BigInt, i32) {
            let ten = BigInt::from(10);
            let mut k = 0;
            while !n.is_zero() && (&n % &ten).is_zero() {
                n /= &ten;
                k += 1;
            }
            (n, k)
        }
        // A denominator of only 2s and 5s goes into a power of ten: 127/5000
        // is 254e-4. Any other stays a quotient: 1 / 180.
        let mut n = self.ratio.numer().clone();
        let mut d = self.ratio.denom().clone();
        let mut k = 0;
        for p in [2, 5] {
            let (p, q) = (BigInt::from(p), BigInt::from(10 / p));
            while (&d % &p).is_zero() {
                d /= &p;
                n *= &q;
                k += 1;
            }
        }
        if !d.is_one() {
            (n, d, k) = (self.ratio.numer().clone(), self.ratio.denom().clone(), 0);
        }
        let (n, a) = split(n);
        let e = a - k;
        let num = if e == 0 { format!("{n}") } else { format!("{n}e{e}") };
        let mut out = if d.is_one() { num } else { format!("{num} / {d}") };
        debug_assert!(!self.ratio.is_negative());
        for _ in 0..self.pi.unsigned_abs() {
            out += if self.pi > 0 { " * core::f64::consts::PI" } else { " / core::f64::consts::PI" };
        }
        out
    }
}

/// Dimension exponent map: field_name -> exponent.  Zero exponents are elided,
/// so an empty map means "dimensionless".
pub type DimVec = BTreeMap<String, i32>;

#[derive(Debug, Clone)]
pub struct Resolved {
    pub dim: DimVec,
    pub multiplier: Factor,
    /// Kind variant name, or `None` for the default linear kind.
    pub kind: Option<String>,
}

impl Resolved {
    fn scalar(v: Factor) -> Self {
        Resolved {
            dim: DimVec::new(),
            multiplier: v,
            kind: None,
        }
    }

    /// A plain number: no dimension and no label.
    fn is_plain(&self) -> bool {
        self.dim.is_empty() && self.kind.is_none()
    }

    pub fn mul(self, other: Self) -> Self {
        Resolved {
            dim: dim_add(&self.dim, &other.dim),
            multiplier: self.multiplier.mul(&other.multiplier),
            kind: kind_mul(&self, &other),
        }
    }

    pub fn div(self, other: Self) -> Self {
        Resolved {
            dim: dim_sub(&self.dim, &other.dim),
            multiplier: self.multiplier.div(&other.multiplier),
            kind: kind_div(&self, &other),
        }
    }

    pub fn pow(self, n: i32) -> Self {
        Resolved {
            dim: dim_scale(&self.dim, n),
            multiplier: self.multiplier.pow(n),
            kind: kind_pow(self.kind.as_deref(), n),
        }
    }
}

fn dim_add(a: &DimVec, b: &DimVec) -> DimVec {
    let mut r = a.clone();
    for (k, v) in b {
        *r.entry(k.clone()).or_insert(0) += v;
    }
    r.retain(|_, v| *v != 0);
    r
}

fn dim_sub(a: &DimVec, b: &DimVec) -> DimVec {
    dim_add(a, &dim_scale(b, -1))
}

fn dim_scale(a: &DimVec, n: i32) -> DimVec {
    if n == 0 {
        return DimVec::new();
    }
    a.iter().map(|(k, v)| (k.clone(), v * n)).collect()
}

// Kinds are labels, as `#[derive(Dimension)]` treats them: a product or
// quotient keeps a label only beside a plain number (`2.Cel` keeps its label,
// `Cel.m` does not), a power keeps it only to the first power.
fn kind_mul(a: &Resolved, b: &Resolved) -> Option<String> {
    if a.is_plain() {
        b.kind.clone()
    } else if b.is_plain() {
        a.kind.clone()
    } else {
        None
    }
}

fn kind_div(a: &Resolved, b: &Resolved) -> Option<String> {
    if b.is_plain() { a.kind.clone() } else { None }
}

fn kind_pow(k: Option<&str>, n: i32) -> Option<String> {
    match (k, n) {
        (Some(s), 1) => Some(s.into()),
        _ => None,
    }
}

/// Resolve all UCUM units to (dimension vector, multiplier, kind): each
/// UCUM base unit is its own axis (its code, exponent 1, times one), and
/// every other unit is resolved from them.
pub fn resolve_all(essence: &UcumEssence) -> HashMap<String, Resolved> {
    let mut db: HashMap<String, Resolved> = HashMap::new();
    for base in &essence.base_units {
        let mut dim = DimVec::new();
        dim.insert(base.code.clone(), 1);
        db.insert(
            base.code.clone(),
            Resolved {
                dim,
                multiplier: Factor::one(),
                kind: None,
            },
        );
    }

    // Build prefix lookup (longest code first for greedy match)
    let mut prefixes: Vec<(String, Factor)> = essence
        .prefixes
        .iter()
        .map(|p| (p.code.clone(), Factor::decimal(&p.value).expect("a prefix's value")))
        .collect();
    prefixes.sort_by_key(|(code, _)| -(code.len() as i64));

    let non_special: Vec<_> = essence
        .units
        .iter()
        .filter(|u| !u.is_special && !u.is_arbitrary && !u.value_unit.is_empty())
        .collect();

    // Iteratively resolve derived units until stable
    for _ in 0..20 {
        let prev = db.len();
        for unit in &non_special {
            if db.contains_key(&unit.code) {
                continue;
            }
            let lookup = |sym: &str| lookup_sym(sym, &db, &prefixes);
            // UCUM writes π as a long decimal; it is π.
            if unit.code == "[pi]" {
                db.insert(unit.code.clone(), Resolved::scalar(Factor::pi()));
                continue;
            }
            let Some(number) = Factor::decimal(&unit.value_number) else {
                continue;
            };
            if let Some(r) = eval_expr(&unit.value_unit, &lookup) {
                db.insert(
                    unit.code.clone(),
                    Resolved {
                        multiplier: r.multiplier.mul(&number),
                        ..r
                    },
                );
            }
        }
        if db.len() == prev {
            break;
        }
    }

    db
}

fn lookup_sym(
    sym: &str,
    db: &HashMap<String, Resolved>,
    prefixes: &[(String, Factor)],
) -> Option<Resolved> {
    if let Some(r) = db.get(sym) {
        return Some(r.clone());
    }
    for (code, val) in prefixes {
        if sym.len() > code.len() && sym.starts_with(code.as_str()) {
            let atom = &sym[code.len()..];
            if let Some(r) = db.get(atom) {
                return Some(Resolved {
                    multiplier: r.multiplier.mul(val),
                    ..r.clone()
                });
            }
        }
    }
    None
}

/// The value of a whole expression: `None` unless every byte of it is read.
fn eval_expr<F: Fn(&str) -> Option<Resolved>>(expr: &str, lookup: &F) -> Option<Resolved> {
    let b = expr.as_bytes();
    let mut pos = 0;
    let r = parse_expr(b, &mut pos, lookup)?;
    (pos == b.len()).then_some(r)
}

fn parse_expr<F: Fn(&str) -> Option<Resolved>>(
    b: &[u8],
    pos: &mut usize,
    lookup: &F,
) -> Option<Resolved> {
    // A leading `/` divides one: `/s` is `1/s`.
    let mut r = if b.get(*pos) == Some(&b'/') {
        Resolved::scalar(Factor::one())
    } else {
        parse_term(b, pos, lookup)?
    };
    while *pos < b.len() {
        match b[*pos] {
            b'.' => {
                *pos += 1;
                r = r.mul(parse_term(b, pos, lookup)?);
            }
            b'/' => {
                *pos += 1;
                r = r.div(parse_term(b, pos, lookup)?);
            }
            _ => break,
        }
    }
    Some(r)
}

fn parse_term<F: Fn(&str) -> Option<Resolved>>(
    b: &[u8],
    pos: &mut usize,
    lookup: &F,
) -> Option<Resolved> {
    // An annotation alone (`{rbc}`) is the number one.
    if b.get(*pos) == Some(&b'{') {
        skip_annotation(b, pos)?;
        return Some(Resolved::scalar(Factor::one()));
    }
    let factor = parse_factor(b, pos, lookup)?;
    let exp = parse_exp(b, pos);
    // An annotation after a factor (`mol{rbc}`, `m2{x}`) leaves its value.
    if b.get(*pos) == Some(&b'{') {
        skip_annotation(b, pos)?;
    }
    if exp == 1 {
        Some(factor)
    } else {
        Some(factor.pow(exp))
    }
}

/// Step over `{...}`; `None` if it is not closed.
fn skip_annotation(b: &[u8], pos: &mut usize) -> Option<()> {
    let close = b[*pos..].iter().position(|&c| c == b'}')?;
    *pos += close + 1;
    Some(())
}

fn parse_factor<F: Fn(&str) -> Option<Resolved>>(
    b: &[u8],
    pos: &mut usize,
    lookup: &F,
) -> Option<Resolved> {
    if *pos >= b.len() {
        return None;
    }
    if b[*pos..].starts_with(b"10") && matches!(b.get(*pos + 2), Some(b'*') | Some(b'^')) {
        *pos += 3;
        let exp = match b.get(*pos) {
            Some(b'-') => {
                *pos += 1;
                -parse_int(b, pos)
            }
            Some(b'+') => {
                *pos += 1;
                parse_int(b, pos)
            }
            _ => parse_int(b, pos),
        };
        return Some(Resolved::scalar(Factor::ten_to(exp)));
    }
    if b[*pos].is_ascii_digit() {
        return Some(Resolved::scalar(parse_number(b, pos)));
    }
    if b[*pos] == b'(' {
        *pos += 1;
        let inner = parse_expr(b, pos, lookup)?;
        if b.get(*pos) != Some(&b')') {
            return None;
        }
        *pos += 1;
        return Some(inner);
    }
    let sym = read_symbol(b, pos);
    if sym.is_empty() {
        return None;
    }
    lookup(&sym)
}

/// An atom with its prefix: everything up to an operator or an exponent,
/// with a bracketed part (`[in_i]`, `m[H2O]`, `B[10.nV]`) read whole.
fn read_symbol(b: &[u8], pos: &mut usize) -> String {
    let start = *pos;
    while *pos < b.len() {
        let ch = b[*pos];
        if ch == b'[' {
            while *pos < b.len() && b[*pos] != b']' {
                *pos += 1;
            }
            if *pos < b.len() {
                *pos += 1;
            }
            continue;
        }
        if matches!(ch, b'.' | b'/' | b'(' | b')' | b'{' | b'}') || ch.is_ascii_digit() {
            break;
        }
        if matches!(ch, b'-' | b'+') && b.get(*pos + 1).is_some_and(|c| c.is_ascii_digit()) {
            break;
        }
        *pos += 1;
    }
    String::from_utf8_lossy(&b[start..*pos]).into_owned()
}

/// An exponent, signed or not (`m2`, `s-1`, `m+2`); one when there is none.
fn parse_exp(b: &[u8], pos: &mut usize) -> i32 {
    let signed =
        |sign| b.get(*pos) == Some(sign) && b.get(*pos + 1).is_some_and(u8::is_ascii_digit);
    if signed(&b'-') {
        *pos += 1;
        return -parse_int(b, pos);
    }
    if signed(&b'+') {
        *pos += 1;
        return parse_int(b, pos);
    }
    if b.get(*pos).is_some_and(|c| c.is_ascii_digit()) {
        return parse_int(b, pos);
    }
    1
}

/// A number factor: UCUM's are whole numbers (`m/3937`, `4.[pi]`).
fn parse_number(b: &[u8], pos: &mut usize) -> Factor {
    let start = *pos;
    while *pos < b.len() && b[*pos].is_ascii_digit() {
        *pos += 1;
    }
    let digits = std::str::from_utf8(&b[start..*pos]).expect("ASCII digits");
    Factor::decimal(digits).expect("a whole number")
}

fn parse_int(b: &[u8], pos: &mut usize) -> i32 {
    let mut n = 0i32;
    while *pos < b.len() && b[*pos].is_ascii_digit() {
        n = n * 10 + (b[*pos] - b'0') as i32;
        *pos += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup(sym: &str) -> Option<Resolved> {
        let dim: DimVec = [(sym.to_string(), 1)].into();
        matches!(sym, "m" | "s" | "mol" | "L" | "[pi]").then(|| Resolved {
            dim,
            multiplier: Factor::one(),
            kind: None,
        })
    }

    fn dec(s: &str) -> Factor {
        Factor::decimal(s).unwrap()
    }

    #[test]
    fn number_factors() {
        let r = eval_expr("m/3937", &lookup).unwrap();
        assert_eq!(r.dim, [("m".to_string(), 1)].into());
        assert_eq!(r.multiplier, Factor::one().div(&dec("3937")));
        assert_eq!(r.multiplier.expr(), "1 / 3937");
        let r = eval_expr("4.[pi].10*-7.m/s2", &lookup).unwrap();
        assert_eq!(r.multiplier, dec("4e-7"));
        assert_eq!(
            r.dim,
            [
                ("[pi]".to_string(), 1),
                ("m".to_string(), 1),
                ("s".to_string(), -2)
            ]
            .into()
        );
    }

    #[test]
    fn leading_slash() {
        let r = eval_expr("/s", &lookup).unwrap();
        assert_eq!(r.dim, [("s".to_string(), -1)].into());
        assert_eq!(r.multiplier, Factor::one());
    }

    #[test]
    fn annotations_are_skipped() {
        let r = eval_expr("mol{rbc}/L", &lookup).unwrap();
        assert_eq!(
            r.dim,
            [("L".to_string(), -1), ("mol".to_string(), 1)].into()
        );
        let r = eval_expr("m2{x}.s", &lookup).unwrap();
        assert_eq!(r.dim, [("m".to_string(), 2), ("s".to_string(), 1)].into());
        let r = eval_expr("{tot}/s", &lookup).unwrap();
        assert_eq!(r.dim, [("s".to_string(), -1)].into());
        assert!(eval_expr("m{x", &lookup).is_none());
    }

    #[test]
    fn plus_signed_exponent() {
        let r = eval_expr("m+2/s", &lookup).unwrap();
        assert_eq!(r.dim, [("m".to_string(), 2), ("s".to_string(), -1)].into());
        let r = eval_expr("10*+3.m", &lookup).unwrap();
        assert_eq!(r.multiplier.expr(), "1e3");
    }

    /// Decimals are read exactly, and π is kept as a power.
    #[test]
    fn factors_are_exact() {
        assert_eq!(dec("2.54").mul(&dec("1e-2")).expr(), "254e-4");
        assert_eq!(dec("0.1").mul(&dec("0.2")), dec("0.02"));
        let degree = Factor::pi().div(&dec("180"));
        assert_eq!(degree.expr(), "1 / 180 * core::f64::consts::PI");
        assert_eq!(dec("6.62607015e-34").expr(), "662607015e-42");
    }

    #[test]
    fn all_input_is_read() {
        assert!(eval_expr("m)s", &lookup).is_none());
        assert!(eval_expr("(m", &lookup).is_none());
        assert!(eval_expr("m s", &lookup).is_none());
        assert!(eval_expr("m/", &lookup).is_none());
        let r = eval_expr("(m/s)2", &lookup).unwrap();
        assert_eq!(r.dim, [("m".to_string(), 2), ("s".to_string(), -2)].into());
    }
}
