use serde::Deserialize;

/// 関数シグネチャの定義 (最小限)
#[derive(Debug, Deserialize)]
pub struct FunctionSig {
    /// (param_name, Type)
    pub inputs: Vec<(String, Type)>,
    #[serde(rename = "output")]
    pub output: Option<Type>,
    #[serde(rename = "is_c_variadic")]
    pub is_c_variadic: bool,
}

/// Rustdoc JSON が型を表すための列挙
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Type {
    BorrowedRef {
        #[serde(rename = "borrowed_ref")]
        borrowed_ref: BorrowedRefType,
    },
    ResolvedPath {
        #[serde(rename = "resolved_path")]
        resolved_path: ResolvedPath,
    },
    Generic {
        generic: String,
    },
    Primitive {
        primitive: String,
    },
    // Tuple, Slice, etc. は省略
}

#[derive(Debug, Deserialize)]
pub struct BorrowedRefType {
    #[serde(rename = "is_mutable")]
    pub is_mutable: bool,
    #[serde(rename = "lifetime")]
    pub lifetime: Option<String>,
    #[serde(rename = "type")]
    pub inner_type: Box<Type>,
}

#[derive(Debug, Deserialize)]
pub struct ResolvedPath {
    pub name: String,
    #[serde(rename = "args")]
    pub args: Option<GenericArgs>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GenericArgs {
    AngleBracketed {
        #[serde(rename = "angle_bracketed")]
        angle_bracketed: AngleBracketedArgs,
    },
}

#[derive(Debug, Deserialize)]
pub struct AngleBracketedArgs {
    #[serde(default)]
    pub args: Vec<GenericArg>,
    #[serde(default)]
    pub constraints: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GenericArg {
    Type { r#type: Box<Type> },
    // 他にもlifetimes, const genericsなどあるが省略
}

/// ----------------------------------------
/// 関数シグネチャを人間向け文字列に変換
/// ----------------------------------------
pub fn function_sig_to_string(name: &str, sig: &FunctionSig) -> String {
    // 引数部分
    let mut params = Vec::new();
    for (param_name, param_type) in &sig.inputs {
        let ty_str = type_to_string(param_type);
        params.push(format!("{}: {}", param_name, ty_str));
    }

    // "fn name(param1: T, param2: U)"
    let mut result = format!("fn {}({})", name, params.join(", "));

    // 戻り値
    if let Some(ty) = &sig.output {
        let return_str = type_to_string(ty);
        if return_str != "()" {
            result.push_str(" -> ");
            result.push_str(&return_str);
        }
    }

    result
}

/// Type列挙をRust風文字列に直す
pub fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::BorrowedRef { borrowed_ref } => {
            let mut s = String::new();
            s.push('&');
            if borrowed_ref.is_mutable {
                s.push_str("mut ");
            }
            // lifetimeも表示したい場合
            if let Some(ref lt) = borrowed_ref.lifetime {
                s.push_str(lt);
                s.push(' ');
            }
            let inner_str = type_to_string(&borrowed_ref.inner_type);
            s.push_str(&inner_str);
            s
        }
        Type::ResolvedPath { resolved_path } => {
            let mut s = resolved_path.name.clone();
            if let Some(args) = &resolved_path.args {
                if let GenericArgs::AngleBracketed { angle_bracketed } = args {
                    if !angle_bracketed.args.is_empty() {
                        let mut generic_parts = Vec::new();
                        for arg in &angle_bracketed.args {
                            if let GenericArg::Type { r#type } = arg {
                                generic_parts.push(type_to_string(r#type));
                            }
                            // lifetimesなどが来た場合は拡張
                        }
                        s.push('<');
                        s.push_str(&generic_parts.join(", "));
                        s.push('>');
                    }
                }
            }
            s
        }
        Type::Generic { generic } => {
            generic.clone() // T, Self etc.
        }
        Type::Primitive { primitive } => {
            primitive.clone() // str, bool, u32 etc.
        }
    }
}
