struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _ = Ruidoso("a");
    let _b = Ruidoso("b");
    println!("fin");
}
