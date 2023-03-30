// use super::*;

// // Holy Decree:
// // Tragically, Rust will have a cycle of dependencies with the product rule
// // unless we evaluate the coefficients before multiplying:
// // L's Grad implementation requires it to be multiplicable by R's, & vice versa
// // So, unfortunately, evaluate we must

// //%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// // Never mind, this happens all the time, even if we evaluate.
// // I think this is why we need Expr<...> around everything.

// // // Holy Fucking Christ
// //
// // impl<
// //         L: ~const Eval<
// //                 Evaluated: ~const core::ops::Mul<R::Evaluated>
// //                                + ~const core::ops::Mul<<R::Differentiated as eval::Typed>::Evaluated>,
// //             > + ~const Grad
// //             + ~const core::ops::Mul<R::Differentiated>,
// //         R: ~const Eval<
// //                 Evaluated: ~const core::ops::Mul<L::Evaluated>
// //                                + ~const core::ops::Mul<<L::Differentiated as eval::Typed>::Evaluated>,
// //             > + ~const Grad
// //             + ~const core::ops::Mul<L::Differentiated>,
// //     > const grad::Typed for Mul<L, R>
// // where
// //     <L::Evaluated as core::ops::Mul<<R::Differentiated as eval::Typed>::Evaluated>>::Output:
// //         ~const core::ops::Add<
// //             <R::Evaluated as core::ops::Mul<<L::Differentiated as eval::Typed>::Evaluated>>::Output,
// //         >,
// //     <L as core::ops::Mul<R::Differentiated>>::Output:
// //         ~const core::ops::Add<<R as core::ops::Mul<L::Differentiated>>::Output>,
// //     <<L as core::ops::Mul<R::Differentiated>>::Output as core::ops::Add<
// //         <R as core::ops::Mul<L::Differentiated>>::Output,
// //     >>::Output: ~const eval::Eval,
// // {
// //     // my sincerest apologies
// //     type Differentiated = <<L as core::ops::Mul<R::Differentiated>>::Output as core::ops::Add<
// //         <R as core::ops::Mul<L::Differentiated>>::Output,
// //     >>::Output;
// // }

// // impl<
// //         L: ~const Eval<
// //                 Evaluated: ~const core::ops::Mul<R::Evaluated>
// //                                + ~const core::ops::Mul<<R::Differentiated as eval::Typed>::Evaluated>,
// //             > + ~const Grad
// //             + ~const core::ops::Mul<R::Differentiated>,
// //         R: ~const Eval<
// //                 Evaluated: ~const core::ops::Mul<L::Evaluated>
// //                                + ~const core::ops::Mul<<L::Differentiated as eval::Typed>::Evaluated>,
// //             > + ~const Grad
// //             + ~const core::ops::Mul<L::Differentiated>,
// //     > const grad::Own for Mul<L, R>
// // where
// //     <L::Evaluated as core::ops::Mul<<R::Differentiated as eval::Typed>::Evaluated>>::Output:
// //         ~const core::ops::Add<
// //             <R::Evaluated as core::ops::Mul<<L::Differentiated as eval::Typed>::Evaluated>>::Output,
// //         >,
// //     <L as core::ops::Mul<R::Differentiated>>::Output:
// //         ~const core::ops::Add<<R as core::ops::Mul<L::Differentiated>>::Output>,
// //     <<L as core::ops::Mul<R::Differentiated>>::Output as core::ops::Add<
// //         <R as core::ops::Mul<L::Differentiated>>::Output,
// //     >>::Output: ~const eval::Eval,
// // {
// //     fn grad<U>(self, x: &U) -> Self::Differentiated {
// //         let g0 = (&self.0).grad(x);
// //         let g1 = (&self.1).grad(x);
// //         (self.0 * g1) + (self.1 * g0)
// //     }
// // }

// // impl<
// //         L: ~const Eval<
// //                 Evaluated: ~const core::ops::Mul<R::Evaluated>
// //                                + ~const core::ops::Mul<<R::Differentiated as eval::Typed>::Evaluated>,
// //             > + ~const Grad
// //             + ~const core::ops::Mul<R::Differentiated>,
// //         R: ~const Eval<
// //                 Evaluated: ~const core::ops::Mul<L::Evaluated>
// //                                + ~const core::ops::Mul<<L::Differentiated as eval::Typed>::Evaluated>,
// //             > + ~const Grad
// //             + ~const core::ops::Mul<L::Differentiated>,
// //     > const grad::Ref for Mul<L, R>
// // where
// //     <L::Evaluated as core::ops::Mul<<R::Differentiated as eval::Typed>::Evaluated>>::Output:
// //         ~const core::ops::Add<
// //             <R::Evaluated as core::ops::Mul<<L::Differentiated as eval::Typed>::Evaluated>>::Output,
// //         >,
// //     <L as core::ops::Mul<R::Differentiated>>::Output:
// //         ~const core::ops::Add<<R as core::ops::Mul<L::Differentiated>>::Output>,
// //     <<L as core::ops::Mul<R::Differentiated>>::Output as core::ops::Add<
// //         <R as core::ops::Mul<L::Differentiated>>::Output,
// //     >>::Output: ~const eval::Eval,
// //     for<'a> &'a L: ~const core::ops::Mul<R::Differentiated>,
// //     for<'a> &'a R: ~const core::ops::Mul<L::Differentiated>,
// //     for<'a> <&'a L as core::ops::Mul<R::Differentiated>>::Output:
// //         ~const core::ops::Add<<&'a R as core::ops::Mul<L::Differentiated>>::Output>,
// // {
// //     fn grad<U>(&self, x: &U) -> Self::Differentiated {
// //         let g0 = (&self.0).grad(x);
// //         let g1 = (&self.1).grad(x);
// //         ((&self.0) * g1) + ((&self.1) * g0)
// //     }
// // }

// impl<
//         L: ~const Grad<
//             Evaluated: ~const Eval<
//                 Evaluated: ~const core::ops::Mul<<R::Differentiated as eval::Typed>::Evaluated>,
//             > + ~const core::ops::Mul<R::Evaluated>,
//         >,
//         R: ~const Grad<Evaluated: ~const Eval>,
//     > const grad::Typed for Mul<L, R>
// {
//     type Differentiated = ();
// }

// impl<
//         L: ~const Grad<
//             Evaluated: ~const Eval<
//                 Evaluated: ~const core::ops::Mul<<R::Differentiated as eval::Typed>::Evaluated>,
//             > + ~const core::ops::Mul<R::Evaluated>,
//         >,
//         R: ~const Grad<
//             Evaluated: ~const Eval<
//                 Evaluated: ~const core::ops::Mul<<L::Differentiated as eval::Typed>::Evaluated>,
//             >,
//         >,
//     > const grad::Own for Mul<L, R>
// {
//     fn grad<U>(self, x: &U) -> <Self as grad::Typed>::Differentiated {
//         let g0 = (&self.0).grad(x);
//         let g1 = (&self.1).grad(x);
//         Add(Mul(self.0.eval(), g1), Mul(self.1.eval(), g0))
//     }
// }
