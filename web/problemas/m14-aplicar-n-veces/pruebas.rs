fn incrementar(x: i64) -> i64 {
    x + 1
}

fn main() {
    let mut fallos = 0;
    let mut comprobar = |nombre: &str, r: i64, esperado: i64| {
        if r == esperado {
            println!("OK: {nombre} = {r}");
        } else {
            println!("FALLO: {nombre} = {r}, se esperaba {esperado}");
            fallos += 1;
        }
    };
    comprobar("aplicar_n(|x| x * 2, 3, 1)", aplicar_n(|x| x * 2, 3, 1), 8);
    comprobar("aplicar_n(|x| x + 10, 0, 5)", aplicar_n(|x| x + 10, 0, 5), 5);
    comprobar("aplicar_n(|x| x - 1, 4, 0)", aplicar_n(|x| x - 1, 4, 0), -4);
    comprobar("aplicar_n(incrementar, 100, 0)", aplicar_n(incrementar, 100, 0), 100);
    let base = 3;
    comprobar("aplicar_n(|x| x * base, 4, 1) con base = 3", aplicar_n(|x| x * base, 4, 1), 81);
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
