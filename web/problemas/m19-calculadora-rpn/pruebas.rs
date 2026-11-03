const CATORCE: i64 = rpn!(3 4 + 2 *);

fn main() {
    let mut ok = true;
    let mut caso = |nombre: &str, obtenido: i64, esperado: i64| {
        if obtenido == esperado { println!("OK: {}", nombre); }
        else { println!("FALLO: {} (esperado {}, obtenido {})", nombre, esperado, obtenido); ok = false; }
    };
    caso("un número", rpn!(5), 5);
    caso("3 4 +", rpn!(3 4 +), 7);
    caso("3 4 + 2 * (constante)", CATORCE, 14);
    caso("10 2 3 * -", rpn!(10 2 3 * -), 4);
    caso("orden de la resta", rpn!(2 10 -), -8);
    caso("larga", rpn!(1 2 + 3 4 + *), 21);
    if ok { println!("TODOS LOS CASOS SUPERADOS"); }
}
