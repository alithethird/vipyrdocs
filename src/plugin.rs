use crate::docstring;
use crate::docstring::Docstring;
use rustpython_ast::text_size::TextRange;
use rustpython_ast::{Arguments, ExprAttribute, ExprCall, ExprYield, ExprYieldFrom, Stmt, StmtAssign, StmtAsyncFunctionDef, StmtClassDef, StmtFunctionDef, StmtRaise, StmtReturn, Visitor};
use rustpython_parser::{parse, Mode};

use rustpython_ast::Expr;

pub fn get_result(code: &str, filename: Option<&str>) -> DocstringCollector {
    let filename = filename.unwrap_or("<embedded>");
    let tree = parse(code, Mode::Interactive, filename);
    let tree_mod = tree.unwrap();
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
    pub attributes: Vec<String>,
}

impl AttributeCollector {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }
}

impl Visitor for AttributeCollector {
    fn visit_expr_attribute(&mut self, node: ExprAttribute<TextRange>) {
        self.attributes.push(node.attr.to_string());
    }
    fn visit_stmt_assign(&mut self, node: StmtAssign<TextRange>) {
        let targets = &node.targets;
        for target in targets {
            if target.as_name_expr().is_some() {
                // Handle direct assignments like: attr_1 = "value"
                let _target = target.as_name_expr().unwrap().id.clone();
                self.attributes.push(_target.to_string());
            } else if target.as_attribute_expr().is_some() {
                // Handle self.attribute assignments like: self.attr_1 = "value"
                let attr_expr = target.as_attribute_expr().unwrap();
                if let Some(value_expr) = attr_expr.value.as_name_expr() {
                    if value_expr.id.as_str() == "self" {
                        self.attributes.push(attr_expr.attr.to_string());
                    }
                }
            }
        }
    }
    fn visit_stmt_function_def(&mut self, node: StmtFunctionDef<TextRange>) {
        for dec in &node.decorator_list {
             if is_property(dec){
                self.attributes.push(node.name.to_string());
            }
        }
        
        // If this is __init__, visit its body to find self.attribute assignments
        if node.name.as_str() == "__init__" {
            for stmt in &node.body {
                self.visit_stmt(stmt.clone());
            }
        }
        // self.generic_visit_stmt_function_def(node);
    }
    fn visit_stmt_async_function_def(&mut self, node: StmtAsyncFunctionDef<TextRange>) {
        for dec in &node.decorator_list {
            if is_property(dec){
                self.attributes.push(node.name.to_string());
            }
        }
        // self.generic_visit_stmt_async_function_def(node);
    }
}
fn is_property(decorator: &Expr) -> bool {
    let property_tag_list = ["property", "cached_property"];
    
    if decorator.is_name_expr() {
        let id = &decorator.as_name_expr().unwrap().id;
        for property_tag in property_tag_list{
            if id.eq_ignore_ascii_case(property_tag) {
                return true;
            }
        }
    }
    
    if decorator.is_call_expr() {
        let call: &ExprCall = decorator.as_call_expr().unwrap();
        if let Some(name_expr) = call.func.as_name_expr() {
            let id = &name_expr.id;
            for property_tag in property_tag_list{
                if id.eq_ignore_ascii_case(property_tag) {
                    return true;
                }
            }
        }
        
        if let Some(attr_expr) = call.func.as_attribute_expr() {
            let attr = &attr_expr.attr;
            for property_tag in property_tag_list{
                if attr.eq_ignore_ascii_case(property_tag) {
                    return true;
                }
            }
        }
    }
    
    if decorator.is_attribute_expr() {
        let attr = &decorator.as_attribute_expr().unwrap().attr;
        for property_tag in property_tag_list{
            if attr.eq_ignore_ascii_case(property_tag) {
                return true;
            }
        }
    }
    
    false
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
            attributes: attribute_collector.attributes,
        };

        self.class_infos.push(class_info);
        self.generic_visit_stmt_class_def(node);
    }
}
