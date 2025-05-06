# vipyrdocs

**Fast. Lethal. Python docstring checks.**

**vipyrdocs** is a blazing-fast, Rust-powered CLI tool for checking Python docstrings — a modern reimagining of `flake8-docstrings-complete`, but faster, stricter, and independent.

## 🚀 Features

- 🐍 Standalone CLI — no plugin dance
- ⚡ Written in Rust for speed
- 📖 Enforces complete and consistent Python docstrings
- 🔍 Parses Python using `rustpython` for safe and fast AST inspection

## 🛠️ Installation

Build from source with [cargo](https://www.rust-lang.org/tools/install):

```
cargo install --path .
```

Or use maturin to build a Python-compatible wheel:

```
maturin develop
```

## 🧪 Usage

```
vipyrdocs path/to/your/python/project
```

Outputs any functions/classes missing docstrings or having incomplete ones.

## 🔮 Roadmap

- Configurable docstring rules
- Output in JSON / SARIF
- Git pre-commit hook support
- VSCode integration

## 📜 License

MIT

`vipyrdocs is not affiliated with flake8, but draws inspiration from its ecosystem.`
