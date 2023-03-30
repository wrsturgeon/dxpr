//! Automatic differentation at compile time.
//!
//! Dependencies:
//! ```text
//!        *Grad
//!       /     \
//! grad::Own grad::Ref
//!       \     /
//!     grad::Typed
//!          |
//!        *Eval
//! ```

use crate::eval::Eval;

/// Output type.
#[const_trait]
pub trait Typed: ~const Eval {
    /// Output of evaluation (i.e. `grad(&x) -> ???`).
    type Differentiated: ~const Eval;
}

/// Implementation taking `self` (moved).
#[const_trait]
pub trait Own: ~const Typed {
    /// Fold an expression into a value.
    fn grad<U>(self, x: &U) -> Self::Differentiated;
}

/// Implementation taking `&self` (not moved).
#[const_trait]
pub trait Ref: ~const Typed {
    /// Fold an expression into a value without consuming the expression.
    fn grad<U>(&self, x: &U) -> Self::Differentiated;
}

/// Automatically differentiate an expression, optionally at compile time (if evaluated into a `const`).
#[const_trait]
pub trait Grad: ~const Own + ~const Ref {}

/// Automagically implement `Grad`.
#[macro_export]
macro_rules! implement_grad {
    ($name:ty >-> $output:ty: |$self:ident, $x:ident| $body:expr) => {
        impl const $crate::grad::Typed for $name {
            type Differentiated = $output;
        }
        impl const $crate::grad::Own for $name {
            #[inline(always)]
            fn grad<U>($self, $x: &U) -> $output {
                $body
            }
        }
        impl const $crate::grad::Ref for $name {
            #[inline(always)]
            fn grad<U>(&$self, $x: &U) -> $output {
                $body
            }
        }
        impl const $crate::grad::Grad for $name {}
    };
    ($($t:ident: $const_trait:path),+ => $name:ty >-> $output:ty: |$self:ident, $x:ident| $body:expr) => {
        impl<$($t: ~const $const_trait),+> const $crate::grad::Typed for $name {
            type Differentiated = $output;
        }
        impl<$($t: ~const $const_trait),+> const $crate::grad::Own for $name {
            #[inline(always)]
            fn grad<U>($self, $x: &U) -> $output {
                $body
            }
        }
        impl<$($t: ~const $const_trait),+> const $crate::grad::Ref for $name {
            #[inline(always)]
            fn grad<U>(&$self, $x: &U) -> $output {
                $body
            }
        }
        impl<$($t: ~const $const_trait),+> const $crate::grad::Grad for $name {}
    };
    // It would be so nice to have `where` as a parallel to C++'s `if constexpr`...
    ($($t:ident: $const_trait:path),+ => $name:ty >-> $output:ty: |$self:ident, $x:ident| where own { $own:expr } else { $ref:expr }) => {
        impl<$($t: ~const $const_trait),+> const $crate::grad::Typed for $name {
            type Differentiated = $output;
        }
        impl<$($t: ~const $const_trait),+> const $crate::grad::Own for $name {
            #[inline(always)]
            fn grad<U>($self, $x: &U) -> $output {
                $own
            }
        }
        impl<$($t: ~const $const_trait),+> const $crate::grad::Ref for $name {
            #[inline(always)]
            fn grad<U>(&$self, $x: &U) -> $output {
                $ref
            }
        }
        impl<$($t: ~const $const_trait),+> const $crate::grad::Grad for $name {}
    };
}
