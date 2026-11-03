use std::future::Future;
use std::pin::pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

// 1. El waker levanta una bandera: «vuelve a sondear»
struct Bandera(AtomicBool);

impl Wake for Bandera {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::SeqCst);
    }
}

// 2. El ejecutor más pequeño posible: sondear solo cuando nos avisan
fn block_on<F: Future>(fut: F) -> F::Output {
    let bandera = Arc::new(Bandera(AtomicBool::new(true)));
    let waker = Waker::from(bandera.clone());
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    let mut sondeos = 0;
    loop {
        if bandera.0.swap(false, Ordering::SeqCst) {
            sondeos += 1;
            if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
                println!("[ejecutor] {sondeos} sondeos");
                return v;
            }
        } else {
            // Nadie nos ha despertado: con E/S real aquí esperaríamos al sistema
            panic!("bloqueo: el future dijo Pending y nadie llamó a wake");
        }
    }
}

// 3. Un future que cede el turno una vez («yield»)
struct Ceder(bool);
impl Future for Ceder {
    type Output = ();
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn sumar(a: u32, b: u32) -> u32 {
    Ceder(false).await;
    a + b
}

fn main() {
    let r = block_on(async {
        let x = sumar(1, 2).await;
        let y = sumar(x, 10).await;
        y * 2
    });
    println!("resultado = {r}");
}
