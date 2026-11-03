// Las macros de comprobación, una a una. Si todas pasan, no dicen nada:
// solo hablan cuando algo falla.

fn doble(x: i32) -> i32 {
    x * 2
}

fn main() {
    // 1. assert!: comprueba que una condición es verdadera.
    assert!(doble(3) > 5);
    println!("1. assert!(doble(3) > 5) ... pasa");

    // 2. assert_eq!: comprueba que dos valores son iguales.
    assert_eq!(doble(21), 42);
    println!("2. assert_eq!(doble(21), 42) ... pasa");

    // 3. assert_ne!: comprueba que dos valores son distintos.
    assert_ne!(doble(1), 3);
    println!("3. assert_ne!(doble(1), 3) ... pasa");

    // 4. Todas aceptan un mensaje extra, con el formato de println!.
    let x = 10;
    assert!(doble(x) % 2 == 0, "doble({x}) debería ser par");
    println!("4. con mensaje propio ... pasa");

    println!("Todas las comprobaciones han pasado.");
}
