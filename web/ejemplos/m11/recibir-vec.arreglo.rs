fn total(precios: &[f64]) -> f64 {
    let mut t = 0.0;
    for p in precios {
        t += p;
    }
    t
}

fn main() {
    let carrito = [2.5, 4.0, 1.25];
    let otro = vec![10.0, 0.5];
    println!("{} {}", total(&carrito), total(&otro));
}
