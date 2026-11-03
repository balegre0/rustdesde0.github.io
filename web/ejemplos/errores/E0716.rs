fn main() {
    let palabras: Vec<&str> = String::from("uno dos").split(' ').collect();
    println!("{palabras:?}");
}
