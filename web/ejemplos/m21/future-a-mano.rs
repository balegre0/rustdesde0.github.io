use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// 1. Una máquina de estados escrita a mano: lo que el compilador genera para un async fn
enum CuentaAtras { Contando(u32), Hecho }

impl Future for CuentaAtras {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self {
            CuentaAtras::Contando(0) => { *self = CuentaAtras::Hecho; Poll::Ready("¡despegue!") }
            CuentaAtras::Contando(n) => {
                println!("  poll: quedan {n}");
                *self = CuentaAtras::Contando(n - 1);
                cx.waker().wake_by_ref(); // «vuelve a llamarme»
                Poll::Pending
            }
            CuentaAtras::Hecho => panic!("poll después de terminar"),
        }
    }
}

async fn paso() {}

// 2. Un async fn: el compilador lo convierte en un enum con un estado por cada .await
async fn con_array() -> u8 {
    let a = [7u8; 100]; // vive a través del .await => se guarda DENTRO del futuro
    paso().await;
    a[0]
}

async fn sin_array(x: u64) -> u64 {
    paso().await;
    x + 1
}

// 3. El ejecutor más simple posible: sondear hasta Ready
fn bloquear<F: Future>(f: F) -> F::Output {
    let mut f = std::pin::pin!(f);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) { return v; }
    }
}

fn main() {
    println!("{}", bloquear(CuentaAtras::Contando(3)));
    println!("con_array() ocupa {} bytes", std::mem::size_of_val(&con_array()));
    println!("sin_array(1) ocupa {} bytes", std::mem::size_of_val(&sin_array(1)));
    println!("resultados: {} {}", bloquear(con_array()), bloquear(sin_array(41)));
}
