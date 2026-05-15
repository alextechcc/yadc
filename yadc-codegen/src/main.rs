mod codegen;
mod model;
mod ucum;

use std::path::PathBuf;
use std::process;

use model::UnitSystem;

const USAGE: &str = "\
Usage:
  yadc-codegen <INPUT.toml> [-o <output-dir>]

Arguments:
  <INPUT.toml>   Path to a .toml unit-system definition.
                 When the file contains a [ucum] table the families are
                 generated from a UCUM source; otherwise [[family]] sections
                 provide the quantity and unit data directly.

Options:
  -o, --output-dir <DIR>   Directory to write the generated crate.
                            Defaults to the crate_name from [system].
  -h, --help               Print this help message.

Examples:
  yadc-codegen si.toml
  yadc-codegen si.toml -o yadc-si
  yadc-codegen ucum.toml
  yadc-codegen ucum.toml -o yadc-ucum
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (input, output_dir) = parse_args(&args);

    let toml_str = std::fs::read_to_string(&input).unwrap_or_else(|e| {
        eprintln!("error: cannot read {}: {e}", input.display());
        process::exit(1);
    });

    let UnitSystem { system, kind, dim, dimensionless, family, ucum } =
        toml::from_str::<UnitSystem>(&toml_str).unwrap_or_else(|e| {
            eprintln!("error: failed to parse {}: {e}", input.display());
            process::exit(1);
        });

    let output_dir = output_dir.unwrap_or_else(|| PathBuf::from(&system.crate_name));
    eprintln!("writing to {} ...", output_dir.display());

    if let Some(ucum_section) = ucum {
        ucum::run(&ucum_section, system, kind, dim, dimensionless, &output_dir)
            .unwrap_or_else(|e| {
                eprintln!("error: {e}");
                process::exit(1);
            });
        eprintln!("done.");
    } else {
        let system = UnitSystem { system, kind, dim, dimensionless, family, ucum: None };

        codegen::write_crate(&system, &output_dir).unwrap_or_else(|e| {
            eprintln!("error: {e}");
            process::exit(1);
        });

        let nonempty: Vec<&str> = system.system.family_order.iter()
            .filter_map(|name| {
                system.family.iter()
                    .find(|f| &f.name == name)
                    .filter(|f| !f.quantity.is_empty())
                    .map(|_| name.as_str())
            })
            .collect();

        eprintln!("wrote {}/src/lib.rs", output_dir.display());
        eprintln!("wrote {}/Cargo.toml", output_dir.display());
        for fam in &nonempty {
            let qty_count = system.family.iter()
                .find(|f| f.name == *fam)
                .map_or(0, |f| f.quantity.len());
            eprintln!("wrote {}/src/{fam}.rs  ({qty_count} quantities)", output_dir.display());
        }
    }
}

fn parse_args(args: &[String]) -> (PathBuf, Option<PathBuf>) {
    let mut input: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                process::exit(0);
            }
            "-o" | "--output-dir" => {
                i += 1;
                output_dir = Some(PathBuf::from(args.get(i).unwrap_or_else(|| {
                    eprintln!("error: --output-dir requires a value");
                    process::exit(1);
                })));
            }
            arg if arg.starts_with("--output-dir=") => {
                output_dir = Some(PathBuf::from(&arg["--output-dir=".len()..]));
            }
            arg if !arg.starts_with('-') => {
                if input.is_some() {
                    eprintln!("error: unexpected argument {arg:?}");
                    process::exit(1);
                }
                input = Some(PathBuf::from(arg));
            }
            arg => {
                eprintln!("error: unknown flag {arg:?}");
                eprintln!("{USAGE}");
                process::exit(1);
            }
        }
        i += 1;
    }

    let input = input.unwrap_or_else(|| {
        eprintln!("{USAGE}");
        process::exit(1);
    });
    (input, output_dir)
}
