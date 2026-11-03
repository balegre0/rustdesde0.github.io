use std::fmt;

struct Punto { x: i32, y: i32 }

// 1. Bloque `impl` inherente: métodos propios del tipo.
impl Punto {
    fn origen() -> Self { Punto { x: 0, y: 0 } }
    fn mover(&mut self, dx: i32) { self.x += dx; }
}

// 2. `impl Trait for Tipo`: implementar un trait.
impl fmt::Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 3. `impl Trait` en un argumento: «cualquier tipo que lo implemente».
fn imprimir(p: impl fmt::Display) {
    println!("{p}");
}

// 4. `impl Trait` en el retorno: un tipo concreto que no se nombra.
fn pares() -> impl Iterator<Item = u32> {
    (0..10).filter(|n| n % 2 == 0)
}

fn main() {
    let mut p = Punto::origen();
    p.mover(3);
    imprimir(p);
    println!("{:?}", pares().collect::<Vec<_>>());
}
