use std::ops::{Add, AddAssign, Index, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, o: Vector) -> Vector {
        Vector { x: self.x + o.x, y: self.y + o.y }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, k: f64) -> Vector {
        Vector { x: self.x * k, y: self.y * k }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector { x: -self.x, y: -self.y }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, o: Vector) {
        self.x += o.x;
        self.y += o.y;
    }
}

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("índice {i} fuera de un vector 2D"),
        }
    }
}

fn main() {
    let a = Vector { x: 1.0, y: 2.0 };
    let b = Vector { x: 0.5, y: -1.0 };
    println!("{:?}", a + b);
    println!("{:?}", a * 3.0);
    println!("{:?}", -a);
    let mut c = a;
    c += b;
    println!("{:?} {} {}", c, c[0], c[1]);
    println!("{}", a + b == Vector { x: 1.5, y: 1.0 });
    // los rangos también son tipos de ops
    let r = 2..5;
    println!("{:?} {} {}", r, r.contains(&4), (1..=3).sum::<i32>());
}
