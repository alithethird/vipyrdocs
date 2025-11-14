use crate::plugin::{ClassInfo, FunctionInfo};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AbstractMethodInfo {
    pub class_name: String,
    pub method_name: String,
    pub has_returns: bool,
    pub has_raises: bool,
    pub has_yields: bool,
    pub file_path: String,
}

#[derive(Debug, Clone)]
pub struct ConcreteMethodInfo {
    pub class_name: String,
    pub method_name: String,
    pub base_classes: Vec<String>,
    pub has_returns: bool,
    pub has_raises: bool,
    pub has_yields: bool,
    pub has_docstring: bool,
    pub file_path: String,
    pub line: usize,
}

pub struct InheritanceTracker {
    /// Maps (class_name, method_name) -> AbstractMethodInfo
    abstract_methods: HashMap<(String, String), AbstractMethodInfo>,
    /// List of all concrete methods that need validation
    concrete_methods: Vec<ConcreteMethodInfo>,
}

impl Default for InheritanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl InheritanceTracker {
    pub fn new() -> Self {
        Self {
            abstract_methods: HashMap::new(),
            concrete_methods: Vec::new(),
        }
    }

    /// Register an abstract method from parsing a file
    pub fn register_abstract_method(&mut self, info: AbstractMethodInfo) {
        let key = (info.class_name.clone(), info.method_name.clone());
        self.abstract_methods.insert(key, info);
    }

    /// Register a concrete method that may implement an abstract method
    pub fn register_concrete_method(&mut self, info: ConcreteMethodInfo) {
        self.concrete_methods.push(info);
    }

    /// Check all concrete methods against abstract methods and return violations
    pub fn validate(&self) -> Vec<InheritanceViolation> {
        let mut violations = Vec::new();

        for concrete in &self.concrete_methods {
            // Skip validation if the concrete method has no docstring
            // (it inherits the docstring from the abstract method)
            if !concrete.has_docstring {
                continue;
            }

            // Check each base class
            for base_class in &concrete.base_classes {
                let key = (base_class.clone(), concrete.method_name.clone());

                if let Some(abstract_method) = self.abstract_methods.get(&key) {
                    // Found an abstract method that this concrete method implements

                    // Check Returns
                    if abstract_method.has_returns && !concrete.has_returns {
                        violations.push(InheritanceViolation {
                            file_path: concrete.file_path.clone(),
                            line: concrete.line,
                            method_name: concrete.method_name.clone(),
                            class_name: concrete.class_name.clone(),
                            base_class: base_class.clone(),
                            violation_type: ViolationType::Returns,
                        });
                    }

                    // Check Raises
                    if abstract_method.has_raises && !concrete.has_raises {
                        violations.push(InheritanceViolation {
                            file_path: concrete.file_path.clone(),
                            line: concrete.line,
                            method_name: concrete.method_name.clone(),
                            class_name: concrete.class_name.clone(),
                            base_class: base_class.clone(),
                            violation_type: ViolationType::Raises,
                        });
                    }

                    // Check Yields
                    if abstract_method.has_yields && !concrete.has_yields {
                        violations.push(InheritanceViolation {
                            file_path: concrete.file_path.clone(),
                            line: concrete.line,
                            method_name: concrete.method_name.clone(),
                            class_name: concrete.class_name.clone(),
                            base_class: base_class.clone(),
                            violation_type: ViolationType::Yields,
                        });
                    }
                }
            }
        }

        violations
    }

    /// Get a set of (file_path, class_name, method_name) for methods that implement abstract methods
    /// These methods should inherit docstrings from their abstract base methods
    pub fn get_methods_implementing_abstract(
        &self,
    ) -> std::collections::HashSet<(String, String, String)> {
        use std::collections::HashSet;
        let mut implementing_methods = HashSet::new();

        for concrete in &self.concrete_methods {
            // Check each base class
            for base_class in &concrete.base_classes {
                let key = (base_class.clone(), concrete.method_name.clone());

                // If this method implements an abstract method, add it to the set
                if self.abstract_methods.contains_key(&key) {
                    implementing_methods.insert((
                        concrete.file_path.clone(),
                        concrete.class_name.clone(),
                        concrete.method_name.clone(),
                    ));
                    break; // No need to check other base classes for this method
                }
            }
        }

        implementing_methods
    }
}

#[derive(Debug, Clone)]
pub struct InheritanceViolation {
    pub file_path: String,
    pub line: usize,
    pub method_name: String,
    pub class_name: String,
    pub base_class: String,
    pub violation_type: ViolationType,
}

#[derive(Debug, Clone)]
pub enum ViolationType {
    Returns,
    Raises,
    Yields,
}

impl InheritanceViolation {
    pub fn to_error_message(&self) -> String {
        match self.violation_type {
            ViolationType::Returns => {
                format!(
                    "method '{}' in class '{}' implements abstract method from '{}' which documents a return value, but this implementation is missing a Returns section in the docstring",
                    self.method_name, self.class_name, self.base_class
                )
            }
            ViolationType::Raises => {
                format!(
                    "method '{}' in class '{}' implements abstract method from '{}' which documents exceptions, but this implementation is missing a Raises section in the docstring",
                    self.method_name, self.class_name, self.base_class
                )
            }
            ViolationType::Yields => {
                format!(
                    "method '{}' in class '{}' implements abstract method from '{}' which documents yields, but this implementation is missing a Yields section in the docstring",
                    self.method_name, self.class_name, self.base_class
                )
            }
        }
    }

    pub fn get_error_code(&self) -> &str {
        match self.violation_type {
            ViolationType::Returns => "D070",
            ViolationType::Raises => "D071",
            ViolationType::Yields => "D072",
        }
    }
}

/// Extract base class names from a class definition
pub fn extract_base_classes(class_info: &ClassInfo) -> Vec<String> {
    let mut base_classes = Vec::new();

    for base in &class_info.def.bases {
        if let Some(name) = extract_name_from_expr(base) {
            base_classes.push(name);
        }
    }

    base_classes
}

/// Extract a simple name from an expression (handles Name and Attribute expressions)
fn extract_name_from_expr(expr: &rustpython_ast::Expr) -> Option<String> {
    use rustpython_ast::Expr;

    match expr {
        Expr::Name(name_expr) => Some(name_expr.id.to_string()),
        Expr::Attribute(attr_expr) => {
            // For cases like abc.ABC, we just want "ABC"
            Some(attr_expr.attr.to_string())
        }
        _ => None,
    }
}

#[allow(dead_code)]
/// Check if a function has abstractmethod decorator
pub fn is_abstractmethod_local(function: &FunctionInfo) -> bool {
    use rustpython_ast::{ExprAttribute, ExprCall};

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
