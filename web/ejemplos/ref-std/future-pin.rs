use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct Nada;
impl Wake for Nada {
    fn wake(self: Arc<Self>) {}
}

async fn doble(x: u32) -> u32 {
    x * 2
}

fn main() {
    // Un futuro no hace nada hasta que alguien lo sondea (poll).
    let futuro = doble(21);
    // pin! lo fija en la pila: a partir de aquí no se puede mover.
    let mut futuro: Pin<&mut _> = pin!(futuro);
    let waker = Waker::from(Arc::new(Nada));
    let mut cx = Context::from_waker(&waker);
    match futuro.as_mut().poll(&mut cx) {
        Poll::Ready(v) => println!("listo: {v}"),
        Poll::Pending => println!("todavía no"),
    }
}
