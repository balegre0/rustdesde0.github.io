fn main() {
    let mut valor = 10;
    let exclusivo = &mut valor;
    let copia = valor + 1;
    *exclusivo += 1;
    println!("{copia}");
}
