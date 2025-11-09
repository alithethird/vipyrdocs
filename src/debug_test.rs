#[cfg(test)]
mod test_debug_attr {
    use crate::rule_engine::lint_file;

    #[test]
    fn test_simple_property() {
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
        println!("Test output: {:?}", output);
        // Let test fail to see output
        assert!(false, "Debug test - output: {:?}", output);
    }
}