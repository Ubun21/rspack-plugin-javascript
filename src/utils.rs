use rspack_core::{SourceType};
use std::path::Path;
use swc_ecma_parser::{EsConfig, TsConfig, Syntax};
use std::sync::Arc;
use swc_common::{FileName, FilePathMapping, SourceMap, GLOBALS};
use swc::{config::IsModule, Compiler as SwcCompiler};
use swc_ecma_ast::Program;

pub fn parse(
  filename: &str,
  source: String,
  source_type: &SourceType,
) -> Program {
  let syntax = syntax_by_source_type(filename, source_type);
  let compiler = get_swc_compiler();
    let fm = compiler
        .cm
        .new_source_file(FileName::Custom(filename.to_string()), source.to_string());
    swc::try_with_handler(
  compiler.cm.clone(),
  Default::default(),
  |handler| {
      GLOBALS.set(&Default::default(), || {
        compiler.parse_js(
            fm, 
            handler,
            swc_ecma_ast::EsVersion::Es2022, 
            syntax, 
            IsModule::Unknown, 
            None)
      })
    })
    .unwrap()
}

pub fn syntax_by_source_type(filename: &str, source_type: &SourceType) -> Syntax {
  match source_type {
      SourceType::Ts | SourceType::Tsx => Syntax::Typescript(TsConfig {
          tsx: source_type == &SourceType::Tsx,
           ..Default::default()
      }),
      SourceType::Js | SourceType::Jsx => Syntax::Es(EsConfig {
          jsx: source_type == &SourceType::Jsx,
          ..Default::default()
      }),
      _ => {
          let ext = Path::new(filename).extension().unwrap().to_str().unwrap();
          syntax_by_ext(ext)
      }
  }
}

pub fn syntax_by_ext(ext: &str) -> Syntax {
  match ext == "ts" || ext == "tsx" {
      true => Syntax::Typescript(TsConfig { 
           tsx: ext == "tsx",
           decorators: false, 
           ..Default::default()
      }),
      false => Syntax::Es(EsConfig {
          jsx: ext == "jsx",
          ..Default::default()
      }),
  }
}

pub fn get_swc_compiler() -> Arc<SwcCompiler> {
  let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
  Arc::new(SwcCompiler::new(cm))
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_parser() {
    let source = String::from("foo + bar(baz.qux)");
    let program = parse("test.ts", source, &SourceType::Js);
    assert_eq!(program.is_module(), false);
  }
}