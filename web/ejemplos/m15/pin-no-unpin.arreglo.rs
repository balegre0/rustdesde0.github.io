use std::marker::PhantomPinned;
use std::pin::Pin;

// Un tipo que declara: «moverme podría romperme».
struct Fijo {
    dato: u32,
    _fijo: PhantomPinned,
}

fn main() {
    // Box::pin lo fija en el heap: desde aquí, nadie puede moverlo.
    let p: Pin<Box<Fijo>> = Box::pin(Fijo { dato: 1, _fijo: PhantomPinned });
    println!("dato = {}", p.dato);
}
