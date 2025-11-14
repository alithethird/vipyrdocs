use crate::docstring;
use crate::docstring::Docstring;
use rustpython_ast::text_size::TextRange;
use rustpython_ast::{
    Arguments, Expr, ExprAttribute, ExprCall, ExprYield, ExprYieldFrom, Stmt, StmtAnnAssign,
    StmtAssign, StmtAsyncFunctionDef, StmtAugAssign, StmtClassDef, StmtFunctionDef, StmtRaise,
    StmtReturn, Visitor,
};
use rustpython_parser::{parse, Mode};

use std::collections::HashSet;

// Helper function to extract byte offset from error messages
fn extract_byte_offset(error_msg: &str) -> Option<usize> {
    // Look for patterns like "at byte offset 3347"
    if let Some(start) = error_msg.find("at byte offset ") {
        let offset_str = &error_msg[start + "at byte offset ".len()..];
        if let Some(end) = offset_str.find(char::is_whitespace) {
            offset_str[..end].parse().ok()
        } else {
            offset_str.parse().ok()
        }
    } else {
        None
    }
}

// Helper function to convert byte offset to line and column number
fn byte_offset_to_line_column(code: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;
    
    for (i, byte) in code.bytes().enumerate() {
        if i >= offset {
            break;
        }
        if byte == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    
    (line, column)
}

pub fn get_result(code: &str, filename: Option<&str>) -> DocstringCollector {
    let filename = filename.unwrap_or("<embedded>");
    let tree = parse(code, Mode::Interactive, filename);
    
    // Handle parsing errors gracefully
    let tree_mod = match tree {
        Ok(parsed_tree) => parsed_tree,
        Err(parse_error) => {
            // Convert byte offset to line number for more helpful error messages
            let error_msg = if let Some(offset) = extract_byte_offset(&parse_error.to_string()) {
                let (line, column) = byte_offset_to_line_column(code, offset);
                format!("Failed to parse Python file '{}': {} at line {}, column {}", 
                       filename, parse_error, line, column)
            } else {
                format!("Failed to parse Python file '{}': {}", filename, parse_error)
            };
            eprintln!("Warning: {}", error_msg);
            
            // Return empty collector for unparseable files
            return DocstringCollector {
                function_infos: Vec::new(),
                class_infos: Vec::new(),
            };
        }
    };
    
    let body = &tree_mod.as_interactive().unwrap().body;
    let mut ds = DocstringCollector {
        function_infos: Vec::new(),
        class_infos: Vec::new(),
    };
    for stmt in body.iter() {
        ds.visit_stmt(stmt.clone());
    }
    ds
}
pub struct DocstringCollector {
    pub function_infos: Vec<FunctionInfo>,
    pub class_infos: Vec<ClassInfo>,
}

#[derive(PartialEq, Clone)]
pub enum YieldKind {
    Yield(ExprYield),
    YieldFrom(ExprYieldFrom),
}

impl YieldKind {
    pub fn range(&self) -> &TextRange {
        match self {
            YieldKind::Yield(def) => &def.range,
            YieldKind::YieldFrom(def) => &def.range,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum FunctionDefKind {
    Sync(StmtFunctionDef<TextRange>),
    Async(StmtAsyncFunctionDef<TextRange>),
}

impl FunctionDefKind {
    pub fn name(&self) -> &str {
        match self {
            FunctionDefKind::Sync(def) => &def.name,
            FunctionDefKind::Async(def) => &def.name,
        }
    }
    pub fn body(&self) -> &Vec<Stmt> {
        match self {
            FunctionDefKind::Sync(def) => &def.body,
            FunctionDefKind::Async(def) => &def.body,
        }
    }
    pub fn range(&self) -> &TextRange {
        match self {
            FunctionDefKind::Sync(def) => &def.range,
            FunctionDefKind::Async(def) => &def.range,
        }
    }
    pub fn decorator_list(&self) -> &Vec<Expr> {
        match self {
            FunctionDefKind::Sync(def) => &def.decorator_list,
            FunctionDefKind::Async(def) => &def.decorator_list,
        }
    }
    pub fn args(&self) -> &Arguments {
        match self {
            FunctionDefKind::Sync(def) => &def.args,
            FunctionDefKind::Async(def) => &def.args,
        }
    }
}

#[derive(PartialEq)]
pub struct FunctionInfo {
    pub def: FunctionDefKind,
    pub returns: Vec<StmtReturn>,
    pub yields: Vec<YieldKind>,
    pub raises: Vec<StmtRaise>,
    pub docstring: Option<Docstring>,
}

#[allow(dead_code)]
pub struct ClassInfo {
    pub def: StmtClassDef<TextRange>,
    pub funcs: Vec<FunctionInfo>,
    pub docstring: Option<Docstring>,
    pub attributes: Vec<String>,
    pub instance_attributes: Vec<String>,
}
fn get_docs(expr: &Expr<TextRange>) -> Option<Docstring> {
    if expr.is_constant_expr() {
        let ds = expr.as_constant_expr().unwrap();
        if !ds.clone().value.is_str() {
            return None;
        }
        let docstring = docstring::parse(ds);
        return Some(docstring);
    }
    None
}
fn get_func(expr: &FunctionDefKind) -> FunctionInfo {
    let mut function_docs: Option<Docstring> = None;

    // Get docstring if the first statement is an Expr
    if let Some(Stmt::Expr(expr_stmt)) = expr.body().first() {
        function_docs = get_docs(&expr_stmt.value);
    }

    // Walk the function body to collect all return statements
    let mut return_collector = ReturnCollector::new();
    let mut raise_collector = RaiseCollector::new();

    let mut yield_collector = YieldCollector::new();
    for stmt in expr.body() {
        return_collector.visit_stmt(stmt.clone());
        raise_collector.visit_stmt(stmt.clone());
        yield_collector.visit_stmt(stmt.clone());
    }

    FunctionInfo {
        def: expr.clone(),
        returns: return_collector.returns,
        raises: raise_collector.raises,
        yields: yield_collector.yields,
        docstring: function_docs,
    }
}
struct YieldCollector {
    pub yields: Vec<YieldKind>,
    func_depth: usize,
    class_depth: usize,
}

impl YieldCollector {
    pub fn new() -> Self {
        Self {
            yields: Vec::new(),
            func_depth: 0,
            class_depth: 0,
        }
    }
}

impl Visitor for YieldCollector {
    fn visit_stmt_function_def(&mut self, node: StmtFunctionDef<TextRange>) {
        self.func_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.func_depth -= 1;
    }

    fn visit_stmt_async_function_def(&mut self, node: StmtAsyncFunctionDef<TextRange>) {
        self.func_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.func_depth -= 1;
    }

    fn visit_stmt_class_def(&mut self, node: StmtClassDef<TextRange>) {
        self.class_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.class_depth -= 1;
    }

    fn visit_expr_yield(&mut self, node: ExprYield<TextRange>) {
        if self.func_depth == 0 && self.class_depth == 0 {
            self.yields.push(YieldKind::Yield(node));
        }
    }

    fn generic_visit_expr_yield_from(&mut self, node: ExprYieldFrom<TextRange>) {
        if self.func_depth == 0 && self.class_depth == 0 {
            self.yields.push(YieldKind::YieldFrom(node));
        }
    }
}

struct RaiseCollector {
    pub raises: Vec<StmtRaise<TextRange>>,
    func_depth: usize,
    class_depth: usize,
}

impl RaiseCollector {
    pub fn new() -> Self {
        Self {
            raises: Vec::new(),
            func_depth: 0,
            class_depth: 0,
        }
    }
}

impl Visitor for RaiseCollector {
    fn visit_stmt_function_def(&mut self, node: StmtFunctionDef<TextRange>) {
        self.func_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.func_depth -= 1;
    }

    fn visit_stmt_async_function_def(&mut self, node: StmtAsyncFunctionDef<TextRange>) {
        self.func_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.func_depth -= 1;
    }

    fn visit_stmt_class_def(&mut self, node: StmtClassDef<TextRange>) {
        self.class_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.class_depth -= 1;
    }

    fn visit_stmt_raise(&mut self, node: StmtRaise<TextRange>) {
        if self.func_depth == 0 && self.class_depth == 0 {
            self.raises.push(node);
        }
    }
}
struct ReturnCollector {
    pub returns: Vec<StmtReturn<TextRange>>,
    func_depth: usize,
    class_depth: usize,
}

impl ReturnCollector {
    pub fn new() -> Self {
        Self {
            returns: Vec::new(),
            func_depth: 0,
            class_depth: 0,
        }
    }
}

impl Visitor for ReturnCollector {
    fn visit_stmt_function_def(&mut self, node: StmtFunctionDef<TextRange>) {
        self.func_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.func_depth -= 1;
    }

    fn visit_stmt_async_function_def(&mut self, node: StmtAsyncFunctionDef<TextRange>) {
        self.func_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.func_depth -= 1;
    }

    fn visit_stmt_class_def(&mut self, node: StmtClassDef<TextRange>) {
        self.class_depth += 1;
        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }
        self.class_depth -= 1;
    }

    fn visit_stmt_return(&mut self, node: StmtReturn<TextRange>) {
        if self.func_depth == 0 && self.class_depth == 0 {
            self.returns.push(node);
        }
    }
}
struct AttributeCollector {
    pub class_attributes: Vec<String>,
    pub instance_attributes: Vec<String>,
    receiver_stack: Vec<HashSet<String>>,
    fn_depth: usize,
}

impl AttributeCollector {
    pub fn new() -> Self {
        Self {
            class_attributes: Vec::new(),
            instance_attributes: Vec::new(),
            receiver_stack: vec![HashSet::new()],
            fn_depth: 0,
        }
    }

    fn push_receivers(&mut self, receivers: HashSet<String>) {
        self.receiver_stack.push(receivers);
    }

    fn pop_receivers(&mut self) {
        self.receiver_stack.pop();
    }

    fn is_tracked_receiver(&self, name: &str) -> bool {
        self.receiver_stack
            .iter()
            .rev()
            .any(|receivers| receivers.contains(name))
    }

    fn record_class_attribute(&mut self, name: String) {
        self.class_attributes.push(name);
    }

    fn record_instance_attribute(&mut self, name: String) {
        self.instance_attributes.push(name);
    }

    fn extract_tracked_attribute_name(&self, expr: &ExprAttribute<TextRange>) -> Option<String> {
        if let Some(name_expr) = expr.value.as_name_expr() {
            if self.is_tracked_receiver(name_expr.id.as_str()) {
                return Some(expr.attr.to_string());
            }
        } else if let Some(inner_attr) = expr.value.as_attribute_expr() {
            if let Some(attr) = self.extract_tracked_attribute_name(inner_attr) {
                return Some(attr);
            }
        }
        None
    }
}

impl Visitor for AttributeCollector {
    fn visit_stmt_assign(&mut self, node: StmtAssign<TextRange>) {
        for target in &node.targets {
            if self.fn_depth == 0 {
                if let Some(name_expr) = target.as_name_expr() {
                    self.record_class_attribute(name_expr.id.to_string());
                    continue;
                }
            }

            if let Some(attr_expr) = target.as_attribute_expr() {
                if let Some(attr_name) = self.extract_tracked_attribute_name(attr_expr) {
                    self.record_instance_attribute(attr_name);
                }
            }
        }
    }

    fn visit_stmt_ann_assign(&mut self, node: StmtAnnAssign<TextRange>) {
        if self.fn_depth == 0 {
            if let Some(name_expr) = node.target.as_name_expr() {
                self.record_class_attribute(name_expr.id.to_string());
            }
        }

        if let Some(attr_expr) = node.target.as_attribute_expr() {
            if let Some(attr_name) = self.extract_tracked_attribute_name(attr_expr) {
                self.record_instance_attribute(attr_name);
            }
        }
    }

    fn visit_stmt_aug_assign(&mut self, node: StmtAugAssign<TextRange>) {
        if self.fn_depth == 0 {
            if let Some(name_expr) = node.target.as_name_expr() {
                self.record_class_attribute(name_expr.id.to_string());
            }
        }

        if let Some(attr_expr) = node.target.as_attribute_expr() {
            if let Some(attr_name) = self.extract_tracked_attribute_name(attr_expr) {
                self.record_instance_attribute(attr_name);
            }
        }
    }

    fn visit_stmt_function_def(&mut self, node: StmtFunctionDef<TextRange>) {
        let was_top_level_method = self.fn_depth == 0;
        self.fn_depth += 1;

        if was_top_level_method {
            if node.decorator_list.iter().any(|dec| is_property(dec)) {
                self.record_class_attribute(node.name.to_string());
            }

            let mut receivers = HashSet::new();
            if !has_decorator_named(&node.decorator_list, "staticmethod") {
                if let Some(first_param) = first_parameter_name(&node.args) {
                    receivers.insert(first_param);
                }
            }
            self.push_receivers(receivers);
        } else {
            self.push_receivers(HashSet::new());
        }

        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }

        self.pop_receivers();
        self.fn_depth -= 1;
    }

    fn visit_stmt_async_function_def(&mut self, node: StmtAsyncFunctionDef<TextRange>) {
        let was_top_level_method = self.fn_depth == 0;
        self.fn_depth += 1;

        if was_top_level_method {
            if node.decorator_list.iter().any(|dec| is_property(dec)) {
                self.record_class_attribute(node.name.to_string());
            }

            let mut receivers = HashSet::new();
            if !has_decorator_named(&node.decorator_list, "staticmethod") {
                if let Some(first_param) = first_parameter_name(&node.args) {
                    receivers.insert(first_param);
                }
            }
            self.push_receivers(receivers);
        } else {
            self.push_receivers(HashSet::new());
        }

        for stmt in &node.body {
            self.visit_stmt(stmt.clone());
        }

        self.pop_receivers();
        self.fn_depth -= 1;
    }

    fn visit_stmt_class_def(&mut self, _node: StmtClassDef<TextRange>) {}
}
fn is_property(decorator: &Expr) -> bool {
    decorator_matches_any(decorator, &["property", "cached_property"])
}

fn decorator_matches_any(decorator: &Expr, names: &[&str]) -> bool {
    names
        .iter()
        .any(|name| decorator_matches_name(decorator, name))
}

fn decorator_matches_name(decorator: &Expr, name: &str) -> bool {
    if decorator.is_name_expr() {
        return decorator
            .as_name_expr()
            .map(|expr| expr.id.eq_ignore_ascii_case(name))
            .unwrap_or(false);
    }

    if decorator.is_call_expr() {
        let call: &ExprCall = decorator.as_call_expr().unwrap();
        if let Some(name_expr) = call.func.as_name_expr() {
            if name_expr.id.eq_ignore_ascii_case(name) {
                return true;
            }
        }

        if let Some(attr_expr) = call.func.as_attribute_expr() {
            if attr_expr.attr.eq_ignore_ascii_case(name) {
                return true;
            }
        }
    }

    if decorator.is_attribute_expr() {
        return decorator
            .as_attribute_expr()
            .map(|expr| expr.attr.eq_ignore_ascii_case(name))
            .unwrap_or(false);
    }

    false
}

fn has_decorator_named(decorators: &[Expr], name: &str) -> bool {
    decorators
        .iter()
        .any(|decorator| decorator_matches_name(decorator, name))
}

fn first_parameter_name(args: &Arguments) -> Option<String> {
    if let Some(arg) = args.posonlyargs.first() {
        return Some(arg.def.arg.to_string());
    }
    if let Some(arg) = args.args.first() {
        return Some(arg.def.arg.to_string());
    }
    None
}
impl Visitor for DocstringCollector {
    fn visit_stmt_async_function_def(&mut self, node: StmtAsyncFunctionDef<TextRange>) {
        let function_info = get_func(&FunctionDefKind::Async(node.clone()));
        if !self
            .class_infos
            .iter()
            .any(|class_info| class_info.funcs.contains(&function_info))
        {
            self.function_infos.push(function_info);
        }
        self.generic_visit_stmt_async_function_def(node);
    }
    fn visit_stmt_function_def(&mut self, node: StmtFunctionDef<TextRange>) {
        let function_info = get_func(&FunctionDefKind::Sync(node.clone()));
        if !self
            .class_infos
            .iter()
            .any(|class_info| class_info.funcs.contains(&function_info))
        {
            self.function_infos.push(function_info);
        }
        self.generic_visit_stmt_function_def(node);
    }

    fn visit_stmt_class_def(&mut self, node: StmtClassDef<TextRange>) {
        let mut class_docs: Option<Docstring> = None;
        let mut class_funcs: Vec<FunctionInfo> = Vec::new();

        let mut attribute_collector = AttributeCollector::new();
        for stmt in &node.body {
            attribute_collector.visit_stmt(stmt.clone());
            if let Stmt::Expr(expr_stmt) = stmt {
                let temp_doc = get_docs(&expr_stmt.value);
                if temp_doc.is_some() {
                    // if !temp_doc.clone().unwrap().is_empty() {
                    class_docs = temp_doc;
                    // }
                }
            }
            if let Stmt::FunctionDef(func_def) = stmt {
                class_funcs.push(get_func(&FunctionDefKind::Sync(func_def.clone())));
            }
        }
        let class_info = ClassInfo {
            def: node.clone(),
            funcs: class_funcs,
            docstring: class_docs,
            attributes: attribute_collector.class_attributes,
            instance_attributes: attribute_collector.instance_attributes,
        };

        self.class_infos.push(class_info);
        self.generic_visit_stmt_class_def(node);
    }
}
