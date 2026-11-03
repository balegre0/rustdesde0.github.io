use std::sync::Mutex;

async fn paso() {}

async fn incrementar(m: &Mutex<u32>) {
    let mut g = m.lock().unwrap();
    *g += 1;
    paso().await; // el guard sigue vivo aquí
}

fn lanzar<F: std::future::Future + Send>(_f: F) {}

fn main() {
    static M: Mutex<u32> = Mutex::new(0);
    lanzar(incrementar(&M));
}
