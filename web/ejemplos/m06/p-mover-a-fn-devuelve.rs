struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn pasar(r: Ruidoso) {
    println!("dentro de pasar: {}", r.0);
}

fn main() {
    let r = Ruidoso("caja");
    pasar(r);
    println!("fin de main");
}
