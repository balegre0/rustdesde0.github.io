use std::rc::{Rc, Weak};

fn main() {
    let jefe = Rc::new(String::from("Marta"));
    // 1. Crea una referencia débil a jefe (Rc::downgrade).
    let recuerdo: Weak<String> = todo!();
    println!("¿vive? {:?}", recuerdo.upgrade());
    // 2. Suelta el único Rc fuerte.
    let _ = &jefe;
    println!("¿vive? {:?}", recuerdo.upgrade());
}
