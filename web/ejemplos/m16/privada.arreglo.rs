mod notas {
    pub fn media(v: &[f64]) -> f64 {
        suma(v) / v.len() as f64
    }

    fn suma(v: &[f64]) -> f64 {
        v.iter().sum()
    }

    // Las pruebas de lo privado viven DENTRO: un módulo hijo ve todo lo de su padre.
    pub fn pruebas_internas() {
        assert_eq!(suma(&[4.0, 6.0]), 10.0);
    }
}

fn main() {
    assert_eq!(notas::media(&[4.0, 6.0]), 5.0);
    notas::pruebas_internas();
    println!("pruebas superadas");
}
