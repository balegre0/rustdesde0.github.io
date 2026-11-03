#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(i64),
    Var(String),
    Suma(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn plegar(e: Expr) -> Expr {
    todo!()
}

fn mostrar(e: &Expr) -> String {
    todo!()
}
