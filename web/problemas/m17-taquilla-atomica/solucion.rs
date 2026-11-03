use std::sync::atomic::{AtomicU32, Ordering};

struct Taquilla {
    quedan: AtomicU32,
}

impl Taquilla {
    fn new(n: u32) -> Taquilla {
        Taquilla { quedan: AtomicU32::new(n) }
    }

    fn vender(&self, n: u32) -> bool {
        self.quedan
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |q| q.checked_sub(n))
            .is_ok()
    }

    fn quedan(&self) -> u32 {
        self.quedan.load(Ordering::Acquire)
    }
}
