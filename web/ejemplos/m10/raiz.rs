fn raiz(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        return Err(format!("no hay raíz real de {x}"));
    }
    Ok(x.sqrt())
}

fn main() {
    println!("{:?}", raiz(16.0));
    println!("{:?}", raiz(-4.0));
}
