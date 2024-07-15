use std::error::Error;
use std::fs;
use std::io::stdout;
use std::io::{stdin, Write};
use std::path::{Path, PathBuf};

mod environment;
mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;

use clap::Parser as ClapParser;
use interpreter::{AstPrinter, Interpreter, InterpreterLike};
use parser::Parser;
use scanner::Scanner;

/// Simple Lox language interpreter.
#[derive(ClapParser, Debug)]
struct Cli {
    /// Lox source file.
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    /// Print AST instead of interpreting.
    #[arg(short, long)]
    ast: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match (cli.ast, cli.file) {
        (true, Some(file)) => run_file(AstPrinter::new(), file),
        (true, None) => run_prompt(AstPrinter::new()),
        (false, Some(file)) => run_file(Interpreter::new(), file),
        (false, None) => run_prompt(Interpreter::new()),
    }?;

    Ok(())
}

/// Load and interpret a Lox source code file
fn run_file<T: InterpreterLike, P: AsRef<Path>>(
    mut interpreter: T,
    path: P,
) -> Result<(), Box<dyn Error>> {
    let string = fs::read_to_string(path)?;
    run(&mut interpreter, &string);
    Ok(())
}

/// Run interactive prompt for the Lox interpreter
fn run_prompt<T: InterpreterLike>(mut interpreter: T) -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    loop {
        print!("> ");
        stdout().flush()?;
        match stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };
        run(&mut interpreter, &line);
        line.clear();
    }
    Ok(())
}

/// Token scanner loop for a single file or line (interactive)
fn run<T: InterpreterLike>(interpreter: &mut T, source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    interpreter.interpret(&statements);
}
