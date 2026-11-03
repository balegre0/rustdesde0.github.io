use std::future::{self, Future};
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

// un future propio: tarda tres sondeos en estar listo
struct TresPasos {
    paso: u32,
}

impl Future for TresPasos {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.paso += 1;
        println!("  sondeo {}", self.paso);
        if self.paso < 3 {
            cx.waker().wake_by_ref(); // avisa: «vuelve a sondearme»
            Poll::Pending
        } else {
            Poll::Ready("listo")
        }
    }
}

async fn sumar(a: u32, b: u32) -> u32 {
    let extra = future::ready(1).await;
    a + b + extra
}

async fn programa() -> u32 {
    let r = TresPasos { paso: 0 }.await;
    println!("TresPasos: {r}");
    sumar(2, 3).await
}

// un ejecutor mínimo: sondea hasta que el future termina
fn bloquear<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

fn main() {
    let f = programa(); // crear un future no ejecuta nada
    println!("creado");
    println!("resultado: {}", bloquear(f));
}
