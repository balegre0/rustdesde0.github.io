#[derive(Clone, Copy)]
enum Semaforo {
    Rojo,
    Ambar,
    Verde,
}

fn main() {
    for s in [Semaforo::Rojo, Semaforo::Ambar, Semaforo::Verde] {
        let texto = match s {
            Semaforo::Rojo => "para",
            Semaforo::Ambar => "cuidado",
            Semaforo::Verde => "pasa",
        };
        println!("{texto}");
    }
}
