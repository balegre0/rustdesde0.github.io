#[derive(Debug)]
struct Dinero {
    centimos: i64,
}

fn main() {
    let a = Dinero { centimos: 150 };
    let b = Dinero { centimos: 275 };
    println!("{:?}", a + b);
}
