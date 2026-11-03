fn main() {
    let precio = 2 + 2;
    assert!(precio > 0, "el precio debe ser positivo");
    println!("primer assert superado");
    assert_eq!(precio, 5, "el total no cuadra");
    println!("esto no se imprime");
}
