enum Semaforo {
    Rojo,
    Ambar,
    Verde,
}

fn main() {
    let s = Semaforo::Ambar;
    // `match` debe cubrir todas las variantes.
    let accion = match s {
        Semaforo::Rojo => "para",
        Semaforo::Verde => "pasa",
    };
    println!("{accion}");
}
