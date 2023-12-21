use std::path::PathBuf;

use directive_transform::TransformVisitor;
use swc_core::ecma::{visit::as_folder, transforms::testing::{test_fixture, FixtureTestConfig}, parser::{EsConfig, Syntax}};

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(syntax(),
    &|_tr| {
        as_folder(TransformVisitor {file_path: "test".into(), paths: ["test".into()].to_vec()})
    },
    &input,
    &output,
    FixtureTestConfig {
        ..Default::default()
    })
}