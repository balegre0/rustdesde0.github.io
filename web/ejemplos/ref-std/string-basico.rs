fn main() {
    // 1. crear
    let mut s = String::new();
    let saludo = String::from("¡Hola");
    let otro = "mundo".to_string();
    // 2. hacer crecer
    s.push_str(&saludo);
    s.push(',');
    s += " ";
    s += &otro;
    s.push('!');
    println!("{s}");
    // 3. longitud en bytes y en caracteres
    println!("bytes: {}, chars: {}", s.len(), s.chars().count());
    // 4. capacidad: reserva de memoria
    let mut v = String::with_capacity(10);
    v.push_str("año");
    println!("len {} cap {}", v.len(), v.capacity());
    // 5. formatear, insertar, quitar
    let f = format!("{}-{}", saludo, 2024);
    println!("{f}");
    let mut t = String::from("ola");
    t.insert(0, 'h');
    let ultimo = t.pop();
    println!("{t} {ultimo:?}");
    t.clear();
    println!("vacío: {}", t.is_empty());
}
