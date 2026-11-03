use std::cell::RefCell;
use std::rc::Rc;

struct Nodo {
    nombre: &'static str,
    otro: RefCell<Option<Rc<Nodo>>>,
}

impl Drop for Nodo {
    fn drop(&mut self) {
        println!("liberado {}", self.nombre);
    }
}

fn main() {
    let a = Rc::new(Nodo { nombre: "a", otro: RefCell::new(None) });
    let b = Rc::new(Nodo { nombre: "b", otro: RefCell::new(None) });
    // 1. a apunta a b y b apunta a a: un ciclo.
    *a.otro.borrow_mut() = Some(Rc::clone(&b));
    *b.otro.borrow_mut() = Some(Rc::clone(&a));
    println!("cuenta de a = {}, de b = {}", Rc::strong_count(&a), Rc::strong_count(&b));
    println!("fin de main");
    // 2. Al salir, cada cuenta baja de 2 a 1, nunca a 0: nadie se libera.
}
