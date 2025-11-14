use crate::constants::{
    arg_in_docstr_msg, arg_not_in_docstr_msg, args_section_in_docstr_msg,
    args_section_not_in_docstr_msg, attr_in_docstr_msg, attr_not_in_docstr_msg,
    attrs_section_in_docstr_msg, attrs_section_not_in_docstr_msg, docstr_missing_msg,
    duplicate_arg_msg, duplicate_attr_docstr_msg, duplicate_exc_msg, exc_in_docstr_msg,
    exc_not_in_docstr_msg, mult_args_sections_in_docstr_msg, mult_attrs_section_in_docstr_msg,
    mult_raises_sections_in_docstr_msg, mult_returns_sections_in_docstr_msg,
    mult_yields_sections_in_docstr_msg, raises_section_in_docstr_msg,
    raises_section_not_in_docstr_msg, re_raise_no_exc_in_docstr_msg, returns_section_in_docstr_msg,
    returns_section_not_in_docstr_msg, yields_section_in_docstr_msg,
    yields_section_not_in_docstr_msg,
};
use crate::plugin::{
    get_result, ClassInfo, DocstringCollector, FunctionDefKind, FunctionInfo, YieldKind,
};
use regex::Regex;
use rustpython_ast::text_size::TextRange;
use rustpython_ast::{Arguments, Expr, ExprAttribute, ExprCall, StmtRaise, StmtReturn};
use rustpython_parser::text_size::TextSize;
use std::collections::{HashMap, HashSet};
use std::fs;

fn read_file(file_name: &str) -> String {
    // Read the file and return the contents
    fs::read_to_string(file_name).unwrap_or_default()
}

fn is_test_file(file_name: Option<&str>) -> bool {
    if file_name.is_some() {
        let file_name = file_name.unwrap().split('/').next_back().unwrap();

        if file_name.starts_with("test_") || file_name.starts_with("conftest.py") {
            return true;
        }
    }
    false
}

lazy_static::lazy_static! {
    static ref SUPPRESSION_REGEX: Regex = Regex::new(
        r"(?i)^\s*vipyrdocs:\s*(disable(?:-next-docstring)?|disable-file)\s*=\s*(?P<codes>.+?)\s*$",
    )
    .unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectiveType {
    Disable,
    DisableNextDocstring,
    DisableFile,
}

#[derive(Default)]
struct DirectiveParseResult {
    line_suppressions: HashMap<usize, HashSet<String>>,
    line_disable_directives: HashMap<usize, HashSet<String>>,
    next_docstring_comments: Vec<(usize, HashSet<String>)>,
    leading_disable_directives: HashMap<usize, HashSet<String>>,
    file_suppressions: HashSet<String>,
}

#[derive(Debug, Clone)]
struct BlockSuppression {
    start_line: usize,
    end_line: usize,
    codes: HashSet<String>,
}

impl BlockSuppression {
    fn contains(&self, line: usize) -> bool {
        line >= self.start_line && line <= self.end_line
    }

    fn matches_code(&self, code: &str) -> bool {
        self.codes.contains(code) || self.codes.contains("ALL")
    }
}

#[derive(Clone, Copy)]
enum DocTargetKind {
    Function,
    Class,
}

struct DocTarget {
    docstring_line: usize,
    start_line: usize,
    end_line: usize,
    kind: DocTargetKind,
    consumed: bool,
}

struct SuppressionIndex {
    line_suppressions: HashMap<usize, HashSet<String>>,
    file_suppressions: HashSet<String>,
    function_blocks: Vec<BlockSuppression>,
    class_blocks: Vec<BlockSuppression>,
}

impl SuppressionIndex {
    fn new(file_contents: &str, collector: &DocstringCollector) -> Self {
        let parse_result = parse_suppression_directives(file_contents);
        let mut function_blocks: Vec<BlockSuppression> = Vec::new();
        let mut class_blocks: Vec<BlockSuppression> = Vec::new();
        let mut doc_targets: Vec<DocTarget> = Vec::new();

        let add_block = |blocks: &mut Vec<BlockSuppression>,
                         start_line: usize,
                         end_line: usize,
                         codes: &HashSet<String>| {
            if start_line == 0 || end_line == 0 {
                return;
            }
            if let Some(existing) = blocks
                .iter_mut()
                .find(|block| block.start_line == start_line && block.end_line == end_line)
            {
                existing.codes.extend(codes.clone());
            } else {
                blocks.push(BlockSuppression {
                    start_line,
                    end_line,
                    codes: codes.clone(),
                });
            }
        };

        for function in &collector.function_infos {
            let (start_line, end_line) = range_to_lines(function.def.range(), file_contents);
            if let Some(codes) = parse_result.line_disable_directives.get(&start_line) {
                add_block(&mut function_blocks, start_line, end_line, codes);
            }

            if start_line > 0 {
                let prev_line = start_line - 1;
                if let Some(codes) = parse_result.leading_disable_directives.get(&prev_line) {
                    add_block(&mut function_blocks, start_line, end_line, codes);
                }
            }

            if let Some(docstring) = function.docstring.as_ref() {
                let (doc_start, _) = range_to_lines(&docstring.get_range(), file_contents);
                if doc_start != 0 {
                    doc_targets.push(DocTarget {
                        docstring_line: doc_start,
                        start_line,
                        end_line,
                        kind: DocTargetKind::Function,
                        consumed: false,
                    });
                }
            }
        }

        for class in &collector.class_infos {
            let (class_start, class_end) = range_to_lines(&class.def.range, file_contents);
            if let Some(codes) = parse_result.line_disable_directives.get(&class_start) {
                add_block(&mut class_blocks, class_start, class_end, codes);
            }

            if class_start > 0 {
                let prev_line = class_start - 1;
                if let Some(codes) = parse_result.leading_disable_directives.get(&prev_line) {
                    add_block(&mut class_blocks, class_start, class_end, codes);
                }
            }

            if let Some(docstring) = class.docstring.as_ref() {
                let (doc_start, _) = range_to_lines(&docstring.get_range(), file_contents);
                if doc_start != 0 {
                    doc_targets.push(DocTarget {
                        docstring_line: doc_start,
                        start_line: class_start,
                        end_line: class_end,
                        kind: DocTargetKind::Class,
                        consumed: false,
                    });
                }
            }

            for method in &class.funcs {
                let (start_line, end_line) = range_to_lines(method.def.range(), file_contents);
                if let Some(codes) = parse_result.line_disable_directives.get(&start_line) {
                    add_block(&mut function_blocks, start_line, end_line, codes);
                }

                if start_line > 0 {
                    let prev_line = start_line - 1;
                    if let Some(codes) = parse_result.leading_disable_directives.get(&prev_line) {
                        add_block(&mut function_blocks, start_line, end_line, codes);
                    }
                }

                if let Some(docstring) = method.docstring.as_ref() {
                    let (doc_start, _) = range_to_lines(&docstring.get_range(), file_contents);
                    if doc_start != 0 {
                        doc_targets.push(DocTarget {
                            docstring_line: doc_start,
                            start_line,
                            end_line,
                            kind: DocTargetKind::Function,
                            consumed: false,
                        });
                    }
                }
            }
        }

        doc_targets.sort_by_key(|target| target.docstring_line);

        let mut comment_entries = parse_result.next_docstring_comments.clone();
        comment_entries.sort_by_key(|(line, _)| *line);

        for (comment_line, codes) in comment_entries {
            if codes.is_empty() {
                continue;
            }
            if let Some(target) = doc_targets
                .iter_mut()
                .find(|target| !target.consumed && target.docstring_line > comment_line)
            {
                target.consumed = true;
                match target.kind {
                    DocTargetKind::Function => {
                        add_block(
                            &mut function_blocks,
                            target.start_line,
                            target.end_line,
                            &codes,
                        );
                    }
                    DocTargetKind::Class => {
                        add_block(
                            &mut class_blocks,
                            target.start_line,
                            target.end_line,
                            &codes,
                        );
                    }
                }
            }
        }

        Self {
            line_suppressions: parse_result.line_suppressions,
            file_suppressions: parse_result.file_suppressions,
            function_blocks,
            class_blocks,
        }
    }

    fn is_suppressed_entry(&self, entry: &str) -> bool {
        if let Some((line, _, message)) = parse_entry(entry) {
            if let Some(code) = extract_code(&message) {
                return self.is_suppressed(line, &code);
            }
        }
        false
    }

    fn is_suppressed(&self, line: usize, code: &str) -> bool {
        let code_upper = code.to_ascii_uppercase();

        if self.file_suppressions.contains("ALL") || self.file_suppressions.contains(&code_upper) {
            return true;
        }

        if let Some(codes) = self.line_suppressions.get(&line) {
            if codes.contains("ALL") || codes.contains(&code_upper) {
                return true;
            }
        }

        for block in &self.function_blocks {
            if block.contains(line) && block.matches_code(&code_upper) {
                return true;
            }
        }

        for block in &self.class_blocks {
            if block.contains(line) && block.matches_code(&code_upper) {
                return true;
            }
        }

        false
    }
}

fn parse_suppression_directives(file_contents: &str) -> DirectiveParseResult {
    let mut result = DirectiveParseResult::default();
    for (idx, line) in file_contents.lines().enumerate() {
        let line_number = idx + 1;
        if let Some(hash_index) = line.find('#') {
            let comment = &line[hash_index + 1..];
            if let Some((directive_type, codes)) = parse_comment_directive(comment) {
                match directive_type {
                    DirectiveType::Disable => {
                        if !codes.is_empty() {
                            result
                                .line_suppressions
                                .entry(line_number)
                                .or_default()
                                .extend(codes.clone());

                            if line[..hash_index].trim().is_empty() {
                                result
                                    .leading_disable_directives
                                    .entry(line_number)
                                    .or_default()
                                    .extend(codes.clone());
                            }

                            result
                                .line_disable_directives
                                .entry(line_number)
                                .or_default()
                                .extend(codes);
                        }
                    }
                    DirectiveType::DisableNextDocstring => {
                        if !codes.is_empty() {
                            result.next_docstring_comments.push((line_number, codes));
                        }
                    }
                    DirectiveType::DisableFile => {
                        result.file_suppressions.extend(codes);
                    }
                }
            }
        }
    }

    result
}

fn parse_comment_directive(comment: &str) -> Option<(DirectiveType, HashSet<String>)> {
    let trimmed = comment.trim();
    let captures = SUPPRESSION_REGEX.captures(trimmed)?;
    let keyword = captures.get(1)?.as_str().to_ascii_lowercase();
    let codes_raw = captures.name("codes")?.as_str();
    let codes = parse_codes(codes_raw);
    if codes.is_empty() {
        return None;
    }
    let directive_type = match keyword.as_str() {
        "disable" => DirectiveType::Disable,
        "disable-next-docstring" => DirectiveType::DisableNextDocstring,
        "disable-file" => DirectiveType::DisableFile,
        _ => return None,
    };
    Some((directive_type, codes))
}

fn parse_codes(raw_codes: &str) -> HashSet<String> {
    raw_codes
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter_map(|token| {
            let token = token.trim();
            if token.is_empty() {
                return None;
            }
            let upper = token.to_ascii_uppercase();
            if upper == "ALL" || upper.starts_with('D') {
                Some(upper)
            } else {
                None
            }
        })
        .collect()
}

fn parse_entry(entry: &str) -> Option<(usize, usize, String)> {
    let (line_part, rest) = entry.split_once(':')?;
    let line: usize = line_part.trim().parse().ok()?;
    let rest = rest.trim_start();
    let (column_part, message) = rest.split_once(' ')?;
    let column: usize = column_part.trim().parse().ok()?;
    Some((line, column, message.trim().to_string()))
}

fn extract_code(message: &str) -> Option<String> {
    message
        .split_whitespace()
        .next()
        .map(|token| token.trim().to_ascii_uppercase())
}

fn range_to_lines(range: &TextRange, file_contents: &str) -> (usize, usize) {
    let start_offset = range.start().to_usize();
    let end_offset = range.end().to_usize();
    let start_line = find_line_and_column(file_contents, start_offset)
        .map(|(line, _)| line)
        .unwrap_or(0);
    let end_index = if end_offset == 0 { 0 } else { end_offset - 1 };
    let end_line = find_line_and_column(file_contents, end_index)
        .map(|(line, _)| line)
        .unwrap_or(start_line);
    (start_line, end_line)
}

pub fn lint_file_with_inheritance(
    code: &str,
    file_name: Option<&str>,
    implementing_methods: Option<&std::collections::HashSet<(String, String, String)>>,
) -> Vec<String> {
    // Make a mutable String to hold the actual code
    let mut code = code.to_string();

    // If there's a file, override it
    if let Some(file) = file_name {
        code = read_file(file); // assuming this returns String
    }

    apply_rules_with_inheritance(code.as_str(), file_name, implementing_methods)
}

pub fn apply_rules_with_inheritance(
    code: &str,
    file_name: Option<&str>,
    implementing_methods: Option<&std::collections::HashSet<(String, String, String)>>,
) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    let things = get_result(code, file_name);

    let test_file = is_test_file(file_name);

    output.extend(generate_rules_output_with_inheritance(
        code,
        &things,
        test_file,
        file_name,
        implementing_methods,
    ));

    // apply the rules
    output
}

/// Finds the (line, column) of `target_string` if it exists within the specified TextRange of `s`.
/// Returns (line_number, column_number) on success. Both are 1-based.
pub fn find_string_in_text_range(
    s: &str,
    range: &TextRange,
    target_strings: Vec<&str>,
) -> Vec<(usize, usize, String)> {
    let start = usize::try_from(range.start().to_u32()).unwrap();
    let end = usize::try_from(range.end().to_u32()).unwrap();

    // Ensure we're working with valid UTF-8 boundaries
    if start >= s.len() || end > s.len() || start > end {
        return Vec::new();
    }

    // Get the substring safely - if boundaries are invalid, return empty
    let sub_str = match s.get(start..end) {
        Some(sub) => sub,
        None => return Vec::new(),
    };

    let sub = sub_str.to_lowercase();
    let mut positions: Vec<(usize, usize, String)> = Vec::new();
    let target_strings_lower: Vec<String> =
        target_strings.iter().map(|t| t.to_lowercase()).collect();

    let mut offset = 0;
    while offset < sub.len() {
        let mut matched = false;
        for (i, target) in target_strings_lower.iter().enumerate() {
            // Use get() for safe slicing
            if let Some(sub_slice) = sub.get(offset..) {
                if sub_slice.starts_with(target.as_str()) {
                    let absolute_pos = start + offset;

                    // Safely slice to get the "before" part
                    if let Some(before) = s.get(..absolute_pos) {
                        let line_number = before.lines().count(); // 1-based

                        let column_number = before
                            .rfind('\n')
                            .map(|idx| absolute_pos.saturating_sub(idx + 1))
                            .unwrap_or(absolute_pos);

                        positions.push((
                            line_number.saturating_sub(2),
                            column_number,
                            target_strings[i].to_string(),
                        ));
                    }
                    offset += target.len();
                    matched = true;
                    break; // only take the first match at this position
                }
            }
        }
        if !matched {
            offset += 1;
        }
    }

    if positions.is_empty() {
        // Safely slice to get the "before" part
        if let Some(before) = s.get(..start) {
            let line_number = before.lines().count(); // 1-based

            let column_number = before
                .rfind('\n')
                .map(|idx| start.saturating_sub(idx + 1))
                .unwrap_or(start);

            positions.push((line_number.saturating_sub(2), column_number, "".to_string()));
        }
    }

    positions
}

fn find_line_and_column(s: &str, char_index: usize) -> Option<(usize, usize)> {
    let mut current_char_index = 0;

    for (line_number, line) in s.lines().enumerate() {
        let line_char_count = line.chars().count();
        let next_char_index = current_char_index + line_char_count;

        if char_index < next_char_index {
            let column = char_index.saturating_sub(current_char_index);
            return Some((line_number + 1, column)); // Lines are 1-based, columns 0-based
        }

        // +1 to account for the newline character (if there was one)
        current_char_index = next_char_index + 1;
    }

    None
}

fn format_problem(line: usize, line_location: usize, error_msg: String) -> String {
    format!("{}:{} {}", line, line_location, error_msg)
}
fn check_functions_for_duplicate_arg_in_args_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        let args = function.def.args();
        let clean_args = cleanse_args(args, false);
        // ignore if function doesn't have args
        if is_args_empty(&clean_args) {
            continue;
        }

        let docstring_args_sections = function.docstring.clone().unwrap().get_args_sections();
        let docstring_args = function.docstring.clone().unwrap().get_args();

        if docstring_args_sections.is_empty() {
            continue;
        }
        let mut counts = HashMap::new();

        let mut _range = function
            .docstring
            .clone()
            .expect("This should not happen")
            .get_range();
        for arg_name in docstring_args {
            let counter = counts.entry(arg_name.clone()).or_insert(0);
            *counter += 1;
            if *counter == 2 {
                let args_lines = find_string_in_text_range(
                    file_contents,
                    &_range,
                    vec!["args", "arguments", "parameters"],
                );
                if let Some((line, line_location, _)) = args_lines.first() {
                    problem_functions.push(format_problem(
                        *line,
                        *line_location,
                        duplicate_arg_msg(arg_name.as_str()),
                    ));
                } else {
                    eprintln!(
                        "Warning: Could not find line information for duplicate arg at position {}",
                        _range.start().to_usize()
                    );
                }
            }
        }
    }

    problem_functions
}

fn check_functions_for_extra_arg_in_args_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        let args = function.def.args();
        let clean_args = cleanse_args(args, true);
        // ignore if function doesn't have args
        if is_args_empty(&clean_args) {
            continue;
        }

        let docstring_args_sections = function.docstring.clone().unwrap().get_args_sections();
        let docstring_args = function.docstring.clone().unwrap().get_args();

        if docstring_args_sections.is_empty() {
            continue;
        }
        let mut _range = function.def.range();
        // if DC022 is here we don't need to check for DC023
        if function
            .docstring
            .clone()
            .unwrap()
            .get_args_sections()
            .len()
            > 1
        {
            continue;
        }

        let mut arg_names: Vec<String> = Vec::new();
        if clean_args.vararg.is_some() {
            arg_names.push(clean_args.vararg.unwrap().arg.to_string());
            // if let Some(_result) =
            //     is_arg_in_docstring(arg_name, &docstring_args, _range, file_contents)
            // {
            //     problem_functions.push(_result);
            // }
        }
        if clean_args.kwarg.is_some() {
            arg_names.push(clean_args.kwarg.unwrap().arg.to_string());
        }
        for arg in clean_args.args {
            arg_names.push(arg.def.arg.to_string());
        }
        for arg in clean_args.kwonlyargs {
            arg_names.push(arg.def.arg.to_string());
        }
        for arg in clean_args.posonlyargs {
            arg_names.push(arg.def.arg.to_string());
        }
        for arg_name in docstring_args {
            if !arg_names.contains(&arg_name) {
                let args_lines =
                    find_string_in_text_range(file_contents, _range, vec![arg_name.as_str()]);
                if let Some((line, line_location, _)) = args_lines.first() {
                    problem_functions.push(format_problem(
                        line + 2,
                        *line_location,
                        arg_in_docstr_msg(arg_name.as_str()),
                    ));
                } else {
                    eprintln!(
                        "Warning: Could not find line information for arg '{}' at position {}",
                        arg_name,
                        _range.start().to_usize()
                    );
                }
            }
        }
    }

    problem_functions
}
fn check_classes_for_attrs_section_not_in_docstr(
    class_info: &ClassInfo,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut errors = Vec::new();

    // Skip if this is a test file (similar pattern to other rules)
    if is_test_file {
        return errors;
    }

    if class_info.docstring.is_none() {
        return errors;
    }

    let public_class_attributes: Vec<String> = class_info
        .attributes
        .iter()
        .filter(|attr| !attr.starts_with('_'))
        .cloned()
        .collect();

    if public_class_attributes.is_empty() {
        return errors;
    }
    if let Some(docstring) = &class_info.docstring {
        // Check if docstring has attrs sections
        if !docstring.has_attrs_sections() {
            let attr_name = public_class_attributes
                .first()
                .cloned()
                .unwrap_or_else(|| class_info.attributes.first().cloned().unwrap_or_default());
            let exc_lines = find_string_in_text_range(
                file_contents,
                &TextRange::new(TextSize::new(0), class_info.def.range.end()),
                vec![attr_name.as_str()],
            );
            if let Some((line, line_location, _)) = exc_lines.first() {
                errors.push(format_problem(
                    *line,
                    *line_location,
                    attrs_section_not_in_docstr_msg(),
                ));
            } else {
                eprintln!(
                    "Warning: Could not find line information for attribute '{}' in class",
                    attr_name
                );
            }
        }
    }

    errors
}
fn check_classes_for_extra_attrs_section_in_docstr(
    class_info: &ClassInfo,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut errors = Vec::new();

    // Skip if this is a test file (similar pattern to other rules)
    if is_test_file {
        return errors;
    }

    if class_info.docstring.is_none() {
        return errors;
    }

    if let Some(docstring) = &class_info.docstring {
        // Check if docstring has attrs sections
        if !docstring.has_attrs_sections() {
            return errors;
        }

        let public_class_attributes: Vec<String> = class_info
            .attributes
            .clone()
            .into_iter()
            .filter(|attr| !attr.starts_with('_'))
            .collect();
        let public_instance_attributes: Vec<String> = class_info
            .instance_attributes
            .clone()
            .into_iter()
            .filter(|attr| !attr.starts_with('_'))
            .collect();

        // Rule 61: If there's an attrs section but no public attributes, it's extra
        // However, if private attributes are documented in the attrs section,
        // then the attrs section is justified
        if public_class_attributes.is_empty() && public_instance_attributes.is_empty() {
            let docstr_attrs = docstring.get_attrs();

            // If the docstring documents a private attribute that's actually present,
            // treat it as a valid attribute definition so the attrs section is justified.
            if docstr_attrs.iter().any(|attr| attr.starts_with('_'))
                && (class_info
                    .attributes
                    .iter()
                    .any(|attr| attr.starts_with('_'))
                    || class_info
                        .instance_attributes
                        .iter()
                        .any(|attr| attr.starts_with('_')))
            {
                return errors;
            }

            if !docstr_attrs.iter().any(|attr| attr.starts_with('_')) {
                let exc_lines = find_string_in_text_range(
                    file_contents,
                    &TextRange::new(TextSize::new(0), class_info.def.range.end()),
                    vec!["attrs", "attributes"],
                );
                if let Some((line, line_location, _)) = exc_lines.first() {
                    errors.push(format_problem(
                        *line,
                        *line_location,
                        attrs_section_in_docstr_msg(),
                    ));
                }
            }
        }
    }
    errors
}

fn check_classes_for_multiple_attrs_section_in_docstr(
    class_info: &ClassInfo,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut errors = Vec::new();

    // Skip if this is a test file (similar pattern to other rules)
    if is_test_file {
        return errors;
    }

    if class_info.docstring.is_none() {
        return errors;
    }
    if let Some(docstring) = &class_info.docstring {
        // Check if docstring has attrs sections
        if !docstring.has_attrs_sections() {
            return errors;
        }
        if docstring.get_attrs_sections().len() == 1 {
            return errors;
        }
        let exc_lines = find_string_in_text_range(
            file_contents,
            &TextRange::new(TextSize::new(0), class_info.def.range.end()),
            vec!["Attrs", "Attributes"],
        );
        // TODO: attribute section can be attrs instead of Attrs. Make sure the
        // find_string_in_text_range function returns the actual found string
        if let Some((line, line_location, _)) = exc_lines.first() {
            let joined_attribute_sections: String = exc_lines
                .iter()
                .map(|(_, _, third)| third)
                .cloned()
                .collect::<Vec<_>>()
                .join(",");

            errors.push(format_problem(
                *line,
                *line_location,
                mult_attrs_section_in_docstr_msg(joined_attribute_sections.as_str()),
            ));
        } else {
            eprintln!("Warning: Could not find line information for multiple attrs section");
        }
    }
    errors
}

fn check_classes_for_multiple_attrs_in_docstr(
    class_info: &ClassInfo,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut errors = Vec::new();

    // Skip if this is a test file (similar pattern to other rules)
    if is_test_file {
        return errors;
    }

    if class_info.docstring.is_none() {
        return errors;
    }
    if let Some(docstring) = &class_info.docstring {
        // Check if docstring has attrs sections
        if !docstring.has_attrs_sections() {
            return errors;
        }

        let duplicates = find_duplicates(&docstring.get_attrs());

        for duplicate in duplicates {
            let exc_lines = find_string_in_text_range(
                file_contents,
                &TextRange::new(TextSize::new(0), class_info.def.range.end()),
                vec![&duplicate],
            );
            // TODO: attribute section can be attrs instead of Attrs. Make sure the
            // find_string_in_text_range function returns the actual found string
            let (line, line_location, _) = exc_lines.first().unwrap().to_owned();

            errors.push(format_problem(
                line,
                line_location,
                duplicate_attr_docstr_msg(duplicate.as_str()),
            ));
        }
    }
    errors
}

fn check_classes_for_missing_attrs_in_docstr(
    class_info: &ClassInfo,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut errors = Vec::new();

    // Skip if this is a test file (similar pattern to other rules)
    if is_test_file {
        return errors;
    }

    if class_info.docstring.is_none() {
        return errors;
    }
    let public_attributes: Vec<String> = class_info
        .attributes
        .clone()
        .into_iter()
        .filter(|attr| !attr.starts_with('_'))
        .collect();

    if public_attributes.is_empty() {
        return errors;
    }

    if let Some(docstring) = &class_info.docstring {
        // Check if docstring has attrs sections
        if !docstring.has_attrs_sections() {
            return errors;
        }

        let docstr_attrs = docstring.get_attrs();

        for attr in &public_attributes {
            if !docstr_attrs.contains(attr) {
                let exc_lines = find_string_in_text_range(
                    file_contents,
                    &TextRange::new(TextSize::new(0), class_info.def.range.end()),
                    vec![attr.as_str()],
                );
                if exc_lines.is_empty() {
                    eprintln!("Warning: Could not find attribute '{}' in source", attr);
                    continue;
                }
                let (line, line_location, _) = exc_lines.first().unwrap().to_owned();
                errors.push(format_problem(
                    line,
                    line_location,
                    attr_not_in_docstr_msg(attr.as_str()),
                ));
            }
        }
    }
    errors
}

fn check_classes_for_extra_attrs_in_docstr(
    class_info: &ClassInfo,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut errors = Vec::new();

    // Skip if this is a test file (similar pattern to other rules)
    if is_test_file {
        return errors;
    }

    if class_info.docstring.is_none() {
        return errors;
    }
    let mut public_attributes: HashSet<String> = class_info
        .attributes
        .iter()
        .filter(|attr| !attr.starts_with('_'))
        .cloned()
        .collect();

    public_attributes.extend(
        class_info
            .instance_attributes
            .iter()
            .filter(|attr| !attr.starts_with('_'))
            .cloned(),
    );

    if public_attributes.is_empty() {
        return errors;
    }

    if let Some(docstring) = &class_info.docstring {
        // Check if docstring has attrs sections
        if !docstring.has_attrs_sections() {
            return errors;
        }

        let docstr_attrs = docstring.get_attrs();

        for attr in &docstr_attrs {
            if !public_attributes.contains(attr) {
                let exc_lines = find_string_in_text_range(
                    file_contents,
                    &TextRange::new(TextSize::new(0), class_info.def.range.end()),
                    vec![attr.as_str()],
                );
                if exc_lines.is_empty() {
                    eprintln!(
                        "Warning: Could not find docstring attribute '{}' in source",
                        attr
                    );
                    continue;
                }
                let (line, line_location, _) = exc_lines.first().unwrap().to_owned();
                errors.push(format_problem(
                    line,
                    line_location,
                    attr_in_docstr_msg(attr.as_str()),
                ));
            }
        }
    }
    errors
}

// fn check_classes_for_extra_attrs_section_in_docstr(
//     class_info: &ClassInfo,
//     file_contents: &str,
//     is_test_file: bool,
// ) -> Vec<String> {
//     let mut errors = Vec::new();
//
//     // Skip if this is a test file (similar pattern to other rules)
//     if is_test_file {
//         return errors;
//     }
//
//     if class_info.docstring.is_none() {
//         return errors;
//     }
//
//     let attributes = &class_info.attributes;
//
//
//     if let Some(docstring) = &class_info.docstring {
//         // Check if docstring has attrs sections
//         if !docstring.has_attrs_sections() {
//             return errors;
//         }
//
//         for attr in docstring.get_attrs(){
//
//             if !attributes.contains(&attr){
//
//                 let exc_lines = find_string_in_text_range(
//                     file_contents,
//                     &TextRange::new(TextSize::new(0), class_info.def.range.end()),
//                     vec![attr.as_str()],
//                 );
//                 let (line, line_location, _) = exc_lines.first().unwrap().to_owned();
//                 errors.push(format_problem(
//                     line,
//                     line_location,
//                     attrs_section_in_docstr_msg(attr),
//                 ));
//             }
//         }
//     }
//     errors
// }
fn check_functions_for_multiple_exc_in_raises_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }
        let excs = function.raises.clone();
        // ignore if function doesn't raise anything
        if excs.is_empty() {
            continue;
        }

        let _docstring = function.docstring.clone().unwrap();
        let docstring_raises = _docstring.get_raises();
        let duplicates = find_duplicates(&docstring_raises);
        for raise in duplicates {
            let exc_lines = find_string_in_text_range(
                file_contents,
                &_docstring.get_range(),
                vec!["Raise", "Raises"],
            );
            let (line, line_location, _) = exc_lines.first().unwrap().to_owned();
            problem_functions.push(format_problem(
                line,
                line_location,
                duplicate_exc_msg(raise.as_str()),
            ));
        }
    }

    problem_functions
}

fn find_duplicates(strings: &Vec<String>) -> Vec<String> {
    let mut counts = HashMap::new();
    let mut duplicates = Vec::new();
    let mut seen = HashSet::new();

    for s in strings {
        let counter = counts.entry(s).or_insert(0);
        *counter += 1;
        if *counter == 2 && !seen.contains(s) {
            duplicates.push(s.clone());
            seen.insert(s.clone());
        }
    }

    duplicates
}

fn check_functions_for_re_raise_no_exc_in_raises_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }
        let excs = function.raises.clone();
        // ignore if function doesn't raise anything
        if excs.is_empty() {
            continue;
        }

        let _docstring = function.docstring.clone().unwrap();
        let docstring_raises = _docstring.get_raises();

        // ignore if docstring doesn't have a raises section
        let mut is_reraise = true;
        for _exc in excs {
            if _exc.exc.is_some() {
                is_reraise = false;
            }
        }
        if is_reraise && docstring_raises.is_empty() {
            let exc_lines =
                find_string_in_text_range(file_contents, &_docstring.get_range(), vec!["raise"]);
            let (line, line_location, _) = exc_lines.first().unwrap().to_owned();
            problem_functions.push(format_problem(
                line,
                line_location,
                re_raise_no_exc_in_docstr_msg(),
            ));
        }
    }

    problem_functions
}

fn check_functions_for_extra_exc_in_raises_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }
        let excs = function.raises.clone();
        // ignore if function doesn't raise anything
        if excs.is_empty() {
            continue;
        }

        let _docstring = function.docstring.clone().unwrap();

        let docstring_raises = _docstring.get_raises();

        // ignore if docstring doesn't have a raises section
        if docstring_raises.is_empty() {
            continue;
        }
        let mut exc_names: Vec<String> = Vec::new();
        let mut has_reraise = false;
        for _exc in excs {
            if _exc.exc.is_none() {
                has_reraise = true;
                continue;
            }
            let exc_name = get_exc_id(_exc);
            if exc_name.is_none() {
                has_reraise = true;
                continue;
            }
            let exc_name = exc_name.unwrap();
            exc_names.append(&mut vec![exc_name]);
        }
        if has_reraise {
            continue;
        }
        for exc_name in docstring_raises {
            if !exc_names.contains(&exc_name) {
                let exc_lines = find_string_in_text_range(
                    file_contents,
                    &_docstring.get_range(),
                    vec!["Raise:", "Raises:"],
                );
                let (line, line_location, _) = exc_lines.first().unwrap().to_owned();
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    exc_in_docstr_msg(exc_name.as_str()),
                ));
            }
        }
    }

    problem_functions
}
fn check_functions_for_missing_exc_in_raises_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }
        let excs = function.raises.clone();
        // ignore if function doesn't raise anything
        if excs.is_empty() {
            continue;
        }
        let _docstring = function.docstring.clone().unwrap();

        let docstring_raises_sections = _docstring.get_raises_sections();
        let docstring_raises = _docstring.get_raises();

        if docstring_raises_sections.is_empty() || docstring_raises_sections.len() > 1 {
            continue;
        }
        for _exc in excs {
            if _exc.exc.is_none() {
                continue;
            }
            let exc_name = get_exc_id(_exc);
            if exc_name.is_none() {
                continue;
            }
            let exc_name = exc_name.unwrap();
            if !docstring_raises.contains(&exc_name) {
                let args_lines = find_string_in_text_range(
                    file_contents,
                    function.def.range(),
                    vec![exc_name.as_str()],
                );
                let (line, line_location, _) = args_lines.first().unwrap().to_owned();
                problem_functions.push(format_problem(
                    line + 2,
                    line_location,
                    exc_not_in_docstr_msg(exc_name.as_str()),
                ));
            }
        }
    }

    problem_functions
}
fn get_exc_id(exc: StmtRaise) -> Option<String> {
    if exc.exc.is_none() {
        return None;
    }
    let _exc = exc.exc.unwrap();

    if _exc.is_attribute_expr() {
        let _exc = _exc.as_attribute_expr();
        Some(_exc.unwrap().attr.to_string())
    } else if _exc.is_named_expr_expr() {
        let _exc = _exc.as_named_expr_expr();
        Some(_exc.unwrap().value.as_name_expr().unwrap().id.to_string())
    } else if _exc.is_name_expr() {
        let _exc = _exc.as_name_expr();
        Some(_exc.unwrap().id.to_string())
    } else if _exc.is_call_expr() {
        let some_func = &_exc.as_call_expr().unwrap().func;
        if some_func.is_attribute_expr() {
            let some_attribute = some_func.as_attribute_expr().unwrap();
            Some(some_attribute.attr.to_string())
        } else if some_func.is_name_expr() {
            let some_exp = some_func.as_name_expr();
            Some(some_exp.unwrap().id.to_string())
        } else if some_func.is_lambda_expr() {
            None
            // Some("Lambda".to_string())
        } else {
            None
        }
    } else {
        None
    }
}
fn check_functions_for_missing_arg_in_args_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        let args = function.def.args();
        let clean_args = cleanse_args(args, true);
        // ignore if function doesn't have args
        if is_args_empty(&clean_args) {
            continue;
        }

        let docstring_args_sections = function.docstring.clone().unwrap().get_args_sections();
        let docstring_args = function.docstring.clone().unwrap().get_args();
        if docstring_args_sections.is_empty() {
            continue;
        }
        let mut _range = function.def.range();
        // if DC022 is here we don't need to check for DC023
        if function
            .docstring
            .clone()
            .unwrap()
            .get_args_sections()
            .len()
            > 1
        {
            continue;
        }

        if clean_args.vararg.is_some() {
            let arg_name = clean_args.vararg.unwrap().arg.to_string();
            if let Some(_result) =
                is_arg_in_docstring(arg_name, &docstring_args, _range, file_contents)
            {
                problem_functions.push(_result);
            }
        }
        if clean_args.kwarg.is_some() {
            let arg_name = clean_args.kwarg.unwrap().arg.to_string();
            if let Some(_result) =
                is_arg_in_docstring(arg_name, &docstring_args, _range, file_contents)
            {
                problem_functions.push(_result);
            }
        }
        for arg in clean_args.args {
            let arg_name = arg.def.arg.to_string();
            if let Some(_result) =
                is_arg_in_docstring(arg_name, &docstring_args, _range, file_contents)
            {
                problem_functions.push(_result);
            }
        }
        for arg in clean_args.kwonlyargs {
            let arg_name = arg.def.arg.to_string();
            if let Some(_result) =
                is_arg_in_docstring(arg_name, &docstring_args, _range, file_contents)
            {
                problem_functions.push(_result);
            }
        }
        for arg in clean_args.posonlyargs {
            let arg_name = arg.def.arg.to_string();
            if let Some(_result) =
                is_arg_in_docstring(arg_name, &docstring_args, _range, file_contents)
            {
                problem_functions.push(_result);
            }
        }
    }

    problem_functions
}

fn is_arg_in_docstring(
    arg_name: String,
    docstring_args: &Vec<String>,
    _range: &TextRange,
    file_contents: &str,
) -> Option<String> {
    if !docstring_args.contains(&arg_name) {
        let args_lines = find_string_in_text_range(file_contents, _range, vec![arg_name.as_str()]);
        let (line, line_location, _) = args_lines.first().unwrap().to_owned();
        return Some(format_problem(
            line + 2,
            line_location,
            arg_not_in_docstr_msg(arg_name.as_str()),
        ));
    }
    None
}
fn check_functions_for_multiple_args_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        let args = function.def.args();
        let clean_args = cleanse_args(args, true);
        // ignore if function doesn't have args
        if is_args_empty(&clean_args) {
            continue;
        }
        if function
            .docstring
            .clone()
            .unwrap()
            .get_args_sections()
            .len()
            > 1
        {
            let mut _range = function.def.range();
            let args_lines = find_string_in_text_range(
                file_contents,
                _range,
                vec!["Args:", "Arguments:", "Parameters:"],
            );
            if args_lines.len() < 2 {
                continue;
            }
            let mut founds: Vec<String> = Vec::new();
            for (_, _, found) in &args_lines {
                // the latest char is a : which we do not want
                founds.push(found[..found.len() - 1].to_string());
            }
            let (line, line_location, _) = args_lines.first().unwrap().to_owned();
            problem_functions.push(format_problem(
                line,
                line_location,
                mult_args_sections_in_docstr_msg(founds.join(",").as_str()),
            ));
        }
    }

    problem_functions
}

fn check_functions_for_multiple_yields_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        if function.docstring.clone().unwrap().get_yields().len() > 1 {
            let mut _range = function.def.range();
            let yield_lines =
                find_string_in_text_range(file_contents, _range, vec!["Yield:", "Yields:"]);
            if yield_lines.len() < 2 {
                continue;
            }
            let mut founds: Vec<String> = Vec::new();
            for (_, _, found) in &yield_lines {
                // the latest char is a : which we do not want
                founds.push(found[..found.len() - 1].to_string());
            }
            let (line, line_location, _) = yield_lines.first().unwrap().to_owned();
            problem_functions.push(format_problem(
                line,
                line_location,
                mult_yields_sections_in_docstr_msg(founds.join(",").as_str()),
            ));
        }
    }

    problem_functions
}
fn check_functions_for_multiple_raises_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        let _docstring = function.docstring.clone().unwrap();

        if _docstring.get_raises_sections().len() > 1 {
            let raise_lines = find_string_in_text_range(
                file_contents,
                &_docstring.get_range(),
                vec!["Raises:", "Raise:"],
            );
            if raise_lines.len() < 2 {
                continue;
            }
            let mut founds: Vec<String> = Vec::new();
            for (_, _, found) in &raise_lines {
                // the latest char is a : which we do not want
                founds.push(found[..found.len() - 1].to_string());
            }
            let (line, line_location, _) = raise_lines.first().unwrap().to_owned();
            problem_functions.push(format_problem(
                line,
                line_location,
                mult_raises_sections_in_docstr_msg(founds.join(",").as_str()),
            ));
        }
    }

    problem_functions
}
fn check_functions_for_multiple_returns_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        if function.docstring.clone().unwrap().get_returns().len() > 1 {
            let mut _range = function.def.range();
            let return_lines =
                find_string_in_text_range(file_contents, _range, vec!["Return:", "Returns:"]);
            if return_lines.len() < 2 {
                continue;
            }
            let mut founds: Vec<String> = Vec::new();
            for (_, _, found) in &return_lines {
                // the latest char is a : which we do not want
                founds.push(found[..found.len() - 1].to_string());
            }
            let (line, line_location, _) = return_lines.first().unwrap().to_owned();
            problem_functions.push(format_problem(
                line,
                line_location,
                mult_returns_sections_in_docstr_msg(founds.join(",").as_str()),
            ));
        }
    }

    problem_functions
}
fn check_functions_for_extra_args_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip_dont_skip_private(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        let args = function.def.args();
        let clean_args = cleanse_args(args, true);
        if !is_args_empty(&clean_args) {
            continue;
        }
        if function.docstring.clone().unwrap().has_args() {
            continue;
        }

        if function.docstring.clone().unwrap().has_args_sections() {
            let mut _range = function.docstring.clone().unwrap().get_range();
            let args_lines = find_string_in_text_range(file_contents, &_range, vec!["Args:"]);
            if args_lines.is_empty() {
                continue;
            }

            for (line, line_location, _) in args_lines {
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    args_section_in_docstr_msg(),
                ));
            }
        }
    }

    problem_functions
}

fn cleanse_args(args: &Arguments, del_private_args: bool) -> Arguments {
    let mut clean_args: Arguments = args.clone();

    if let Some(vararg) = clean_args.vararg.clone() {
        let arg_name = vararg.arg.trim();
        let should_drop =
            matches!(arg_name, "self" | "cls") || (del_private_args && arg_name.starts_with('_'));
        if should_drop {
            clean_args.vararg = None;
        }
    }

    if let Some(kwarg) = clean_args.kwarg.clone() {
        let arg_name = kwarg.arg.trim();
        if del_private_args && arg_name.starts_with('_') {
            clean_args.kwarg = None;
        }
    }

    clean_args.args.retain(|arg| {
        let arg_name = arg.def.arg.trim();
        if matches!(arg_name, "self" | "cls") {
            return false;
        }
        if del_private_args && arg_name.starts_with('_') {
            return false;
        }
        true
    });

    clean_args.kwonlyargs.retain(|arg| {
        let arg_name = arg.def.arg.trim();
        if del_private_args && arg_name.starts_with('_') {
            return false;
        }
        true
    });

    clean_args.posonlyargs.retain(|arg| {
        let arg_name = arg.def.arg.trim();
        if matches!(arg_name, "self" | "cls") {
            return false;
        }
        if del_private_args && arg_name.starts_with('_') {
            return false;
        }
        true
    });

    clean_args
}

fn check_functions_for_extra_yields_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip_dont_skip_private(function, is_test_file) {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        // Skip abstract methods - they can have yields sections without yielding
        if is_abstractmethod(function) {
            continue;
        }

        let yield_statements: &Vec<YieldKind> = &function.yields;

        if (yield_statements.len() == 1
            && is_yield_empty(&file_contents, yield_statements.first().unwrap()))
            || yield_statements.is_empty() && function.docstring.clone().unwrap().has_yields()
        {
            let mut _range = function.def.range();
            let yield_lines = find_string_in_text_range(file_contents, _range, vec!["Yields:"]);
            if yield_lines.is_empty() {
                continue;
            }
            for (line, line_location, _) in yield_lines {
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    yields_section_in_docstr_msg(),
                ));
            }
        }
    }

    problem_functions
}

fn check_functions_for_extra_raises_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip_dont_skip_private(function, is_test_file) {
            continue;
        }
        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }
        // Skip abstract methods - they can have raises sections without raising
        if is_abstractmethod(function) {
            continue;
        }
        let _docstring = function.docstring.clone().unwrap();

        if function.raises.is_empty() && _docstring.has_raises_sections() {
            let raise_lines = find_string_in_text_range(
                file_contents,
                &_docstring.get_range(),
                vec!["Raise:", "Raises:"],
            );
            if raise_lines.is_empty() {
                continue;
            }
            for (line, line_location, _) in raise_lines {
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    raises_section_in_docstr_msg(),
                ));
            }
        }
    }

    problem_functions
}
fn check_functions_for_extra_returns_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip_dont_skip_private(function, is_test_file) {
            continue;
        }
        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }
        // Skip abstract methods - they can have returns sections without returning
        if is_abstractmethod(function) {
            continue;
        }
        let _docstring = function.docstring.clone().unwrap();

        let return_statements: &Vec<StmtReturn> = &function.returns;

        if ((return_statements.len() == 1 && return_statements.first().unwrap().value.is_none())
            || return_statements.is_empty())
            && _docstring.has_returns()
        {
            let return_lines =
                find_string_in_text_range(file_contents, function.def.range(), vec!["Returns:"]);
            if return_lines.is_empty() {
                continue;
            }
            for (line, line_location, _) in return_lines {
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    returns_section_in_docstr_msg(),
                ));
            }
        }
    }

    problem_functions
}

fn check_functions_for_missing_raises_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }
        let raise_statements: &Vec<StmtRaise> = &function.raises;
        // ignore if function doesn't have returns
        if raise_statements.is_empty() {
            continue;
        }
        if function.docstring.is_none() {
            continue;
        }

        if !function.docstring.as_ref().unwrap().has_raises_sections() {
            for ret in raise_statements {
                let (line, line_location) =
                    find_line_and_column(file_contents, ret.range.start().to_usize())
                        .unwrap_or((0, 0));
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    raises_section_not_in_docstr_msg(),
                ));
            }
        }
    }

    problem_functions
}

fn check_functions_for_missing_yields_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }
        // ignore if function doesn't have yields
        let yield_statements: &Vec<YieldKind> = &function.yields;
        if yield_statements.is_empty() {
            continue;
        }

        if function.docstring.is_none() {
            continue;
        }

        if !function.docstring.clone().unwrap().has_yields() {
            for _yield in yield_statements {
                let _range = &_yield.range();
                if is_yield_empty(&file_contents, _yield) {
                    continue;
                }
                let (line, line_location) =
                    find_line_and_column(file_contents, _range.start().to_usize())
                        .unwrap_or((0, 0));
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    yields_section_not_in_docstr_msg(),
                ));
            }
        }
    }

    problem_functions
}

fn is_args_empty(args: &Arguments) -> bool {
    if args.vararg.is_some() {
        return false;
    }
    if args.kwarg.is_some() {
        return false;
    }
    if !args.kwonlyargs.is_empty() {
        return false;
    }
    if !args.args.is_empty() {
        return false;
    }
    if !args.posonlyargs.is_empty() {
        return false;
    }
    true
}

fn check_functions_for_missing_args_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }
        // ignore if function doesn't have args
        let args = function.def.args();
        let clean_args = cleanse_args(args, true);

        if is_args_empty(&clean_args) {
            continue;
        }

        if function.docstring.is_none() {
            continue;
        }

        if function.docstring.clone().unwrap().has_args_sections() {
            continue;
        }

        let _range = function.def.range();
        let doc_loc = find_string_in_text_range(file_contents, _range, vec!["\"\"\""]);
        let (line, line_location, _) = doc_loc.first().unwrap().to_owned();

        problem_functions.push(format_problem(
            line + 2,
            line_location,
            args_section_not_in_docstr_msg(),
        ));
    }

    problem_functions
}

fn check_functions_for_missing_returns_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip(function, is_test_file) {
            continue;
        }
        // ignore if function doesn't have returns
        let return_statements: &Vec<StmtReturn> = &function.returns;
        if return_statements.is_empty() {
            continue;
        }
        if function.docstring.is_none() {
            continue;
        }

        if !function.docstring.clone().unwrap().has_returns() {
            for ret in return_statements {
                if ret.value.is_some() {
                    let _range = &ret.range;

                    let (line, line_location) =
                        find_line_and_column(file_contents, _range.start().to_usize())
                            .unwrap_or((0, 0));
                    problem_functions.push(format_problem(
                        line,
                        line_location,
                        returns_section_not_in_docstr_msg(),
                    ));
                }
            }
        }
    }

    problem_functions
}

fn generate_rules_output_with_inheritance(
    file_contents: &str,
    things: &DocstringCollector,
    is_test_file: bool,
    file_name: Option<&str>,
    implementing_methods: Option<&std::collections::HashSet<(String, String, String)>>,
) -> Vec<String> {
    let suppressions = SuppressionIndex::new(file_contents, things);
    // DC0010: docstring missing on a function/ method/ class
    let mut problem_functions: Vec<String> = Vec::new();

    // DC0010: docstring missing on a function/ method/ class
    problem_functions.extend(check_functions_for_missing_docstring_with_inheritance(
        &things.function_infos,
        file_contents,
        is_test_file,
        file_name,
        implementing_methods,
    ));

    // DCO030: function/ method that returns a value does not have the returns section in the docstring.
    problem_functions.extend(check_functions_for_missing_returns_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));

    // DC031: function/ method that does not return a value should not
    // have the returns section in the docstring
    problem_functions.extend(check_functions_for_extra_returns_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));

    // DC032: a docstring should only contain a single returns
    // section, found %s
    problem_functions.extend(check_functions_for_multiple_returns_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));

    // DC040: function/ method that yields a value should have the
    // yields section in the docstring
    problem_functions.extend(check_functions_for_missing_yields_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));

    // DC041: function/ method that does not yield a value should not
    // have the yields section in the docstring
    problem_functions.extend(check_functions_for_extra_yields_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));

    // DC042: a docstring should only contain a single yields
    // section, found %s
    problem_functions.extend(check_functions_for_multiple_yields_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC020: function/ method with arguments should have the
    // arguments section in the docstring
    problem_functions.extend(check_functions_for_missing_args_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC021: function/ method without arguments should not have the
    // arguments section in the docstring
    problem_functions.extend(check_functions_for_extra_args_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC022: function/ method without arguments should not have the
    // arguments section in the docstring
    problem_functions.extend(check_functions_for_multiple_args_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC023: argument should be described in the docstring
    problem_functions.extend(check_functions_for_missing_arg_in_args_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC024: argument should not be described in the docstring
    problem_functions.extend(check_functions_for_extra_arg_in_args_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC025: argument documented multiple times
    problem_functions.extend(check_functions_for_duplicate_arg_in_args_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC050: function/ method that raises a value should have the
    // raises section in the docstring
    problem_functions.extend(check_functions_for_missing_raises_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC051: function/ method that does not raise a value should not
    // have the raises section in the docstring
    problem_functions.extend(check_functions_for_extra_raises_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC052: a docstring should only contain a single raises
    // section, found %s
    problem_functions.extend(check_functions_for_multiple_raises_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC053: exception should be described in the docstring
    problem_functions.extend(check_functions_for_missing_exc_in_raises_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC054: exception should not be described in the docstring
    problem_functions.extend(check_functions_for_extra_exc_in_raises_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC055: reraise exception not described in the docstring
    problem_functions.extend(check_functions_for_re_raise_no_exc_in_raises_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    // DC056: exception documented multiple times in the docstring
    problem_functions.extend(check_functions_for_multiple_exc_in_raises_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    for class_info in &things.class_infos {
        let class_name = class_info.def.name.to_string();

        problem_functions.extend(check_functions_for_missing_docstring_in_class(
            &class_info.funcs,
            file_contents,
            is_test_file,
            file_name,
            Some(&class_name),
            implementing_methods,
        ));
        problem_functions.extend(check_functions_for_missing_returns_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_extra_returns_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_multiple_returns_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_missing_yields_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_extra_yields_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_multiple_yields_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_missing_args_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_extra_args_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_multiple_args_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_missing_arg_in_args_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_extra_arg_in_args_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_duplicate_arg_in_args_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_missing_raises_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_extra_raises_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_multiple_raises_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_missing_exc_in_raises_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_extra_exc_in_raises_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_re_raise_no_exc_in_raises_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        problem_functions.extend(check_functions_for_multiple_exc_in_raises_section(
            &class_info.funcs,
            file_contents,
            is_test_file,
        ));
        // DC060: attribute section not in docstring
        problem_functions.extend(check_classes_for_attrs_section_not_in_docstr(
            class_info,
            file_contents,
            is_test_file,
        ));
        // DC061: attribute section in docstring but no attribute
        problem_functions.extend(check_classes_for_extra_attrs_section_in_docstr(
            class_info,
            file_contents,
            is_test_file,
        ));
        // DC062: There should only be 1 attribute section in docstring
        problem_functions.extend(check_classes_for_multiple_attrs_section_in_docstr(
            class_info,
            file_contents,
            is_test_file,
        ));
        // DC063: Attribute should be in docstring
        problem_functions.extend(check_classes_for_missing_attrs_in_docstr(
            class_info,
            file_contents,
            is_test_file,
        ));

        // DC064: Attribute should not be in docstring
        problem_functions.extend(check_classes_for_extra_attrs_in_docstr(
            class_info,
            file_contents,
            is_test_file,
        ));

        // DC065: Attribute documented multiple times
        problem_functions.extend(check_classes_for_multiple_attrs_in_docstr(
            class_info,
            file_contents,
            is_test_file,
        ));
    }
    problem_functions
        .into_iter()
        .filter(|entry| !suppressions.is_suppressed_entry(entry))
        .collect()
}

fn check_functions_for_missing_docstring_in_class(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
    file_name: Option<&str>,
    class_name: Option<&str>,
    implementing_methods: Option<&std::collections::HashSet<(String, String, String)>>,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip_dont_skip_private(function, is_test_file) {
            continue;
        }

        if function.docstring.is_none() {
            // Check if this method implements an abstract method
            // If so, skip D010 check (it inherits the docstring)
            if let (Some(impl_methods), Some(file_path), Some(cls_name)) =
                (implementing_methods, file_name, class_name)
            {
                let method_name = function.def.name().to_string();
                let key = (file_path.to_string(), cls_name.to_string(), method_name);

                if impl_methods.contains(&key) {
                    // This method implements an abstract method, skip D010
                    continue;
                }
            }

            let (line, line_location) =
                find_line_and_column(file_contents, function.def.range().start().to_usize())
                    .unwrap_or((0, 0));
            problem_functions.push(format_problem(line, line_location, docstr_missing_msg()));
        }
    }

    problem_functions
}

fn check_functions_for_missing_docstring_with_inheritance(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
    file_name: Option<&str>,
    implementing_methods: Option<&std::collections::HashSet<(String, String, String)>>,
) -> Vec<String> {
    // For top-level functions (not in a class), use empty class name
    check_functions_for_missing_docstring_in_class(
        function_infos,
        file_contents,
        is_test_file,
        file_name,
        Some(""),
        implementing_methods,
    )
}

fn is_property(function: &FunctionInfo) -> bool {
    for decorator in function.def.decorator_list() {
        if decorator.is_name_expr() {
            let id = &decorator.as_name_expr().unwrap().id;
            if id.eq_ignore_ascii_case("property") {
                return true;
            }
        }
        if decorator.is_call_expr() {
            let call: &ExprCall = decorator.as_call_expr().unwrap();
            if let Some(name_expr) = call.func.as_name_expr() {
                let id = &name_expr.id;
                if id.eq_ignore_ascii_case("property") {
                    return true;
                }
            }
        }
    }

    false
}

fn is_overload(function: &FunctionInfo) -> bool {
    for decorator in function.def.decorator_list() {
        if decorator.is_name_expr() {
            let id = &decorator.as_name_expr().unwrap().id;
            if id.eq_ignore_ascii_case("overload") {
                return true;
            }
        }

        if decorator.is_call_expr() {
            let call: &ExprCall = decorator.as_call_expr().unwrap();
            if let Some(name_expr) = call.func.as_name_expr() {
                let id = &name_expr.id;
                if id.eq_ignore_ascii_case("overload") {
                    return true;
                }
            }
        }

        if decorator.is_attribute_expr() {
            let attr: &ExprAttribute = decorator.as_attribute_expr().unwrap();
            if attr.value.is_name_expr() {
                let name = &attr.value.as_name_expr().unwrap().id;
                if attr.attr.to_string() == "overload" && name == "typing" {
                    return true;
                }
            }
        }
    }
    false
}

fn is_abstractmethod(function: &FunctionInfo) -> bool {
    for decorator in function.def.decorator_list() {
        if decorator.is_name_expr() {
            let id = &decorator.as_name_expr().unwrap().id;
            if id.eq_ignore_ascii_case("abstractmethod") {
                return true;
            }
        }

        if decorator.is_call_expr() {
            let call: &ExprCall = decorator.as_call_expr().unwrap();
            if let Some(name_expr) = call.func.as_name_expr() {
                let id = &name_expr.id;
                if id.eq_ignore_ascii_case("abstractmethod") {
                    return true;
                }
            }
        }

        if decorator.is_attribute_expr() {
            let attr: &ExprAttribute = decorator.as_attribute_expr().unwrap();
            if attr.value.is_name_expr() {
                let name = &attr.value.as_name_expr().unwrap().id;
                if (attr.attr.to_string() == "abstractmethod" && name == "abc")
                    || (attr.attr.to_string() == "abstractmethod" && name == "ABC")
                {
                    return true;
                }
            }
        }
    }
    false
}

fn is_fixture(function: FunctionDefKind) -> bool {
    let mut is_fixture = false;

    for decorator in function.decorator_list() {
        if decorator.is_name_expr() && is_name_fixture_decorator(decorator) {
            is_fixture = true;
            break;
        }

        if decorator.is_call_expr() {
            let call: &ExprCall = decorator.as_call_expr().unwrap();
            let _f = call.func.clone();
            if let Some(attr_expr) = call.func.as_attribute_expr() {
                if attr_expr.attr.to_string() == "fixture" {
                    is_fixture = true;
                    break;
                }
            }
            if let Some(name_expr) = call.func.as_name_expr() {
                let id = &name_expr.id;
                if id.eq_ignore_ascii_case("fixture") {
                    is_fixture = true;
                    break;
                }
            }
        }
        if decorator.is_attribute_expr() {
            let attr: &ExprAttribute = decorator.as_attribute_expr().unwrap();
            if attr.attr.eq_ignore_ascii_case("fixture") {
                is_fixture = true;
                break;
            }
        }
    }

    is_fixture
}

fn is_cached_property(function: FunctionDefKind) -> bool {
    let mut is_fixture = false;

    for decorator in function.decorator_list() {
        if decorator.is_name_expr() {
            let id = &decorator.as_name_expr().unwrap().id;
            if id.eq_ignore_ascii_case("cached_property") {
                return true;
            }
        }

        if decorator.is_call_expr() {
            let call: &ExprCall = decorator.as_call_expr().unwrap();
            let _f = call.func.clone();
            if let Some(attr_expr) = call.func.as_attribute_expr() {
                if attr_expr.attr.eq_ignore_ascii_case("cached_property") {
                    is_fixture = true;
                    break;
                }
            }
            if let Some(name_expr) = call.func.as_name_expr() {
                let id = &name_expr.id;
                if id.eq_ignore_ascii_case("fixture") {
                    is_fixture = true;
                    break;
                }
            }
        }
        if decorator.is_attribute_expr() {
            let attr: &ExprAttribute = decorator.as_attribute_expr().unwrap();
            if attr.attr.eq_ignore_ascii_case("cached_property") {
                is_fixture = true;
                break;
            }
        }
    }

    is_fixture
}
fn is_yield_empty(file_contents: &&str, yield_kind: &YieldKind) -> bool {
    let _range: &TextRange = yield_kind.range();

    let start = usize::try_from(_range.start().to_u32()).unwrap();
    let end = usize::try_from(_range.end().to_u32()).unwrap();

    let sub = &file_contents[start..end];
    // if it doesn't yield any value
    if sub == "yield" {
        return true;
    }
    false
}
fn is_name_fixture_decorator(decorator: &Expr) -> bool {
    let id = &decorator.as_name_expr().unwrap().id;
    if id.to_string().to_lowercase() == "fixture" {
        return true;
    }
    false
}
fn should_skip_dont_skip_private(function: &FunctionInfo, is_test_file: bool) -> bool {
    // ignore overloads
    // Skip function if *any* decorator is an overload
    if is_overload(function) {
        return true;
    }
    if is_property(function) {
        return true;
    }
    let func_name = function.def.name().to_string();
    if func_name.starts_with("test_") && is_test_file {
        return true;
    }
    if is_cached_property(function.def.clone()) {
        return true;
    }
    if is_fixture(function.def.clone()) && is_test_file {
        return true;
    }
    false
}

fn should_skip(function: &FunctionInfo, is_test_file: bool) -> bool {
    // ignore overloads
    // Skip function if *any* decorator is an overload
    if is_overload(function) {
        return true;
    }
    if is_property(function) {
        return true;
    }
    let func_name = function.def.name().to_string();
    if func_name.starts_with("test_") && is_test_file {
        return true;
    }
    if is_cached_property(function.def.clone()) {
        return true;
    }
    if is_fixture(function.def.clone()) && is_test_file {
        return true;
    }
    if func_name.starts_with("_") {
        return true;
    }
    false
}

/// Collect inheritance information from a file for cross-file validation
pub fn collect_inheritance_info(
    file_name: &str,
    tracker: &mut crate::inheritance::InheritanceTracker,
) {
    use crate::inheritance::{extract_base_classes, AbstractMethodInfo, ConcreteMethodInfo};

    let code = read_file(file_name);
    let collector = get_result(&code, Some(file_name));

    // Process classes
    for class_info in &collector.class_infos {
        let class_name = class_info.def.name.to_string();
        let base_classes = extract_base_classes(class_info);

        // Process methods in the class
        for method in &class_info.funcs {
            let method_name = method.def.name().to_string();

            // Skip private methods
            if method_name.starts_with("_") && method_name != "__init__" {
                continue;
            }

            // Check if this is an abstract method
            if is_abstractmethod(method) {
                // Register abstract method
                let has_returns = method
                    .docstring
                    .as_ref()
                    .map(|d| d.has_returns())
                    .unwrap_or(false);
                let has_raises = method
                    .docstring
                    .as_ref()
                    .map(|d| d.has_raises_sections())
                    .unwrap_or(false);
                let has_yields = method
                    .docstring
                    .as_ref()
                    .map(|d| d.has_yields())
                    .unwrap_or(false);

                tracker.register_abstract_method(AbstractMethodInfo {
                    class_name: class_name.clone(),
                    method_name: method_name.clone(),
                    has_returns,
                    has_raises,
                    has_yields,
                    file_path: file_name.to_string(),
                });
            } else if !base_classes.is_empty() {
                // This is a concrete method in a class with base classes
                // It might be implementing an abstract method
                let has_docstring = method.docstring.is_some();
                let has_returns = method
                    .docstring
                    .as_ref()
                    .map(|d| d.has_returns())
                    .unwrap_or(false);
                let has_raises = method
                    .docstring
                    .as_ref()
                    .map(|d| d.has_raises_sections())
                    .unwrap_or(false);
                let has_yields = method
                    .docstring
                    .as_ref()
                    .map(|d| d.has_yields())
                    .unwrap_or(false);

                // Get the line number
                let (line, _) = find_line_and_column(&code, method.def.range().start().to_usize())
                    .unwrap_or((0, 0));

                tracker.register_concrete_method(ConcreteMethodInfo {
                    class_name: class_name.clone(),
                    method_name: method_name.clone(),
                    base_classes: base_classes.clone(),
                    has_returns,
                    has_raises,
                    has_yields,
                    has_docstring,
                    file_path: file_name.to_string(),
                    line,
                });
            }
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
pub fn lint_file(code: &str, file_name: Option<&str>) -> Vec<String> {
    lint_file_with_inheritance(code, file_name, None)
}
