use std::future::Future;
use std::pin::pin;
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

trait Sensor {
    async fn medir(&self) -> i32;
}

struct Termometro { grados: i32 }
struct Doble<S: Sensor>(S);

impl Sensor for Termometro {
    async fn medir(&self) -> i32 { self.grados }
}

// Un sensor que envuelve a otro y duplica su medida
impl<S: Sensor> Sensor for Doble<S> {
    async fn medir(&self) -> i32 { self.0.medir().await * 2 }
}

fn main() {
    let t = Termometro { grados: 21 };
    let d = Doble(Doble(Termometro { grados: 5 }));
    println!("{} {}", block_on(t.medir()), block_on(d.medir()));
}
