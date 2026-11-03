fn main() {
    let a = String::from("pan");
    // `b` se declara fuera del bloque: su ámbito llega hasta el último uso
    let b = String::from("queso");
    {
        println!("dentro: {a} y {b}");
    }
    println!("fuera: {a} y {b}");
}
