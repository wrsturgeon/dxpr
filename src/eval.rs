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
    type Evaluated;
}

/// Implementation taking `self` (moved).
#[const_trait]
pub trait Own: ~const Typed {
    /// Fold an expression into a value.
    fn eval(self) -> Self::Evaluated;
}

/// Implementation taking `&self` (not moved).
#[const_trait]
pub trait Ref: ~const Typed {
    /// Fold an expression into a value without consuming the expression.
    fn eval(&self) -> Self::Evaluated;
}

/// Trait analogous to a function: an output type and a callable, `eval`.
#[const_trait]
pub trait Eval: ~const Own + ~const Ref {}

/// Automagically implement `Eval`.
#[macro_export]
macro_rules! implement_eval {
    ($name:ty >-> $output:ty: |$self:ident| $body:expr) => {
        impl const $crate::eval::Typed for $name {
            type Evaluated = $output;
        }
        impl const $crate::eval::Own for $name {
            #[inline(always)]
            fn eval($self) -> $output {
                $body
            }
        }
        impl const $crate::eval::Ref for $name {
            #[inline(always)]
            fn eval(&$self) -> $output {
                $body
            }
        }
        impl const Eval for $name {}
    };
    ($($t:ident: $const_trait:path),+ => $name:ty >-> $output:ty: |$self:ident| $body:expr) => {
        impl<$($t: ~const $const_trait),+> const $crate::eval::Typed for $name {
            type Evaluated = $output;
        }
        impl<$($t: ~const $const_trait),+> const $crate::eval::Own for $name {
            #[inline(always)]
            fn eval($self) -> $output {
                $body
            }
        }
        impl<$($t: ~const $const_trait),+> const $crate::eval::Ref for $name {
            #[inline(always)]
            fn eval(&$self) -> $output {
                $body
            }
        }
        impl<$($t: ~const $const_trait),+> const $crate::eval::Eval for $name {}
    };
    // It would be so nice to have `where` as a parallel to C++'s `if constexpr`...
    ($($t:ident: $const_trait:path),+ => $name:ty >-> $output:ty: |$self:ident| where own { $own:expr } else { $ref:expr }) => {
        impl<$($t: ~const $const_trait),+> const $crate::eval::Typed for $name {
            type Evaluated = $output;
        }
        impl<$($t: ~const $const_trait),+> const $crate::eval::Own for $name {
            #[inline(always)]
            fn eval($self) -> $output {
                $own
            }
        }
        impl<$($t: ~const $const_trait),+> const $crate::eval::Ref for $name {
            #[inline(always)]
            fn eval(&$self) -> $output {
                $ref
            }
        }
        impl<$($t: ~const $const_trait),+> const $crate::eval::Eval for $name {}
    };
}

/// Take a function and make it lazily evaluable (under the same name, but in mod \[this mod]::dxpr)
#[macro_export]
macro_rules! op_from_fn {
    ($existing_fn:ident($($arg:ident: $t:ty),+) -> $o:ty) => {
        mod dxpr {
            #![allow(non_camel_case_types)]
            mod ops {
                use $crate::eval::Eval;
                pub struct $existing_fn<$($arg: ~const Eval<Evaluated = $t>),+> { $($arg: $crate::expr::Expr<$arg>),+ }
                impl<$($arg: ~const Eval<Evaluated = $t>),+> $existing_fn<$($arg),+> {
                    #[inline(always)]
                    pub const fn new($($arg: $crate::expr::Expr<$arg>),+) -> Self {
                        Self { $($arg),+ }
                    }
                }
                $crate::implement_eval!(
                    $($arg: Eval<Evaluated = $t>),+ =>
                    $existing_fn<$($arg),+> >-> $o:
                    |self| where own {
                        super::super::$existing_fn($(self.$arg.eval()),+)
                    } else {
                        super::super::$existing_fn($((&self.$arg).eval()),+)
                    }
                );
            }
            #[inline(always)]
            pub const fn $existing_fn<$($arg: ~const $crate::eval::Eval<Evaluated = $t>),+>($($arg: $crate::expr::Expr<$arg>),+) -> ops::$existing_fn<$($arg),+> {
                ops::$existing_fn::new($($arg),+)
            }
        }
    };
}

const fn shit(a: i32, b: i32) -> i32 {
    a + b
}

op_from_fn!(shit(a: i32, b: i32) -> i32);

#[allow(unused)]
const fn check() {
    #![allow(clippy::assertions_on_constants)]
    use crate::{expr::Expr, ops::Add, prelude::*};
    const A: Expr<Add<&i32, &i32>> = var(&1) + var(&2); // -> 3
    const B: Expr<Add<&i32, &i32>> = var(&4) + var(&8); // -> 12
    const C: i32 = dxpr::shit(A, B).eval(); // -> 15
    assert!(C == 15);
}
const _: () = check();
