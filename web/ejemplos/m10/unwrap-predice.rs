fn main() {
    let a: i32 = "15".parse().unwrap();
    println!("a = {a}");
    let b: i32 = "quince".parse().expect("la configuración debe traer un número");
    println!("b = {b}");
}
