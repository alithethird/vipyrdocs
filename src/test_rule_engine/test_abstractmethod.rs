#[cfg(test)]
use crate::constants::{raises_section_in_docstr_msg, returns_section_in_docstr_msg};
use crate::rule_engine::lint_file;

fn general_test(code: &str, expected: Vec<String>) {
    let output = lint_file(code, None);
    println!("{:#?}", output);
    assert_eq!(output.len(), expected.len());
    for (index, exp) in expected.iter().enumerate() {
        assert_eq!(
            &output[index], exp,
            "Mismatch at output index {}: got `{}`, expected `{}`",
            index, output[index], exp
        );
    }
}

#[test]
fn test_abstractmethod_with_returns_section_no_error() {
    let code: &str = r#"
from abc import abstractmethod

class BaseClass:
    @abstractmethod
    def abstract_func(self, arg1):
        """Abstract function with a return value.
        
        Args:
            arg1: The first argument.
            
        Returns:
            Some value.
        """
        pass
"#;
    let expected: Vec<String> = vec![];
    general_test(code, expected);
}

#[test]
fn test_abstractmethod_with_raises_section_no_error() {
    let code: &str = r#"
from abc import abstractmethod

class BaseClass:
    @abstractmethod
    def abstract_func(self):
        """Abstract function that raises.
        
        Raises:
            ValueError: When something is wrong.
        """
        pass
"#;
    let expected: Vec<String> = vec![];
    general_test(code, expected);
}

#[test]
fn test_abstractmethod_with_yields_section_no_error() {
    let code: &str = r#"
from abc import abstractmethod

class BaseClass:
    @abstractmethod
    def abstract_func(self):
        """Abstract generator function.
        
        Yields:
            Some value.
        """
        pass
"#;
    let expected: Vec<String> = vec![];
    general_test(code, expected);
}

#[test]
fn test_abstractmethod_abc_module_qualifier() {
    let code: &str = r#"
from abc import ABC, abstractmethod

class BaseClass(ABC):
    @abstractmethod
    def abstract_func(self):
        """Abstract function.
        
        Returns:
            Some value.
        """
        pass
"#;
    let expected: Vec<String> = vec![];
    general_test(code, expected);
}

#[test]
fn test_abstractmethod_attribute_style() {
    let code: &str = r#"
import abc

class BaseClass:
    @abc.abstractmethod
    def abstract_func(self):
        """Abstract function.
        
        Returns:
            Some value.
        """
        pass
"#;
    let expected: Vec<String> = vec![];
    general_test(code, expected);
}

#[test]
fn test_non_abstractmethod_missing_returns_still_errors() {
    let code: &str = r#"
def function_1():
    """Docstring 1.

    Returns:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", returns_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_non_abstractmethod_missing_raises_still_errors() {
    let code: &str = r#"
def function_1():
    """Docstring 1.

    Raises:
    """
"#;
    let expected: Vec<String> = vec![format!("3:4 {}", raises_section_in_docstr_msg())];
    general_test(code, expected);
}

#[test]
fn test_abstractmethod_combined_with_other_decorators() {
    let code: &str = r#"
from abc import abstractmethod

class BaseClass:
    @property
    @abstractmethod
    def abstract_prop(self):
        """Abstract property.
        
        Returns:
            Some value.
        """
        pass
"#;
    let expected: Vec<String> = vec![];
    general_test(code, expected);
}

#[test]
fn test_abstractmethod_async_function() {
    let code: &str = r#"
from abc import abstractmethod

class BaseClass:
    @abstractmethod
    async def abstract_async_func(self):
        """Abstract async function.
        
        Returns:
            Some value.
        """
        pass
"#;
    let expected: Vec<String> = vec![];
    general_test(code, expected);
}
