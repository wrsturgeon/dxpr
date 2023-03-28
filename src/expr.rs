//! Expression template! Here it is!

use crate::{eval, eval::Eval, grad, grad::Grad, leaf::Leaf};

/// Expression template! Here it is!
#[derive(Debug)]
pub struct Expr<T: ~const Eval>(pub T);

impl<T: ~const Eval> const eval::Typed for Expr<T> {
    type Output = <T as eval::Typed>::Output;
}
impl<T: ~const Eval> const eval::Own for Expr<T> {
    #[inline(always)]
    fn eval(self) -> <Self as eval::Typed>::Output {
        self.0.eval()
    }
}
impl<T: ~const Eval> const eval::Ref for Expr<T> {
    #[inline(always)]
    fn eval(&self) -> <Self as eval::Typed>::Output {
        #[allow(clippy::needless_borrow)] // TODO: Hey! Tell the clippy team about this! :)
        (&self.0).eval()
    }
}
impl<T: ~const Eval> const Eval for Expr<T> {}

impl<T: ~const Grad> const grad::Typed for Expr<T> {
    type Output = Expr<<T as grad::Typed>::Output>;
}
impl<T: ~const Grad> const grad::Own for Expr<T> {
    #[inline(always)]
    fn grad<U>(self, x: &U) -> <Self as grad::Typed>::Output {
        Expr(self.0.grad(x))
    }
}
impl<T: ~const Grad> const grad::Ref for Expr<T> {
    #[inline(always)]
    fn grad<U>(&self, x: &U) -> <Self as grad::Typed>::Output {
        #[allow(clippy::needless_borrow)] // TODO: this as well
        Expr((&self.0).grad(x))
    }
}
impl<T: ~const Grad> const Grad for Expr<T> {}

/// Treat this reference as the basis for a `dxpr` tree rather than as a Rust value to eagerly evaluate.
#[inline(always)]
pub const fn var<T: Leaf>(x: &T) -> Expr<&T> {
    Expr(x)
}
