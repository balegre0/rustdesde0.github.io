trait Almacen {
    async fn leer(&self) -> u32;
}

struct Memoria;
impl Almacen for Memoria {
    async fn leer(&self) -> u32 { 1 }
}

fn main() {
    // Cada impl devuelve un future de tipo y tamaño distintos: no cabe en una vtable
    let a: Box<dyn Almacen> = Box::new(Memoria);
    let _ = a;
}
