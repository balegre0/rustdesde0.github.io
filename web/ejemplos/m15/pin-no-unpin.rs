use std::marker::PhantomPinned;
use std::pin::Pin;

// Un tipo que declara: «moverme podría romperme».
struct Fijo {
    dato: u32,
    _fijo: PhantomPinned,
}

fn main() {
    let mut f = Fijo { dato: 1, _fijo: PhantomPinned };
    let p = Pin::new(&mut f); // ¿fijarlo en la pila con Pin::new?
    println!("{}", p.dato);
}
