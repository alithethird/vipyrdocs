use super::{parse_from_str_for_tests, Docstring, _Section, _get_sections};
use rustpython_ast::text_size::{TextRange, TextSize};

#[derive(Clone, Copy)]
struct SectionExpectation {
    name: Option<&'static str>,
    subs: &'static [&'static str],
}

struct SectionCase {
    input: &'static [&'static str],
    expected: &'static [SectionExpectation],
}

fn build_section(expect: &SectionExpectation) -> _Section {
    _Section {
        name: expect.name.map(|name| name.to_string()),
        subs: expect.subs.iter().map(|sub| sub.to_string()).collect(),
    }
}

#[test]
fn get_sections_handles_variety_of_inputs() {
    let cases = [
        SectionCase {
            input: &[""],
            expected: &[],
        },
        SectionCase {
            input: &[" "],
            expected: &[],
        },
        SectionCase {
            input: &["\t"],
            expected: &[],
        },
        SectionCase {
            input: &["line 1"],
            expected: &[SectionExpectation {
                name: None,
                subs: &[],
            }],
        },
        SectionCase {
            input: &["line 1", "line 2"],
            expected: &[SectionExpectation {
                name: None,
                subs: &[],
            }],
        },
        SectionCase {
            input: &["line 1", "name_1:"],
            expected: &[SectionExpectation {
                name: None,
                subs: &["name_1"],
            }],
        },
        SectionCase {
            input: &["line 1:"],
            expected: &[SectionExpectation {
                name: None,
                subs: &[],
            }],
        },
        SectionCase {
            input: &["name_1:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &[" name_1:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &["\tname_1:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &["  name_1:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &["name_1: "],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &["name_1: description"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &["name_1:", "description 1"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &["name_1:", "sub_name_1:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1"],
            }],
        },
        SectionCase {
            input: &["name_1:", "sub_name_1 (text 1):"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1"],
            }],
        },
        SectionCase {
            input: &["name_1:", " sub_name_1:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1"],
            }],
        },
        SectionCase {
            input: &["name_1:", "sub_name_1: "],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1"],
            }],
        },
        SectionCase {
            input: &["name_1:", "sub_name_1: description"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1"],
            }],
        },
        SectionCase {
            input: &["name_1:", "sub_name_1:", "description 1"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1"],
            }],
        },
        SectionCase {
            input: &["name_1:", "description 1", "sub_name_1:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1"],
            }],
        },
        SectionCase {
            input: &["name_1:", "sub_name_1:", "sub_name_2:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1", "sub_name_2"],
            }],
        },
        SectionCase {
            input: &["name_1:", "sub_name_1:", "sub_name_2:", "sub_name_3:"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &["sub_name_1", "sub_name_2", "sub_name_3"],
            }],
        },
        SectionCase {
            input: &["name_1:", "description 1", "description 2"],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &["name_1:", ""],
            expected: &[SectionExpectation {
                name: Some("name_1"),
                subs: &[],
            }],
        },
        SectionCase {
            input: &["name_1:", "", "name_2:"],
            expected: &[
                SectionExpectation {
                    name: Some("name_1"),
                    subs: &[],
                },
                SectionExpectation {
                    name: Some("name_2"),
                    subs: &[],
                },
            ],
        },
        SectionCase {
            input: &["# name_1:", "#  ", "# name_2:"],
            expected: &[
                SectionExpectation {
                    name: Some("name_1"),
                    subs: &[],
                },
                SectionExpectation {
                    name: Some("name_2"),
                    subs: &[],
                },
            ],
        },
        SectionCase {
            input: &["# name_1:", " ", "# name_2:"],
            expected: &[
                SectionExpectation {
                    name: Some("name_1"),
                    subs: &[],
                },
                SectionExpectation {
                    name: Some("name_2"),
                    subs: &[],
                },
            ],
        },
        SectionCase {
            input: &["name_1:", "\t", "name_2:"],
            expected: &[
                SectionExpectation {
                    name: Some("name_1"),
                    subs: &[],
                },
                SectionExpectation {
                    name: Some("name_2"),
                    subs: &[],
                },
            ],
        },
        SectionCase {
            input: &["name_1:", "  ", "name_2:"],
            expected: &[
                SectionExpectation {
                    name: Some("name_1"),
                    subs: &[],
                },
                SectionExpectation {
                    name: Some("name_2"),
                    subs: &[],
                },
            ],
        },
        SectionCase {
            input: &["name_1:", "sub_name_1:", "", "name_2:"],
            expected: &[
                SectionExpectation {
                    name: Some("name_1"),
                    subs: &["sub_name_1"],
                },
                SectionExpectation {
                    name: Some("name_2"),
                    subs: &[],
                },
            ],
        },
        SectionCase {
            input: &["name_1:", "", "name_2:", "sub_name_1:"],
            expected: &[
                SectionExpectation {
                    name: Some("name_1"),
                    subs: &[],
                },
                SectionExpectation {
                    name: Some("name_2"),
                    subs: &["sub_name_1"],
                },
            ],
        },
        SectionCase {
            input: &["name_1:", "", "name_2:", "", "name_3:"],
            expected: &[
                SectionExpectation {
                    name: Some("name_1"),
                    subs: &[],
                },
                SectionExpectation {
                    name: Some("name_2"),
                    subs: &[],
                },
                SectionExpectation {
                    name: Some("name_3"),
                    subs: &[],
                },
            ],
        },
    ];

    for case in cases {
        let rendered_input = case
            .input
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let result = _get_sections(rendered_input);

        let expected = case
            .expected
            .iter()
            .map(build_section)
            .collect::<Vec<_Section>>();

        assert_eq!(
            result, expected,
            "Unexpected sections for input {:?}",
            case.input
        );
    }
}

#[derive(Default)]
struct DocstringExpectation {
    args: Option<&'static [&'static str]>,
    args_sections: Option<&'static [&'static str]>,
    attrs: Option<&'static [&'static str]>,
    attrs_sections: Option<&'static [&'static str]>,
    returns_sections: Option<&'static [&'static str]>,
    yields_sections: Option<&'static [&'static str]>,
    raises: Option<&'static [&'static str]>,
    raises_sections: Option<&'static [&'static str]>,
}

struct ParseCase {
    value: &'static str,
    expected: DocstringExpectation,
}

fn to_string_vec(slice_opt: Option<&[&str]>) -> Option<Vec<String>> {
    slice_opt.map(|items| items.iter().map(|item| item.to_string()).collect())
}

fn build_expected_docstring(expect: &DocstringExpectation) -> Docstring {
    Docstring::new(
        to_string_vec(expect.args),
        to_string_vec(expect.args_sections),
        to_string_vec(expect.attrs),
        to_string_vec(expect.attrs_sections),
        to_string_vec(expect.returns_sections),
        to_string_vec(expect.yields_sections),
        to_string_vec(expect.raises),
        to_string_vec(expect.raises_sections),
        TextRange::new(TextSize::new(0), TextSize::new(0)),
    )
}

#[test]
fn parse_extracts_expected_sections() {
    let cases = [
        ParseCase {
            value: "",
            expected: DocstringExpectation::default(),
        },
        ParseCase {
            value: "short description",
            expected: DocstringExpectation::default(),
        },
        ParseCase {
            value: "short description\n\nlong description",
            expected: DocstringExpectation::default(),
        },
        ParseCase {
            value: "short description\n\nArgs:\n    ",
            expected: DocstringExpectation {
                args: Some(&[]),
                args_sections: Some(&["Args"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nArgs:\n    arg_1:\n    ",
            expected: DocstringExpectation {
                args: Some(&["arg_1"]),
                args_sections: Some(&["Args"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\n# Args:\n#     arg_1:\n#     arg_2:\n#     ",
            expected: DocstringExpectation {
                args: Some(&["arg_1", "arg_2"]),
                args_sections: Some(&["Args"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nargs:\n    arg_1:\n    ",
            expected: DocstringExpectation {
                args: Some(&["arg_1"]),
                args_sections: Some(&["args"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nArguments:\n    arg_1:\n    ",
            expected: DocstringExpectation {
                args: Some(&["arg_1"]),
                args_sections: Some(&["Arguments"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nParameters:\n    arg_1:\n    ",
            expected: DocstringExpectation {
                args: Some(&["arg_1"]),
                args_sections: Some(&["Parameters"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nArgs:\n    arg_1:\n\nParameters:\n    arg_2:\n    ",
            expected: DocstringExpectation {
                args: Some(&["arg_1"]),
                args_sections: Some(&["Args", "Parameters"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nAttrs:\n    ",
            expected: DocstringExpectation {
                attrs: Some(&[]),
                attrs_sections: Some(&["Attrs"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nAttrs:\n\nAttributes:\n    ",
            expected: DocstringExpectation {
                attrs: Some(&[]),
                attrs_sections: Some(&["Attrs", "Attributes"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nAttrs:\n\nAttrs:\n    ",
            expected: DocstringExpectation {
                attrs: Some(&[]),
                attrs_sections: Some(&["Attrs", "Attrs"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nAttrs:\n    attr_1:\n    ",
            expected: DocstringExpectation {
                attrs: Some(&["attr_1"]),
                attrs_sections: Some(&["Attrs"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nAttrs:\n    attr_1:\n    attr_2:\n    ",
            expected: DocstringExpectation {
                attrs: Some(&["attr_1", "attr_2"]),
                attrs_sections: Some(&["Attrs"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nattrs:\n    attr_1:\n    ",
            expected: DocstringExpectation {
                attrs: Some(&["attr_1"]),
                attrs_sections: Some(&["attrs"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nAttributes:\n    attr_1:\n    ",
            expected: DocstringExpectation {
                attrs: Some(&["attr_1"]),
                attrs_sections: Some(&["Attributes"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nReturns:\n    ",
            expected: DocstringExpectation {
                returns_sections: Some(&["Returns"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nReturns:\n    The return value.\n    ",
            expected: DocstringExpectation {
                returns_sections: Some(&["Returns"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nReturn:\n    ",
            expected: DocstringExpectation {
                returns_sections: Some(&["Return"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nReturns:\n\nReturns:\n    ",
            expected: DocstringExpectation {
                returns_sections: Some(&["Returns", "Returns"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nReturns:\n\nReturn:\n    ",
            expected: DocstringExpectation {
                returns_sections: Some(&["Returns", "Return"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nYields:\n    ",
            expected: DocstringExpectation {
                yields_sections: Some(&["Yields"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nYields:\n    The return value.\n    ",
            expected: DocstringExpectation {
                yields_sections: Some(&["Yields"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nYield:\n    ",
            expected: DocstringExpectation {
                yields_sections: Some(&["Yield"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nYields:\n\nYields:\n    ",
            expected: DocstringExpectation {
                yields_sections: Some(&["Yields", "Yields"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nYields:\n\nYield:\n    ",
            expected: DocstringExpectation {
                yields_sections: Some(&["Yields", "Yield"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nRaises:\n    ",
            expected: DocstringExpectation {
                raises: Some(&[]),
                raises_sections: Some(&["Raises"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nRaises:\n\nRaises:\n    ",
            expected: DocstringExpectation {
                raises: Some(&[]),
                raises_sections: Some(&["Raises", "Raises"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nRaises:\n\nRaise:\n    ",
            expected: DocstringExpectation {
                raises: Some(&[]),
                raises_sections: Some(&["Raises", "Raise"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nRaises:\n    exc_1:\n    ",
            expected: DocstringExpectation {
                raises: Some(&["exc_1"]),
                raises_sections: Some(&["Raises"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nRaises:\n    exc_1:\n    exc_2:\n    ",
            expected: DocstringExpectation {
                raises: Some(&["exc_1", "exc_2"]),
                raises_sections: Some(&["Raises"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nraises:\n    exc_1:\n    ",
            expected: DocstringExpectation {
                raises: Some(&["exc_1"]),
                raises_sections: Some(&["raises"]),
                ..Default::default()
            },
        },
        ParseCase {
            value: "short description\n\nAttrs:\n    attr_1:\n\nArgs:\n    arg_1:\n\nReturns:\n    The return value.\n\nYields:\n    The yield value.\n\nRaises:\n    exc_1:\n    ",
            expected: DocstringExpectation {
                args: Some(&["arg_1"]),
                args_sections: Some(&["Args"]),
                attrs: Some(&["attr_1"]),
                attrs_sections: Some(&["Attrs"]),
                returns_sections: Some(&["Returns"]),
                yields_sections: Some(&["Yields"]),
                raises: Some(&["exc_1"]),
                raises_sections: Some(&["Raises"]),
                ..Default::default()
            },
        },
    ];

    for case in cases {
        let parsed = parse_from_str_for_tests(case.value);
        let expected = build_expected_docstring(&case.expected);

        assert_eq!(
            parsed, expected,
            "Docstring mismatch for input: {:?}",
            case.value
        );
    }
}

#[test]
fn test_multiline_subsections() {
    let lines = vec![
        "arrange: This is a very important part so".to_string(),
        "    the arrange sentence has to be loooong.".to_string(),
        "act: Do the test.".to_string(),
        "assert: It better not fail.".to_string(),
    ];
    
    let sections = _get_sections(lines);
    
    // Print for debugging
    for section in &sections {
        eprintln!("Section: {:?}", section);
    }
    
    // The first line "arrange: ..." is treated as a section (because it's first line and matches pattern)
    // The continuation line should not create a new subsection
    // "act:" and "assert:" should be subsections under "arrange:"
    assert_eq!(sections.len(), 1, "Expected 1 section"); 
    assert_eq!(sections[0].name, Some("arrange".to_string()), "Expected section name to be 'arrange'");
    assert_eq!(sections[0].subs.len(), 2, "Expected 2 subsections, got: {:?}", sections[0].subs);
    assert!(sections[0].subs.contains(&"act".to_string()));
    assert!(sections[0].subs.contains(&"assert".to_string()));
}

#[test]
fn test_multiline_subsections_in_named_section() {
    // Test when subsections with multi-line descriptions are within a named section
    let lines = vec![
        "Args:".to_string(),
        "    arg1: This is a very long description that needs to".to_string(),
        "        span multiple lines because it's important.".to_string(),
        "    arg2: Short description.".to_string(),
        "    arg3: Another long description that also".to_string(),
        "        needs multiple lines.".to_string(),
    ];
    
    let sections = _get_sections(lines);
    
    eprintln!("Sections: {:?}", sections);
    
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].name, Some("Args".to_string()));
    assert_eq!(sections[0].subs.len(), 3, "Expected 3 args, got: {:?}", sections[0].subs);
    assert!(sections[0].subs.contains(&"arg1".to_string()));
    assert!(sections[0].subs.contains(&"arg2".to_string()));
    assert!(sections[0].subs.contains(&"arg3".to_string()));
}

#[test]
fn test_multiline_subsections_given_when_then() {
    // Test the given/when/then pattern common in tests
    let lines = vec![
        "given: A test setup with multiple components that require".to_string(),
        "    detailed explanation across lines.".to_string(),
        "when: The action is performed.".to_string(),
        "then: The expected outcome should be this specific thing that".to_string(),
        "    also needs a detailed explanation.".to_string(),
    ];
    
    let sections = _get_sections(lines);
    
    eprintln!("Sections: {:?}", sections);
    
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].name, Some("given".to_string()));
    assert_eq!(sections[0].subs.len(), 2, "Expected 2 subsections, got: {:?}", sections[0].subs);
    assert!(sections[0].subs.contains(&"when".to_string()));
    assert!(sections[0].subs.contains(&"then".to_string()));
}

#[test]
fn test_multiline_with_multiple_continuation_lines() {
    // Test with multiple continuation lines for a single subsection
    let lines = vec![
        "Args:".to_string(),
        "    param1: This is line 1".to_string(),
        "        This is line 2 of the same parameter".to_string(),
        "        And this is line 3".to_string(),
        "    param2: Another parameter.".to_string(),
    ];
    
    let sections = _get_sections(lines);
    
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].name, Some("Args".to_string()));
    assert_eq!(sections[0].subs.len(), 2);
    assert!(sections[0].subs.contains(&"param1".to_string()));
    assert!(sections[0].subs.contains(&"param2".to_string()));
}
