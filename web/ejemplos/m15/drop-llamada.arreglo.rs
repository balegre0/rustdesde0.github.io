struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("adiós, {}", self.0);
    }
}

fn main() {
    let x = Ruidoso("x");
    drop(x); // std::mem::drop: se queda con x y lo suelta una sola vez
    println!("fin");
}
