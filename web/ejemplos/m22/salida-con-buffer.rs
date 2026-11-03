use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    // 1. stdout bloqueado y envuelto en un BufWriter: junta muchas escrituras en pocas
    let salida = io::stdout().lock();
    let mut w = BufWriter::new(salida);

    // 2. write!/writeln! funcionan con cualquier tipo que implemente Write
    for i in 1..=5 {
        writeln!(w, "línea {i}: {}", "*".repeat(i))?;
    }
    // 3. Un Vec<u8> también implementa Write: escribir en memoria
    let mut memoria: Vec<u8> = Vec::new();
    write!(memoria, "{}-{}", 4, 2)?;
    writeln!(w, "en memoria: {:?} = {:?}", memoria, String::from_utf8_lossy(&memoria))?;

    // 4. flush vacía el buffer; también ocurre al destruir el BufWriter
    w.flush()?;
    eprintln!("(esto va por stderr, sin buffer)");
    Ok(())
}
