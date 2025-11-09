use regex::Regex;
use rustpython_ast::text_size::TextRange;
#[cfg(test)]
use rustpython_ast::text_size::TextSize;
use rustpython_ast::ExprConstant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

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
    static ref _SUB_SECTION_PATTERN: Regex = Regex::new(r"\s*(\w+)( \(.*\))?:").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct _Section {
    name: Option<String>,
    subs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Docstring {
    args: Option<Vec<String>>,
    args_sections: Option<Vec<String>>,
    attrs: Option<Vec<String>>,
    attrs_sections: Option<Vec<String>>,
    returns_sections: Option<Vec<String>>,
    yields_sections: Option<Vec<String>>,
    raises: Option<Vec<String>>,
    raises_sections: Option<Vec<String>>,
    range: TextRange,
}

impl Docstring {
    fn new(
        args: Option<Vec<String>>,
        args_sections: Option<Vec<String>>,
        attrs: Option<Vec<String>>,
        attrs_sections: Option<Vec<String>>,
        returns_sections: Option<Vec<String>>,
        yields_sections: Option<Vec<String>>,
        raises: Option<Vec<String>>,
        raises_sections: Option<Vec<String>>,
        range: TextRange,
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
            range,
        }
    }

    pub fn has_returns(&self) -> bool {
        self.returns_sections
            .as_ref()
            .map(|sections| !sections.is_empty())
            .unwrap_or(false)
    }

    pub fn get_returns(&self) -> Vec<String> {
        self.returns_sections.clone().unwrap_or_default()
    }

    pub fn has_raises_sections(&self) -> bool {
        self.raises_sections
            .as_ref()
            .map(|sections| !sections.is_empty())
            .unwrap_or(false)
    }

    pub fn get_raises(&self) -> Vec<String> {
        self.raises.clone().unwrap_or_default()
    }

    pub fn get_raises_sections(&self) -> Vec<String> {
        self.raises_sections.clone().unwrap_or_default()
    }

    pub fn has_yields(&self) -> bool {
        self.yields_sections
            .as_ref()
            .map(|sections| !sections.is_empty())
            .unwrap_or(false)
    }

    pub fn get_yields(&self) -> Vec<String> {
        self.yields_sections.clone().unwrap_or_default()
    }

    pub fn has_args_sections(&self) -> bool {
        self.args_sections
            .as_ref()
            .map(|sections| !sections.is_empty())
            .unwrap_or(false)
    }

    pub fn get_args_sections(&self) -> Vec<String> {
        self.args_sections.clone().unwrap_or_default()
    }

    pub fn has_args(&self) -> bool {
        self.args
            .as_ref()
            .map(|args| !args.is_empty())
            .unwrap_or(false)
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone().unwrap_or_default()
    }

    pub fn get_range(&self) -> TextRange {
        self.range
    }

    pub fn has_attrs_sections(&self) -> bool {
        self.attrs_sections
            .as_ref()
            .map(|sections| !sections.is_empty())
            .unwrap_or(false)
    }

    pub fn get_attrs(&self) -> Vec<String> {
        self.attrs.clone().unwrap_or_default()
    }

    pub fn get_attrs_sections(&self) -> Vec<String> {
        self.attrs_sections.clone().unwrap_or_default()
    }
}

impl PartialEq for Docstring {
    fn eq(&self, other: &Self) -> bool {
        fn sorted(opt: &Option<Vec<String>>) -> Vec<String> {
            let mut v = opt.clone().unwrap_or_default();
            v.sort();
            v
        }

        sorted(&self.args) == sorted(&other.args)
            && sorted(&self.args_sections) == sorted(&other.args_sections)
            && sorted(&self.attrs) == sorted(&other.attrs)
            && sorted(&self.attrs_sections) == sorted(&other.attrs_sections)
            && sorted(&self.returns_sections) == sorted(&other.returns_sections)
            && sorted(&self.yields_sections) == sorted(&other.yields_sections)
            && sorted(&self.raises) == sorted(&other.raises)
            && sorted(&self.raises_sections) == sorted(&other.raises_sections)
            && self.range == other.range
    }
}

impl Eq for Docstring {}

impl fmt::Display for Docstring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Docstring(\n  args={:?},\n  args_sections={:?},\n  attrs={:?},\n  attrs_sections={:?},\n  returns_sections={:?},\n  yields_sections={:?},\n  raises={:?},\n  raises_sections={:?},\n range={:?})",
            self.args,
            self.args_sections,
            self.attrs,
            self.attrs_sections,
            self.returns_sections,
            self.yields_sections,
            self.raises,
            self.raises_sections,
            self.range,
        )
    }
}

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

    while let Some(line) = lines.find(|l| !l.trim().is_empty()) {
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

        // Extract subsections, handling multi-line descriptions
        let mut subs: Vec<String> = Vec::new();
        for line in section_lines.iter() {
            // Check if this line starts a new subsection
            if let Some(caps) = _SUB_SECTION_PATTERN.captures(line) {
                if let Some(sub_name) = caps.get(1).map(|m| m.as_str().to_string()) {
                    subs.push(sub_name);
                }
            }
            // Otherwise, it's a continuation line for the previous subsection - we just skip it
            // (the description is not stored, only the subsection names)
        }

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
fn _get_all_section_names_by_name<'a>(name: &str, sections: &'a [_Section]) -> Option<Vec<String>> {
    let valid_names = &_SECTION_NAMES[name];

    let all_section_names: Vec<String> = sections
        .iter()
        .filter_map(|section| {
            section.name.as_ref().and_then(|n| {
                let lower = n.to_lowercase();
                if valid_names.contains(lower.as_str()) {
                    Some(n.clone()) // <- Return original casing
                } else {
                    None
                }
            })
        })
        .collect();
    if all_section_names.is_empty() {
        return None;
    }
    Some(all_section_names)
}

pub fn parse(constant_expr: &ExprConstant) -> Docstring {
    let value = constant_expr.clone().value.expect_str();
    let sections = _get_sections(value.lines().map(|line| line.to_string()).collect());
    build_docstring_from_sections(sections, constant_expr.range)
}

fn build_docstring_from_sections(sections: Vec<_Section>, range: TextRange) -> Docstring {
    let args_section = _get_section_by_name("args", &sections);
    let attrs_section = _get_section_by_name("attrs", &sections);
    let raises_section = _get_section_by_name("raises", &sections);

    Docstring::new(
        args_section.map(|s| s.subs.clone()),
        _get_all_section_names_by_name("args", &sections),
        attrs_section.map(|s| s.subs.clone()),
        _get_all_section_names_by_name("attrs", &sections),
        _get_all_section_names_by_name("returns", &sections),
        _get_all_section_names_by_name("yields", &sections),
        raises_section.map(|s| s.subs.clone()),
        _get_all_section_names_by_name("raises", &sections),
        range,
    )
}

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) fn parse_from_str_for_tests(value: &str) -> Docstring {
    let sections = _get_sections(value.lines().map(|line| line.to_string()).collect());
    build_docstring_from_sections(sections, TextRange::new(TextSize::new(0), TextSize::new(0)))
}
