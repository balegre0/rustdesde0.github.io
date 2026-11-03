use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Buzon = Rc<RefCell<Vec<String>>>;

struct Boletin {
    suscriptores: Vec<Weak<RefCell<Vec<String>>>>,
}

impl Boletin {
    fn suscribir(&mut self, buzon: &Buzon) {
        self.suscriptores.push(Rc::downgrade(buzon));
    }

    fn publicar(&mut self, mensaje: &str) -> usize {
        let mut entregados = 0;
        self.suscriptores.retain(|w| match w.upgrade() {
            Some(buzon) => {
                buzon.borrow_mut().push(mensaje.to_string());
                entregados += 1;
                true
            }
            None => false,
        });
        entregados
    }

    fn activos(&self) -> usize {
        self.suscriptores.iter().filter(|w| w.upgrade().is_some()).count()
    }
}
