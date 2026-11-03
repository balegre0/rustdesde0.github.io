fn guardar<'a>(lista: &mut Vec<&'a str>, texto: &'a str) {
    lista.push(texto);
}

fn main() {
    let mut lista = Vec::new();
    guardar(&mut lista, "hola");
    println!("{lista:?}");
}
