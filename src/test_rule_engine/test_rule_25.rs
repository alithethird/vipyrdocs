#[cfg(test)]
use crate::constants::duplicate_arg_msg;
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
fn test_rule_25_function_single_arg_docstring_duplicate_arg() {
    let code = r#"
def function_1(arg_1):
    """Docstring 1.

    Args:
        arg_1:
        arg_1:
    """
"#;
    let expected = vec![format!("3:4 {}", duplicate_arg_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_25_function_single_unused_arg_docstring_duplicate_arg() {
    let code = r#"
def function_1(_arg_1):
    """Docstring 1.

    Args:
        _arg_1:
        _arg_1:
    """
"#;
    let expected = vec![format!("3:4 {}", duplicate_arg_msg("_arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_25_function_single_arg_docstring_duplicate_arg_many() {
    let code = r#"
def function_1(arg_1):
    """Docstring 1.

    Args:
        arg_1:
        arg_1:
        arg_1:
    """
"#;
    let expected = vec![format!("3:4 {}", duplicate_arg_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_25_function_multiple_arg_docstring_duplicate_arg_first() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_1:
        arg_1:
        arg_2:
    """
"#;
    let expected = vec![format!("3:4 {}", duplicate_arg_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_25_function_multiple_arg_docstring_duplicate_arg_second() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_1:
        arg_2:
        arg_2:
    """
"#;
    let expected = vec![format!("3:4 {}", duplicate_arg_msg("arg_2"))];
    general_test(code, expected);
}

#[test]
fn test_rule_25_function_multiple_arg_docstring_duplicate_arg_all() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_1:
        arg_1:
        arg_2:
        arg_2:
    """
"#;
    let expected = vec![
        format!("3:4 {}", duplicate_arg_msg("arg_1")),
        format!("3:4 {}", duplicate_arg_msg("arg_2")),
    ];
    general_test(code, expected);
}
#[test]
fn test_rule_25_method_single_arg_docstring_single_arg_duplicate() {
    let code = r#"
class Class1:
    """Docstring."""
    def function_1(self, arg_1):
        """Docstring 1.

        Args:
            arg_1:
            arg_1:
        """
"#;
    let expected = vec![format!("5:8 {}", duplicate_arg_msg("arg_1"))];
    general_test(code, expected);
}
