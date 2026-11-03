use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorPedido {
    Cantidad(ParseIntError),
    Cero,
}

fn cantidad(texto: &str) -> Result<u32, ErrorPedido> {
    // Sin impl From, se convierte a mano con map_err (la variante es una función).
    let n: u32 = texto.parse().map_err(ErrorPedido::Cantidad)?;
    if n == 0 {
        return Err(ErrorPedido::Cero);
    }
    Ok(n)
}

fn main() {
    println!("{:?}", cantidad("3"));
    println!("{:?}", cantidad("0"));
    if let Err(ErrorPedido::Cantidad(e)) = cantidad("tres") {
        println!("no es un número: {e}");
    }
}
