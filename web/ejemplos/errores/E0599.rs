struct Contador {
    n: u32,
}

fn main() {
    let mut c = Contador { n: 0 };
    c.incrementar();
    println!("{}", c.n);
}
