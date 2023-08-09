use clap::Parser;

/// take a file path and run it.
#[derive(Parser)]
#[command(name = "Clia")]
#[command(author = "Thomas D. <depierre.thomas@gmail.com>")]
#[command(version = "0.1-beta")]
#[command(about = "Run Clia code", long_about = None)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let input = std::fs::read_to_string(&args.path).expect("could not read file");
    let parse = syntax::clia_to_cst(&input);
    let root = ast::Root::cast(parse.syntax()).unwrap();
    let janet_code = janet_backend::ast_to_janet(root);

    let _ = janet_backend::run_janet_code(&janet_code);
}
