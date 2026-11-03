use std::future::Future;
use std::pin::pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct Bandera(AtomicBool);

impl Wake for Bandera {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::SeqCst);
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let bandera = Arc::new(Bandera(AtomicBool::new(true)));
    let waker = Waker::from(bandera.clone());
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    loop {
        if bandera.0.swap(false, Ordering::SeqCst) {
            if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
                return v;
            }
        } else {
            panic!("tarea dormida para siempre");
        }
    }
}
