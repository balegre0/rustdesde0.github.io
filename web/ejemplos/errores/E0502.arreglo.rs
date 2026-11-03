fn main() {
    let mut lista = vec![1, 2, 3];
    let primero = lista[0]; // copiamos el i32 en vez de tomarlo prestado
    lista.push(4);
    println!("{primero} {lista:?}");
}
