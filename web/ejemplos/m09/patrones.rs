struct Punto {
    x: i32,
    y: i32,
}

enum Evento {
    Clic { pos: Punto, boton: u8 },
    Tecla(char),
    Rueda(i32),
}

fn describir(e: &Evento) -> String {
    match e {
        // 1. Patrón anidado: desmontar un struct dentro de una variante.
        Evento::Clic { pos: Punto { x: 0, y: 0 }, .. } => String::from("clic en la esquina"),
        // 2. Guarda (`if`): una condición extra para el brazo.
        Evento::Clic { pos: Punto { x, y }, boton } if *boton == 2 => format!("clic derecho en ({x}, {y})"),
        // 3. `..` ignora los campos que no te interesan.
        Evento::Clic { pos, .. } => format!("clic en ({}, {})", pos.x, pos.y),
        // 4. Rango de caracteres + enlace con `@`: comprobar y guardar a la vez.
        Evento::Tecla(c @ 'a'..='z') => format!("minúscula {c}"),
        // 5. Patrón «o» con `|`.
        Evento::Tecla('ñ' | 'Ñ') => String::from("¡la eñe!"),
        Evento::Tecla(c) => format!("otra tecla: {c}"),
        // 6. Rangos de números (también semiabiertos: `1..`).
        Evento::Rueda(0) => String::from("rueda quieta"),
        Evento::Rueda(n @ 1..) => format!("rueda arriba {n}"),
        Evento::Rueda(n) => format!("rueda abajo {}", -n),
    }
}

fn main() {
    let eventos = [
        Evento::Clic { pos: Punto { x: 0, y: 0 }, boton: 1 },
        Evento::Clic { pos: Punto { x: 4, y: 9 }, boton: 2 },
        Evento::Clic { pos: Punto { x: 4, y: 9 }, boton: 1 },
        Evento::Tecla('q'),
        Evento::Tecla('ñ'),
        Evento::Tecla('7'),
        Evento::Rueda(3),
        Evento::Rueda(-2),
    ];
    for e in &eventos {
        println!("{}", describir(e));
    }

    // 7. Patrones de slice: primero, último y «el resto».
    let notas = [4, 8, 6, 10];
    // Con un array de tamaño fijo este patrón SIEMPRE encaja: basta un `let`.
    let [primera, .., ultima] = notas;
    println!("primera {primera}, última {ultima}");
    match &notas[..] {
        [] => println!("vacío"),
        [solo] => println!("solo una: {solo}"),
        [cabeza, resto @ ..] => println!("cabeza {cabeza}, quedan {}", resto.len()),
    }

    // 8. Los patrones también funcionan en `let` y en parámetros.
    let Punto { x: a, y: b } = Punto { x: 10, y: 20 };
    println!("a={a} b={b}");
}
