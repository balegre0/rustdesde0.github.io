use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Dinero {
    centimos: i64,
}

// 1. Implementa Add para Dinero.
// 2. Implementa Display para que Dinero { centimos: 425 } se vea "4.25 €"
//    (pista: centimos / 100 y centimos % 100 con {:02}).

fn main() {
    let a = Dinero { centimos: 150 };
    let b = Dinero { centimos: 275 };
    println!("{a:?} {b:?}");
    // Descomenta cuando lo tengas:
    // println!("{}", a + b);
    // println!("{}", a + b + Dinero { centimos: 5 });
}
