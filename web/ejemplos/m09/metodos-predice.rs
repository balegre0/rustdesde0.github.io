struct Contador {
    n: u32,
}

impl Contador {
    fn sumar(&mut self) {
        self.n += 1;
    }
    fn valor(&self) -> u32 {
        self.n
    }
}

fn main() {
    let mut c = Contador { n: 10 };
    c.sumar();
    c.sumar();
    println!("{}", c.valor());
}
