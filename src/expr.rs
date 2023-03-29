//! Expression template! Here it is!

use crate::{eval, eval::Eval, grad, grad::Grad, leaf::Leaf};

/// Expression template! Here it is!
#[derive(Debug)]
pub struct Expr<T: ~const Eval>(pub(crate) T);

// TODO: Hey! Tell the clippy team about this! :)
crate::implement_eval!(T: Eval => Expr<T> >-> <T as eval::Typed>::Output: |self| where own { self.0.eval() } else { #[allow(clippy::needless_borrow)] (&self.0).eval() });
crate::implement_grad!(T: Grad => Expr<T> >-> Expr<<T as grad::Typed>::Output>: |self, x| where own { Expr(self.0.grad(x)) } else { #[allow(clippy::needless_borrow)] Expr((&self.0).grad(x)) });

/// Treat this reference as the basis for a `dxpr` tree rather than as a Rust value to eagerly evaluate.
#[inline(always)]
pub const fn var<T: ~const Leaf>(x: &T) -> Expr<&T> {
    Expr(x)
}
