#[cfg(test)]
use crate::constants::duplicate_exc_msg;
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
fn test_rule_56_function_single_raise_docstring_raises_duplicate() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
        Exc1:
    """
    raise Exc1
"#;
    let expected = vec![format!("3:4 {}", duplicate_exc_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_56_function_single_raise_docstring_raises_duplicate_many() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
        Exc1:
        Exc1:
    """
    raise Exc1
"#;
    let expected = vec![format!("3:4 {}", duplicate_exc_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_56_function_multiple_raise_docstring_raises_duplicate_first() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
        Exc1:
        Exc2:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![format!("3:4 {}", duplicate_exc_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_56_function_multiple_raise_docstring_raises_duplicate_second() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
        Exc2:
        Exc2:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![format!("3:4 {}", duplicate_exc_msg("Exc2"))];
    general_test(code, expected);
}

#[test]
fn test_rule_56_function_multiple_raise_docstring_raises_duplicate_all() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
        Exc1:
        Exc2:
        Exc2:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![
        format!("3:4 {}", duplicate_exc_msg("Exc1")),
        format!("3:4 {}", duplicate_exc_msg("Exc2")),
    ];
    general_test(code, expected);
}
