use std::ops::Add;

#[derive(Debug)]
struct Dinero {
    centimos: i64,
}

impl Add for Dinero {
    type Output = Dinero;
    fn add(self, otro: Dinero) -> Dinero {
        Dinero { centimos: self.centimos + otro.centimos }
    }
}

fn main() {
    let a = Dinero { centimos: 150 };
    let b = Dinero { centimos: 275 };
    println!("{:?}", a + b);
}
