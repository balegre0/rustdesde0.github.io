use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Nodo {
    nombre: String,
    padre: RefCell<Weak<Nodo>>,
}

fn main() {
    let padre = Rc::new(Nodo { nombre: "raíz".into(), padre: RefCell::new(Weak::new()) });
    let hijo = Rc::new(Nodo { nombre: "hoja".into(), padre: RefCell::new(Weak::new()) });
    // el hijo apunta al padre con un Weak: no lo mantiene vivo
    *hijo.padre.borrow_mut() = Rc::downgrade(&padre);
    println!("fuertes {} débiles {}", Rc::strong_count(&padre), Rc::weak_count(&padre));
    // upgrade devuelve Some mientras el valor exista
    if let Some(p) = hijo.padre.borrow().upgrade() {
        println!("{} es hijo de {}", hijo.nombre, p.nombre);
    }
    drop(padre);
    println!("{:?}", hijo.padre.borrow().upgrade().map(|p| p.nombre.clone()));
}
