use std::sync::Arc;

fn main() {
    let mapa = Arc::new(vec!["norte", "sur"]);
    let copias: Vec<Arc<Vec<&str>>> = vec![Arc::clone(&mapa), Arc::clone(&mapa)];
    println!("dueños = {}", Arc::strong_count(&mapa));
    drop(copias);
    println!("dueños = {}", Arc::strong_count(&mapa));
}
