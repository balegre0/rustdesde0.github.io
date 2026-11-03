fn main() {
    let ok = |n: &str, p: u16, d: bool| Ok(Config { nombre: n.to_string(), puerto: p, depurar: d });
    let casos: [(&str, Result<Config, ErrorConfig>); 7] = [
        ("nombre = web\npuerto = 8080\ndepurar = true", ok("web", 8080, true)),
        ("# comentario\n\n  puerto=80 \ndepurar=false\nnombre = mi app", ok("mi app", 80, false)),
        ("nombre = x\npuerto 80", Err(ErrorConfig::Formato(2))),
        ("color = rojo", Err(ErrorConfig::ClaveDesconocida(1, "color".into()))),
        ("nombre = x\npuerto = 70000", Err(ErrorConfig::Valor(2, "puerto".into()))),
        ("depurar = si", Err(ErrorConfig::Valor(1, "depurar".into()))),
        ("puerto = 1\ndepurar = true", Err(ErrorConfig::Falta("nombre"))),
    ];
    let mut fallos = 0;
    for (texto, esperado) in casos {
        let obtenido = leer(texto);
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", texto, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", texto, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
