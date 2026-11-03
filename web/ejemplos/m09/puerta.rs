struct PuertaCerrada;
struct PuertaAbierta;

impl PuertaCerrada {
    fn abrir(self) -> PuertaAbierta {
        println!("se abre");
        PuertaAbierta
    }
}

impl PuertaAbierta {
    fn cerrar(self) -> PuertaCerrada {
        println!("se cierra");
        PuertaCerrada
    }
}

fn main() {
    let p = PuertaCerrada;
    let p = p.abrir();
    let p = p.cerrar();
    let _p = p.abrir();
}
