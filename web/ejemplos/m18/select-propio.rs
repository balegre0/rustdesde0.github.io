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

struct Trabajo { nombre: &'static str, n: u32 }
impl Future for Trabajo {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if self.n == 0 { return Poll::Ready(self.nombre); }
        self.n -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
// 1. Al soltar un Trabajo sin terminar, avisamos
impl Drop for Trabajo {
    fn drop(&mut self) {
        if self.n > 0 {
            println!("  {} cancelado (le faltaban {} sondeos)", self.nombre, self.n);
        }
    }
}

enum Gana<A, B> { Izq(A), Der(B) }

// 2. select: el primero que termine gana; el otro se suelta (drop) = cancelado
async fn select<A: Future, B: Future>(a: A, b: B) -> Gana<A::Output, B::Output> {
    let mut a = pin!(a);
    let mut b = pin!(b);
    std::future::poll_fn(|cx| {
        if let Poll::Ready(v) = a.as_mut().poll(cx) { return Poll::Ready(Gana::Izq(v)); }
        if let Poll::Ready(v) = b.as_mut().poll(cx) { return Poll::Ready(Gana::Der(v)); }
        Poll::Pending
    })
    .await
}

fn main() {
    let r = block_on(async {
        let rapido = Trabajo { nombre: "rápido", n: 1 };
        let lento = Trabajo { nombre: "lento", n: 5 };
        let r = select(lento, rapido).await;
        println!("select terminó");
        r
    });
    match r {
        Gana::Izq(n) | Gana::Der(n) => println!("ganó: {n}"),
    }
}
