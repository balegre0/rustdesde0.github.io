use std::rc::Rc;

async fn paso() {}

async fn tarea() {
    let contador = Rc::new(5);
    paso().await; // `contador` sigue vivo a través del .await
    println!("{contador}");
}

// Lo que exigen los ejecutores multihilo al lanzar una tarea
fn lanzar<F: std::future::Future + Send + 'static>(_f: F) {}

fn main() {
    lanzar(tarea());
}
