#[derive(Debug, PartialEq)]
enum ErrorSuma {
    NoEsNumero(usize),
    Desborda,
}

fn sumar_todo(textos: &[&str]) -> Result<u8, ErrorSuma> {
    let mut total: u8 = 0;
    for (i, t) in textos.iter().enumerate() {
        let n: u8 = t.trim().parse().map_err(|_| ErrorSuma::NoEsNumero(i))?;
        total = total.checked_add(n).ok_or(ErrorSuma::Desborda)?;
    }
    Ok(total)
}
