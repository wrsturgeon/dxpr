//! Expression template! Here it is!

use crate::{Eval, EvalRef, Leaf};

/// Expression template! Here it is!
#[derive(Debug)]
pub struct Expr<T: ~const Eval>(pub T);

impl<T: ~const Eval> const Eval for Expr<T> {
    type EvalOutput = T::EvalOutput;
    #[inline(always)]
    fn eval(self) -> Self::EvalOutput {
        self.0.eval()
    }
}
impl<T: ~const EvalRef> const EvalRef for Expr<T> {
    #[inline(always)]
    fn eval(&self) -> Self::EvalOutput {
        #[allow(clippy::needless_borrow)] // TODO: Hey! Tell the clippy team about this! :)
        (&self.0).eval()
    }
}

/// Treat this reference as the basis for a `dxpr` tree rather than as a Rust value to eagerly evaluate.
#[inline(always)]
pub const fn var<T: Leaf>(x: &T) -> Expr<&T> {
    Expr(x)
}
