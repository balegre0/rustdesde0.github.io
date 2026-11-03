use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::task::{Wake, Waker};

// Un atómico sí es Sync: el waker puede viajar a cualquier hilo
struct Avisos(AtomicU32);

impl Wake for Avisos {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    let a = Arc::new(Avisos(AtomicU32::new(0)));
    let w = Waker::from(a.clone());
    w.wake();
    println!("{}", a.0.load(Ordering::SeqCst));
}
