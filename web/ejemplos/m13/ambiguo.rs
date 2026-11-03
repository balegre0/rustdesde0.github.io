trait ConvertirA<T> {
    fn convertir(&self) -> T;
}

struct Celsius(f64);

impl ConvertirA<f64> for Celsius {
    fn convertir(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl ConvertirA<String> for Celsius {
    fn convertir(&self) -> String {
        format!("{} °C", self.0)
    }
}

fn main() {
    let t = Celsius(100.0);
    let x = t.convertir();
    println!("{x}");
}
