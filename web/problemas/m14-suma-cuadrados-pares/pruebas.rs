fn main() {
    let casos: [(&[i64], i64); 5] = [
        (&[1, 2, 3, 4], 20),
        (&[], 0),
        (&[-3, -2], 4),
        (&[1, 3, 5], 0),
        (&[10, 0, 7, 6], 136),
    ];
    let mut fallos = 0;
    for (entrada, esperado) in casos {
        let r = suma_cuadrados_pares(entrada);
        if r == esperado {
            println!("OK: suma_cuadrados_pares({entrada:?}) = {r}");
        } else {
            println!("FALLO: suma_cuadrados_pares({entrada:?}) = {r}, se esperaba {esperado}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
