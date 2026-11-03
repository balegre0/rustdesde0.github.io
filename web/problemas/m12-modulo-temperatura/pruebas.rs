fn main() {
    use temperatura::conversion::a_fahrenheit;
    use temperatura::Celsius;
    let mut fallos = 0;
    let casos = [(100.0, Some(212.0)), (0.0, Some(32.0)), (-40.0, Some(-40.0)), (-273.15, Some(-459.67)), (-300.0, None)];
    for (c, esperado) in casos {
        let obtenido = Celsius::nueva(c).map(|t| (a_fahrenheit(&t) * 100.0).round() / 100.0);
        if obtenido == esperado {
            println!("OK: {c} °C -> {:?}", obtenido);
        } else {
            println!("FALLO: {c} °C: esperaba {:?}, obtuve {:?}", esperado, obtenido);
            fallos += 1;
        }
    }
    let t = Celsius::nueva(21.5);
    if t.as_ref().map(|t| t.grados()) == Some(21.5) {
        println!("OK: grados() = 21.5");
    } else {
        println!("FALLO: grados() debía devolver 21.5");
        fallos += 1;
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
