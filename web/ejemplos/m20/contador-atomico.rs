// Mejor: un atómico. Ni unsafe ni static mut.
use std::sync::atomic::{AtomicU32, Ordering};

static CONTADOR: AtomicU32 = AtomicU32::new(0);

fn main() {
    CONTADOR.fetch_add(1, Ordering::Relaxed);
    println!("{}", CONTADOR.load(Ordering::Relaxed));
}
