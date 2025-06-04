#[cfg(test)]
use crate::constants::{exc_in_docstr_msg, exc_not_in_docstr_msg};
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
fn test_rule_54_function_raises_single_exc_docstring_exc_different() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc2:
    """
    raise Exc1
"#;
    let expected = vec![
        format!("8:10 {}", exc_not_in_docstr_msg("Exc1")),
        format!("3:4 {}", exc_in_docstr_msg("Exc2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_54_function_single_exc_docstring_multiple_exc_different() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc2:
        Exc3:
    """
    raise Exc1
"#;
    let expected = vec![
        format!("9:10 {}", exc_not_in_docstr_msg("Exc1")),
        format!("3:4 {}", exc_in_docstr_msg("Exc2")),
        format!("3:4 {}", exc_in_docstr_msg("Exc3")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_54_function_multiple_exc_docstring_multiple_exc_different() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc3:
        Exc4:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![
        format!("9:10 {}", exc_not_in_docstr_msg("Exc1")),
        format!("10:10 {}", exc_not_in_docstr_msg("Exc2")),
        format!("3:4 {}", exc_in_docstr_msg("Exc3")),
        format!("3:4 {}", exc_in_docstr_msg("Exc4")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_54_function_multiple_exc_docstring_multiple_exc_first_different() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc3:
        Exc2:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![
        format!("9:10 {}", exc_not_in_docstr_msg("Exc1")),
        format!("3:4 {}", exc_in_docstr_msg("Exc3")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_54_function_multiple_exc_docstring_multiple_exc_last_different() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
        Exc3:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![
        format!("10:10 {}", exc_not_in_docstr_msg("Exc2")),
        format!("3:4 {}", exc_in_docstr_msg("Exc3")),
    ];
    general_test(code, expected);
}
