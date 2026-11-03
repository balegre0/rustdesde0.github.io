use std::sync::Arc;

async fn paso() {}

async fn tarea() {
    let contador = Arc::new(5); // Arc sí es Send
    paso().await;
    println!("{contador}");
}

fn lanzar<F: std::future::Future + Send + 'static>(_f: F) {
    println!("la tarea es Send: se podría lanzar en un ejecutor multihilo");
}

fn main() {
    lanzar(tarea());
}
