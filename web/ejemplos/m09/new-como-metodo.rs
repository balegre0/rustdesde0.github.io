struct Punto {
    x: i32,
    y: i32,
}

impl Punto {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let a = Punto::new(1, 2);
    let b = a.new(5, 6);
    println!("{} {}", b.x, b.y);
}
