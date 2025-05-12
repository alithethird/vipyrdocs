#[cfg(test)]
use crate::constants::mult_args_sections_in_docstr_msg;
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
fn test_rule_22_function_has_single_args_docstring_multiple_args_sections_same_name() {
    let code: &str = r#"
def function_1(arg_1):
    """Docstring 1.

    Args:
arg_1:

    Args:
arg_1:
    """
"#;
    let expected: Vec<String> = vec![format!(
        "3:4 {}",
        mult_args_sections_in_docstr_msg("Args,Args".to_string())
    )];
    general_test(code, expected);
}

#[test]
fn test_rule_22_function_has_single_args_docstring_multiple_args_sections_different_name() {
    let code: &str = r#"
def function_1(arg_1):
    """Docstring 1.

    Args:
arg_1:

    Arguments:
arg_1:
    """
"#;
    let expected: Vec<String> = vec![format!(
        "3:4 {}",
        mult_args_sections_in_docstr_msg("Args,Arguments".to_string())
    )];
    general_test(code, expected);
}
