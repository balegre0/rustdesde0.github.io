use std::cell::Cell;

struct Contador {
    nombre: String,
    usos: Cell<u32>,
}

impl Contador {
    fn usar(&self) {
        // &self (no &mut self) y aun así cambia: mutabilidad interior
        self.usos.set(self.usos.get() + 1);
    }
}

fn main() {
    let c = Contador { nombre: String::from("botón"), usos: Cell::new(0) };
    c.usar();
    c.usar();
    println!("{} usado {} veces", c.nombre, c.usos.get());
    let x = Cell::new(5);
    let viejo = x.replace(9);
    println!("{viejo} {}", x.get());
    x.update(|v| v * 2);
    println!("{}", x.into_inner());
}
