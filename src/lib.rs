mod visitors;
mod utils;
use swc_ecma_visit::VisitMutWith;
use swc_common::{FileName, GLOBALS};
use swc_ecma_transforms::{pass::noop, react};
use swc_common::comments::SingleThreadedComments;
pub use visitors::*;
pub use utils::*;
use rspack_core::{SourceType, Module, Plugin, PluginContext, RenderManifestArgs, Asset};
use swc_ecma_ast::EsVersion;
use visitors::DependencyScanner;

#[derive(Debug)]
pub struct JsModule {
  pub uri: String,
  pub source_type: SourceType,
  pub ast: swc_ecma_ast::Program,
}

impl Module for JsModule {
  fn render(&self) -> String {
    use::swc::config::{self as swc_config, SourceMapsConfig};
        let complier = get_swc_compiler();
        let output = swc::try_with_handler(
            complier.cm.clone(),
            Default::default(), 
            |handler| {
                GLOBALS.set(&Default::default(), || {
                    let fm = complier
                    .cm
                    .new_source_file(FileName::Custom(self.uri.clone()), self.uri.clone());
                let source_map = false;
                complier.process_js_with_custom_pass(
                    fm,
                    Some(self.ast.clone()), 
                    handler, 
                    &swc_config::Options {
                        config: swc_config::Config {
                            jsc: swc_config::JscConfig {
                              target: Some(EsVersion::Es2022),
                              syntax: Some(syntax_by_source_type(self.uri.as_str(), &self.source_type)),
                              transform: Some(swc_config::TransformConfig {
                                react: react::Options {
                                  runtime: Some(react::Runtime::Automatic),
                                  ..Default::default()
                                },
                                ..Default::default()
                              })
                              .into(),
                              ..Default::default()
                            },
                            inline_sources_content: true.into(),
                            // emit_source_map_columns: (!matches!(options.mode, BundleMode::Dev)).into(),
                            source_maps: Some(SourceMapsConfig::Bool(source_map)),
                            ..Default::default()
                          },
                          // top_level_mark: Some(bundle_ctx.top_level_mark),
                          ..Default::default()
                    },
                    SingleThreadedComments::default(), 
                    |_| noop(), 
                    |_| {
                        noop()
                    })
                })
        })
        .unwrap();
        output.code
  }

  fn dependencies(&mut self) -> Vec<rspack_core::ModuleDependency> {
    let mut dependency = DependencyScanner::default();
    self.ast.visit_mut_with(&mut dependency);
    dependency.dependencies.into_iter().collect()
  }
}

#[derive(Debug)]
pub struct JsPlugin;

impl Plugin for JsPlugin {
  fn register_parse_module(&self, _ctx: rspack_core::PluginContext) -> Option<Vec<SourceType>> {
      Some(vec![
        SourceType::Js,
        SourceType::Ts,
        SourceType::Jsx,
        SourceType::Tsx,
      ])
  }

  fn parse_module(&self, ctx: rspack_core::PluginContext<& mut rspack_core::JobContext>, args: rspack_core::ParseModuleArgs) -> rspack_core::BoxModule {
      let ast = parse(
        args.uri,
        args.source,
        ctx.context.source_type.as_ref().unwrap(),
      );
      Box::new(JsModule {
        uri: args.uri.to_string(),
        source_type: ctx.context.source_type.as_ref().unwrap().clone(),
        ast,
      })
  }

  fn render_manifest(
    &self, 
    _ctx: PluginContext, 
    _args: RenderManifestArgs) -> Vec<Asset> {
    vec![]
  }
}