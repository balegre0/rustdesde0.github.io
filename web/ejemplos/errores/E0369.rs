struct Dinero {
    centimos: u64,
}

fn main() {
    let a = Dinero { centimos: 150 };
    let b = Dinero { centimos: 250 };
    let total = a + b;
    println!("{}", total.centimos);
}
