struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let v = vec![Ruidoso("uno"), Ruidoso("dos")];
    println!("antes de soltar");
    drop(v);
    println!("fin");
}
