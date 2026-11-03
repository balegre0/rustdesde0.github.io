// Escribe invertir: devuelve el texto al revés, carácter a carácter.
// main lo somete a pruebas de propiedades con textos aleatorios con «ñ» y tildes.
fn invertir(s: &str) -> String {
    s.chars().rev().collect()
}

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
    // Un caso concreto primero.
    assert_eq!(invertir("año"), "oña");
    let letras = ['a', 'ñ', 'é', 'z', ' ', 'Ü', '1'];
    let mut azar = Azar(99);
    for _ in 0..500 {
        let largo = (azar.siguiente() % 12) as usize;
        let s: String = (0..largo)
            .map(|_| letras[(azar.siguiente() % letras.len() as u64) as usize])
            .collect();
        let r = invertir(&s);
        // Propiedad 1: invertir dos veces deja el texto como estaba.
        assert_eq!(invertir(&r), s);
        // Propiedad 2: no cambia el número de caracteres.
        assert_eq!(r.chars().count(), s.chars().count());
        // Propiedad 3: el primer carácter pasa a ser el último.
        assert_eq!(r.chars().last(), s.chars().next());
    }
    println!("500 textos aleatorios: invertir cumple las 3 propiedades");
}
