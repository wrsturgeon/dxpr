use crate::*;

#[cfg(feature = "std")]
#[test]
fn dbg_var() {
    dbg!(var(&4));
}

#[cfg(feature = "std")]
#[test]
fn dbg_expr() {
    dbg!(Expr(var(&4)));
}

#[cfg(feature = "std")]
#[test]
fn dbg_neg() {
    dbg!(-var(&4));
}

#[cfg(feature = "std")]
#[test]
fn dbg_not() {
    dbg!(!var(&false));
}

#[test]
fn eval_var() {
    assert_eq!(4, *var(&4).eval())
}

#[test]
fn const_eval_var() {
    const VALUE: i32 = *var(&4).eval();
    assert_eq!(4, VALUE);
}

#[test]
fn eval_expr() {
    assert_eq!(4, *Expr(var(&4)).eval())
}

#[test]
fn const_eval_expr() {
    const VALUE: i32 = *Expr(var(&4)).eval();
    assert_eq!(4, VALUE);
}

#[test]
fn neg_var() {
    assert_eq!(-4, (-var(&4)).eval());
}

#[test]
fn const_neg_var() {
    const VALUE: i32 = (-var(&4)).eval();
    assert_eq!(-4, VALUE);
}

#[test]
fn neg_expr() {
    assert_eq!(4, (--var(&4)).eval());
}

#[test]
fn const_neg_expr() {
    const VALUE: i32 = (--var(&4)).eval();
    assert_eq!(4, VALUE);
}

#[test]
fn not_var() {
    assert_eq!(false, (!var(&true)).eval());
}

#[test]
fn not_expr() {
    assert_eq!(true, (!!var(&true)).eval());
}

#[test]
fn eval_twice() {
    let expr: Expr<ops::Neg<Expr<ops::Neg<Expr<&i32>>>>> = --var(&4);
    (&expr).eval();
    expr.eval();
}

#[test]
fn add_var() {
    assert_eq!(3, (var(&1) + var(&2)).eval());
}

#[test]
fn add_expr() {
    assert_eq!(15, ((var(&1) + var(&2)) + (var(&4) + var(&8))).eval());
}

#[test]
fn grad_tautological_self_runtime() {
    let x = 1;
    let g = var(&x).grad(&x);
    let v: &i32 = g.eval();
    assert_eq!(1, *v);
}

#[test]
fn grad_tautological_other_runtime() {
    let x = 1;
    let y = 1; // same value!
    let g = var(&x).grad(&y);
    let v: &i32 = g.eval();
    assert_eq!(0, *v);
}
