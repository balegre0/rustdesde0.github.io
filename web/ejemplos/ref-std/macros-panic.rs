fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("división por cero: {a} / {b}");
    }
    a / b
}

fn main() {
    println!("{}", dividir(10, 2));
    println!("{}", dividir(1, 0));
    println!("no se llega aquí");
}
