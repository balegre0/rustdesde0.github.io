fn agregar(mut s: String) -> String {
    s.push('!');
    s
}

fn main() {
    let s = String::from("hola");
    let s = agregar(s);
    let s = agregar(s);
    println!("{s}");
}
