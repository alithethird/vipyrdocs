use crate::rule_engine::lint_file;

#[test]
fn test_disable_single_rule_on_function() {
    let code = r#"
def function_1(arg_1):  # vipyrdocs: disable=D020
    """Docstring."""
"#;
    let output = lint_file(code, None);
    assert!(
        output.is_empty(),
        "Expected no lint messages, got {output:?}"
    );
}

#[test]
fn test_disable_all_rules_on_function() {
    let code = r#"

def function_1(arg_1):  # vipyrdocs: disable=ALL
    """Docstring."""
    return 1
"#;
    let output = lint_file(code, None);
    assert!(
        output.is_empty(),
        "Expected no lint messages, got {output:?}"
    );
}

#[test]
fn test_disable_next_docstring_directive() {
    let code = r#"
# vipyrdocs: disable-next-docstring=D020
def function_1(arg_1):
    """Docstring."""
"#;
    let output = lint_file(code, None);
    assert!(
        output.is_empty(),
        "Expected no lint messages, got {output:?}"
    );
}

#[test]
fn test_inline_disable_on_statement() {
    let code = r#"
def function_1():
    """Docstring."""
    return 1  # vipyrdocs: disable=D030
"#;
    let output = lint_file(code, None);
    assert!(
        output.is_empty(),
        "Expected no lint messages, got {output:?}"
    );
}

#[test]
fn test_disable_comment_before_definition() {
    let code = r#"
# vipyrdocs: disable=D020
def function_1(arg_1):
    """Docstring."""
"#;
    let output = lint_file(code, None);
    assert!(
        output.is_empty(),
        "Expected no lint messages, got {output:?}"
    );
}
