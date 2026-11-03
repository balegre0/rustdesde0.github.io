fn main() {
    let casos: [(&[i32], i32); 4] = [(&[1, 2, 3, 4], 6), (&[], 0), (&[-2, -3], -2), (&[7, 9], 0)];
    let mut fallos = 0;
    for (entrada, esperado) in casos {
        let r = suma_pares(entrada);
        if r == esperado {
            println!("OK: suma_pares({entrada:?}) = {r}");
        } else {
            println!("FALLO: suma_pares({entrada:?}) = {r}, se esperaba {esperado}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
