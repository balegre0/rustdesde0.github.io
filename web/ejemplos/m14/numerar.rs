// Devuelve cada nombre con su número de orden, empezando en 1, saltándote los vacíos.
fn numerar(nombres: &[&str]) -> Vec<String> {
    nombres
        .iter()
        .filter(|n| !n.is_empty())
        .enumerate()
        .map(|(i, n)| format!("{}. {n}", i + 1))
        .collect()
}

fn main() {
    for linea in numerar(&["Ana", "", "Leo", "Zoë"]) {
        println!("{linea}");
    }
    println!("{:?}", numerar(&[]));
}
