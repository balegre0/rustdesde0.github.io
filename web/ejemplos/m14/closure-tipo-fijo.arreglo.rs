fn main() {
    // Dos closures distintos: cada uno fija su propio tipo.
    let identidad_entero = |x: i32| x;
    let identidad_real = |x: f64| x;
    let a = identidad_entero(5);
    let b = identidad_real(2.5);
    println!("{a} {b}");
}
