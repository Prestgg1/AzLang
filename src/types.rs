use crate::syntax::Syntax;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Eded,             // usize
    KicikEded,        // i8
    BoyukEded,        // i128
    Metn,             // String
    Array(Box<Type>), // Yeni: array tipi, içindəki elementin tipi ilə
}

impl Type {
    pub fn from_str(s: &str, syntax: &Syntax) -> Option<Type> {
        if s == syntax.integer {
            Some(Type::Eded)
        } else if s == syntax.lowinteger {
            Some(Type::KicikEded)
        } else if s == syntax.biginteger {
            Some(Type::BoyukEded)
        } else if s == syntax.string {
            Some(Type::Metn)
        } else if s.starts_with("siyahı<") && s.ends_with('>') {
            let inner = &s[8..s.len() - 1]; // siyahı<mətn> → mətn
            let inner_type = Type::from_str(inner, syntax)?;
            Some(Type::Array(Box::new(inner_type)))
        } else {
            None
        }
    }

    pub fn to_rust_type(&self) -> &'static str {
        match self {
            Type::Eded => "usize",
            Type::KicikEded => "i8",
            Type::BoyukEded => "i128",
            Type::Metn => "String",
            Type::Array(_) => "Vec<_>",
        }
    }

    pub fn to_rust_value(&self, value: &str, is_const: bool) -> String {
        match self {
            Type::Metn => {
                let cleaned = value.trim_matches('"');
                if is_const {
                    format!("\"{}\"", cleaned)
                } else {
                    format!("String::from(\"{}\")", cleaned)
                }
            }
            Type::Array(inner_type) => {
                let cleaned = value.trim_matches(&['[', ']'][..]);
                let items: Vec<String> = cleaned
                    .split(',')
                    .map(|s| inner_type.to_rust_value(s.trim(), false))
                    .collect();
                format!("vec![{}]", items.join(", "))
            }
            _ => value.to_string(),
        }
    }
}
