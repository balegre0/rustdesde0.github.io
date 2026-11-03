trait Duplicar {
    // Devolver `Self` impide usar el trait como `dyn`:
    // detrás de un `dyn` no se sabe qué tipo (ni qué tamaño) es `Self`.
    fn duplicar(&self) -> Self;
}

fn usar(x: &dyn Duplicar) {
    let _ = x;
}

fn main() {}
