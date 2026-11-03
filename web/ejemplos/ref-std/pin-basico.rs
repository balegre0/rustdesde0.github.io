use std::marker::PhantomPinned;
use std::pin::{pin, Pin};

struct Fijo {
    dato: u32,
    _no_mover: PhantomPinned, // hace que el tipo no sea Unpin
}

fn leer(p: Pin<&mut Fijo>) -> u32 {
    p.dato
}

fn main() {
    // pin! fija un valor en la pila: ya no se podrá mover
    let mut f = pin!(Fijo { dato: 7, _no_mover: PhantomPinned });
    println!("{}", leer(f.as_mut()));
    println!("{}", leer(f.as_mut()));
    // Box::pin lo fija en el montón
    let b: Pin<Box<Fijo>> = Box::pin(Fijo { dato: 9, _no_mover: PhantomPinned });
    println!("{}", b.dato);
    // con tipos Unpin (casi todos), Pin no restringe nada
    let mut n = 5;
    let mut p = Pin::new(&mut n);
    *p += 1;
    println!("{}", *p);
}
