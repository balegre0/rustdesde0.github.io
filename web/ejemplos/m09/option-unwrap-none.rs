fn main() {
    let v: Vec<i32> = Vec::new();
    let ultimo = v.last();
    println!("Vamos a sacar el último…");
    println!("{}", ultimo.unwrap());
}
