fn imprimir(v: &[&str]) {
    for s in v { println!("{s}"); }
}

fn main() {
    let nombres: Vec<String> = vec!["Ana".into(), "Luis".into()];
    // Convertir cada elemento a mano.
    let refs: Vec<&str> = nombres.iter().map(|s| s.as_str()).collect();
    imprimir(&refs);
}
