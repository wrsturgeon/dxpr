//! Differentiable expression templates in Rust.
//!
//! ## Examples
//!
//! At runtime:
//! ```rust
//! use dxpr::{Eval, var};
//! let x = 4;
//! let a = var(&x);
//! let expression = -a;
//! let value = expression.eval();
//! assert_eq!(-4, value);
//! ```
//!
//! At compile time:
//! ```rust
//! #![feature(const_trait_impl)]
//! use dxpr::{ops, Eval, Expr, var};
//! const X: i32 = 4;
//! const A: Expr<&i32> = var(&X);
//! const EXPRESSION: Expr<ops::Neg<Expr<&i32>>> = -A;
//! const VALUE: i32 = EXPRESSION.eval();
//! assert_eq!(-4, VALUE);
//! ```
//!
//! We can reuse an expression without copying (e.g. for machine learning) by calling `eval` on a reference:
//! ```rust
//! use dxpr::{ops, Eval, EvalRef, Expr, var};
//! let x = 4;
//! let a = var(&x);
//! let expression = -a;
//! assert_eq!(-4, (&expression).eval());
//! assert_eq!(-4, (&expression).eval());
//! assert_eq!(-4, (&expression).eval());
//! // still movable the last time:
//! assert_eq!(-4, expression.eval());
//! // can't use it again:    ------ `expression` moved due to this method call
//! // /* bad! */ assert_eq!(-4, expression.eval());
//! //                           ^^^^^^^^^^ value used here after move
//! ```

#![deny(warnings, missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_precise_live_drops, const_raw_ptr_comparison, const_trait_impl)]

pub mod eval;
pub mod expr;
pub mod grad;
pub mod leaf;
pub mod ops;

pub use eval::{Eval, EvalRef};
pub use expr::{var, Expr};
pub use grad::Grad;
pub use leaf::Leaf;

#[cfg(test)]
mod test;
