use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Nodo {
    nombre: &'static str,
    padre: RefCell<Weak<Nodo>>,    // 1. hacia arriba: Weak (no cuenta como dueño)
    hijos: RefCell<Vec<Rc<Nodo>>>, //    hacia abajo: Rc (dueño)
}

impl Drop for Nodo {
    fn drop(&mut self) {
        println!("liberado {}", self.nombre);
    }
}

fn nodo(nombre: &'static str) -> Rc<Nodo> {
    Rc::new(Nodo { nombre, padre: RefCell::new(Weak::new()), hijos: RefCell::new(vec![]) })
}

fn main() {
    let hoja = nodo("hoja");
    {
        let raiz = nodo("raíz");
        raiz.hijos.borrow_mut().push(Rc::clone(&hoja));
        // 2. Rc::downgrade crea un Weak: suma a weak_count, no a strong_count.
        *hoja.padre.borrow_mut() = Rc::downgrade(&raiz);
        println!("raíz: fuertes = {}, débiles = {}", Rc::strong_count(&raiz), Rc::weak_count(&raiz));
        println!("hoja: fuertes = {}, débiles = {}", Rc::strong_count(&hoja), Rc::weak_count(&hoja));
        // 3. upgrade() da Some(Rc) si el padre sigue vivo.
        match hoja.padre.borrow().upgrade() {
            Some(p) => println!("el padre de la hoja es {}", p.nombre),
            None => println!("la hoja no tiene padre"),
        }
    }
    // 4. La raíz ya se liberó: upgrade() da None.
    println!("¿padre tras el bloque? {}", hoja.padre.borrow().upgrade().is_some());
    println!("hoja: fuertes = {}", Rc::strong_count(&hoja));
    println!("fin de main");
}
