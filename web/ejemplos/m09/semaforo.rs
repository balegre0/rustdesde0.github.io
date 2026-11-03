#[derive(Debug)]
enum Semaforo {
    Verde,
    Ambar,
    Rojo,
}

impl Semaforo {
    fn siguiente(&self) -> Semaforo {
        match self {
            Semaforo::Verde => Semaforo::Ambar,
            Semaforo::Ambar => Semaforo::Rojo,
            Semaforo::Rojo => Semaforo::Verde,
        }
    }
}

fn main() {
    let mut s = Semaforo::Verde;
    for _ in 0..4 {
        println!("{:?}", s);
        s = s.siguiente();
    }
}
