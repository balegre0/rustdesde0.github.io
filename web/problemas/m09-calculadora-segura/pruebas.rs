
fn main() {
    let casos = [
        ("2 + 3", Op::Sumar, 2, 3, Some(5)),
        ("10 - 15", Op::Restar, 10, 15, Some(-5)),
        ("6 * 7", Op::Multiplicar, 6, 7, Some(42)),
        ("7 / 2", Op::Dividir, 7, 2, Some(3)),
        ("1 / 0", Op::Dividir, 1, 0, None),
        ("MAX * 2", Op::Multiplicar, i32::MAX, 2, None),
        ("MAX + 1", Op::Sumar, i32::MAX, 1, None),
        ("MIN - 1", Op::Restar, i32::MIN, 1, None),
        ("MIN / -1", Op::Dividir, i32::MIN, -1, None),
        ("-8 / 2", Op::Dividir, -8, 2, Some(-4)),
    ];
    let mut fallos = 0;
    for (nombre, op, a, b, esperado) in casos {
        let obtenido = aplicar(op, a, b);
        if obtenido == esperado {
            println!("OK: {nombre} -> {:?}", obtenido);
        } else {
            println!("FALLO: {nombre}: esperaba {:?}, obtuve {:?}", esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
