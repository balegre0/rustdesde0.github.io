use std::marker::PhantomData;

struct Abierta;
struct Cerrada;

struct Puerta<Estado> {
    _estado: PhantomData<Estado>,
}

impl Puerta<Cerrada> {
    fn nueva() -> Puerta<Cerrada> {
        Puerta { _estado: PhantomData }
    }
    fn abrir(self) -> Puerta<Abierta> {
        Puerta { _estado: PhantomData }
    }
}

impl Puerta<Abierta> {
    fn pasar(&self) {}
}

fn main() {
    let p = Puerta::nueva();
    p.pasar(); // ¡pasar por una puerta cerrada!
    let _ = p.abrir();
}
