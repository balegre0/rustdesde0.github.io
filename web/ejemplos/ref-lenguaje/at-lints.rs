#![warn(missing_debug_implementations)] // nivel de un aviso para todo el crate

#[derive(Debug)]
pub struct Punto {
    pub x: i32,
}

fn main() {
    #[allow(unused_variables)] // permitido solo en esta sentencia
    let sin_usar = 1;

    #[expect(unused_mut, reason = "se queda mut para el ejemplo")]
    let mut p = Punto { x: 3 };

    println!("{p:?}");
}
