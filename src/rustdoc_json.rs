use serde::Deserialize;
use std::collections::HashMap;

use crate::signature_builder::{function_sig_to_string, FunctionSig};

/// ----------------------------------------
/// Rustdoc JSON のトップレベル
/// ----------------------------------------
#[derive(Debug, Deserialize)]
pub struct RustDocJson {
    /// "index" フィールド: ID文字列 -> Item
    pub index: HashMap<String, Item>,
}

/// ----------------------------------------
/// Rustdoc JSON 内の1つのアイテム
/// (関数, 構造体, enum, など)
/// ----------------------------------------
#[derive(Debug, Deserialize)]
pub struct Item {
    /// アイテム名 (function の場合は関数名)
    pub name: Option<String>,

    /// ドキュメントコメント
    #[serde(default)]
    pub docs: Option<String>,

    /// 詳細情報は "inner" フィールドに入る
    pub inner: ItemInner,
}

/// ----------------------------------------
/// ItemInner: functionキーがあれば関数
/// (他にも struct, enum, trait, impl, ... がありうる)
/// ----------------------------------------
#[derive(Debug, Deserialize)]
pub struct ItemInner {
    /// "function": Option<Function> で関数かどうか判断
    pub function: Option<Function>,

    // もし struct や enum も取り込みたい場合:
    // pub struct_: Option<StructItem>,
    // pub enum_: Option<EnumItem>,
    // etc.
}

/// ----------------------------------------
/// 関数アイテム
/// ----------------------------------------
#[derive(Debug, Deserialize)]
pub struct Function {
    /// 関数シグネチャ
    pub sig: FunctionSig,
    // generics, header, has_body なども
    // ここに入っているが今回は省略
}

/// ----------------------------------------
/// (1) functionかどうかを判定し、
/// シグネチャ文字列を生成する関数
/// ----------------------------------------
pub fn item_to_signature_string(item: &Item) -> Option<String> {
    // 関数名
    let name = item.name.as_deref().unwrap_or("unknown");

    // functionがSomeなら関数として扱う
    let Some(func) = &item.inner.function else {
        return None;
    };

    // signature_builder側で文字列を作る
    let sig_str = function_sig_to_string(name, &func.sig);
    Some(sig_str)
}
