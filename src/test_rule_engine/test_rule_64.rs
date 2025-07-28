#[cfg(test)]
use crate::constants::{attr_not_in_docstr_msg, attr_in_docstr_msg};
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
fn test_rule_64_class_has_single_attr_docstring_attr_different() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_2:
    """
    attr_1 = "value 1"
"#;
    let expected = vec![format!("6:4 {}", attr_not_in_docstr_msg("attr_1")),
                        format!("4:8 {}", attr_in_docstr_msg("attr_2"))];
    general_test(code, expected);
}
#[test]
fn test_rule_64_class_single_attr_docstring_multiple_attrs_different() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_2:
        attr_3:
    """
    attr_1 = "value 1"
"#;
    let expected = vec![
        format!("7:4 {}", attr_not_in_docstr_msg("attr_1")),
        format!("4:8 {}", attr_in_docstr_msg("attr_2")),
        format!("5:8 {}", attr_in_docstr_msg("attr_3")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_64_class_multiple_attr_docstring_multiple_attrs_different() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_3:
        attr_4:
    """
    attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![
        format!("7:4 {}", attr_not_in_docstr_msg("attr_1")),
        format!("8:4 {}", attr_not_in_docstr_msg("attr_2")),
        format!("4:8 {}", attr_in_docstr_msg("attr_3")),
        format!("5:8 {}", attr_in_docstr_msg("attr_4")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_64_class_multiple_attr_docstring_multiple_attrs_first_different() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_2:
        attr_3:
    """
    attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![
        format!("7:4 {}", attr_not_in_docstr_msg("attr_1")),
        format!("5:8 {}", attr_in_docstr_msg("attr_3")),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_64_class_multiple_attr_docstring_multiple_attrs_second_different() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
        attr_3:
    """
    attr_1 = "value 1"
    attr_2 = "value 2"
"#;
    let expected = vec![
        format!("8:4 {}", attr_not_in_docstr_msg("attr_2")),
        format!("5:8 {}", attr_in_docstr_msg("attr_3")),
    ];
    general_test(code, expected);
}
