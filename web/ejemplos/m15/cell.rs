use std::cell::Cell;

struct Boton {
    texto: &'static str,
    pulsaciones: Cell<u32>, // 1. se puede cambiar aunque solo tengamos &Boton
}

// 2. Recibe un préstamo compartido (&), no exclusivo.
fn pulsar(b: &Boton) {
    b.pulsaciones.set(b.pulsaciones.get() + 1);
}

fn main() {
    let b = Boton { texto: "Aceptar", pulsaciones: Cell::new(0) };
    pulsar(&b);
    pulsar(&b);
    pulsar(&b);
    // 3. get() devuelve una copia del valor: Cell nunca presta lo de dentro.
    println!("«{}» pulsado {} veces", b.texto, b.pulsaciones.get());
    let viejo = b.pulsaciones.replace(100);
    println!("antes {viejo}, ahora {}", b.pulsaciones.get());
}
