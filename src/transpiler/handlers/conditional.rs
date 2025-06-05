use super::Context;
use crate::error::Error;
use crate::parser::conditional::Conditional;
use crate::transpiler::handlers::dispatch_statement;
#[derive(Debug)]

pub struct ConditionalHandler<'a>(pub &'a [Conditional]);

impl<'a> ConditionalHandler<'a> {
    pub fn handle(&self, context: &mut Context) -> Result<(), Error> {
        for if_stmt in self.0 {
            match if_stmt {
                Conditional::If { condition, body } => {
                    let mut block = String::new();
                    block.push_str(&format!("if {} {{\n", condition));

                    for stmt in body {
                        let mut nested_context = context.nested();
                        dispatch_statement(stmt, &mut nested_context, true)?;
                        for line in nested_context.main_body {
                            block.push_str(&format!("    {}\n", line));
                        }
                    }

                    block.push_str("}\n");
                    context.main_body.push(block);
                }
                Conditional::ElseIf { condition, body } => {
                    let mut block = String::new();
                    block.push_str(&format!("else if {} {{\n", condition));

                    for stmt in body {
                        let mut nested_context = context.nested(); // context-nin nested metodu olmalıdır
                        dispatch_statement(stmt, &mut nested_context, true)?;
                        for line in nested_context.main_body {
                            block.push_str(&format!("    {}\n", line));
                        }
                    }

                    block.push_str("}\n");
                    context.main_body.push(block);
                }
                Conditional::Else { body } => {
                    let mut block = String::new();
                    block.push_str("else {\n");

                    for stmt in body {
                        let mut nested_context = context.nested(); // context-nin nested metodu olmalıdır
                        dispatch_statement(stmt, &mut nested_context, true)?;
                        for line in nested_context.main_body {
                            block.push_str(&format!("    {}\n", line));
                        }
                    }

                    block.push_str("}\n");
                    context.main_body.push(block);
                }
            }
        }

        Ok(())
    }
}
