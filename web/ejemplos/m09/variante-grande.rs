use std::mem::size_of;

#[allow(dead_code)]
enum Mensaje {
    Ping,
    Datos([u8; 1000]),
}

#[allow(dead_code)]
enum MensajeConCaja {
    Ping,
    Datos(Box<[u8; 1000]>),
}

fn main() {
    // 1. Cada Ping ocupa lo mismo que el caso más grande.
    println!("Mensaje:        {} bytes", size_of::<Mensaje>());
    // 2. Con Box, la carga va al heap y el enum guarda solo un puntero.
    println!("MensajeConCaja: {} bytes", size_of::<MensajeConCaja>());
    let cola: Vec<MensajeConCaja> = (0..100).map(|_| MensajeConCaja::Ping).collect();
    println!("100 pings ocupan {} bytes en el Vec", cola.len() * size_of::<MensajeConCaja>());
}
