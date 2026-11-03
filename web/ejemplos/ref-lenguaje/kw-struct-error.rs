struct Punto { x: i32, y: i32 }

fn main() {
    // Al crear un struct hay que dar todos los campos.
    let p = Punto { x: 1 };
    println!("{}", p.x);
}
