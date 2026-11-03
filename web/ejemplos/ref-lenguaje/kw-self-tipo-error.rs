fn main() {
    // `Self` solo existe dentro de impl, trait o la definición de un tipo.
    let x: Self = 5;
    println!("{x}");
}
