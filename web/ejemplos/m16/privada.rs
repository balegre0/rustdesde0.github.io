// Un módulo hace de «crate»: lo que no es pub no se ve desde fuera,
// igual que un test de integración no ve lo privado de tu biblioteca.
mod notas {
    pub fn media(v: &[f64]) -> f64 {
        suma(v) / v.len() as f64
    }

    fn suma(v: &[f64]) -> f64 {
        v.iter().sum()
    }
}

fn main() {
    // Como haría un archivo de tests/: usar la biblioteca desde fuera.
    assert_eq!(notas::media(&[4.0, 6.0]), 5.0);
    assert_eq!(notas::suma(&[4.0, 6.0]), 10.0);
    println!("pruebas superadas");
}
