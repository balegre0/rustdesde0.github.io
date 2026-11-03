fn main() {
    let mut fallos = 0;
    let casos = ["00000101", "11111111", "10000000", "01111111", "11111011", "00000000", "10101010"];
    for bits in casos {
        let esperado = u8::from_str_radix(bits, 2).unwrap() as i8;
        let obtenido = con_signo(bits);
        if obtenido == esperado {
            println!("OK: {bits} -> {obtenido}");
        } else {
            println!("FALLO: {bits}: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
