use std::pin::Pin;

async fn tarea() -> u8 { 1 }

fn main() {
    let mut fut = tarea();
    // Pin::new solo acepta tipos Unpin; un future de async fn no lo es
    let _fijado = Pin::new(&mut fut);
}
