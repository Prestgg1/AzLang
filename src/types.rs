#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Eded,      // usize
    KicikEded, // i8
    BoyukEded, // i128
}

impl Type {
    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "ədəd" => Some(Type::Eded),
            "kiçik_ədəd" => Some(Type::KicikEded),
            "böyük_ədəd" => Some(Type::BoyukEded),
            _ => None,
        }
    }

    pub fn to_rust_type(&self) -> &'static str {
        match self {
            Type::Eded => "usize",
            Type::KicikEded => "i8",
            Type::BoyukEded => "i128",
        }
    }
}
