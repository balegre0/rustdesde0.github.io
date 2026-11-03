struct Temperatura {
    celsius: f64,
}

impl Temperatura {
    // 1. Constante asociada: pertenece al tipo, no a un valor concreto.
    const CERO_ABSOLUTO: f64 = -273.15;

    // 2. Función asociada (sin `self`): se llama con `Tipo::funcion(...)`.
    //    Por convención, el constructor principal se llama `new`.
    fn new(celsius: f64) -> Self {
        // 3. `Self` = el tipo del bloque impl (aquí, Temperatura).
        Self { celsius }
    }

    // 4. Otros constructores con nombre propio.
    fn desde_fahrenheit(f: f64) -> Self {
        Self::new((f - 32.0) * 5.0 / 9.0)
    }

    // 5. Un constructor que puede negarse: devuelve Option (lo verás en la lección 6).
    fn valida(celsius: f64) -> Option<Self> {
        if celsius < Self::CERO_ABSOLUTO { None } else { Some(Self::new(celsius)) }
    }
}

// 6. Puedes tener varios bloques impl para el mismo tipo.
impl Temperatura {
    fn describir(&self) -> String {
        format!("{:.1} °C", self.celsius)
    }
}

fn main() {
    let t = Temperatura::new(25.0);
    let f = Temperatura::desde_fahrenheit(212.0);
    println!("{} y {}", t.describir(), f.describir());
    println!("cero absoluto: {}", Temperatura::CERO_ABSOLUTO);
    println!("¿-300 °C es válida? {}", Temperatura::valida(-300.0).is_some());
    println!("¿-10 °C es válida? {}", Temperatura::valida(-10.0).is_some());
}
