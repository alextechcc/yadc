//! Evaluate UCUM unit expressions to dimension vectors + multipliers.

use super::parse::UcumEssence;
use crate::model::UcumBaseMap;
use std::collections::{BTreeMap, HashMap};

/// Dimension exponent map: field_name -> exponent.  Zero exponents are elided,
/// so an empty map means "dimensionless".
pub type DimVec = BTreeMap<String, i32>;

#[derive(Debug, Clone)]
pub struct Resolved {
    pub dim: DimVec,
    pub multiplier: f64,
    /// Kind variant name, or `None` for the default linear kind.
    pub kind: Option<String>,
}

impl Resolved {
    fn scalar(v: f64) -> Self {
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
            multiplier: self.multiplier * other.multiplier,
            kind: kind_mul(&self, &other),
        }
    }

    pub fn div(self, other: Self) -> Self {
        Resolved {
            dim: dim_sub(&self.dim, &other.dim),
            multiplier: self.multiplier / other.multiplier,
            kind: kind_div(&self, &other),
        }
    }

    pub fn pow(self, n: i32) -> Self {
        Resolved {
            dim: dim_scale(&self.dim, n),
            multiplier: self.multiplier.powi(n),
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

/// Resolve all UCUM units to (dimension vector, multiplier, kind).
///
/// Seeding strategy:
/// 1. Every UCUM base unit is self-seeded: code -> its own dim field, x1.0, no kind.
/// 2. Entries in `base_maps` override individual codes.
///
/// With an empty `base_maps` slice the output uses UCUM-native dimensions
/// (`g`, `C`, `rad`, ... as real dim fields).  Supply overrides to remap to ISQ.
pub fn resolve_all(essence: &UcumEssence, base_maps: &[UcumBaseMap]) -> HashMap<String, Resolved> {
    let mut db: HashMap<String, Resolved> = HashMap::new();

    // Step 1: self-seed every UCUM base unit
    for base in &essence.base_units {
        let mut dim = DimVec::new();
        dim.insert(base.code.clone(), 1);
        db.insert(
            base.code.clone(),
            Resolved {
                dim,
                multiplier: 1.0,
                kind: None,
            },
        );
    }

    // Step 2: apply user overrides from [[ucum.base]]
    for bm in base_maps {
        db.insert(
            bm.code.clone(),
            Resolved {
                dim: bm
                    .dims
                    .iter()
                    .map(|(k, v)| (k.clone(), *v as i32))
                    .collect(),
                multiplier: bm.multiplier,
                kind: bm.kind.clone(),
            },
        );
    }

    // Build prefix lookup (longest code first for greedy match)
    let mut prefixes: Vec<(String, f64)> = essence
        .prefixes
        .iter()
        .map(|p| (p.code.clone(), p.value))
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
            if let Some(r) = eval_expr(&unit.value_unit, &lookup) {
                db.insert(
                    unit.code.clone(),
                    Resolved {
                        multiplier: r.multiplier * unit.value_number,
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
    prefixes: &[(String, f64)],
) -> Option<Resolved> {
    if let Some(r) = db.get(sym) {
        return Some(r.clone());
    }
    for (code, val) in prefixes {
        if sym.len() > code.len() && sym.starts_with(code.as_str()) {
            let atom = &sym[code.len()..];
            if let Some(r) = db.get(atom) {
                return Some(Resolved {
                    multiplier: r.multiplier * val,
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
        Resolved::scalar(1.0)
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
        return Some(Resolved::scalar(1.0));
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
        return Some(Resolved::scalar(10f64.powi(exp)));
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
fn parse_number(b: &[u8], pos: &mut usize) -> f64 {
    let mut n = 0.0;
    while *pos < b.len() && b[*pos].is_ascii_digit() {
        n = n * 10.0 + f64::from(b[*pos] - b'0');
        *pos += 1;
    }
    n
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
            multiplier: 1.0,
            kind: None,
        })
    }

    #[test]
    fn number_factors() {
        let r = eval_expr("m/3937", &lookup).unwrap();
        assert_eq!(r.dim, [("m".to_string(), 1)].into());
        assert_eq!(r.multiplier, 1.0 / 3937.0);
        let r = eval_expr("4.[pi].10*-7.m/s2", &lookup).unwrap();
        assert_eq!(r.multiplier, 4.0 * 1e-7);
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
        assert_eq!(r.multiplier, 1.0);
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
        assert_eq!(r.multiplier, 1000.0);
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
