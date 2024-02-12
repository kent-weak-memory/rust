use crate::{LateContext, LateLintPass, LintContext};
use rustc_ast::ast::LitKind;
use rustc_hir as hir;
use rustc_middle::ty::{self, UintTy};

// TODO(seharris): can we make this example produce an error so release builds stop warning about it not producing one?
declare_lint! {
    /// The `usize_as_pointer` lint detects casts from usize to pointer types that
    /// are likely to cause crashes on CHERI.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let data = 42 as i32;
    /// let address = &data as *const i32 as usize;
    /// let _pointer = address as *const i32;
    /// ```
    ///
    /// {{produces}}
    ///
    /// ### Explanation
    ///
    /// On CHERI hardware, casting an integer to a pointer results in an invalid
    /// pointer which will trigger `SIGPROT` and crash the program if it is
    /// dereferenced.
    /// While some conversions are ok because the result is never derferenced, for
    /// example when `0 as *const T` is used to represent invalid pointers, in most
    /// cases it will be necessary to modify the affected code to avoid casting
    /// pointers to integer types and back.
    /// Alternative, sometimes it is possible to add the integer part to some
    /// still-existing base pointer.
    /// If unstable features are an option, the strict provenance API can make
    /// this easier and produce much more understandable code.
    pub USIZE_AS_POINTER,
    Allow,
    // TODO(seharris): where will this be visible?
    "make users aware of issues with invalid pointers on CHERI-like targets"
}

declare_lint_pass!(UsizeAsPointer => [USIZE_AS_POINTER]);

impl<'tcx> LateLintPass<'tcx> for UsizeAsPointer {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expression: &'tcx hir::Expr<'tcx>) {
        if let hir::ExprKind::Cast(input_expression, _target_type) = expression.kind {
            // Ignore casts of small integer values to pointers.
            // This is to avoid emitting warnings for `NULL` and similar values used for
            // signalling that aren't supposed to ever be dereferenced.
            // We still warn on larger values (with an arbitrary choice of what "large"
            // should mean) to catch things like pointers to memory-mapped devices,
            // which probably won't work as intended.
            if let hir::ExprKind::Lit(literal) = &input_expression.kind {
                if let LitKind::Int(value, _literal_type) = literal.node {
                    if value < 16 {
                        return;
                    }
                }
            }

            let typeck_results = cx.typeck_results();
            let input_type = typeck_results.node_type(input_expression.hir_id);
            // We also have `_target_type.kind`, but that's an HIR type instead of a
            // `rustc_middle::ty`, and the second of those provides more detailed
            // description of the type.
            let target_type = typeck_results.node_type(expression.hir_id);
            match (input_type.kind(), target_type.kind()) {
                (ty::Uint(UintTy::Usize), ty::RawPtr(_)) => cx.struct_span_lint(
                    USIZE_AS_POINTER,
                    expression.span,
                    |lint| {
                        // TODO: later compiler versions added translation support, it would be nice to support that if we update to a newer compiler.
                        //       1.56.0 doesn't look like it has any of the code for this.
                        lint.build("casting an integer to a pointer will produce an invalid pointer on CHERI, and will result in crashes if it is ever dereferenced").emit();
                    },
                ),
                _ => (),
            }
        }
    }
}
