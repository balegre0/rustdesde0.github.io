struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  drop: {}", self.0);
    }
}

fn main() {
    // 1. `caja` es dueña del Vec; el Vec es dueño de sus tres elementos
    let caja = vec![Ruidoso("uno"), Ruidoso("dos"), Ruidoso("tres")];
    println!("la caja tiene {} cosas", caja.len());
    println!("fin de main");
    // 2. Al liberar la raíz (`caja`), se libera todo lo que cuelga de ella
}
