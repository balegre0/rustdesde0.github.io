use std::ops::Add;

struct Dinero {
    centimos: u64,
}

impl Add for Dinero {
    type Output = Dinero;
    fn add(self, otro: Dinero) -> Dinero {
        Dinero { centimos: self.centimos + otro.centimos }
    }
}

fn main() {
    let a = Dinero { centimos: 150 };
    let b = Dinero { centimos: 250 };
    let total = a + b;
    println!("{}", total.centimos);
}
