use std::io::{self, BufRead, BufReader, Read};

fn main() -> io::Result<()> {
    // &[u8] implementa Read: sirve para probar sin archivos
    let mut fuente: &[u8] = b"hola\nmundo\n";
    let mut todo = String::new();
    let n = fuente.read_to_string(&mut todo)?;
    println!("{n} bytes: {todo:?}");

    let mut trozo = [0u8; 3];
    let mut otra: &[u8] = b"abcdef";
    let leidos = otra.read(&mut trozo)?;
    println!("{leidos} {:?}", &trozo[..leidos]);

    // BufReader añade un búfer y los métodos de BufRead
    let lector = BufReader::new("a,b\nc,d\n".as_bytes());
    for linea in lector.lines() {
        let linea = linea?;
        println!("{:?}", linea.split(',').collect::<Vec<_>>());
    }
    Ok(())
}
