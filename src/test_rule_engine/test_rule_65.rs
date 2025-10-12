#[cfg(test)]
use crate::constants::duplicate_attr_docstr_msg;
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
fn test_rule_65_class_single_attr_docstring_single_attr_duplicate() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_1:
    """
    attr_1 = "value 1"
"#;
    let expected = vec![
        format!("4:8 {}", duplicate_attr_docstr_msg("attr_1")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_65_class_single_private_attr_docstring_single_attr_duplicate() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        _attr_1:
        _attr_1:
    """
    _attr_1 = "value 1"
"#;
    let expected = vec![
        format!("4:8 {}", duplicate_attr_docstr_msg("_attr_1")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_65_class_single_attr_docstring_single_attr_duplicate_many() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_1:
        attr_1:
    """
    attr_1 = "value 1"
"#;
    let expected = vec![
        format!("4:8 {}", duplicate_attr_docstr_msg("attr_1")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_65_class_multiple_attr_docstring_duplicate_attr_first() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_1:
        attr_2:
    """
    attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![
        format!("4:8 {}", duplicate_attr_docstr_msg("attr_1")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_65_class_multiple_attr_docstring_duplicate_attr_second() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_2:
        attr_2:
    """
    attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![
        format!("5:8 {}", duplicate_attr_docstr_msg("attr_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_65_class_multiple_attr_docstring_duplicate_attr_all() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_1:
        attr_2:
        attr_2:
    """
    attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![
        format!("4:8 {}", duplicate_attr_docstr_msg("attr_1")),
        format!("6:8 {}", duplicate_attr_docstr_msg("attr_2")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_65_class_single_attr_init_docstring_single_attr_duplicate() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_1:
    """
    def __init__(self):
        """Docstring 2."""
        self.attr_1 = "value 1"
"#;
    let expected = vec![
        format!("4:8 {}", duplicate_attr_docstr_msg("attr_1")),
    ];
    general_test(code, expected);
}
