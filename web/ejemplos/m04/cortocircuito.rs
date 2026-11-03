fn es_par(n: i32) -> bool {
    println!("  (miro si {n} es par)");
    n % 2 == 0
}

fn main() {
    // && solo evalúa la derecha si la izquierda es true.
    println!("1: {}", false && es_par(4));
    println!("2: {}", true && es_par(4));
    // || solo evalúa la derecha si la izquierda es false.
    println!("3: {}", true || es_par(5));
    // Así se protege una división: la derecha no se evalúa si d es 0.
    let d = 0;
    println!("4: {}", d != 0 && 10 / d > 2);
}
