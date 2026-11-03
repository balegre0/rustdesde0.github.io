fn main() {
    let a = [String::from("x"), String::from("y")];
    // Desde la edición 2021, `array.into_iter()` da los valores (String), no referencias.
    for s in a.into_iter() {
        let propio: String = s;
        print!("{propio} ");
    }
    println!();
}
