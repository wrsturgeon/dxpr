//! Trait to provide analogies to 0 (`ZERO`) and 1 (`UNIT`).

use crate::{eval, eval::Eval, grad, grad::Grad};

/// Analogies to 0 (`ZERO`) and 1 (`UNIT`).
pub trait Leaf {
    /// Type of `UNIT` and `ZERO`.
    type Output: ~const Eval;
    /// Analogous to 1.
    const UNIT: Self::Output;
    /// Analogous to 0.
    const ZERO: Self::Output;
}

impl<T: Leaf> const eval::Typed for &T {
    type Output = Self;
}
impl<T: Leaf> const eval::Own for &T {
    fn eval(self) -> <Self as eval::Typed>::Output {
        self
    }
}
impl<T: Leaf> const eval::Ref for &T {
    fn eval(&self) -> <Self as eval::Typed>::Output {
        self
    }
}
impl<T: Leaf> const Eval for &T {}

impl<T: Leaf> const grad::Typed for &T {
    type Output = <T as Leaf>::Output;
}
impl<T: Leaf> const grad::Own for &T {
    fn grad<U>(self, x: &U) -> <Self as grad::Typed>::Output {
        match (x as *const U).guaranteed_eq(self as *const _ as *const U) {
            None => panic!("Couldn't tell whether two values were the same (this often happens at compile time and seems to be an issue with Rust itself)"),
            Some(false) => T::ZERO,
            Some(true) => T::UNIT,
        }
    }
}
impl<T: Leaf> const grad::Ref for &T {
    fn grad<U>(&self, x: &U) -> <Self as grad::Typed>::Output {
        match (x as *const U).guaranteed_eq(self as *const _ as *const U) {
            None => panic!("Couldn't tell whether two values were the same (this often happens at compile time and seems to be an issue with Rust itself)"),
            Some(false) => T::ZERO,
            Some(true) => T::UNIT,
        }
    }
}
impl<T: Leaf> const Grad for &T {}

macro_rules! impl_leaf {
    ($t:ident, $z:expr, $u:expr) => {
        impl Leaf for $t {
            type Output = &'static Self;
            const UNIT: Self::Output = &$u;
            const ZERO: Self::Output = &$z;
        }
    };
}

impl_leaf!(bool, false, true);
impl_leaf!(u8, 0, 1);
impl_leaf!(u16, 0, 1);
impl_leaf!(u32, 0, 1);
impl_leaf!(u64, 0, 1);
impl_leaf!(u128, 0, 1);
impl_leaf!(i8, 0, 1);
impl_leaf!(i16, 0, 1);
impl_leaf!(i32, 0, 1);
impl_leaf!(i64, 0, 1);
impl_leaf!(i128, 0, 1);
impl_leaf!(f32, 0., 1.);
impl_leaf!(f64, 0., 1.);
