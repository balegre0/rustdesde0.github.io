macro_rules! cinco_veces {
    ($e:expr) => {{
        // 1. Esta `x` es de la macro: no choca con ninguna `x` de fuera
        let x = 5;
        x * $e
    }};
}

fn main() {
    let x = 2;
    // 2. Aquí `x` es la de main (vale 2): la macro no la tapa
    let r = cinco_veces!(x + 1);
    println!("r = {}, x sigue valiendo {}", r, x);
}
