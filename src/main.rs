use std::fs::File;
use std::io::{BufReader, Error as IoError};

use serde_json::Error as SerdeError;

// 同じディレクトリにある別ファイル(mod)
mod rustdoc_json;
mod signature_builder;

// 必要な型や関数をuse
use rustdoc_json::{RustDocJson, item_to_signature_string};

fn main() -> Result<(), IoError> {
    // 1. Rustdoc JSONファイルのパス
    //    例: "target/doc/roogle_current/roogle_current.json"
    //    プロジェクト名に応じて変わるので要確認
    let args: Vec<String> = std::env::args().collect();
    let json_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <rustdoc_json_path>", args[0]);
            eprintln!("  rustdoc_json_path: path to rustdoc JSON file");
            eprintln!("  Example: target/doc/crate_name/crate_name.json");
            return Err(IoError::new(std::io::ErrorKind::InvalidInput, "Missing required argument"));
        }
    };

    // 2. JSONファイルを読み込む
    let file = File::open(json_path).map_err(|e| {
        eprintln!("Failed to open file '{}': {}", json_path, e);
        e
    })?;
    let reader = BufReader::new(file);
    let doc: RustDocJson = serde_json::from_reader(reader)
        .map_err(|e: SerdeError| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;

    // 3. index 内のItemを順番に見て、functionだけシグネチャ文字列化
    for item in doc.index.values() {
        if let Some(sig_str) = item_to_signature_string(item) {
            println!("{}", sig_str);
        }
    }

    Ok(())
}
