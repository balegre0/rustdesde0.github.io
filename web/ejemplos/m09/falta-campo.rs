struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Punto { x: 1 };
    println!("{} {}", p.x, p.y);
}
