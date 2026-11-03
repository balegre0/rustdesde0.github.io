fn main() {
    let s = String::from("hola");
    let primera = s.chars().next();
    let primer_byte = s.as_bytes()[0];
    println!("{:?} {}", primera, primer_byte);
}
