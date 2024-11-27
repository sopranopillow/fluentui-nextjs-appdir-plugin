use std::path::PathBuf;

use directive_transform::TransformVisitor;
use swc_core::ecma::{
    parser::{EsSyntax, Syntax},
    transforms::testing::{test_fixture, FixtureTestConfig},
    visit::visit_mut_pass,
};

// use std::path::{Path, PathBuf};

// use directive_transform::TransformVisitor;
// use swc_core::{
//     common::{sync::Lrc, SourceMap},
//     ecma::{
//         ast::{EsVersion, Pass, Program},
//         codegen::{text_writer, Emitter},
//         parser::{lexer::Lexer, EsSyntax, Parser, StringInput, Syntax},
//         visit::visit_mut_pass,
//     },
// };
// use testing::{run_test2, NormalizedOutput};

// pub fn print(cm: Lrc<SourceMap>, program: &Program) -> String {
//     let mut buf = Vec::new();
//     {
//         let mut emitter = Emitter {
//             cfg: Default::default(),
//             cm: cm.clone(),
//             wr: Box::new(text_writer::JsWriter::new(cm, "\n", &mut buf, None)),
//             comments: None,
//         };

//         emitter.emit_program(program).unwrap();
//     }

//     let s = String::from_utf8_lossy(&buf);
//     s.to_string()
// }

// fn run<F, P>(syntax: Syntax, input: &Path, op: F)
// where
//     F: FnOnce() -> P,
//     P: Pass,
// {
//     let dir = input.parent().unwrap();
//     let output = dir.join(format!(
//         "output.{}",
//         input.extension().unwrap().to_string_lossy(),
//     ));

//     run_test2(false, |cm, handler| {
//         let fm = cm.load_file(input).unwrap();

//         let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);
//         let mut parser = Parser::new_from(lexer);

//         let program = parser
//             .parse_program()
//             .map_err(|err| err.into_diagnostic(&handler).emit())?;

//         let mut folder = op();

//         let program = program.apply(&mut folder);

//         let actual = print(cm, &program);

//         let actual = NormalizedOutput::from(actual);

//         println!("AAAAAActual {:?}", actual);

//         actual.compare_to_file(&output).unwrap();

//         Ok(())
//     })
//     .unwrap();
// }

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    // run(
    //     Syntax::Es(EsSyntax {
    //         ..Default::default()
    //     }),
    //     &input,
    //     || {
    //         visit_mut_pass(TransformVisitor {
    //             file_path: "path_to_look_for/current_path".into(),
    //             paths: ["path_to_look_for".into()].to_vec(),
    //         })
    //     },
    // );

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

    // test_fixture(
    //     Syntax::Es(EsSyntax {
    //         ..Default::default()
    //     }),
    //     &|tester| transformer(tester),
    //     &input,
    //     &output,
    //     Default::default(),
    // );

    // test_fixture(Syntax::Es(EsSyntax {
    //     js: input.to_string_lossy().ends_with(".js"),
    //     ..Default::default()
    // }), true, TransformVisitor, &input, &output);

    // test_fixture(Syntax::Es(EsConfig {
    //     js: input.to_string_lossy().ends_with(".js"),
    //     ..Default::default()
    // }),
    // &|t| (tr(), properties(t, true))
    // ),
    // &input,
    // &output
}
