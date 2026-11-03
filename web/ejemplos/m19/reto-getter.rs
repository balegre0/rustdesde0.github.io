macro_rules! campo {
    ($nombre:ident, $tipo:ty, $valor:expr) => {
        fn $nombre() -> $tipo { $valor }
    };
}

fn main() {
    campo!(edad, u8, 30);
    campo!(ciudad, &'static str, "Cádiz");
    println!("{} {}", edad(), ciudad());
}
