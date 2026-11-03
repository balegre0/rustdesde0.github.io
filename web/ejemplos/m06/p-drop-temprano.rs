struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let a = Ruidoso("a");
    let _b = Ruidoso("b");
    drop(a);
    println!("fin");
}
