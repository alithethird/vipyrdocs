use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
mod constants;
mod docstring;
mod plugin;
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

fn get_files_recursively(path: PathBuf) -> Vec<String> {
    let mut py_files = Vec::new();
    visit_dirs(&path, &mut py_files);
    py_files
}

fn visit_dirs(dir: &Path, py_files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, py_files);
            } else if let Some(ext) = path.extension() {
                if ext == "py" {
                    if let Some(path_str) = path.to_str() {
                        py_files.push(path_str.to_string());
                    }
                }
            }
        }
    }
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
    if cli.path.is_dir() {
        let files = get_files_recursively(cli.path);

        println!("🐍 Scan result: ");
        for file in files {
            let output = rule_engine::lint_file("", Some(file.as_str()));
            println!("{}: ", file);

            for line in output {
                println!("  - {}", line);
            }
        }
    } else if cli.path.is_file() {
        let output = rule_engine::lint_file("", cli.path.to_str());

        println!("🐍 Scan result: ");
        for line in output {
            println!("  - {}", line);
        }
    }
}
