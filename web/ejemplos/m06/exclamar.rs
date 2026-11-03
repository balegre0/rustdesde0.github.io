// Completa `exclamar`: recibe la propiedad de un String,
// le añade "!" al final y devuelve la propiedad.
fn exclamar(mut s: String) -> String {
    // Como somos dueños, podemos declarar el parámetro `mut` y modificarlo
    s.push('!');
    s
}

fn main() {
    let saludo = String::from("hola");
    let saludo = exclamar(saludo);
    let saludo = exclamar(saludo);
    println!("{saludo}");
}
