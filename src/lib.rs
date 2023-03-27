//! Differentiable expression templates in Rust.
//!
//! ### Examples
//!
//! At runtime:
//! ```
//! use dxpr::{Eval, Var};
//! let expression = -Var(4);
//! let value = expression.eval();
//! assert_eq!(-4, value);
//! ```
//!
//! At compile time:
//! ```
//! #![feature(const_trait_impl)]
//! use dxpr::{ops, Eval, Expr, Var};
//! const EXPRESSION: Expr<ops::Neg<Var<i32>>> = -Var(4);
//! const VALUE: i32 = EXPRESSION.eval();
//! assert_eq!(-4, VALUE);
//! ```

#![deny(warnings, missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_precise_live_drops, const_trait_impl)]

pub mod ops;

/// Trait analogous to a function: an output type and a callable, `eval`.
#[const_trait]
pub trait Eval: Sized {
    /// Output of evaluation (i.e. `eval() -> ???`).
    type EvalOutput;
    /// Fold an expression into a value.
    fn eval(self) -> Self::EvalOutput;
}

/// Expression template! Here it is!
#[derive(Debug)]
pub struct Expr<T: ~const Eval>(T);
impl<T: ~const Eval> const Eval for Expr<T> {
    type EvalOutput = T::EvalOutput;
    #[inline(always)]
    fn eval(self) -> Self::EvalOutput {
        self.0.eval()
    }
}

/// Not only a value but a unique identifier w.r.t. which we can differentiate.
#[derive(Debug)]
pub struct Var<T>(pub T);
impl<T> const Eval for Var<T> {
    type EvalOutput = T;
    #[inline(always)]
    fn eval(self) -> Self::EvalOutput {
        self.0
    }
}

#[cfg(test)]
mod test;
