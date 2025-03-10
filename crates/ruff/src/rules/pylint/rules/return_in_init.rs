use rustpython_parser::ast::{Constant, ExprKind, Stmt, StmtKind};

use ruff_macros::{define_violation, derive_message_formats};

use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::rules::pylint::helpers::in_dunder_init;
use crate::violation::Violation;

define_violation!(
    /// ## What it does
    /// Checks for `__init__` methods that return values.
    ///
    /// ## Why is this bad?
    /// The `__init__` method is the constructor for a given Python class,
    /// responsible for initializing, rather than creating, new objects.
    ///
    /// The `__init__` method has to return `None`. Returning any value from
    /// an `__init__` method will result in a runtime error.
    ///
    /// ## Example
    /// ```python
    /// class Example:
    ///     def __init__(self):
    ///         return []
    /// ```
    ///
    /// Use instead:
    /// ```python
    /// class Example:
    ///    def __init__(self):
    ///       self.value = []
    /// ```
    ///
    /// ## References
    /// * [CodeQL: `py-explicit-return-in-init`](https://codeql.github.com/codeql-query-help/python/py-explicit-return-in-init/)
    pub struct ReturnInInit;
);
impl Violation for ReturnInInit {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Explicit return in `__init__`")
    }
}

/// PLE0101
pub fn return_in_init(checker: &mut Checker, stmt: &Stmt) {
    if let StmtKind::Return { value } = &stmt.node {
        if let Some(expr) = value {
            if matches!(
                expr.node,
                ExprKind::Constant {
                    value: Constant::None,
                    ..
                }
            ) {
                // Explicit `return None`.
                return;
            }
        } else {
            // Implicit `return`.
            return;
        }
    }

    if in_dunder_init(checker) {
        checker
            .diagnostics
            .push(Diagnostic::new(ReturnInInit, Range::from_located(stmt)));
    }
}
