fn main() {
    let casos: [(&[u32], Vec<u32>); 4] = [
        (&[1, 2, 3, 4], vec![20, 40]),
        (&[], vec![]),
        (&[7, 9], vec![]),
        (&[0, 10, 11], vec![0, 100]),
    ];
    let mut fallos = 0;
    for (entrada, esperado) in casos {
        let obtenido = tuberia(entrada);
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
