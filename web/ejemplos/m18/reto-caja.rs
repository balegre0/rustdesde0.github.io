use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

type Caja = Pin<Box<dyn Future<Output = u32>>>;

// Sondea cada future hasta que termine y suma todos los resultados
fn sumar_todas(futs: Vec<Caja>) -> u32 {
    let mut cx = Context::from_waker(Waker::noop());
    let mut total = 0;
    for mut f in futs {
        loop {
            if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
                total += v;
                break;
            }
        }
    }
    total
}

async fn diez() -> u32 { 10 }

fn main() {
    // Tres futures de TIPOS distintos, juntos gracias a Pin<Box<dyn Future>>
    let futs: Vec<Caja> = vec![
        Box::pin(diez()),
        Box::pin(async { 20 }),
        Box::pin(async { diez().await * 3 }),
    ];
    println!("total = {}", sumar_todas(futs));
}
