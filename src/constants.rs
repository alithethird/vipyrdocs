pub const ERROR_CODE_PREFIX: &str = "D";
pub const MORE_INFO_BASE: &str = " (more info: https://example.com/";

pub fn docstr_missing_code() -> String {
    format!("{}010", ERROR_CODE_PREFIX)
}
pub fn docstr_missing_msg() -> String {
    format!(
        "{} docstring should be defined for a function/ method/ class{}{}",
        docstr_missing_code(),
        MORE_INFO_BASE,
        docstr_missing_code().to_lowercase()
    )
}

pub fn args_section_not_in_docstr_code() -> String {
    format!("{}020", ERROR_CODE_PREFIX)
}
pub fn args_section_not_in_docstr_msg() -> String {
    format!(
        "{} a function/ method with arguments should have the arguments section in the docstring{}{}",
        args_section_not_in_docstr_code(),
        MORE_INFO_BASE,
        args_section_not_in_docstr_code().to_lowercase()
    )
}

pub fn args_section_in_docstr_code() -> String {
    format!("{}021", ERROR_CODE_PREFIX)
}
pub fn args_section_in_docstr_msg() -> String {
    format!(
        "{} a function/ method without arguments should not have the arguments section in the docstring{}{}",
        args_section_in_docstr_code(),
        MORE_INFO_BASE,
        args_section_in_docstr_code().to_lowercase()
    )
}

pub fn returns_section_not_in_docstr_code() -> String {
    format!("{}030", ERROR_CODE_PREFIX)
}

pub fn returns_section_not_in_docstr_msg() -> String {
    format!(
        "{} function/ method that returns a value should have the returns section in the docstring{}{}",
        returns_section_not_in_docstr_code(),
        MORE_INFO_BASE,
        returns_section_not_in_docstr_code().to_lowercase()
    )
}

pub fn returns_section_in_docstr_code() -> String {
    format!("{}031", ERROR_CODE_PREFIX)
}
pub fn returns_section_in_docstr_msg() -> String {
    format!(
        "{} function/ method that does not return a value should not have the returns section in the docstring{}{}",
        returns_section_in_docstr_code(),
        MORE_INFO_BASE,
        returns_section_in_docstr_code().to_lowercase()
    )
}

pub fn mult_returns_sections_in_docstr_code() -> String {
    format!("{}032", ERROR_CODE_PREFIX)
}
pub fn mult_returns_sections_in_docstr_msg(found: String) -> String {
    format!(
        "{} a docstring should only contain a single returns section, found {}{}{}",
        mult_returns_sections_in_docstr_code(),
        found.to_string(),
        MORE_INFO_BASE,
        mult_returns_sections_in_docstr_code().to_lowercase()
    )
}

pub fn yields_section_not_in_docstr_code() -> String {
    format!("{}040", ERROR_CODE_PREFIX)
}
pub fn yields_section_not_in_docstr_msg() -> String {
    format!(
        "{} function/ method that yields a value should have the yields section in the docstring{}{}",
        yields_section_not_in_docstr_code(),
        MORE_INFO_BASE,
        yields_section_not_in_docstr_code().to_lowercase()
    )
}

pub fn yields_section_in_docstr_code() -> String {
    format!("{}041", ERROR_CODE_PREFIX)
}
pub fn yields_section_in_docstr_msg() -> String {
    format!(
        "{} function/ method that does not yield a value should not have the yields section in the docstring{}{}",
        yields_section_in_docstr_code(),
        MORE_INFO_BASE,
        yields_section_in_docstr_code().to_lowercase()
    )
}

pub fn mult_yields_sections_in_docstr_code() -> String {
    format!("{}042", ERROR_CODE_PREFIX)
}
pub fn mult_yields_sections_in_docstr_msg(found: String) -> String {
    format!(
        "{} a docstring should only contain a single yields section, found {}{}{}",
        mult_yields_sections_in_docstr_code(),
        found.to_string(),
        MORE_INFO_BASE,
        mult_yields_sections_in_docstr_code().to_lowercase()
    )
}
