
// ---------- pruebas ----------
static SOLTADOS_SIN_TERMINAR: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static SONDEOS_TOTALES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

struct Tarda { faltan: u32, valor: &'static str, terminado: bool }
impl std::future::Future for Tarda {
    type Output = &'static str;
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<&'static str> {
        SONDEOS_TOTALES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if self.faltan == 0 {
            self.terminado = true;
            return std::task::Poll::Ready(self.valor);
        }
        self.faltan -= 1;
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}
impl Drop for Tarda {
    fn drop(&mut self) {
        if !self.terminado {
            SOLTADOS_SIN_TERMINAR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
fn tarda(faltan: u32, valor: &'static str) -> Tarda { Tarda { faltan, valor, terminado: false } }

fn ejecutar<F: std::future::Future>(f: F) -> Option<F::Output> {
    let mut f = std::pin::pin!(f);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    for _ in 0..1000 {
        if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return Some(v);
        }
    }
    None
}

fn contadores() -> (u32, u32) {
    (SOLTADOS_SIN_TERMINAR.swap(0, std::sync::atomic::Ordering::SeqCst),
     SONDEOS_TOTALES.swap(0, std::sync::atomic::Ordering::SeqCst))
}

fn main() {
    let mut fallos = 0;

    let r = ejecutar(el_primero(vec![tarda(4, "lento"), tarda(1, "rápido"), tarda(1, "empate")]));
    let (soltados, sondeos) = contadores();
    if r == Some(Some((1, "rápido"))) { println!("OK: gana (1, \"rápido\")"); }
    else { println!("FALLO: se esperaba Some((1, \"rápido\")) y salió {r:?}"); fallos += 1; }
    if soltados == 2 { println!("OK: los 2 perdedores se soltaron sin terminar"); }
    else { println!("FALLO: se soltaron {soltados} futures sin terminar, se esperaban 2"); fallos += 1; }
    // vuelta 1: 3 sondeos (todos Pending); vuelta 2: lento Pending, rápido Ready → 5
    if sondeos == 5 { println!("OK: 5 sondeos en total (nadie se sondeó tras el ganador)"); }
    else { println!("FALLO: {sondeos} sondeos en total, se esperaban 5"); fallos += 1; }

    let r = ejecutar(el_primero(vec![tarda(0, "ya"), tarda(0, "también")]));
    let (soltados, _) = contadores();
    if r == Some(Some((0, "ya"))) && soltados == 1 { println!("OK: empate en la primera vuelta → gana el índice 0"); }
    else { println!("FALLO: empate → {r:?} con {soltados} soltados, se esperaba Some((0, \"ya\")) con 1"); fallos += 1; }

    let r = ejecutar(el_primero(Vec::<Tarda>::new()));
    if r == Some(None) { println!("OK: lista vacía → None"); }
    else { println!("FALLO: lista vacía → {r:?}, se esperaba None"); fallos += 1; }

    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
