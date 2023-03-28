# dxpr
Differentiable expression templates in Rust.

## Examples

At runtime:
```rust
use dxpr::{Eval, var};
let x = 4;
let a = var(&x);
let expression = -a;
let value = expression.eval();
assert_eq!(-4, value);
```

At compile time:
```rust
#![feature(const_trait_impl)]
use dxpr::{ops, Eval, Expr, var};
const X: i32 = 4;
const A: Expr<&i32> = var(&X);
const EXPRESSION: Expr<ops::Neg<Expr<&i32>>> = -A;
const VALUE: i32 = EXPRESSION.eval();
assert_eq!(-4, VALUE);
```

Automatic differentiation:
```rust   
use dxpr::{var, Eval, Grad};
let a = 4;
let b = 4;
let ga = (var(&a) - var(&b)).grad(&a);
let gb = (var(&a) - var(&b)).grad(&b);
assert_eq!(1, ga.eval());
assert_eq!(-1, gb.eval());
```

We can reuse an expression without copying (e.g. for machine learning) by calling `eval` on a reference:
```rust
use dxpr::{ops, Eval, EvalRef, Expr, var};
let x = 4;
let expression = -var(&x);
assert_eq!(-4, (&expression).eval());
assert_eq!(-4, (&expression).eval());
assert_eq!(-4, (&expression).eval());
// still movable the last time:
assert_eq!(-4, expression.eval());
// can't use it again:    ------ `expression` moved due to this method call
// /* bad! */ assert_eq!(-4, expression.eval());
//                           ^^^^^^^^^^ value used here after move
```

## To do:
- Differentiation on arbitrary expressions
- `Index` and `RangeBound` operators (i.e. `a[b]` and `a..b`)
