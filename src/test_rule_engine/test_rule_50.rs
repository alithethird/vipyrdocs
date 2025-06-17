#[cfg(test)]
use crate::constants::raises_section_not_in_docstr_msg;
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
fn test_rule_50_method_raises_single_exc_docstring_no_raises_section() {
    let code: &str = r#"
class Class1:
    """Docstring."""
    def function_1(self):
        """Docstring 1."""
        raise Exc1
"#;
    let expected: Vec<String> = vec![format!("6:8 {}", raises_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_50_function_raises_single_exc_docstring_no_raises_section() {
    let code: &str = r#"
def function_1():
    """Docstring 1."""
    raise Exc1
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", raises_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_50_private_function_raises_single_exc_docstring_no_raises_section() {
    let code: &str = r#"
def _function_1():
    """Docstring 1."""
    raise Exc1
"#;
    let expected: Vec<String> = vec![];
    general_test(code, expected);
}

#[test]
fn test_rule_50_multiple_function_raises_single_exc_docstring_no_raises_section() {
    let code: &str = r#"
def function_1():
    """Docstring 1."""
    raise Exc1

def function_2():
    """Docstring 2."""
    raise Exc2
"#;
    let expected: Vec<String> = vec![
        format!("4:4 {}", raises_section_not_in_docstr_msg()),
        format!("8:4 {}", raises_section_not_in_docstr_msg()),
    ];
    general_test(code, expected);
}
