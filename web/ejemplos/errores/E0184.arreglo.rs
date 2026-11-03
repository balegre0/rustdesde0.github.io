#[derive(Clone)]
struct Ficha(u32);

impl Drop for Ficha {
    fn drop(&mut self) {
        println!("adiós, ficha {}", self.0);
    }
}

fn main() {
    let f = Ficha(1);
    println!("{}", f.0);
}
