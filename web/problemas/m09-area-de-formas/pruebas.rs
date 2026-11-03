
fn main() {
    let casos = [
        ("rectángulo 3×4", Forma::Rectangulo { ancho: 3.0, alto: 4.0 }, 12.0),
        ("triángulo 10×3", Forma::Triangulo { base: 10.0, altura: 3.0 }, 15.0),
        ("círculo r=1", Forma::Circulo { radio: 1.0 }, PI),
        ("círculo r=2", Forma::Circulo { radio: 2.0 }, 4.0 * PI),
        ("rectángulo 0×9", Forma::Rectangulo { ancho: 0.0, alto: 9.0 }, 0.0),
    ];
    let mut fallos = 0;
    for (nombre, forma, esperado) in &casos {
        let obtenido = area(forma);
        if (obtenido - esperado).abs() < 1e-9 {
            println!("OK: {nombre} = {obtenido}");
        } else {
            println!("FALLO: {nombre}: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
