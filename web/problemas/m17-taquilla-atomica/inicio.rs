use std::sync::atomic::{AtomicU32, Ordering};

struct Taquilla {
    quedan: AtomicU32,
}

impl Taquilla {
    fn new(n: u32) -> Taquilla {
        Taquilla { quedan: AtomicU32::new(n) }
    }

    fn vender(&self, n: u32) -> bool {
        todo!()
    }

    fn quedan(&self) -> u32 {
        self.quedan.load(Ordering::Acquire)
    }
}
