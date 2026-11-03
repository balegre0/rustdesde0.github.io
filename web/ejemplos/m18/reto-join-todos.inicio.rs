use std::future::{poll_fn, Future};
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

struct Tarda(u32, u32); // (sondeos que faltan, valor)
impl Future for Tarda {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        if self.0 == 0 { return Poll::Ready(self.1); }
        self.0 -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

// Espera a TODOS y devuelve los resultados en el mismo orden de entrada
async fn join_todos(futs: Vec<Tarda>) -> Vec<u32> {
    // Pista: guarda un Vec<Option<u32>> de resultados y usa poll_fn;
    // Tarda es Unpin, así que Pin::new(&mut f) vale.
    todo!()
}

fn main() {
    let v = block_on(join_todos(vec![Tarda(3, 1), Tarda(0, 2), Tarda(1, 3)]));
    println!("{v:?}");
}
