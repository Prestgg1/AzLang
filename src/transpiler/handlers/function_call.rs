use super::super::utils::handle_case_conversion;
use super::{Context, StatementHandler};
use crate::error::Error;

pub struct FunctionCallHandler<'a> {
    pub name: &'a str,
    pub args: &'a [String],
}

impl StatementHandler for FunctionCallHandler<'_> {
    fn handle(&self, context: &mut Context) -> Result<(), Error> {
        match self.name {
            "böyüt" | "kiçilt" => handle_case_conversion(self.name, self.args, context),
            _ => {
                context
                    .main_body
                    .push(format!("    {}({});", self.name, self.args.join(", ")));
                Ok(())
            }
        }
    }
}
