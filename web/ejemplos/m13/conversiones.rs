
#[derive(Debug)]
struct Porcentaje(u8);

// 1. From: conversión que SIEMPRE funciona. Implementarla te regala Into.
impl From<bool> for Porcentaje {
    fn from(b: bool) -> Self {
        if b { Porcentaje(100) } else { Porcentaje(0) }
    }
}

// 2. TryFrom: conversión que PUEDE fallar; devuelve Result.
impl TryFrom<i32> for Porcentaje {
    type Error = String;
    fn try_from(x: i32) -> Result<Self, Self::Error> {
        if (0..=100).contains(&x) {
            Ok(Porcentaje(x as u8))
        } else {
            Err(format!("{x} no está entre 0 y 100"))
        }
    }
}

// 3. AsRef<str>: acepta String, &str, &String… sin copiar nada.
fn letras(texto: impl AsRef<str>) -> usize {
    texto.as_ref().chars().count()
}

fn main() {
    let lleno: Porcentaje = true.into();
    println!("{lleno:?} = {}%", lleno.0);

    println!("{:?}", Porcentaje::try_from(42));
    println!("{:?}", Porcentaje::try_from(250));

    // TryInto es la otra cara de TryFrom (está en el preludio).
    let r: Result<Porcentaje, _> = 7.try_into();
    println!("{r:?}");

    // Conversiones de la biblioteca estándar:
    let grande: i64 = i64::from(7u8); // nunca pierde información
    let pequeno = u8::try_from(300i32); // podría perderla
    println!("{grande} {pequeno:?}");

    println!("{} {}", letras("año"), letras(String::from("cigüeña")));
}
