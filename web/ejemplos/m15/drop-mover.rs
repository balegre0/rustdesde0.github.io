struct Ruidoso(&'static str);

impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("adiós, {}", self.0);
    }
}

fn quedarse(r: Ruidoso) {
    println!("me quedo con {}", r.0);
}

fn main() {
    let x = Ruidoso("x");
    let y = Ruidoso("y");
    quedarse(x);
    println!("fin de main");
    let _ = &y;
}
