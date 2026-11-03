fn parsear_edad(texto: &str) -> Option<u32> {
    // `parse` devuelve un Result; `.ok()` lo convierte en Option (mundo 10).
    texto.trim().parse().ok()
}

// 4. let else: «saca esto o vete». El else TIENE que salir (return, break, panic…).
fn saludar(entrada: &str) -> String {
    let Some(edad) = parsear_edad(entrada) else {
        return String::from("eso no es una edad");
    };
    // Aquí `edad` ya es un u32, sin Option.
    format!("tienes {edad} años")
}

fn main() {
    // 1. if let: un solo patrón, sin obligación de cubrir el resto.
    let config_color: Option<&str> = Some("azul");
    if let Some(color) = config_color {
        println!("color elegido: {color}");
    }

    // 2. if let ... else
    let nada: Option<&str> = None;
    if let Some(c) = nada {
        println!("color: {c}");
    } else {
        println!("sin color, uso el de siempre");
    }

    // 3. while let: repetir mientras el patrón encaje.
    let mut tareas = vec!["fregar", "estudiar Rust", "dormir"];
    while let Some(t) = tareas.pop() {
        println!("hago: {t}");
    }

    println!("{}", saludar(" 31 "));
    println!("{}", saludar("treinta"));

    // 5. Cadenas de let (let chains, edición 2024): varias condiciones con &&.
    let usuario = Some("Iñigo");
    let puntos = Some(120);
    if let Some(u) = usuario && let Some(p) = puntos && p > 100 {
        println!("{u} tiene premio ({p} puntos)");
    }
}
