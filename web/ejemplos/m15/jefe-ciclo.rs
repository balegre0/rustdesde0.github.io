use std::cell::RefCell;
use std::rc::Rc;

struct Carpeta {
    nombre: &'static str,
    madre: RefCell<Option<Rc<Carpeta>>>, // ¡Rc hacia arriba!
    hijas: RefCell<Vec<Rc<Carpeta>>>,
}

impl Drop for Carpeta {
    fn drop(&mut self) {
        println!("borrada {}", self.nombre);
    }
}

fn main() {
    let raiz = Rc::new(Carpeta { nombre: "inicio", madre: RefCell::new(None), hijas: RefCell::new(vec![]) });
    let fotos = Rc::new(Carpeta { nombre: "fotos", madre: RefCell::new(None), hijas: RefCell::new(vec![]) });
    raiz.hijas.borrow_mut().push(Rc::clone(&fotos));
    *fotos.madre.borrow_mut() = Some(Rc::clone(&raiz));
    let fotos_nombre = fotos.nombre;
    drop(fotos);
    drop(raiz);
    println!("fin ({fotos_nombre})");
}
