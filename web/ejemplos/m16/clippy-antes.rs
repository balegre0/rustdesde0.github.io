// Compila y funciona... pero Clippy tiene 7 cosas que decir.
fn es_vacia(v: &Vec<i32>) -> bool {
    if v.len() == 0 { return true; } else { return false; }
}

fn total(v: &[i32]) -> i32 {
    let mut t = 0;
    for i in 0..v.len() {
        t = t + v[i];
    }
    t
}

fn main() {
    let notas = vec![7, 5, 9];
    println!("¿vacía? {}", es_vacia(&notas));
    println!("total: {}", total(&notas));
}
