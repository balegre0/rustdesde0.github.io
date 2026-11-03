struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _x = Ruidoso("primero");
    let _x = Ruidoso("segundo");
    println!("fin");
}
