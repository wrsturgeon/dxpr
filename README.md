# dxpr
Differentiable expression templates in Rust.

## Use

At runtime:
```rust
use dxpr::{Eval, Var};
let a = Var(4);
let expression = -a;
let value = expression.eval();
assert_eq!(-4, value);
```

At compile time:
```rust
#![feature(const_trait_impl)]
use dxpr::{ops, Eval, Expr, Var};
const A: Var<i32> = Var(4);
const EXPRESSION: Expr<ops::Neg<Var<i32>>> = -A;
const VALUE: i32 = EXPRESSION.eval();
assert_eq!(-4, VALUE);
```

Reusing an expression without copying (e.g. for machine learning) requires only that all variables _refer_ to values rather than hold them (look for `Var(&...`):
```rust
use dxpr::{ops, EvalRef, Expr, Var};
let a = 4;
let b = Var(&a);
let expression: Expr<ops::Neg<Var<&i32>>> = -b;
assert_eq!(-4, (&expression).eval());
assert_eq!(-4, (&expression).eval());
assert_eq!(-4, (&expression).eval());
// still movable the last time, but `a` sticks around
assert_eq!(-4, expression.eval());
```

## To do:
- Implement the full set of `core::ops`
- Assign variables unique IDs
  - ...at compile time
    - NOT CURRENTLY AVAILABLE: can't mutate statics at compile time
    - Alternative: manually specify ID
- Differentiate
