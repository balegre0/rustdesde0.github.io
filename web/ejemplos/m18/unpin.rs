use std::marker::PhantomPinned;
use std::pin::Pin;

fn es_unpin<T: Unpin>() -> bool { true }

struct Normal { _n: u32 }
struct Anclado { _n: u32, _no_mover: PhantomPinned }

fn main() {
    // 1. Casi todo es Unpin: moverlo aunque esté «fijado» no rompe nada
    println!("u32: {}", es_unpin::<u32>());
    println!("String: {}", es_unpin::<String>());
    println!("Normal: {}", es_unpin::<Normal>());

    // 2. Con un tipo Unpin, Pin no estorba: puedes sacar &mut sin unsafe
    let mut x = 5;
    let mut p = Pin::new(&mut x);
    *p.as_mut().get_mut() += 1;
    println!("x = {x}");

    // 3. Un tipo con PhantomPinned NO es Unpin (descomenta para ver el error)
    // es_unpin::<Anclado>();
    let a = Box::pin(Anclado { _n: 1, _no_mover: PhantomPinned });
    println!("Anclado fijado en el heap: {} bytes", std::mem::size_of_val(&*a));
}
