fn main() {
    let casos: [(&[&str], Vec<Vec<&str>>); 4] = [
        (&["roma", "sol", "amor", "los", "mar"], vec![vec!["amor", "roma"], vec!["los", "sol"], vec!["mar"]]),
        (&[], vec![]),
        (&["año", "oña", "ano"], vec![vec!["ano"], vec!["año", "oña"]]),
        (&["ab", "ba", "ab"], vec![vec!["ab", "ab", "ba"]]),
    ];
    let mut fallos = 0;
    for (entrada, esperado) in casos {
        let obtenido = agrupar(entrada);
        let esperado: Vec<Vec<String>> = esperado.iter().map(|g| g.iter().map(|s| s.to_string()).collect()).collect();
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", entrada, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", entrada, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
