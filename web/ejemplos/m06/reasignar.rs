struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  drop: {}", self.0);
    }
}

fn main() {
    // 1. Una variable `mut` puede recibir un valor nuevo
    let mut r = Ruidoso("viejo");
    println!("antes de reasignar: {}", r.0);
    // 2. Al reasignar, el valor viejo pierde a su dueño: se libera en ese momento
    r = Ruidoso("nuevo");
    println!("después de reasignar: {}", r.0);
    // 3. Mover y volver a llenar: tras moverla, la variable vuelve a ser útil
    let otra = r;
    r = Ruidoso("tercero");
    println!("otra = {}, r = {}", otra.0, r.0);
    println!("fin de main");
}
