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
    /// Si se salta de línea estando en la última fila, todo sube una fila (la de arriba se pierde).
    fn escribir_byte(&mut self, b: u8) {
        if b == b'\n' {
            self.salto();
            return;
        }
        if self.columna == COLUMNAS {
            self.salto();
        }
        self.celdas[self.fila][self.columna] = b;
        self.columna += 1;
    }

    fn salto(&mut self) {
        self.columna = 0;
        if self.fila + 1 < FILAS {
            self.fila += 1;
        } else {
            for f in 1..FILAS {
                self.celdas[f - 1] = self.celdas[f];
            }
            self.celdas[FILAS - 1] = [b' '; COLUMNAS];
        }
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
