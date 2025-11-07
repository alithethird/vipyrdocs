use crate::rule_engine::lint_file;

fn main() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    @property
    def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let output = lint_file(code, None);
    println!("Output: {:?}", output);
}