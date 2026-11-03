use std::cell::Cell;
use std::sync::Arc;
use std::task::{Wake, Waker};

// Cell no es Sync: no se puede compartir entre hilos
struct Avisos(Cell<u32>);

impl Wake for Avisos {
    fn wake(self: Arc<Self>) {
        self.0.set(self.0.get() + 1);
    }
}

fn main() {
    let a = Arc::new(Avisos(Cell::new(0)));
    let w = Waker::from(a.clone());
    w.wake();
    println!("{}", a.0.get());
}
