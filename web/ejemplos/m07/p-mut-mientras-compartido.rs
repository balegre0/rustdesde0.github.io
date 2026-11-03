fn main() {
    let mut s = String::from("hola");
    let vista = &s;
    s.push('!');
    println!("{vista}");
}
