struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  drop: {}", self.0);
    }
}

fn main() {
    let grande = Ruidoso("grande");
    let pequeno = Ruidoso("pequeño");
    // 1. `drop(x)` es una función corriente: se queda con `x` y no lo devuelve
    drop(grande);
    println!("ya no necesito `grande`; sigo trabajando con {}", pequeno.0);
    println!("fin de main");
    // 2. `pequeno` se libera al final, como siempre
}
