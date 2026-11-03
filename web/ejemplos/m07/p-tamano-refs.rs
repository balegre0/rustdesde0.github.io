fn main() {
    let s = String::from("hola");
    let r: &String = &s;
    let v: &str = &s;
    println!("{}", size_of_val(&v) == 2 * size_of_val(&r));
    println!("{} {}", r.len(), v.len());
}
