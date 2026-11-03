
fn prueba_num(n: i64) -> Box<Expr> {
    Box::new(Expr::Num(n))
}

fn main() {
    use Expr::*;
    let casos: Vec<(&str, Expr, i64, usize)> = vec![
        ("(2 + 3) * -4", Mul(Box::new(Suma(prueba_num(2), prueba_num(3))), Box::new(Neg(prueba_num(4)))), -20, 3),
        ("7", Num(7), 7, 0),
        ("-(-(1 + 1))", Neg(Box::new(Neg(Box::new(Suma(prueba_num(1), prueba_num(1)))))), 2, 3),
        ("10 * 10 + 1", Suma(Box::new(Mul(prueba_num(10), prueba_num(10))), prueba_num(1)), 101, 2),
        ("0 * -9", Mul(prueba_num(0), Box::new(Neg(prueba_num(9)))), 0, 2),
    ];
    let mut fallos = 0;
    for (texto, e, valor, ops) in &casos {
        let (r1, r2) = (evalua(e), cuenta_ops(e));
        if r1 == *valor && r2 == *ops {
            println!("OK: {texto} → evalua = {r1}, cuenta_ops = {r2}");
        } else {
            println!("FALLO: {texto} → evalua = {r1}, cuenta_ops = {r2}; se esperaba {valor} y {ops}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
