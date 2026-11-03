trait Guardar {
    // Un método NO genérico: cabe en la vtable.
    fn guardar_texto(&self, texto: &str);

    // El genérico se queda, pero solo para tipos con tamaño conocido:
    // `where Self: Sized` lo saca de la vtable.
    fn guardar<T: std::fmt::Debug>(&self, cosa: T)
    where
        Self: Sized,
    {
        self.guardar_texto(&format!("{cosa:?}"));
    }
}

struct Consola;

impl Guardar for Consola {
    fn guardar_texto(&self, texto: &str) {
        println!("guardado: {texto}");
    }
}

fn main() {
    let g: &dyn Guardar = &Consola;
    g.guardar_texto("5");
    Consola.guardar(vec![1, 2]);
}
