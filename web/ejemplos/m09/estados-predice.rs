struct Borrador { texto: String }
struct Publicado { texto: String, visitas: u32 }

impl Borrador {
    fn new(texto: &str) -> Self {
        Borrador { texto: texto.to_string() }
    }
    fn publicar(self) -> Publicado {
        Publicado { texto: self.texto, visitas: 0 }
    }
}

impl Publicado {
    fn visitar(&mut self) {
        self.visitas += 1;
    }
}

fn main() {
    let b = Borrador::new("Hola, mundo");
    let mut p = b.publicar();
    p.visitar();
    p.visitar();
    println!("«{}» tiene {} visitas", p.texto, p.visitas);
}
