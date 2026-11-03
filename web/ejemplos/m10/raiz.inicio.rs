// Escribe `raiz(x: f64) -> Result<f64, String>`:
// si x es negativo, Err("no hay raíz real de X") (con el número); si no, Ok(x.sqrt()).

fn raiz(x: f64) -> Result<f64, String> {
    todo!()
}

fn main() {
    println!("{:?}", raiz(16.0));
    println!("{:?}", raiz(-4.0));
}
