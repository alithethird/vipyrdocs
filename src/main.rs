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
    /// Paths to Python files or directories to check
    #[arg(value_name = "PATH", num_args = 1..)]
    paths: Vec<PathBuf>,
}

fn collect_python_files(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        if is_python_file(path) {
            return vec![path.to_path_buf()];
        }
        return Vec::new();
    }

    let mut py_files = Vec::new();
    if path.is_dir() {
        visit_dirs(path, &mut py_files);
    }
    py_files
}

fn visit_dirs(dir: &Path, py_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, py_files);
            } else if is_python_file(&path) {
                py_files.push(path);
            }
        }
    }
}

fn is_python_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("py"))
        .unwrap_or(false)
}

fn main() {
    let cli = Cli::parse();

    let mut files_scanned = 0usize;
    let mut any_missing_paths = false;

    for path in cli.paths {
        if !path.exists() {
            eprintln!("❌ Error: Path '{}' does not exist.", path.display());
            any_missing_paths = true;
            continue;
        }

        println!("🐍 Scanning path: {}", path.display());
        let files = collect_python_files(&path);

        if files.is_empty() {
            if path.is_file() {
                println!("  ⚠️ Skipping: not a Python file.");
            } else {
                println!("  ⚠️ No Python files found in directory.");
            }
            continue;
        }

        println!("🐍 Scan result:");
        for file in files {
            let file_str = match file.to_str() {
                Some(value) => value.to_string(),
                None => {
                    eprintln!(
                        "  ⚠️ Skipping '{}': path is not valid UTF-8.",
                        file.display()
                    );
                    continue;
                }
            };

            let output = rule_engine::lint_file("", Some(file_str.as_str()));
            println!("{}:", file_str);

            if output.is_empty() {
                println!("  ✅ No issues found.");
            } else {
                for line in output {
                    println!("  - {}", line);
                }
            }

            files_scanned += 1;
        }
    }

    if files_scanned == 0 && !any_missing_paths {
        println!("⚠️ No Python files scanned.");
    }
}
