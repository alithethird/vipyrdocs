#[cfg(test)]
use crate::constants::raises_section_in_docstr_msg;
use crate::rule_engine::lint_file;

fn general_test(code: &str, expected: Vec<String>) {
    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index], exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index, output[index], exp
        );
    }
}

#[test]
fn test_rule_51_function_raises_no_exc_docstring_raises_section() {
    let code: &str = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", raises_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_51_private_function_raises_no_exc_docstring_raises_section() {
    let code: &str = r#"
def _function_1():
    """Docstring 1.

    Raises:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", raises_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_51_method_raises_no_exc_docstring_raises_section() {
    let code: &str = r#"
class Class1:
    """Docstring."""
    def function_1(self):
        """Docstring 1.

        Raises:
        """
"#;
    let expected: Vec<String> = vec![format!("5:8 {}", raises_section_in_docstr_msg())];
    general_test(code, expected);
}
