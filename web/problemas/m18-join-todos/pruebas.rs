
// ---------- pruebas ----------
static SONDEOS_TRAS_READY: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

struct Tarda<T> { faltan: u32, valor: Option<T> }
impl<T: Unpin> std::future::Future for Tarda<T> {
    type Output = T;
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<T> {
        if self.valor.is_none() {
            SONDEOS_TRAS_READY.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            return std::task::Poll::Pending;
        }
        if self.faltan == 0 {
            return std::task::Poll::Ready(self.valor.take().unwrap());
        }
        self.faltan -= 1;
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}
fn tarda<T>(faltan: u32, v: T) -> Tarda<T> { Tarda { faltan, valor: Some(v) } }

fn ejecutar<F: std::future::Future>(f: F) -> Option<(F::Output, u32)> {
    let mut f = std::pin::pin!(f);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    for n in 1..=1000 {
        if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return Some((v, n));
        }
    }
    None
}

fn main() {
    let mut fallos = 0;

    let r = ejecutar(join_todos(vec![tarda(3, 'a'), tarda(0, 'b'), tarda(1, 'c')]));
    if r == Some((vec!['a', 'b', 'c'], 4)) { println!("OK: ['a', 'b', 'c'] en 4 sondeos"); }
    else { println!("FALLO: se esperaba Some((['a', 'b', 'c'], 4)) y salió {r:?}"); fallos += 1; }

    let r = ejecutar(join_todos(Vec::<Tarda<u8>>::new()));
    if r == Some((vec![], 1)) { println!("OK: lista vacía → [] en 1 sondeo"); }
    else { println!("FALLO: lista vacía → {r:?}, se esperaba Some(([], 1))"); fallos += 1; }

    let r = ejecutar(join_todos(vec![tarda(5, 10), tarda(5, 20)]));
    if r == Some((vec![10, 20], 6)) { println!("OK: dos que tardan 5 → [10, 20] en 6 sondeos (a la vez)"); }
    else { println!("FALLO: dos que tardan 5 → {r:?}, se esperaba Some(([10, 20], 6))"); fallos += 1; }

    let extra = SONDEOS_TRAS_READY.load(std::sync::atomic::Ordering::SeqCst);
    if extra == 0 { println!("OK: ningún future se sondeó después de Ready"); }
    else { println!("FALLO: {extra} sondeos a futures que ya habían terminado"); fallos += 1; }

    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
