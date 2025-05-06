use std::path::PathBuf;
use clap::{Parser};

mod docstring;
mod plugin;
mod constants;
mod rule_engine;
/// 🐍 vipyrdocs — Fast. Lethal. Python docstring checks.
#[derive(Parser, Debug)]
#[command(
    name = "vipyrdocs",
    version = "0.1.0",
    about = "🐍 vipyrdocs — Fast. Lethal. Python docstring checks.",
    long_about = r#"
vipyrdocs  — Fast. Lethal. Python docstring checks.

Usage:
  vipyrdocs <PATH> [options]

Arguments:
  <PATH>              Path to a Python file or directory

Options:
  -h, --help          Show this help message and exit
  -V, --version       Show version info and exit

Examples:
  vipyrdocs my_script.py
  vipyrdocs ./src

🔥 Strike out undocumented code with precision.
"#
)]
struct Cli {
    /// Path to a Python file or directory to check
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    if !cli.path.exists() {
        eprintln!("❌ Error: Path '{}' does not exist.", cli.path.display());
        std::process::exit(1);
    }

    println!("🐍 Scanning path: {}", cli.path.display());

    // TODO: Call your core logic here
    // _core::check_docstrings(cli.path);
    if  cli.path.is_dir() {
        
    }
    else if cli.path.is_file() {
        let output = rule_engine::lint_file("",  cli.path.to_str());

        println!("🐍 Scan result: ");
        for line in output {
            println!("  - {}", line);
        }
    }
}
