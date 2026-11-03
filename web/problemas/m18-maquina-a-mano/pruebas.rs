
// ---------- pruebas ----------
async fn dos_pasos_async(x: u32) -> u32 {
    let a = doble(x).await;
    let b = doble(a).await;
    a + b
}

fn ejecutar<F: std::future::Future>(f: F) -> Option<(F::Output, u32)> {
    let mut f = std::pin::pin!(f);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    for n in 1..=100 {
        if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return Some((v, n));
        }
    }
    None
}

fn main() {
    let mut fallos = 0;
    for x in [3u32, 0, 10, 1000] {
        let a_mano = ejecutar(dos_pasos(x));
        let compilador = ejecutar(dos_pasos_async(x));
        if a_mano.is_some() && a_mano == compilador {
            let (v, n) = a_mano.unwrap();
            println!("OK: dos_pasos({x}) = {v} en {n} sondeos, igual que la versión async");
        } else {
            println!("FALLO: dos_pasos({x}): a mano {a_mano:?}, versión async {compilador:?} (valor, sondeos)");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
