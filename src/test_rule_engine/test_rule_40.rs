#[cfg(test)]
use crate::constants::yields_section_not_in_docstr_msg;
use crate::rule_engine::lint_file;

#[test]
fn test_rule_40_function_single_yield_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield 1
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_private_function_single_yield_value_yields_not_in_docstring() {
    let code: &str = r#"
def _function_1():
    """Docstring."""
    yield 1
"#;
    let expected: Vec<String> = Vec::new();
    general_test(code, expected);
}
#[test]
fn test_rule_40_function_single_yield_from_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield from tuple()
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_function_single_falsely_yield_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield 0
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_function_single_none_yield_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield None
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_async_function_single_yield_value_yields_not_in_docstring() {
    let code: &str = r#"
async def function_1():
    """Docstring."""
    yield 1
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_method_single_yield_value_yields_not_in_docstring() {
    let code: &str = r#"
class FooClass:
    """Docstring."""
    def function_1(self):
        """Docstring."""
        yield 1
"#;
    let expected: Vec<String> = vec![format!("6:8 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_function_single_nested_yield_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    if True:
        yield 1
"#;
    let expected: Vec<String> = vec![format!("5:8 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}
#[test]
fn test_rule_40_function_multiple_yield_values_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield 11
    yield 12
"#;
    let expected: Vec<String> = vec![
        format!("4:4 {}", yields_section_not_in_docstr_msg()),
        format!("5:4 {}", yields_section_not_in_docstr_msg()),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_40_function_multiple_yield_first_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield 11
    yield
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_function_multiple_yield_second_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield
    yield 12
"#;
    let expected: Vec<String> = vec![format!("5:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_function_multiple_yield_from_values_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield from tuple()
    yield from list()
"#;
    let expected: Vec<String> = vec![
        format!("4:4 {}", yields_section_not_in_docstr_msg()),
        format!("5:4 {}", yields_section_not_in_docstr_msg()),
    ];
    general_test(code, expected);
}

#[test]
fn test_rule_40_function_multiple_yield_from_first_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield from tuple()
    yield
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_rule_40_function_multiple_yield_from_second_value_yields_not_in_docstring() {
    let code: &str = r#"
def function_1():
    """Docstring."""
    yield
    yield from list()
"#;
    let expected: Vec<String> = vec![format!("5:4 {}", yields_section_not_in_docstr_msg())];
    general_test(code, expected);
}

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
