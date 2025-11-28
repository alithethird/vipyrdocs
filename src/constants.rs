pub const ERROR_CODE_PREFIX: &str = "D";
pub const MORE_INFO_BASE: &str = "https://github.com/alithethird/vipyrdocs/wiki/The-ruleset#"; //" (more info: https://example.com/";

fn create_doc_link(_doc: String, rule_explanation: &str) -> String{
    format!("{}{}{}",
        MORE_INFO_BASE,
        _doc.to_lowercase(),
        rule_explanation,
           // TODO: Update this when you write the docs
        )
}

pub fn docstr_missing_code() -> String {
    format!("{}010", ERROR_CODE_PREFIX)
}
pub fn docstr_missing_msg() -> String {
    format!(
        "{} docstring should be defined for a function/ class {}",
        docstr_missing_code(),
        create_doc_link(docstr_missing_code(), "-missing-docstring")
    )
}

pub fn args_section_not_in_docstr_code() -> String {
    format!("{}020", ERROR_CODE_PREFIX)
}
pub fn args_section_not_in_docstr_msg() -> String {
    format!(
        "{} a function with arguments should have the arguments section in the docstring {}",
        args_section_not_in_docstr_code(),
        create_doc_link(args_section_not_in_docstr_code(), "-missing-args-section")
    )
}

pub fn args_section_in_docstr_code() -> String {
    format!("{}021", ERROR_CODE_PREFIX)
}
pub fn args_section_in_docstr_msg() -> String {
    format!(
        "{} a function without arguments should not have the arguments section in the docstring {}",
        args_section_in_docstr_code(),
        create_doc_link(args_section_in_docstr_code(), "-extra-args-section")
    )
}

pub fn mult_args_sections_in_docstr_code() -> String {
    format!("{}022", ERROR_CODE_PREFIX)
}
pub fn mult_args_sections_in_docstr_msg(found: &str) -> String {
    format!(
        "{} a docstring should only contain a single arguments section, found {} {}",
        mult_args_sections_in_docstr_code(),
        found,
        create_doc_link(mult_args_sections_in_docstr_code(), "-multiple-args-section")
    )
}
pub fn arg_not_in_docstr_code() -> String {
    format!("{}023", ERROR_CODE_PREFIX)
}
pub fn arg_not_in_docstr_msg(_arg: &str) -> String {
    format!(
        "{} \"{}\" argument should be described in the docstring {}",
        arg_not_in_docstr_code(),
        _arg,
        create_doc_link(arg_not_in_docstr_code(), "-missing-argument-in-the-section")
    )
}

pub fn arg_in_docstr_code() -> String {
    format!("{}024", ERROR_CODE_PREFIX)
}
pub fn arg_in_docstr_msg(_arg: &str) -> String {
    format!(
        "{} \"{}\" argument should not be described in the docstring {}",
        arg_in_docstr_code(),
        _arg,
        create_doc_link(arg_in_docstr_code(), "-mysterious-argument-in-the-section")
    )
}

pub fn duplicate_arg_in_docstr_code() -> String {
    format!("{}025", ERROR_CODE_PREFIX)
}
pub fn duplicate_arg_msg(_arg: &str) -> String {
    format!(
        "{} \"{}\" argument documented multiple times {}",
        duplicate_arg_in_docstr_code(),
        _arg,
        create_doc_link(duplicate_arg_in_docstr_code(), "-duplicate-arguments-in-the-section")
    )
}
pub fn returns_section_not_in_docstr_code() -> String {
    format!("{}030", ERROR_CODE_PREFIX)
}

pub fn returns_section_not_in_docstr_msg() -> String {
    format!(
        "{} function that returns a value should have the returns section in the docstring {}",
        returns_section_not_in_docstr_code(),
        create_doc_link(returns_section_not_in_docstr_code(), "-missing-returns-section")
    )
}

pub fn returns_section_in_docstr_code() -> String {
    format!("{}031", ERROR_CODE_PREFIX)
}
pub fn returns_section_in_docstr_msg() -> String {
    format!(
        "{} function that does not return a value should not have the returns section in the docstring {}",
        returns_section_in_docstr_code(),
        create_doc_link(returns_section_in_docstr_code(), "-extra-returns-section")
    )
}

pub fn mult_returns_sections_in_docstr_code() -> String {
    format!("{}032", ERROR_CODE_PREFIX)
}
pub fn mult_returns_sections_in_docstr_msg(found: &str) -> String {
    format!(
        "{} a docstring should only contain a single returns section, found {} {}",
        mult_returns_sections_in_docstr_code(),
        found,
        create_doc_link(mult_returns_sections_in_docstr_code(), "-multiple-returns-section")
    )
}

pub fn yields_section_not_in_docstr_code() -> String {
    format!("{}040", ERROR_CODE_PREFIX)
}
pub fn yields_section_not_in_docstr_msg() -> String {
    format!(
        "{} function that yields a value should have the yields section in the docstring {}",
        yields_section_not_in_docstr_code(),
        create_doc_link(yields_section_not_in_docstr_code(), "-missing-yields-section")
    )
}

pub fn yields_section_in_docstr_code() -> String {
    format!("{}041", ERROR_CODE_PREFIX)
}
pub fn yields_section_in_docstr_msg() -> String {
    format!(
        "{} function that does not yield a value should not have the yields section in the docstring {}",
        yields_section_in_docstr_code(),
        create_doc_link(yields_section_in_docstr_code(), "-extra-yields-section")
    )
}

pub fn mult_yields_sections_in_docstr_code() -> String {
    format!("{}042", ERROR_CODE_PREFIX)
}
pub fn mult_yields_sections_in_docstr_msg(found: &str) -> String {
    format!(
        "{} a docstring should only contain a single yields section, found {} {}",
        mult_yields_sections_in_docstr_code(),
        found,
        create_doc_link(mult_yields_sections_in_docstr_code(), "-multiple-yields-section")
    )
}

pub fn raises_section_not_in_docstr_code() -> String {
    format!("{}050", ERROR_CODE_PREFIX)
}
pub fn raises_section_not_in_docstr_msg() -> String {
    format!(
        "{} a function that raises an exception should have the raises section in the docstring {}",
        raises_section_not_in_docstr_code(),
        create_doc_link(raises_section_not_in_docstr_code(), "-missing-raises-section")
    )
}
pub fn raises_section_in_docstr_code() -> String {
    format!("{}051", ERROR_CODE_PREFIX)
}
pub fn raises_section_in_docstr_msg() -> String {
    format!(
        "{} a function that does not raise an exception should not have the raises section in the docstring {}",
        raises_section_in_docstr_code(),
        create_doc_link(raises_section_in_docstr_code(), "-extra-raises-section")
    )
}

pub fn mult_raises_sections_in_docstr_code() -> String {
    format!("{}052", ERROR_CODE_PREFIX)
}
pub fn mult_raises_sections_in_docstr_msg(found: &str) -> String {
    format!(
        "{} a docstring should only contain a single raises section, found {} {}",
        mult_raises_sections_in_docstr_code(),
        found,
        create_doc_link(mult_raises_sections_in_docstr_code(), "-multiple-raises-section")
    )
}

pub fn exc_not_in_docstr_code() -> String {
    format!("{}053", ERROR_CODE_PREFIX)
}
pub fn exc_not_in_docstr_msg(_raise: &str) -> String {
    format!(
        "{} \"{}\" exception should be described in the docstring {}",
        exc_not_in_docstr_code(),
        _raise,
        create_doc_link(exc_not_in_docstr_code(), "-missing-exception-in-the-section")
    )
}

pub fn exc_in_docstr_code() -> String {
    format!("{}054", ERROR_CODE_PREFIX)
}
pub fn exc_in_docstr_msg(_raise: &str) -> String {
    format!(
        "{} \"{}\" exception should not be described in the docstring {}",
        exc_in_docstr_code(),
        _raise,
        create_doc_link(exc_in_docstr_code(), "-mysterious-exception-in-the-section")
    )
}

pub fn re_raise_no_exc_in_docstr_code() -> String {
    format!("{}055", ERROR_CODE_PREFIX)
}
pub fn re_raise_no_exc_in_docstr_msg() -> String {
    format!(
        "{} a function that re-raises exceptions should describe at least one exception in the raises section of the docstring {}",
        re_raise_no_exc_in_docstr_code(),
        create_doc_link(re_raise_no_exc_in_docstr_code(), "-missing-re-raised-exception-in-the-section")
    )
}

pub fn duplicate_exc_code() -> String {
    format!("{}056", ERROR_CODE_PREFIX)
}
pub fn duplicate_exc_msg(_raise: &str) -> String {
    format!(
        "{} \"{}\" exception documented multiple times {}",
        duplicate_exc_code(),
        _raise,
        create_doc_link(duplicate_exc_code(), "-duplicate-exception-in-the-section")
    )
}

pub fn attrs_section_not_in_docstr_code() -> String {
    format!("{}060", ERROR_CODE_PREFIX)
}

pub fn attrs_section_not_in_docstr_msg() -> String {
    format!(
        "{} a class with attributes should have the attributes section in the docstring {}",
        attrs_section_not_in_docstr_code(),
        create_doc_link(attrs_section_not_in_docstr_code(), "-missing-attributes-section")
    )
}

pub fn attrs_section_in_docstr_code() -> String {
    format!("{}061", ERROR_CODE_PREFIX)
}

pub fn attrs_section_in_docstr_msg() -> String {
    format!(
        "{} a class without attributes should not have the attributes section in the docstring {}",
        attrs_section_in_docstr_code(),
        create_doc_link(attrs_section_in_docstr_code(), "-extra-attributes-section")
    )
}
pub fn mult_attrs_section_in_docstr_code() -> String {
    format!("{}062", ERROR_CODE_PREFIX)
}

pub fn mult_attrs_section_in_docstr_msg(_attribute: &str) -> String {
    format!(
        "{} a docstring should only contain a single attributes section, found {} {}",
        mult_attrs_section_in_docstr_code(),
        _attribute,
        create_doc_link(mult_attrs_section_in_docstr_code(), "-multiple-attributes-section")
    )
}

pub fn attr_not_in_docstr_code() -> String {
    format!("{}063", ERROR_CODE_PREFIX)
}

pub fn attr_not_in_docstr_msg(_attribute: &str) -> String {
    format!(
        "{} {} attribute/property should be described in the docstring {}",
        attr_not_in_docstr_code(),
        _attribute,
        create_doc_link(attr_not_in_docstr_code(), "-missing-attributes-in-the-section")
    )
}

pub fn attr_in_docstr_code() -> String {
    format!("{}064", ERROR_CODE_PREFIX)
}

pub fn attr_in_docstr_msg(_attribute: &str) -> String {
    format!(
        "{} {} attribute should not be described in the docstring {}",
        attr_in_docstr_code(),
        _attribute,
        create_doc_link(attr_in_docstr_code(), "-mysterious-attributes-in-the-section")
    )
}

pub fn duplicate_attr_docstr_code() -> String {
    format!("{}065", ERROR_CODE_PREFIX)
}

pub fn duplicate_attr_docstr_msg(_attribute: &str) -> String {
    format!(
        "{} {} attribute documented multiple times {}",
        duplicate_attr_docstr_code(),
        _attribute,
        create_doc_link(attr_in_docstr_code(), "-duplicate-attributes-in-the-section")
    )
}
