use std::future::{poll_fn, Future};
use std::pin::Pin;
use std::task::Poll;

async fn join_todos<F: Future + Unpin>(futs: Vec<F>) -> Vec<F::Output> {
    todo!()
}
