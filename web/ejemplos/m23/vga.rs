// El modo texto VGA: una matriz de 25 × 80 celdas en la dirección 0xb8000.
// Cada celda son 2 bytes: el carácter (ASCII) y el color.
// Aquí la simulamos con un array y la «pintamos» al final (más pequeña: 5 × 20).
const FILAS: usize = 5;
const COLUMNAS: usize = 20;

#[derive(Clone, Copy)]
#[repr(C)]
struct Celda {
    caracter: u8,
    color: u8, // 4 bits de fondo, 4 de texto
}

struct Pantalla {
    celdas: [[Celda; COLUMNAS]; FILAS],
    columna: usize,
}

impl Pantalla {
    fn nueva() -> Pantalla {
        Pantalla { celdas: [[Celda { caracter: b' ', color: 0x0f }; COLUMNAS]; FILAS], columna: 0 }
    }

    fn escribir_byte(&mut self, b: u8) {
        match b {
            b'\n' => self.nueva_linea(),
            b => {
                if self.columna >= COLUMNAS {
                    self.nueva_linea();
                }
                let fila = FILAS - 1; // siempre se escribe en la última fila
                self.celdas[fila][self.columna] = Celda { caracter: b, color: 0x0f };
                self.columna += 1;
            }
        }
    }

    fn nueva_linea(&mut self) {
        // Desplaza todo una fila hacia arriba y vacía la última.
        for f in 1..FILAS {
            self.celdas[f - 1] = self.celdas[f];
        }
        self.celdas[FILAS - 1] = [Celda { caracter: b' ', color: 0x0f }; COLUMNAS];
        self.columna = 0;
    }

    fn escribir(&mut self, s: &str) {
        for b in s.bytes() {
            // Solo ASCII imprimible o salto de línea; lo demás, un cuadradito (0xfe).
            self.escribir_byte(if b == b'\n' || (0x20..=0x7e).contains(&b) { b } else { 0xfe });
        }
    }

    fn pintar(&self) {
        println!("+{}+", "-".repeat(COLUMNAS));
        for fila in &self.celdas {
            let texto: String = fila.iter().map(|c| if c.caracter == 0xfe { '■' } else { c.caracter as char }).collect();
            println!("|{texto}|");
        }
        println!("+{}+", "-".repeat(COLUMNAS));
    }
}

fn main() {
    let mut p = Pantalla::nueva();
    p.escribir("Hola, kernel!\nLinea 2\nUna linea demasiado larga para caber\nñ no es ASCII");
    p.pintar();
    println!("bytes por celda: {}", size_of::<Celda>());
}
