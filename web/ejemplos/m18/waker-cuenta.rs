use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

// 1. Nuestro waker: solo cuenta cuántas veces lo despiertan
struct Contador {
    avisos: AtomicUsize,
}

impl Wake for Contador {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.avisos.fetch_add(1, Ordering::SeqCst);
    }
}

// 2. Un future que se despierta a sí mismo dos veces
struct DosVueltas(u8);

impl Future for DosVueltas {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 == 2 {
            return Poll::Ready(());
        }
        self.0 += 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

fn main() {
    let contador = Arc::new(Contador { avisos: AtomicUsize::new(0) });
    // 3. Arc<impl Wake> se convierte en Waker
    let waker = Waker::from(contador.clone());
    let mut cx = Context::from_waker(&waker);

    let mut fut = DosVueltas(0);
    let mut sondeos = 0;
    while Pin::new(&mut fut).poll(&mut cx).is_pending() {
        sondeos += 1;
    }
    println!("Pending devuelto: {sondeos} veces");
    println!("avisos recibidos: {}", contador.avisos.load(Ordering::SeqCst));
}
