struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let mut x = Ruidoso("primero");
    println!("antes {}", x.0);
    x = Ruidoso("segundo");
    println!("fin {}", x.0);
}
