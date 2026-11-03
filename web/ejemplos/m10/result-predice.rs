fn main() {
    let textos = ["42", "-7", "3.5", "hola", ""];
    for t in textos {
        match t.parse::<i32>() {
            Ok(n) => println!("{t:?} → número {n}"),
            Err(e) => println!("{t:?} → error: {e}"),
        }
    }
}
