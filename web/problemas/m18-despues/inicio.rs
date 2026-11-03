use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Despues {
    n: u32,
    valor: u64,
}

fn despues(n: u32, valor: u64) -> Despues {
    Despues { n, valor }
}

impl Future for Despues {
    type Output = u64;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u64> {
        todo!()
    }
}
