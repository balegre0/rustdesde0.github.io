const TRES: usize = cuenta!(a, b + 1, f(x));

fn main() {
    let mut ok = true;
    let mut caso = |nombre: &str, obtenido: usize, esperado: usize| {
        if obtenido == esperado { println!("OK: {}", nombre); }
        else { println!("FALLO: {} (esperado {}, obtenido {})", nombre, esperado, obtenido); ok = false; }
    };
    caso("vacío", cuenta!(), 0);
    caso("uno", cuenta!(42), 1);
    caso("constante con nombres que no existen", TRES, 3);
    caso("coma final", cuenta!(1, 2, 3, 4,), 4);
    let arr = [0u8; cuenta!(1, 2)];
    caso("tamaño de array", arr.len(), 2);
    let mut efectos = 0;
    let _ = cuenta!({ efectos += 1; 1 }, { efectos += 1; 2 });
    caso("no evalúa las expresiones", efectos, 0);
    if ok { println!("TODOS LOS CASOS SUPERADOS"); }
}
