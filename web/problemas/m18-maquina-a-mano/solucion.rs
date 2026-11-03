use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Devuelve Pending una vez y después Ready(n * 2)
struct Doble { n: u32, cedido: bool }
fn doble(n: u32) -> Doble { Doble { n, cedido: false } }
impl Future for Doble {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        if self.cedido {
            return Poll::Ready(self.n * 2);
        }
        self.cedido = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

enum DosPasos {
    Inicio { x: u32 },
    Primero { espera: Doble },
    Segundo { a: u32, espera: Doble },
    Fin,
}

fn dos_pasos(x: u32) -> DosPasos {
    DosPasos::Inicio { x }
}

impl Future for DosPasos {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        loop {
            match &mut *self {
                DosPasos::Inicio { x } => {
                    let x = *x;
                    *self = DosPasos::Primero { espera: doble(x) };
                }
                DosPasos::Primero { espera } => match Pin::new(espera).poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(a) => *self = DosPasos::Segundo { a, espera: doble(a) },
                },
                DosPasos::Segundo { a, espera } => match Pin::new(espera).poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(b) => {
                        let total = *a + b;
                        *self = DosPasos::Fin;
                        return Poll::Ready(total);
                    }
                },
                DosPasos::Fin => panic!("sondeado tras terminar"),
            }
        }
    }
}
