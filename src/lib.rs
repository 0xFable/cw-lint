#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_lexer;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_parse;
extern crate rustc_parse_format;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate rustc_typeck;
use clippy_utils::match_def_path;
use rustc_lint::{LateLintPass,LateContext};
use rustc_hir::{def_id::DefId, Body, Expr};
use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::ty::{is_type_diagnostic_item, is_type_lang_item};
use cosmwasm_std::{CanonicalAddr};


// TODO: Move this out into a sep file
pub fn is_canon_addr_call(cx: &LateContext<'_>, fn_def_id: DefId) -> bool {
    match_def_path(cx, fn_def_id, &CANONADDRCALL)
}

pub const CANONADDRCALL: [&str; 2] = ["cosmwasm_std", "CanonicalAddr"];


dylint_linting::declare_late_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub FILL_ME_IN,
    Warn,
    "description goes here"
}

// impl<'tcx> LateLintPass<'tcx> for FillMeIn {
//     // A list of things you might check can be found here:
//     // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html

// }

impl<'hir> LateLintPass<'hir>for FillMeIn {
    fn check_expr(&mut self, cx: &LateContext<'hir>, expr: &'hir Expr<'hir>) {
        // Getting the fn definition type
        if let Some(fn_def_id) = cx.typeck_results().type_dependent_def_id(expr.hir_id) {
            // Use check_match_def to find out whether the cosmwasm_std::CanonicalAddr is used
            if is_canon_addr_call(cx, fn_def_id) {
                // The type is an `Option`
                span_lint_and_help(
                    cx,
                    FILL_ME_IN,
                    expr.span,
                    "you seem to be using a `CanonicalAddr`! This is often not needed and Addr should be used instead.",
                    None,
                    "an `Addr` might work",
                );
                
            } 
        }
    }
}


#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
