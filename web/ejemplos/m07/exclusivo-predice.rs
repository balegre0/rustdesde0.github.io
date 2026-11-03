fn agregar(s: &mut String, extra: &str) {
    s.push_str(extra);
}

fn main() {
    let mut frase = String::from("hola");
    agregar(&mut frase, ", ");
    agregar(&mut frase, "mundo");
    println!("{frase} ({} bytes)", frase.len());
}
