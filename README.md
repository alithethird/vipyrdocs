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

## 🧪 Usage

```
vipyrdocs path/to/your/python/project
```

Outputs any functions/classes missing docstrings or having incomplete ones.

### Inline Rule Suppression

Add a `# vipyrdocs: disable=<CODES>` comment to the end of a `def`/`class` line or statement to silence specific checks (for example, `# vipyrdocs: disable=D020`). Use `ALL` to silence everything for that scope, or `disable-next-docstring`/`disable-file` variants to target the next docstring or whole file when needed.

## 🔮 Roadmap

- Configurable docstring rules
- Output in JSON / SARIF
- Git pre-commit hook support
- VSCode integration

### Current rules 9/26

- 👌 DCO010: docstring missing on a function/ method/ class.
- 👌 DCO020: function/ method has one or more arguments and the docstring does not have an arguments section.
- 👌 DCO021: function/ method with no arguments and the docstring has an arguments section.
- 👌 DCO022: function/ method with one or more arguments and the docstring has multiple arguments sections.
- 👌 DCO023: function/ method has one or more arguments not described in the docstring.
- 👌 DCO024: function/ method has one or more arguments described in the docstring which are not arguments of the function/ method.
- 👌 DCO025: function/ method has one or more arguments described in the docstring multiple times.
- 👌 DCO030: function/ method that returns a value does not have the returns section in the docstring.
- 👌 DCO031: function/ method that does not return a value has the returns section in the docstring.
- 👌 DCO032: function/ method that returns a value and the docstring has multiple returns sections.
- 👌 DCO040: function/ method that yields a value does not have the yields section in the docstring.
- 👌 DCO041: function/ method that does not yield a value has the yields section in the docstring.
- 👌 DCO042: function/ method that yields a value and the docstring has multiple yields sections.
- 👌 DCO050: function/ method raises one or more exceptions and the docstring does not have a raises section.
- 👌 DCO051: function/ method that raises no exceptions and the docstring has a raises section.
- 👌 DCO052: function/ method that raises one or more exceptions and the docstring has multiple raises sections.
- 👌 DCO053: function/ method that raises one or more exceptions where one or more of the exceptions is not described in the docstring.
- 👌 DCO054: function/ method has one or more exceptions described in the docstring which are not raised in the function/ method.
- 👌 DCO055: function/ method that has a raise without an exception has an empty raises section in the docstring.
- 👌 DCO056: function/ method has one or more exceptions described in the docstring multiple times.
- 👌 DCO060: class has one or more public attributes and the docstring does not have an attributes section.
- 👌 DCO061: class with no attributes and the docstring has an attributes section.
- 👌 DCO062: class with one or more attributes and the docstring has multiple attributes sections.
- 👌 DCO063: class has one or more public attributes not described in the docstring.
- 👌 DCO064: class has one or more attributes described in the docstring which are not attributes of the class.
- 🙅 DCO065: class has one or more attributes described in the docstring multiple times.

## 📜 License

MIT

`vipyrdocs is not affiliated with flake8, but draws inspiration from its ecosystem.`
