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
        todo!()
    }

    fn reiniciar(&self) -> u32 {
        todo!()
    }
}
