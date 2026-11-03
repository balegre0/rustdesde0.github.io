
// ---------- pruebas ----------
struct ContarAvisos(std::sync::atomic::AtomicU32);
impl std::task::Wake for ContarAvisos {
    fn wake(self: std::sync::Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    fn wake_by_ref(self: &std::sync::Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// Sondea hasta Ready (máx. 1000 veces). Devuelve (valor, sondeos, avisos)
fn ejecutar(mut f: Despues) -> Option<(u64, u32, u32)> {
    let avisos = std::sync::Arc::new(ContarAvisos(std::sync::atomic::AtomicU32::new(0)));
    let waker = std::task::Waker::from(avisos.clone());
    let mut cx = std::task::Context::from_waker(&waker);
    for sondeos in 1..=1000 {
        if let std::task::Poll::Ready(v) = std::future::Future::poll(std::pin::Pin::new(&mut f), &mut cx) {
            return Some((v, sondeos, avisos.0.load(std::sync::atomic::Ordering::SeqCst)));
        }
    }
    None
}

fn main() {
    let mut fallos = 0;
    let casos = [(0u32, 7u64), (3, 7), (1, 42), (10, 0)];
    for (n, v) in casos {
        let esperado = Some((v, n + 1, n));
        let r = ejecutar(despues(n, v));
        if r == esperado {
            println!("OK: despues({n}, {v}) → Ready({v}) en {} sondeos y {n} avisos", n + 1);
        } else {
            println!("FALLO: despues({n}, {v}) → (valor, sondeos, avisos) = {r:?}, se esperaba {esperado:?}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
