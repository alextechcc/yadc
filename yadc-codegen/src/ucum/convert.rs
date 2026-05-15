//! Convert resolved UCUM data into a `model::UnitSystem`.

use std::collections::{BTreeMap, HashMap, HashSet};

use super::parse::{UcumEssence, Unit};
use super::resolve::{DimVec, Resolved};
use crate::model::{
    DimConfig, DimensionlessDef, FamilyDef, KindConfig, QuantityDef, SystemConfig,
    UnitDef, UnitSystem,
};

// Metric prefix set applied to isMetric=yes units.
// (ucum_code, scale_factor, print_symbol, name)
const METRIC_PREFIXES: &[(&str, f64, &str, &str)] = &[
    ("T",  1e12,  "T",  "tera"),
    ("G",  1e9,   "G",  "giga"),
    ("M",  1e6,   "M",  "mega"),
    ("k",  1e3,   "k",  "kilo"),
    ("h",  1e2,   "h",  "hecto"),
    ("da", 1e1,   "da", "deca"),
    ("d",  1e-1,  "d",  "deci"),
    ("c",  1e-2,  "c",  "centi"),
    ("m",  1e-3,  "m",  "milli"),
    ("u",  1e-6,  "µ",  "micro"),
    ("n",  1e-9,  "n",  "nano"),
    ("p",  1e-12, "p",  "pico"),
    ("f",  1e-15, "f",  "femto"),
    ("a",  1e-18, "a",  "atto"),
];

fn class_to_family(class: &str) -> &'static str {
    match class {
        "si" | "" => "si",
        "dimless" => "dimless",
        "heat" => "heat",
        "cgs" => "cgs",
        "infotech" => "infotech",
        "iso1000" => "iso1000",
        "intcust" => "intcust",
        "us-lengths" | "us-volumes" | "us-masses" | "us-distances" => "us",
        "brit-length" | "brit-volumes" => "british",
        "avoirdupois" | "troy" | "apoth" => "us",
        "clinical" | "chemical" => "clinical",
        "const" => "constants",
        _ => "misc",
    }
}

fn pfx_pascal(name: &str) -> &'static str {
    match name {
        "yotta" => "Yotta", "zetta" => "Zetta", "exa"  => "Exa",
        "peta"  => "Peta",  "tera"  => "Tera",  "giga" => "Giga",
        "mega"  => "Mega",  "kilo"  => "Kilo",  "hecto"=> "Hecto",
        "deca"  => "Deca",  "deci"  => "Deci",  "centi"=> "Centi",
        "milli" => "Milli", "micro" => "Micro", "nano" => "Nano",
        "pico"  => "Pico",  "femto" => "Femto", "atto" => "Atto",
        "zepto" => "Zepto", "yocto" => "Yocto",
        _ => "Unknown",
    }
}

/// Convert a UCUM property or name string to a PascalCase Rust identifier.
pub fn to_pascal(s: &str) -> String {
    let s = s
        .replace('è', "e").replace('é', "e").replace('ê', "e").replace('ë', "e")
        .replace('à', "a").replace('â', "a").replace('ä', "a")
        .replace('ô', "o").replace('ö', "o").replace('û', "u").replace('ü', "u")
        .replace('î', "i").replace('ï', "i").replace('ç', "c")
        .replace('µ', "u").replace('Å', "A").replace('°', "Degree");

    s.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn fmt_mult(v: f64) -> String {
    if v == 1.0 { return "1.0".to_string(); }
    let s = format!("{v}");
    if s.contains('.') || s.contains('e') || s.contains('E') { s } else { format!("{s}.0") }
}

/// Convert DimVec (i32 exponents) to the BTreeMap<String, i64> used by QuantityDef.
fn to_qty_dims(dim: &DimVec) -> BTreeMap<String, i64> {
    dim.iter().map(|(k, v)| (k.clone(), *v as i64)).collect()
}

/// Hardcoded temperature quantities — Celsius/Fahrenheit are isSpecial in UCUM.
fn temperature_quantities() -> Vec<QuantityDef> {
    let k_dim: BTreeMap<String, i64> = [("K".to_string(), 1i64)].into();
    vec![
        QuantityDef {
            name: "ThermodynamicTemperature".to_string(),
            dims: k_dim.clone(),
            kind: Some("Affine".to_string()),
            dim_delta: Some("TemperatureInterval".to_string()),
            units: vec![
                UnitDef { variant: "Kelvin".into(),           multiplier: "1.0".into(),       symbol: "K".into(),  long: "kelvin".into(),             offset: None },
                UnitDef { variant: "DegreeCelsius".into(),    multiplier: "1.0".into(),       symbol: "°C".into(), long: "degree Celsius".into(),      offset: Some(273.15) },
                UnitDef { variant: "DegreeFahrenheit".into(), multiplier: "5.0 / 9.0".into(), symbol: "°F".into(), long: "degree Fahrenheit".into(),   offset: Some(459.67) },
                UnitDef { variant: "DegreeRankine".into(),    multiplier: "5.0 / 9.0".into(), symbol: "°R".into(), long: "degree Rankine".into(),      offset: None },
            ],
        },
        QuantityDef {
            name: "TemperatureInterval".to_string(),
            dims: k_dim,
            kind: None,
            dim_delta: None,
            units: vec![
                UnitDef { variant: "Kelvin".into(),      multiplier: "1.0".into(),   symbol: "K".into(),  long: "kelvin".into(),      offset: None },
                UnitDef { variant: "Millikelvin".into(), multiplier: "0.001".into(), symbol: "mK".into(), long: "millikelvin".into(), offset: None },
                UnitDef { variant: "Microkelvin".into(), multiplier: "1e-6".into(),  symbol: "µK".into(), long: "microkelvin".into(), offset: None },
            ],
        },
    ]
}

/// Key for grouping units into a single `QuantityDef`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GroupKey {
    dim: DimVec,
    kind: Option<String>,
    family: String,
}

pub fn to_unit_system(
    essence: &UcumEssence,
    resolved: &HashMap<String, Resolved>,
    system_cfg: SystemConfig,
    kind_cfg: KindConfig,
    dim_cfg: DimConfig,
    dimless: DimensionlessDef,
) -> UnitSystem {
    let mut groups: HashMap<GroupKey, (String, Vec<UnitDef>)> = HashMap::new();
    let mut dimless_units: Vec<UnitDef> = dimless.units;

    let all_codes: HashSet<String> = essence.units.iter()
        .map(|u| u.code.clone())
        .chain(essence.base_units.iter().map(|b| b.code.clone()))
        .collect();

    // Process UCUM base units (m, s, g, rad, K, C, cd)
    for base in &essence.base_units {
        let Some(r) = resolved.get(&base.code) else { continue; };
        if base.property.contains("temperature") { continue; }

        let key = GroupKey { dim: r.dim.clone(), kind: r.kind.clone(), family: "si".to_string() };
        let (_, units_vec) = groups.entry(key.clone())
            .or_insert_with(|| (base.property.clone(), Vec::new()));
        units_vec.push(UnitDef {
            variant: to_pascal(&base.name),
            multiplier: fmt_mult(r.multiplier),
            symbol: base.print_symbol.clone(),
            long: base.name.clone(),
            offset: None,
        });
        for &(pfx_code, pfx_val, pfx_sym, pfx_name) in METRIC_PREFIXES {
            let prefixed_code = format!("{pfx_code}{}", base.code);
            if all_codes.contains(&prefixed_code) { continue; }
            let mult = fmt_mult(r.multiplier * pfx_val);
            let entry = groups.get_mut(&key).unwrap();
            entry.1.push(UnitDef {
                variant: format!("{}{}", pfx_pascal(pfx_name), to_pascal(&base.name)),
                multiplier: mult,
                symbol: format!("{pfx_sym}{}", base.print_symbol),
                long: format!("{pfx_name}{}", base.name),
                offset: None,
            });
        }
    }

    for unit in &essence.units {
        if unit.is_arbitrary || unit.is_special { continue; }
        let Some(r) = resolved.get(&unit.code) else { continue; };
        if unit.property.contains("temperature") { continue; }

        let family = class_to_family(&unit.class);

        if r.dim.is_empty() && r.kind.is_none()
            && !matches!(unit.class.as_str(), "si" | "iso1000" | "intl")
        {
            if family == "dimless" {
                dimless_units.push(make_unit_def(unit, r.multiplier));
            }
            continue;
        }

        let key = GroupKey { dim: r.dim.clone(), kind: r.kind.clone(), family: family.to_string() };
        let (_, units_vec) = groups
            .entry(key.clone())
            .or_insert_with(|| (unit.property.clone(), Vec::new()));
        units_vec.push(make_unit_def(unit, r.multiplier));

        if unit.is_metric {
            for &(pfx_code, pfx_val, pfx_sym, pfx_name) in METRIC_PREFIXES {
                let prefixed_code = format!("{pfx_code}{}", unit.code);
                if all_codes.contains(&prefixed_code) { continue; }
                let mult = fmt_mult(r.multiplier * pfx_val);
                let entry = groups.get_mut(&key).unwrap();
                entry.1.push(UnitDef {
                    variant: format!("{}{}", pfx_pascal(pfx_name), to_pascal(&unit.name)),
                    multiplier: mult,
                    symbol: format!("{pfx_sym}{}", unit.print_symbol),
                    long: format!("{pfx_name}{}", unit.name),
                    offset: None,
                });
            }
        }
    }

    let mut family_qtys: HashMap<String, HashMap<String, QuantityDef>> = HashMap::new();

    for (GroupKey { dim, kind, family }, (property, units_vec)) in groups {
        if units_vec.is_empty() { continue; }
        let qty_name = to_pascal(&property);
        if qty_name.is_empty() { continue; }

        let fam_map = family_qtys.entry(family).or_default();
        let entry = fam_map.entry(qty_name.clone()).or_insert_with(|| QuantityDef {
            name: qty_name,
            dims: to_qty_dims(&dim),
            kind,
            dim_delta: None,
            units: Vec::new(),
        });
        let existing: HashSet<String> = entry.units.iter().map(|u| u.variant.clone()).collect();
        for u in units_vec {
            if !existing.contains(&u.variant) {
                entry.units.push(u);
            }
        }
    }

    // Insert hardcoded temperature quantities into "heat" family
    let heat_map = family_qtys.entry("heat".to_string()).or_default();
    for qty in temperature_quantities() {
        heat_map.insert(qty.name.clone(), qty);
    }

    let mut families: Vec<FamilyDef> = Vec::new();
    for fam_name in &system_cfg.family_order {
        if let Some(qty_map) = family_qtys.remove(fam_name.as_str()) {
            let mut qtys: Vec<QuantityDef> = qty_map.into_values().collect();
            qtys.sort_by(|a, b| a.name.cmp(&b.name));
            if !qtys.is_empty() {
                families.push(FamilyDef { name: fam_name.clone(), quantity: qtys });
            }
        }
    }
    // Remaining families not in the ordered list
    let mut extras: Vec<_> = family_qtys.into_iter().collect();
    extras.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, qty_map) in extras {
        let mut qtys: Vec<QuantityDef> = qty_map.into_values().collect();
        qtys.sort_by(|a, b| a.name.cmp(&b.name));
        if !qtys.is_empty() {
            families.push(FamilyDef { name, quantity: qtys });
        }
    }

    UnitSystem {
        system: system_cfg,
        kind: kind_cfg,
        dim: dim_cfg,
        dimensionless: DimensionlessDef { units: dimless_units },
        family: families,
        ucum: None,
    }
}

fn make_unit_def(unit: &Unit, multiplier: f64) -> UnitDef {
    UnitDef {
        variant: to_pascal(&unit.name),
        multiplier: fmt_mult(multiplier),
        symbol: unit.print_symbol.clone(),
        long: unit.name.clone(),
        offset: None,
    }
}
