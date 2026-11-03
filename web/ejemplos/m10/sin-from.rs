use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorPedido {
    Cantidad(ParseIntError),
    Cero,
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
}
