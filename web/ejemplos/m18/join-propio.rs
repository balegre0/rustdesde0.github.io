use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

// Un future que necesita `n` sondeos; dice su nombre en cada uno
struct Trabajo { nombre: &'static str, n: u32 }
impl Future for Trabajo {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        println!("  sondeo {} (faltan {})", self.nombre, self.n);
        if self.n == 0 { return Poll::Ready(self.nombre); }
        self.n -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

// 1. join: sondea los dos futures en cada vuelta y espera a que acaben ambos
async fn join<A: Future, B: Future>(a: A, b: B) -> (A::Output, B::Output) {
    let mut a = pin!(a);
    let mut b = pin!(b);
    let mut ra = None;
    let mut rb = None;
    std::future::poll_fn(move |cx| {
        if ra.is_none() {
            if let Poll::Ready(v) = a.as_mut().poll(cx) { ra = Some(v); }
        }
        if rb.is_none() {
            if let Poll::Ready(v) = b.as_mut().poll(cx) { rb = Some(v); }
        }
        if ra.is_some() && rb.is_some() {
            Poll::Ready((ra.take().unwrap(), rb.take().unwrap()))
        } else {
            Poll::Pending
        }
    })
    .await
}

fn main() {
    let (x, y) = block_on(join(
        Trabajo { nombre: "A", n: 2 },
        Trabajo { nombre: "B", n: 1 },
    ));
    println!("terminaron {x} y {y}");
}
