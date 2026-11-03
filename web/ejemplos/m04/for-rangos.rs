fn main() {
    // 1. `a..b` incluye a `a` y excluye a `b`
    for i in 0..5 {
        print!("{i} ");
    }
    println!("  <- 0..5");

    // 2. `a..=b` incluye los dos extremos
    for i in 1..=5 {
        print!("{i} ");
    }
    println!("  <- 1..=5");

    // 3. Al revés con `.rev()` y de dos en dos con `.step_by(2)`
    for i in (1..=5).rev() {
        print!("{i} ");
    }
    println!("  <- (1..=5).rev()");
    for i in (0..10).step_by(3) {
        print!("{i} ");
    }
    println!("  <- (0..10).step_by(3)");

    // 4. Un rango vacío no da ninguna vuelta
    for i in 5..5 {
        print!("{i} ");
    }
    println!("  <- 5..5 (vacío)");
}
