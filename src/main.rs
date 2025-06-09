extern crate lazy_static;
pub mod lexer;
pub mod parser;
pub mod runner;
pub mod syntax;
pub mod transpiler;
pub mod utils;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use color_eyre::eyre::{Result, eyre};
pub use runner::*;
pub use syntax::Syntax;
pub use transpiler::*;
pub use utils::*;
#[derive(Parser)]
#[command(
    name = "azcli",
    about = "AzLang ilə yaz, tərtib et, işə sal — bir əmrlə!",
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// .AzLang kodlarını compile edib işə salır
    Build {
        /// Məs: examples/program.az
        path: String,
    },
    /// Compile edilmiş output faylını işə sal
    Run {
        /// Məs: output/output
        binary: String,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut cmd = Cli::command();

    cmd = cmd.help_template(
        "\x1b[36m{before-help}AzCLI — {about}\x1b[0m\n\n\
         \x1b[33mİstifadə:\x1b[0m {usage}\n\n\
         \x1b[32mƏmrlər:\x1b[0m\n{subcommands}\n\n\
         \x1b[35mSeçimlər:\x1b[0m\n{options}\n\n\
         \x1b[31mYardım üçün əlavə suallarınız varsa bizimlə əlaqə saxlayın!\x1b[0m\n\n\
         {after-help}",
    );
    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches)?;

    match cli.command {
        Commands::Build { path } => build(&path)?,
        Commands::Run { binary } => run(&binary)?,
    }

    Ok(())
}

fn build(input_path: &str) -> Result<()> {
    let input_code = utils::read_file(input_path).map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;

    let syntax = Syntax::load().map_err(|e| eyre!("Syntax xətası!: {}", e))?;

    let tokens = lexer::Lexer::new(&input_code, &syntax).tokenize();
    println!("Tokens: {:#?}", tokens);
    let mut parser = parser::Parser::new(tokens);
    let parsed = parser.parse().map_err(|e| eyre!("Parser xətası: {}", e))?;
    println!("Parsed AST: {:#?}", parsed); // EXPECTED: Expr::String("Mən AzLang ilə yazdım!")
    let rust_code =
        transpiler::transpile(&parsed).map_err(|e| eyre!("Transpilasiya xətası: {}", e))?;
    println!("Rust code: {}", rust_code);
    utils::write_file("output/output.rs", &rust_code)
        .map_err(|e| eyre!("Rust faylı yazıla bilmədi: {}", e))?;
    if runner::compile_and_run("output/output.rs", "output/output").is_err() {
        eprintln!("❌ Proqram işləmədi.");
    }

    /*
       let parsed = parser::parse(&input_code, &syntax).map_err(|e| eyre!("Syntax xətası!: {}", e))?;
    */
    /*     let rust_code =
        transpiler::transpile(&parsed).map_err(|e| eyre!("Transpilasiya xətası: {}", e))?;

    utils::write_file("output/output.rs", &rust_code)
        .map_err(|e| eyre!("Rust faylı yazıla bilmədi: {}", e))?;

    if runner::compile_and_run("output/output.rs", "output/output").is_err() {
        eprintln!("❌ Proqram işləmədi.");
    } */

    Ok(())
}

fn run(binary: &str) -> Result<()> {
    use std::path::Path;
    use std::process::Command;

    let binary_path = Path::new(binary);
    if !binary_path.exists() {
        return Err(eyre!("Fayl mövcud deyil: {}", binary));
    }

    let status = Command::new(binary_path).status()?;
    if !status.success() {
        eprintln!("⚠️ Proqram icrası zamanı xəta.");
    }

    Ok(())
}
