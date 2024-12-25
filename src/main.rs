use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error as IoError};
use std::path::PathBuf;

/// インデックスに格納する最小限の情報例
/// - 名前（関数名・型名 など）
/// - ドキュメント（docコメントの一部など）
#[derive(Debug, Serialize, Deserialize)]
pub struct RoogleItem {
    pub name: String,
    pub doc: String,
}

/// RoogleIndex は、RoogleItem のリストを保持する単純な構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct RoogleIndex {
    pub items: Vec<RoogleItem>,
}

impl RoogleIndex {
    /// 空のインデックスを作成する
    pub fn new() -> Self {
        RoogleIndex { items: vec![] }
    }

    /// インメモリの `items` を JSON ファイルに書き出す
    pub fn save_to_file(&self, path: &str) -> Result<(), IoError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        // serde_json で pretty-print して保存
        serde_json::to_writer_pretty(writer, &self).map_err(|e| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    /// JSON ファイルからインデックスを読み込む
    pub fn load_from_file(path: &str) -> Result<Self, IoError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: RoogleIndex = serde_json::from_reader(reader)
            .map_err(|e: SerdeError| IoError::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(index)
    }

    /// 新しいエントリを追加する（名前とドキュメント）
    pub fn add_item(&mut self, name: &str, doc: &str) {
        let item = RoogleItem {
            name: name.to_string(),
            doc: doc.to_string(),
        };
        self.items.push(item);
    }

    /// 簡易的な検索例 (名前に keyword が含まれる要素を返す)
    pub fn search_by_name(&self, keyword: &str) -> Vec<&RoogleItem> {
        self.items
            .iter()
            .filter(|item| item.name.contains(keyword))
            .collect()
    }
}

/// メイン関数
fn main() -> Result<(), IoError> {
    // 1. 新しいインデックスを作る
    let mut index = RoogleIndex::new();

    // 2. サンプルデータを追加してみる
    index.add_item(
        "foo",
        "Example function foo. This is a doc string for demonstration."
    );
    index.add_item(
        "bar",
        "Example function bar. Another doc string here."
    );
    index.add_item(
        "baz",
        "Struct baz with interesting traits. Demonstration doc."
    );

    // 3. ローカルファイルに保存
    let index_path = "roogle_index.json";
    index.save_to_file(index_path)?;
    println!("Saved index to {}", index_path);

    // 4. JSONファイルから再度読み込み
    let loaded_index = RoogleIndex::load_from_file(index_path)?;
    println!("Loaded index: {:#?}", loaded_index);

    // 5. 簡易検索を実行
    let keyword = "ba";
    let results = loaded_index.search_by_name(keyword);
    println!("Search results for '{}':", keyword);
    for item in results {
        println!("- {}: {}", item.name, item.doc);
    }

    Ok(())
}

