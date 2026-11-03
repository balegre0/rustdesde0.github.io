fn main() {
    let r = Rectangulo { ancho: 2.0, alto: 3.0 };
    let c = Circulo { radio: 1.0 };
    let mut fallos = 0;
    let redondear = |x: f64| (x * 1000.0).round() / 1000.0;
    let casos: [(&str, String, String); 5] = [
        ("área del rectángulo", format!("{}", redondear(r.area())), String::from("6")),
        ("área del círculo", format!("{}", redondear(c.area())), String::from("3.142")),
        ("nombre del rectángulo", r.nombre(), String::from("rectángulo")),
        ("nombre por defecto", c.nombre(), String::from("figura")),
        ("área total", format!("{}", redondear(area_total(&[&r, &c, &r]))), String::from("15.142")),
    ];
    for (que, obtenido, esperado) in casos {
        if obtenido == esperado {
            println!("OK: {que} = {obtenido}");
        } else {
            println!("FALLO: {que}: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
