fn main() {
    let s = String::from("hola");
    let a = &s;
    let b = &s;
    let c = a;
    println!("{a} {b} {c} {}", s.len());
}
