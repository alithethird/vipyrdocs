use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
mod constants;
mod docstring;
mod inheritance;
mod plugin;
mod rule_engine;

use inheritance::InheritanceTracker;
/// 🐍 vipyrdocs — Fast. Lethal. Python docstring checks.
#[derive(Parser, Debug)]
#[command(
    name = "vipyrdocs",
    version = "0.1.3",
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
    let mut files_with_issues = 0usize;
    let mut total_issue_count = 0usize;
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

        // Create inheritance tracker for cross-file validation
        let mut tracker = InheritanceTracker::new();

        // First pass: collect all abstract methods and concrete methods
        for file in &files {
            let file_str = match file.to_str() {
                Some(value) => value,
                None => continue,
            };

            // Read file and collect inheritance info
            rule_engine::collect_inheritance_info(file_str, &mut tracker);
        }

        // Validate inheritance relationships
        let inheritance_violations = tracker.validate();

        // Get methods that implement abstract methods (for docstring inheritance)
        let implementing_methods = tracker.get_methods_implementing_abstract();

        println!("🐍 Scan result:");
        let mut issues_found = false;

        // Second pass: check regular docstring rules
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

            let mut output = rule_engine::lint_file_with_inheritance(
                "",
                Some(file_str.as_str()),
                Some(&implementing_methods),
            );

            // Add inheritance violations for this file
            for violation in &inheritance_violations {
                if violation.file_path == file_str {
                    let error_msg = format!(
                        "{}:{} {} {}",
                        violation.line,
                        0,
                        violation.get_error_code(),
                        violation.to_error_message()
                    );
                    output.push(error_msg);
                }
            }

            if output.is_empty() {
                files_scanned += 1;
                continue;
            }

            if !issues_found {
                issues_found = true;
            }

            files_with_issues += 1;
            total_issue_count += output.len();

            println!("  🚨 {}:", file_str);
            for line in output {
                println!("  - {}", line);
            }

            files_scanned += 1;
        }

        if !issues_found {
            println!("  ✅ No issues found.");
        }
    }

    if files_scanned == 0 && !any_missing_paths {
        println!("⚠️ No Python files scanned.");
        return;
    }

    if files_scanned > 0 {
        println!(
            "\n📊 Summary: scanned {} file{}; {} had issues; {} issue{} total.",
            files_scanned,
            if files_scanned == 1 { "" } else { "s" },
            files_with_issues,
            total_issue_count,
            if total_issue_count == 1 { "" } else { "s" }
        );
    }
}
