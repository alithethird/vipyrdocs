#[cfg(test)]
use crate::constants::mult_raises_sections_in_docstr_msg;
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
fn test_rule_52_function_raises_single_excs_docstring_multiple_raises_sections_same_name() {
    let code: &str = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:

    Raises:
        Exc1:
    """
    raise Exc1
"#;
    let expected: Vec<String> = vec![format!(
        "3:4 {}",
        mult_raises_sections_in_docstr_msg("Raises,Raises")
    )];
    general_test(code, expected);
}

#[test]
fn test_rule_52_function_raises_single_excs_docstring_multiple_raises_sections_different_name() {
    let code: &str = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:

    Raise:
        Exc1:
    """
    raise Exc1
"#;
    let expected: Vec<String> = vec![format!(
        "3:4 {}",
        mult_raises_sections_in_docstr_msg("Raises,Raise")
    )];
    general_test(code, expected);
}
