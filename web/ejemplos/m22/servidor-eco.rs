use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

// Atiende a un cliente: devuelve cada línea que recibe, en mayúsculas.
fn atender(flujo: TcpStream) -> std::io::Result<()> {
    let quien = flujo.peer_addr()?;
    println!("conexión de {quien}");
    let mut escritor = flujo.try_clone()?;
    for linea in BufReader::new(flujo).lines() {
        let linea = linea?;
        writeln!(escritor, "{}", linea.to_uppercase())?;
    }
    println!("{quien} se fue");
    Ok(())
}

fn main() -> std::io::Result<()> {
    // 1. Reservar un puerto en todas las interfaces locales
    let oyente = TcpListener::bind("127.0.0.1:7878")?;
    println!("escuchando en {}", oyente.local_addr()?);
    // 2. Aceptar conexiones una a una (para varias a la vez: un hilo por cliente, o async)
    for flujo in oyente.incoming() {
        if let Err(e) = atender(flujo?) {
            eprintln!("error con un cliente: {e}");
        }
    }
    Ok(())
}
