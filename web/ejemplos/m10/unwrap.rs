fn main() {
    // 1. unwrap: «dame el valor; si es Err/None, pánico». Útil en pruebas y prototipos.
    let n: i32 = "7".parse().unwrap();
    println!("unwrap: {n}");

    // 2. expect: igual, pero TÚ escribes el mensaje del pánico. Mejor que unwrap.
    let m: i32 = "8".parse().expect("el literal \"8\" es un número válido");
    println!("expect: {m}");

    // 3. Alternativas SIN pánico:
    let x: i32 = "nada".parse().unwrap_or(0); // valor de reserva
    let y: i32 = "nada".parse().unwrap_or_default(); // el «por defecto» del tipo (0)
    let z: i32 = "nada".parse().unwrap_or_else(|e| {
        println!("  (aviso: {e}; uso -1)");
        -1
    });
    println!("unwrap_or: {x}, unwrap_or_default: {y}, unwrap_or_else: {z}");

    // 4. Cuándo SÍ está bien: cuando el fallo es imposible por cómo está escrito el programa.
    let digitos = "2026";
    let anio: u32 = digitos.parse().expect("son cuatro dígitos escritos a mano");
    println!("año {anio}");
}
