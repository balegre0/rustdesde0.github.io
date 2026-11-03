fn main() {
    // 1. Con mut se pueden cambiar sus partes
    let mut punto = (0, 0);
    punto.0 = 5;
    punto.1 += 2;
    // 2. _ ignora una parte al desestructurar
    let (x, _) = punto;
    println!("{:?} x={x}", punto);
    // 3. Una tupla de un elemento lleva coma final
    let uno = (7,);
    println!("{:?}", uno);
}
