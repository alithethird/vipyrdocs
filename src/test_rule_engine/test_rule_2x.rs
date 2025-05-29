#[cfg(test)]
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
fn test_rule_2x_function_single_arg_docstring_single_arg() {
    let code = r#"
def function_1(arg_1):
    """Docstring 1.

    Args:
        arg_1:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_private_function_single_arg_docstring_single_arg() {
    let code = r#"
def _function_1(arg_1):
    """Docstring 1.

    Args:
        arg_1:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_single_unused_arg_docstring_single_arg() {
    let code = r#"
def function_1(_arg_1):
    """Docstring 1.

    Args:
        _arg_1:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_private_function_single_arg_docstring_no_arg() {
    let code = r#"
def _function_1(arg_1):
    """Docstring 1."""
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_single_unused_arg_docstring_no_args() {
    let code = r#"
def function_1(_arg_1):
    """Docstring 1."""
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_single_unused_args_vararg_docstring_single_arg() {
    let code = r#"
def function_1(*_args):
    """Docstring 1.

    Args:
        _args:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_single_unused_args_vararg_docstring_no_args() {
    let code = r#"
def function_1(*_args):
    """Docstring 1."""
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_single_unused_kwargs_docstring_single_arg() {
    let code = r#"
def function_1(**_kwargs):
    """Docstring 1.

    Args:
        _kwargs:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_single_unused_kwargs_docstring_no_args() {
    let code = r#"
def function_1(**_kwargs):
    """Docstring 1."""
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_varargs_docstring_varargs() {
    let code = r#"
def function_1(*args):
    """Docstring 1.

    Args:
        args:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_kwargs_docstring_kwargs() {
    let code = r#"
def function_1(**kwargs):
    """Docstring 1.

    Args:
        kwargs:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_varargs_kwargs_docstring_both() {
    let code = r#"
def function_1(*args, **kwargs):
    """Docstring 1.

    Args:
        args:
        kwargs:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_multiple_args_docstring_multiple_args() {
    let code = r#"
def function_1(arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_1:
        arg_2:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_multiple_args_first_unused_docstring_second_arg() {
    let code = r#"
def function_1(_arg_1, arg_2):
    """Docstring 1.

    Args:
        arg_2:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_function_multiple_args_second_unused_docstring_first_arg() {
    let code = r#"
def function_1(arg_1, _arg_2):
    """Docstring 1.

    Args:
        arg_1:
    """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_method_single_arg_docstring_single_arg() {
    let code = r#"
class Class1:
    """Docstring."""
    def function_1(self, arg_1):
        """Docstring 1.

        Args:
            arg_1:
        """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_method_single_arg_docstring_single_arg_staticmethod() {
    let code = r#"
class Class1:
    """Docstring."""
    @staticmethod
    def function_1(arg_1):
        """Docstring 1.

        Args:
            arg_1:
        """
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_2x_method_single_arg_docstring_single_arg_classmethod() {
    let code = r#"
class Class1:
    """Docstring."""
    @classmethod
    def function_1(cls, arg_1):
        """Docstring 1.

        Args:
            arg_1:
        """
"#;
    let expected = vec![];
    general_test(code, expected);
}
