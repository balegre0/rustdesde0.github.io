fn main() {
    let s = String::from("hola");
    let r = &s;
    let t = s;
    println!("{r} {t}");
}
