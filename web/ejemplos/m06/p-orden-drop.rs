struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _x = Ruidoso("x");
    {
        let _y = Ruidoso("y");
    }
    let _z = Ruidoso("z");
    println!("fin");
}
