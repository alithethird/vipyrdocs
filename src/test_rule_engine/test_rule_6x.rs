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

// Positive test cases (no errors expected)

#[test]
fn test_rule_6x_class_single_attr_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_property_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    @property
    def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_cached_property_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    @cached_property
    def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_functools_cached_property_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    @functools.cached_property
    def attr_1():
        """Docstring 2."""
        return "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_attr_typed_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    attr_1: str = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_attr_augmented_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    attr_1 += "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_attr_init_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    def __init__(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_attr_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_multiple_attr_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_2:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
    def method_2(self):
        """Docstring 3."""
        self.attr_2 = "value 2"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_attr_classmethod_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    @classmethod
    def method_1(cls):
        """Docstring 2."""
        cls.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_private_attr_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        _attr_1:
    """
    _attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_private_attr_no_docstring() {
    let code = r#"
class Class1:
    """Docstring 1."""
    _attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_var_init_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def __init__(self):
        """Docstring 2."""
        var_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_property_with_assignment_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    @property
    def attr_1(self):
        """Docstring 2."""
        self.attr_2 = "value 2"
        return "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_property_with_assignment_docstring_both_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_2:
    """
    @property
    def attr_1(self):
        """Docstring 2."""
        self.attr_2 = "value 2"
        return "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_in_init_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def __init__(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_in_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_typed_in_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        self.attr_1: str = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_typed_in_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1: str = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_augmented_in_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        self.attr_1 += "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_augmented_in_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1 += "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_multiple_attr_in_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = self.attr_2 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_multiple_attr_in_method_docstring_multiple_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_2:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = self.attr_2 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_nested_in_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        self.attr_1.nested_attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_nested_in_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1.nested_attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_deep_nested_in_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        self.attr_1.nested_attr_1.nested_attr_2 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_deep_nested_in_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1.nested_attr_1.nested_attr_2 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_multiple_attr_in_multiple_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
    def method_2(self):
        """Docstring 3."""
        self.attr_2 = "value 2"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_multiple_attr_in_multiple_method_docstring_single_attr_first() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
    def method_2(self):
        """Docstring 3."""
        self.attr_2 = "value 2"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_multiple_attr_in_multiple_method_docstring_single_attr_second() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_2:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
    def method_2(self):
        """Docstring 3."""
        self.attr_2 = "value 2"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_multiple_attr_in_multiple_method_docstring_multiple_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_2:
    """
    def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
    def method_2(self):
        """Docstring 3."""
        self.attr_2 = "value 2"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_in_async_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    async def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_in_async_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    async def method_1(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_in_classmethod_method_docstring_no_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    @classmethod
    def method_1(cls):
        """Docstring 2."""
        cls.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_has_single_attr_in_classmethod_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    @classmethod
    def method_1(cls):
        """Docstring 2."""
        cls.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_var_method_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        var_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_var_classmethod_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1."""
    @classmethod
    def method_1(cls):
        """Docstring 2."""
        var_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_multiple_attr_docstring_multiple_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_2:
    """
    attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_multiple_attr_first_private_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_2:
    """
    _attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_multiple_attr_second_private_docstring_single_attr() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
    attr_1 = "value 1"
    _attr_2 = "value 2"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_nested_class_single_attr_docstring_no_attrs() {
    let code = r#"
class Class1:
    """Docstring 1."""
    class Class2:
        """Docstring 2.

        Attrs:
            attr_1:
        """
        attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_attr_method_nested_method_docstring_no_attrs() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        def nested_function_1(self):
            """Docstring 3."""
            self.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_attr_method_nested_async_method_docstring_no_attrs() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        async def nested_function_1(self):
            """Docstring 3."""
            self.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_6x_class_single_attr_method_nested_classmethod_docstring_no_attrs() {
    let code = r#"
class Class1:
    """Docstring 1."""
    def method_1(self):
        """Docstring 2."""
        def nested_function_1(cls):
            """Docstring 3."""
            cls.attr_1 = "value 1"
"#;
    let expected = vec![];
    general_test(code, expected);
}
