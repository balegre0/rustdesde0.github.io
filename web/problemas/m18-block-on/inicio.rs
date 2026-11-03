use std::future::Future;
use std::pin::pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct Bandera(AtomicBool);

impl Wake for Bandera {
    fn wake(self: Arc<Self>) {
        todo!()
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    todo!()
}
