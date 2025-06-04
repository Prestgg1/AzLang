#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Eded,      // usize
    KicikEded, // i8
    BoyukEded, // i128
    Metn,      // String
}

impl Type {
    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "ədəd" => Some(Type::Eded),
            "kiçik_ədəd" => Some(Type::KicikEded),
            "böyük_ədəd" => Some(Type::BoyukEded),
            "mətn" => Some(Type::Metn),
            _ => None,
        }
    }

    pub fn to_rust_type(&self) -> &'static str {
        match self {
            Type::Eded => "usize",
            Type::KicikEded => "i8",
            Type::BoyukEded => "i128",
            Type::Metn => "String",
        }
    }

    pub fn to_rust_value(&self, value: &str, is_const: bool) -> String {
        match self {
            Type::Metn => {
                let cleaned = value.trim_matches('"');
                if is_const {
                    // const üçün &str literal
                    format!("\"{}\"", cleaned)
                } else {
                    // mutable dəyişən üçün String::from
                    format!("String::from(\"{}\")", cleaned)
                }
            }
            Type::Eded => format!("{}usize", value),
            Type::KicikEded => format!("{}i8", value),
            Type::BoyukEded => format!("{}i128", value),
        }
    }
}
