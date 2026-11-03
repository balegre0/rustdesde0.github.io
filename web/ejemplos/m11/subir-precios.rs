fn main() {
    let mut precios = vec![10.0, 20.0, 5.0];
    for p in &mut precios {
        *p *= 1.10;
    }
    let mut total = 0.0;
    for p in &precios {
        total += p;
    }
    println!("total {:.2}", total);
    println!("{:?}", precios);
}
