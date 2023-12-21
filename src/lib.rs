use directive_transform::{Config, TransformVisitor};
use swc_core::{
    ecma:: {
        ast::Program,
        visit::VisitMutWith
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata, metadata::TransformPluginMetadataContextKind},
};

#[plugin_transform]
pub fn process_transform(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config_json = &data.get_transform_plugin_config()
        .expect("failed to get plugin data, paths must be provided");
    let config = serde_json::from_str::<Config>(config_json).expect("invalid config for fluentui-next-appdir-directive swc plugin");

    let file_path = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s,
        None => String::from("")
    };

    program.visit_mut_with(&mut TransformVisitor {file_path: file_path, paths: config.paths});
    program
}
