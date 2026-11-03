trait Guardar {
    fn guardar<T: std::fmt::Debug>(&self, cosa: T);
}

struct Consola;

impl Guardar for Consola {
    fn guardar<T: std::fmt::Debug>(&self, cosa: T) {
        println!("guardado: {cosa:?}");
    }
}

fn main() {
    let g: &dyn Guardar = &Consola;
    g.guardar(5);
}
