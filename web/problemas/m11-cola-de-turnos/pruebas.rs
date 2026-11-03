fn main() {
    let v = |xs: &[&str]| xs.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let casos: [(&[&str], Vec<String>, Vec<String>); 4] = [
        (&["llega Ana", "llega Luis", "atiende"], v(&["Ana"]), v(&["Luis"])),
        (&["llega Ana", "urgente Zoe", "atiende", "atiende"], v(&["Zoe", "Ana"]), v(&[])),
        (&["atiende", "llega Bea"], v(&[]), v(&["Bea"])),
        (&["llega A", "llega B", "urgente C", "atiende", "llega D"], v(&["C"]), v(&["A", "B", "D"])),
    ];
    let mut fallos = 0;
    for (ordenes, a, q) in casos {
        let obtenido = atendidos(ordenes);
        let esperado = (a, q);
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", ordenes, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", ordenes, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
