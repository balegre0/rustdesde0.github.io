use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// Cede el turno una vez
struct Ceder(bool);
impl Future for Ceder {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 { return Poll::Ready(()); }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

// ❌ No es seguro cancelarla: saca el mensaje ANTES de un .await
async fn procesar_mal(cola: &RefCell<VecDeque<&'static str>>) {
    let msg = cola.borrow_mut().pop_front();
    Ceder(false).await; // si nos cancelan aquí, `msg` se pierde
    println!("  procesado: {msg:?}");
}

// ✅ Segura: espera primero y saca el mensaje cuando ya no hay más .await
async fn procesar_bien(cola: &RefCell<VecDeque<&'static str>>) {
    Ceder(false).await;
    let msg = cola.borrow_mut().pop_front();
    println!("  procesado: {msg:?}");
}

fn sondear_una_vez_y_cancelar(fut: impl Future<Output = ()>) {
    let mut fut = Box::pin(fut);
    let mut cx = Context::from_waker(Waker::noop());
    let _ = fut.as_mut().poll(&mut cx);
    // se suelta aquí: cancelado
}

fn main() {
    let cola = RefCell::new(VecDeque::from(["a", "b"]));
    sondear_una_vez_y_cancelar(procesar_mal(&cola));
    println!("tras cancelar procesar_mal: {:?}", cola.borrow());

    let cola = RefCell::new(VecDeque::from(["a", "b"]));
    sondear_una_vez_y_cancelar(procesar_bien(&cola));
    println!("tras cancelar procesar_bien: {:?}", cola.borrow());
}
