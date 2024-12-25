use serde::Deserialize;
use serde_json::Value;

/// ----------------------------------------
/// 関数シグネチャ (Rustdoc JSON の一部)
/// ----------------------------------------
#[derive(Debug, Deserialize)]
pub struct FunctionSig {
    /// (param_name, type)
    pub inputs: Vec<(String, Type)>,
    /// 戻り値。なければNone (e.g. "-> ()" 相当)
    pub output: Option<Type>,
    /// C-variadicかどうか
    #[serde(default)]
    pub is_c_variadic: bool,
}

/// ----------------------------------------
/// Rustdoc JSON における型表現
/// いろいろなケースがあるため、fallbackを用意
/// ----------------------------------------
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Type {
    /// 参照: { "borrowed_ref": { ... } }
    BorrowedRef {
        borrowed_ref: BorrowedRefType,
    },

    /// ユーザー定義型や標準ライブラリの型: { "resolved_path": { ... } }
    ResolvedPath {
        resolved_path: ResolvedPath,
    },

    /// ジェネリック: { "generic": "T" } や { "generic": "Self" } など
    Generic {
        generic: String,
    },

    /// プリミティブ型: { "primitive": "str" } や { "primitive": "u32" } など
    Primitive {
        primitive: String,
    },

    /// タプル型: { "tuple": [ Type, Type, ... ] }
    Tuple {
        tuple: Vec<Type>,
    },

    /// スライス: { "slice": Type }
    Slice {
        slice: Box<Type>,
    },

    /// そのほか (raw_pointer, qualified_pathなど) が出てくる場合は
    /// ここに落ちる
    Other(Value),
}

/// 参照型: &T / &mut T
#[derive(Debug, Deserialize)]
pub struct BorrowedRefType {
    pub is_mutable: bool,
    pub lifetime: Option<String>,
    #[serde(rename = "type")]
    pub inner_type: Box<Type>,
}

/// ResolvedPath: 型名 + ジェネリクス引数 (AngleBracketed) など
#[derive(Debug, Deserialize)]
pub struct ResolvedPath {
    pub name: String,
    pub args: Option<GenericArgs>,
    // "id" など他にもあり得るが省略
}

/// ジェネリクスの引数
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GenericArgs {
    /// 例: "angle_bracketed": { "args": [...], "constraints": [...] }
    AngleBracketed {
        angle_bracketed: AngleBracketedArgs,
    },
    // 他にも "parenthesized" など場合によりあり
}

/// <T, U, ...>
#[derive(Debug, Deserialize)]
pub struct AngleBracketedArgs {
    #[serde(default)]
    pub args: Vec<GenericArg>,
    #[serde(default)]
    pub constraints: Vec<String>,
}

/// ジェネリック引数は型だけとは限らないが、今回は型に限定
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GenericArg {
    Type { r#type: Box<Type> },
    // Lifetime, Const generics などは今回は割愛
}

/// ----------------------------------------
/// 関数シグネチャをRust風の文字列に
/// 例: fn load_from_file(path: &str) -> Result<Self, IoError>
/// ----------------------------------------
pub fn function_sig_to_string(name: &str, sig: &FunctionSig) -> String {
    // 引数部分
    let mut params = Vec::new();
    for (param_name, param_type) in &sig.inputs {
        let ty_str = type_to_string(param_type);
        params.push(format!("{}: {}", param_name, ty_str));
    }

    // "fn name(param1: Ty, param2: Ty)"
    let mut result = format!("fn {}({})", name, params.join(", "));

    // 戻り値
    if let Some(ref out_ty) = sig.output {
        let out_str = type_to_string(out_ty);
        if out_str != "()" {
            // () はわざわざ表示しない
            result.push_str(" -> ");
            result.push_str(&out_str);
        }
    }

    result
}

/// ----------------------------------------
/// 型をRustっぽい文字列に変換する
/// ----------------------------------------
pub fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::BorrowedRef { borrowed_ref } => {
            let mut s = String::new();
            s.push('&');
            if borrowed_ref.is_mutable {
                s.push_str("mut ");
            }
            // lifetime
            if let Some(ref lt) = borrowed_ref.lifetime {
                s.push_str(lt);
                s.push(' ');
            }
            // 再帰的に中身を文字列化
            s.push_str(&type_to_string(&borrowed_ref.inner_type));
            s
        }
        Type::ResolvedPath { resolved_path } => {
            let mut s = resolved_path.name.clone();
            // ジェネリクス引数
            if let Some(ref args) = resolved_path.args {
                s.push_str(&generic_args_to_string(args));
            }
            s
        }
        Type::Generic { generic } => generic.clone(),
        Type::Primitive { primitive } => primitive.clone(),
        Type::Tuple { tuple } => {
            // 例: (T, U, i32)
            let parts: Vec<String> = tuple.iter().map(|t| type_to_string(t)).collect();
            format!("({})", parts.join(", "))
        }
        Type::Slice { slice } => {
            // 例: [T]
            // 通常Rustでは & [T] がよくあるが、ここでは生スライスとして表示
            let inner_str = type_to_string(slice);
            format!("[{}]", inner_str)
        }
        Type::Other(val) => {
            // 予期しない型 (raw_pointer, qualified_pathなど)
            // いきなりJSON全部を表示すると長いので、簡単にマーカーを入れておく
            format!("/* unknown: {} */", val)
        }
    }
}

/// ----------------------------------------
/// ジェネリクス引数を <...> の文字列に
/// 例: <T, U>
/// ----------------------------------------
fn generic_args_to_string(args: &GenericArgs) -> String {
    match args {
        GenericArgs::AngleBracketed { angle_bracketed } => {
            if angle_bracketed.args.is_empty() {
                // e.g. "Vec<>" みたいになってしまうなら空を返す
                "".to_string()
            } else {
                let mut parts = Vec::new();
                for arg in &angle_bracketed.args {
                    match arg {
                        GenericArg::Type { r#type } => {
                            parts.push(type_to_string(r#type));
                        }
                    }
                }
                format!("<{}>", parts.join(", "))
            }
        }
    }
}
