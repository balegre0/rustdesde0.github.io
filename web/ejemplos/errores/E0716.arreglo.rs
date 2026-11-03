fn main() {
    let texto = String::from("uno dos"); // el String vive en una variable
    let palabras: Vec<&str> = texto.split(' ').collect();
    println!("{palabras:?}");
}
