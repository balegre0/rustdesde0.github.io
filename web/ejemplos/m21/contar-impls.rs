// El solver de traits demuestra `Option<Option<Option<()>>>: Contar`
// encadenando reglas, igual que una prueba lógica.
trait Contar {
    fn n() -> usize;
}

// Regla 1 (hecho): () cumple Contar.
impl Contar for () {
    fn n() -> usize { 0 }
}

// Regla 2: si T cumple Contar, entonces Option<T> también.
impl<T: Contar> Contar for Option<T> {
    fn n() -> usize { 1 + T::n() }
}

fn main() {
    println!("{}", <() as Contar>::n());
    println!("{}", <Option<()> as Contar>::n());
    println!("{}", <Option<Option<Option<()>>> as Contar>::n());
}
