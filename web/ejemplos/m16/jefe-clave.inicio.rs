// Jefe del mundo 16: una función con su batería de pruebas completa.
// Escribe validar_clave. Las reglas, en este orden:
//   1. al menos 8 caracteres (caracteres, no bytes)  -> Corta
//   2. ningún espacio en blanco                        -> ConEspacio
//   3. al menos un dígito del 0 al 9                   -> SinNumero
//   4. al menos una letra mayúscula (también Ñ, Á...)  -> SinMayuscula

#[derive(Debug, PartialEq)]
enum ErrorClave {
    Corta,
    ConEspacio,
    SinNumero,
    SinMayuscula,
}

fn validar_clave(clave: &str) -> Result<(), ErrorClave> {
    todo!()
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
    // 1. Pruebas de ejemplos: una por regla y los casos límite.
    let casos = [
        ("Abc1", Err(ErrorClave::Corta)),
        ("Abcdef1", Err(ErrorClave::Corta)),
        ("Abcdefg1", Ok(())),
        ("Abc def12", Err(ErrorClave::ConEspacio)),
        ("Abcdefgh", Err(ErrorClave::SinNumero)),
        ("abcdefg1", Err(ErrorClave::SinMayuscula)),
        ("ñandú1Ñx", Ok(())),
        ("ñandú1ñx", Err(ErrorClave::SinMayuscula)),
        ("ÑÑÑ1", Err(ErrorClave::Corta)),
    ];
    for (clave, esperado) in &casos {
        assert_eq!(&validar_clave(clave), esperado, "con la clave «{clave}»");
        println!("ok  {clave:<10} -> {esperado:?}");
    }

    // 2. Prueba de propiedades: nunca entra en pánico y, si acepta, cumple todo.
    let piezas = ["a", "B", "7", " ", "ñ", "Ñ", "xyz"];
    let mut azar = Azar(16);
    let mut aceptadas = 0;
    for _ in 0..2000 {
        let n = (azar.siguiente() % 8) as usize;
        let clave: String = (0..n)
            .map(|_| piezas[(azar.siguiente() % piezas.len() as u64) as usize])
            .collect();
        if validar_clave(&clave).is_ok() {
            assert!(clave.chars().count() >= 8);
            assert!(clave.chars().any(|c| c.is_ascii_digit()));
            assert!(!clave.contains(' '));
            aceptadas += 1;
        }
    }
    println!("propiedades: 2000 claves aleatorias, {aceptadas} aceptadas, todas correctas");
}
