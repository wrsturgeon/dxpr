//! Automatic differentation at compile time.
//!
//! Dependencies:
//! ```text
//!        *Grad
//!       /     \
//! grad::Own grad::Ref
//!       \     /
//!     grad::Typed
//!          |
//!        *Eval
//! ```

use crate::eval::Eval;

/// Output type.
#[const_trait]
pub trait Typed: ~const Eval {
    /// Output of evaluation (i.e. `grad(&x) -> ???`).
    type Output: ~const Eval;
}

/// Implementation taking `self` (moved).
#[const_trait]
pub trait Own: ~const Typed {
    /// Fold an expression into a value.
    fn grad<U>(self, x: &U) -> <Self as Typed>::Output;
}

/// Implementation taking `&self` (not moved).
#[const_trait]
pub trait Ref: ~const Typed {
    /// Fold an expression into a value without consuming the expression.
    fn grad<U>(&self, x: &U) -> <Self as Typed>::Output;
}

/// Automatically differentiate an expression, optionally at compile time (if evaluated into a `const`).
#[const_trait]
pub trait Grad: ~const Own + ~const Ref {}
