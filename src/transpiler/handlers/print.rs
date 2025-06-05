use super::super::utils::check_assignments_in_print;
use super::{Context, StatementHandler};
use crate::error::Error;
use crate::functions::handle_print;

pub struct PrintHandler<'a>(pub &'a str);

impl StatementHandler for PrintHandler<'_> {
    fn handle(&self, context: &mut Context) -> Result<(), Error> {
        let content = self.0;
        context
            .main_body
            .push(format!("    {};", handle_print(content)));
        check_assignments_in_print(
            content,
            &context.mutable_vars,
            &mut context.used_as_assignment,
        );
        Ok(())
    }
}
