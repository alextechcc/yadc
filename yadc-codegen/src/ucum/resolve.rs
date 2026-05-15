//! Evaluate UCUM unit expressions to dimension vectors + multipliers.

use std::collections::{BTreeMap, HashMap};
use super::parse::UcumEssence;
use crate::model::UcumBaseMap;

/// Dimension exponent map: field_name → exponent.  Zero exponents are elided,
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
        Resolved { dim: DimVec::new(), multiplier: v, kind: None }
    }

    pub fn mul(self, other: Self) -> Self {
        Resolved {
            dim: dim_add(&self.dim, &other.dim),
            multiplier: self.multiplier * other.multiplier,
            kind: kind_mul(self.kind.as_deref(), other.kind.as_deref()),
        }
    }

    pub fn div(self, other: Self) -> Self {
        Resolved {
            dim: dim_sub(&self.dim, &other.dim),
            multiplier: self.multiplier / other.multiplier,
            kind: kind_div(self.kind.as_deref(), other.kind.as_deref()),
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
    if n == 0 { return DimVec::new(); }
    a.iter().map(|(k, v)| (k.clone(), v * n)).collect()
}

// Kind algebra: Angle × Angle = SolidAngle, SolidAngle / Angle = Angle, else Linear.
fn kind_mul(a: Option<&str>, b: Option<&str>) -> Option<String> {
    match (a, b) {
        (Some("Angle"), Some("Angle")) => Some("SolidAngle".into()),
        (None, k) | (k, None) => k.map(str::to_string),
        _ => None,
    }
}

fn kind_div(a: Option<&str>, b: Option<&str>) -> Option<String> {
    match (a, b) {
        (Some("SolidAngle"), Some("Angle")) => Some("Angle".into()),
        (k, None) => k.map(str::to_string),
        _ => None,
    }
}

fn kind_pow(k: Option<&str>, n: i32) -> Option<String> {
    match (k, n) {
        (Some("Angle"), 2) => Some("SolidAngle".into()),
        (Some(s), 1) => Some(s.into()),
        _ => None,
    }
}

/// Resolve all UCUM units to (dimension vector, multiplier, kind).
///
/// Seeding strategy:
/// 1. Every UCUM base unit is self-seeded: code → its own dim field, ×1.0, no kind.
/// 2. Entries in `base_maps` override individual codes.
///
/// With an empty `base_maps` slice the output uses UCUM-native dimensions
/// (`g`, `C`, `rad`, … as real dim fields).  Supply overrides to remap to ISQ.
pub fn resolve_all(
    essence: &UcumEssence,
    base_maps: &[UcumBaseMap],
) -> HashMap<String, Resolved> {
    let mut db: HashMap<String, Resolved> = HashMap::new();

    // Step 1: self-seed every UCUM base unit
    for base in &essence.base_units {
        let mut dim = DimVec::new();
        dim.insert(base.code.clone(), 1);
        db.insert(base.code.clone(), Resolved { dim, multiplier: 1.0, kind: None });
    }

    // Step 2: apply user overrides from [[ucum.base]]
    for bm in base_maps {
        db.insert(bm.code.clone(), Resolved {
            dim: bm.dims.iter().map(|(k, v)| (k.clone(), *v as i32)).collect(),
            multiplier: bm.multiplier,
            kind: bm.kind.clone(),
        });
    }

    // UCUM annotation-only term
    db.insert("1".into(), Resolved::scalar(1.0));

    // Build prefix lookup (longest code first for greedy match)
    let mut prefixes: Vec<(String, f64)> = essence.prefixes.iter()
        .map(|p| (p.code.clone(), p.value))
        .collect();
    prefixes.sort_by_key(|(code, _)| -(code.len() as i64));

    let non_special: Vec<_> = essence.units.iter()
        .filter(|u| !u.is_special && !u.is_arbitrary && !u.value_unit.is_empty())
        .collect();

    // Iteratively resolve derived units until stable
    for _ in 0..20 {
        let prev = db.len();
        for unit in &non_special {
            if db.contains_key(&unit.code) { continue; }
            let lookup = |sym: &str| lookup_sym(sym, &db, &prefixes);
            if let Some(r) = eval_expr(&unit.value_unit, &lookup) {
                db.insert(unit.code.clone(), Resolved {
                    multiplier: r.multiplier * unit.value_number,
                    ..r
                });
            }
        }
        if db.len() == prev { break; }
    }

    db
}

fn lookup_sym(
    sym: &str,
    db: &HashMap<String, Resolved>,
    prefixes: &[(String, f64)],
) -> Option<Resolved> {
    if sym == "1" { return Some(Resolved::scalar(1.0)); }
    if let Some(r) = db.get(sym) { return Some(r.clone()); }
    for (code, val) in prefixes {
        if sym.len() > code.len() && sym.starts_with(code.as_str()) {
            let atom = &sym[code.len()..];
            if let Some(r) = db.get(atom) {
                return Some(Resolved { multiplier: r.multiplier * val, ..r.clone() });
            }
        }
    }
    None
}

fn eval_expr<F: Fn(&str) -> Option<Resolved>>(expr: &str, lookup: &F) -> Option<Resolved> {
    let b = expr.as_bytes();
    let mut pos = 0;
    parse_expr(b, &mut pos, lookup)
}

fn parse_expr<F: Fn(&str) -> Option<Resolved>>(
    b: &[u8], pos: &mut usize, lookup: &F,
) -> Option<Resolved> {
    let mut r = parse_term(b, pos, lookup)?;
    while *pos < b.len() {
        match b[*pos] {
            b'.' => { *pos += 1; r = r.mul(parse_term(b, pos, lookup)?); }
            b'/' => { *pos += 1; r = r.div(parse_term(b, pos, lookup)?); }
            _ => break,
        }
    }
    Some(r)
}

fn parse_term<F: Fn(&str) -> Option<Resolved>>(
    b: &[u8], pos: &mut usize, lookup: &F,
) -> Option<Resolved> {
    if b.get(*pos) == Some(&b'{') {
        *pos += 1;
        while *pos < b.len() && b[*pos] != b'}' { *pos += 1; }
        if *pos < b.len() { *pos += 1; }
        return Some(Resolved::scalar(1.0));
    }
    let factor = parse_factor(b, pos, lookup)?;
    let exp = parse_exp(b, pos);
    if exp == 1 { Some(factor) } else { Some(factor.pow(exp)) }
}

fn parse_factor<F: Fn(&str) -> Option<Resolved>>(
    b: &[u8], pos: &mut usize, lookup: &F,
) -> Option<Resolved> {
    if *pos >= b.len() { return None; }
    if b[*pos..].starts_with(b"10") {
        if matches!(b.get(*pos + 2), Some(b'*') | Some(b'^')) {
            *pos += 3;
            let exp = parse_int(b, pos);
            return Some(Resolved::scalar(10f64.powi(exp)));
        }
    }
    if b[*pos] == b'(' {
        *pos += 1;
        let inner = parse_expr(b, pos, lookup)?;
        if b.get(*pos) == Some(&b')') { *pos += 1; }
        return Some(inner);
    }
    let sym = read_symbol(b, pos);
    if sym.is_empty() { return None; }
    lookup(&sym)
}

fn read_symbol(b: &[u8], pos: &mut usize) -> String {
    let start = *pos;
    if b.get(*pos) == Some(&b'[') {
        *pos += 1;
        while *pos < b.len() && b[*pos] != b']' { *pos += 1; }
        if *pos < b.len() { *pos += 1; }
        return String::from_utf8_lossy(&b[start..*pos]).into_owned();
    }
    while *pos < b.len() {
        let ch = b[*pos];
        if matches!(ch, b'.' | b'/' | b'(' | b')' | b'{' | b'}') { break; }
        if ch == b'-' && b.get(*pos + 1).is_some_and(|c| c.is_ascii_digit()) { break; }
        if ch.is_ascii_digit() { break; }
        *pos += 1;
    }
    String::from_utf8_lossy(&b[start..*pos]).into_owned()
}

fn parse_exp(b: &[u8], pos: &mut usize) -> i32 {
    if b.get(*pos) == Some(&b'-') && b.get(*pos + 1).is_some_and(|c| c.is_ascii_digit()) {
        *pos += 1;
        return -parse_int(b, pos);
    }
    if b.get(*pos).is_some_and(|c| c.is_ascii_digit()) {
        return parse_int(b, pos);
    }
    1
}

fn parse_int(b: &[u8], pos: &mut usize) -> i32 {
    let mut n = 0i32;
    while *pos < b.len() && b[*pos].is_ascii_digit() {
        n = n * 10 + (b[*pos] - b'0') as i32;
        *pos += 1;
    }
    n
}
