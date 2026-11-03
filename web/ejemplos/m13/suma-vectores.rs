use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct V2 {
    x: i32,
    y: i32,
}

impl Add for V2 {
    type Output = V2;
    fn add(self, otro: V2) -> V2 {
        V2 { x: self.x + otro.x, y: self.y + otro.y }
    }
}

fn main() {
    let a = V2 { x: 1, y: 2 };
    let b = V2 { x: 10, y: 20 };
    println!("{:?}", a + b + a);
}
