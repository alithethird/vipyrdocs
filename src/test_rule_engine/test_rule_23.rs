#[cfg(test)]
use crate::constants::arg_not_in_docstr_msg;
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
fn test_rule_23_function_has_single_arg_docstring_no_arg() {
    let code = r#"
def function_1(arg_1):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![format!("2:15 {}", arg_not_in_docstr_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_async_function_has_single_arg_docstring_no_arg() {
    let code = r#"
async def function_1(arg_1):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![format!("2:21 {}", arg_not_in_docstr_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_single_positional_only_arg_docstring_no_arg() {
    let code = r#"
def function_1(arg_1, /):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![format!("2:15 {}", arg_not_in_docstr_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_multiple_positional_only_args_docstring_no_arg() {
    let code = r#"
def function_1(arg_1, arg_2, /):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![
        format!("2:15 {}", arg_not_in_docstr_msg("arg_1")),
        format!("2:22 {}", arg_not_in_docstr_msg("arg_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_23_method_single_positional_only_arg_docstring_no_arg() {
    let code = r#"
class Class1:
    """Docstring."""
    def function_1(self, arg_1, /):
        """Docstring 1.

        Args:
        """
"#;
    let expected = vec![format!("4:25 {}", arg_not_in_docstr_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_single_kwonly_arg_docstring_no_arg() {
    let code = r#"
def function_1(*, arg_1):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![format!("2:18 {}", arg_not_in_docstr_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_multiple_kwonly_args_docstring_no_arg() {
    let code = r#"
def function_1(*, arg_1, arg_2):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![
        format!("2:18 {}", arg_not_in_docstr_msg("arg_1")),
        format!("2:25 {}", arg_not_in_docstr_msg("arg_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_23_method_single_kwonly_arg_docstring_no_arg() {
    let code = r#"
class Class1:
    """Docstring."""
    def function_1(self, *, arg_1):
        """Docstring 1.

        Args:
        """
"#;
    let expected = vec![format!("4:28 {}", arg_not_in_docstr_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_with_args_docstring_no_arg() {
    let code = r#"
def function_1(*args):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![format!("2:16 {}", arg_not_in_docstr_msg("args"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_with_kwargs_docstring_no_arg() {
    let code = r#"
def function_1(**kwargs):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![format!("2:17 {}", arg_not_in_docstr_msg("kwargs"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_with_args_and_kwargs_docstring_no_arg() {
    let code = r#"
def function_1(*args, **kwargs):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![
        format!("2:16 {}", arg_not_in_docstr_msg("args")),
        format!("2:24 {}", arg_not_in_docstr_msg("kwargs")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_with_args_and_kwonly_docstring_no_arg() {
    let code = r#"
def function_1(*args, arg_1):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![
        format!("2:16 {}", arg_not_in_docstr_msg("args")),
        format!("2:22 {}", arg_not_in_docstr_msg("arg_1")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_multiple_args_docstring_no_args() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![
        format!("2:15 {}", arg_not_in_docstr_msg("arg_1")),
        format!("2:22 {}", arg_not_in_docstr_msg("arg_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_args_first_ignored_docstring_no_arg() {
    let code = r#"
def function_1(_arg_1, arg_2):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![format!("2:23 {}", arg_not_in_docstr_msg("arg_2"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_args_second_ignored_docstring_no_arg() {
    let code = r#"
def function_1(arg_1, _arg_2):
    """Docstring 1.

    Args:
    """
"#;
    let expected = vec![format!("2:15 {}", arg_not_in_docstr_msg("arg_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_docstring_has_first_arg_only() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_1:
    """
"#;
    let expected = vec![format!("2:22 {}", arg_not_in_docstr_msg("arg_2"))];
    general_test(code, expected);
}

#[test]
fn test_rule_23_function_docstring_has_second_arg_only() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_2:
    """
"#;
    let expected = vec![format!("2:15 {}", arg_not_in_docstr_msg("arg_1"))];
    general_test(code, expected);
}
