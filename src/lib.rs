//! Differentiable expression templates in Rust.
//!
//! ### Examples
//!
//! At runtime:
//! ```rust
//! use dxpr::{Eval, Var};
//! let a = Var(4);
//! let expression = -a;
//! let value = expression.eval();
//! assert_eq!(-4, value);
//! ```
//!
//! At compile time:
//! ```rust
//! #![feature(const_trait_impl)]
//! use dxpr::{ops, Eval, Expr, Var};
//! const A: Var<i32> = Var(4);
//! const EXPRESSION: Expr<ops::Neg<Var<i32>>> = -A;
//! const VALUE: i32 = EXPRESSION.eval();
//! assert_eq!(-4, VALUE);
//! ```
//!
//! Reusing an expression without copying (e.g. for machine learning) requires only that all variables _refer_ to values rather than hold them (look for `Var(&...`):
//! ```rust
//! use dxpr::{ops, EvalRef, Expr, Var};
//! let a = 4;
//! let b = Var(&a);
//! let expression: Expr<ops::Neg<Var<&i32>>> = -b;
//! assert_eq!(-4, (&expression).eval());
//! assert_eq!(-4, (&expression).eval());
//! assert_eq!(-4, (&expression).eval());
//! // still movable the last time, but `a` sticks around
//! assert_eq!(-4, expression.eval());
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

/// Like `Eval` but can be evaluated more than once by taking `&self` instead of `self`.
#[const_trait]
pub trait EvalRef: ~const Eval {
    /// Fold an expression into a value without consuming the expression.
    fn eval(&self) -> Self::EvalOutput;
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
impl<T: ~const EvalRef> const EvalRef for Expr<T> {
    #[inline(always)]
    fn eval(&self) -> Self::EvalOutput {
        EvalRef::eval(&self.0)
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
impl<T> const EvalRef for Var<&T> {
    #[inline(always)]
    fn eval(&self) -> Self::EvalOutput {
        self.0
    }
}

#[cfg(test)]
mod test;
