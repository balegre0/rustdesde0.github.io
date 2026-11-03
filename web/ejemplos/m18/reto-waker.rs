use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::task::{Wake, Waker};

// Un waker que suma 10 cada vez que lo despiertan
struct Hucha(AtomicU32);

impl Wake for Hucha {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(10, Ordering::SeqCst);
    }
}

fn main() {
    let hucha = Arc::new(Hucha(AtomicU32::new(0)));
    let waker = Waker::from(hucha.clone());
    waker.wake_by_ref();
    waker.wake_by_ref();
    let copia = waker.clone();
    copia.wake(); // wake consume el waker
    println!("hucha = {}", hucha.0.load(Ordering::SeqCst));
}
