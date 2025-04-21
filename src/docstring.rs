use pyo3::{prelude::*, wrap_pymodule};
use std::array::IntoIter;
use std::env::args;
use std::iter::Zip;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref _SECTION_NAMES: HashMap<&'static str, HashSet<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("args", HashSet::from(["args", "arguments", "parameters"]));
        map.insert("attrs", HashSet::from(["attributes", "attrs"]));
        map.insert("returns", HashSet::from(["return", "returns"]));
        map.insert("yields", HashSet::from(["yield", "yields"]));
        map.insert("raises", HashSet::from(["raises", "raise"]));
        map
    };

}

lazy_static::lazy_static! {
    static ref SECTION_NAME_PATTERN: Regex = Regex::new(r"^\s*(\w+):").unwrap();
}

lazy_static::lazy_static! {
    static ref _SUB_SECTION_PATTERN : regex::Regex = regex::Regex::new(r"\s*(\w+)( \(.*\))?:").unwrap();
}

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct _Section {
    name: Option<String>,
    subs: Vec<String>,
}

#[pymethods]
impl _Section {
    #[new]
    #[pyo3(signature = (name=None, subs=None))]
    fn new(name: Option<String>, subs: Option<Vec<String>>) -> Self {
        let subsections = subs.unwrap_or_else(|| vec![]);
        _Section {
            name,
            subs: subsections,
        }
    }
    fn __eq__(&self, other: &Self) -> PyResult<bool> {
        let mut self_subs = self.subs.clone();
        let mut other_subs = other.subs.clone();
        self_subs.sort();
        other_subs.sort();

        Ok(self.name == other.name && self_subs == other_subs)
    }

    fn __repr__(&self) -> String {
        let name_str = match &self.name {
            Some(name) => format!("\"{}\"", name),
            None => "None".to_string(),
        };
        let subs_str = format!("{:?}", self.subs);
        format!("_Section(name={}, subs={})", name_str, subs_str)
    }
}

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct Docstring {
    args: Option<Vec<String>>,
    args_sections: Option<Vec<String>>,
    attrs: Option<Vec<String>>,
    attrs_sections: Option<Vec<String>>,
    returns_sections: Option<Vec<String>>,
    yields_sections: Option<Vec<String>>,
    raises: Option<Vec<String>>,
    raises_sections: Option<Vec<String>>,
}

#[pymethods]
impl Docstring {
    #[new]
    #[pyo3(signature = (
    args=None,
    args_sections=None,
    attrs=None,
    attrs_sections=None,
    returns_sections=None,
    yields_sections=None,
    raises=None,
    raises_sections=None
    ))]
    fn new(
        args: Option<Vec<String>>,
        args_sections: Option<Vec<String>>,
        attrs: Option<Vec<String>>,
        attrs_sections: Option<Vec<String>>,
        returns_sections: Option<Vec<String>>,
        yields_sections: Option<Vec<String>>,
        raises: Option<Vec<String>>,
        raises_sections: Option<Vec<String>>,
    ) -> Self {
        Docstring {
            args,
            args_sections,
            attrs,
            attrs_sections,
            returns_sections,
            yields_sections,
            raises,
            raises_sections,
        }
    }

    fn __eq__(&self, other: &Docstring) -> PyResult<bool> {
        fn sorted(opt: &Option<Vec<String>>) -> Vec<String> {
            let mut v = opt.clone().unwrap_or_default();
            v.sort();
            v
        }

        Ok(sorted(&self.args) == sorted(&other.args)
            && sorted(&self.args_sections) == sorted(&other.args_sections)
            && sorted(&self.attrs) == sorted(&other.attrs)
            && sorted(&self.attrs_sections) == sorted(&other.attrs_sections)
            && sorted(&self.returns_sections) == sorted(&other.returns_sections)
            && sorted(&self.yields_sections) == sorted(&other.yields_sections)
            && sorted(&self.raises) == sorted(&other.raises)
            && sorted(&self.raises_sections) == sorted(&other.raises_sections))
    }
    fn __repr__(&self) -> String {
        format!(
        "Docstring(\n  args={:?},\n  args_sections={:?},\n  attrs={:?},\n  attrs_sections={:?},\n  returns_sections={:?},\n  yields_sections={:?},\n  raises={:?},\n  raises_sections={:?}\n)",
        self.args,
        self.args_sections,
        self.attrs,
        self.attrs_sections,
        self.returns_sections,
        self.yields_sections,
        self.raises,
        self.raises_sections,
    )
    }
}

#[pyfunction]
pub fn _get_sections(lines: Vec<String>) -> Vec<_Section> {
    let cleaned_lines: Vec<String> = lines
        .into_iter()
        .map(|line| {
            if line.trim_start().starts_with("# ") {
                line.trim_start()[2..].to_string()
            } else {
                line
            }
        })
        .collect();

    let mut sections: Vec<_Section> = Vec::new();
    let mut lines = cleaned_lines.into_iter().peekable();


    while let Some(mut line) = lines.find(|l| !l.trim().is_empty()) {
        // Check if it's a section name
        let section_name = SECTION_NAME_PATTERN
            .captures(&line)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

        let mut section_lines: Vec<String> = Vec::new();


        // Keep collecting lines until we hit a blank line or EOF
        while let Some(peek) = lines.peek() {
            if peek.trim().is_empty() {
                // consume the empty line
                lines.next();
                break;
            }
            section_lines.push(lines.next().unwrap());
        }

        let subs = section_lines
            .iter()
            .filter_map(|line| {
                _SUB_SECTION_PATTERN
                    .captures(line)
                    .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            })
            .collect();

        sections.push(_Section {
            name: section_name,
            subs,
        });
    }

    sections
}

fn _get_section_by_name<'a>(name: &str, sections: &'a [_Section]) -> Option<&'a _Section> {
    let valid_names = &_SECTION_NAMES[name];

    sections.iter().find(|section| {
        section
            .name
            .as_ref()
            .map(|n| valid_names.contains(n.to_lowercase().as_str()))
            .unwrap_or(false)
    })
}
fn _get_all_section_names_by_name<'a>(name: &str, sections: &'a [_Section]) -> Vec<String> {
    let valid_names = &_SECTION_NAMES[name];

    sections
        .iter()
        .filter_map(|section| {
            section.name.as_ref().and_then(|n| {
                let lower = n.to_lowercase();
                if valid_names.contains(lower.as_str()) {
                    Some(n.clone())  // <- Return original casing
                } else {
                    None
                }
            })
        })
        .collect()
}


#[pyfunction]
pub fn parse(value: String) -> Docstring {
    let sections = _get_sections(value.lines().map(|line| line.to_string()).collect());

    let args_section = _get_section_by_name("args", &sections);
    let attrs_section = _get_section_by_name("attrs", &sections);
    let raises_section = _get_section_by_name("raises", &sections);

    return Docstring::new(
        args_section.map(|s| s.subs.clone()),
        Some(_get_all_section_names_by_name("args", &sections)),
        attrs_section.map(|s| s.subs.clone()),
        Some(_get_all_section_names_by_name("attrs", &sections)),
        Some(_get_all_section_names_by_name("returns", &sections)),
        Some(_get_all_section_names_by_name("yields", &sections)),
        raises_section.map(|s| s.subs.clone()),
        Some(_get_all_section_names_by_name("raises", &sections)),
    );
}

fn empty_docstring() -> Docstring {
    return Docstring::new(None, None, None, None, None, None, None, None);
}
////////// Tests

struct TestInput {
    input: Vec<String>,
    expected: Option<Vec<_Section>>,
}

struct TestParseInput {
    input: String,
    expected: Docstring,
}

#[test]
pub fn test_parse() {
    let test_inputs = [
        TestParseInput {
            input: "".to_string(),
            expected: empty_docstring(),
        },
        TestParseInput {
            input: "short description

# Args:
#     arg_1:
#     arg_2:
#     "
                .to_string(),
            expected: Docstring {
                args: Some(vec!["arg_1".to_string(), "arg_2".to_string()]),
                args_sections: Some(vec!["Args".to_string()]),
                attrs: None,
                attrs_sections: None,
                returns_sections: None,
                yields_sections: None,
                raises: None,
                raises_sections: None,
            },
        },
    ];

    for input in test_inputs.iter() {
        let returned_docstring = parse(input.input.clone());

        println!("Input: {:?}", input.input);
        println!(
            "Returned: {:?}\nExpected: {:?}\n\n\n",
            returned_docstring, input.expected
        );

        assert_eq!(returned_docstring, input.expected);
    }
}

#[test]
pub fn test_get_sections() {
    let test_inputs = [
        TestInput {
            input: vec!["".to_string()],
            expected: None,
        },
        TestInput {
            input: vec![" ".to_string()],
            expected: None,
        },
        TestInput {
            input: vec!["\t".to_string()],
            expected: None,
        },
        TestInput {
            input: vec!["line 1".to_string()],
            expected: Some(vec![_Section {
                name: None,
                subs: vec![],
            }]),
        },
        TestInput {
            input: vec!["line 1".to_string(), "line 2".to_string()],
            expected: Some(vec![_Section {
                name: None,
                subs: vec![],
            }]),
        },
        TestInput {
            input: vec!["line 1".to_string(), "name_1:".to_string()],
            expected: Some(vec![_Section {
                name: None,
                subs: vec!["name_1".to_string()],
            }]),
        },
        TestInput {
            input: vec!["line 1:".to_string()],
            expected: Some(vec![_Section {
                name: None,
                subs: vec![],
            }]),
        },
        TestInput {
            input: vec!["name_1:".to_string()],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestInput {
            input: vec![" name_1:".to_string()],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestInput {
            input: vec!["\tname_1:".to_string()],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestInput {
            input: vec!["  name_1:".to_string()],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestInput {
            input: vec!["name_1: ".to_string()],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
        TestInput {
            input: vec!["name_1: description".to_string()],
            expected: Some(vec![_Section {
                name: Some("name_1".to_string()),
                subs: vec![],
            }]),
        },
    ];

    for input in test_inputs.iter() {
        let returned_sections = _get_sections(input.input.clone());

        println!("Line: {:?}", input.input);
        println!(
            "||| Comparing: {:?} with {:?}",
            returned_sections, input.expected
        );

        match (&returned_sections.is_empty(), &input.expected) {
            (true, None) => {
                // Both are empty, test passes
                println!("Test passed for input: {:?}\n\n\n", input.input);
            }
            (false, Some(expected_sections)) => {
                // compare the element size
                assert_eq!(
                    returned_sections.len(),
                    expected_sections.len(),
                    "Length mismatch for input: {:?}. Returned: {:?}, Expected: {:?}\n\n\n",
                    input.input,
                    returned_sections,
                    expected_sections
                );

                // Compare the vectors element by element
                for (returned, expected) in returned_sections.iter().zip(expected_sections.iter()) {
                    println!("Returned: {:?}\nExpected: {:?}\n\n\n", returned, expected);
                    assert_eq!(returned, expected);
                }
            }
            _ => {
                // Mismatch between returned and expected
                panic!(
                    "Test failed for input: {:?}. Returned: {:?}, Expected: {:?}",
                    input.input, returned_sections, input.expected
                );
            }
        }
    }
}
