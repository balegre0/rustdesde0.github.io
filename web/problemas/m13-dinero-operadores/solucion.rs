use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Euros(i64);

impl Add for Euros {
    type Output = Euros;
    fn add(self, otro: Euros) -> Euros {
        Euros(self.0 + otro.0)
    }
}

impl Sub for Euros {
    type Output = Euros;
    fn sub(self, otro: Euros) -> Euros {
        Euros(self.0 - otro.0)
    }
}

impl Mul<i64> for Euros {
    type Output = Euros;
    fn mul(self, n: i64) -> Euros {
        Euros(self.0 * n)
    }
}

impl fmt::Display for Euros {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let signo = if self.0 < 0 { "-" } else { "" };
        let n = self.0.abs();
        write!(f, "{signo}{},{:02} €", n / 100, n % 100)
    }
}
