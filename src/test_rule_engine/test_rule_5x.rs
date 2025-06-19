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
fn test_rule_5x_private_function_raises_single_exc_docstring_no_raises_section() {
    let code = r#"
def _function_1():
    """Docstring 1."""
    raise Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_raise_no_exc_docstring_raises_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
      Exc1:
    """
    raise
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_private_function_single_raise_no_exc_docstring_raises_exc() {
    let code = r#"
def _function_1():
    """Docstring 1.

    Raises:
      Exc1:
    """
    raise
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_raise_exc_docstring_raises() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
    """
    raise Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_nested_function_exc_docstring_no_raises() {
    let code = r#"
def function_1():
    """Docstring 1."""
    def function_2():
        """Docstring 2.

        Raises:
            Exc1:
        """
        raise Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_nested_async_function_exc_docstring_no_raises() {
    let code = r#"
def function_1():
    """Docstring 1."""
    async def function_2():
        """Docstring 2.

        Raises:
            Exc1:
        """
        raise Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_nested_class_exc_docstring_no_raises() {
    let code = r#"
def function_1():
    """Docstring 1."""
    class Class1:
        """Docstring 2."""
        raise Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_exc_call_docstring_single_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
    """
    raise Exc1()
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_exc_lambda_docstring_single_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
    """
    raise (lambda: True)()
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_exc_attribute_docstring_single_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
    """
    raise module.Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_single_exc_attribute_call_docstring_single_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
    """
    raise module.Exc1()
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_function_multiple_exc_docstring_multiple_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
        Exc2:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_method_single_exc_docstring_single_exc() {
    let code = r#"
class Class1:
    """Docstring."""
    def function_1(self):
        """Docstring 1.

        Raises:
            Exc1:
        """
        raise Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_method_single_exc_docstring_single_exc_staticmethod() {
    let code = r#"
class Class1:
    """Docstring."""
    @staticmethod
    def function_1():
        """Docstring 1.

        Raises:
            Exc1:
        """
        raise Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_5x_method_single_exc_docstring_single_exc_classmethod() {
    let code = r#"
class Class1:
    """Docstring."""
    @classmethod
    def function_1(cls):
        """Docstring 1.

        Raises:
            Exc1:
        """
        raise Exc1
"#;
    let expected = vec![];
    general_test(code, expected);
}
