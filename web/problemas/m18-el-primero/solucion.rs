use std::future::{poll_fn, Future};
use std::pin::Pin;
use std::task::Poll;

async fn el_primero<F: Future + Unpin>(futs: Vec<F>) -> Option<(usize, F::Output)> {
    if futs.is_empty() {
        return None;
    }
    let mut futs = futs;
    poll_fn(move |cx| {
        for (i, f) in futs.iter_mut().enumerate() {
            if let Poll::Ready(v) = Pin::new(f).poll(cx) {
                return Poll::Ready(Some((i, v)));
            }
        }
        Poll::Pending
    })
    .await
}
