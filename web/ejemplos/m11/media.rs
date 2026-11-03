fn media(datos: &[f64]) -> Option<f64> {
    if datos.is_empty() {
        return None;
    }
    let mut suma = 0.0;
    for d in datos {
        suma += d;
    }
    Some(suma / datos.len() as f64)
}

fn main() {
    let v = vec![2.0, 4.0, 9.0];
    println!("{:?}", media(&v));
    println!("{:?}", media(&v[..2]));
    println!("{:?}", media(&[]));
}
