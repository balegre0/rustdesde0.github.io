#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Talla {
    Grande,
    Pequena,
    Mediana,
}

fn main() {
    let mut tallas = vec![Talla::Mediana, Talla::Grande, Talla::Pequena];
    tallas.sort();
    println!("{tallas:?}");
}
