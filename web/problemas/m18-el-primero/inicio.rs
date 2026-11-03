use std::future::{poll_fn, Future};
use std::pin::Pin;
use std::task::Poll;

async fn el_primero<F: Future + Unpin>(futs: Vec<F>) -> Option<(usize, F::Output)> {
    todo!()
}
