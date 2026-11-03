// Completa es_par para que todas las comprobaciones de main pasen.
fn es_par(n: i32) -> bool {
    todo!()
}

fn main() {
    assert!(es_par(4));
    assert!(!es_par(7));
    assert!(es_par(0), "el cero es par");
    assert!(es_par(-2), "los negativos también pueden ser pares");
    assert_eq!(es_par(-3), false);
    println!("5 comprobaciones superadas");
}
