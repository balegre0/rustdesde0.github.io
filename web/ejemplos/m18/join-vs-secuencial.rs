use std::future::{poll_fn, Future};
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

// Cuenta cuántos sondeos hace falta al bloque entero
fn block_on<F: Future>(fut: F) -> (F::Output, u32) {
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

struct Tarda(u32);
impl Future for Tarda {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 == 0 { return Poll::Ready(()); }
        self.0 -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

async fn join2(a: Tarda, b: Tarda) {
    let (mut a, mut b) = (a, b);
    let (mut fa, mut fb) = (false, false);
    poll_fn(|cx| {
        if !fa { fa = Pin::new(&mut a).poll(cx).is_ready(); }
        if !fb { fb = Pin::new(&mut b).poll(cx).is_ready(); }
        if fa && fb { Poll::Ready(()) } else { Poll::Pending }
    })
    .await
}

fn main() {
    let (_, n1) = block_on(async {
        Tarda(3).await;
        Tarda(3).await;
    });
    let (_, n2) = block_on(join2(Tarda(3), Tarda(3)));
    println!("secuencial: {n1} sondeos");
    println!("join: {n2} sondeos");
}
