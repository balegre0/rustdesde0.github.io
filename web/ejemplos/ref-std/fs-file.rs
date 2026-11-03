use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // File::create: crea o vacía el archivo, para escribir
    let mut f = File::create("registro.txt")?;
    writeln!(f, "inicio")?;
    drop(f); // se cierra al soltarlo

    // OpenOptions: abrir para añadir al final
    let mut f = OpenOptions::new().append(true).open("registro.txt")?;
    writeln!(f, "segunda entrada")?;
    drop(f);

    // File::open: abrir para leer, línea a línea con BufReader
    let f = File::open("registro.txt")?;
    for (i, linea) in BufReader::new(f).lines().enumerate() {
        println!("{}: {}", i + 1, linea?);
    }
    let error = File::open("falta.txt").unwrap_err();
    println!("{error}");
    Ok(())
}
