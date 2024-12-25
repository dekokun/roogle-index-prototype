use serde::Deserialize;
use std::collections::HashMap;

use crate::signature_builder::{function_sig_to_string, FunctionSig};

/// Rustdoc JSON のトップレベル例
#[derive(Debug, Deserialize)]
pub struct RustDocJson {
    #[serde(rename = "index")]
    pub index: HashMap<String, Item>,
}

/// Rustdoc JSON における1つのアイテム (関数、構造体 等々)
#[derive(Debug, Deserialize)]
pub struct Item {
    /// アイテム名 (関数名など)
    pub name: Option<String>,
    /// アイテムの種類: function / struct / enum / ...
    #[serde(rename = "kind")]
    pub kind: String,
    /// 詳細情報 (function の場合のみ `inner.function` が入っている)
    #[serde(rename = "inner")]
    pub inner: ItemInner,
}

/// ItemInner は、function ならば Function 構造体
/// (struct, enum, trait など 他のバリエーションは省略)
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", content = "function")]
pub enum ItemInner {
    #[serde(rename = "function")]
    Function(Function),
}

/// 関数の詳細 (シグネチャなど)
#[derive(Debug, Deserialize)]
pub struct Function {
    pub sig: FunctionSig,
    // generics, header, has_body などもありうる
}

/// ----------------------------------------
/// 関数シグネチャを文字列化する簡易ヘルパー
/// ----------------------------------------
pub fn item_to_signature_string(item: &Item) -> Option<String> {
    if item.kind != "function" {
        return None;
    }

    let name = item.name.as_deref().unwrap_or("unknown");
    if let ItemInner::Function(func) = &item.inner {
        // signature_builder.rs に定義したロジックを呼び出して
        // Rustっぽいシグネチャ文字列を作る
        let sig_str = function_sig_to_string(name, &func.sig);
        Some(sig_str)
    } else {
        None
    }
}
