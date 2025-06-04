#[cfg(test)]
use crate::constants::{raises_section_not_in_docstr_msg, re_raise_no_exc_in_docstr_msg};
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
fn test_rule_55_function_single_raise_no_exc_docstring_no_raises_exc() {
    let code = r#"
def function_1():
    """Docstring 1."""
    raise
"#;
    let expected = vec![
        format!("3:4 {}", raises_section_not_in_docstr_msg()),
        format!("3:4 {}", re_raise_no_exc_in_docstr_msg()),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_55_method_raise_no_exc_docstring_no_raises() {
    let code = r#"
class Class1:
    """Docstring."""
    def function_1(self):
        """Docstring 1."""
        raise
"#;
    let expected = vec![
        format!("5:8 {}", raises_section_not_in_docstr_msg()),
        format!("5:8 {}", re_raise_no_exc_in_docstr_msg()),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_55_function_raise_no_exc_docstring_raises_empty() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    raise
"#;
    let expected = vec![format!("3:4 {}", re_raise_no_exc_in_docstr_msg())];
    general_test(code, expected);
}
