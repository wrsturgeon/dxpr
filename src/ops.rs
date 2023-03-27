//! Mathematical operations as trees.
//! Evaluation traverses depth-first and folds into a value.

use crate::*;

macro_rules! unary_op {
    ($Name:ident, $name:ident, $doc:literal) => {
        #[derive(Debug)]
        #[doc = $doc]
        pub struct $Name<T: ~const Eval>(T)
        where
            T::EvalOutput: ~const core::ops::$Name;
        impl<T: ~const core::ops::$Name> const core::ops::$Name for Var<T> {
            type Output = Expr<$Name<Self>>;
            #[inline(always)]
            fn $name(self) -> Self::Output {
                Expr($Name(self))
            }
        }
        impl<T: ~const Eval> const core::ops::$Name for Expr<T>
        where
            T::EvalOutput: ~const core::ops::$Name,
        {
            type Output = Expr<$Name<Self>>;
            #[inline(always)]
            fn $name(self) -> Self::Output {
                Expr($Name(self))
            }
        }
        impl<T: ~const Eval> const Eval for $Name<T>
        where
            T::EvalOutput: ~const core::ops::$Name,
        {
            type EvalOutput = <T::EvalOutput as core::ops::$Name>::Output;
            #[inline(always)]
            fn eval(self) -> Self::EvalOutput {
                core::ops::$Name::$name(self.0.eval())
            }
        }
    };
}

unary_op!(Neg, neg, "Arithmetic negation (e.g. `-4`).");
unary_op!(Not, not, "Logical negation (e.g. `!true`).");
