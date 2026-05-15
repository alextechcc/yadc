//! Parse ucum-essence.xml into plain Rust structs.

use roxmltree::Document;

#[derive(Debug, Default)]
pub struct UcumEssence {
    pub prefixes: Vec<Prefix>,
    pub base_units: Vec<BaseUnit>,
    pub units: Vec<Unit>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Prefix {
    pub code: String,
    pub value: f64,
    pub name: String,
    pub print_symbol: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BaseUnit {
    pub code: String,
    pub name: String,
    pub print_symbol: String,
    pub property: String,
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub code: String,
    pub name: String,
    pub print_symbol: String,
    pub property: String,
    pub class: String,
    pub is_metric: bool,
    pub is_special: bool,
    pub is_arbitrary: bool,
    pub value_unit: String,
    pub value_number: f64,
}

pub fn parse(xml: &str) -> Result<UcumEssence, Box<dyn std::error::Error>> {
    let doc = Document::parse(xml)?;
    let root = doc.root_element();
    let mut essence = UcumEssence::default();

    for child in root.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "prefix" => {
                let code = child.attribute("Code").unwrap_or("").to_string();
                let mut value = 0.0f64;
                let mut name = String::new();
                let mut print_symbol = String::new();
                for elem in child.children().filter(|n| n.is_element()) {
                    match elem.tag_name().name() {
                        "name" => name = elem.text().unwrap_or("").to_string(),
                        "printSymbol" => print_symbol = elem.text().unwrap_or("").to_string(),
                        "value" => {
                            if let Some(v) = elem.attribute("value") {
                                value = v.parse().unwrap_or(0.0);
                            }
                        }
                        _ => {}
                    }
                }
                if !code.is_empty() {
                    essence.prefixes.push(Prefix { code, value, name, print_symbol });
                }
            }
            "base-unit" => {
                let code = child.attribute("Code").unwrap_or("").to_string();
                let mut name = String::new();
                let mut print_symbol = String::new();
                let mut property = String::new();
                for elem in child.children().filter(|n| n.is_element()) {
                    match elem.tag_name().name() {
                        "name" => name = elem.text().unwrap_or("").to_string(),
                        "printSymbol" => print_symbol = elem.text().unwrap_or("").to_string(),
                        "property" => property = elem.text().unwrap_or("").to_string(),
                        _ => {}
                    }
                }
                if !code.is_empty() {
                    essence.base_units.push(BaseUnit { code, name, print_symbol, property });
                }
            }
            "unit" => {
                let code = child.attribute("Code").unwrap_or("").to_string();
                let is_metric = child.attribute("isMetric").unwrap_or("no") == "yes";
                let is_special = child.attribute("isSpecial").unwrap_or("no") == "yes";
                let is_arbitrary = child.attribute("isArbitrary").unwrap_or("no") == "yes";
                let class = child.attribute("class").unwrap_or("").to_string();
                let mut name = String::new();
                let mut print_symbol = String::new();
                let mut property = String::new();
                let mut value_unit = String::new();
                let mut value_number = 1.0f64;
                for elem in child.children().filter(|n| n.is_element()) {
                    match elem.tag_name().name() {
                        "name" => name = elem.text().unwrap_or("").to_string(),
                        "printSymbol" => print_symbol = elem.text().unwrap_or("").to_string(),
                        "property" => property = elem.text().unwrap_or("").to_string(),
                        "value" => {
                            if let Some(u) = elem.attribute("Unit") {
                                value_unit = u.to_string();
                            }
                            if let Some(v) = elem.attribute("value") {
                                value_number = v.parse().unwrap_or(1.0);
                            }
                        }
                        _ => {}
                    }
                }
                if !code.is_empty() {
                    essence.units.push(Unit {
                        code, name, print_symbol, property, class,
                        is_metric, is_special, is_arbitrary,
                        value_unit, value_number,
                    });
                }
            }
            _ => {}
        }
    }

    Ok(essence)
}
