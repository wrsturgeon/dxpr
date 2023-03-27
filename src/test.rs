use crate::*;

#[cfg(feature = "std")]
#[test]
fn dbg_var() {
    dbg!(Var(4));
}

#[cfg(feature = "std")]
#[test]
fn dbg_expr() {
    dbg!(Expr(Var(4)));
}

#[cfg(feature = "std")]
#[test]
fn dbg_neg() {
    dbg!(-Var(4));
}

#[cfg(feature = "std")]
#[test]
fn dbg_not() {
    dbg!(!Var(false));
}

#[test]
fn eval_var() {
    assert_eq!(4, Var(4).eval())
}

#[test]
fn const_eval_var() {
    const VALUE: i32 = Var(4).eval();
    assert_eq!(4, VALUE);
}

#[test]
fn eval_expr() {
    assert_eq!(4, Expr(Var(4)).eval())
}

#[test]
fn const_eval_expr() {
    const VALUE: i32 = Expr(Var(4)).eval();
    assert_eq!(4, VALUE);
}

#[test]
fn neg_var() {
    assert_eq!(-4, (-Var(4)).eval());
}

#[test]
fn const_neg_var() {
    const VALUE: i32 = (-Var(4)).eval();
    assert_eq!(-4, VALUE);
}

#[test]
fn neg_expr() {
    assert_eq!(4, (--Var(4)).eval());
}

#[test]
fn const_neg_expr() {
    const VALUE: i32 = (--Var(4)).eval();
    assert_eq!(4, VALUE);
}

#[test]
fn not_var() {
    assert_eq!(false, (!Var(true)).eval());
}

#[test]
fn not_expr() {
    assert_eq!(true, (!!Var(true)).eval());
}
