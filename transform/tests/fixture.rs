use std::path::PathBuf;

use directive_transform::TransformVisitor;
use swc_core::ecma::{
    parser::{EsSyntax, Syntax},
    transforms::testing::{test_fixture, FixtureTestConfig},
    visit::visit_mut_pass,
};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {

    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        &|_t| {
            visit_mut_pass(TransformVisitor {
                file_path: "test".into(),
                paths: ["test".into()].to_vec(),
            })
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
