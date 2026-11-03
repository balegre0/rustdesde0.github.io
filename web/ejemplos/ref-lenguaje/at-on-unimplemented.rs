#[diagnostic::on_unimplemented(
    message = "`{Self}` no se puede guardar en el inventario",
    label = "falta implementar `Guardable` para este tipo",
    note = "añade `impl Guardable for {Self}`"
)]
trait Guardable {
    fn clave(&self) -> String;
}

struct Caja;

fn guardar<T: Guardable>(t: T) -> String {
    t.clave()
}

fn main() {
    println!("{}", guardar(Caja));
}
