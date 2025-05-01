use crate::constants::{docstr_missing_msg, mult_returns_sections_in_docstr_msg, returns_section_in_docstr_msg, returns_section_not_in_docstr_msg, yields_section_not_in_docstr_msg};
use crate::plugin::{get_result, DocstringCollector, FunctionDefKind, FunctionInfo, YieldKind};
use pyo3::prelude::*;
use rustpython_ast::text_size::TextRange;
use rustpython_ast::{Expr, ExprAttribute, ExprCall, ExprYield, StmtReturn, TextSize};
use std::fs;

use pyo3;

fn read_file(file_name: &str) -> String {
    // Read the file and return the contents
    let file_contents = fs::read_to_string(&file_name).unwrap_or_default();

    return file_contents;
}

fn is_test_file(file_name: Option<&str>) -> bool {
    if file_name.is_some() {
        let file_name = file_name.unwrap().split('/').last().unwrap();

        if file_name.starts_with("test_") {
            return true;
        } else if file_name.starts_with("conftest.py") {
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

    return apply_rules(code.as_str(), file_name);
}

#[pyfunction]
#[pyo3(signature = (code, file_name=None))]
pub fn apply_rules(code: &str, file_name: Option<&str>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    let things = get_result(&code, file_name);

    let test_file = is_test_file(file_name);
    // apply the rules
    // DC0010: docstring missing on a function/ method/ class
    output.extend(check_for_missing_docstring(&code, &things, test_file));

    // DCO030: function/ method that returns a value does not have the returns section in the docstring.
    output.extend(check_for_missing_returns_section(&code, &things, test_file));

    // DC031: function/ method that does not return a value should not
    // have the returns section in the docstring
    output.extend(check_for_extra_returns_section(&code, &things, test_file));

    // DC032: a docstring should only contain a single returns
    // section, found %s
    output.extend(check_for_multiple_returns_section(&code, &things, test_file));

    // DC040: function/ method that yields a value should have the
    // yields section in the docstring
    output.extend(check_for_missing_yields_section(&code, &things, test_file));
    

    return output;
}

/// Finds the (line, column) of `target_string` if it exists within the specified TextRange of `s`.
/// Returns (line_number, column_number) on success. Both are 1-based.
pub fn find_string_in_text_range(
    s: &str,
    range: TextRange,
    target_strings: Vec<&str>,
) -> Vec<(usize, usize, String)> {
    let start = usize::try_from(range.start().to_u32()).unwrap();
    let end = usize::try_from(range.end().to_u32()).unwrap();

    let sub = &s[start..end];
    let mut positions: Vec<(usize, usize, String)> = Vec::new();

    for target in target_strings {
        let mut offset = 0;
        while let Some(pos) = sub[offset..].find(target) {
            let absolute_pos = start + offset + pos;

            // Find line and column
            let before = &s[..absolute_pos];
            let line_number = before.lines().count(); // 1-based

            let column_number = before
                .rfind('\n')
                .map(|idx| absolute_pos - idx - 1)
                .unwrap_or(absolute_pos);

            positions.push((line_number - 2, column_number, target.to_string())); // line_number -2 to make it 0-based and not count """
            offset += pos + 1; // Move past the current match
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


fn check_for_multiple_returns_section(
    file_contents: &str,
    things: &DocstringCollector,
    is_test_file: bool,
) -> Vec<String> {
    // DCO032: function/ method that returns a value does not have the returns section in the docstring.
    let mut problem_functions: Vec<String> = Vec::new();

    problem_functions.extend(check_functions_for_multiple_returns_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    for class_infos in &things.class_infos {
        problem_functions.extend(check_functions_for_multiple_returns_section(
            &class_infos.funcs,
            file_contents,
            is_test_file,
        ));
    }

    problem_functions
}

fn check_for_extra_returns_section(
    file_contents: &str,
    things: &DocstringCollector,
    is_test_file: bool,
) -> Vec<String> {
    // DCO031: function/ method that returns a value does not have the returns section in the docstring.
    let mut problem_functions: Vec<String> = Vec::new();

    problem_functions.extend(check_functions_for_extra_returns_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    for class_infos in &things.class_infos {
        problem_functions.extend(check_functions_for_extra_returns_section(
            &class_infos.funcs,
            file_contents,
            is_test_file,
        ));
    }

    problem_functions
}

fn check_for_missing_yields_section(
    file_contents: &str,
    things: &DocstringCollector,
    is_test_file: bool,
) -> Vec<String> {
    // DCO040
    let mut problem_functions: Vec<String> = Vec::new();

    problem_functions.extend(check_functions_for_missing_yields_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    for class_infos in &things.class_infos {
        problem_functions.extend(check_functions_for_missing_yields_section(
            &class_infos.funcs,
            file_contents,
            is_test_file,
        ));
    }

    problem_functions
}

fn check_for_missing_returns_section(
    file_contents: &str,
    things: &DocstringCollector,
    is_test_file: bool,
) -> Vec<String> {
    // DCO030: function/ method that returns a value does not have the returns section in the docstring.
    let mut problem_functions: Vec<String> = Vec::new();

    problem_functions.extend(check_functions_for_missing_returns_section(
        &things.function_infos,
        file_contents,
        is_test_file,
    ));
    for class_infos in &things.class_infos {
        problem_functions.extend(check_functions_for_missing_returns_section(
            &class_infos.funcs,
            file_contents,
            is_test_file,
        ));
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
        // ignore overloads
        // Skip function if *any* decorator is an overload
        if is_overload(&function) {
            continue;
        }
        let func_name = function.def.name().to_string();
        if func_name.starts_with("test_") && is_test_file {
            continue;
        }
        if is_fixture(function.def.clone()) && is_test_file {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }
        
        if function.docstring.clone().unwrap().get_returns().len() > 1 {

            let mut _range = function.def.range();
            let return_lines =
                find_string_in_text_range(file_contents, _range.clone(), vec!["Return:","Returns:"]);
            if return_lines.len() < 2 {
                continue;
            }
            let mut founds: Vec<String> = Vec::new();
            for (_, _, found) in &return_lines {
                // the latest char is a : which we do not want
                founds.push(found[..found.len() - 1].to_string());
            }
            let (line, line_location, found) = return_lines.first().unwrap().to_owned();
                problem_functions.push(format_problem(
                    line,
                    line_location,
                    mult_returns_sections_in_docstr_msg(founds.join(",").to_string())
                ));

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
        // ignore overloads
        // Skip function if *any* decorator is an overload
        if is_overload(&function) {
            continue;
        }
        let func_name = function.def.name().to_string();
        if func_name.starts_with("test_") && is_test_file {
            continue;
        }
        if is_fixture(function.def.clone()) && is_test_file {
            continue;
        }

        // ignore if function doesn't have docstrings
        if function.docstring.is_none() {
            continue;
        }

        let return_statements: &Vec<StmtReturn> = &function.returns;

        if (return_statements.len() == 1 && return_statements.first().unwrap().value == None)
            || return_statements.is_empty()
        {
            if function.docstring.clone().unwrap().has_returns() {
                let mut _range = function.def.range();
                let return_lines =
                    find_string_in_text_range(file_contents, _range.clone(), vec!["Returns:"]);
                if return_lines.is_empty() {
                    continue;
                }
                for (line, line_location, target) in return_lines
                {
                    problem_functions.push(format_problem(
                        line,
                        line_location,
                        returns_section_in_docstr_msg(),
                    ));
                }            }
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
        // ignore overloads
        // Skip function if *any* decorator is an overload
        if is_overload(&function) {
            continue;
        }
        let func_name = function.def.name().to_string();
        if func_name.starts_with("test_") && is_test_file {
            continue;
        }
        if is_fixture(function.def.clone()) && is_test_file {
            continue;
        }
        if func_name.starts_with("_") {
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

        if !function.docstring.clone().unwrap().has_returns() {
            for _yield in yield_statements {
                    let _range = &_yield.range();

                let start = usize::try_from(_range.start().to_u32()).unwrap();
                let end = usize::try_from(_range.end().to_u32()).unwrap();

                let sub = &file_contents[start..end];
                // if doesn't yield any value    
                if sub == "yield"{
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

fn check_functions_for_missing_returns_section(
    function_infos: &Vec<FunctionInfo>,
    file_contents: &str,
    is_test_file: bool,
) -> Vec<String> {
    let mut problem_functions: Vec<String> = Vec::new();

    for function in function_infos {
        // ignore overloads
        // Skip function if *any* decorator is an overload
        if is_overload(&function) {
            continue;
        }
        let func_name = function.def.name().to_string();
        if func_name.starts_with("test_") && is_test_file {
            continue;
        }
        if is_fixture(function.def.clone()) && is_test_file {
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

fn check_for_missing_docstring(
    file_contents: &str,
    things: &DocstringCollector,
    is_test_file: bool,
) -> Vec<String> {
    // DC0010: docstring missing on a function/ method/ class
    let mut problem_functions: Vec<String> = Vec::new();

    problem_functions.extend(check_functions_for_missing_docstring(
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
        // ignore overloads
        // Skip function if *any* decorator is an overload
        if is_overload(&function) {
            continue;
        }
        if function.docstring.is_none() {
            let func_name = function.def.name().to_string();
            if func_name.starts_with("test_") && is_test_file {
                continue;
            }
            if is_fixture(function.def.clone()) && is_test_file {
                continue;
            }
            let (line, line_location) =
                find_line_and_column(file_contents, function.def.range().start().to_usize())
                    .unwrap();

            problem_functions.push(format_problem(line, line_location, docstr_missing_msg()));
        }
    }

    problem_functions
}

fn is_overload(function: &FunctionInfo) -> bool {
    for decorator in function.def.decorator_list() {
        if decorator.is_name_expr() {
            let id = &decorator.as_name_expr().unwrap().id;
            if id.to_string() == "overload" {
                return true;
            }
        }

        if decorator.is_call_expr() {
            let call: &ExprCall = decorator.as_call_expr().unwrap();
            if let Some(name_expr) = call.func.as_name_expr() {
                let id = &name_expr.id;
                if id.to_string() == "overload" {
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
//
// fn has_fixture_attribute(decorator: &Expr) -> bool {
//     if decorator.is_attribute_expr() {
//         let attr: &ExprAttribute = decorator.as_attribute_expr().unwrap();
//         if attr.attr.to_string() == "fixture" {
//             return true;
//         }
//     }
//     false
// }

fn is_fixture(function: FunctionDefKind) -> bool {
    let mut is_fixture = false;

    for decorator in function.decorator_list() {
        if decorator.is_name_expr() {
            if is_name_fixture_decorator(decorator) {
                is_fixture = true;
                break;
            }
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
                if id.to_string() == "fixture" {
                    is_fixture = true;
                    break;
                }
            }
        }
        if decorator.is_attribute_expr() {
            let attr: &ExprAttribute = decorator.as_attribute_expr().unwrap();
            if attr.attr.to_string() == "fixture" {
                is_fixture = true;
                break;
            }
        }
    }

    return is_fixture;
}

fn is_name_fixture_decorator(decorator: &Expr) -> bool {
    let id = &decorator.as_name_expr().unwrap().id;
    if id.to_string().to_lowercase() == "fixture" {
        return true;
    }
    false
}
