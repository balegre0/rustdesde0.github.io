use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

// Ejecuta el future y devuelve (resultado, número de sondeos)
fn block_on_contando<F: Future>(fut: F) -> (F::Output, u32) {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    let mut n = 0;
    loop {
        n += 1;
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return (v, n);
        }
    }
}

struct Ceder(bool);
impl Future for Ceder {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 { return Poll::Ready(()); }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

fn main() {
    let (v, n) = block_on_contando(async {
        Ceder(false).await;
        Ceder(false).await;
        Ceder(false).await;
        "fin"
    });
    println!("{v} en {n} sondeos");
}
