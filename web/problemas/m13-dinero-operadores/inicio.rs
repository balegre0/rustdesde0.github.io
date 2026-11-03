use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Euros(i64);

impl Add for Euros {
    type Output = Euros;
    fn add(self, otro: Euros) -> Euros {
        todo!()
    }
}

impl Sub for Euros {
    type Output = Euros;
    fn sub(self, otro: Euros) -> Euros {
        todo!()
    }
}

impl Mul<i64> for Euros {
    type Output = Euros;
    fn mul(self, n: i64) -> Euros {
        todo!()
    }
}

impl fmt::Display for Euros {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
