//! Differentiable expression templates in Rust.
//!
//! ## Examples
//!
//! At runtime:
//! ```rust
//! use dxpr::prelude::*;
//! let x = 4;
//! assert_eq!(-4, (-var(&x)).eval());
//! assert_eq!(-1, (-var(&x)).grad(&x).eval());
//! assert_eq!(0, (-var(&x)).grad(&x).grad(&x).eval());
//! assert_eq!(0, (-var(&x)).grad(&x).grad(&x).grad(&x).grad(&x).grad(&x).eval());
//! ```
//!
//! At compile time:
//! ```rust
//! #![feature(const_trait_impl)]
//! use dxpr::{expr::Expr, ops, prelude::*};
//! const X: i32 = 4;
//! const A: Expr<&i32> = var(&X);
//! const EXPRESSION: Expr<ops::Neg<Expr<&i32>>> = -A;
//! const VALUE: i32 = EXPRESSION.eval();
//! assert_eq!(-4, VALUE);
//! // Rust currently can't compare pointers to constants at compile time, so no compile-time autodiff at the moment
//! // Plans in the works to fix this with the ability to manually specify a "variable ID."
//! // Either way, your `build.rs` could easily evaluate an arbitrary expression and write it, unevaluated, to a file.
//! ```
//!
//! We can reuse an expression without copying any variables by calling `eval` or `grad` on a reference:
//! ```rust
//! use dxpr::prelude::*;
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

/// Convenient traits.
pub mod prelude {
    pub use crate::eval::{Own as _, Ref as _};
    pub use crate::expr::var;
    pub use crate::grad::{Own as _, Ref as _};
}

#[cfg(test)]
mod test;
