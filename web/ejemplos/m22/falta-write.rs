use std::io;

fn main() {
    let mut salida = io::stdout().lock();
    // writeln! llama al método write_fmt... que viene del trait Write, y no lo hemos importado
    writeln!(salida, "hola").unwrap();
}
