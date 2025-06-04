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

pub fn mult_args_sections_in_docstr_code() -> String {
    format!("{}022", ERROR_CODE_PREFIX)
}
pub fn mult_args_sections_in_docstr_msg(found: &str) -> String {
    format!(
        "{} a docstring should only contain a single arguments section, found {}{}{}",
        mult_args_sections_in_docstr_code(),
        found,
        MORE_INFO_BASE,
        mult_args_sections_in_docstr_code().to_lowercase()
    )
}
pub fn arg_not_in_docstr_code() -> String {
    format!("{}023", ERROR_CODE_PREFIX)
}
pub fn arg_not_in_docstr_msg(_arg: &str) -> String {
    format!(
        "{} \"{}\" argument should be described in the docstring{}{}",
        arg_not_in_docstr_code(),
        _arg,
        MORE_INFO_BASE,
        arg_not_in_docstr_code().to_lowercase()
    )
}

pub fn arg_in_docstr_code() -> String {
    format!("{}024", ERROR_CODE_PREFIX)
}
pub fn arg_in_docstr_msg(_arg: &str) -> String {
    format!(
        "{} \"{}\" argument should not be described in the docstring{}{}",
        arg_in_docstr_code(),
        _arg,
        MORE_INFO_BASE,
        arg_in_docstr_code().to_lowercase()
    )
}

pub fn duplicate_arg_in_docstr_code() -> String {
    format!("{}025", ERROR_CODE_PREFIX)
}
pub fn duplicate_arg_msg(_arg: &str) -> String {
    format!(
        "{} \"{}\" argument documented multiple times{}{}",
        duplicate_arg_in_docstr_code(),
        _arg,
        MORE_INFO_BASE,
        duplicate_arg_in_docstr_code().to_lowercase()
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
pub fn mult_returns_sections_in_docstr_msg(found: &str) -> String {
    format!(
        "{} a docstring should only contain a single returns section, found {}{}{}",
        mult_returns_sections_in_docstr_code(),
        found,
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
pub fn mult_yields_sections_in_docstr_msg(found: &str) -> String {
    format!(
        "{} a docstring should only contain a single yields section, found {}{}{}",
        mult_yields_sections_in_docstr_code(),
        found,
        MORE_INFO_BASE,
        mult_yields_sections_in_docstr_code().to_lowercase()
    )
}

pub fn raises_section_not_in_docstr_code() -> String {
    format!("{}050", ERROR_CODE_PREFIX)
}
pub fn raises_section_not_in_docstr_msg() -> String {
    format!(
        "{} a function/ method that raises an exception should have the raises section in the docstring {}{}",
        raises_section_not_in_docstr_code(),
        MORE_INFO_BASE,
        raises_section_not_in_docstr_code().to_lowercase()
    )
}
pub fn raises_section_in_docstr_code() -> String {
    format!("{}051", ERROR_CODE_PREFIX)
}
pub fn raises_section_in_docstr_msg() -> String {
    format!(
        "{} a function/ method that does not raise an exception should not have the raises section in the docstring {}{}",
        raises_section_in_docstr_code(),
        MORE_INFO_BASE,
        raises_section_in_docstr_code().to_lowercase()
    )
}

pub fn mult_raises_sections_in_docstr_code() -> String {
    format!("{}052", ERROR_CODE_PREFIX)
}
pub fn mult_raises_sections_in_docstr_msg(found: &str) -> String {
    format!(
        "{} a docstring should only contain a single raises section, found {}{}{}",
        mult_raises_sections_in_docstr_code(),
        found,
        MORE_INFO_BASE,
        mult_raises_sections_in_docstr_code().to_lowercase()
    )
}

pub fn exc_not_in_docstr_code() -> String {
    format!("{}053", ERROR_CODE_PREFIX)
}
pub fn exc_not_in_docstr_msg(_raise: &str) -> String {
    format!(
        "{} \"{}\" exception should be described in the docstring{}{}",
        exc_not_in_docstr_code(),
        _raise,
        MORE_INFO_BASE,
        exc_not_in_docstr_code().to_lowercase()
    )
}

pub fn exc_in_docstr_code() -> String {
    format!("{}054", ERROR_CODE_PREFIX)
}
pub fn exc_in_docstr_msg(_raise: &str) -> String {
    format!(
        "{} \"{}\" exception should not be described in the docstring{}{}",
        exc_in_docstr_code(),
        _raise,
        MORE_INFO_BASE,
        exc_in_docstr_code().to_lowercase()
    )
}

pub fn re_raise_no_exc_in_docstr_code() -> String {
    format!("{}055", ERROR_CODE_PREFIX)
}
pub fn re_raise_no_exc_in_docstr_msg() -> String {
    format!(
        "{} a function/ method that re-raises exceptions should describe at least one exception in the raises section of the docstring{}{}",
        re_raise_no_exc_in_docstr_code(),
        MORE_INFO_BASE,
        re_raise_no_exc_in_docstr_code().to_lowercase()
    )
}
