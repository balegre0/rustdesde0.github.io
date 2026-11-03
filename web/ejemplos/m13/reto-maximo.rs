use std::fmt::Display;

fn mostrar_lista<T: Display>(lista: &[T]) -> String {
    let mut texto = String::new();
    for (i, x) in lista.iter().enumerate() {
        if i > 0 {
            texto.push_str(" | ");
        }
        texto.push_str(&x.to_string());
    }
    texto
}

fn main() {
    println!("{}", mostrar_lista(&[1, 2, 3]));
    println!("{}", mostrar_lista(&["sol", "luna"]));
    println!("{}", mostrar_lista(&[2.5]));
}
