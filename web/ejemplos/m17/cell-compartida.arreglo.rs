use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

fn main() {
    let visitas = AtomicU32::new(0);
    thread::scope(|s| {
        s.spawn(|| visitas.fetch_add(1, Ordering::Relaxed));
        s.spawn(|| visitas.fetch_add(1, Ordering::Relaxed));
    });
    println!("{}", visitas.load(Ordering::Relaxed));
}
