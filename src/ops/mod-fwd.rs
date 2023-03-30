//! Mathematical operations as trees.
//! Evaluation traverses depth-first and folds into a value.
//!
//! NOTE: Rust's orphan rule creates the need to wrap everything in `Expr`.
//! We would be able to define addition between traits e.g. in C++,
//! but since addition isn't defined in this crate (it's in `core`),
//! Rust prevents us from blanket-implementing it in this crate.
//! But we can always get away with keeping `Expr` wrapping only the outer layer:
//! every time we add an operation to the tree, keep only its `self.0`.

mod mul;

use crate::{eval, eval::Eval, expr::Expr, grad, grad::Grad};

macro_rules! unary_op {
    ($Name:ident, $name:ident, $doc:literal) => {
        #[derive(Debug)]
        #[doc = $doc]
        pub struct $Name<T: ~const Eval>(Expr<T>);
        impl<T: ~const Eval> const core::ops::$Name for Expr<T> {
            type Output = Expr<$Name<Expr<T>>>;
            #[inline(always)]
            fn $name(self) -> Self::Output {
                Expr($Name(Expr(self)))
            }
        }
        $crate::implement_eval!(
            T: Eval<Evaluated: ~const core::ops::$Name> =>
            $Name<Expr<T>> >-> <T::Evaluated as core::ops::$Name>::Output:
            |self| where own {
                core::ops::$Name::$name(self.0.eval())
            } else {
                core::ops::$Name::$name((&self.0).eval())
            }
        );
    };
}

macro_rules! unary_grad {
    ($Name:ident, $Diff:ident) => {
        // TODO: see end of this block
        impl<
                T: ~const Eval<Evaluated: ~const core::ops::$Name>
                    + ~const Grad<Differentiated: ~const Eval<Evaluated: ~const core::ops::$Diff>>,
            > const grad::Typed for $Name<Expr<T>>
        {
            type Differentiated = $Diff<Expr<T::Differentiated>>;
        }
        impl<
                T: ~const Eval<Evaluated: ~const core::ops::$Name>
                    + ~const Grad<Differentiated: ~const Eval<Evaluated: ~const core::ops::$Diff>>,
            > const grad::Own for $Name<Expr<T>>
        {
            #[inline(always)]
            fn grad<U>(self, x: &U) -> Self::Differentiated {
                $Diff(self.0.grad(x))
            }
        }
        impl<
                T: ~const Eval<Evaluated: ~const core::ops::$Name>
                    + ~const Grad<Differentiated: ~const Eval<Evaluated: ~const core::ops::$Diff>>,
            > const grad::Ref for $Name<Expr<T>>
        {
            #[inline(always)]
            fn grad<U>(&self, x: &U) -> Self::Differentiated {
                $Diff((&self.0).grad(x))
            }
        }
        impl<
                T: ~const Eval<Evaluated: ~const core::ops::$Name>
                    + ~const Grad<Differentiated: ~const Eval<Evaluated: ~const core::ops::$Diff>>,
            > const Grad for $Name<Expr<T>>
        {
        }
        // TODO: Rust doesn't like `+`, but apart from that, this should work
        // $crate::implement_grad!(
        //     T: Eval<Evaluated: ~const core::ops::$Name> + ~const Grad<Differentiated: ~const Eval<Evaluated: ~const core::ops::$Diff>> =>
        //     $Name<Expr<T>> >-> $Diff<T::Differentiated>:
        //     |self, x| where own {
        //         $Diff(self.0.grad(x))
        //     } else {
        //         $Diff((&self.0).grad(x))
        //     }
        // );
    };
}

macro_rules! binary_op {
    ($Name:ident, $name:ident, $doc:literal) => {
        #[derive(Debug)]
        #[doc = $doc]
        pub struct $Name<L: ~const Eval, R: ~const Eval>(Expr<L>, Expr<R>);
        impl<
                L: ~const Eval,
                R: ~const Eval,
            > const core::ops::$Name<Expr<R>> for Expr<L>
        {
            type Output = Expr<$Name<Expr<L>, Expr<R>>>;
            #[inline(always)]
            fn $name(self, arg: Expr<R>) -> Self::Output {
                Expr($Name(self, arg))
            }
        }
        $crate::implement_eval!(
            L: Eval<Evaluated: ~const core::ops::$Name<R::Evaluated>>, R: Eval =>
            $Name<Expr<L>, Expr<R>> >-> <L::Evaluated as core::ops::$Name<R::Evaluated>>::Output:
            |self| where own {
                core::ops::$Name::$name(self.0.eval(), self.1.eval())
            } else {
                core::ops::$Name::$name((&self.0).eval(), (&self.1).eval())
            }
        );
    };
}

macro_rules! binary_grad {
    ($Name:ident, $Diff:ident) => {
        impl<
                L: ~const Eval<Evaluated: ~const core::ops::$Name<R::Evaluated>>
                    + ~const Grad<
                        Differentiated: ~const Eval<
                            Evaluated: ~const core::ops::$Diff<
                                <R::Differentiated as eval::Typed>::Evaluated,
                            >,
                        >,
                    >,
                R: ~const Grad,
            > const grad::Typed for $Name<L, R>
        {
            type Differentiated = $Diff<L::Differentiated, R::Differentiated>;
        }
        impl<
                L: ~const Eval<Evaluated: ~const core::ops::$Name<R::Evaluated>>
                    + ~const Grad<
                        Differentiated: ~const Eval<
                            Evaluated: ~const core::ops::$Diff<
                                <R::Differentiated as eval::Typed>::Evaluated,
                            >,
                        >,
                    >,
                R: ~const Grad,
            > const grad::Own for $Name<L, R>
        {
            #[inline(always)]
            fn grad<U>(self, x: &U) -> Self::Differentiated {
                $Diff(self.0.grad(x), self.1.grad(x))
            }
        }
        impl<
                L: ~const Eval<Evaluated: ~const core::ops::$Name<R::Evaluated>>
                    + ~const Grad<
                        Differentiated: ~const Eval<
                            Evaluated: ~const core::ops::$Diff<
                                <R::Differentiated as eval::Typed>::Evaluated,
                            >,
                        >,
                    >,
                R: ~const Grad,
            > const grad::Ref for $Name<L, R>
        {
            #[inline(always)]
            fn grad<U>(&self, x: &U) -> Self::Differentiated {
                $Diff((&self.0).grad(x), (&self.1).grad(x))
            }
        }
        impl<
                L: ~const Eval<Evaluated: ~const core::ops::$Name<R::Evaluated>>
                    + ~const Grad<
                        Differentiated: ~const Eval<
                            Evaluated: ~const core::ops::$Diff<
                                <R::Differentiated as eval::Typed>::Evaluated,
                            >,
                        >,
                    >,
                R: ~const Grad,
            > const Grad for $Name<L, R>
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

// TODO:
// binary_op!(Index, index, "Subscript indexing (e.g. `a[b]`)");
// binary_op!(RangeBounds, rangebounds, "Range bounds (e.g. `a..b`)");
