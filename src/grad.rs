//! Automatic differentation at compile time.

use crate::{Eval, Expr};

/// Automatically differentiate an expression, optionally at compile time (if evaluated into a `const`).
#[const_trait]
pub trait Grad {
    /// Output of evaluation (i.e. `grad(&x) -> ???`).
    type GradOutput: ~const Eval;
    /// Differentiate an expression with respect to a variable.
    /// Note that this variable is NOT RECOGNIZED BY VALUE:
    /// variables must be
    fn grad<T>(self, x: &T) -> Self::GradOutput;
}

impl<T: ~const Eval + ~const Grad> const Grad for Expr<T> {
    type GradOutput = Expr<T::GradOutput>;
    fn grad<U>(self, x: &U) -> Self::GradOutput {
        Expr(self.0.grad(x))
    }
}
