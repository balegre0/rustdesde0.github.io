// 1. Lo mismo que la macro `cuadrado!`, pero como función genérica
fn cuadrado<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

fn siguiente(n: &mut i32) -> i32 {
    *n += 1;
    *n
}

fn main() {
    let mut n = 0;
    // 2. El argumento se evalúa UNA vez, antes de la llamada
    let r = cuadrado(siguiente(&mut n));
    println!("r = {}, n = {}", r, n);
    // 3. Y funciona con varios tipos numéricos gracias al genérico
    println!("{} {}", cuadrado(1.5), cuadrado(7u8));
}
