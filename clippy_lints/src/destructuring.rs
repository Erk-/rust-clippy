use clippy_utils::{diagnostics::span_lint_and_then};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::declare_lint_pass;
use rustc_ast::ast::{Pat, PatKind};

declare_clippy_lint! {
    /// What it does
    /// Nothing yet
    ///
    /// ### Example
    /// ```no_run
    /// struct Foo {
    ///   bar: u8,
    ///   baz: u8,
    /// }
    ///
    /// let f = Foo { bar: 1, baz: 2 };
    /// let Foo { bar, .. } = f;
    /// ```
    #[clippy::version = "1.90.0"]
    pub REST_WHEN_DESTRUCTURING,
    nursery,
    "rest (..) in destructuring expression"
}

declare_lint_pass!(RestDestructuring => [REST_WHEN_DESTRUCTURING]);

impl EarlyLintPass for RestDestructuring {
    fn check_pat(&mut self, cx: &EarlyContext<'_>, pat: &Pat) {
        if let PatKind::Struct(_, _, _, rustc_ast::ast::PatFieldsRest::Rest) = pat.kind {
            #[expect(clippy::collapsible_span_lint_calls, reason = "rust-clippy#7797")]
            span_lint_and_then(
                cx,
                REST_WHEN_DESTRUCTURING,
                pat.span,
                "AAAAAAAAAAAAAAAAAAAAAAAAAAa",
                |diag| {
                    diag.help("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAaaaaaaaaaaaaaaaaaaaa");
                },
            );
        }
    }
}
