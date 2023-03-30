//! Trait to provide analogies to 0 (`ZERO`) and 1 (`UNIT`).

use crate::{eval::Eval, grad, grad::Grad};

/// Analogies to 0 (`ZERO`) and 1 (`UNIT`).
#[const_trait]
pub trait Leaf {
    /// Type of `UNIT` and `ZERO`.
    type Unit;
    /// Analogous to 1.
    const UNIT: Self::Unit;
    /// Analogous to 0.
    const ZERO: Self::Unit;
}

crate::implement_eval!(T: Leaf => &T >-> Self: |self| self);

impl<T: ~const Leaf<Unit: ~const Eval>> const grad::Typed for &T {
    type Differentiated = T::Unit;
}
impl<T: ~const Leaf<Unit: ~const Eval>> const grad::Own for &T {
    #[inline(always)]
    fn grad<U>(self, x: &U) -> Self::Differentiated {
        match (x as *const U as *const T).guaranteed_eq(self as *const T) {
            None => panic!("Couldn't tell whether two values were the same (this often happens at compile time and seems to be an issue with Rust itself)"),
            Some(false) => T::ZERO,
            Some(true) => T::UNIT,
        }
    }
}
impl<T: ~const Leaf<Unit: ~const Eval>> const grad::Ref for &T {
    #[inline(always)]
    fn grad<U>(&self, x: &U) -> Self::Differentiated {
        match (x as *const U as *const T).guaranteed_eq(*self as *const T) {
            None => panic!("Couldn't tell whether two values were the same (this often happens at compile time and seems to be an issue with Rust itself)"),
            Some(false) => T::ZERO,
            Some(true) => T::UNIT,
        }
    }
}
impl<T: ~const Leaf<Unit: ~const Eval>> const Grad for &T {}

/// Automagically implement `Leaf`.
macro_rules! implement_leaf {
    ($t:ty, $z:expr, $u:expr) => {
        impl const Leaf for $t {
            type Unit = &'static Self;
            const UNIT: Self::Unit = &$u;
            const ZERO: Self::Unit = &$z;
        }
    };
}

implement_leaf!(bool, false, true);
implement_leaf!(u8, 0, 1);
implement_leaf!(u16, 0, 1);
implement_leaf!(u32, 0, 1);
implement_leaf!(u64, 0, 1);
implement_leaf!(u128, 0, 1);
implement_leaf!(i8, 0, 1);
implement_leaf!(i16, 0, 1);
implement_leaf!(i32, 0, 1);
implement_leaf!(i64, 0, 1);
implement_leaf!(i128, 0, 1);
implement_leaf!(f32, 0., 1.);
implement_leaf!(f64, 0., 1.);
