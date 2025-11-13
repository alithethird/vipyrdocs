#[cfg(test)]
use crate::inheritance::{
    AbstractMethodInfo, ConcreteMethodInfo, InheritanceTracker, ViolationType,
};

#[test]
fn test_inheritance_tracker_returns_violation() {
    let mut tracker = InheritanceTracker::new();

    // Register an abstract method that documents Returns
    tracker.register_abstract_method(AbstractMethodInfo {
        class_name: "BaseClass".to_string(),
        method_name: "process".to_string(),
        has_returns: true,
        has_raises: false,
        has_yields: false,
        file_path: "/tmp/base.py".to_string(),
    });

    // Register a concrete method that doesn't document Returns
    tracker.register_concrete_method(ConcreteMethodInfo {
        class_name: "ImplClass".to_string(),
        method_name: "process".to_string(),
        base_classes: vec!["BaseClass".to_string()],
        has_returns: false, // Missing Returns!
        has_raises: false,
        has_yields: false,
        file_path: "/tmp/impl.py".to_string(),
        has_docstring: true,
        line: 10,
    });

    let violations = tracker.validate();
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].method_name, "process");
    assert_eq!(violations[0].class_name, "ImplClass");
    assert_eq!(violations[0].base_class, "BaseClass");
    assert!(matches!(
        violations[0].violation_type,
        ViolationType::MissingReturns
    ));
    assert_eq!(violations[0].get_error_code(), "D070");
}

#[test]
fn test_inheritance_tracker_raises_violation() {
    let mut tracker = InheritanceTracker::new();

    // Register an abstract method that documents Raises
    tracker.register_abstract_method(AbstractMethodInfo {
        class_name: "BaseClass".to_string(),
        method_name: "validate".to_string(),
        has_returns: false,
        has_raises: true,
        has_yields: false,
        file_path: "/tmp/base.py".to_string(),
    });

    // Register a concrete method that doesn't document Raises
    tracker.register_concrete_method(ConcreteMethodInfo {
        class_name: "ImplClass".to_string(),
        method_name: "validate".to_string(),
        base_classes: vec!["BaseClass".to_string()],
        has_returns: false,
        has_raises: false, // Missing Raises!
        has_yields: false,
        file_path: "/tmp/impl.py".to_string(),
        has_docstring: true,
        line: 20,
    });

    let violations = tracker.validate();
    assert_eq!(violations.len(), 1);
    assert!(matches!(
        violations[0].violation_type,
        ViolationType::MissingRaises
    ));
    assert_eq!(violations[0].get_error_code(), "D071");
}

#[test]
fn test_inheritance_tracker_yields_violation() {
    let mut tracker = InheritanceTracker::new();

    // Register an abstract method that documents Yields
    tracker.register_abstract_method(AbstractMethodInfo {
        class_name: "BaseClass".to_string(),
        method_name: "generate".to_string(),
        has_returns: false,
        has_raises: false,
        has_yields: true,
        file_path: "/tmp/base.py".to_string(),
    });

    // Register a concrete method that doesn't document Yields
    tracker.register_concrete_method(ConcreteMethodInfo {
        class_name: "ImplClass".to_string(),
        method_name: "generate".to_string(),
        base_classes: vec!["BaseClass".to_string()],
        has_returns: false,
        has_raises: false,
        has_yields: false, // Missing Yields!
        file_path: "/tmp/impl.py".to_string(),
        has_docstring: true,
        line: 30,
    });

    let violations = tracker.validate();
    assert_eq!(violations.len(), 1);
    assert!(matches!(
        violations[0].violation_type,
        ViolationType::MissingYields
    ));
    assert_eq!(violations[0].get_error_code(), "D072");
}

#[test]
fn test_inheritance_tracker_no_violation_when_documented() {
    let mut tracker = InheritanceTracker::new();

    // Register an abstract method
    tracker.register_abstract_method(AbstractMethodInfo {
        class_name: "BaseClass".to_string(),
        method_name: "process".to_string(),
        has_returns: true,
        has_raises: true,
        has_yields: false,
        file_path: "/tmp/base.py".to_string(),
    });

    // Register a concrete method that properly documents everything
    tracker.register_concrete_method(ConcreteMethodInfo {
        class_name: "ImplClass".to_string(),
        method_name: "process".to_string(),
        base_classes: vec!["BaseClass".to_string()],
        has_returns: true, // Properly documented!
        has_raises: true,  // Properly documented!
        has_yields: false,
        file_path: "/tmp/impl.py".to_string(),
        has_docstring: true,
        line: 10,
    });

    let violations = tracker.validate();
    assert_eq!(violations.len(), 0); // No violations!
}

#[test]
fn test_inheritance_tracker_multiple_base_classes() {
    let mut tracker = InheritanceTracker::new();

    // Register abstract methods from two different base classes
    tracker.register_abstract_method(AbstractMethodInfo {
        class_name: "BaseA".to_string(),
        method_name: "method_a".to_string(),
        has_returns: true,
        has_raises: false,
        has_yields: false,
        file_path: "/tmp/base_a.py".to_string(),
    });

    tracker.register_abstract_method(AbstractMethodInfo {
        class_name: "BaseB".to_string(),
        method_name: "method_b".to_string(),
        has_returns: false,
        has_raises: true,
        has_yields: false,
        file_path: "/tmp/base_b.py".to_string(),
    });

    // Register concrete methods that inherit from both
    tracker.register_concrete_method(ConcreteMethodInfo {
        class_name: "ImplClass".to_string(),
        method_name: "method_a".to_string(),
        base_classes: vec!["BaseA".to_string(), "BaseB".to_string()],
        has_returns: false, // Missing Returns from BaseA!
        has_raises: false,
        has_yields: false,
        file_path: "/tmp/impl.py".to_string(),
        has_docstring: true,
        line: 10,
    });

    tracker.register_concrete_method(ConcreteMethodInfo {
        class_name: "ImplClass".to_string(),
        method_name: "method_b".to_string(),
        base_classes: vec!["BaseA".to_string(), "BaseB".to_string()],
        has_returns: false,
        has_raises: false, // Missing Raises from BaseB!
        has_yields: false,
        file_path: "/tmp/impl.py".to_string(),
        has_docstring: true,
        line: 20,
    });

    let violations = tracker.validate();
    assert_eq!(violations.len(), 2);
}

#[test]
fn test_inheritance_tracker_no_violation_when_no_abstract() {
    let mut tracker = InheritanceTracker::new();

    // Register a concrete method with no corresponding abstract method
    tracker.register_concrete_method(ConcreteMethodInfo {
        class_name: "ImplClass".to_string(),
        method_name: "process".to_string(),
        base_classes: vec!["BaseClass".to_string()],
        has_returns: false,
        has_raises: false,
        has_yields: false,
        file_path: "/tmp/impl.py".to_string(),
        has_docstring: true,
        line: 10,
    });

    let violations = tracker.validate();
    assert_eq!(violations.len(), 0); // No violations because there's no abstract method
}
