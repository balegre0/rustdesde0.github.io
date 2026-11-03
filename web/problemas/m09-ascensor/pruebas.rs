
fn main() {
    use Estado::*;
    use Evento::*;
    let casos = [
        (Parado { planta: 0 }, Llamar(5), Subiendo { desde: 0, hasta: 5 }),
        (Parado { planta: 7 }, Llamar(2), Bajando { desde: 7, hasta: 2 }),
        (Parado { planta: 3 }, Llamar(3), Parado { planta: 3 }),
        (Parado { planta: 3 }, Llamar(11), Parado { planta: 3 }),
        (Parado { planta: 3 }, Llegar, Parado { planta: 3 }),
        (Parado { planta: 4 }, Emergencia, Parado { planta: 4 }),
        (Subiendo { desde: 0, hasta: 5 }, Llegar, Parado { planta: 5 }),
        (Bajando { desde: 7, hasta: 2 }, Llegar, Parado { planta: 2 }),
        (Subiendo { desde: 1, hasta: 9 }, Llamar(4), Subiendo { desde: 1, hasta: 9 }),
        (Subiendo { desde: 1, hasta: 9 }, Emergencia, Parado { planta: 1 }),
        (Bajando { desde: 8, hasta: 0 }, Emergencia, Parado { planta: 8 }),
        (Parado { planta: 10 }, Llamar(10), Parado { planta: 10 }),
    ];
    let mut fallos = 0;
    for (estado, evento, esperado) in casos {
        let obtenido = transicion(estado, evento);
        if obtenido == esperado {
            println!("OK: {:?} + {:?} -> {:?}", estado, evento, obtenido);
        } else {
            println!("FALLO: {:?} + {:?}: esperaba {:?}, obtuve {:?}", estado, evento, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
