fn main() {
    let a = String::from("pan");
    {
        let b = String::from("queso");
        println!("dentro: {a} y {b}");
    }
    println!("fuera: {a} y {b}");
}
