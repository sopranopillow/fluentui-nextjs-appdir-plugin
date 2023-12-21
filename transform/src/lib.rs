use serde::Deserialize;
use swc_core::{
    ecma:: {
        ast::{ModuleItem, ExprStmt, Str, Expr, Lit, Stmt},
        visit::VisitMut,
    },
    common::DUMMY_SP,
};

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub paths: Vec<String>
}

pub struct TransformVisitor {
    pub file_path: String,
    pub paths: Vec<String>
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self,n: &mut swc_core::ecma::ast::Module) {
        for path in &self.paths {
            if self.file_path.contains(path) {
                // Creating line for "use client" directive
                let directive = &[ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: "use client".into(),
                        raw: None
                    })))
                }))];

                // We need to splice this to be able to prepend since rust doesn't provide this functionality
                n.body.splice(0..0, directive.iter().cloned());
            }
        }
    }
}