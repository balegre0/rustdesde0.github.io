fn contar_letras<S: AsRef<str>>(texto: S) -> usize {
    texto.as_ref().chars().count()
}

fn duplicar_todo<T: AsMut<[i32]>>(mut datos: T) -> T {
    for x in datos.as_mut() {
        *x *= 2;
    }
    datos
}

fn main() {
    // la misma función acepta &str, String y &String
    let propio = String::from("ñandú");
    println!("{}", contar_letras("hola"));
    println!("{}", contar_letras(&propio));
    println!("{}", contar_letras(propio));
    println!("{:?}", duplicar_todo(vec![1, 2, 3]));
    println!("{:?}", duplicar_todo([4, 5]));
}
