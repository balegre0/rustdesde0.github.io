use std::sync::mpsc;

fn main() {
    let (_tx, rx) = mpsc::channel::<i32>();
    let rx2 = rx.clone(); // ¿dos consumidores?
    let _ = rx2;
}
