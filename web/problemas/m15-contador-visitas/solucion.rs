use std::cell::Cell;

struct Pagina {
    titulo: String,
    visitas: Cell<u32>,
}

impl Pagina {
    fn nueva(titulo: &str) -> Pagina {
        Pagina { titulo: titulo.to_string(), visitas: Cell::new(0) }
    }

    fn visitar(&self) -> u32 {
        let n = self.visitas.get() + 1;
        self.visitas.set(n);
        n
    }

    fn reiniciar(&self) -> u32 {
        self.visitas.replace(0)
    }
}
