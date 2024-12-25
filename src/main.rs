use std::fs::File;
use std::io::{BufReader, Error as IoError};
use serde_json::Error as SerdeError;

// 他ファイルのモジュールを宣言
mod rustdoc_json;
mod signature_builder;

// それぞれ名前をわかりやすく使うためにuseする
use rustdoc_json::{RustDocJson, item_to_signature_string};
use signature_builder::function_sig_to_string;

fn main() -> Result<(), IoError> {
    // ----------------------------------------
    // 1. Rustdoc JSONファイルの読み込み
    // ----------------------------------------
    // 例えば cargo doc で生成された JSON を読み込む
    // 環境によってパスは変わるかもしれません
    let json_path = "target/doc/roogle_index_prototype.json";
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);

    // デシリアライズ
    let doc: RustDocJson = serde_json::from_reader(reader)
        .map_err(|e: SerdeError| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;

    // ----------------------------------------
    // 2. 各アイテムをチェックして、functionのみシグネチャを表示
    // ----------------------------------------
    for item in doc.index.values() {
        // rustdoc_json.rs に定義した関数で、functionのシグネチャを取る
        if let Some(function_sig) = item_to_signature_string(item) {
            println!("{}", function_sig);
        }
    }

    // ----------------------------------------
    // 3. もし個別にロジックを使いたい場合の例
    // ----------------------------------------
    // 例えば "fn ..." の文字列をさらに加工したいなら、
    // signature_builder の関数を直接呼び出すこともできる。
    // (今回は省略)

    Ok(())
}
