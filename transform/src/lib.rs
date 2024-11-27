use serde::Deserialize;
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, ExprStmt, Lit, Module, ModuleItem, Stmt, Str},
        visit::VisitMut,
    },
};

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub paths: Vec<String>,
}

pub struct TransformVisitor {
    pub file_path: String,
    pub paths: Vec<String>,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, n: &mut Module) {
        for path in &self.paths {
            if self.file_path.contains(path) {
                // Creating line for "use client" directive
                let directive = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: "use client".into(),
                        raw: None,
                    }))),
                }));
                n.body.insert(0, directive.clone());
            }
        }
    }
}
