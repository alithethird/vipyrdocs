#[cfg(test)]
use crate::constants::yields_section_in_docstr_msg;
use crate::rule_engine::lint_file;

#[test]
fn test_rule_41_function_no_yield_yields_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring.

    Yields:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", yields_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_41_private_function_no_yield_yields_in_docstring() {
    let code: &str = r#"
def _function_1():
    """Docstring.

    Yields:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", yields_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_41_method_no_yield_yields_in_docstring() {
    let code: &str = r#"
class Class1:
    """Docstring."""
    def function_1():
        """Docstring.

        Yields:
        """
"#;
    let expected: Vec<String> = vec![format!("5:8 {}", yields_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_41_function_yield_no_value_yields_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring.

    Yields:
    """
    yield
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", yields_section_in_docstr_msg())];
    general_test(code, expected);
}

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

