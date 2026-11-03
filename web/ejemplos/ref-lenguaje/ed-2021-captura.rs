struct Persona { nombre: String, apodos: Vec<String> }

fn main() {
    let mut p = Persona { nombre: "Ada".into(), apodos: vec![] };
    // Desde la edición 2021, la closure captura solo `p.apodos`, no `p` entero.
    let mut agregar = |a: &str| p.apodos.push(a.to_string());
    // Así que podemos tocar `p.nombre` mientras la closure está viva.
    p.nombre.push_str(" L.");
    agregar("condesa");
    agregar("matemática");
    println!("{} {:?}", p.nombre, p.apodos);
}
