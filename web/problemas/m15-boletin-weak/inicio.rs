use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Buzon = Rc<RefCell<Vec<String>>>;

struct Boletin {
    suscriptores: Vec<Weak<RefCell<Vec<String>>>>,
}

impl Boletin {
    fn suscribir(&mut self, buzon: &Buzon) {
        let _ = buzon;
        todo!()
    }

    fn publicar(&mut self, mensaje: &str) -> usize {
        let _ = mensaje;
        todo!()
    }

    fn activos(&self) -> usize {
        todo!()
    }
}
