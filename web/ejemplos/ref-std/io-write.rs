use std::io::{self, Write};

fn main() -> io::Result<()> {
    // stdout implementa Write
    let mut salida = io::stdout().lock();
    writeln!(salida, "línea con writeln!")?;
    salida.write_all(b"bytes sueltos\n")?;
    salida.flush()?;
    // un Vec<u8> también implementa Write: escribir en memoria
    let mut buf: Vec<u8> = Vec::new();
    write!(buf, "{}+{}={}", 2, 2, 4)?;
    println!("{:?} -> {}", buf, String::from_utf8_lossy(&buf));
    // stderr es otra salida (para errores)
    eprintln!("esto va a la salida de errores");
    Ok(())
}
