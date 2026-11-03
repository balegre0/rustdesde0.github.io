enum Expr {
    Num(i64),
    Suma(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn evalua(e: &Expr) -> i64 {
    let _ = e;
    todo!()
}

fn cuenta_ops(e: &Expr) -> usize {
    let _ = e;
    todo!()
}
