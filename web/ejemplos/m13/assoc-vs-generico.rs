// 1. Parámetro genérico: un tipo puede implementar el trait VARIAS veces,
//    una por cada T.
trait ConvertirA<T> {
    fn convertir(&self) -> T;
}

struct Celsius(f64);

impl ConvertirA<f64> for Celsius {
    fn convertir(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0 // a Fahrenheit
    }
}

impl ConvertirA<String> for Celsius {
    fn convertir(&self) -> String {
        format!("{} °C", self.0)
    }
}

// 2. Tipo asociado: cada tipo implementa el trait UNA vez y elige su Item.
trait Generador {
    type Item;
    fn generar(&mut self) -> Self::Item;
}

struct Contador {
    n: u32,
}

impl Generador for Contador {
    type Item = u32;
    fn generar(&mut self) -> u32 {
        self.n += 1;
        self.n
    }
}

// 3. Con tipo asociado, la función no necesita un segundo genérico.
fn tres<G: Generador>(g: &mut G) -> [G::Item; 3] {
    [g.generar(), g.generar(), g.generar()]
}

// 4. Una constante asociada: un valor que cada tipo fija.
trait Moneda {
    const SIMBOLO: &'static str;
}
struct Euro;
impl Moneda for Euro {
    const SIMBOLO: &'static str = "€";
}

fn main() {
    let t = Celsius(100.0);
    // Hay dos impl: hay que decir cuál queremos con una anotación.
    let f: f64 = t.convertir();
    let s: String = t.convertir();
    println!("{f} / {s}");

    let mut c = Contador { n: 0 };
    println!("{:?}", tres(&mut c));
    println!("Precio: 5 {}", Euro::SIMBOLO);
}
