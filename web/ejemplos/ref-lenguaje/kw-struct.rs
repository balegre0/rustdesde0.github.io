// 1. Struct con campos con nombre.
#[derive(Debug, Clone)]
struct Usuario {
    nombre: String,
    edad: u8,
    activo: bool,
}

// 2. Struct tupla y struct unidad.
#[derive(Debug)]
struct Metros(f64);
#[derive(Debug)]
struct Marcador;

fn main() {
    let edad = 30;
    // Atajo: `edad` en lugar de `edad: edad`.
    let u = Usuario { nombre: "Eva".to_string(), edad, activo: true };
    // 3. Actualización: copia el resto de campos de `u`.
    let u2 = Usuario { nombre: "Leo".to_string(), ..u.clone() };
    println!("{} {} {}", u.nombre, u.edad, u.activo);
    println!("{u2:?}");
    let m = Metros(1.5);
    println!("{} {:?} {:?}", m.0, m, Marcador);
    println!("{} bytes", size_of::<Marcador>());
}
