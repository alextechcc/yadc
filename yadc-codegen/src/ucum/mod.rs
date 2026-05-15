//! UCUM mode: fetch ucum-essence.xml and generate a yadc-compatible crate.

mod convert;
mod parse;
mod resolve;

use std::path::Path;

use crate::model::{
    DimConfig, DimensionlessDef, KindConfig, SystemConfig, UcumSection, UnitSystem,
};

const GITHUB_RELEASES_API: &str =
    "https://api.github.com/repos/ucum-org/ucum/releases/latest";
const UCUM_RAW_BASE: &str =
    "https://raw.githubusercontent.com/ucum-org/ucum";

/// Fetch (or read locally), parse, resolve, and build a `UnitSystem` from UCUM data.
///
/// The caller supplies the four config sections from the TOML (`[system]`, `[kind]`,
/// `[dim]`, `[dimensionless]`); this function generates the family data from UCUM and
/// returns a complete `UnitSystem` ready for `codegen::write_crate`.
pub fn build(
    ucum: &UcumSection,
    system_cfg: SystemConfig,
    kind_cfg: KindConfig,
    dim_cfg: DimConfig,
    dimless: DimensionlessDef,
) -> Result<UnitSystem, Box<dyn std::error::Error>> {
    let xml = match ucum.source.as_str() {
        "latest" => fetch_latest()?,
        s if s.starts_with("http://") || s.starts_with("https://") => {
            eprintln!("fetching {s}");
            fetch_url(s)?
        }
        path => std::fs::read_to_string(path)?,
    };

    let essence = parse::parse(&xml)?;
    eprintln!(
        "parsed: {} prefixes, {} base units, {} units",
        essence.prefixes.len(),
        essence.base_units.len(),
        essence.units.len()
    );

    let resolved = resolve::resolve_all(&essence, &ucum.base);
    let unresolved = essence.units.iter()
        .filter(|u| !u.is_special && !u.is_arbitrary && !resolved.contains_key(&u.code))
        .count();
    eprintln!("resolved: {} units ({unresolved} unresolved/skipped)", resolved.len());

    Ok(convert::to_unit_system(&essence, &resolved, system_cfg, kind_cfg, dim_cfg, dimless))
}

/// Write a crate from UCUM data to `output_dir`.
pub fn run(
    ucum: &UcumSection,
    system_cfg: SystemConfig,
    kind_cfg: KindConfig,
    dim_cfg: DimConfig,
    dimless: DimensionlessDef,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let system = build(ucum, system_cfg, kind_cfg, dim_cfg, dimless)?;
    crate::codegen::write_crate(&system, output_dir)?;
    Ok(())
}

fn fetch_latest() -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("querying GitHub for latest ucum-org/ucum release...");
    let tag = fetch_latest_tag()?;
    eprintln!("latest release: {tag}");
    let url = format!("{UCUM_RAW_BASE}/{tag}/ucum-essence.xml");
    eprintln!("fetching {url}");
    fetch_url(&url)
}

fn fetch_latest_tag() -> Result<String, Box<dyn std::error::Error>> {
    let body = ureq::get(GITHUB_RELEASES_API)
        .set("User-Agent", "yadc-codegen/0.1")
        .call()?
        .into_string()?;
    extract_json_string(&body, "tag_name")
}

fn extract_json_string(json: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let needle = format!("\"{}\":\"", key);
    let start = json
        .find(&needle)
        .ok_or_else(|| format!("key {key:?} not found in GitHub API response"))?;
    let rest = &json[start + needle.len()..];
    let end = rest
        .find('"')
        .ok_or_else(|| format!("unterminated string value for key {key:?}"))?;
    Ok(rest[..end].to_string())
}

fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(ureq::get(url)
        .set("User-Agent", "yadc-codegen/0.1")
        .call()?
        .into_string()?)
}
