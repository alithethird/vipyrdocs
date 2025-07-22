#[cfg(test)]
use crate::constants::attrs_section_in_docstr_msg;
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
fn test_rule_61_class_has_single_unused_attr_docstring_attrs() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
    """
    _attr_1 = "value 1"
"#;
    let expected = vec![format!("3:4 {}", attrs_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_61_class_has_no_attrs_docstring_attrs_section() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:
    """
"#;
    let expected = vec![format!("3:4 {}", attrs_section_in_docstr_msg())];
    general_test(code, expected);
}
