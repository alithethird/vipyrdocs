#[cfg(test)]
use crate::constants::{arg_in_docstr_msg, arg_not_in_docstr_msg};
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
fn test_rule_24_function_has_single_arg_docstring_arg_different() {
    let code = r#"
def function_1(arg_1):
    """Docstring 1.

    Args:
        arg_2:
    """
"#;
    let expected = vec![
        format!("2:15 {}", arg_not_in_docstr_msg("arg_1")),
        format!("6:8 {}", arg_in_docstr_msg("arg_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_24_function_single_arg_docstring_multiple_args_different() {
    let code = r#"
def function_1(arg_1):
    """Docstring 1.

    Args:
        arg_2:
        arg_3:
    """
"#;
    let expected = vec![
        format!("2:15 {}", arg_not_in_docstr_msg("arg_1")),
        format!("6:8 {}", arg_in_docstr_msg("arg_2")),
        format!("7:8 {}", arg_in_docstr_msg("arg_3")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_24_function_multiple_arg_docstring_multiple_args_different() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_3:
        arg_4:
    """
"#;
    let expected = vec![
        format!("2:15 {}", arg_not_in_docstr_msg("arg_1")),
        format!("2:22 {}", arg_not_in_docstr_msg("arg_2")),
        format!("6:8 {}", arg_in_docstr_msg("arg_3")),
        format!("7:8 {}", arg_in_docstr_msg("arg_4")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_24_function_multiple_arg_docstring_multiple_args_first_different() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_3:
        arg_2:
    """
"#;
    let expected = vec![
        format!("2:15 {}", arg_not_in_docstr_msg("arg_1")),
        format!("6:8 {}", arg_in_docstr_msg("arg_3")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_24_function_multiple_arg_docstring_multiple_args_last_different() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_1:
        arg_3:
    """
"#;
    let expected = vec![
        format!("2:22 {}", arg_not_in_docstr_msg("arg_2")),
        format!("7:8 {}", arg_in_docstr_msg("arg_3")),
    ];
    general_test(code, expected);
}
