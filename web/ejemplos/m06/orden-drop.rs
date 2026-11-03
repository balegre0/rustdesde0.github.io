// Un tipo propio que avisa cuando lo liberan.
// (`struct`, `impl` y `Drop` los estudiarás en los mundos 09 y 13; aquí solo los usamos.)
struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  drop: {}", self.0);
    }
}

fn main() {
    // 1. Dos dueños en `main`
    let _a = Ruidoso("a");
    let _b = Ruidoso("b");
    {
        // 2. Uno más dentro de un bloque
        let _c = Ruidoso("c");
        println!("fin del bloque interior");
    }
    let _d = Ruidoso("d");
    println!("fin de main");
    // 3. Al salir de main se liberan d, b y a: en orden inverso al de creación
}
