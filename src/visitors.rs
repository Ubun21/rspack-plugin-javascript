use linked_hash_set::LinkedHashSet;
use rspack_core::{ModuleDependency, ResolveKind};
use swc_atoms::JsWord;
use swc_ecma_ast::{ModuleDecl, CallExpr, Callee};
use swc_ecma_visit::{VisitMut};

#[derive(Default)]
pub struct DependencyScanner {
  pub dependencies: LinkedHashSet<ModuleDependency>,
}

impl DependencyScanner {
    pub fn add_dependency(&mut self, specifier: JsWord, kind: ResolveKind) {
      self.dependencies.insert_if_absent(ModuleDependency {
        specifier: specifier.to_string(),
        kind,
      });
    }

    pub fn add_import(&mut self, module_decl: &mut ModuleDecl) {
      if let ModuleDecl::Import(import_decl) = module_decl {
        let source = import_decl.src.value.clone();
        self.add_dependency(source, ResolveKind::Import);
      }
    }

    pub fn add_require(&mut self, call_expr: &CallExpr) {

    }

}

impl VisitMut for DependencyScanner {

  fn visit_mut_module_decl(&mut self, node: &mut ModuleDecl) {
    self.add_import(node);
  }
}
