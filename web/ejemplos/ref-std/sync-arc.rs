use std::sync::Arc;

fn main() {
    let datos = Arc::new(vec![1, 2, 3]);
    let otra = Arc::clone(&datos);
    println!("{} {:?}", Arc::strong_count(&datos), otra);
    drop(otra);
    println!("{}", Arc::strong_count(&datos));
}
