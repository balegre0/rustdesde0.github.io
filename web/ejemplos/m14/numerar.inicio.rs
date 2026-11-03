// Devuelve cada nombre con su número de orden, empezando en 1, saltándote los vacíos.
// Usa una cadena de adaptadores: filter, enumerate, map… y collect al final.
fn numerar(nombres: &[&str]) -> Vec<String> {
    todo!()
}

fn main() {
    for linea in numerar(&["Ana", "", "Leo", "Zoë"]) {
        println!("{linea}");
    }
    println!("{:?}", numerar(&[]));
}
