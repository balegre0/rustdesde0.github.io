#[derive(Debug, Clone, Copy)]
struct Punto { x: i32, y: i32 }

impl Punto {
    // 1. `Self` es el tipo del bloque impl: aquí, `Punto`.
    fn nuevo(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn reflejo(self) -> Self {
        Self { x: -self.x, ..self }
    }
}

trait Doble {
    // 2. En un trait, `Self` es «el tipo que implemente este trait».
    fn doble(&self) -> Self;
}

impl Doble for i32 {
    fn doble(&self) -> Self { self * 2 }
}

fn main() {
    let p = Punto::nuevo(3, 4).reflejo();
    println!("{p:?}");
    println!("x + y = {}", p.x + p.y);
    println!("{}", 21.doble());
}
