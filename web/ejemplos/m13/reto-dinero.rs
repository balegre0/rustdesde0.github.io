use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Dinero {
    centimos: i64,
}

impl Add for Dinero {
    type Output = Dinero;
    fn add(self, o: Dinero) -> Dinero {
        Dinero { centimos: self.centimos + o.centimos }
    }
}

impl fmt::Display for Dinero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02} €", self.centimos / 100, self.centimos % 100)
    }
}

fn main() {
    let a = Dinero { centimos: 150 };
    let b = Dinero { centimos: 275 };
    println!("{}", a + b);
    println!("{}", a + b + Dinero { centimos: 5 });
}
