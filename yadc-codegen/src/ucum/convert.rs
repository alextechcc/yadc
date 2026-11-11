//! Convert resolved UCUM data into a `model::UnitSystem`.

use std::collections::{BTreeMap, HashMap};

use super::parse::{UcumEssence, Unit};
use super::resolve::{DimVec, Resolved};
use crate::model::{
    BaseConst, DimConfig, DimensionlessDef, ExtraConst, FamilyDef, KindConfig, QuantityDef,
    SystemConfig, UnitDef, UnitSystem,
};

fn class_to_family(class: &str) -> &'static str {
    match class {
        "si" | "" => "si",
        "dimless" => "dimless",
        "heat" => "heat",
        "cgs" => "cgs",
        "infotech" => "infotech",
        "iso1000" => "iso1000",
        "intcust" => "intcust",
        "us-lengths" | "us-volumes" | "us-masses" => "us",
        "brit-length" | "brit-volumes" => "british",
        "avoirdupois" | "troy" | "apoth" => "us",
        "clinical" | "chemical" => "clinical",
        "const" => "constants",
        _ => "misc",
    }
}

/// Convert a UCUM property or name string to a PascalCase Rust identifier.
pub fn to_pascal(s: &str) -> String {
    let s = s
        .replace(['è', 'é', 'ê', 'ë'], "e")
        .replace(['à', 'â', 'ä'], "a")
        .replace(['ô', 'ö'], "o")
        .replace(['û', 'ü', 'µ'], "u")
        .replace(['î', 'ï'], "i")
        .replace('ç', "c")
        .replace('Å', "A")
        .replace('°', "Degree");

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
    if v == 1.0 {
        return "1.0".to_string();
    }
    let s = format!("{v}");
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

/// Convert DimVec (i32 exponents) to the BTreeMap<String, i64> used by QuantityDef.
fn to_qty_dims(dim: &DimVec) -> BTreeMap<String, i64> {
    dim.iter().map(|(k, v)| (k.clone(), *v as i64)).collect()
}

/// A plain multiple: `m` times the quantity's coherent unit.
fn multiple(variant: &str, m: &str, symbol: &str, long: &str) -> UnitDef {
    UnitDef {
        variant: variant.into(),
        multiplier: Some(m.into()),
        symbol: symbol.into(),
        long: long.into(),
        constant: None,
        by: None,
        via: None,
        non_const: false,
    }
}

/// A scale: `step`s counted from `zero`, `Scale(zero, step)`.
fn scale(variant: &str, symbol: &str, long: &str, step: &str, zero: &str) -> UnitDef {
    UnitDef {
        variant: variant.into(),
        multiplier: None,
        symbol: symbol.into(),
        long: long.into(),
        constant: None,
        by: Some(format!("Scale({zero}, {step})")),
        via: None,
        non_const: false,
    }
}

/// Hardcoded temperature quantities - Celsius/Fahrenheit are isSpecial in
/// UCUM. An absolute temperature has the kind of the config's `ABSOLUTE_ZERO`
/// base constant - a label with rules, or the affine kind - and is read on
/// scales (a step and a zero); a temperature step is a multiple of the
/// kelvin. The zeros are defined from the `ABSOLUTE_ZERO` and `KELVIN` base
/// constants.
fn temperature_quantities(kind: &str) -> Vec<QuantityDef> {
    let k_dim: BTreeMap<String, i64> = [("K".to_string(), 1i64)].into();
    let mut fahrenheit = multiple("DegreeFahrenheit", "5.0 / 9.0", "°F", "degrees Fahrenheit");
    fahrenheit.constant = Some("DEGREE_FAHRENHEIT".into());
    vec![
        QuantityDef {
            name: "ThermodynamicTemperature".to_string(),
            dims: k_dim.clone(),
            kind: Some(kind.to_string()),
            consts: vec![
                ExtraConst {
                    name: "ICE_POINT".into(),
                    ty: None,
                    value: "ABSOLUTE_ZERO + 273.15 * KELVIN".into(),
                    doc: "0 °C, the ice point of water: 273.15 K.".into(),
                },
                ExtraConst {
                    name: "FAHRENHEIT_ZERO".into(),
                    ty: None,
                    value: "ICE_POINT - 32.0 * DEGREE_FAHRENHEIT".into(),
                    doc: "0 °F: 32 °F below the ice point.".into(),
                },
            ],
            units: vec![
                scale("Kelvin", "K", "kelvin", "KELVIN", "ABSOLUTE_ZERO"),
                scale(
                    "DegreeCelsius",
                    "°C",
                    "degrees Celsius",
                    "KELVIN",
                    "ICE_POINT",
                ),
                scale(
                    "DegreeFahrenheit",
                    "°F",
                    "degrees Fahrenheit",
                    "DEGREE_FAHRENHEIT",
                    "FAHRENHEIT_ZERO",
                ),
                scale(
                    "DegreeRankine",
                    "°R",
                    "degrees Rankine",
                    "DEGREE_FAHRENHEIT",
                    "ABSOLUTE_ZERO",
                ),
            ],
        },
        QuantityDef {
            name: "TemperatureInterval".to_string(),
            dims: k_dim,
            kind: None,
            consts: Vec::new(),
            units: vec![
                multiple("Kelvin", "1.0", "K", "kelvin"),
                multiple("Millikelvin", "0.001", "mK", "millikelvin"),
                multiple("Microkelvin", "1e-6", "µK", "microkelvin"),
                fahrenheit,
            ],
        },
    ]
}

/// Key for grouping units into a single `QuantityDef`: one quantity per
/// family, UCUM property, dimension and kind.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct GroupKey {
    family: String,
    name: String,
    dim: DimVec,
    kind: Option<String>,
}

/// Add `unit` to its group, unless the group already has a unit of that name.
fn push_unit(groups: &mut BTreeMap<GroupKey, Vec<UnitDef>>, key: &GroupKey, unit: UnitDef) {
    let units = groups.entry(key.clone()).or_default();
    if !units.iter().any(|u| u.variant == unit.variant) {
        units.push(unit);
    }
}

/// A name suffix that tells apart quantities of one property by their
/// dimension and kind, its axes in `[dim]` order: `KgPerM3`, `PerS`, `One`
/// for dimension one.
fn dim_suffix(dim: &DimVec, kind: Option<&str>, fields: &[String]) -> String {
    let part = |positive: bool| -> String {
        fields
            .iter()
            .filter_map(|k| dim.get(k).map(|e| (k, e)))
            .filter(|&(_, &e)| (e > 0) == positive)
            .map(|(k, e)| match e.abs() {
                1 => to_pascal(k),
                n => format!("{}{n}", to_pascal(k)),
            })
            .collect()
    };
    let (num, den) = (part(true), part(false));
    let mut s = if num.is_empty() && den.is_empty() {
        "One".to_string()
    } else {
        num
    };
    if !den.is_empty() {
        s += "Per";
        s += &den;
    }
    if let Some(k) = kind {
        s += k;
    }
    s
}

pub fn to_unit_system(
    essence: &UcumEssence,
    resolved: &HashMap<String, Resolved>,
    system_cfg: SystemConfig,
    kind_cfg: KindConfig,
    dim_cfg: DimConfig,
    base: Vec<BaseConst>,
    dimless: DimensionlessDef,
) -> UnitSystem {
    let mut groups: BTreeMap<GroupKey, Vec<UnitDef>> = BTreeMap::new();
    let mut dimless_units: Vec<UnitDef> = dimless.units;

    // Process UCUM base units (m, s, g, rad, K, C, cd)
    for base in &essence.base_units {
        let Some(r) = resolved.get(&base.code) else {
            continue;
        };
        let name = to_pascal(&base.property);
        if base.property.contains("temperature") || name.is_empty() {
            continue;
        }

        let key = GroupKey {
            family: "si".to_string(),
            name,
            dim: r.dim.clone(),
            kind: r.kind.clone(),
        };
        push_unit(
            &mut groups,
            &key,
            UnitDef {
                variant: to_pascal(&base.name),
                multiplier: Some(fmt_mult(r.multiplier)),
                symbol: base.print_symbol.clone(),
                long: base.name.clone(),
                constant: None,
                by: None,
                via: None,
                non_const: false,
            },
        );
    }

    for unit in &essence.units {
        if unit.is_arbitrary || unit.is_special {
            continue;
        }
        let Some(r) = resolved.get(&unit.code) else {
            continue;
        };
        let name = to_pascal(&unit.property);
        if unit.property.contains("temperature") || name.is_empty() {
            continue;
        }

        let family = class_to_family(&unit.class);
        if family == "dimless" {
            assert!(
                r.dim.is_empty() && r.kind.is_none(),
                "UCUM unit `{}` is of class `dimless` but resolves to dimension {:?}, kind {:?}",
                unit.code,
                r.dim,
                r.kind,
            );
            dimless_units.push(make_unit_def(unit, r.multiplier));
            continue;
        }

        let key = GroupKey {
            family: family.to_string(),
            name,
            dim: r.dim.clone(),
            kind: r.kind.clone(),
        };
        push_unit(&mut groups, &key, make_unit_def(unit, r.multiplier));
    }

    // A property name shared by groups of different dimensions or kinds in
    // one family is told apart by a dimension suffix on each.
    let mut name_count: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    for key in groups.keys() {
        *name_count.entry((&key.family, &key.name)).or_default() += 1;
    }
    let mut family_qtys: BTreeMap<String, Vec<QuantityDef>> = BTreeMap::new();
    for (key, units) in &groups {
        let name = if name_count[&(key.family.as_str(), key.name.as_str())] > 1 {
            format!(
                "{}{}",
                key.name,
                dim_suffix(&key.dim, key.kind.as_deref(), &dim_cfg.fields)
            )
        } else {
            key.name.clone()
        };
        family_qtys
            .entry(key.family.clone())
            .or_default()
            .push(QuantityDef {
                name,
                dims: to_qty_dims(&key.dim),
                kind: key.kind.clone(),
                consts: Vec::new(),
                units: units.clone(),
            });
    }

    // Insert hardcoded temperature quantities into "heat" family
    let absolute = base
        .iter()
        .find(|b| b.name == "ABSOLUTE_ZERO")
        .and_then(|b| b.kind.clone())
        .expect(
            "UCUM's absolute temperatures are counted from an `ABSOLUTE_ZERO` base constant with \
             a kind of its own: add `[[base]] name = \"ABSOLUTE_ZERO\"` with `kind = \"..\"` \
             (a `[kind]` variant: a label with rules, or the affine kind)",
        );
    family_qtys
        .entry("heat".to_string())
        .or_default()
        .extend(temperature_quantities(&absolute));

    for (fam_name, qtys) in &mut family_qtys {
        assert!(
            system_cfg.family_order.contains(fam_name),
            "UCUM units fall in family `{fam_name}`, which [system].family_order does not list: \
             add it",
        );
        qtys.sort_by(|a, b| a.name.cmp(&b.name));
        for pair in qtys.windows(2) {
            assert!(
                pair[0].name != pair[1].name,
                "two quantities named `{}` in family `{fam_name}`",
                pair[0].name,
            );
        }
    }
    let families: Vec<FamilyDef> = system_cfg
        .family_order
        .iter()
        .filter_map(|name| {
            family_qtys.remove(name).map(|quantity| FamilyDef {
                name: name.clone(),
                quantity,
            })
        })
        .collect();

    UnitSystem {
        system: system_cfg,
        kind: kind_cfg,
        dim: dim_cfg,
        base,
        dimensionless: DimensionlessDef {
            units: dimless_units,
        },
        family: families,
        ucum: None,
    }
}

fn make_unit_def(unit: &Unit, multiplier: f64) -> UnitDef {
    UnitDef {
        variant: to_pascal(&unit.name),
        multiplier: Some(fmt_mult(multiplier)),
        symbol: unit.print_symbol.clone(),
        long: unit.name.clone(),
        constant: None,
        by: None,
        via: None,
        non_const: false,
    }
}
