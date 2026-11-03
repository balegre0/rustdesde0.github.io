// 1. Imprime con size_of el tamaño de Option<u16>, con este formato exacto:
//    Option<u16>: ? bytes
// 2. Imprime si Option<&u16> mide lo mismo que &u16, con este formato exacto:
//    Option<&u16> mide lo mismo que &u16: true/false
//    (comparamos en vez de imprimir el número, porque una referencia mide 4 bytes
//    en el navegador y 8 en un PC de 64 bits, como el del Playground).

use std::mem::size_of;

fn main() {
    // escribe aquí
}
