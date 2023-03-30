# dxpr
Differentiable expression templates in compile-time, dependency-free, no_std Rust.

## Examples

Basics:
```rust
use dxpr::prelude::*;
let x = 4; // Any type works; no special wrappers
let expr = -var(&x); // Unevaluated expression
assert_eq!(-4, expr.eval()); // Evaluated (expr moved)
let dvdx = (-var(&x)).grad(&x); // Automatic differentiation!
// ...into another differentiable expression!
assert_eq!(-1, (&dvdx).eval()); // dvdx NOT moved yet: reusable
assert_eq!(0, dvdx.grad(&x).grad(&x).grad(&x).grad(&x).eval());
```

Want to implement differentiable expressions for your own types and functions?
```rust
#![feature(const_trait_impl)]
use dxpr::{prelude::*, eval::Eval};
struct Teleport { arg: u8 }; // Unevaluated Teleport operation
const fn tp(x: u8) -> Teleport { Teleport { arg: x } } // Not yet!
dxpr::implement_eval!( // Now define what the operation does:
  Teleport >-> u16: // Op type >-> output type
  |self| (self.arg as u16) << 8); // Function definition
let _ = tp(1); // Unevaluated expression
assert_eq!(256, tp(1).eval()); // Done!
```

At compile time:
```rust
#![feature(const_trait_impl)]
use dxpr::{expr::Expr, ops::Neg, prelude::*};
const X: i32 = 4;
const A: Expr<&i32> = var(&X);
const EXPRESSION: Expr<Neg<Neg<Neg<&i32>>>> = ---A;
const VALUE: i32 = EXPRESSION.eval();
assert_eq!(-4, VALUE);
// Rust currently can't compare pointers to constants at compile time, so no compile-time autodiff at the moment
// Plans in the works to fix this with the ability to manually specify a "variable ID."
// Either way, your `build.rs` could easily evaluate an arbitrary expression and write it, unevaluated, to a file.
```

## To do:
- Figure out why Rust doesn't like `+` in macro `:path` arguments
- Interoperability with builtin constants
- `Index` and `RangeBound` operators (i.e. `a[b]` and `a..b`)
