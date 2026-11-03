
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
    let mut fallos = 0;
    let comp = [
        ("aaabcc", "3a1b2c"),
        ("ñññ", "3ñ"),
        ("", ""),
        ("a", "1a"),
        ("abab", "1a1b1a1b"),
        ("  é", "2 1é"),
    ];
    for (entrada, esperado) in comp {
        let obtenido = comprimir(entrada);
        if obtenido == esperado {
            println!("OK: comprimir(«{entrada}») = «{obtenido}»");
        } else {
            println!("FALLO: comprimir(«{entrada}»): esperaba «{esperado}», obtuve «{obtenido}»");
            fallos += 1;
        }
    }
    let desc: [(&str, Option<&str>); 8] = [
        ("3a1b2c", Some("aaabcc")),
        ("2ñ1a", Some("ñña")),
        ("12x", Some("xxxxxxxxxxxx")),
        ("", Some("")),
        ("a", None),
        ("3", None),
        ("x3", None),
        ("0a", Some("")),
    ];
    for (entrada, esperado) in desc {
        let obtenido = descomprimir(entrada);
        if obtenido.as_deref() == esperado {
            println!("OK: descomprimir(«{entrada}») = {obtenido:?}");
        } else {
            println!("FALLO: descomprimir(«{entrada}»): esperaba {esperado:?}, obtuve {obtenido:?}");
            fallos += 1;
        }
    }
    // Propiedad de ida y vuelta.
    let letras = ['a', 'a', 'a', 'b', 'ñ', 'é', ' ', '🦀'];
    let mut azar = Azar(2024);
    let mut propiedad_ok = true;
    for _ in 0..1000 {
        let n = (azar.siguiente() % 30) as usize;
        let s: String = (0..n)
            .map(|_| letras[(azar.siguiente() % letras.len() as u64) as usize])
            .collect();
        let ida = comprimir(&s);
        if descomprimir(&ida).as_deref() != Some(s.as_str()) {
            println!("FALLO: propiedad: «{s}» -> «{ida}» -> {:?}", descomprimir(&ida));
            propiedad_ok = false;
            break;
        }
    }
    if propiedad_ok {
        println!("OK: ida y vuelta con 1000 textos aleatorios");
    } else {
        fallos += 1;
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
