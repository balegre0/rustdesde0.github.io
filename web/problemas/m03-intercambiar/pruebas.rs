
fn main() {
    let casos = [
        ((7, 'a'), ('a', 7)),
        ((-3, 'ñ'), ('ñ', -3)),
        ((0, '🦀'), ('🦀', 0)),
        ((i32::MAX, 'Z'), ('Z', i32::MAX)),
        ((i32::MIN, ' '), (' ', i32::MIN)),
    ];
    let mut fallos = 0;
    for (entrada, esperado) in casos {
        let obtenido = intercambiar(entrada);
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", entrada, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", entrada, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
