use crate::constants::mult_yields_sections_in_docstr_msg;
use crate::rule_engine::lint_file;

#[test]
fn test_rule_42_function_yield_multiple_yields_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring.

    Yields:

    Yields:
    """
    yield 1
"#;
    let expected: Vec<String> = vec![format!(
        "3:4 {}",
        mult_yields_sections_in_docstr_msg("Yields,Yields".to_string())
    )];
    general_test(code, expected);
}

#[test]
fn test_rule_42_function_yield_from_multiple_yields_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring.

    Yields:

    Yields:
    """
    yield from tuple()
"#;
    let expected: Vec<String> = vec![format!(
        "3:4 {}",
        mult_yields_sections_in_docstr_msg("Yields,Yields".to_string())
    )];
    general_test(code, expected);
}

#[test]
fn test_rule_42_method_yield_multiple_yields_in_docstring() {
    let code: &str = r#"
class Class1:
    """Docstring."""
    def function_1():
        """Docstring.

        Yields:

        Yields:
        """
        yield 1
"#;
    let expected: Vec<String> = vec![format!(
        "5:8 {}",
        mult_yields_sections_in_docstr_msg("Yields,Yields".to_string())
    )];
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
