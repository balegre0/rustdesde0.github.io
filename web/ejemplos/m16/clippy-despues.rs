// La misma lógica tras hacer caso a Clippy.
fn es_vacia(v: &[i32]) -> bool {
    v.is_empty()
}

fn total(v: &[i32]) -> i32 {
    v.iter().sum()
}

fn main() {
    let notas = vec![7, 5, 9];
    println!("¿vacía? {}", es_vacia(&notas));
    println!("total: {}", total(&notas));
}
