// En vez de punteros que se apuntan entre sí, un Vec dueño de todo
// y números (índices) para decir quién es hijo de quién.
struct Nodo {
    nombre: &'static str,
    padre: Option<usize>,
    hijos: Vec<usize>,
}

struct Arbol {
    nodos: Vec<Nodo>,
}

impl Arbol {
    // 1. Añadir devuelve el índice del nodo nuevo.
    fn nuevo(&mut self, nombre: &'static str, padre: Option<usize>) -> usize {
        let id = self.nodos.len();
        self.nodos.push(Nodo { nombre, padre, hijos: Vec::new() });
        if let Some(p) = padre {
            self.nodos[p].hijos.push(id);
        }
        id
    }

    // 2. Subir hasta la raíz siguiendo los índices.
    fn ruta(&self, mut id: usize) -> String {
        let mut partes = vec![self.nodos[id].nombre];
        while let Some(p) = self.nodos[id].padre {
            partes.push(self.nodos[p].nombre);
            id = p;
        }
        partes.reverse();
        partes.join("/")
    }
}

fn main() {
    let mut a = Arbol { nodos: Vec::new() };
    let raiz = a.nuevo("casa", None);
    let cocina = a.nuevo("cocina", Some(raiz));
    let nevera = a.nuevo("nevera", Some(cocina));
    let _salon = a.nuevo("salón", Some(raiz));
    println!("{}", a.ruta(nevera));
    println!("la casa tiene {} hijos", a.nodos[raiz].hijos.len());
    // 3. Sin Rc, sin RefCell, sin ciclos: al soltar `a`, todo se libera.
}
