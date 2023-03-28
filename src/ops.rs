//! Mathematical operations as trees.
//! Evaluation traverses depth-first and folds into a value.

use crate::{eval, eval::Eval, expr::Expr, grad, grad::Grad};

macro_rules! unary_op {
    ($Name:ident, $name:ident, $doc:literal) => {
        #[derive(Debug)]
        #[doc = $doc]
        pub struct $Name<T: ~const Eval>(T);
        impl<T: ~const Eval> const core::ops::$Name for Expr<T>
        where
            <T as eval::Typed>::Output: ~const core::ops::$Name,
        {
            type Output = Expr<$Name<Self>>;
            #[inline(always)]
            fn $name(self) -> Self::Output {
                Expr($Name(self))
            }
        }
        impl<T: ~const Eval> const eval::Typed for $Name<T>
        where
            <T as eval::Typed>::Output: ~const core::ops::$Name,
        {
            type Output = <<T as eval::Typed>::Output as core::ops::$Name>::Output;
        }
        impl<T: ~const Eval> const eval::Own for $Name<T>
        where
            <T as eval::Typed>::Output: ~const core::ops::$Name,
        {
            #[inline(always)]
            fn eval(self) -> <Self as eval::Typed>::Output {
                core::ops::$Name::$name(self.0.eval())
            }
        }
        impl<T: ~const Eval> const eval::Ref for $Name<T>
        where
            <T as eval::Typed>::Output: ~const core::ops::$Name,
        {
            #[inline(always)]
            fn eval(&self) -> <Self as eval::Typed>::Output {
                core::ops::$Name::$name((&self.0).eval())
            }
        }
        impl<T: ~const Eval> const Eval for $Name<T> where
            <T as eval::Typed>::Output: ~const core::ops::$Name
        {
        }
    };
}

macro_rules! unary_grad {
    ($Name:ident, $Diff:ident) => {
        impl<T: ~const Grad> const grad::Typed for $Name<T>
        where
            <T as eval::Typed>::Output: ~const core::ops::$Name,
            <<T as grad::Typed>::Output as eval::Typed>::Output: ~const core::ops::$Diff,
        {
            type Output = $Diff<<T as grad::Typed>::Output>;
        }
        impl<T: ~const Grad> const grad::Own for $Name<T>
        where
            <T as eval::Typed>::Output: ~const core::ops::$Name,
            <<T as grad::Typed>::Output as eval::Typed>::Output: ~const core::ops::$Diff,
        {
            fn grad<U>(self, x: &U) -> <Self as grad::Typed>::Output {
                $Diff(self.0.grad(x))
            }
        }
        impl<T: ~const Grad> const grad::Ref for $Name<T>
        where
            <T as eval::Typed>::Output: ~const core::ops::$Name,
            <<T as grad::Typed>::Output as eval::Typed>::Output: ~const core::ops::$Diff,
        {
            fn grad<U>(&self, x: &U) -> <Self as grad::Typed>::Output {
                $Diff((&self.0).grad(x))
            }
        }
        impl<T: ~const Grad> const Grad for $Name<T>
        where
            <T as eval::Typed>::Output: ~const core::ops::$Name,
            <<T as grad::Typed>::Output as eval::Typed>::Output: ~const core::ops::$Diff,
        {
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
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>,
        {
            type Output = Expr<$Name<Self, R>>;
            #[inline(always)]
            fn $name(self, arg: R) -> Self::Output {
                Expr($Name(self, arg))
            }
        }
        impl<L: ~const Eval, R: ~const Eval> const eval::Typed for $Name<L, R>
        where
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>,
        {
            type Output = <<L as eval::Typed>::Output as core::ops::$Name<
                <R as eval::Typed>::Output,
            >>::Output;
        }
        impl<L: ~const Eval, R: ~const Eval> const eval::Own for $Name<L, R>
        where
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>,
        {
            #[inline(always)]
            fn eval(self) -> <Self as eval::Typed>::Output {
                core::ops::$Name::$name(self.0.eval(), self.1.eval())
            }
        }
        impl<L: ~const Eval, R: ~const Eval> const eval::Ref for $Name<L, R>
        where
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>,
        {
            #[inline(always)]
            fn eval(&self) -> <Self as eval::Typed>::Output {
                core::ops::$Name::$name((&self.0).eval(), (&self.1).eval())
            }
        }
        impl<L: ~const Eval, R: ~const Eval> const Eval for $Name<L, R> where
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>
        {
        }
    };
}

macro_rules! binary_grad {
    ($Name:ident, $Diff:ident) => {
        impl<L: ~const Grad, R: ~const Grad> const grad::Typed for $Name<L, R>
        where
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>,
            <<L as grad::Typed>::Output as eval::Typed>::Output:
                ~const core::ops::$Diff<<<R as grad::Typed>::Output as eval::Typed>::Output>,
        {
            type Output = $Diff<<L as grad::Typed>::Output, <R as grad::Typed>::Output>;
        }
        impl<L: ~const Grad, R: ~const Grad> const grad::Own for $Name<L, R>
        where
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>,
            <<L as grad::Typed>::Output as eval::Typed>::Output:
                ~const core::ops::$Diff<<<R as grad::Typed>::Output as eval::Typed>::Output>,
        {
            fn grad<U>(self, x: &U) -> <Self as grad::Typed>::Output {
                $Diff(self.0.grad(x), self.1.grad(x))
            }
        }
        impl<L: ~const Grad, R: ~const Grad> const grad::Ref for $Name<L, R>
        where
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>,
            <<L as grad::Typed>::Output as eval::Typed>::Output:
                ~const core::ops::$Diff<<<R as grad::Typed>::Output as eval::Typed>::Output>,
        {
            fn grad<U>(&self, x: &U) -> <Self as grad::Typed>::Output {
                $Diff((&self.0).grad(x), (&self.1).grad(x))
            }
        }
        impl<L: ~const Grad, R: ~const Grad> const Grad for $Name<L, R>
        where
            <L as eval::Typed>::Output: ~const core::ops::$Name<<R as eval::Typed>::Output>,
            <<L as grad::Typed>::Output as eval::Typed>::Output:
                ~const core::ops::$Diff<<<R as grad::Typed>::Output as eval::Typed>::Output>,
        {
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
