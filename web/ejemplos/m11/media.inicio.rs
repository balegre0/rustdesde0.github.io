// Escribe `media(datos: &[f64]) -> Option<f64>`: None si está vacío; si no, la media.
// Pruébala con un Vec, con un array y con un trozo.

fn media(datos: &[f64]) -> Option<f64> {
    todo!()
}

fn main() {
    let v = vec![2.0, 4.0, 9.0];
    println!("{:?}", media(&v));
    println!("{:?}", media(&v[..2]));
    println!("{:?}", media(&[]));
}
