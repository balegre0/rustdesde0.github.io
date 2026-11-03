#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(i64),
    Var(String),
    Suma(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn plegar(e: Expr) -> Expr {
    use Expr::*;
    match e {
        Suma(a, b) => match (plegar(*a), plegar(*b)) {
            (Num(x), Num(y)) => Num(x.wrapping_add(y)),
            (Num(0), otro) | (otro, Num(0)) => otro,
            (a, b) => Suma(Box::new(a), Box::new(b)),
        },
        Mul(a, b) => match (plegar(*a), plegar(*b)) {
            (Num(x), Num(y)) => Num(x.wrapping_mul(y)),
            (Num(0), _) | (_, Num(0)) => Num(0),
            (Num(1), otro) | (otro, Num(1)) => otro,
            (a, b) => Mul(Box::new(a), Box::new(b)),
        },
        hoja => hoja,
    }
}

fn mostrar(e: &Expr) -> String {
    match e {
        Expr::Num(n) => n.to_string(),
        Expr::Var(v) => v.clone(),
        Expr::Suma(a, b) => format!("({} + {})", mostrar(a), mostrar(b)),
        Expr::Mul(a, b) => format!("({} * {})", mostrar(a), mostrar(b)),
    }
}
