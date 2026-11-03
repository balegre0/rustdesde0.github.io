#![deny(unused_must_use)]

#[must_use]
fn calcular() -> u32 { 42 }

fn main() {
    // Con `deny`, ignorar un valor #[must_use] es un error.
    calcular();
}
