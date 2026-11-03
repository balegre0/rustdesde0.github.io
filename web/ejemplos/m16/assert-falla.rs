fn suma(a: i32, b: i32) -> i32 {
    a - b // ¡ups! una resta disfrazada de suma
}

fn main() {
    println!("Comprobando suma...");
    assert_eq!(suma(2, 2), 4);
    println!("suma funciona");
}
