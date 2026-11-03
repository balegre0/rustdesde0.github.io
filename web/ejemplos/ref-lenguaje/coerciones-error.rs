fn imprimir(v: &[&str]) {
    for s in v { println!("{s}"); }
}

fn main() {
    let nombres: Vec<String> = vec!["Ana".into(), "Luis".into()];
    let refs: Vec<&String> = nombres.iter().collect();
    // La coerción &String -> &str no entra «dentro» de un slice: &[&String] no es &[&str].
    imprimir(&refs);
}
