struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Punto { x: 1, y: 2 };
    println!("{}", p.z);
}
