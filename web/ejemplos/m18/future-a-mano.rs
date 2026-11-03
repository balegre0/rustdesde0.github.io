use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// 1. Un future propio: necesita `veces` sondeos antes de estar listo
struct Cuenta {
    veces: u32,
}

impl Future for Cuenta {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.veces == 0 {
            Poll::Ready("¡hecho!")
        } else {
            println!("  poll: faltan {}", self.veces);
            self.veces -= 1;
            // 2. Prometemos avisar: pedimos que nos vuelvan a sondear
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let mut fut = Cuenta { veces: 3 };
    let mut cx = Context::from_waker(Waker::noop());
    // 3. Sondeamos a mano hasta que diga Ready
    let mut sondeos = 0;
    let resultado = loop {
        sondeos += 1;
        match Pin::new(&mut fut).poll(&mut cx) {
            Poll::Ready(v) => break v,
            Poll::Pending => {}
        }
    };
    println!("{resultado} tras {sondeos} sondeos");
}
