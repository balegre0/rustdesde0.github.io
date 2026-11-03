fn main() {
    let a = String::from("pan");
    let b = String::from("queso");
    let c = a + " con " + &b;
    println!("{c} / {b}");
}
