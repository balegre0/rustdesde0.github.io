// Haz que `visitar` cuente visitas con solo un préstamo compartido (&self).
// Pista: Cell<u32> usa UnsafeCell por dentro y es totalmente segura.
use std::cell::Cell;

struct Pagina {
    nombre: String,
    visitas: Cell<u32>,
}

impl Pagina {
    fn visitar(&self) {
        self.visitas.set(self.visitas.get() + 1);
    }
}

fn main() {
    let p = Pagina { nombre: "inicio".into(), visitas: Cell::new(0) };
    let a = &p;
    let b = &p;
    a.visitar();
    b.visitar();
    a.visitar();
    println!("{}: {} visitas", p.nombre, p.visitas.get());
}
