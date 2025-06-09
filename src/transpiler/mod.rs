pub mod expr;
pub mod main_fn;
pub mod utils;

use crate::parser::Program;
use main_fn::generate_main_fn;

pub fn transpile(program: &Program) -> Result<String, String> {
    generate_main_fn(program)
}
