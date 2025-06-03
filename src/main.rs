extern crate lazy_static;
mod error;
pub mod parser;
pub mod print_utils;
mod runner;
pub mod transpiler;
mod types;
mod utils;
mod warning;

use color_eyre::eyre::{Result, eyre};

fn main() -> Result<()> {
    // color_eyre install — rəngli error üçün
    color_eyre::install()?;

    let input_code =
        utils::read_file("examples/program.az").map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;

    // Burada parse-da erroru .map_err ilə dəyişdiririk ki, rəngli görünsün
    let parsed = parser::parse(&input_code).map_err(|e| eyre!("Syntax xətası!: {}", e))?;

    let rust_code = transpiler::transpile(&parsed);

    utils::write_file("output/output.rs", &rust_code)
        .map_err(|e| eyre!("Rust faylı yazıla bilmədi: {}", e))?;

    if runner::compile_and_run("output/output.rs", "output/output").is_err() {
        eprintln!("Proqram işləmədi");
    }

    Ok(())
}
