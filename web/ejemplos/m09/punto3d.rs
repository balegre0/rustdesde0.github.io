struct Punto3(i32, i32, i32);

fn main() {
    let p = Punto3(2, -1, 7);
    println!("suma = {}", p.0 + p.1 + p.2);
}
