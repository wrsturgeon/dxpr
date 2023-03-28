# dxpr
Differentiable expression templates in Rust.

## Use

At runtime:
```
use dxpr::{Eval, Var};
let expression = -Var(4);
let value = expression.eval();
assert_eq!(-4, value);
```

At compile time:
```
#![feature(const_trait_impl)]
use dxpr::{ops, Eval, Expr, Var};
const EXPRESSION: Expr<ops::Neg<Var<i32>>> = -Var(4);
const VALUE: i32 = EXPRESSION.eval();
assert_eq!(-4, VALUE);
```

## To do:
- Implement the full set of `core::ops`
- Assign variables unique IDs
  - ...at compile time
    - NOT CURRENTLY AVAILABLE: can't mutate statics at compile time
    - Alternative: manually specify ID
- Differentiate
