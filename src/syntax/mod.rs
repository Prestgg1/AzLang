use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
pub struct Syntax {
    pub print: String,
    pub mutable_decl: String,
    pub constant_decl: String,
    pub function_def: String,
    pub conditional: String,
    pub _else: String,
    pub else_if: String,
    pub drop: String,
    pub integer: String,
    pub biginteger: String,
    pub lowinteger: String,
    pub string: String,
    pub _loop: String,
    pub len: String,
    pub input: String,
    pub listtype: String,
    pub bool: String,
    pub return_name: String,
}

impl Syntax {
    pub fn load() -> std::result::Result<Self, std::io::Error> {
        let contents = include_str!("./config.json");
        let syntax = from_str(&contents)?;
        Ok(syntax)
    }
    pub fn is_type_str(&self, word: &str) -> bool {
        word == self.string
            || word == self.integer
            || word == self.biginteger
            || word == self.lowinteger
    }
}
