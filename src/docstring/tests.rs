use super::{_Section, _get_sections};

struct TestCase {
    input: &'static [&'static str],
    expected: Option<Vec<_Section>>,
}

#[test]
fn get_sections_handles_common_cases() {
    let cases = [
        TestCase {
            input: &[""],
            expected: None,
        },
        TestCase {
            input: &[" "],
            expected: None,
        },
        TestCase {
            input: &["\t"],
            expected: None,
        },
        TestCase {
            input: &["line 1"],
            expected: Some(vec![_Section {
                name: None,
                subs: vec![],
            }]),
        },
        TestCase {
            input: &["line 1", "line 2"],
            expected: Some(vec![_Section {
                name: None,
                subs: vec![],
            }]),
        },
        TestCase {
            input: &["line 1", "name_1:"],
            expected: Some(vec![_Section {
                name: None,
                subs: vec!["name_1".to_string()],
            }]),
        },
        TestCase {
            input: &["line 1:"],
            expected: Some(vec![_Section {
                name: None,
                subs: vec![],
            }]),
        },
        TestCase {
            input: &["name_1:"],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestCase {
            input: &[" name_1:"],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestCase {
            input: &["\tname_1:"],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestCase {
            input: &["  name_1:"],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestCase {
            input: &["name_1: "],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestCase {
            input: &["name_1: description"],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
    ];

    for case in cases {
        let input_lines = case.input;
        let expected = case.expected;

        let rendered_input = input_lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let result = _get_sections(rendered_input);

        match (expected, result.is_empty()) {
            (None, true) => continue,
            (Some(expected_sections), false) => {
                assert_eq!(
                    result, expected_sections,
                    "Unexpected sections parsed for input {:?}",
                    input_lines
                );
            }
            (None, false) => panic!(
                "Expected no sections for input {:?}, got {:?}",
                input_lines, result
            ),
            (Some(_), true) => panic!("Expected sections for input {:?}, got none", input_lines),
        }
    }
}
