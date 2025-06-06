use crate::error::Error;
use crate::parser::Statement;
use crate::transpiler::Context;
use crate::transpiler::handlers::dispatch_statement;

#[derive(Debug)]
pub struct LoopHandler<'a> {
    pub iterator: &'a String,
    pub iterable: &'a String,
    pub body: &'a [Statement],
}

impl<'a> LoopHandler<'a> {
    pub fn handle(&self, context: &mut Context) -> Result<(), Error> {
        let mut block = String::new();
        block.push_str(&format!("for {} in {} {{\n", self.iterator, self.iterable));

        for stmt in self.body {
            let mut nested_context = context.nested();
            dispatch_statement(stmt, &mut nested_context, true)?;
            for line in nested_context.main_body {
                block.push_str(&format!("    {}\n", line));
            }
        }

        block.push_str("}\n");
        context.main_body.push(block);

        Ok(())
    }
}
