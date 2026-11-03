use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Una carpeta conoce a sus hijas (Rc: es su dueña) y a su madre (Weak: no lo es).
struct Carpeta {
    nombre: String,
    bytes: u64,
    madre: RefCell<Weak<Carpeta>>,
    hijas: RefCell<Vec<Rc<Carpeta>>>,
}

impl Drop for Carpeta {
    fn drop(&mut self) {
        println!("borrada {}", self.nombre);
    }
}

fn nueva(nombre: &str, bytes: u64) -> Rc<Carpeta> {
    Rc::new(Carpeta {
        nombre: nombre.to_string(),
        bytes,
        madre: RefCell::new(Weak::new()),
        hijas: RefCell::new(Vec::new()),
    })
}

// 1. Engancha `hija` dentro de `madre` en los dos sentidos.
fn meter(madre: &Rc<Carpeta>, hija: &Rc<Carpeta>) {
    let _ = (madre, hija);
    todo!()
}

// 2. Tamaño total: los bytes propios más los de todas las hijas.
fn total(c: &Carpeta) -> u64 {
    let _ = c;
    todo!()
}

// 3. Ruta completa: subir por las madres con upgrade().
fn ruta(c: &Rc<Carpeta>) -> String {
    let mut partes = vec![c.nombre.clone()];
    let mut actual = c.madre.borrow().upgrade();
    while let Some(m) = actual {
        partes.push(m.nombre.clone());
        actual = m.madre.borrow().upgrade();
    }
    partes.reverse();
    partes.join("/")
}

fn main() {
    let raiz = nueva("inicio", 10);
    let fotos = nueva("fotos", 300);
    let viaje = nueva("viaje", 2000);
    meter(&raiz, &fotos);
    meter(&fotos, &viaje);
    println!("{}", ruta(&viaje));
    println!("total = {} bytes", total(&raiz));
    println!("fotos: fuertes = {}, débiles = {}", Rc::strong_count(&fotos), Rc::weak_count(&fotos));
    drop(viaje);
    drop(fotos);
    println!("soltando la raíz...");
    drop(raiz);
    println!("fin");
}
