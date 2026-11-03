struct Contador {
    n: u32,
}

impl Contador {
    fn sumar(&mut self) {
        self.n += 1;
    }
}

fn main() {
    let c = Contador { n: 0 };
    c.sumar();
    println!("{}", c.n);
}
