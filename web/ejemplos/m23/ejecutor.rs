use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::task::{Context, Poll, Wake, Waker};

// Un futuro que «espera» un número de ticks: cada sondeo descuenta uno.
struct Esperar {
    ticks: u32,
}

impl Future for Esperar {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.ticks == 0 {
            Poll::Ready(())
        } else {
            self.ticks -= 1;
            cx.waker().wake_by_ref(); // «vuelve a sondearme»
            Poll::Pending
        }
    }
}

struct Despertador;
impl Wake for Despertador {
    fn wake(self: Arc<Self>) {}
}

static SONDEOS: AtomicU32 = AtomicU32::new(0);

// El ejecutor más simple posible: un bucle que sondea hasta que la tarea termina.
// Embassy hace lo mismo, pero duerme la CPU hasta que una interrupción la despierta.
fn bloquear_en<F: Future>(mut f: F) -> F::Output {
    let waker = Waker::from(Arc::new(Despertador));
    let mut cx = Context::from_waker(&waker);
    // SAFETY: `f` no se mueve después de esta línea: vive en este marco hasta el final.
    let mut f = unsafe { Pin::new_unchecked(&mut f) };
    loop {
        SONDEOS.fetch_add(1, Ordering::Relaxed);
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

async fn tarea_led() -> &'static str {
    Esperar { ticks: 2 }.await; // encender
    Esperar { ticks: 3 }.await; // apagar
    "LED: dos fases completadas"
}

fn main() {
    let r = bloquear_en(tarea_led());
    println!("{r} en {} sondeos", SONDEOS.load(Ordering::Relaxed));
}
