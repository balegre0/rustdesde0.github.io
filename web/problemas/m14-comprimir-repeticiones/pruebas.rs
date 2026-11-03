fn main() {
    let casos: [(&str, &[(char, usize)]); 6] = [
        ("aaabcc", &[('a', 3), ('b', 1), ('c', 2)]),
        ("abab", &[('a', 1), ('b', 1), ('a', 1), ('b', 1)]),
        ("ññn", &[('ñ', 2), ('n', 1)]),
        ("", &[]),
        ("z", &[('z', 1)]),
        ("   !!", &[(' ', 3), ('!', 2)]),
    ];
    let mut fallos = 0;
    for (texto, esperado) in casos {
        let r = comprimir(texto);
        if r == esperado {
            println!("OK: comprimir({texto:?}) = {r:?}");
        } else {
            println!("FALLO: comprimir({texto:?}) = {r:?}, se esperaba {esperado:?}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
