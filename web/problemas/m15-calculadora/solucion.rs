enum Expr {
    Num(i64),
    Suma(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn evalua(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Suma(a, b) => evalua(a) + evalua(b),
        Expr::Mul(a, b) => evalua(a) * evalua(b),
        Expr::Neg(a) => -evalua(a),
    }
}

fn cuenta_ops(e: &Expr) -> usize {
    match e {
        Expr::Num(_) => 0,
        Expr::Suma(a, b) | Expr::Mul(a, b) => 1 + cuenta_ops(a) + cuenta_ops(b),
        Expr::Neg(a) => 1 + cuenta_ops(a),
    }
}
