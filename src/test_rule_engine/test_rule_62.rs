#[cfg(test)]
use crate::constants::mult_attrs_section_in_docstr_msg;
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
fn test_rule_62_class_has_single_attrs_docstring_multiple_attrs_sections_alternate_name() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:

    Attributes:
        attr_1:
    """
    attr_1 = "value 1"
"#;
    let expected = vec![format!(
        "3:4 {}",
        mult_attrs_section_in_docstr_msg("Attrs,Attributes")
    )];
    general_test(code, expected);
}

#[test]
fn test_rule_62_class_has_single_attrs_docstring_multiple_attrs_sections_same_name() {
    let code = r#"
class Class1:
    """Docstring 1.

    Attrs:
        attr_1:

    Attrs:
        attr_1:
    """
    attr_1 = "value 1"
"#;
    let expected = vec![format!(
        "3:4 {}",
        mult_attrs_section_in_docstr_msg("Attrs,Attrs")
    )];
    general_test(code, expected);
}
