// Completa `siguiente`: Verde → Ambar, Ambar → Rojo, Rojo → Verde.

#[derive(Debug)]
enum Semaforo {
    Verde,
    Ambar,
    Rojo,
}

impl Semaforo {
    fn siguiente(&self) -> Semaforo {
        todo!()
    }
}

fn main() {
    let mut s = Semaforo::Verde;
    for _ in 0..4 {
        println!("{:?}", s);
        s = s.siguiente();
    }
}
