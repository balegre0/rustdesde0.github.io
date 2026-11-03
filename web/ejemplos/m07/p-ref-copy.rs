fn main() {
    let s = String::from("hola");
    let r1 = &s;
    let r2 = r1;
    println!("{r1} {r2} {s}");
}
