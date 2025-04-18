use pyo3::{prelude::*, wrap_pymodule};
use std::env::args;
use std::iter::Zip;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref SECTION_NAMES: HashMap<&'static str, HashSet<&'static str>> = {
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
    static ref SECTION_NAME_PATTERN: regex::Regex = regex::Regex::new(r"\s*(\w+):").unwrap();
}
lazy_static::lazy_static! {
    static ref _SUB_SECTION_PATTERN : regex::Regex = regex::Regex::new(r"\s*(\w+)( \(.*\))?:").unwrap();
}
lazy_static::lazy_static! {
    static ref _SECTION_END_PATTERN : regex::Regex = regex::Regex::new(r"\s*$").unwrap();
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
        _Section { name, subs: subsections }
    }

    fn __eq__(&self, other: &Self) -> PyResult<bool> {
        Ok(self.name == other.name && self.subs == other.subs)
    }
    
}


#[pyclass]
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
        Ok(self.args == other.args)
    }
}

#[pyfunction]
pub fn parse(value: String) -> Docstring {
    return Docstring::new(None, None, None, None, None, None, None, None);
}

#[pyfunction]
pub fn _get_sections(lines: Vec<String>) -> Vec<_Section> {
    let mut sections: Vec<_Section> = Vec::new();
    let mut current_section: Option<_Section> = None;

    for line in lines {
        let mmm = SECTION_NAME_PATTERN.is_match(&line);
        println!("capture: {:?}", mmm.to_string());
        let capt = SECTION_NAME_PATTERN.captures(&line);
        println!("capture: {:?}", capt);
        println!("line: {:?}", line);
        if SECTION_NAME_PATTERN.is_match(&line) {
        let capt = SECTION_NAME_PATTERN.captures(&line).unwrap();
        let subsections: Vec<String> = Vec::new();
        sections.push(_Section { name: None, subs: subsections});
        }
    }

    if let Some(section) = current_section.take() {
        sections.push(section);
    }

    sections
}

#[pymodule]
fn docstring(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_get_sections, m)?)?;
    m.add_class::<Docstring>()?;
    Ok(())
}

////////// Tests

struct TestInput {
    input: &'static str,
    expected: Option<Vec<_Section>>,
}

#[test]
pub fn test_get_sections() {
    let test_inputs = [
        TestInput {
            input: "",
            expected: None,
        },
        TestInput {
            input: " ",
            expected: None,
        },
        TestInput {
            input: "\t",
            expected: None,
        },
        TestInput {
            input: "line 1",
            expected: Some(vec![_Section {
                name: None,
                subs: vec![],
            }]),
        },
    ];

    for input in test_inputs.iter() {
        let returned_sections = _get_sections(vec![input.input.to_string()]);
        match (&returned_sections.is_empty(), &input.expected) {
            (true, None) => {
                // Both are empty, test passes
                println!("Test passed for input: {:?}", input.input);
            }
            (false, Some(expected_sections)) => {
                // Compare the vectors element by element
                for (returned, expected) in returned_sections.iter().zip(expected_sections.iter()) {
                    println!("Comparing: {:?} with {:?}", returned, expected);
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
