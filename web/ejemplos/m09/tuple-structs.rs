// 1. Tuple struct: campos sin nombre, se accede por posición (.0, .1, ...).
struct Color(u8, u8, u8);

// 2. Newtype: un tuple struct de un solo campo que da un tipo NUEVO a un número.
struct Celsius(f64);
struct Fahrenheit(f64);

// 3. Unit struct: sin campos. Ocupa 0 bytes; sirve de «marca».
struct Silencio;

fn a_fahrenheit(c: Celsius) -> Fahrenheit {
    Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
}

fn main() {
    let naranja = Color(255, 165, 0);
    println!("rojo={} verde={} azul={}", naranja.0, naranja.1, naranja.2);

    let hoy = Celsius(21.5);
    let f = a_fahrenheit(hoy);
    println!("21.5 °C = {} °F", f.0);

    // 4. Un tuple struct y una tupla con los mismos datos son tipos distintos.
    let tupla: (u8, u8, u8) = (255, 165, 0);
    println!("tupla: {:?}", tupla);

    let _s = Silencio;
    println!("Silencio ocupa {} bytes", std::mem::size_of::<Silencio>());
    println!("Celsius ocupa {} bytes, igual que un f64 ({})", std::mem::size_of::<Celsius>(), std::mem::size_of::<f64>());
}
