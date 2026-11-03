use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorPedido {
    Cantidad(ParseIntError),
    Cero,
}

impl From<ParseIntError> for ErrorPedido {
    fn from(e: ParseIntError) -> Self {
        ErrorPedido::Cantidad(e)
    }
}

fn cantidad(texto: &str) -> Result<u32, ErrorPedido> {
    let n = texto.parse::<u32>()?;
    if n == 0 {
        return Err(ErrorPedido::Cero);
    }
    Ok(n)
}

fn main() {
    println!("{:?}", cantidad("3"));
    println!("{:?}", cantidad("0"));
    println!("{:?}", cantidad("tres"));
}
