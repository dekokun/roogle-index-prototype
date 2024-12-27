use std::fs::File;
use std::io::{BufReader, Error as IoError};
use std::path::PathBuf;

use clap::Parser;
use serde_json::Error as SerdeError;

mod rustdoc_json;
mod signature_builder;

use rustdoc_json::{RustDocJson, item_to_signature_string};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to rustdoc JSON file (e.g., target/doc/crate_name/crate_name.json)
    #[arg(value_name = "RUSTDOC_JSON_PATH")]
    json_path: PathBuf,
}

fn main() -> Result<(), IoError> {
    let args = Args::parse();

    let file = File::open(&args.json_path).map_err(|e| {
        eprintln!("Failed to open file '{}': {}", args.json_path.display(), e);
        e
    })?;
    let reader = BufReader::new(file);
    let doc: RustDocJson = serde_json::from_reader(reader)
        .map_err(|e: SerdeError| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;

    for item in doc.index.values() {
        if let Some(sig_str) = item_to_signature_string(item) {
            println!("{}", sig_str);
        }
    }

    Ok(())
}
