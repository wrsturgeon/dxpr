//! Trait to provide analogies to 0 (`ZERO`) and 1 (`UNIT`).

use crate::{Eval, EvalRef, Expr, Grad};

/// Analogies to 0 (`ZERO`) and 1 (`UNIT`).
pub trait Leaf {
    /// Type of `UNIT` and `ZERO`.
    type LeafOutput: ~const crate::Eval;
    /// Analogous to 1.
    const UNIT: Self::LeafOutput;
    /// Analogous to 0.
    const ZERO: Self::LeafOutput;
}

impl<T: Leaf> const Eval for &T {
    type EvalOutput = Self;
    fn eval(self) -> Self::EvalOutput {
        self
    }
}

impl<T: Leaf> const EvalRef for &T {
    fn eval(&self) -> Self::EvalOutput {
        self
    }
}

impl<T: Leaf> Grad for &T {
    type GradOutput = Expr<<T as Leaf>::LeafOutput>;
    fn grad<U>(self, x: &U) -> Self::GradOutput {
        match (x as *const U).guaranteed_eq(self as *const _ as *const U) {
            None => panic!(),
            Some(false) => Expr(T::ZERO),
            Some(true) => Expr(T::UNIT),
        }
    }
}

macro_rules! impl_leaf {
    ($t:ident, $z:expr, $u:expr) => {
        impl Leaf for $t {
            type LeafOutput = &'static Self;
            const UNIT: Self::LeafOutput = &$u;
            const ZERO: Self::LeafOutput = &$z;
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
