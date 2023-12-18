use swc_core::{
    ecma:: {
        ast::{ModuleItem, ExprStmt, Str, Expr, Lit, Stmt},
        transforms::testing::test,
        // for some reason vscode complains of unused as_folder, but it's needed for the tests below
        visit::{as_folder, VisitMut},
    },
    common::DUMMY_SP,
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self,n: &mut swc_core::ecma::ast::Module) {
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

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    append_directive_to_top,
    r#"console.log("transform");"#
);
