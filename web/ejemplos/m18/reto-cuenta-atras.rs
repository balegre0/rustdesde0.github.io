use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// Devuelve Pending `n` veces y después Ready(total de sondeos)
struct CuentaAtras { n: u32, sondeos: u32 }

impl Future for CuentaAtras {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        self.sondeos += 1;
        if self.n == 0 {
            Poll::Ready(self.sondeos)
        } else {
            self.n -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
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
