fn main() {
    let mut valor = 10;
    let copia = valor + 1;
    let exclusivo = &mut valor;
    *exclusivo += 1;
    println!("{copia} {valor}");
}
