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

#[test]
fn eval_var() {
    assert_eq!(4, Var(4).eval())
}

#[test]
fn eval_expr() {
    assert_eq!(4, Expr(Var(4)).eval())
}

#[test]
fn neg_var() {
    assert_eq!((-Var(4)).eval(), -4);
}

#[test]
fn neg_expr() {
    assert_eq!((--Var(4)).eval(), 4);
}

#[test]
fn not_var() {
    assert_eq!((!Var(true)).eval(), false);
}

#[test]
fn not_expr() {
    assert_eq!((!!Var(true)).eval(), true);
}
