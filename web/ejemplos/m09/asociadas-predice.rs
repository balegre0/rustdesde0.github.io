struct Punto {
    x: i32,
    y: i32,
}

impl Punto {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn origen() -> Self {
        Self::new(0, 0)
    }
}

fn main() {
    let a = Punto::new(3, 4);
    let o = Punto::origen();
    println!("({}, {}) y ({}, {})", a.x, a.y, o.x, o.y);
}
