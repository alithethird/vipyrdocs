use crate::docstring;
use crate::docstring::Docstring;
use rustpython_ast::text_size::TextRange;
use rustpython_ast::{
    Arguments, ExprYield, ExprYieldFrom, Stmt, StmtAsyncFunctionDef, StmtClassDef, StmtFunctionDef,
    StmtReturn, Visitor,
};
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
    // pub fn name(&self) -> &str {
    //     match self {
    //         FunctionDefKind::Sync(def) => &def.name,
    //         FunctionDefKind::Async(def) => &def.name,
    //     }
    // }
    // pub fn body(&self) -> &Vec<Stmt> {
    //     match self {
    //         FunctionDefKind::Sync(def) => &def.body,
    //         FunctionDefKind::Async(def) => &def.body,
    //     }
    // }
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
    pub docstring: Option<Docstring>,
}
//
// impl FunctionInfo{
//     fn is_test_function(&self) -> bool {
//         if self.def.name().starts_with("test") {
//             return true;
//         }
//         false
//     }
//     fn is_private_function(&self) -> bool {
//         if self.def.name().starts_with("_private") && !self.def.name().ends_with("_"){
//             return true;
//         }
//         false
//     }
// }

#[allow(dead_code)]
pub struct ClassInfo {
    pub def: StmtClassDef<TextRange>,
    pub funcs: Vec<FunctionInfo>,
    pub docstring: Option<Docstring>,
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

    let mut yield_collector = YieldCollector::new();
    for stmt in expr.body() {
        return_collector.visit_stmt(stmt.clone());
        yield_collector.visit_stmt(stmt.clone());
    }

    FunctionInfo {
        def: expr.clone(),
        returns: return_collector.returns,
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

//
// struct ReturnCollector {
//     pub returns: Vec<StmtReturn>,
// }
//
// impl Visitor for ReturnCollector {
//     fn visit_stmt_return(&mut self, node: StmtReturn<TextRange>) {
//         self.returns.push(node);
//     }
// }

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

        for stmt in &node.body {
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
        };

        self.class_infos.push(class_info);
        self.generic_visit_stmt_class_def(node);
    }
}
