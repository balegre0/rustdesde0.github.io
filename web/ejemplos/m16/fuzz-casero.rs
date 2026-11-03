// Un «fuzzer» de juguete: lanza miles de entradas aleatorias contra una
// función y para en cuanto una la hace fallar.

// Generador pseudoaleatorio xorshift: siempre da la misma secuencia
// para la misma semilla, así el fallo se puede reproducir.
struct Azar(u64);

impl Azar {
    fn siguiente(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

// La función bajo prueba: lee «HH:MM» y devuelve los minutos del día.
// Tiene un fallo escondido. ¿Lo ves antes de que lo encuentre el fuzzer?
fn minutos(texto: &str) -> Option<u32> {
    let (h, m) = texto.split_once(':')?;
    let h: u32 = h.parse().ok()?;
    let m: u32 = m.parse().ok()?;
    if h > 23 || m > 59 {
        return None;
    }
    Some(h * 60 + m)
}

// Comprueba lo que SIEMPRE debe cumplirse: el resultado cabe en un día.
fn propiedad(texto: &str) -> bool {
    match minutos(texto) {
        Some(total) => total < 24 * 60 && texto.len() <= 5,
        None => true,
    }
}

fn main() {
    let mut azar = Azar(2024);
    let piezas = ["0", "1", "2", "3", "5", "9", ":", "+", "-", "00"];
    for intento in 1..=100_000 {
        // 1. Fabricamos una entrada juntando de 1 a 6 piezas al azar.
        let largo = 1 + azar.siguiente() % 6;
        let mut texto = String::new();
        for _ in 0..largo {
            texto.push_str(piezas[(azar.siguiente() % piezas.len() as u64) as usize]);
        }
        // 2. Si la propiedad falla, hemos encontrado un caso raro.
        if !propiedad(&texto) {
            println!("Intento {intento}: «{texto}» -> {:?}", minutos(&texto));
            return;
        }
    }
    println!("Ningún fallo en 100 000 intentos");
}
