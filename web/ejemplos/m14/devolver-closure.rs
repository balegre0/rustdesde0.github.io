// 1. Devolver un closure: su tipo no tiene nombre, así que decimos «algo que es Fn».
fn sumador(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn main() {
    let mas_cinco = sumador(5);
    let mas_cien = sumador(100);
    println!("{} {}", mas_cinco(1), mas_cien(1));

    // 2. Para guardar closures DISTINTOS en una misma lista, los metemos en Box<dyn Fn>.
    let operaciones: Vec<(&str, Box<dyn Fn(i32) -> i32>)> = vec![
        ("doble", Box::new(|x| x * 2)),
        ("cuadrado", Box::new(|x| x * x)),
        ("más cinco", Box::new(sumador(5))),
    ];
    for (nombre, op) in &operaciones {
        println!("{nombre}(7) = {}", op(7));
    }
}
