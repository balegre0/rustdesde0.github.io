fn agregar(s: &String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("hola");
    agregar(&s);
    println!("{s}");
}
