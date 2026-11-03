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
    let b = Punto::new(5, 6);
    println!("{} {} {} {}", a.x, a.y, b.x, b.y);
}
