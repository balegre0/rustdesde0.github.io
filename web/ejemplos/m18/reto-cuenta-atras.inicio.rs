use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// Devuelve Pending `n` veces y después Ready(total de sondeos)
struct CuentaAtras { n: u32, sondeos: u32 }

impl Future for CuentaAtras {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        // 1. suma un sondeo
        // 2. si n == 0 -> Ready(sondeos)
        // 3. si no: resta 1, avisa con cx.waker().wake_by_ref() y devuelve Pending
        todo!()
    }
}

fn main() {
    let mut f = CuentaAtras { n: 4, sondeos: 0 };
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(total) = Pin::new(&mut f).poll(&mut cx) {
            println!("listo tras {total} sondeos");
            break;
        }
    }
}
