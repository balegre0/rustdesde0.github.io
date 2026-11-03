use std::ops::{Add, AddAssign, Index, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
struct V2 {
    x: f64,
    y: f64,
}

// 1. a + b  →  Add::add(a, b)
impl Add for V2 {
    type Output = V2;
    fn add(self, o: V2) -> V2 {
        V2 { x: self.x + o.x, y: self.y + o.y }
    }
}

// 2. v * 2.0: el lado derecho puede ser de OTRO tipo (Mul<f64>).
impl Mul<f64> for V2 {
    type Output = V2;
    fn mul(self, k: f64) -> V2 {
        V2 { x: self.x * k, y: self.y * k }
    }
}

// 3. -v  →  Neg::neg(v)
impl Neg for V2 {
    type Output = V2;
    fn neg(self) -> V2 {
        V2 { x: -self.x, y: -self.y }
    }
}

// 4. a += b  →  AddAssign::add_assign(&mut a, b)
impl AddAssign for V2 {
    fn add_assign(&mut self, o: V2) {
        self.x += o.x;
        self.y += o.y;
    }
}

// 5. v[0]  →  *Index::index(&v, 0)
impl Index<usize> for V2 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("V2 solo tiene los índices 0 y 1, no {i}"),
        }
    }
}

fn main() {
    let a = V2 { x: 1.0, y: 2.0 };
    let b = V2 { x: 0.5, y: -1.0 };
    println!("{:?}", a + b);
    println!("{:?}", a * 3.0);
    println!("{:?}", -a);
    let mut c = a;
    c += b;
    c += b;
    println!("{:?} x={} y={}", c, c[0], c[1]);
    println!("{}", a + b == V2 { x: 1.5, y: 1.0 }); // == viene de PartialEq
    println!("{}", a[2]);
}
