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
    type Output;
}

/// Implementation taking `self` (moved).
#[const_trait]
pub trait Own: ~const Typed {
    /// Fold an expression into a value.
    fn eval(self) -> <Self as Typed>::Output;
}

/// Implementation taking `&self` (not moved).
#[const_trait]
pub trait Ref: ~const Typed {
    /// Fold an expression into a value without consuming the expression.
    fn eval(&self) -> <Self as Typed>::Output;
}

/// Trait analogous to a function: an output type and a callable, `eval`.
#[const_trait]
pub trait Eval: ~const Own + ~const Ref {}

/// Automagically implement `Eval`.
#[macro_export]
macro_rules! implement_eval {
    ($name:ty >-> $output:ty: |$self:ident| $body:expr) => {
        impl const $crate::eval::Typed for $name {
            type Output = $output;
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
    ($t:ident: $const_trait:ident => $name:ty >-> $output:ty: |$self:ident| $body:expr) => {
        impl<$t: ~const $const_trait> const $crate::eval::Typed for $name {
            type Output = $output;
        }
        impl<$t: ~const $const_trait> const $crate::eval::Own for $name {
            #[inline(always)]
            fn eval($self) -> $output {
                $body
            }
        }
        impl<$t: ~const $const_trait> const $crate::eval::Ref for $name {
            #[inline(always)]
            fn eval(&$self) -> $output {
                $body
            }
        }
        impl<$t: ~const $const_trait> const $crate::eval::Eval for $name {}
    };
    // It would be so nice to have `where` as a parallel to C++'s `if constexpr`...
    ($t:ident: $const_trait:ident => $name:ty >-> $output:ty: |$self:ident| where own { $own:expr } else { $ref:expr }) => {
        impl<$t: ~const $const_trait> const $crate::eval::Typed for $name {
            type Output = $output;
        }
        impl<$t: ~const $const_trait> const $crate::eval::Own for $name {
            #[inline(always)]
            fn eval($self) -> $output {
                $own
            }
        }
        impl<$t: ~const $const_trait> const $crate::eval::Ref for $name {
            #[inline(always)]
            fn eval(&$self) -> $output {
                $ref
            }
        }
        impl<$t: ~const $const_trait> const $crate::eval::Eval for $name {}
    };
}
