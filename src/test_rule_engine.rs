mod test_rule_40;
mod test_rule_41;
mod test_rule_42;

use rstest::rstest;
use crate::rule_engine::lint_file;
use crate::constants::{returns_section_in_docstr_msg, returns_section_not_in_docstr_msg, yields_section_not_in_docstr_msg};

#[test]
pub fn test_lint_file() {
    let test_code: Vec<(&str, Vec<String>)> = vec![
        (
            r#"
@pytest.fixture(scope="module")
def foo_prefix_call():
    pass
"#,
            Vec::new(),
        ),(
            r#"
@additional.pytest.fixture
def foo_nested_prefix():
    pass
"#,
            Vec::new(),
        ),
        (
            r#"
@overload
def function_1():
    ...
"#,
            Vec::new(),
        ),
        (
            r#"
def function_1():
    1
"#,
            Vec::new(),
        ),
        (
            r#"
@overload()
def function_1():
    ...
"#,
            Vec::new(),
        ),
        (
            r#"
@typing.overload
def function_1():
    ...
"#,
            Vec::new(),
        ),
    ];

    let _ = lint_file(
        r#"
@pytest.fixture(scope="module")
def foo():
    pass
"#,
        Some("conftest.py"),
    );
    for (code, expected) in test_code {
        let output = lint_file(code, None);
        println!("{:#?}", output);
        for index in 0..expected.len() {
            assert_eq!(output[index], expected[index]);
        }
    }
    // assert!(output.contains("Missing Docstring"));
    // assert!(output.contains("no_function_docstring.py"));
}

#[rstest]
#[test]
#[case::function_in_class_no_return_value(
    r#"
class FooClass:
    """Docstring."""
    def function_1(self):
        """Docstring."""
"#,
    Vec::<String>::new()
)]#[case::function_no_return_value(
    r#"
def function_1():
    """Docstring."""
"#,
    Vec::<String>::new()
)]
#[case::function_single_return_value_returns_not_in_docstring(
    r#"
def function_1():
    """Docstring."""
    return 1
"#,
    vec![format!("4:4 {}", returns_section_not_in_docstr_msg())]
)]
#[case::function_single_falsely_return_value_returns_not_in_docstring(
    r#"
def function_1():
    """Docstring."""
    return 0
"#,
    vec![format!("4:4 {}", returns_section_not_in_docstr_msg())]
)]
#[case::function_single_none_return_value_returns_not_in_docstring(
    r#"
def function_1():
    """Docstring."""
    return None
"#,
    vec![format!("4:4 {}", returns_section_not_in_docstr_msg())]
)]
#[case::async_function_single_return_value_returns_not_in_docstring(
    r#"
async def function_1():
    """Docstring."""
    return 1
"#,
    vec![format!("4:4 {}", returns_section_not_in_docstr_msg())]
)]
#[case::method_single_return_value_returns_not_in_docstring(
    r#"
class FooClass:
    """Docstring."""
    def function_1(self):
        """Docstring."""
        return 1
"#,
    vec![format!("6:8 {}", returns_section_not_in_docstr_msg())]
)]
#[case::function_single_nested_return_value_returns_not_in_docstring(
    r#"
def function_1():
    """Docstring."""
    if True:
        return 1
"#,
    vec![format!("5:8 {}", returns_section_not_in_docstr_msg())]
)]
#[case::function_multiple_return_value_returns_not_in_docstring(
    r#"
def function_1():
    """Docstring."""
    return 11
    return 12
"#,
    vec![
        format!("4:4 {}", returns_section_not_in_docstr_msg()),
        format!("5:4 {}", returns_section_not_in_docstr_msg())
    ]
)]
#[case::function_multiple_return_first_value_returns_not_in_docstring(
    r#"
def function_1():
    """Docstring."""
    return 11
    return
"#,
    vec![format!("4:4 {}", returns_section_not_in_docstr_msg())]
)]
#[case::function_multiple_return_second_value_returns_not_in_docstring(
    r#"
def function_1():
    """Docstring."""
    return
    return 12
"#,
    vec![format!("5:4 {}", returns_section_not_in_docstr_msg())]
)]
fn test_rule_30(#[case] code: &str, #[case] expected: Vec<String>) {
    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index],
            exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index,
            output[index],
            exp
        );
    }
}


#[rstest]
#[test]
#[case::function_no_return_returns_in_docstring(
    r#"
def function_1():
    """Docstring.

    Returns:
    """
"#,
    vec![format!("3:4 {}", returns_section_in_docstr_msg())]
)]
#[case::private_function_no_return_returns_in_docstring(
    r#"
def _function_1():
    """Docstring.

    Returns:
    """
"#,
    vec![format!("3:4 {}", returns_section_in_docstr_msg())]
)]
#[case::method_no_return_returns_in_docstring(
    r#"
class Class1:
    """Docstring."""
    def function_1():
        """Docstring.

        Returns:
        """
"#,
    vec![format!("5:8 {}", returns_section_in_docstr_msg())]
)]
#[case::function_return_no_value_returns_in_docstring(
    r#"
def function_1():
    """Docstring.

    Returns:
    """
    return
"#,
    vec![format!("3:4 {}", returns_section_in_docstr_msg())]
)]
fn test_rule_31(#[case] code: &str, #[case] expected: Vec<String>) {
    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index],
            exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index,
            output[index],
            exp
        );
    }
}




#[test]
fn test_rule_31_function_return_no_value_returns_in_docstring()
{
    let code: &str = r#"
def function_1():
    """Docstring.

    Returns:
    """
    return
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", returns_section_in_docstr_msg())];
    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index],
            exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index,
            output[index],
            exp
        );
    }
}




#[test]
fn test_rule_30_async_function_single_return_value_returns_not_in_docstring()
{
    let code: &str = r#"
async def function_1():
    """Docstring."""
    return 1
"#;
    let expected: Vec<String> = vec![format!("4:4 {}", returns_section_not_in_docstr_msg())];   
    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index],
            exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index,
            output[index],
            exp
        );
    }
}

#[test]
fn test_rule_30_function_single_nested_return_str_value_returns_not_in_docstring()
{
    let code: &str = r#"
def function_1():
    """Docstring."""
    if False:
        return "true"
"#;
    let expected: Vec<String> = vec![format!("5:8 {}", returns_section_not_in_docstr_msg())];
    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index],
            exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index,
            output[index],
            exp
        );
    }
}
#[test]
fn test_rule_30_function_single_nested_return_value_returns_not_in_docstring()
{
    let code: &str = r#"
def function_1():
    """Docstring."""
    if True:
        return 1
"#;
    let expected: Vec<String> = vec![format!("5:8 {}", returns_section_not_in_docstr_msg())];
    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index],
            exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index,
            output[index],
            exp
        );
    }
}
#[test]
fn test_rule__()
{
    let code: &str = r#"
def function_1():
    """Docstring."""
    return
    return 12
"#;
//     let code = r#"
// def foo():
//     """Docstring.
// 
//     Returns:
//         A value.
// 
//     Return:
//         A value.
//     """  # noqa: {mult_returns_sections_in_docstr_code}
//     return 1"#;
    let expected: Vec<String> = vec![format!("5:4 {}", returns_section_in_docstr_msg())];    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index],
            exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index,
            output[index],
            exp
        );
    }
}








// #[test]
// #[rstest]
// #[case::function_return_multiple_returns_in_docstring(
//     r#"
// def function_1():
//     """Docstring.
//
//     Returns:
//
//     Returns:
//     """
//     return 1
// "#,
//     vec![format!("3:4 {}", mult_returns_section_in_docstr_msg("Returns,Returns"))]
// )]
// #[case::function_return_no_value_returns_in_docstring(
//     r#"
// class Class1:
//     """Docstring."""
//     def function_1():
//         """Docstring.
//
//         Returns:
//
//         Returns:
//         """
//         return 1
// "#,
//     vec![format!("5:8 {}", mult_returns_section_in_docstr_msg("Returns,Returns"))]
// )]
// fn test_rule_32(#[case] code: &str, #[case] expected: Vec<String>) {
//     let output = lint_file(code, None);
//     println!("{:#?}", output);
//     assert_eq!(output.len(), expected.len());
//     for (index, exp) in expected.iter().enumerate() {
//         assert_eq!(
//             &output[index],
//             exp,
//             "Mismatch at output index {}: got `{}`, expected `{}`",
//             index,
//             output[index],
//             exp
//         );
//     }
// }
//
