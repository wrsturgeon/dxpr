//! Mathematical operations as trees.
//! Evaluation traverses depth-first and folds into a value.

use crate::*;

macro_rules! unary_op {
    ($Name:ident, $name:ident, $doc:literal) => {
        #[derive(Debug)]
        #[doc = $doc]
        pub struct $Name<T: ~const Eval>(T);
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
        impl<T: ~const EvalRef> const EvalRef for $Name<T>
        where
            T::EvalOutput: ~const core::ops::$Name,
        {
            #[inline(always)]
            fn eval(&self) -> Self::EvalOutput {
                core::ops::$Name::$name((&self.0).eval())
            }
        }
    };
}

macro_rules! unary_grad {
    ($Name:ident, $Diff:ident) => {
        impl<T: ~const Eval + ~const Grad> const Grad for $Name<T>
        where
            <T::GradOutput as Eval>::EvalOutput: ~const core::ops::$Diff,
        {
            type GradOutput = $Diff<T::GradOutput>;
            fn grad<U>(self, x: &U) -> Self::GradOutput {
                $Diff(self.0.grad(x))
            }
        }
    };
}

macro_rules! binary_op {
    ($Name:ident, $name:ident, $doc:literal) => {
        #[derive(Debug)]
        #[doc = $doc]
        pub struct $Name<L: ~const Eval, R: ~const Eval>(L, R);
        impl<L: ~const Eval, R: ~const Eval> const core::ops::$Name<R> for Expr<L>
        where
            L::EvalOutput: ~const core::ops::$Name<R::EvalOutput>,
        {
            type Output = Expr<$Name<Self, R>>;
            #[inline(always)]
            fn $name(self, arg: R) -> Self::Output {
                Expr($Name(self, arg))
            }
        }
        impl<L: ~const Eval, R: ~const Eval> const Eval for $Name<L, R>
        where
            L::EvalOutput: ~const core::ops::$Name<R::EvalOutput>,
        {
            type EvalOutput = <L::EvalOutput as core::ops::$Name<R::EvalOutput>>::Output;
            #[inline(always)]
            fn eval(self) -> Self::EvalOutput {
                core::ops::$Name::$name(self.0.eval(), self.1.eval())
            }
        }
        impl<L: ~const EvalRef, R: ~const EvalRef> const EvalRef for $Name<L, R>
        where
            L::EvalOutput: ~const core::ops::$Name<R::EvalOutput>,
        {
            #[inline(always)]
            fn eval(&self) -> Self::EvalOutput {
                core::ops::$Name::$name((&self.0).eval(), (&self.1).eval())
            }
        }
    };
}

macro_rules! binary_grad {
    ($Name:ident, $Diff:ident) => {
        impl<L: ~const Eval + ~const Grad, R: ~const Eval + ~const Grad> const Grad for $Name<L, R>
        where
            <L::GradOutput as Eval>::EvalOutput:
                ~const core::ops::$Diff<<R::GradOutput as Eval>::EvalOutput>,
        {
            type GradOutput = $Diff<L::GradOutput, R::GradOutput>;
            fn grad<U>(self, x: &U) -> Self::GradOutput {
                $Diff(self.0.grad(x), self.1.grad(x))
            }
        }
    };
}

// List of ops: https://doc.rust-lang.org/core/ops/

unary_op!(Neg, neg, "Arithmetic negation (e.g. `-4`).");
unary_op!(Not, not, "Logical negation (e.g. `!true`).");

unary_grad!(Neg, Neg);
// TODO...

binary_op!(Add, add, "Arithmetic addition (e.g. `a + b`");
binary_op!(BitAnd, bitand, "Bitwise conjunction (e.g. `a & b`)");
binary_op!(BitOr, bitor, "Bitwise inclusive-or (e.g. `a | b`)");
binary_op!(BitXor, bitxor, "Bitwise exclusive-or (e.g. `a ^ b`)");
binary_op!(Div, div, "Arithmetic division (e.g. `a / b`)"); // TODO: quotient rule
binary_op!(Mul, mul, "Arithmetic multiplication (e.g. `a * b`)"); // TODO: product rule
binary_op!(Rem, rem, "Arithmetic remainder (e.g. `a % b`)"); //TODO: just the left argument
binary_op!(Shl, shl, "Arithmetic left-shift (e.g. `a << b`)"); //TODO: more complicated
binary_op!(Shr, shr, "Arithmetic right-shift (e.g. `a << b`)"); //TODO: more complicated
binary_op!(Sub, sub, "Arithmetic subtraction (e.g. `a - b`)"); //TODO: more complicated

binary_grad!(Add, Add);
binary_grad!(Sub, Sub);
// TODO...

// TODO:
// binary_op!(Index, index, "Subscript indexing (e.g. `a[b]`)");
// binary_op!(RangeBounds, rangebounds, "Range bounds (e.g. `a..b`)");
