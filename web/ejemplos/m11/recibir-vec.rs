fn total(precios: &Vec<f64>) -> f64 {
    let mut t = 0.0;
    for p in precios {
        t += p;
    }
    t
}

fn main() {
    let carrito = [2.5, 4.0, 1.25];
    println!("{}", total(&carrito));
}
