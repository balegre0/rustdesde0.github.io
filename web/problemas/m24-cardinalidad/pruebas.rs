fn main() {
    use Tipo::*;
    let casos: [(&str, Tipo, u128); 8] = [
        ("(bool, u8)", Tupla(vec![Bool, U8]), 512),
        ("Option<bool>", Opcion(Box::new(Bool)), 3),
        ("Result<bool, Option<bool>>", Enum(vec![Bool, Opcion(Box::new(Bool))]), 5),
        ("()", Tupla(vec![]), 1),
        ("enum sin variantes", Enum(vec![]), 0),
        ("[bool; 10]", Array(Box::new(Bool), 10), 1024),
        ("Result<u8, Infallible>", Enum(vec![U8, Vacio]), 256),
        ("(u8, Infallible)", Tupla(vec![U8, Vacio]), 0),
    ];
    let mut fallos = 0;
    for (nombre, t, esperado) in casos {
        let obtenido = cardinalidad(&t);
        if obtenido == esperado {
            println!("OK: |{nombre}| = {obtenido}");
        } else {
            println!("FALLO: |{nombre}|: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    let _ = Unidad;
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
