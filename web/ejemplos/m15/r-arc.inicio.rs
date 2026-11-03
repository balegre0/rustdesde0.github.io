use std::sync::Arc;

fn main() {
    let mapa = Arc::new(vec!["norte", "sur"]);
    // 1. Crea un Vec con dos clones de `mapa` (Arc::clone).
    let copias: Vec<Arc<Vec<&str>>> = todo!();
    println!("dueños = {}", Arc::strong_count(&mapa));
    // 2. Suelta `copias` para que el contador vuelva a 1.
    let _ = &copias;
    println!("dueños = {}", Arc::strong_count(&mapa));
}
