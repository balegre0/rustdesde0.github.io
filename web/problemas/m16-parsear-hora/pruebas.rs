
struct Azar(u64);

impl Azar {
    fn siguiente(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn main() {
    let casos: [(&str, Option<(u8, u8)>); 17] = [
        ("09:05", Some((9, 5))),
        ("00:00", Some((0, 0))),
        ("23:59", Some((23, 59))),
        ("12:30", Some((12, 30))),
        ("24:00", None),
        ("23:60", None),
        ("+0005:5", None),
        ("9:05", None),
        ("09:5", None),
        ("09-05", None),
        (" 9:05", None),
        ("+9:05", None),
        ("", None),
        ("ñ1:05", None),
        ("12:3ñ", None),
        ("1ñ:0", None),
        ("🕐:00", None),
    ];
    let mut fallos = 0;
    for (texto, esperado) in casos {
        let obtenido = parsear_hora(texto);
        if obtenido == esperado {
            println!("OK: «{texto}» -> {obtenido:?}");
        } else {
            println!("FALLO: «{texto}»: esperaba {esperado:?}, obtuve {obtenido:?}");
            fallos += 1;
        }
    }
    // Propiedad: si acepta un texto, al volver a escribirlo como HH:MM sale el mismo texto.
    let piezas = ["0", "1", "2", "5", "9", ":", "+", "ñ", " "];
    let mut azar = Azar(16);
    let mut propiedad_ok = true;
    for _ in 0..3000 {
        let n = 1 + (azar.siguiente() % 6) as usize;
        let t: String = (0..n).map(|_| piezas[(azar.siguiente() % piezas.len() as u64) as usize]).collect();
        if let Some((h, m)) = parsear_hora(&t) {
            if format!("{h:02}:{m:02}") != t {
                println!("FALLO: propiedad: aceptó «{t}» como {h:02}:{m:02}");
                propiedad_ok = false;
                break;
            }
        }
    }
    if propiedad_ok {
        println!("OK: propiedad de ida y vuelta con 3000 textos aleatorios");
    } else {
        fallos += 1;
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
