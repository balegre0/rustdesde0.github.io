use std::future::{poll_fn, Future};
use std::pin::Pin;
use std::task::Poll;

async fn join_todos<F: Future + Unpin>(futs: Vec<F>) -> Vec<F::Output> {
    let mut futs = futs;
    let mut res: Vec<Option<F::Output>> = futs.iter().map(|_| None).collect();
    poll_fn(move |cx| {
        for (i, f) in futs.iter_mut().enumerate() {
            if res[i].is_none() {
                if let Poll::Ready(v) = Pin::new(f).poll(cx) {
                    res[i] = Some(v);
                }
            }
        }
        if res.iter().all(|r| r.is_some()) {
            Poll::Ready(res.iter_mut().map(|r| r.take().unwrap()).collect())
        } else {
            Poll::Pending
        }
    })
    .await
}
