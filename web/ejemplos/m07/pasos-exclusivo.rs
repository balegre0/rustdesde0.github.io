fn exclamar(texto: &mut String) {
    texto.push('!');
}

fn main() {
    let mut saludo = String::from("Hola");
    exclamar(&mut saludo);
    exclamar(&mut saludo);
    println!("{saludo}");
}
