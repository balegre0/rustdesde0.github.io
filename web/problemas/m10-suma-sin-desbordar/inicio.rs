#[derive(Debug, PartialEq)]
enum ErrorSuma {
    NoEsNumero(usize),
    Desborda,
}

fn sumar_todo(textos: &[&str]) -> Result<u8, ErrorSuma> {
    todo!()
}
