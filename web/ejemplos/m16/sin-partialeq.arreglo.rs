#[derive(Debug, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

fn origen() -> Punto {
    Punto { x: 0, y: 0 }
}

fn main() {
    assert_eq!(origen(), Punto { x: 0, y: 0 });
    println!("el origen está en su sitio");
}
