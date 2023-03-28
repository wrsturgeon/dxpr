//! Traits to provide `.eval()`.
//!
//! Dependencies:
//! ```text
//!        *Eval
//!       /     \
//! eval::Own eval::Ref
//!       \     /
//!     eval::Typed
//!          |
//!        Sized
//! ```

/// Output type.
#[const_trait]
pub trait Typed: Sized {
    /// Output of evaluation (i.e. `eval() -> ???`).
    type Output;
}

/// Implementation taking `self` (moved).
#[const_trait]
pub trait Own: ~const Typed {
    /// Fold an expression into a value.
    fn eval(self) -> <Self as Typed>::Output;
}

/// Implementation taking `&self` (not moved).
#[const_trait]
pub trait Ref: ~const Typed {
    /// Fold an expression into a value without consuming the expression.
    fn eval(&self) -> <Self as Typed>::Output;
}

/// Trait analogous to a function: an output type and a callable, `eval`.
#[const_trait]
pub trait Eval: ~const Own + ~const Ref {}
