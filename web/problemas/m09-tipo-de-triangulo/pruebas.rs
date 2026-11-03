
fn main() {
    let casos = [
        ((3, 3, 3), Triangulo::Equilatero),
        ((3, 3, 5), Triangulo::Isosceles),
        ((5, 3, 3), Triangulo::Isosceles),
        ((3, 5, 3), Triangulo::Isosceles),
        ((3, 4, 5), Triangulo::Escaleno),
        ((1, 2, 3), Triangulo::Invalido),
        ((0, 0, 0), Triangulo::Invalido),
        ((10, 1, 1), Triangulo::Invalido),
        ((7, 10, 5), Triangulo::Escaleno),
    ];
    let mut fallos = 0;
    for ((a, b, c), esperado) in casos {
        let obtenido = clasificar(a, b, c);
        if obtenido == esperado {
            println!("OK: ({a}, {b}, {c}) -> {:?}", obtenido);
        } else {
            println!("FALLO: ({a}, {b}, {c}): esperaba {:?}, obtuve {:?}", esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
