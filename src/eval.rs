//! Traits to provide `.eval()`.

/// Trait analogous to a function: an output type and a callable, `eval`.
#[const_trait]
pub trait Eval: Sized {
    /// Output of evaluation (i.e. `eval() -> ???`).
    type EvalOutput;
    /// Fold an expression into a value.
    fn eval(self) -> Self::EvalOutput;
}

/// Like `Eval` but can be evaluated more than once by taking `&self` instead of `self`.
#[const_trait]
pub trait EvalRef: ~const Eval {
    /// Fold an expression into a value without consuming the expression.
    fn eval(&self) -> Self::EvalOutput;
}
