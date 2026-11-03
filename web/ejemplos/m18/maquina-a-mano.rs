use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

// Un future que tarda un sondeo («cede el turno» una vez)
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

// 1. Lo que escribirías:  async fn doble(x: u32) -> u32 { Ceder(false).await; x * 2 }
// 2. Lo que el compilador genera, más o menos, escrito a mano:
enum Doble {
    SinEmpezar { x: u32 },
    Esperando { x: u32, dentro: Ceder },
    Terminado,
}

impl Future for Doble {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        loop {
            match &mut *self {
                Doble::SinEmpezar { x } => {
                    println!("  estado: SinEmpezar -> Esperando");
                    let x = *x;
                    *self = Doble::Esperando { x, dentro: Ceder(false) };
                }
                Doble::Esperando { x, dentro } => {
                    // 3. «.await» = sondear el future de dentro; si no está, devolver Pending
                    match Pin::new(dentro).poll(cx) {
                        Poll::Pending => {
                            println!("  estado: Esperando (Pending)");
                            return Poll::Pending;
                        }
                        Poll::Ready(()) => {
                            let r = *x * 2;
                            println!("  estado: Esperando -> Terminado");
                            *self = Doble::Terminado;
                            return Poll::Ready(r);
                        }
                    }
                }
                Doble::Terminado => panic!("sondeado después de terminar"),
            }
        }
    }
}

fn main() {
    let mut fut = pin!(Doble::SinEmpezar { x: 21 });
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            println!("resultado: {v}");
            break;
        }
    }
}
