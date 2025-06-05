use crate::constants::{
    arg_in_docstr_msg, arg_not_in_docstr_msg, args_section_in_docstr_msg,
    args_section_not_in_docstr_msg, docstr_missing_msg, duplicate_arg_msg, exc_in_docstr_msg,
    exc_not_in_docstr_msg, mult_args_sections_in_docstr_msg, mult_raises_sections_in_docstr_msg,
    mult_returns_sections_in_docstr_msg, mult_yields_sections_in_docstr_msg,
    raises_section_in_docstr_msg, raises_section_not_in_docstr_msg, returns_section_in_docstr_msg,
    returns_section_not_in_docstr_msg, yields_section_in_docstr_msg,
    yields_section_not_in_docstr_msg,
};
use crate::plugin::{get_result, DocstringCollector, FunctionDefKind, FunctionInfo, YieldKind};
use pyo3::prelude::*;
use rustpython_ast::text_size::TextRange;
use rustpython_ast::{Arguments, Expr, ExprAttribute, ExprCall, StmtRaise, StmtReturn};
use std::collections::HashMap;
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

pub fn lint_file(code: &str, file_name: Option<&str>) -> Vec<String> {
    // Make a mutable String to hold the actual code
    let mut code = code.to_string();

    // If there's a file, override it
    if let Some(file) = file_name {
        code = read_file(file); // assuming this returns String
    }

    apply_rules(code.as_str(), file_name)
}

#[pyfunction]
#[pyo3(signature = (code, file_name=None))]
pub fn apply_rules(code: &str, file_name: Option<&str>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    let things = get_result(code, file_name);

    let test_file = is_test_file(file_name);

    output.extend(generate_rules_output(code, &things, test_file));

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

    let sub = &s[start..end].to_lowercase();
    let mut positions: Vec<(usize, usize, String)> = Vec::new();
    let target_strings_lower: Vec<String> =
        target_strings.iter().map(|t| t.to_lowercase()).collect();

    let mut offset = 0;
    while offset < sub.len() {
        let mut matched = false;
        for (i, target) in target_strings_lower.iter().enumerate() {
            if sub[offset..].starts_with(target) {
                let absolute_pos = start + offset;

                let before = &s[..absolute_pos];
                let line_number = before.lines().count(); // 1-based

                let column_number = before
                    .rfind('\n')
                    .map(|idx| absolute_pos - idx - 1)
                    .unwrap_or(absolute_pos);

                positions.push((
                    line_number - 2,
                    column_number,
                    target_strings[i].to_string(),
                ));
                offset += target.len();
                matched = true;
                break; // only take the first match at this position
            }
        }
        if !matched {
            offset += 1;
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
            let column = char_index - current_char_index;
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

        println!("{}", function.docstring.clone().unwrap().__repr__());
        if docstring_args_sections.is_empty() {
            continue;
        }
        println!("docstring_args: {:?}", docstring_args);
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
                let (line, line_location, _) = args_lines.first().unwrap().to_owned();
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    duplicate_arg_msg(arg_name.as_str()),
                ));
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

        println!("{}", function.docstring.clone().unwrap().__repr__());
        if docstring_args_sections.is_empty() {
            continue;
        }
        println!("docstring_args: {:?}", docstring_args);
        println!("clean_args: {:?}", clean_args);
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
                let (line, line_location, _) = args_lines.first().unwrap().to_owned();
                problem_functions.push(format_problem(
                    line + 2,
                    line_location,
                    arg_in_docstr_msg(arg_name.as_str()),
                ));
            }
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

        let docstring_raises = function.docstring.clone().unwrap().get_raises();

        // ignore if docstring doesn't have a raises section
        if docstring_raises.is_empty() {
            continue;
        }
        let mut _range = function.docstring.clone().unwrap().get_range();
        // // if DC022 is here we don't need to check for DC023
        // if function
        //     .docstring
        //     .clone()
        //     .unwrap()
        //     .get_raises_sections()
        //     .len()
        //     > 1
        // {
        //     continue;
        // }
        let mut exc_names: Vec<String> = Vec::new();
        for _exc in excs {
            if _exc.exc.is_none() {
                continue;
            }
            let exc_name = get_exc_id(_exc);
            if exc_name.is_none() {
                continue;
            }
            let exc_name = exc_name.unwrap();
            exc_names.append(&mut vec![exc_name]);
        }
        for exc_name in docstring_raises {
            if !exc_names.contains(&exc_name) {
                let exc_lines =
                    find_string_in_text_range(file_contents, &_range, vec!["Raise:", "Raises:"]);
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

        let docstring_raises_sections = function.docstring.clone().unwrap().get_raises_sections();
        let docstring_raises = function.docstring.clone().unwrap().get_raises();

        if docstring_raises_sections.is_empty() {
            continue;
        }
        let mut _range = function.def.range();
        // if DC022 is here we don't need to check for DC023
        if function
            .docstring
            .clone()
            .unwrap()
            .get_raises_sections()
            .len()
            > 1
        {
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
                let args_lines =
                    find_string_in_text_range(file_contents, _range, vec![exc_name.as_str()]);
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
    if _exc.is_call_expr() {
        let _exc = _exc.as_call_expr();
        return Some(_exc.unwrap().func.as_name_expr().unwrap().id.to_string());
    }
    if _exc.is_named_expr_expr() {
        let _exc = _exc.as_named_expr_expr();
        return Some(_exc.unwrap().value.as_name_expr().unwrap().id.to_string());
    } else if _exc.is_name_expr() {
        let _exc = _exc.as_name_expr();
        return Some(_exc.unwrap().id.to_string());
    } else if _exc.is_attribute_expr() {
        let _exc = _exc.as_attribute_expr();
        return Some(_exc.unwrap().attr.to_string());
    } else {
        return None;
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

        println!("{}", function.docstring.clone().unwrap().__repr__());
        if docstring_args_sections.is_empty() {
            continue;
        }
        println!("docstring_args: {:?}", docstring_args);
        println!("clean_args: {:?}", clean_args);
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
    println!(
        "arg: {}, docstring_args: {:?}, contains",
        arg_name, docstring_args
    );
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
            println!("args_lines: {:?}", args_lines);
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

        if function
            .docstring
            .clone()
            .unwrap()
            .get_raises_sections()
            .len()
            > 1
        {
            let mut _range = &function.docstring.clone().unwrap().get_range();
            let raise_lines =
                find_string_in_text_range(file_contents, _range, vec!["Raises:", "Raise:"]);
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
    if args.vararg.is_some() {
        let arg_name = args.vararg.clone().unwrap().arg.trim().to_owned();
        if arg_name == "self" {
            clean_args.vararg = None;
        }
        if arg_name == "cls" {
            clean_args.vararg = None;
        }
        if del_private_args && arg_name.starts_with("_") {
            clean_args.vararg = None;
        }
    }
    if args.kwarg.is_some() {
        let arg_name = args.kwarg.clone().unwrap().arg.trim().to_owned();
        if del_private_args && arg_name.starts_with("_") {
            clean_args.kwarg = None;
        }
    }
    for (index, arg) in args.args.iter().enumerate() {
        let arg_name = arg.def.arg.trim();
        if arg_name == "self" {
            clean_args.args.remove(index);
        }
        if arg_name == "cls" {
            clean_args.args.remove(index);
        }
        if del_private_args && arg_name.starts_with("_") {
            clean_args.args.remove(index);
        }
    }

    for (index, arg) in args.kwonlyargs.iter().enumerate() {
        let arg_name = arg.def.arg.trim();
        if del_private_args && arg_name.starts_with("_") {
            clean_args.kwonlyargs.remove(index);
        }
    }
    for (index, arg) in args.posonlyargs.iter().enumerate() {
        let arg_name = arg.def.arg.trim();
        if arg_name == "self" {
            clean_args.posonlyargs.remove(index);
        }
        if arg_name == "cls" {
            clean_args.posonlyargs.remove(index);
        }
        if del_private_args && arg_name.starts_with("_") {
            clean_args.posonlyargs.remove(index);
        }
    }
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

        let raise_statements: &Vec<StmtRaise> = &function.raises;

        if ((raise_statements.len() == 1 && raise_statements.first().unwrap().exc.is_none())
            || raise_statements.is_empty())
            && function.docstring.clone().unwrap().has_raises()
        {
            let mut _range = function.def.range();
            let raise_lines =
                find_string_in_text_range(file_contents, _range, vec!["Raise:", "Raises:"]);
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

        let return_statements: &Vec<StmtReturn> = &function.returns;

        if ((return_statements.len() == 1 && return_statements.first().unwrap().value.is_none())
            || return_statements.is_empty())
            && function.docstring.clone().unwrap().has_returns()
        {
            let mut _range = function.def.range();
            let return_lines = find_string_in_text_range(file_contents, _range, vec!["Returns:"]);
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
        // ignore if function doesn't have returns
        let raise_statements: &Vec<StmtRaise> = &function.raises;
        if raise_statements.is_empty() {
            continue;
        }
        if function.docstring.is_none() {
            continue;
        }

        if !function.docstring.clone().unwrap().has_raises() {
            for ret in raise_statements {
                if ret.exc.is_some() {
                    let _range = &ret.range;

                    let (line, line_location) =
                        find_line_and_column(file_contents, _range.start().to_usize()).unwrap();
                    problem_functions.push(format_problem(
                        line - 1,
                        line_location,
                        raises_section_not_in_docstr_msg(),
                    ));
                }
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
                    find_line_and_column(file_contents, _range.start().to_usize()).unwrap();
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
                        find_line_and_column(file_contents, _range.start().to_usize()).unwrap();
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

fn generate_rules_output(
    file_contents: &str,
    things: &DocstringCollector,
    is_test_file: bool,
) -> Vec<String> {
    // DC0010: docstring missing on a function/ method/ class
    let mut problem_functions: Vec<String> = Vec::new();

    // DC0010: docstring missing on a function/ method/ class
    problem_functions.extend(check_functions_for_missing_docstring(
        &things.function_infos,
        file_contents,
        is_test_file,
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
    for class_info in &things.class_infos {
        problem_functions.extend(check_functions_for_missing_docstring(
            &class_info.funcs,
            file_contents,
            is_test_file,
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
    }
    problem_functions
}

fn check_functions_for_missing_docstring(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        if should_skip_dont_skip_private(function, is_test_file) {
            continue;
        }

        if function.docstring.is_none() {
            let (line, line_location) =
                find_line_and_column(file_contents, function.def.range().start().to_usize())
                    .unwrap();

            problem_functions.push(format_problem(line, line_location, docstr_missing_msg()));
        }
    }

    problem_functions
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
