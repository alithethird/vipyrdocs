use crate::constants::args_section_in_docstr_msg;
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
fn test_rule_21_function_has_no_args_docstring_args_section() {
    let code: &str = r#"
def function_1():
    """Docstring 1.

    Args:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", args_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_21_private_function_has_no_args_docstring_args_section() {
    let code: &str = r#"
def _function_1():
    """Docstring 1.

    Args:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", args_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_21_function_has_single_unused_arg_docstring_args() {
    let code: &str = r#"
def function_1(_arg_1):
    """Docstring 1.

    Args:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", args_section_in_docstr_msg())];
    general_test(code, expected);
}
