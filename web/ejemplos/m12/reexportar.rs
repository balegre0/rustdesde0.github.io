mod motor {
    mod piezas {
        pub struct Bujia;
        pub fn encender(_b: &Bujia) -> &'static str {
            "brum"
        }
    }

    // `pub use` re-exporta: quien use `motor` ve estos nombres aquí,
    // aunque el código viva en `motor::piezas`.
    pub use piezas::{encender, Bujia};
}

fn main() {
    let b = motor::Bujia;
    println!("{}", motor::encender(&b));
}
