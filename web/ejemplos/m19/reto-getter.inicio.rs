// Escribe la macro `campo!` para que `campo!(edad, u8, 30);`
// genere una función `fn edad() -> u8 { 30 }`.
macro_rules! campo {
    () => {};
}

fn main() {
    // campo!(edad, u8, 30);
    // campo!(ciudad, &'static str, "Cádiz");
    // println!("{} {}", edad(), ciudad());
}
