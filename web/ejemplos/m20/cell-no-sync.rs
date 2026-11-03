// Cell tiene mutabilidad interior sin sincronizar: no se puede compartir entre hilos.
use std::cell::Cell;

fn exige_sync<T: Sync>(_: &T) {}

fn main() {
    let c = Cell::new(1);
    exige_sync(&c);
}
