// Reto: la terminal de texto de tu kernel, con salto de línea, ajuste y desplazamiento.
const FILAS: usize = 3;
const COLUMNAS: usize = 8;

struct Terminal {
    celdas: [[u8; COLUMNAS]; FILAS],
    fila: usize,
    columna: usize,
}

impl Terminal {
    fn nueva() -> Terminal {
        Terminal { celdas: [[b' '; COLUMNAS]; FILAS], fila: 0, columna: 0 }
    }

    /// Escribe un byte: '\n' salta de línea; si la línea está llena, salta antes de escribir.
    fn escribir_byte(&mut self, b: u8) {
        todo!()
    }

    /// Pasa a la línea siguiente. Si ya estás en la última fila, todo sube una fila
    /// (la de arriba se pierde) y la última queda en blanco.
    fn salto(&mut self) {
        todo!()
    }

    fn escribir(&mut self, s: &str) {
        for b in s.bytes() {
            self.escribir_byte(b);
        }
    }

    fn pintar(&self) {
        for fila in &self.celdas {
            println!("[{}]", String::from_utf8_lossy(fila));
        }
    }
}

fn main() {
    let mut t = Terminal::nueva();
    t.escribir("kernel\nv0.1");
    t.pintar();
    println!("--");
    t.escribir("\nmemoria ok\nlisto");
    t.pintar();
}
