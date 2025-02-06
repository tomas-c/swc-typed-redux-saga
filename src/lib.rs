use std::collections::HashSet;
use swc_core::{
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    ecma::ast::*,
    ecma::visit::{visit_mut_pass, VisitMut, VisitMutWith},
};


struct TransformVisitor {
    local_idents: HashSet<String>,
}

impl TransformVisitor {
    pub fn new() -> Self {
        Self {
            local_idents: HashSet::new(),
        }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        if &import_decl.src.value != "typed-redux-saga/macro" {
            return;
        }

        for specifier in &import_decl.specifiers {
            if let ImportSpecifier::Named(local) = specifier {
                self.local_idents.insert(String::from(&*local.local.sym));
            }
        }

        import_decl.src.raw = None;
        import_decl.src.value = "redux-saga/effects".into();
    }

    fn visit_mut_yield_expr(&mut self, yield_expr: &mut YieldExpr) {
        if let Some(arg) = &yield_expr.arg {
            if let Expr::Call(call_expr) = &**arg {
                if let Callee::Expr(callee_expr) = &call_expr.callee {
                    if let Expr::Ident(id) = &**callee_expr {
                        if self.local_idents.contains(&*id.sym) {
                            yield_expr.delegate = false
                        }
                    }
                }
            }
        }
        
        yield_expr.visit_mut_children_with(self);
    }
}

/// Transforms a [`Program`].
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(&mut visit_mut_pass(TransformVisitor::new()))
}

#[cfg(test)]
mod tests {
    use swc_core::{
        ecma::parser::{Syntax},
        ecma::transforms::testing::test_inline
    };

    use super::*;

    fn transform_visitor() -> impl 'static + VisitMut + Pass {
        visit_mut_pass(TransformVisitor::new())
    }

    test_inline!(
        Syntax::default(),
        |_| transform_visitor(),
        replaces_import,
        r#"import {put} from "typed-redux-saga/macro";"#,
        r#"import {put} from "redux-saga/effects";"#
    );

    test_inline!(
        Syntax::default(),
        |_| transform_visitor(),
        replaces_aliased_import,
        r#"import {put as _put} from "typed-redux-saga/macro";"#,
        r#"import {put as _put} from "redux-saga/effects";"#
    );

    test_inline!(
        Syntax::default(),
        |_| transform_visitor(),
        replaces_yield_delegate,
        r#"import {put} from "typed-redux-saga/macro";
        function* test() { yield* put(); }"#,
        r#"import {put} from "redux-saga/effects";
        function* test() { yield put(); }"#
    );

    test_inline!(
        Syntax::default(),
        |_| transform_visitor(),
        replaces_yield_delegate_with_args,
        r#"import {put} from "typed-redux-saga/macro";
        function* test() { yield* put("test"); }"#,
        r#"import {put} from "redux-saga/effects";
        function* test() { yield put("test"); }"#
    );

    test_inline!(
        Syntax::default(),
        |_| transform_visitor(),
        replaces_aliased_yield_delegate,
        r#"import {put as _put} from "typed-redux-saga/macro";
        function* test() { yield* _put(); }"#,
        r#"import {put as _put} from "redux-saga/effects";
        function* test() { yield _put(); }"#
    );

    test_inline!(
        Syntax::default(),
        |_| transform_visitor(),
        replaces_correct_yield_delegate,
        r#"import {put} from "typed-redux-saga/macro";
        import {call} from "typed-redux-saga";
        function* test() { yield* put(); }
        function* test() { yield* call(); }"#,
        r#"import {put} from "redux-saga/effects";
        import {call} from "typed-redux-saga";
        function* test() { yield put(); }
        function* test() { yield* call(); }"#
    );

    test_inline!(
        Syntax::default(),
        |_| transform_visitor(),
        replaces_multiple_yield_delegates,
        r#"import {put, call} from "typed-redux-saga/macro";
        function* test1() { yield* put(); }
        function* test1() { yield* call(); }"#,
        r#"import {put, call} from "redux-saga/effects";
        function* test1() { yield put(); }
        function* test1() { yield call(); }"#
    );

    test_inline!(
        Syntax::default(),
        |_| transform_visitor(),
        replaces_nested_yield_delegates,
        r#"
        import {put, call, fork} from "typed-redux-saga/macro";
        function* test1() { 
            yield* fork(function* backgroundTask() {
                yield* put(); 
            })
        }
        "#,
        r#"
        import {put, call, fork} from "redux-saga/effects";
        function* test1() { 
            yield fork(function* backgroundTask() {
                yield put(); 
            })
        }
        "#
    );
}
