struct Contador {
    n: u32,
}

impl Contador {
    fn incrementar(&mut self) {
        self.n += 1;
    }
}

fn main() {
    let mut c = Contador { n: 0 };
    c.incrementar();
    println!("{}", c.n);
}
