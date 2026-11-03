fn main() {
    use Expr::*;
    let n = |x: i64| Box::new(Num(x));
    let v = |s: &str| Box::new(Var(s.to_string()));
    let casos: [(Expr, &str); 6] = [
        (Suma(n(2), n(3)), "5"),
        (Suma(v("x"), Box::new(Mul(n(2), n(3)))), "(x + 6)"),
        (Mul(v("y"), Box::new(Suma(n(1), n(0)))), "y"),
        (Mul(Box::new(Suma(v("a"), v("b"))), Box::new(Suma(n(-2), n(2)))), "0"),
        (Suma(n(0), Box::new(Mul(n(1), v("z")))), "z"),
        (Suma(v("x"), Box::new(Mul(n(2), v("y")))), "(x + (2 * y))"),
    ];
    let mut fallos = 0;
    for (e, esperado) in casos {
        let antes = mostrar(&e);
        let obtenido = mostrar(&plegar(e));
        if obtenido == esperado {
            println!("OK: {antes} -> {obtenido}");
        } else {
            println!("FALLO: {antes}: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
