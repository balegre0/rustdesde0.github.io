use std::rc::{Rc, Weak};

fn main() {
    let jefe = Rc::new(String::from("Marta"));
    // 1. Una referencia débil: no mantiene viva a Marta.
    let recuerdo: Weak<String> = Rc::downgrade(&jefe);
    println!("¿vive? {:?}", recuerdo.upgrade());
    // 2. Soltamos el único Rc fuerte.
    drop(jefe);
    println!("¿vive? {:?}", recuerdo.upgrade());
}
