
// ---------- pruebas ----------
static SONDEOS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

// Cede `veces` veces; en cada cesión despierta `avisos` veces
struct Cede { veces: u32, avisos: u32 }
impl std::future::Future for Cede {
    type Output = ();
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        SONDEOS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if self.veces == 0 {
            return std::task::Poll::Ready(());
        }
        self.veces -= 1;
        for _ in 0..self.avisos {
            cx.waker().wake_by_ref();
        }
        std::task::Poll::Pending
    }
}

fn sondeos() -> u32 {
    SONDEOS.swap(0, std::sync::atomic::Ordering::SeqCst)
}

fn main() {
    let mut fallos = 0;

    let r = block_on(async { 5 });
    if r == 5 { println!("OK: block_on(async {{ 5 }}) = 5"); } else { println!("FALLO: se esperaba 5 y salió {r}"); fallos += 1; }

    sondeos();
    let r = block_on(async { Cede { veces: 3, avisos: 1 }.await; "fin" });
    let n = sondeos();
    if r == "fin" && n == 4 { println!("OK: cede 3 veces → \"fin\" con 4 sondeos"); }
    else { println!("FALLO: cede 3 veces → {r:?} con {n} sondeos (se esperaba \"fin\" con 4)"); fallos += 1; }

    let r = block_on(async { Cede { veces: 2, avisos: 3 }.await; 10 });
    let n = sondeos();
    if r == 10 && n == 3 { println!("OK: tres avisos por cesión no provocan sondeos de más (3)"); }
    else { println!("FALLO: con 3 avisos por cesión: resultado {r}, {n} sondeos (se esperaba 10 con 3)"); fallos += 1; }

    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
