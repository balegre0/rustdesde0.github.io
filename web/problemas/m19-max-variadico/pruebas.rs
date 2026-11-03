fn main() {
    let mut ok = true;
    let mut caso = |nombre: &str, obtenido: i64, esperado: i64| {
        if obtenido == esperado {
            println!("OK: {}", nombre);
        } else {
            println!("FALLO: {} (esperado {}, obtenido {})", nombre, esperado, obtenido);
            ok = false;
        }
    };
    caso("un valor", max_de!(7), 7);
    caso("tres valores", max_de!(3, 9, 4), 9);
    caso("negativos y coma final", max_de!(-5, -2, -8,), -2);
    caso("el máximo al final", max_de!(1, 2, 3, 4, 5, 100), 100);
    let mut llamadas = 0;
    let mut cuenta = |v: i64| { llamadas += 1; v };
    let m = max_de!(cuenta(10), cuenta(20), cuenta(15));
    caso("valor con efectos", m, 20);
    caso("cada argumento se evalúa una vez", llamadas, 3);
    if ok { println!("TODOS LOS CASOS SUPERADOS"); }
}
