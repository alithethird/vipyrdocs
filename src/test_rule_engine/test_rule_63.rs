#[cfg(test)]
use crate::constants::attr_not_in_docstr_msg;
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
fn test_rule_63_class_has_single_property_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    @property
    def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let expected = vec![format!("6:8 {}", attr_not_in_docstr_msg("attr_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_has_single_cached_property_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    @cached_property
    def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let expected = vec![format!("6:8 {}", attr_not_in_docstr_msg("attr_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_has_single_functools_cached_property_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    @functools.cached_property
    def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let expected = vec![format!("6:8 {}", attr_not_in_docstr_msg("attr_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_has_single_property_with_assignment_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    @property
    def attr_1(self):
        """Docstring 2."""
        self.attr_2 = "value 2"
        return "value 1"
"#;
    let expected = vec![format!("6:8 {}", attr_not_in_docstr_msg("attr_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_has_single_async_property_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    @property
    async def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let expected = vec![format!("6:14 {}", attr_not_in_docstr_msg("attr_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_has_single_property_call_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    @property()
    def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let expected = vec![format!("6:8 {}", attr_not_in_docstr_msg("attr_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_has_single_attr_after_init_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    def __init__(self):
        """Docstring 2."""
    attr_1 = "value 1"
"#;
    let expected = vec![format!("7:4 {}", attr_not_in_docstr_msg("attr_1"))];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_has_multiple_property_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    @property
    def attr_1():
        """Docstring 2."""
        return "value 1"
    @property
    def attr_2():
        """Docstring 3."""
        return "value 3"
"#;
    let expected = vec![
        format!("6:8 {}", attr_not_in_docstr_msg("attr_1")),
        format!("10:8 {}", attr_not_in_docstr_msg("attr_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_multiple_attrs_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![
        format!("5:4 {}", attr_not_in_docstr_msg("attr_1")),
        format!("6:4 {}", attr_not_in_docstr_msg("attr_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_multiple_attrs_first_private_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    _attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![
        format!("6:4 {}", attr_not_in_docstr_msg("attr_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_63_class_multiple_attrs_second_private_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    attr_1 = "value 1"
    _attr_2 = "value 2"
"#;
    let expected = vec![
        format!("5:4 {}", attr_not_in_docstr_msg("attr_1")),
    ];
    general_test(code, expected);
}
