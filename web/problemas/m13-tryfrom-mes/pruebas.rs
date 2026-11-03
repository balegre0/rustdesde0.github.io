fn main() {
    let mut fallos = 0;
    let casos: [(u8, Result<Mes, MesInvalido>); 6] = [
        (1, Ok(Mes::Enero)),
        (3, Ok(Mes::Marzo)),
        (12, Ok(Mes::Diciembre)),
        (0, Err(MesInvalido(0))),
        (13, Err(MesInvalido(13))),
        (255, Err(MesInvalido(255))),
    ];
    for (n, esperado) in casos {
        let obtenido: Result<Mes, MesInvalido> = n.try_into();
        if obtenido == esperado {
            println!("OK: {n} -> {:?}", obtenido);
        } else {
            println!("FALLO: {n}: esperaba {:?}, obtuve {:?}", esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
