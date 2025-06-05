use super::{Context, StatementHandler};
use crate::error::Error;
use crate::types::Type;

pub struct VariableDeclHandler<'a> {
    pub mutable: bool,
    pub name: &'a str,
    pub var_type: &'a Type,
    pub value: &'a str,
}

impl StatementHandler for VariableDeclHandler<'_> {
    fn handle(&self, context: &mut Context) -> Result<(), Error> {
        let rust_type = self.var_type.to_rust_type();
        if self.mutable {
            context.mutable_vars.insert(self.name.to_string());

            context.main_body.push(format!(
                "    let mut {}: {} = {};",
                self.name, rust_type, self.value
            ));
        } else {
            let decl = if rust_type == "String" {
                format!("    const {}: &str = {};", self.name, self.value)
            } else {
                format!("    const {}: {} = {};", self.name, rust_type, self.value)
            };
            context.main_body.push(decl);
        }
        Ok(())
    }
}
