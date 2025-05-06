use crate::constants::args_section_not_in_docstr_msg;
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
fn test_rule_20_function_has_single_arg_docstring_no_args_section() {
    let code: &str = r#"
def function_1(arg_1):
    """Docstring 1."""
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", args_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_20_multiple_function_has_single_arg_docstring_no_args_section() {
    let code: &str = r#"
def function_1(arg_1):
    """Docstring 1."""

def function_2(arg_2):
    """Docstring 2."""
"#;
    let expected: Vec<String> = vec![
        format!("3:4 {}", args_section_not_in_docstr_msg()),
        format!("6:4 {}", args_section_not_in_docstr_msg()),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_20_method_has_single_arg_docstring_no_args_section() {
    let code: &str = r#"
class Class1:
    """Docstring."""
    def function_1(self, arg_1):
        """Docstring 1."""
"#;
    let expected: Vec<String> = vec![format!("5:8 {}", args_section_not_in_docstr_msg())];
    general_test(code, expected);
}
