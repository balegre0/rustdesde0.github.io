#[diagnostic::on_unimplemented(
    message = "`{Self}` no se puede guardar en el inventario",
    label = "falta implementar `Guardable` para este tipo",
    note = "añade `impl Guardable for {Self}`"
)]
trait Guardable {
    fn clave(&self) -> String;
}

struct Caja;

impl Guardable for Caja {
    fn clave(&self) -> String {
        String::from("caja-1")
    }
}

fn guardar<T: Guardable>(t: T) -> String {
    t.clave()
}

fn main() {
    println!("{}", guardar(Caja));
}
