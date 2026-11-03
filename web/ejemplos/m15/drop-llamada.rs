struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("adiós, {}", self.0);
    }
}

fn main() {
    let x = Ruidoso("x");
    x.drop(); // ¿llamar al destructor a mano?
    println!("fin");
}
