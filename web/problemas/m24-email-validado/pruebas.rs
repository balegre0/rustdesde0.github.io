fn main() {
    use correo::Email;
    let mut fallos = 0;
    let validos = [("  Ana@Ejemplo.ES ", "ana", "ejemplo.es"), ("x.y@a.b.c", "x.y", "a.b.c")];
    for (s, u, d) in validos {
        match Email::parsear(s) {
            Ok(e) if e.usuario() == u && e.dominio() == d => println!("OK: {s:?} -> {}@{}", e.usuario(), e.dominio()),
            Ok(e) => {
                println!("FALLO: {s:?}: esperaba {u}@{d}, obtuve {}@{}", e.usuario(), e.dominio());
                fallos += 1;
            }
            Err(m) => {
                println!("FALLO: {s:?} debía ser válido, dio Err({m:?})");
                fallos += 1;
            }
        }
    }
    let invalidos = [
        ("   ", "vacío"),
        ("sin-arroba.es", "hace falta una @"),
        ("a@@b.es", "hace falta una @"),
        ("@ejemplo.es", "falta el usuario"),
        ("ana@ejemplo", "dominio no válido"),
        ("ana@.es", "dominio no válido"),
        ("ana@ejemplo.", "dominio no válido"),
    ];
    for (s, esperado) in invalidos {
        match Email::parsear(s) {
            Err(m) if m == esperado => println!("OK: {s:?} -> Err({m:?})"),
            Err(m) => {
                println!("FALLO: {s:?}: esperaba Err({esperado:?}), obtuve Err({m:?})");
                fallos += 1;
            }
            Ok(_) => {
                println!("FALLO: {s:?} debía ser inválido");
                fallos += 1;
            }
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
