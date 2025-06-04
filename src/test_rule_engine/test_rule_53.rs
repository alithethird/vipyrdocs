#[cfg(test)]
use crate::constants::exc_not_in_docstr_msg;
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
fn test_rule_53_function_raises_single_exc_docstring_no_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    raise Exc1
"#;
    let expected = vec![format!("7:10 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_function_raises_single_and_plain_raise_docstring_no_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    raise Exc1
    raise
"#;
    let expected = vec![format!("7:10 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_function_raises_exc_call_docstring_no_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    raise Exc1()
"#;
    let expected = vec![format!("7:10 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_function_raises_nested_exc_docstring_no_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    raise module.Exc1
"#;
    let expected = vec![format!("7:17 {}", exc_not_in_docstr_msg("Exc1"))];
    // this returns 7:10 in the original
    general_test(code, expected);
}

#[test]
fn test_rule_53_async_function_raises_exc_docstring_no_exc() {
    let code = r#"
async def function_1():
    """Docstring 1.

    Raises:
    """
    raise Exc1
"#;
    let expected = vec![format!("7:10 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_function_raises_multiple_excs_docstring_no_exc() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![
        format!("7:10 {}", exc_not_in_docstr_msg("Exc1")),
        format!("8:10 {}", exc_not_in_docstr_msg("Exc2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_53_nested_function_raises_docstring_no_exc_on_outer() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    def function_2():
        """Docstring 2.

        Raises:
            Exc1:
        """
        raise Exc1
    raise Exc2
"#;
    let expected = vec![format!("14:10 {}", exc_not_in_docstr_msg("Exc2"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_nested_async_function_raises_docstring_no_exc_on_outer() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    async def function_2():
        """Docstring 2.

        Raises:
            Exc1:
        """
        raise Exc1
    raise Exc2
"#;
    let expected = vec![format!("14:10 {}", exc_not_in_docstr_msg("Exc2"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_nested_class_raises_docstring_no_exc_on_outer() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    class Class1:
        """Docstring 2."""
        raise Exc1
    raise Exc2
"#;
    let expected = vec![format!("10:10 {}", exc_not_in_docstr_msg("Exc2"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_function_raises_then_nested_function() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
    raise Exc1
    def function_2():
        """Docstring 2.

        Raises:
            Exc2:
        """
        raise Exc2
"#;
    let expected = vec![format!("7:10 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_function_multiple_excs_docstring_first() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc1:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![format!("9:10 {}", exc_not_in_docstr_msg("Exc2"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_function_multiple_excs_docstring_second() {
    let code = r#"
def function_1():
    """Docstring 1.

    Raises:
        Exc2:
    """
    raise Exc1
    raise Exc2
"#;
    let expected = vec![format!("8:10 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}
#[test]
fn test_rule_53_method_raises_single_exc_docstring_no_exc() {
    let code = r#"
class Class1:
    """Docstring."""
    def function_1(self):
        """Docstring 1.

        Raises:
        """
        raise Exc1
"#;
    let expected = vec![format!("9:14 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_staticmethod_raises_single_exc_docstring_no_exc() {
    let code = r#"
class Class1:
    """Docstring."""
    @staticmethod
    def function_1():
        """Docstring 1.

        Raises:
        """
        raise Exc1
"#;
    let expected = vec![format!("10:14 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_53_classmethod_raises_single_exc_docstring_no_exc() {
    let code = r#"
class Class1:
    """Docstring."""
    @classmethod
    def function_1(cls):
        """Docstring 1.

        Raises:
        """
        raise Exc1
"#;
    let expected = vec![format!("10:14 {}", exc_not_in_docstr_msg("Exc1"))];
    general_test(code, expected);
}
